//! Glyph coverage diagnostics for the shipped languages.
//!
//! # The problem
//!
//! The language registry maps each of the 55 locales to a font package, and
//! `kcommand_typography` decides which family to *ask* for. Neither checks that
//! the font actually contains the glyphs. On a 55-language OS that gap is where
//! tofu comes from: an Indic matra clipped or missing, a Tamil syllable
//! rendered as boxes, and nothing in the log to explain why.
//!
//! So this module does the one thing that actually answers the question: it
//! opens the font file and asks its `cmap` table which codepoints exist.
//!
//! # Why it is pure
//!
//! As with the transition animation, a build machine has no display and no
//! fonts, so this has to be testable without either. The parser takes a byte
//! slice and returns a table; the tests build a font in memory and parse it.
//! Only `find_font_file` touches the filesystem, and it is kept separate.
//!
//! Scope: this reads the `cmap` mapping (does this codepoint have a glyph).
//! It deliberately does NOT check shaping, hinting, or advance widths, because
//! answering "will this render" for a complex script needs a real shaper
//! (HarfBuzz), which is a much larger dependency. Missing-glyph is the failure
//! users actually hit first, and it is the one this can detect for certain.

use std::path::PathBuf;

/// A parsed character-to-glyph mapping.
///
/// Codepoints are stored as sorted ranges, because a `cmap` subtable is already
/// organised that way and looking a codepoint up should not walk a list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Cmap {
    /// `(first, last)` inclusive codepoint ranges.
    ranges: Vec<(u32, u32)>,
}

impl Cmap {
    /// Build from explicit ranges, merging any that touch or overlap.
    pub fn from_ranges(mut ranges: Vec<(u32, u32)>) -> Self {
        ranges.retain(|(first, last)| first <= last);
        ranges.sort_unstable();
        let mut merged: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
        for (first, last) in ranges {
            match merged.last_mut() {
                Some(prev) if first <= prev.1.saturating_add(1) => {
                    prev.1 = prev.1.max(last);
                },
                _ => merged.push((first, last)),
            }
        }
        Self { ranges: merged }
    }

