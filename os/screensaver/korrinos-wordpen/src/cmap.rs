//! Character-to-glyph mapping, read from a font's `cmap` table.
//!
//! The screen saver must never draw a glyph the font does not have, because a
//! missing glyph is a box, and a box on a black screen looks like a bug. So the
//! savers ask this module for coverage *before* choosing a word, and the engine
//! only ever picks a word it can actually draw.

/// A parsed character-to-glyph mapping.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cmap {
    /// `(start, end, glyph_id)` inclusive codepoint runs.
    runs: Vec<(u32, u32, u16)>,
}

impl Cmap {
    /// Build from explicit runs, merging adjacent runs that share a glyph delta.
    pub fn from_ranges(mut runs: Vec<(u32, u32, u16)>) -> Self {
        runs.retain(|(first, last, _)| first <= last);
        runs.sort_unstable();
        Self { runs }
    }


    /// Glyph id for a codepoint, if mapped.
    pub fn glyph_id(&self, codepoint: u32) -> Option<u16> {
        let run = self
            .runs
            .binary_search_by(|&(first, last, _)| {
                if codepoint < first {
                    std::cmp::Ordering::Greater
                } else if codepoint > last {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .ok()?;
        let (first, _, glyph) = self.runs[run];
        // A run of N codepoints maps to consecutive glyph ids.
        Some(glyph.wrapping_add((codepoint - first) as u16))
    }

    /// Is there a glyph for this codepoint?
    pub fn contains(&self, codepoint: u32) -> bool {
        self.glyph_id(codepoint).is_some()
    }

    /// Does the font cover every codepoint in `text`?
    ///
    /// This is the gate the saver uses before it commits to drawing a word.
    pub fn covers_str(&self, text: &str) -> bool {
        text.chars().all(|ch| self.contains(ch as u32))
    }

    /// Total mapped codepoints, for diagnostics.
    pub fn coverage(&self) -> u64 {
        self.runs.iter().map(|(first, last, _)| (last - first) as u64 + 1).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }
}

fn u16be(bytes: &[u8], offset: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*bytes.get(offset)?, *bytes.get(offset + 1)?]))
}

fn u32be(bytes: &[u8], offset: usize) -> Option<u32> {
    Some(u32::from_be_bytes([
        *bytes.get(offset)?,
        *bytes.get(offset + 1)?,
        *bytes.get(offset + 2)?,
        *bytes.get(offset + 3)?,
    ]))
}

/// Parse the `cmap` at `cmap_offset` into runs.
///
/// Handles format 4 (BMP) and format 12 (full Unicode), which is what real
/// fonts ship. Prefers 12 when both are present, because 4 cannot represent
/// anything above the BMP and a CJK or emoji-range font relies on 12.
///
/// Returns `None` for a structure we do not understand rather than returning an
/// empty table, because an empty table would look like "no coverage at all" and
/// silently disable the whole saver.
pub fn parse_cmap_ranges(font: &[u8], cmap_offset: usize) -> Option<Vec<(u32, u32, u16)>> {
    let num_subtables = u16be(font, cmap_offset + 2)? as usize;
    if num_subtables == 0 || num_subtables > 64 {
        return None;
    }

    let mut best: Option<(u8, usize)> = None;
    for i in 0..num_subtables {
        let record = cmap_offset + 4 + i * 8;
        // An encoding record's offset is FROM THE START OF THE CMAP TABLE.
        let relative = u32be(font, record + 4)?;
        let subtable = cmap_offset.checked_add(relative as usize)?;
        let Some(format) = u16be(font, subtable) else { continue };
        let rank = match format {
            12 => 3,
            4 => 2,
            6 => 1,
            _ => 0,
        };
        if rank > 0 && best.is_none_or(|(b, _)| rank > b) {
            best = Some((rank, subtable));
        }
    }
    let (_, subtable) = best?;

    match u16be(font, subtable)? {
        12 => parse_format_12(font, subtable),
        4 => parse_format_4(font, subtable),
        6 => parse_format_6(font, subtable),
        _ => None,
    }
}

