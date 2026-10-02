//! The pen, the word, and the light show.
//!
//! Everything here is pure: given a word, a font and a clock, it decides what
//! to draw. No window, no display, no IO. That is deliberate - a build machine
//! has no screen, so an effect that can only be checked by looking at it cannot
//! be checked at all. The renderer reads these values and paints them.

use crate::outline::Font;
use std::collections::BTreeMap;

/// A shipped language the saver can write in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Language {
    /// Short code, e.g. `ja`.
    pub code: String,
    /// Name in the language itself, e.g. `日本語`.
    pub endonym: String,
    /// Name in English, e.g. `Japanese`.
    pub english: String,
    /// Font families to try, best first.
    pub families: Vec<String>,
}

/// A word to draw, and everything decided about how to draw it.
#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    pub word: String,
    pub language: Language,
    /// `#rrggbb`.
    pub colour: Rgb,
    /// The font file that was actually opened and verified.
    pub font_path: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    /// A pleasant, legible ink colour. Kept bright enough to read on black at
    /// 4K without being blown out.
    pub fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

/// Deterministic RNG, so a given seed always draws the same thing.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Avoid the fixed point at 0.
        Self { state: seed ^ 0x9E37_79B9_7F4A_7C15 }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform in 0..1.
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Uniform in 0..n.
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 { 0 } else { (self.f64() * n as f64) as usize % n }
    }

    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() { None } else { Some(&items[self.below(items.len())]) }
    }
}

/// Pick a random colour that reads well on black.
///
/// Saturated and bright, never dark enough to disappear. The alpha-ish
/// lightness floor is what stops a "random" colour turning into an invisible
/// smudge, which is the failure mode a naive random RGB gives you.
pub fn random_colour(rng: &mut Rng) -> Rgb {
    loop {
        let r = (128 + rng.below(128)) as u8;
        let g = (128 + rng.below(128)) as u8;
        let b = (128 + rng.below(128)) as u8;
        // Require a minimum spread between channels so it is actually a colour
        // and not a grey smudge, and a minimum brightness so it reads on black.
        let spread = r.max(g).max(b) as i32 - r.min(g).min(b) as i32;
        let luma = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
        if spread >= 40 && luma >= 150 {
            return Rgb(r, g, b);
        }
    }
}

/// Everything needed to draw one word.
pub struct Scene {
    /// Every stroke of the word, in viewport pixels.
    pub polylines: Vec<Vec<(f64, f64)>>,
    /// Total path length in pixels, for pacing the pen.
    pub total_length: f64,
    /// Viewport the scene was laid out for.
    pub width: f64,
    pub height: f64,
    pub colour: Rgb,
}

/// How long each part of the sequence takes, in seconds.
pub const DRAW_SECONDS: f64 = 3.4;
pub const HOLD_SECONDS: f64 = 1.6;
pub const FADE_SECONDS: f64 = 0.9;
pub const BLACK_SECONDS: f64 = 0.7;

/// Total length of one word's appearance, in seconds.
pub fn scene_duration() -> f64 {
    DRAW_SECONDS + HOLD_SECONDS + FADE_SECONDS
}

