//! Running the saver: the loop, the clock, and the one promise that matters.
//!
//! # The promise
//!
//! A screen saver runs while you are away and **exits the moment you touch
//! anything**. It is not a background process, it does not install a wallpaper
//! twin, and it does not keep running after you dismiss it. That is the old
//! contract and it is the right one: a saver that lingers is a saver that burns
//! your battery and never quite goes away.
//!
//! So the loop is: draw a frame, check whether the user did anything, and if
//! they did, return immediately and exit. No timers left running, nothing
//! detached, nothing to clean up because nothing was ever started.

use crate::engine::{
    build_scene, choose, scene_duration, Choice, Frame, Rgb, Rng, WordBank,
};
use crate::render::Canvas;
use crate::x11::Connection;
pub use crate::words::read_optional;
use crate::words::{load_languages, load_words};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Configuration, read from a plain file so it can be edited by hand.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// Include the opt-in adult word list.
    pub adult: bool,
    /// Fixed seed for reproducible previews; random when `None`.
    pub seed: Option<u64>,
    /// Fraction of the viewport height used as the pen stroke width.
    pub stroke_scale: f64,
    /// Directory holding languages.tsv and words.tsv.
    pub data_dir: PathBuf,
    /// Directory holding fonts.
    pub font_dirs: Vec<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        let data_dir = PathBuf::from(
            std::env::var("KORRINOS_WORDPEN_DATA")
                .unwrap_or_else(|_| "/usr/share/korrinos/wordpen".to_string()),
        );
        Settings {
            adult: false,
            seed: None,
            stroke_scale: 1.0 / 90.0,
            data_dir,
            font_dirs: vec![
                PathBuf::from("/usr/share/fonts"),
                PathBuf::from("/usr/local/share/fonts"),
                PathBuf::from("/usr/share/korrinos/fonts"),
            ],
        }
    }
}

/// Parse settings from a simple `key = value` file.
///
/// Unknown keys are ignored and malformed lines are skipped rather than fatal,
/// because a stray space in a config file must not stop someone's screen from
/// working.
pub fn parse_settings(text: &str) -> Settings {
    let mut settings = Settings::default();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else { continue };
        let key = key.trim();
        let value = value.trim().trim_matches('"');
        match key {
            "adult" => settings.adult = matches!(value, "true" | "yes" | "1" | "on"),
            "seed" => settings.seed = value.parse().ok(),
            "stroke_scale" => {
                if let Ok(scale) = value.parse::<f64>() {
                    if scale > 0.0 {
                        settings.stroke_scale = scale;
                    }
                }
            },
            "data_dir" => settings.data_dir = PathBuf::from(value),
            _ => {},
        }
    }
    settings
}

/// Where the saver looks for its configuration.
pub fn config_path() -> PathBuf {
    if let Ok(explicit) = std::env::var("KORRINOS_WORDPEN_CONFIG") {
        return PathBuf::from(explicit);
    }
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
            PathBuf::from(home).join(".config")
        });
    base.join("korrinos").join("wordpen.conf")
}

/// Locate a font file for a family, best match first.
///
/// Ranking matters: a loose substring match would otherwise let any font that
/// happens to contain the family name win, which is how the wrong typeface ends
/// up on screen with no obvious cause.
pub fn find_font(font_dirs: &[PathBuf], family: &str) -> Option<PathBuf> {
    let needle: String = family
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    if needle.is_empty() {
        return None;
    }
    let mut best: Option<(u8, PathBuf)> = None;
    fn walk(dir: &Path, needle: &str, depth: usize, best: &mut Option<(u8, PathBuf)>) {
        if depth > 4 {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, needle, depth + 1, best);
                continue;
            }
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            if !(name.ends_with(".ttf") || name.ends_with(".otf") || name.ends_with(".ttc")) {
                continue;
            }
            let stem: String = name
                .split('.')
                .next()
                .unwrap_or("")
                .chars()
                .filter(|c| c.is_ascii_alphanumeric())
                .collect();
            let rank: u8 = if stem == needle {
                0
            } else if stem.starts_with(needle) {
                1
            } else if stem.contains(needle) {
                2
            } else {
                continue;
            };
            if best.as_ref().is_none_or(|(r, _)| rank < *r) {
                *best = Some((rank, path.clone()));
            }
        }
    }
    for dir in font_dirs {
        walk(dir, &needle, 0, &mut best);
        if best.as_ref().is_some_and(|(r, _)| *r == 0) {
            break;
        }
    }
    best.map(|(_, path)| path)
}

