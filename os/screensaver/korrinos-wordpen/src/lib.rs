//! KorrinOS `wordpen` — a handwriting screen saver.
//!
//! The screen goes black. A pen, in a random bright colour, writes one word in
//! one of the languages KorrinOS ships. It holds, fades, and then it is black
//! again until the next word.
//!
//! # The rule this crate is built around
//!
//! **It never draws a box.** A screensaver that renders a word in a script the
//! font cannot draw shows the user a row of tofu and looks broken. So a word is
//! only ever chosen after the font's `cmap` has confirmed, character by
//! character, that the glyphs exist. That check is in [`engine::choose`] and it
//! is what the rest of the design is arranged around.
//!
//! # Layout of the crate
//!
//! - [`cmap`] — which codepoints a font has. The gate.
//! - [`outline`] — glyph outlines as polylines, read from the font's `glyf`.
//!   Extracting real letterforms is what lets one code path draw all 55
//!   languages, instead of hand-authoring strokes per script.
//! - [`engine`] — the pure part: word choice, colour, layout, and the frame
//!   timeline. No IO, no window, fully testable without a display.
//! - [`render`] — rasterises strokes and writes PNG, so a frame can be looked at
//!   even on a machine with no screen.
//!
//! # What it is not
//!
//! There is no shaper here. Glyphs are drawn in codepoint order, one per
//! character, with no reordering and no ligatures. For Arabic and the Indic
//! scripts that means isolated forms laid out left to right, which reads as an
//! alphabet rather than as running text. That is a stated limit rather than a
//! silent bug: correct shaping is HarfBuzz, and pulling it in would dwarf this
//! crate. Drawing the alphabet correctly in every script is worth far more than
//! drawing running text correctly in one.

pub mod cmap;
pub mod engine;
pub mod outline;
pub mod render;
pub mod screen;
pub mod x11;
pub mod words;

pub use engine::{Choice, Frame, Language, Rgb, Rng, Scene, WordBank};
pub use outline::Font;