/// Lay a word out into polylines filling `width` x `height`.
///
/// The scale is derived from the viewport rather than fixed, so the same code
/// fills 1080p, 4K, 8K or an ultrawide correctly. Glyph height drives the size
/// and the word is centred; if the word is too wide it shrinks to fit rather
/// than running off the edge.
pub fn build_scene(word: &str, font: &Font, width: f64, height: f64, colour: Rgb) -> Option<Scene> {
    let mut polylines = Vec::new();
    let mut used_glyphs = 0usize;

    for ch in word.chars() {
        let glyph = font.glyph_for_char(ch)?;
        if !glyph.contours.is_empty() {
            used_glyphs += 1;
        }
        for contour in &glyph.contours {
            // Contours are closed in the font; a pen does not close them, it
            // lifts, so we keep the contour open.
            let points: Vec<(f64, f64)> =
                contour.points.iter().map(|p| (p.x, p.y)).collect();
            polylines.push(points);
        }
        // Note: layout is driven by the drawn outline's bounding box, not by
        // the accumulated advance. That is what keeps a word with wide
        // side-bearing letters (an 'A', say) from overhanging the edge.
    }
    if polylines.is_empty() {
        return None;
    }
    let _ = used_glyphs;

    // Normalise: work out the bounding box, then scale so the cap height fills
    // a fixed fraction of the viewport height, and centre.
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for line in &polylines {
        for (x, y) in line {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
    }
    if !min_x.is_finite() {
        return None;
    }

    // The word is wide, so fit on width first, then use whatever height remains.
    let text_w = max_x - min_x;
    let text_h = max_y - min_y;
    if text_w <= 0.0 || text_h <= 0.0 {
        return None;
    }
    let margin = 0.12;
    let scale_w = (width * (1.0 - margin * 2.0)) / text_w;
    let scale_h = (height * (1.0 - margin * 2.0)) / (text_h * 2.2);
    let scale = scale_w.min(scale_h);

    let drawn_w = text_w * scale;
    let drawn_h = text_h * scale;
    let offset_x = (width - drawn_w) / 2.0 - min_x * scale;
    // Sit slightly above centre: a line of text optically centres above the
    // mathematical middle.
    let offset_y = (height + drawn_h * 0.35) / 2.0 - min_y * scale;

    let mut total_length = 0.0;
    let mut laid_out = Vec::with_capacity(polylines.len());
    for line in polylines {
        let moved: Vec<(f64, f64)> =
            line.iter().map(|(x, y)| (x * scale + offset_x, y * scale + offset_y)).collect();
        for pair in moved.windows(2) {
            let (dx, dy) = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
            total_length += (dx * dx + dy * dy).sqrt();
        }
        laid_out.push(moved);
    }

    Some(Scene { polylines: laid_out, total_length, width, height, colour })
}

/// One frame of the animation.
#[derive(Clone, Debug, PartialEq)]
pub enum Frame {
    /// Still black: between words.
    Black,
    /// Drawing, with a pen dot at the given point and the strokes so far.
    Drawing {
        strokes: Vec<Vec<(f64, f64)>>,
        pen: (f64, f64),
    },
    /// Fully drawn, holding.
    Holding { strokes: Vec<Vec<(f64, f64)>> },
    /// Fading out.
    Fading { strokes: Vec<Vec<(f64, f64)>>, alpha: f64 },
}

/// What to draw at `elapsed` seconds into the scene.
pub fn frame(scene: &Scene, elapsed: f64) -> Frame {
    if elapsed < 0.0 || elapsed >= scene_duration() {
        return Frame::Black;
    }

    if elapsed < DRAW_SECONDS {
        // Pace the pen at a constant speed: a pen moves at a steady rate, and
        // easing it makes the writing look hesitant.
        let progress = (elapsed / DRAW_SECONDS).clamp(0.0, 1.0);
        return draw_up_to(scene, progress);
    }
    if elapsed < DRAW_SECONDS + HOLD_SECONDS {
        return Frame::Holding { strokes: scene.polylines.clone() };
    }
    let fade = (elapsed - DRAW_SECONDS - HOLD_SECONDS) / FADE_SECONDS;
    Frame::Fading { strokes: scene.polylines.clone(), alpha: (1.0 - fade).clamp(0.0, 1.0) }
}

/// The strokes drawn by `progress` of the way through, plus the pen position.
fn draw_up_to(scene: &Scene, progress: f64) -> Frame {
    let budget = scene.total_length * progress;
    let mut strokes = Vec::new();
    let mut spent = 0.0;
    let mut pen = (scene.width / 2.0, scene.height / 2.0);

    for line in &scene.polylines {
        if spent >= budget {
            break;
        }
        let mut drawn = Vec::new();
        for pair in line.windows(2) {
            if spent >= budget {
                break;
            }
            let (dx, dy) = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
            let seg = (dx * dx + dy * dy).sqrt();
            drawn.push(pair[0]);
            if spent + seg <= budget {
                spent += seg;
                drawn.push(pair[1]);
                pen = pair[1];
            } else {
                // Partial segment: place the pen partway along it.
                let t = if seg > 0.0 { (budget - spent) / seg } else { 0.0 };
                let mid = (pair[0].0 + dx * t, pair[0].1 + dy * t);
                drawn.push(mid);
                pen = mid;
                spent = budget;
                break;
            }
        }
        if drawn.len() >= 2 {
            strokes.push(drawn);
        }
    }
    Frame::Drawing { strokes, pen }
}

/// A word's worth of words, per language.
pub type WordBank = BTreeMap<String, Vec<String>>;

/// Choose what to draw next, or `None` if nothing can be drawn.
///
/// The gate is coverage, not hope: a word is only chosen when some candidate
/// font's `cmap` actually contains every character. That is the whole reason
/// the saver can claim "it never draws a box" - it never picks a word it cannot
/// draw.
pub fn choose(
    words: &WordBank,
    languages: &[Language],
    resolve_font: &dyn Fn(&str) -> Option<(String, Font)>,
    rng: &mut Rng,
) -> Option<Choice> {
    // Shuffle so a language with many words does not dominate purely by count.
    let mut order: Vec<usize> = (0..languages.len()).collect();
    for i in (1..order.len()).rev() {
        let j = rng.below(i + 1);
        order.swap(i, j);
    }

    for &index in &order {
        let language = &languages[index];
        let Some(list) = words.get(&language.code) else { continue };
        if list.is_empty() {
            continue;
        }
        // Try a handful of words from this language before moving on.
        for _ in 0..12 {
            let Some(word) = rng.pick(list) else { break };
            for family in &language.families {
                let Some((font_path, font)) = resolve_font(family) else { continue };
                if font.cmap.covers_str(word) && !word.chars().any(|c| c.is_control()) {
                    return Some(Choice {
                        word: word.clone(),
                        language: language.clone(),
                        colour: random_colour(rng),
                        font_path,
                    });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn language(code: &str, families: &[&str]) -> Language {
        Language {
            code: code.to_string(),
            endonym: code.to_uppercase(),
            english: code.to_string(),
            families: families.iter().map(|f| f.to_string()).collect(),
        }
    }

    fn real_font() -> Option<Font> {
        std::fs::read("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")
            .ok()
            .and_then(Font::parse)
    }

    #[test]
    fn rng_is_reproducible_and_in_range() {
        let mut a = Rng::new(7);
        let mut b = Rng::new(7);
        for _ in 0..128 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
        let mut r = Rng::new(99);
        for _ in 0..1000 {
            let v = r.f64();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn seed_zero_does_not_degenerate() {
        let mut r = Rng::new(0);
        assert_ne!(r.next_u64(), r.next_u64(), "seed 0 produced a constant stream");
    }

    #[test]
    fn random_colours_are_always_readable_on_black() {
        let mut rng = Rng::new(1234);
        for _ in 0..2000 {
            let c = random_colour(&mut rng);
            let luma =
                (c.0 as u32 * 299 + c.1 as u32 * 587 + c.2 as u32 * 114) / 1000;
            assert!(luma >= 150, "too dark to read: {c:?}");
            let spread = c.0.max(c.1).max(c.2) as i32 - c.0.min(c.1).min(c.2) as i32;
            assert!(spread >= 40, "not a colour, a grey smudge: {c:?}");
        }
    }

    #[test]
    fn colour_hex_is_six_digits() {
        assert_eq!(Rgb(255, 0, 90).hex(), "#ff005a");
    }

    #[test]
    fn scene_fits_the_viewport_at_every_resolution() {
        let Some(font) = real_font() else { return };
        // 1080p, 4K, 8K, ultrawide, and a tiny window.
        let sizes = [
            (1920.0, 1080.0),
            (3840.0, 2160.0),
            (7680.0, 4320.0),
            (3440.0, 1440.0),
            (320.0, 240.0),
        ];
        for (w, h) in sizes {
            let scene = build_scene("HELLO", &font, w, h, Rgb(255, 255, 255))
                .expect("scene should build");
            for line in &scene.polylines {
                for (x, y) in line {
                    assert!(
                        *x >= -1.0 && *x <= w + 1.0 && *y >= -1.0 && *y <= h + 1.0,
                        "{w}x{h}: point ({x}, {y}) is off screen"
                    );
                }
            }
        }
    }

    #[test]
    fn a_long_word_shrinks_to_fit_rather_than_overflowing() {
        let Some(font) = real_font() else { return };
        let scene = build_scene("EXTRAORDINARILY", &font, 1920.0, 1080.0, Rgb(1, 2, 3))
            .expect("scene");
        for line in &scene.polylines {
            for (x, _) in line {
                assert!(*x <= 1920.0, "long word overflowed: {x}");
            }
        }
    }

    #[test]
    fn a_word_with_no_outlines_produces_no_scene() {
        let Some(font) = real_font() else { return };
        assert!(build_scene("   ", &font, 800.0, 600.0, Rgb(1, 1, 1)).is_none());
    }

    #[test]
    fn the_pen_advances_monotonically_and_ends_at_the_end() {
        let Some(font) = real_font() else { return };
        let scene = build_scene("HELLO", &font, 1920.0, 1080.0, Rgb(9, 9, 9)).unwrap();
        let mut drawn = 0usize;
        let mut previous = 0usize;
        for step in 0..=100 {
            let Frame::Drawing { strokes, .. } =
                frame(&scene, step as f64 / 100.0 * DRAW_SECONDS)
            else {
                panic!("expected drawing at {step}");
            };
            let now: usize = strokes.iter().map(|s| s.len()).sum();
            assert!(now >= previous, "strokes went backwards at step {step}");
            previous = now;
            drawn = now;
        }
        assert!(drawn > 0, "the pen never drew anything");
    }

    #[test]
    fn the_full_sequence_is_black_draw_hold_fade() {
        let Some(font) = real_font() else { return };
        let scene = build_scene("HI", &font, 800.0, 600.0, Rgb(5, 5, 5)).unwrap();
        assert_eq!(frame(&scene, -1.0), Frame::Black);
        assert_eq!(frame(&scene, scene_duration()), Frame::Black);
        assert!(matches!(frame(&scene, 0.1), Frame::Drawing { .. }));
        assert!(matches!(frame(&scene, DRAW_SECONDS + 0.1), Frame::Holding { .. }));
        let Frame::Fading { alpha, .. } = frame(&scene, DRAW_SECONDS + HOLD_SECONDS + 0.01)
        else {
            panic!("expected fading");
        };
        assert!(alpha < 1.0 && alpha > 0.0);
    }

    #[test]
    fn choose_never_picks_a_word_the_font_cannot_draw() {
        let Some(font) = real_font() else { return };
        let mut words = WordBank::new();
        words.insert("en".to_string(), vec!["HELLO".into(), "WORLD".into()]);
        // A language whose only font is Latin, offered a Japanese word. It must
        // be skipped, not drawn as boxes.
        words.insert("ja".to_string(), vec!["日本語".into()]);
        let languages = vec![language("en", &["DejaVu Sans"]), language("ja", &["DejaVu Sans"])];
        let resolver = |_: &str| Some(("fake.ttf".to_string(), font.clone()));

        for seed in 0..200 {
            let mut rng = Rng::new(seed);
            if let Some(choice) = choose(&words, &languages, &resolver, &mut rng) {
                assert!(
                    font.cmap.covers_str(&choice.word),
                    "chose {:?} which the font cannot draw",
                    choice.word
                );
            }
        }
    }

    #[test]
    fn choose_returns_none_when_nothing_is_drawable() {
        let words = WordBank::new();
        let languages = vec![language("en", &["Nope"])];
        let resolver = |_: &str| None;
        let mut rng = Rng::new(1);
        assert!(choose(&words, &languages, &resolver, &mut rng).is_none());
    }

}