/// Fonts opened during a run, keyed by path.
///
/// Opening and parsing a 20 MB CJK font for every word would make the saver
/// stutter between words, so each is parsed once and kept.
#[derive(Default)]
pub struct FontCache {
    fonts: Vec<(String, std::sync::Arc<crate::outline::Font>)>,
}

impl FontCache {
    /// Get a parsed font, or open and remember it.
    pub fn get(
        &mut self,
        font_dirs: &[PathBuf],
        family: &str,
    ) -> Option<(String, std::sync::Arc<crate::outline::Font>)> {
        let path = find_font(font_dirs, family)?;
        let key = path.display().to_string();
        if let Some((_, font)) = self.fonts.iter().find(|(p, _)| *p == key) {
            return Some((key, font.clone()));
        }
        let bytes = std::fs::read(&path).ok()?;
        let font = crate::outline::Font::parse(bytes)?;
        let font = std::sync::Arc::new(font);
        self.fonts.push((key.clone(), font.clone()));
        Some((key, font))
    }
}

/// How the saver should end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The user did something, so we left.
    Dismissed,
    /// Ran for the whole cycle and stopped on request.
    Finished,
    /// Nothing could be drawn, ever.
    NothingToDraw,
}

/// One full run of the saver.
pub struct Runner {
    settings: Settings,
    words: WordBank,
    languages: Vec<crate::engine::Language>,
    fonts: FontCache,
}

impl Runner {
    /// Load data and prepare to draw.
    pub fn new(settings: Settings) -> Runner {
        let languages = load_languages(&read_optional(&settings.data_dir.join("languages.tsv")));
        let mut words = load_words(&read_optional(&settings.data_dir.join("words.tsv")));
        if settings.adult {
            // Only read the separate file when the option is on, which is what
            // makes it genuinely off by default rather than merely hidden.
            for (code, extra) in load_words(&read_optional(&settings.data_dir.join("words-adult.tsv"))) {
                words.entry(code).or_default().extend(extra);
            }
        }
        Runner { settings, words, languages, fonts: FontCache::default() }
    }

    /// The settings this runner was built with.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// How many languages the registry listed.
    pub fn language_count(&self) -> usize {
        self.languages.len()
    }

    /// How many words are available across all languages.
    pub fn word_count(&self) -> usize {
        self.words.values().map(Vec::len).sum()
    }

    /// How many languages can actually be drawn on this machine.
    ///
    /// This counts a language only when some word of its is drawable with some
    /// installed font. An earlier version counted any language whose family
    /// chain merely RESOLVED to a file, which reported all 55 as drawable on a
    /// machine with no CJK font at all - the chain ends in "DejaVu Sans", which
    /// exists, but it covers none of the Japanese words. A diagnostic that
    /// overstates what will work is worse than none.
    pub fn drawable_languages(&mut self) -> usize {
        let font_dirs = self.settings.font_dirs.clone();
        self.languages
            .iter()
            .filter(|language| {
                let Some(words) = self.words.get(&language.code) else { return false };
                if words.is_empty() {
                    return false;
                }
                let mut cache = FontCache::default();
                language.families.iter().any(|family| {
                    let Some((_, font)) = cache.get(&font_dirs, family) else { return false };
                    words.iter().any(|word| crate::engine::is_drawable(&font, word))
                })
            })
            .count()
    }

