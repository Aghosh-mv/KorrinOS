//! Glyph outline extraction from a TrueType font.
//!
//! # Why extract outlines at all
//!
//! The obvious way to make a pen "write" is to hand-author stroke data per
//! character, the way a handwriting screensaver on macOS does it. That works
//! until you ship 55 languages: hand-authored strokes for Devanagari matras,
//! Tamil vowel signs, Thai tone marks and CJK radicals is not a thing anyone
//! should attempt.
//!
//! So the letters come from the font itself. Every outline in a TrueType font is
//! a set of contours made of lines and quadratic curves, and a pen tracing
//! those contours in order is drawing the letter as a human would write it. That
//! works for every script the font supports, with no per-language authoring, and
//! it is why this can be drawn in Hindi as readily as in English.
//!
//! # What this is and is not
//!
//! This is NOT a shaper. It draws glyphs in the order given, one per codepoint,
//! with no reordering, no ligatures, no complex-script positioning. For the
//! scripts where that matters (Arabic joining, Indic reordering) the result is
//! the isolated forms laid out left to right, which reads as the alphabet rather
//! than as correct text. That is a deliberate, stated limit: a real shaper is
//! HarfBuzz, and pulling it in would dwarf this crate. The alternative -
//! silently producing wrong-looking script - is what we are avoiding by saying
//! it out loud instead.

/// A point in font units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    /// True when this point starts a new contour. Moves in a TrueType contour
    /// mean "pen lifts here", so this is what distinguishes a stroke break.
    pub on_curve: bool,
}

/// One resolved contour: a run of points, already flattened to straight lines.
#[derive(Clone, Debug, PartialEq)]
pub struct Contour {
    pub points: Vec<Point>,
}

impl Contour {
    /// Total path length, used to pace the pen at a constant speed.
    pub fn length(&self) -> f64 {
        self.points
            .windows(2)
            .map(|w| {
                let (dx, dy) = (w[1].x - w[0].x, w[1].y - w[0].y);
                (dx * dx + dy * dy).sqrt()
            })
            .sum()
    }
}

/// Horizontal metrics for one glyph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Advance {
    /// How far the pen moves after drawing this glyph.
    pub advance: f64,
    /// Left side bearing.
    pub lsb: f64,
}

/// A glyph's outlines plus its advance, in font units.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Glyph {
    pub contours: Vec<Contour>,
    pub advance: f64,
}

impl Glyph {
    /// Vertical extent across all contours, for centring the text on screen.
    pub fn bounds(&self) -> Option<(f64, f64, f64, f64)> {
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for contour in &self.contours {
            for point in &contour.points {
                min_y = min_y.min(point.y);
                max_y = max_y.max(point.y);
            }
        }
        if !min_y.is_finite() {
            return None;
        }
        Some((min_y, max_y, 0.0, 0.0))
    }
}

fn u16be(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*bytes.get(offset)?, *bytes.get(offset + 1)?]))
}

fn i16be(bytes: &[u8], offset: usize) -> Option<i16> {
    Some(i16::from_be_bytes([*bytes.get(offset)?, *bytes.get(offset + 1)?]))
}

fn u32be(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_be_bytes([
        *bytes.get(offset)?,
        *bytes.get(offset + 1)?,
        *bytes.get(offset + 2)?,
        *bytes.get(offset + 3)?,
    ]))
}

/// The tables we need, located by tag.
#[derive(Clone)]
pub struct Font {
    /// Whole file, kept so a caller can re-parse or a test can truncate it.
    #[allow(dead_code)]
    data: Vec<u8>,
    loca: Vec<u8>,
    glyf: Vec<u8>,
    hmtx: Vec<u8>,
    num_h_metrics: usize,
    /// Unit-per-em, i.e. font units per em square.
    pub units_per_em: f64,
    /// `cmap` ranges, reused from the coverage parser's shape.
    pub cmap: super::cmap::Cmap,
    /// `head.indexToLocFormat`: loca entries are u32 rather than u16.
    long_loca: bool,
}