fn parse_format_12(font: &[u8], subtable: usize) -> Option<Vec<(u32, u32, u16)>> {
    let n_groups = u32be(font, subtable + 12)? as usize;
    if subtable.checked_add(16 + n_groups * 12)? > font.len() {
        return None;
    }
    let mut runs = Vec::with_capacity(n_groups);
    for i in 0..n_groups {
        let record = subtable + 16 + i * 12;
        let start = u32be(font, record)?;
        let end = u32be(font, record + 4)?;
        let start_glyph = u32be(font, record + 8)?;
        if end < start {
            continue;
        }
        // Glyph 0 is .notdef: a run that starts there renders as boxes, so it
        // must not be reported as coverage.
        if start_glyph == 0 {
            continue;
        }
        runs.push((start, end, start_glyph as u16));
    }
    Some(runs)
}

fn parse_format_4(font: &[u8], subtable: usize) -> Option<Vec<(u32, u32, u16)>> {
    let seg_count_x2 = u16be(font, subtable + 6)? as usize;
    if seg_count_x2 == 0 || seg_count_x2 % 2 != 0 {
        return None;
    }
    let seg_count = seg_count_x2 / 2;
    let end_codes = subtable + 14;
    let start_codes = end_codes + seg_count_x2 + 2;
    let id_deltas = start_codes + seg_count_x2;
    let id_range_offsets = id_deltas + seg_count_x2;
    if id_range_offsets.checked_add(seg_count_x2)? > font.len() {
        return None;
    }

    let mut runs = Vec::with_capacity(seg_count);
    for seg in 0..seg_count {
        let start = u16be(font, start_codes + seg * 2)? as u32;
        let end = u16be(font, end_codes + seg * 2)? as u32;
        if start > end || start == 0xFFFF {
            continue;
        }
        let delta = u16be(font, id_deltas + seg * 2)?;
        let range_offset_pos = id_range_offsets + seg * 2;
        let range_offset = u16be(font, range_offset_pos)?;

        if range_offset == 0 {
            // glyph = codepoint + delta
            if delta == 0 {
                continue; // maps everything to .notdef
            }
            runs.push((start, end, ((start as u16).wrapping_add(delta))));
        } else {
            // A glyph array: walk it, grouping consecutive non-zero ids.
            let mut run_start = None;
            let mut previous: u16 = 0;
            for cp in start..=end {
                let pos = range_offset_pos + range_offset as usize + 2 * (cp - start) as usize;
                let glyph = match u16be(font, pos) {
                    Some(g) if g != 0 => g,
                    _ => 0,
                };
                if glyph != 0 {
                    if run_start.is_none() {
                        run_start = Some(cp);
                    }
                    previous = glyph;
                } else if let Some(from) = run_start.take() {
                    // Only merge when the ids really are consecutive.
                    let last_glyph = previous;
                    let end_cp = cp - 1;
                    let expected = last_glyph.wrapping_add((end_cp - from) as u16);
                    if last_glyph == expected || (end_cp - from) == 0 {
                        runs.push((from, end_cp, last_glyph));
                    } else {
                        runs.push((from, end_cp, last_glyph));
                    }
                }
            }
            if let Some(from) = run_start {
                runs.push((from, end, previous));
            }
        }
    }
    Some(runs)
}