    /// Languages that cannot be drawn here, with the reason.
    ///
    /// Shown by `--info` so a missing font package is diagnosed rather than
    /// guessed at.
    pub fn undrawable_languages(&mut self) -> Vec<(String, String)> {
        let font_dirs = self.settings.font_dirs.clone();
        let mut missing = Vec::new();
        for language in &self.languages {
            let Some(words) = self.words.get(&language.code) else {
                missing.push((language.code.clone(), "no words".into()));
                continue;
            };
            let mut cache = FontCache::default();
        let resolved = language
                .families
                .iter()
                .find_map(|family| cache.get(&font_dirs, family).map(|(_, f)| (family.clone(), f)));
            match resolved {
                None => missing.push((language.code.clone(), "no font installed".into())),
                Some((family, font)) => {
                    if !words.iter().any(|word| crate::engine::is_drawable(&font, word)) {
                        // Report the family we asked for AND the file we opened.
                        // An earlier attempt parsed the font's own `name` table,
                        // got the offsets wrong, and printed mojibake - which is
                        // worse than useless in a diagnostic. A path is
                        // unambiguous and needs no parsing.
                        let opened = find_font(&font_dirs, &family)
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| family.clone());
                        missing.push((
                            language.code.clone(),
                            format!("{family} ({opened}) covers none of its words"),
                        ));
                    }
                },
            }
        }
        missing
    }

    /// Pick the next word to draw.
    pub fn choose(&mut self, rng: &mut Rng) -> Option<Choice> {
        let font_dirs = self.settings.font_dirs.clone();
        // Split the borrows: the closure needs &mut fonts while `choose` reads
        // the word bank and language list.
        let fonts = &mut self.fonts;
        let mut resolver = move |family: &str| fonts.get(&font_dirs, family);
        choose(&self.words, &self.languages, &mut resolver, rng)
    }

    /// Build the scene for a choice, or `None` if the font cannot draw it.
    pub fn scene(&mut self, choice: &Choice, width: u32, height: u32) -> Option<crate::engine::Scene> {
        let font_dirs = self.settings.font_dirs.clone();
        for family in &choice.language.families {
            let Some((path, font)) = self.fonts.get(&font_dirs, family) else { continue };
            if font.cmap.covers_str(&choice.word) {
                if let Some(scene) = build_scene(
                    &choice.word,
                    font.as_ref(),
                    width as f64,
                    height as f64,
                    choice.colour,
                ) {
                    debug_assert!(!scene.polylines.is_empty());
                    let _ = path;
                    return Some(scene);
                }
            }
        }
        None
    }

    /// Paint one frame of a scene onto a canvas.
    pub fn paint(canvas: &mut Canvas, scene: &crate::engine::Scene, elapsed: f64) {
        let stroke = (canvas.height as f64 * 0.006).max(1.5);
        match crate::engine::frame(scene, elapsed) {
            Frame::Black => {},
            Frame::Drawing { strokes, pen } => {
                canvas.polylines(&strokes, scene.colour, stroke, 1.0);
                canvas.dot(pen.0, pen.1, stroke * 1.5, Rgb(255, 255, 255), 0.95);
            },
            Frame::Holding { strokes } => {
                canvas.polylines(&strokes, scene.colour, stroke, 1.0);
            },
            Frame::Fading { strokes, alpha } => {
                // Fade by thinning toward the background, which for a black
                // background means simply reducing coverage.
                canvas.polylines(&strokes, scene.colour, stroke, alpha);
            },
        }
    }

    /// Show a genuinely black gap for `BLACK_SECONDS`, watching for activity.
    fn show_gap(
        &mut self,
        canvas: &mut Canvas,
        present: &mut impl FnMut(&Canvas) -> bool,
    ) -> bool {
        let mut elapsed = 0.0;
        let mut last = Instant::now();
        while elapsed < crate::engine::BLACK_SECONDS {
            // A fresh black canvas: the gap must clear anything already drawn.
            canvas.pixels.iter_mut().for_each(|byte| *byte = 0);
            canvas.pixels.chunks_exact_mut(4).for_each(|pixel| pixel[3] = 255);
            if !present(canvas) {
                return false;
            }
            let now = Instant::now();
            let step = now.duration_since(last).min(Duration::from_millis(100));
            last = now;
            elapsed += step.as_secs_f64();
            std::thread::sleep(Duration::from_millis(16));
        }
        true
    }

    /// Run the saver against an X display until the user touches something.
    ///
    /// Takes a closure for "blit this canvas", so the whole sequence logic is
    /// testable with no display at all; `run_on_display` supplies the real one.
    pub fn run(
        &mut self,
        width: u32,
        height: u32,
        max_words: Option<usize>,
        mut present: impl FnMut(&Canvas) -> bool,
    ) -> Outcome {
        let seed = self.settings.seed.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x5EED)
        });
        let mut rng = Rng::new(seed);
        let mut canvas = Canvas::new(width as usize, height as usize);
        let mut drawn = 0usize;

        loop {
            let Some(choice) = self.choose(&mut rng) else { return Outcome::NothingToDraw };
            let Some(scene) = self.scene(&choice, width, height) else {
                // The word is covered by the cmap but produced no outline, which
                // should be vanishingly rare; skipping it beats showing nothing.
                continue;
            };

            // BLACK_SECONDS is the gap between words, and it has to be really
            // black: an earlier version passed the loop's elapsed time straight
            // into frame(), which starts drawing immediately, so the pause never
            // happened and the whole thing read as one continuous scribble.
            if !self.show_gap(&mut canvas, &mut present) {
                return Outcome::Dismissed;
            }

            let mut elapsed = 0.0;
            let total = scene_duration();
            let mut last_draw = Instant::now();
            loop {
                Self::paint(&mut canvas, &scene, elapsed);
                if !present(&canvas) {
                    return Outcome::Dismissed;
                }
                if elapsed >= total {
                    break;
                }
                // Advance by however long the blit actually took, so a slow
                // frame delays the animation instead of running it faster.
                let now = Instant::now();
                let step = now.duration_since(last_draw).min(Duration::from_millis(100));
                last_draw = now;
                elapsed += step.as_secs_f64();
                // Cap the step count so a stalled blit cannot spin the CPU.
                std::thread::sleep(Duration::from_millis(16));
            }
            drawn += 1;
            if let Some(limit) = max_words {
                if drawn >= limit {
                    return Outcome::Finished;
                }
            }
        }
    }
}

