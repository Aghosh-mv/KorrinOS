//! Languages and words.
//!
//! Word lists are data, not code, so they can be edited without touching the
//! engine. The loader is strict on purpose: a language with no word list, or a
//! word the font cannot draw, is skipped rather than rendered as a box.

use crate::engine::{Language, WordBank};
use std::collections::BTreeMap;
use std::path::Path;

/// One row of the language registry: `code`, `endonym`, `english`, families.
///
/// Families are `|`-separated so one field can express the fallback chain.
pub fn parse_language(line: &str) -> Option<Language> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let mut cols = line.split('\t');
    let code = cols.next()?.trim();
    let endonym = cols.next()?.trim();
    let english = cols.next()?.trim();
    if code.is_empty() || english.is_empty() {
        return None;
    }
    // Optional trailing field: the fallback family chain.
    let families = cols
        .next()
        .map(|f| f.split('|').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default();
    Some(Language {
        code: code.to_string(),
        endonym: endonym.to_string(),
        english: english.to_string(),
        families,
    })
}

/// Load the language registry. Rows that do not parse are skipped, not fatal:
/// one bad row should not take out the other 54 languages.
pub fn load_languages(text: &str) -> Vec<Language> {
    text.lines().filter_map(parse_language).collect()
}

/// Load a word bank. Format is `code<TAB>word` per line; `#` starts a comment.
///
/// A word containing a control character is rejected, because it would be
/// undrawable and, worse, could inject terminal escapes into a fullscreen app.
pub fn load_words(text: &str) -> WordBank {
    let mut bank: WordBank = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let Some((code, word)) = line.split_once('\t') else { continue };
        let code = code.trim();
        let word = word.trim();
        if code.is_empty() || word.is_empty() {
            continue;
        }
        if word.chars().any(|c| c.is_control()) {
            continue;
        }
        // The gate: at least one character, and not so long that it cannot be
        // read at a glance. Long words also render as unreadable smears.
        if word.chars().count() > 14 {
            continue;
        }
        bank.entry(code.to_string()).or_default().push(word.to_string());
    }
    bank
}

/// Read a file, returning empty text when it is absent.
///
/// A missing word file must not stop the saver; it just means fewer languages
/// are available tonight.
pub fn read_optional(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_language_row() {
        let lang = parse_language("ja\t日本語\tJapanese\tNoto Sans CJK JP|Noto Sans")
            .expect("should parse");
        assert_eq!(lang.code, "ja");
        assert_eq!(lang.endonym, "日本語");
        assert_eq!(lang.english, "Japanese");
        assert_eq!(lang.families, vec!["Noto Sans CJK JP", "Noto Sans"]);
    }

    #[test]
    fn a_language_may_have_no_family_chain() {
        let lang = parse_language("en\tEnglish\tEnglish").expect("parses");
        assert!(lang.families.is_empty());
    }

    #[test]
    fn comments_and_blank_lines_are_skipped() {
        let text = "# header\n\nen\tEnglish\tEnglish\tNoto Sans\n";
        let langs = load_languages(text);
        assert_eq!(langs.len(), 1);
        assert_eq!(langs[0].code, "en");
    }

    #[test]
    fn one_bad_row_does_not_take_out_the_others() {
        let text = "en\tEnglish\tEnglish\tNoto Sans\ngarbage\nfr\tFrançais\tFrench\tNoto Sans\n";
        let langs = load_languages(text);
        assert_eq!(langs.len(), 2, "the two good rows must survive");
    }

    #[test]
    fn words_load_into_per_language_buckets() {
        let text = "# words\nen\tHELLO\nen\tWORLD\nfr\tBONJOUR\n";
        let bank = load_words(text);
        assert_eq!(bank.get("en").map(Vec::len), Some(2));
        assert_eq!(bank.get("fr").map(Vec::len), Some(1));
    }

    #[test]
    fn control_characters_in_a_word_are_rejected() {
        // A word carrying an escape sequence must never reach the renderer.
        let text = "en\tHELLO\nen\tBAD\u{1b}ESC\nen\tTAB\tHERE\n";
        let bank = load_words(text);
        let words = bank.get("en").expect("bucket");
        assert!(words.iter().all(|w| !w.contains('\u{1b}')));
        assert!(!words.iter().any(|w| w.contains("ESC")));
    }

    #[test]
    fn overlong_words_are_rejected() {
        // Fifteen letters is an unreadable smear at any resolution.
        let text = "en\tSHORT\nen\tTHISISAVERYLONGWORDINDEED\n";
        let bank = load_words(text);
        assert_eq!(bank.get("en").map(Vec::len), Some(1));
    }

    #[test]
    fn a_missing_file_yields_no_languages_rather_than_an_error() {
        assert_eq!(read_optional(Path::new("/nonexistent/words.tsv")), "");
        assert!(load_languages(&read_optional(Path::new("/nonexistent/x.tsv"))).is_empty());
    }
}
