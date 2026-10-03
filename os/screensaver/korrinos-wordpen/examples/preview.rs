//! Render frames to PNG so the animation can be looked at without a display.
//!
//! A build machine has no screen, so the only way to know the pen is actually
//! writing readable letters is to put the pixels in a file and open it.

use std::path::Path;
use wordpen::engine::{build_scene, Rgb, Frame};
use wordpen::outline::Font;
use wordpen::render::Canvas;
use wordpen::words::{load_languages, read_optional};

fn main() {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
    let languages = load_languages(&read_optional(&data.join("languages.tsv")));
    let words = wordpen::words::load_words(&read_optional(&data.join("words.tsv")));

    let out = std::env::args().nth(1).unwrap_or_else(|| "/tmp/wordpen".to_string());
    std::fs::create_dir_all(&out).expect("output dir");

    // One language per invocation keeps the preview readable.
    let want = std::env::args().nth(2).unwrap_or_else(|| "en".to_string());
    let Some(language) = languages.iter().find(|l| l.code == want) else {
        eprintln!("no language {want}");
        return;
    };
    let Some(word) = words.get(&want).and_then(|w| w.first()) else {
        eprintln!("no words for {want}");
        return;
    };

    // Find a font that actually covers the word: same rule as the saver.
    let mut font = None;
    for family in &language.families {
        for dir in ["/usr/share/fonts", "/usr/local/share/fonts"] {
            if let Some(path) = find_font(Path::new(dir), family) {
                if let Ok(bytes) = std::fs::read(&path) {
                    if let Some(parsed) = Font::parse(bytes) {
                        if parsed.cmap.covers_str(word) {
                            font = Some((path.display().to_string(), parsed));
                            break;
                        }
                    }
                }
            }
        }
        if font.is_some() { break; }
    }
    let Some((path, font)) = font else {
        eprintln!("no installed font covers {word:?}");
        return;
    };
    println!("word={word:?} language={} font={path}", language.english);

    let (w, h) = (1280u32, 720u32);
    let scene = build_scene(word, &font, w as f64, h as f64, Rgb(120, 230, 255))
        .expect("scene");
    let stroke = (h as f64 / 90.0).max(2.0);

    for (label, elapsed) in [
        ("01-start", 0.15),
        ("02-third", wordpen::engine::DRAW_SECONDS * 0.35),
        ("02b-half", wordpen::engine::DRAW_SECONDS * 0.55),
        ("03-most", wordpen::engine::DRAW_SECONDS * 0.85),
        ("04-done", wordpen::engine::DRAW_SECONDS + 0.2),
    ] {
        let mut canvas = Canvas::new(w as usize, h as usize);
        let frame = wordpen::engine::frame(&scene, elapsed);
        match frame {
            Frame::Drawing { strokes, pen } => {
                canvas.polylines(&strokes, scene.colour, stroke, 1.0);
                canvas.dot(pen.0, pen.1, stroke * 1.6, Rgb(255, 255, 255), 0.95);
            },
            Frame::Holding { strokes } | Frame::Fading { strokes, .. } => {
                canvas.polylines(&strokes, scene.colour, stroke, 1.0);
            },
            Frame::Black => {},
        }
        let file = format!("{out}/{want}-{label}.png");
        std::fs::write(&file, canvas.to_png()).expect("write png");
        println!("  wrote {file} ({} px lit)", canvas.lit_pixels());
    }
}

fn find_font(dir: &Path, family: &str) -> Option<std::path::PathBuf> {
    let needle: String = family
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    // Collect every candidate with a rank, then take the best. Ranking is what
    // makes the FIRST family in a language's chain actually win: a loose
    // substring match would otherwise drag in any font that mentions the name.
    let mut best: Option<(u8, std::path::PathBuf)> = None;
    fn walk(dir: &Path, needle: &str, depth: usize, best: &mut Option<(u8, std::path::PathBuf)>) {
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
            let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
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
            } else if stem.starts_with(&needle) {
                1
            } else if stem.contains(&needle) {
                2
            } else {
                continue;
            };
            let better = best.as_ref().is_none_or(|(r, _)| rank < *r);
            if better {
                *best = Some((rank, path.clone()));
            }
        }
    }
    walk(dir, &needle, 0, &mut best);
    best.map(|(_, p)| p)
}