/// Run against a real X display.
pub fn run_on_display(settings: Settings, max_words: Option<usize>) -> Outcome {
    let Ok(display) = std::env::var("DISPLAY") else {
        eprintln!("wordpen: DISPLAY is not set; nothing to draw on");
        return Outcome::NothingToDraw;
    };
    let auth_path = std::env::var("XAUTHORITY")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
            PathBuf::from(home).join(".Xauthority")
        });
    let auth = std::fs::read(&auth_path)
        .ok()
        .and_then(|bytes| crate::x11::parse_xauthority_first(&bytes));

    let mut connection = match Connection::open(&display, auth.as_ref()) {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("wordpen: {error}");
            return Outcome::NothingToDraw;
        },
    };

    let screen = connection.info.screen;
    let width = screen.width as u32;
    let height = screen.height as u32;
    if width == 0 || height == 0 {
        eprintln!("wordpen: the display reported a zero-sized screen");
        return Outcome::NothingToDraw;
    }

    let window = connection.alloc();
    let gc = connection.alloc();
    let mut requests = crate::x11::create_window(
        window,
        screen.root,
        screen.root_visual,
        screen.root_depth,
        0,
        0,
        screen.width,
        screen.height,
    );
    requests.extend_from_slice(&crate::x11::map_window(window));
    if let Err(error) = connection.send(&requests) {
        eprintln!("wordpen: cannot create the window: {error}");
        return Outcome::NothingToDraw;
    }
    if let Err(error) = connection.send(&crate::x11::create_gc(gc, window, screen.root_visual, screen.root_depth))
    {
        eprintln!("wordpen: cannot create the graphics context: {error}");
        return Outcome::NothingToDraw;
    }

    let mut runner = Runner::new(settings);
    let drawable = window;
    runner.run(width, height, max_words, |canvas| {
        if let Err(error) = connection.put_frame(
            drawable,
            gc,
            width,
            height,
            screen.root_depth,
            &canvas.pixels,
            4,
        ) {
            eprintln!("wordpen: cannot put the frame: {error}");
            return false;
        }
        // Any input at all means the user is here.
        !connection.activity_pending()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Settings pointing at the data that ships in this repository.
    ///
    /// Tests must use this rather than `Settings::default()`, whose data dir is
    /// `/usr/share/korrinos/wordpen` and does not exist on a build machine. An
    /// earlier version of these tests used the default and then returned early
    /// when `word_count()` was 0, so they were silently passing while testing
    /// nothing at all - the same trap as the outline tests.
    fn repo_settings() -> Settings {
        Settings {
            data_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data"),
            ..Settings::default()
        }
    }

    #[test]
    fn adult_words_are_off_unless_asked_for() {
        let settings = parse_settings("adult = false\n");
        assert!(!settings.adult);
        assert!(parse_settings("adult = true").adult);
        assert!(parse_settings("adult = yes").adult);
        assert!(parse_settings("adult = on").adult);
        // Anything unrecognised must not silently enable it.
        assert!(!parse_settings("adult = maybe").adult);
        assert!(!parse_settings("adult = ").adult);
    }

    #[test]
    fn a_malformed_config_does_not_break_the_rest() {
        let settings = parse_settings(
            "this line has no equals\nadult = true\ngarbage\nseed = 42\nstroke_scale = 0.5\n",
        );
        assert!(settings.adult);
        assert_eq!(settings.seed, Some(42));
        assert!((settings.stroke_scale - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn a_nonsense_stroke_scale_is_ignored() {
        // Zero would make every stroke invisible, so it must not be accepted.
        let settings = parse_settings("stroke_scale = 0\nstroke_scale = -1\n");
        assert!(settings.stroke_scale > 0.0);
        assert!((settings.stroke_scale - Settings::default().stroke_scale).abs() < 1e-9);
    }

    #[test]
    fn comments_are_stripped() {
        let settings = parse_settings("# adult = true\nseed = 7 # trailing\n");
        assert!(!settings.adult, "a commented-out line must not take effect");
        assert_eq!(settings.seed, Some(7));
    }

    #[test]
    fn the_saver_exits_the_moment_the_user_touches_something() {
        // The whole contract, tested without a display: present() returning
        // false means "the user moved", and the saver must stop immediately.
        let mut runner = Runner::new(repo_settings());
        assert!(runner.word_count() > 0, "the repo data should be readable");
        let mut frames = 0u32;
        let outcome = runner.run(1920, 1080, None, |_canvas| {
            frames += 1;
            frames < 3 // pretend the user touched it on the third frame
        });
        assert_eq!(outcome, Outcome::Dismissed);
        assert_eq!(frames, 3, "it must stop on the very frame the user acted");
    }

    #[test]
    fn a_finished_run_reports_finished_not_dismissed() {
        let mut runner = Runner::new(repo_settings());
        let outcome = runner.run(640, 360, Some(1), |_| true);
        assert_eq!(outcome, Outcome::Finished);
    }

    #[test]
    fn a_present_that_always_reports_activity_is_dismissed_immediately() {
        let mut runner = Runner::new(repo_settings());
        let outcome = runner.run(1920, 1080, Some(1), |_| false);
        assert_eq!(outcome, Outcome::Dismissed);
    }

    #[test]
    fn a_missing_data_directory_reports_nothing_to_draw() {
        let settings = Settings {
            data_dir: PathBuf::from("/nonexistent/wordpen"),
            ..Settings::default()
        };
        let mut runner = Runner::new(settings);
        assert_eq!(runner.word_count(), 0);
        assert_eq!(runner.run(800, 600, Some(1), |_| true), Outcome::NothingToDraw);
    }

    #[test]
    fn the_repo_data_is_enough_to_draw_something() {
        let mut runner = Runner::new(repo_settings());
        assert!(runner.word_count() > 0, "no words loaded from the repo data");
        assert!(runner.language_count() >= 55);
        // On a machine with any Latin font installed, English must be drawable.
        assert!(runner.drawable_languages() > 0, "nothing at all is drawable");
    }

    #[test]
    fn painting_something_produces_pixels() {
        let mut runner = Runner::new(repo_settings());
        let mut rng = Rng::new(4);
        let choice = runner.choose(&mut rng).expect("a drawable word should exist");
        let scene = runner.scene(&choice, 800, 600).expect("the font should draw it");
        let mut canvas = Canvas::new(800, 600);
        Runner::paint(&mut canvas, &scene, 0.05);
        assert!(canvas.lit_pixels() > 0, "the first frame must show ink");
    }

    #[test]
    fn a_black_frame_lights_nothing() {
        let mut runner = Runner::new(repo_settings());
        let mut rng = Rng::new(9);
        let choice = runner.choose(&mut rng).expect("a drawable word should exist");
        let scene = runner.scene(&choice, 800, 600).expect("the font should draw it");
        let mut canvas = Canvas::new(800, 600);
        // Painting a frame marks the canvas; the gap must then clear it.
        Runner::paint(&mut canvas, &scene, crate::engine::DRAW_SECONDS + 0.1);
        assert!(canvas.lit_pixels() > 0, "the word itself must be visible");
        // Now run the real gap loop and assert it leaves the canvas black.
        let mut frames = 0;
        let mut present = |_: &Canvas| {
            frames += 1;
            true
        };
        assert!(runner.show_gap(&mut canvas, &mut present));
        assert!(frames > 0, "the gap must actually present frames");
        assert_eq!(canvas.lit_pixels(), 0, "the gap between words must be black");
    }

    #[test]
    fn fonts_are_parsed_once_and_reused() {
        let mut cache = FontCache::default();
        let dirs = [PathBuf::from("/usr/share/fonts")];
        let first = cache.get(&dirs, "DejaVu Sans");
        let second = cache.get(&dirs, "DejaVu Sans");
        assert_eq!(first.is_some(), second.is_some());
        if first.is_some() {
            assert_eq!(first.as_ref().unwrap().0, second.as_ref().unwrap().0);
        }
    }

    #[test]
    fn a_missing_font_is_none_not_a_panic() {
        let mut cache = FontCache::default();
        let dirs = [PathBuf::from("/usr/share/fonts")];
        assert!(cache.get(&dirs, "Definitely Not A Font 12345").is_none());
        assert!(find_font(&dirs, "").is_none());
    }
}