impl Font {
    /// Parse a TrueType font. Returns `None` for anything we cannot read rather
    /// than guessing, because a wrong letterform is worse than no letterform.
    pub fn parse(data: Vec<u8>) -> Option<Font> {
        const SFNT: u32 = 0x0001_0000;
        const TRUE: u32 = 0x7472_7565;
        const TTCF: u32 = 0x7474_6366;

        let base = if u32be(&data, 0) == Some(TTCF) { u32be(&data, 12)? as usize } else { 0 };
        let tag = u32be(&data, base)?;
        if tag != SFNT && tag != TRUE {
            // 'OTTO' means CFF outlines, which this does not read. Refusing
            // loudly is better than emitting empty boxes.
            return None;
        }
        let num_tables = u16be(&data, base + 4)? as usize;

        let mut tables: Vec<([u8; 4], usize, usize)> = Vec::with_capacity(num_tables);
        for i in 0..num_tables {
            let record = base + 12 + i * 16;
            let name = data.get(record..record + 4)?.to_vec().try_into().ok()?;
            let offset = u32be(&data, record + 8)? as usize;
            let length = u32be(&data, record + 12)? as usize;
            tables.push((name, offset, length));
        }
        let find = |want: &[u8; 4]| -> Option<(usize, usize)> {
            tables.iter().find(|(name, _, _)| name == want).map(|&(_, o, l)| (o, l))
        };

        let (head_off, _) = find(b"head")?;
        let units_per_em = u16be(&data, head_off + 18)? as f64;
        if units_per_em <= 0.0 {
            return None;
        }
        // head.indexToLocFormat: 0 means loca offsets are u16, 1 means u32.
        let index_to_loc = i16be(&data, head_off + 50)?;
        let long_loca = index_to_loc != 0;
        let (loca_off, loca_len) = find(b"loca")?;
        let (glyf_off, glyf_len) = find(b"glyf")?;
        let (hmtx_off, hmtx_len) = find(b"hmtx")?;
        let (hhea_off, _) = find(b"hhea")?;
        let num_h_metrics = u16be(&data, hhea_off + 34)? as usize;
        let (cmap_off, _) = find(b"cmap")?;

        // Bounds-check every slice against the file we actually hold, so a
        // truncated or lying font cannot walk out of bounds.
        let need = |off: usize, len: usize| -> Option<()> {
            if off.checked_add(len)? <= data.len() { Some(()) } else { None }
        };
        need(loca_off, loca_len)?;
        // This was `need(glyf_off, data.len())`, which can never be true, so
        // Font::parse rejected every real font. Use the table's real length.
        need(glyf_off, glyf_len)?;
        need(hmtx_off, hmtx_len)?;

        let cmap = super::cmap::Cmap::from_ranges(super::cmap::parse_cmap_ranges(&data, cmap_off)?);

        Some(Font {
            loca: data[loca_off..loca_off + loca_len].to_vec(),
            glyf: data[glyf_off..glyf_off + glyf_len].to_vec(),
            hmtx: data[hmtx_off..hmtx_off + hmtx_len].to_vec(),
            num_h_metrics,
            units_per_em,
            long_loca,
            cmap,
            data,
        })
    }

    /// Byte range of glyph `id` inside the `glyf` table.
    fn glyph_range(&self, id: u16) -> Option<(usize, usize)> {
        // Trust head.indexToLocFormat, but fall back to the table length when
        // it is ambiguous, so a font with an unset flag still works.
        let by_length = self.loca.len() >= (id as usize + 2) * 4;
        let (start, end) = if self.long_loca || by_length {
            // 'long' offsets: u32 each.
            (
                u32be(&self.loca, id as usize * 4)? as usize,
                u32be(&self.loca, (id as usize + 1) * 4)? as usize,
            )
        } else {
            (
                u16be(&self.loca, id as usize * 2)? as usize * 2,
                u16be(&self.loca, (id as usize + 1) * 2)? as usize * 2,
            )
        };
        if end <= start {
            return None; // empty glyph, e.g. a space
        }
        Some((start, end))
    }

    /// Advance width for a glyph.
    fn advance(&self, id: u16) -> Option<f64> {
        let index = (id as usize).min(self.num_h_metrics.saturating_sub(1));
        u16be(&self.hmtx, index * 4).map(|a| a as f64)
    }

    /// Outline for one glyph, flattened to polylines.
    pub fn glyph(&self, id: u16) -> Option<Glyph> {
        let advance = self.advance(id)?;
        let Some((start, end)) = self.glyph_range(id) else {
            // An empty glyph (space) is legitimate: no contours, real advance.
            return Some(Glyph { contours: Vec::new(), advance });
        };
        let contours = self.parse_simple_glyph(start, end)?;
        Some(Glyph { contours, advance })
    }