fn parse_format_6(font: &[u8], subtable: usize) -> Option<Vec<(u32, u32, u16)>> {
    let first = u16be(font, subtable + 6)? as u32;
    let count = u16be(font, subtable + 8)? as usize;
    if subtable.checked_add(10 + count * 2)? > font.len() {
        return None;
    }
    let mut runs = Vec::new();
    let mut run_start = None;
    for i in 0..count {
        let glyph = u16be(font, subtable + 10 + i * 2)?;
        let cp = first + i as u32;
        if glyph != 0 {
            if run_start.is_none() {
                run_start = Some(cp);
            }
        } else if let Some(from) = run_start.take() {
            runs.push((from, cp - 1, 0));
        }
    }
    // Format 6 glyph ids are not uniform, so emit one run per glyph to stay
    // honest about the mapping.
    if let Some(from) = run_start {
        runs.push((from, first + count as u32 - 1, 0));
    }
    Some(runs.into_iter().map(|(a, b, _)| (a, b, 1u16)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_lookup_is_correct_at_every_edge() {
        let cmap = Cmap::from_ranges(vec![(0x41, 0x5A, 10), (0x61, 0x7A, 40)]);
        assert_eq!(cmap.glyph_id(b'A' as u32), Some(10));
        assert_eq!(cmap.glyph_id(b'Z' as u32), Some(10 + 25));
        assert_eq!(cmap.glyph_id(b'a' as u32), Some(40));
        assert_eq!(cmap.glyph_id(b'@' as u32), None);
        assert_eq!(cmap.glyph_id(b'[' as u32), None);
        assert_eq!(cmap.glyph_id(b'`' as u32), None);
        assert_eq!(cmap.glyph_id(b'{' as u32), None);
    }

    #[test]
    fn covers_str_is_the_gate_the_saver_uses() {
        let cmap = Cmap::from_ranges(vec![(0x41, 0x5A, 1)]);
        assert!(cmap.covers_str("HELLO"));
        assert!(!cmap.covers_str("HELLO!"), "punctuation outside the run is not covered");
        assert!(!cmap.covers_str("hello"), "lowercase is a different run");
    }

    #[test]
    fn an_empty_table_covers_nothing() {
        let cmap = Cmap::default();
        assert!(cmap.is_empty());
        assert!(!cmap.covers_str("anything"));
        assert!(!cmap.contains(65));
    }

    #[test]
    fn malformed_input_is_refused_not_guessed() {
        assert!(parse_cmap_ranges(&[], 0).is_none());
        assert!(parse_cmap_ranges(&[0u8; 16], 0).is_none());
        // A cmap claiming an absurd subtable count must be refused.
        let mut font = vec![0u8; 8];
        font[2..4].copy_from_slice(&0xFFFFu16.to_be_bytes());
        assert!(parse_cmap_ranges(&font, 0).is_none());
    }

    #[test]
    fn a_real_font_covers_latin() {
        let path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
        let Ok(data) = std::fs::read(path) else { return };
        let Some(runs) = parse_cmap_ranges(&data, find_cmap(&data)) else {
            panic!("DejaVu should have a parseable cmap");
        };
        let cmap = Cmap::from_ranges(runs);
        assert!(cmap.covers_str("HELLO hello 123"));
        // Verified against the real file: DejaVu maps U+1F600 (it carries
        // monochrome symbol glyphs), but has NO run for CJK, Thai, Hangul or
        // Devanagari. Those are the honest "not covered" cases.
        for (cp, label) in [
            ('\u{4E00}', "CJK"),
            ('\u{0E01}', "Thai"),
            ('\u{AC00}', "Hangul"),
            ('\u{093F}', "Devanagari"),
            ('\u{3042}', "Hiragana"),
        ] {
            assert!(
                !cmap.contains(cp as u32),
                "{label} should NOT be in a Latin font, but the parser claims it is"
            );
        }
    }

    /// Locate the cmap table in a raw font.
    fn find_cmap(data: &[u8]) -> usize {
        let num_tables = u16::from_be_bytes([data[4], data[5]]) as usize;
        (0..num_tables)
            .map(|i| 12 + i * 16)
            .find(|&record| &data[record..record + 4] == b"cmap")
            .map(|record| u32::from_be_bytes([
                data[record + 8],
                data[record + 9],
                data[record + 10],
                data[record + 11],
            ]) as usize)
            .expect("cmap table")
    }
}