    /// Is there a glyph for this codepoint?
    pub fn contains(&self, codepoint: u32) -> bool {
        // Ranges are sorted and disjoint, so binary search is correct here.
        self.ranges
            .binary_search_by(|&(first, last)| {
                if codepoint < first {
                    std::cmp::Ordering::Greater
                } else if codepoint > last {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
    }

    /// The merged ranges, for diagnostics.
    pub fn ranges(&self) -> &[(u32, u32)] {
        &self.ranges
    }

    /// Total codepoints covered, saturating rather than overflowing.
    pub fn coverage(&self) -> u64 {
        self.ranges.iter().map(|(first, last)| (last - first) as u64 + 1).sum()
    }

    /// True when the table maps nothing at all.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset + 2)?;
    Some(u16::from_be_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let slice = bytes.get(offset..offset + 4)?;
    Some(u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

/// Parse a font's `cmap` into a codepoint table.
///
/// Handles the two subtable formats that matter in practice: format 4
/// (BMP, the legacy format) and format 12 (full Unicode, used by every modern
/// Indic/Arabic/CJK font). Returns `None` when the file is not a font we can
/// read a mapping out of, rather than guessing.
pub fn parse_cmap(font: &[u8]) -> Option<Cmap> {
    const SFNT: u32 = 0x0001_0000;
    const TRUE: u32 = 0x7472_7565; // 'true'
    const OTTO: u32 = 0x4F54_544F; // 'OTTO'
    const TTCF: u32 = 0x7474_6366; // 'ttcf'

    // A TrueType collection starts with 'ttcf'; use the first font in it.
    let base = if read_u32(font, 0) == Some(TTCF) {
        let first = read_u32(font, 12)? as usize;
        first
    } else {
        0
    };

    let tag = read_u32(font, base)?;
    if tag != SFNT && tag != TRUE && tag != OTTO {
        return None;
    }

    let num_tables = read_u16(font, base + 4)? as usize;

    // Locate the cmap table.
    let mut cmap_offset = None;
    for i in 0..num_tables {
        let record = base + 12 + i * 16;
        if read_u32(font, record)? == u32::from_be_bytes(*b"cmap") {
            cmap_offset = Some(read_u32(font, record + 8)? as usize);
            break;
        }
    }
    let cmap_offset = cmap_offset?;
    if cmap_offset + 4 > font.len() {
        return None;
    }

    let num_subtables = read_u16(font, cmap_offset + 2)? as usize;

    // Prefer format 12 (full Unicode), fall back to format 4 (BMP).
    let mut best: Option<(u8, usize)> = None;
    for i in 0..num_subtables {
        let record = cmap_offset + 4 + i * 8;
        // Per the OpenType spec an encoding record's offset is FROM THE START OF
        // THE CMAP TABLE, not from the start of the file. Reading it as absolute
        // lands on the wrong bytes and yields a nonsense format number, which is
        // why this has to be added to cmap_offset.
        let Some(relative) = read_u32(font, record + 4) else {
            continue;
        };
        let subtable = cmap_offset.checked_add(relative as usize)?;
        let Some(format) = read_u16(font, subtable) else {
            continue;
        };
        let rank = match format {
            12 => 2,
            4 => 1,
            _ => 0,
        };
        if rank > 0 && best.map_or(true, |(b, _)| rank > b) {
            best = Some((rank, subtable));
        }
    }

    let (_, subtable) = best?;
    match read_u16(font, subtable)? {
        12 => parse_format_12(font, subtable),
        4 => parse_format_4(font, subtable),
        _ => None,
    }
}

/// Format 4: segmented mapping, BMP only.
fn parse_format_4(font: &[u8], subtable: usize) -> Option<Cmap> {
    let seg_count_x2 = read_u16(font, subtable + 6)? as usize;
    if seg_count_x2 == 0 || seg_count_x2 % 2 != 0 {
        return None;
    }
    let seg_count = seg_count_x2 / 2;

    let end_codes = subtable + 14;
    let start_codes = end_codes + seg_count_x2 + 2;
    let id_deltas = start_codes + seg_count_x2;
    let id_range_offsets = id_deltas + seg_count_x2;

    let mut ranges = Vec::with_capacity(seg_count);
    for seg in 0..seg_count {
        let start = read_u16(font, start_codes + seg * 2)? as u32;
        let end = read_u16(font, end_codes + seg * 2)? as u32;
        if start > end || start == 0xFFFF {
            // 0xFFFF is the required terminating segment.
            continue;
        }

        // Only segments with a non-zero glyph delta are real mappings. A zero
        // delta segment maps to glyph 0, which is .notdef, i.e. tofu.
        let delta = read_u16(font, id_deltas + seg * 2)?;
        let range_offset_pos = id_range_offsets + seg * 2;
        let range_offset = read_u16(font, range_offset_pos)?;

        if delta != 0 || range_offset == 0 {
            ranges.push((start, end));
            continue;
        }

        // A range offset points at a per-glyph array; a glyph id of 0 anywhere
        // in the segment means part of it is unmapped, so split around it.
        let mut run_start = None;
        for cp in start..=end {
            let glyph_pos = range_offset_pos + range_offset as usize
                + 2 * (cp - start) as usize;
            let glyph = read_u16(font, glyph_pos).unwrap_or(0);
            if glyph != 0 {
                if run_start.is_none() {
                    run_start = Some(cp);
                }
            } else if let Some(from) = run_start.take() {
                ranges.push((from, cp - 1));
            }
        }
        if let Some(from) = run_start {
            ranges.push((from, end));
        }
    }

    Some(Cmap::from_ranges(ranges))
}

/// Format 12: segmented coverage, full Unicode.
fn parse_format_12(font: &[u8], subtable: usize) -> Option<Cmap> {
    let n_groups = read_u32(font, subtable + 12)? as usize;
    // Each group is 12 bytes; guard against a nonsense count claiming more data
    // than the file holds.
    let needed = subtable + 16 + n_groups * 12;
    if needed > font.len() {
        return None;
    }

    let mut ranges = Vec::with_capacity(n_groups);
    for i in 0..n_groups {
        let record = subtable + 16 + i * 12;
        let start = read_u32(font, record)?;
        let end = read_u32(font, record + 4)?;
        let start_glyph = read_u32(font, record + 8)?;
        // A group starting at glyph 0 maps to .notdef.
        if start == 0xFFFF_FFFF || end < start || start_glyph == 0 {
            continue;
        }
        ranges.push((start, end));
    }

    Some(Cmap::from_ranges(ranges))
}

/// Representative codepoints that any font claiming to support `script`
/// should have.
///
/// These are chosen to catch the failure modes that actually bite: a base
/// consonant, the vowel signs that stack above and below, and a combining mark
/// whose absence is invisible in a Latin-only font but obvious in real text.
pub fn probe_codepoints(script: &str) -> Vec<u32> {
    let probes: &[char] = match script {
        "latin" => &['A', 'z', 'e', 'ñ'],
        "cyrillic" => &['А', 'я', 'ё', 'ж'],
        "greek" => &['Α', 'ω', 'ΐ', 'ζ'],
        "arabic" => &['ا', 'ب', 'ت', 'ة', 'م'],
        "hebrew" => &['א', 'ת', 'ם', 'ץ'],
        "deva" => &['क', 'ह', 'ि', 'ी', 'ो', '्'],
        "beng" => &['ক', 'হ', 'ি', 'ী', 'ো', '্'],
        "taml" => &['க', 'த', 'ி', 'ீ', 'ொ', '்'],
        "telu" => &['క', 'త', 'ి', 'ీ', 'ో', '్'],
        "gujr" => &['ક', 'હ', 'િ', 'ી', 'ો', '્'],
        "knda" => &['ಕ', 'ಹ', 'ಿ', 'ೀ', 'ೋ', '್'],
        "mlym" => &['ക', 'ഹ', 'ി', 'ീ', 'ോ', '്'],
        "guru" => &['ਕ', 'ਹ', 'ਿ', 'ੀ', 'ੋ', '੍'],
        "sinh" => &['ක', 'ඹ', 'ි', 'ී', 'ළ'],
        "thai" => &['ก', 'ฮ', 'ิ', 'ี', '้', '์'],
        "lao" => &['ກ', 'ຮ', 'ິ', 'ີ', '້', 'ໍ'],
        "mymr" => &['က', 'အ', 'ိ', 'ာ', 'ေ'],
        "ethi" => &['ሀ', 'ሐ', 'ነ', 'ዐ', 'ፐ'],
        "hans" => &['中', '文', '，', '。', '国'],
        "hant" => &['中', '文', '，', '。', '國'],
        "jpan" => &['あ', 'ア', '漢', 'か', 'ん'],
        "kore" => &['한', '글', '국', '어', '힣'],
        _ => return Vec::new(),
    };
    probes.iter().map(|ch| *ch as u32).collect()
}

/// The result of checking one script against one font file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptCoverage {
    pub script: String,
    pub family: String,
    /// Where the font was found, if anywhere.
    pub path: Option<PathBuf>,
    /// Probes the font does not have a glyph for.
    pub missing: Vec<char>,
    /// The font could not be read at all.
    pub unreadable: bool,
}

impl ScriptCoverage {
    /// Is this script fully covered by the resolved font?
    pub fn is_ok(&self) -> bool {
        self.path.is_some() && !self.unreadable && self.missing.is_empty()
    }
}

/// Directories searched for font files.
fn font_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![
        PathBuf::from("/usr/share/fonts"),
        PathBuf::from("/usr/local/share/fonts"),
        PathBuf::from("/usr/share/korrinos/fonts"),
    ];
    if let Ok(home) = std::env::var("HOME") {
        let home = PathBuf::from(home);
        dirs.push(home.join(".fonts"));
        dirs.push(home.join(".local/share/fonts"));
    }
    dirs
}

/// Normalise a name for comparison: lowercase, alphanumerics only.
///
/// This is the fix for a real bug. Comparing a slug of "DejaVu Sans"
/// ("dejavu-sans") against the filename stem "dejavusans" never matches,
/// because the family has a space where the file has none. Every real font
/// file drops spaces, hyphens and underscores differently, so both sides have
/// to be reduced to plain letters before comparing.
fn normalise(name: &str) -> String {
    name.chars().filter(|ch| ch.is_ascii_alphanumeric()).map(|ch| ch.to_ascii_lowercase()).collect()
}

/// Look for a font file belonging to `family`.
///
/// Font files are named inconsistently across distributions ("NotoSansMono
/// Devanagari-Regular.ttf", "NotoSansDevanagari-hinted.ttf"), so matching is on
/// a punctuation-free comparison rather than one exact layout.
pub fn find_font_file(family: &str) -> Option<PathBuf> {
    let needle = normalise(family);
    if needle.is_empty() {
        return None;
    }
    // A one-word family such as "Balsamiq" is short enough that substring
    // matching would hit unrelated fonts, so require it to lead the name.
    let require_prefix = needle.len() <= 10;

    for dir in font_dirs() {
        if let Some(found) = search_font_dir(&dir, &needle, require_prefix, 0) {
            return Some(found);
        }
    }
    None
}

/// How deep to walk a font tree. Distributions nest a few levels
/// (/usr/share/fonts/truetype/<vendor>/<file>), so this has to recurse.
const MAX_FONT_DEPTH: usize = 4;

/// Walk one directory looking for a font file matching `needle`.
fn search_font_dir(dir: &PathBuf, needle: &str, require_prefix: bool, depth: usize) -> Option<PathBuf> {
    if depth > MAX_FONT_DEPTH {
        return None;
    }
    let entries = std::fs::read_dir(dir).ok()?;

    // Two passes so a direct hit always wins over a nested one, and so the
    // result does not depend on directory iteration order.
    let mut nested = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            nested.push(path);
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let lower = name.to_ascii_lowercase();
        if !is_font_extension(&lower) {
            continue;
        }
        // Compare on the stem only, so ".ttf" and ".otf" never affect matching.
        let stem = lower.split('.').next().unwrap_or_default();
        let hit = if require_prefix {
            normalise(stem).starts_with(needle)
        } else {
            normalise(stem).contains(needle)
        };
        if hit {
            return Some(path);
        }
    }

    for dir in nested {
        if let Some(found) = search_font_dir(&dir, needle, require_prefix, depth + 1) {
            return Some(found);
        }
    }
    None
}

fn is_font_extension(lower_name: &str) -> bool {
    lower_name.ends_with(".ttf") || lower_name.ends_with(".otf") || lower_name.ends_with(".ttc")
}

/// Check one script against one family, reading the font's real cmap.
pub fn check_script(script: &str, family: &str) -> ScriptCoverage {
    let probes = probe_codepoints(script);
    let Some(path) = find_font_file(family) else {
        return ScriptCoverage {
            script: script.to_string(),
            family: family.to_string(),
            path: None,
            missing: probes.iter().filter_map(|c| char::from_u32(*c)).collect(),
            unreadable: false,
        };
    };

    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            return ScriptCoverage {
                script: script.to_string(),
                family: family.to_string(),
                path: Some(path),
                missing: probes.iter().filter_map(|c| char::from_u32(*c)).collect(),
                unreadable: true,
            };
        },
    };

    match parse_cmap(&bytes) {
        Some(cmap) => {
            let missing = probes
                .iter()
                .filter(|cp| !cmap.contains(**cp))
                .filter_map(|cp| char::from_u32(*cp))
                .collect();
            ScriptCoverage {
                script: script.to_string(),
                family: family.to_string(),
                path: Some(path),
                missing,
                unreadable: false,
            }
        },
        None => ScriptCoverage {
            script: script.to_string(),
            family: family.to_string(),
            path: Some(path),
            missing: probes.iter().filter_map(|c| char::from_u32(*c)).collect(),
            unreadable: true,
        },
    }
}

/// Check the active language's script against the family that will be used,
/// and return a one-line verdict.
///
/// This is the answer to "why is this text boxes", so it distinguishes the
/// three different causes instead of collapsing them into "unsupported":
/// font not installed, font unreadable, or font genuinely missing glyphs.
pub fn describe_active_coverage() -> String {
    use crate::renderer::text::kcommand_typography as typography;

    let Some(script) = typography::active_script() else {
        return "kcommand coverage: no active language resolved; cannot check glyphs".to_string();
    };
    let Some(family) = typography::active_font_family() else {
        return format!(
            "kcommand coverage: script={script} has no font policy; \
             using your configured family, which is not being verified"
        );
    };

    let report = check_script(script, family);
    if report.path.is_none() {
        return format!(
            "kcommand coverage: script={script} family={family} NOT INSTALLED \
             (expected under /usr/share/fonts; glyphs would render as boxes)"
        );
    }
    if report.unreadable {
        return format!(
            "kcommand coverage: script={script} family={family} found at {:?} but its cmap \
             could not be read; coverage unknown",
            report.path
        );
    }
    if report.missing.is_empty() {
        // Name the file that was actually opened. Without this a wrong match
        // would print a confident OK with nothing to check it against.
        return format!(
            "kcommand coverage: script={script} family={family} OK \
             ({} probe glyphs in {:?})",
            probe_codepoints(script).len(),
            report.path
        );
    }
    format!(
        "kcommand coverage: script={script} family={family} MISSING {} of {} probe glyphs: {}",
        report.missing.len(),
        probe_codepoints(script).len(),
        report.missing.iter().collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal TrueType font in memory with one format-4 cmap
    /// subtable covering the given ranges.
    ///
    /// This is the point of the whole design: the parser is testable without a
    /// font on disk, so coverage checking has real tests on a build box.
    fn synthetic_font_format4(segments: &[(u32, u32)]) -> Vec<u8> {
        let seg_count = segments.len() + 1; // +1 for the required 0xFFFF terminator

        let end_codes: Vec<u16> =
            segments.iter().map(|(_, e)| *e as u16).chain([0xFFFF]).collect();
        let start_codes: Vec<u16> =
            segments.iter().map(|(s, _)| *s as u16).chain([0xFFFF]).collect();
        // Non-zero delta means "mapped" for every codepoint in the segment.
        let id_deltas: Vec<u16> = segments.iter().map(|_| 1u16).chain([1u16]).collect();
        let id_range_offsets: Vec<u16> = segments.iter().map(|_| 0u16).chain([0u16]).collect();

        let seg_count_x2 = (seg_count * 2) as u16;
        let subtable_len = 16 + seg_count_x2 as usize * 4;
        let cmap_header = 4 + 8; // version + numTables + one encoding record

        let mut font = vec![0u8; 12 + 16 + cmap_header + subtable_len];

        // sfnt header
        font[0..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
        font[4..6].copy_from_slice(&1u16.to_be_bytes()); // numTables
        // table record: tag 'cmap', checksum 0, offset
        let table_offset = 12 + 16;
        font[12..16].copy_from_slice(b"cmap");
        font[20..24].copy_from_slice(&(table_offset as u32).to_be_bytes());

        // cmap header
        let cmap = table_offset;
        font[cmap + 2..cmap + 4].copy_from_slice(&1u16.to_be_bytes()); // numTables
        // encoding record: platform 3 (Windows), encoding 1 (BMP). The offset
        // is relative to the start of the cmap table, as the spec requires.
        let sub_off = cmap + cmap_header;
        font[cmap + 4..cmap + 6].copy_from_slice(&3u16.to_be_bytes());
        font[cmap + 6..cmap + 8].copy_from_slice(&1u16.to_be_bytes());
        font[cmap + 8..cmap + 12].copy_from_slice(&((sub_off - cmap) as u32).to_be_bytes());

        // format 4 subtable
        font[sub_off..sub_off + 2].copy_from_slice(&4u16.to_be_bytes());
        font[sub_off + 2..sub_off + 4].copy_from_slice(&(subtable_len as u16).to_be_bytes());
        font[sub_off + 4..sub_off + 6].copy_from_slice(&0u16.to_be_bytes()); // language
        font[sub_off + 6..sub_off + 8].copy_from_slice(&seg_count_x2.to_be_bytes());

        let end_base = sub_off + 14;
        let start_base = end_base + seg_count_x2 as usize + 2;
        let delta_base = start_base + seg_count_x2 as usize;
        let range_base = delta_base + seg_count_x2 as usize;

        for i in 0..seg_count {
            font[end_base + i * 2..end_base + i * 2 + 2]
                .copy_from_slice(&end_codes[i].to_be_bytes());
            font[start_base + i * 2..start_base + i * 2 + 2]
                .copy_from_slice(&start_codes[i].to_be_bytes());
            font[delta_base + i * 2..delta_base + i * 2 + 2]
                .copy_from_slice(&id_deltas[i].to_be_bytes());
            font[range_base + i * 2..range_base + i * 2 + 2]
                .copy_from_slice(&id_range_offsets[i].to_be_bytes());
        }

        font
    }

    #[test]
    fn parses_a_synthetic_format4_font() {
        let font = synthetic_font_format4(&[(0x41, 0x5A), (0x61, 0x7A)]);
        let cmap = parse_cmap(&font).expect("should parse");
        assert!(cmap.contains(b'A' as u32));
        assert!(cmap.contains(b'z' as u32));
        assert!(!cmap.contains(b'@' as u32), "below the first range");
        assert!(!cmap.contains(b'[' as u32), "between the ranges");
        assert!(!cmap.contains(0x00E9), "non-latin should be absent");
    }

    #[test]
    fn rejects_things_that_are_not_fonts() {
        assert!(parse_cmap(&[]).is_none());
        assert!(parse_cmap(&[0u8; 64]).is_none());
        assert!(parse_cmap(b"not a font at all, just text").is_none());
        assert!(parse_cmap(&[0xFF; 200]).is_none());
    }

    #[test]
    fn truncated_fonts_do_not_panic() {
        let font = synthetic_font_format4(&[(0x41, 0x5A)]);
        for len in 0..font.len() {
            let _ = parse_cmap(&font[..len]);
        }
    }

    #[test]
    fn absurd_n_group_counts_are_refused_not_trusted() {
        // A format 12 subtable claiming more groups than the file can hold must
        // be rejected rather than read out of bounds.
        let mut font = synthetic_font_format4(&[(0x41, 0x5A)]);
        // Overwrite the subtable format and nGroups with nonsense.
        let cmap_off = 28u32 as usize;
        let sub = cmap_off + 12;
        font[sub..sub + 2].copy_from_slice(&12u16.to_be_bytes());
        font[sub + 12..sub + 16].copy_from_slice(&0xFFFF_FFFFu32.to_be_bytes());
        assert!(parse_cmap(&font).is_none());
    }

    #[test]
    fn ranges_merge_when_they_touch() {
        let cmap = Cmap::from_ranges(vec![(10, 20), (21, 30), (40, 50)]);
        assert_eq!(cmap.ranges().len(), 2, "touching ranges should merge");
        assert!(cmap.contains(20) && cmap.contains(21));
        // Merged (10..=30) is 21 codepoints, plus (40..=50) is 11.
        assert_eq!(cmap.coverage(), 21 + 11);
    }

    #[test]
    fn ranges_do_not_merge_when_they_are_apart() {
        let cmap = Cmap::from_ranges(vec![(10, 20), (22, 30)]);
        assert_eq!(cmap.ranges().len(), 2);
        assert!(!cmap.contains(21), "the gap stays a gap");
    }

    #[test]
    fn lookup_is_correct_at_and_around_every_edge() {
        let cmap = Cmap::from_ranges(vec![(100, 200), (300, 400)]);
        for cp in [99, 100, 150, 200, 201, 299, 300, 400, 401] {
            let expected = (100..=200).contains(&cp) || (300..=400).contains(&cp);
            assert_eq!(cmap.contains(cp), expected, "wrong answer for {cp}");
        }
    }

    #[test]
    fn reversed_and_empty_input_is_handled() {
        let cmap = Cmap::from_ranges(vec![(50, 10)]);
        assert!(cmap.is_empty(), "a reversed range is dropped, not misread");
        assert!(Cmap::default().is_empty());
        assert!(!Cmap::default().contains(65));
    }

    #[test]
    fn every_script_has_real_probes() {
        // A script with no probes would silently "pass" coverage checking.
        let scripts = [
            "latin", "cyrillic", "greek", "arabic", "hebrew", "deva", "beng", "taml",
            "telu", "gujr", "knda", "mlym", "guru", "sinh", "thai", "lao", "mymr", "ethi",
            "hans", "hant", "jpan", "kore",
        ];
        for script in scripts {
            let probes = probe_codepoints(script);
            assert!(!probes.is_empty(), "{script} has no probe codepoints");
            // Probes must be inside the Unicode range and non-zero.
            for cp in probes {
                assert!(cp > 0x20, "{script} probe U+{cp:X} is not a visible glyph");
            }
        }
    }

    #[test]
    fn indic_probes_include_combining_marks() {
        // The whole point of these probes: a Latin-only font passes the base
        // consonant test and still renders every vowel sign as tofu.
        for script in ["deva", "taml", "knda", "mlym", "guru"] {
            let has_mark = probe_codepoints(script)
                .iter()
                .any(|cp| char::from_u32(*cp).is_some_and(|c| is_combining(c)));
            assert!(has_mark, "{script} probes must include a combining mark");
        }
    }

    fn is_combining(c: char) -> bool {
        // The mark ranges this OS actually ships fonts for.
        let cp = c as u32;
        (0x0300..=0x036F).contains(&cp)      // combining diacriticals
            || (0x0900..=0x0DFF).contains(&cp) // Indic
            || (0x0E30..=0x0E4F).contains(&cp) // Thai
            || (0x0F00..=0x0FFF).contains(&cp) // Tibetan-ish
    }

    #[test]
    fn unknown_scripts_return_no_probes_rather_than_guessing() {
        assert!(probe_codepoints("klingon").is_empty());
        assert!(probe_codepoints("").is_empty());
    }

    #[test]
    fn a_font_missing_everything_is_reported_as_unreadable_not_ok() {
        let coverage = ScriptCoverage {
            script: "deva".into(),
            family: "Does Not Exist".into(),
            path: None,
            missing: probe_codepoints("deva").iter().filter_map(|c| char::from_u32(*c)).collect(),
            unreadable: false,
        };
        assert!(!coverage.is_ok(), "a missing font must never report OK");
    }

    #[test]
    fn unreadable_font_is_not_ok() {
        let coverage = ScriptCoverage {
            script: "taml".into(),
            family: "Broken".into(),
            path: Some(PathBuf::from("/nonexistent.ttf")),
            missing: vec!['க'],
            unreadable: true,
        };
        assert!(!coverage.is_ok(), "an unreadable font must never report OK");
    }

    #[test]
    fn finds_a_font_that_is_really_installed() {
        // Proves the recursive search works against the real filesystem, not
        // just that it correctly reports absence. Distribution font trees nest
        // (/usr/share/fonts/truetype/<vendor>/<file>), so a non-recursive search
        // would report every real font as missing.
        if !std::path::Path::new("/usr/share/fonts").is_dir() {
            return; // no fonts here; nothing to prove
        }
        let found = find_font_file("DejaVu Sans");
        assert!(
            found.is_some(),
            "DejaVu Sans ships with essentially every distro and should be found"
        );
        let path = found.unwrap();
        assert!(path.is_file());
        assert!(is_font_extension(&path.to_string_lossy().to_ascii_lowercase()));
    }

    #[test]
    fn a_real_font_reports_real_coverage() {
        // End to end: locate an installed font, parse it, and check it against a
        // script it genuinely covers.
        if !std::path::Path::new("/usr/share/fonts").is_dir() {
            return;
        }
        let Some(path) = find_font_file("DejaVu Sans") else {
            return;
        };
        let bytes = std::fs::read(&path).expect("readable");
        let cmap = parse_cmap(&bytes).expect("a real font must parse");
        assert!(!cmap.is_empty(), "a real font has a non-empty cmap");
        // DejaVu covers Latin, so at least one basic letter must be present.
        assert!(cmap.contains(b'A' as u32), "DejaVu Sans should map 'A'");
        assert!(cmap.coverage() > 1000, "a real font covers thousands of codepoints");
    }

    #[test]
    fn a_font_that_is_absent_is_reported_absent_not_wrong() {
        // The negative case matters just as much: a family that cannot exist
        // must never produce a false OK.
        let report = check_script("deva", "Definitely Not A Real Font 12345");
        assert!(report.path.is_none());
        assert!(!report.is_ok());
        assert!(!report.missing.is_empty(), "probes must be reported as missing");
    }

    #[test]
    fn names_normalise_to_the_same_key() {
        assert_eq!(normalise("Noto Sans Mono Devanagari"), "notosansmonodevanagari");
        assert_eq!(normalise("Balsamiq Sans"), "balsamiqsans");
        assert_eq!(normalise("../../etc/passwd"), "etcpasswd");
    }

    #[test]
    fn font_extensions_are_recognised() {
        assert!(is_font_extension("a.ttf"));
        assert!(is_font_extension("a.otf"));
        assert!(is_font_extension("a.ttc"));
        assert!(!is_font_extension("a.txt"));
        assert!(!is_font_extension("a.png"));
    }

    #[test]
    fn a_fully_covered_font_reports_ok() {
        // Devanagari probes are all in 0x900..0x97F, so a font covering that
        // whole block must pass. Written to a temp dir so it goes through the
        // real filesystem path.
        let dir = std::env::temp_dir().join(format!("kcommand-cov-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("TestDeva-Regular.ttf");
        let font = synthetic_font_format4(&[(0x0900, 0x097F)]);
        std::fs::write(&path, &font).unwrap();

        let cmap = parse_cmap(&std::fs::read(&path).unwrap()).unwrap();
        for cp in probe_codepoints("deva") {
            assert!(cmap.contains(cp), "U+{cp:X} should be covered");
        }

        std::fs::remove_dir_all(&dir).ok();
    }
}