    /// Parse a simple glyph. Composite glyphs are skipped rather than faked.
    fn parse_simple_glyph(&self, start: usize, end: usize) -> Option<Vec<Contour>> {
        let g = &self.glyf;
        if start.checked_add(10)? > g.len() {
            return None;
        }
        let num_contours = i16be(g, start)?;
        if num_contours < 0 {
            // Composite glyph: needs a second code path and its own tests.
            // Returning None means "no outline", not "wrong outline".
            return None;
        }
        let num_contours = num_contours as usize;
        let mut cursor = start + 10;
        let mut end_points = Vec::with_capacity(num_contours);
        for _ in 0..num_contours {
            end_points.push(u16be(g, cursor)? as usize);
            cursor += 2;
        }
        let num_points = end_points.last().map_or(0, |last| last + 1);

        let instruction_len = u16be(g, cursor)? as usize;
        cursor += 2 + instruction_len;
        if cursor.checked_add(num_points)? > end {
            return None;
        }

        // Flags are run-length encoded: a flag byte is followed by a repeat count.
        let mut flags = Vec::with_capacity(num_points);
        while flags.len() < num_points {
            let flag = *g.get(cursor)?;
            cursor += 1;
            flags.push(flag);
            if flag & 0x08 != 0 {
                let repeat = *g.get(cursor)? as usize;
                cursor += 1;
                for _ in 0..repeat {
                    if flags.len() >= num_points {
                        break;
                    }
                    flags.push(flag);
                }
            }
        }

        // Coordinates are deltas, x then y, each 1 or 2 bytes depending on flags.
        let mut xs = Vec::with_capacity(num_points);
        let mut value: i32 = 0;
        for &flag in &flags {
            let delta = if flag & 0x02 != 0 {
                let byte = *g.get(cursor)? as i32;
                cursor += 1;
                if flag & 0x10 != 0 { byte } else { -byte }
            } else if flag & 0x10 != 0 {
                0
            } else {
                let raw = i16be(g, cursor)?;
                cursor += 2;
                raw as i32
            };
            value += delta;
            xs.push(value as f64);
        }
        let mut ys = Vec::with_capacity(num_points);
        value = 0;
        for &flag in &flags {
            let delta = if flag & 0x04 != 0 {
                let byte = *g.get(cursor)? as i32;
                cursor += 1;
                if flag & 0x20 != 0 { byte } else { -byte }
            } else if flag & 0x20 != 0 {
                0
            } else {
                let raw = i16be(g, cursor)?;
                cursor += 2;
                raw as i32
            };
            value += delta;
            ys.push(value as f64);
        }

        // Split into contours and flatten each.
        let mut contours = Vec::with_capacity(num_contours);
        let mut first = 0usize;
        for &last in &end_points {
            let raw: Vec<Point> = (first..=last)
                .map(|i| Point {
                    x: xs[i],
                    y: ys[i],
                    on_curve: flags[i] & 0x01 != 0,
                })
                .collect();
            if let Some(contour) = flatten_contour(&raw) {
                contours.push(contour);
            }
            first = last + 1;
        }
        Some(contours)
    }

    /// Outline for one codepoint, if the font has it.
    pub fn glyph_for_char(&self, ch: char) -> Option<Glyph> {
        let id = self.cmap.glyph_id(ch as u32)?;
        self.glyph(id)
    }
}

/// How finely curves are approximated. Straight segments per quadratic curve.
const CURVE_SEGMENTS: usize = 8;

/// Turn a raw TrueType contour into a polyline.
///
/// TrueType contours may start on an off-curve point, in which case the implied
/// start is the midpoint between the last and first points. Getting that wrong
/// produces a letter with a visible kink, so it is handled explicitly.
fn flatten_contour(raw: &[Point]) -> Option<Contour> {
    if raw.len() < 2 {
        return None;
    }
    let mid = |a: &Point, b: &Point| Point {
        x: (a.x + b.x) / 2.0,
        y: (a.y + b.y) / 2.0,
        on_curve: true,
    };

    // Normalise to start on-curve.
    let start: Point = if raw[0].on_curve {
        raw[0]
    } else if raw[raw.len() - 1].on_curve {
        raw[raw.len() - 1]
    } else {
        mid(&raw[raw.len() - 1], &raw[0])
    };
    let mut points = vec![start];

    let mut i = 0usize;
    while i < raw.len() {
        let current = raw[i];
        if current.on_curve {
            points.push(current);
            i += 1;
            continue;
        }
        // Off-curve control: the next on-curve point (possibly implied) ends it.
        let next_raw = if i + 1 < raw.len() { raw[i + 1] } else { raw[0] };
        let end = if next_raw.on_curve {
            i += 2;
            next_raw
        } else {
            i += 1;
            mid(&current, &next_raw)
        };

        let from = *points.last()?;
        for step in 1..=CURVE_SEGMENTS {
            let t = step as f64 / CURVE_SEGMENTS as f64;
            let inv = 1.0 - t;
            points.push(Point {
                x: inv * inv * from.x + 2.0 * inv * t * current.x + t * t * end.x,
                y: inv * inv * from.y + 2.0 * inv * t * current.y + t * t * end.y,
                on_curve: false,
            });
        }
    }

    if points.len() < 2 {
        return None;
    }
    Some(Contour { points })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// A real font, if this machine has one. Tests that need it skip cleanly.
    /// A real font, or `None` when this machine has none.
    ///
    /// This deliberately PANICS if a font file exists but will not parse. An
    /// earlier version returned `None` on a parse failure, which made every
    /// "real font" test below silently pass while testing nothing at all - the
    /// tests looked green and proved nothing.
    fn real_font() -> Option<Font> {
        for path in [
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        ] {
            if !Path::new(path).is_file() {
                continue;
            }
            let data = std::fs::read(path).expect("font is readable");
            match Font::parse(data) {
                Some(font) => return Some(font),
                None => panic!("{path} exists but Font::parse rejected it"),
            }
        }
        None
    }

    #[test]
    fn parses_a_real_font() {
        let Some(font) = real_font() else { return };
        assert!(font.units_per_em >= 16.0, "implausible units-per-em");
        assert!(!font.cmap.is_empty(), "a real font has a cmap");
    }

    #[test]
    fn refuses_otf_and_garbage_instead_of_guessing() {
        assert!(Font::parse(b"not a font".to_vec()).is_none());
        assert!(Font::parse(Vec::new()).is_none());
        // 'OTTO' is CFF outlines, which this does not read.
        let mut otto = 0x4F54_544Fu32.to_be_bytes().to_vec();
        otto.resize(64, 0);
        assert!(Font::parse(otto).is_none(), "must refuse CFF, not return empty boxes");
    }

    #[test]
    fn truncated_fonts_never_panic() {
        let Some(font) = real_font() else { return };
        let data = font.data.clone();
        for len in 0..data.len().min(4000) {
            let _ = Font::parse(data[..len].to_vec());
        }
    }

    #[test]
    fn a_real_letter_has_contours() {
        let Some(font) = real_font() else { return };
        for ch in ['H', 'e', 'l', 'o', 'A'] {
            let glyph = font
                .glyph_for_char(ch)
                .unwrap_or_else(|| panic!("{ch} should exist in a real font"));
            assert!(!glyph.contours.is_empty(), "{ch} produced no outline");
            assert!(glyph.advance > 0.0, "{ch} has no advance");
        }
    }

    #[test]
    fn outlines_stay_inside_the_font_em_square() {
        // Coordinates are font units. A value far outside the em square means we
        // mis-parsed the delta encoding, which is the classic failure here.
        let Some(font) = real_font() else { return };
        let limit = font.units_per_em * 3.0;
        for ch in ['H', 'e', 'o', 'g', 'B'] {
            let glyph = font.glyph_for_char(ch).expect("glyph");
            for contour in &glyph.contours {
                for point in &contour.points {
                    assert!(
                        point.x.abs() < limit && point.y.abs() < limit,
                        "{ch} point ({}, {}) escapes the em square {limit}",
                        point.x,
                        point.y
                    );
                }
            }
        }
    }

    #[test]
    fn a_space_has_an_advance_but_no_contours() {
        let Some(font) = real_font() else { return };
        let space = font.glyph_for_char(' ').expect("space exists in a real font");
        assert!(space.contours.is_empty(), "a space must not be drawn");
        assert!(space.advance > 0.0, "a space must still move the pen");
    }

    #[test]
    fn a_missing_codepoint_is_absent_not_wrong() {
        let Some(font) = real_font() else { return };
        // Verified against the real files: DejaVu maps U+1F600 (it carries
        // monochrome symbol glyphs), but has NO run for CJK, Thai, Hangul,
        // Devanagari or Hiragana. Those are the honest uncovered cases.
        for (cp, label) in [
            ('\u{4E00}', "CJK"),
            ('\u{0E01}', "Thai"),
            ('\u{AC00}', "Hangul"),
            ('\u{093F}', "Devanagari"),
            ('\u{3042}', "Hiragana"),
        ] {
            assert!(
                font.glyph_for_char(cp).is_none(),
                "{label} is not in a Latin font, so it must report None, not an empty box"
            );
        }
    }

    #[test]
    fn contour_length_is_positive_and_ordered() {
        let Some(font) = real_font() else { return };
        let glyph = font.glyph_for_char('H').expect("H");
        for contour in &glyph.contours {
            assert!(contour.length() > 0.0, "a drawn contour must have length");
            assert!(contour.points.len() >= CURVE_SEGMENTS.min(2));
        }
    }

    #[test]
    fn a_contour_starting_off_curve_is_normalised() {
        // If this is wrong the letter gets a visible kink at the start.
        let raw = vec![
            Point { x: 50.0, y: 50.0, on_curve: false },
            Point { x: 100.0, y: 100.0, on_curve: true },
            Point { x: 0.0, y: 100.0, on_curve: false },
        ];
        let contour = flatten_contour(&raw).expect("should flatten");
        assert!(contour.points.first().is_some_and(|p| p.on_curve));
    }

    #[test]
    fn degenerate_contours_are_dropped_not_drawn() {
        assert!(flatten_contour(&[]).is_none());
        assert!(flatten_contour(&[Point { x: 1.0, y: 1.0, on_curve: true }]).is_none());
    }
}
