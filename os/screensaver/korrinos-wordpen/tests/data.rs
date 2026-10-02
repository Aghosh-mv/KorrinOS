//! Checks on the shipped data files.
//!
//! Data files rot. A language gets added to the registry and nobody adds its
//! words, or an editor saves a file with a tab in the wrong place, or a word
//! grows long enough to be an unreadable smear. None of that fails a build by
//! itself - it just quietly means fewer languages, or a word that never draws.
//! So the data is tested like code.

use wordpen::engine::WordBank;
use wordpen::words::{load_languages, load_words};
use std::path::{Path, PathBuf};

fn data_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR is the crate root, so this works from any cwd.
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data")
}

fn read(name: &str) -> String {
    std::fs::read_to_string(data_dir().join(name)).unwrap_or_default()
}

#[test]
fn the_language_registry_ships_and_parses() {
    let languages = load_languages(&read("languages.tsv"));
    assert_eq!(
        languages.len(),
        55,
        "the saver must cover every shipped locale, not a hand-picked few"
    );
}

#[test]
fn every_language_has_a_font_chain() {
    for language in load_languages(&read("languages.tsv")) {
        assert!(
            !language.families.is_empty(),
            "{} has no font family, so it could never be drawn",
            language.code
        );
        assert!(
            language.families.len() >= 2,
            "{} has no fallback, so one missing font means no words at all",
            language.code
        );
    }
}

#[test]
fn the_word_file_covers_every_language() {
    let languages = load_languages(&read("languages.tsv"));
    let bank = load_words(&read("words.tsv"));
    let missing: Vec<&str> = languages
        .iter()
        .map(|l| l.code.as_str())
        .filter(|code| !bank.contains_key(*code))
        .collect();
    assert!(
        missing.is_empty(),
        "these languages are in the registry but have no words: {missing:?}"
    );
}

#[test]
fn no_words_reference_a_language_that_does_not_exist() {
    let known: Vec<String> =
        load_languages(&read("languages.tsv")).into_iter().map(|l| l.code).collect();
    let bank = load_words(&read("words.tsv"));
    let orphans: Vec<&String> = bank.keys().filter(|k| !known.contains(k)).collect();
    assert!(orphans.is_empty(), "words reference unknown languages: {orphans:?}");
}

#[test]
fn every_word_survives_the_loader() {
    // A word the loader rejects is a word that can never be drawn, so a word
    // file that quietly loses entries is worse than an empty one.
    let bank = load_words(&read("words.tsv"));
    let total: usize = bank.values().map(Vec::len).sum();
    let raw = read("words.tsv")
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .count();
    assert_eq!(total, raw, "the loader dropped {raw} words it should have kept");
    assert!(total > 400, "expected a substantial word list, got {total}");
}

#[test]
fn no_duplicate_words() {
    let mut bank: WordBank = load_words(&read("words.tsv"));
    for (code, words) in bank.iter_mut() {
        let before = words.len();
        words.sort();
        words.dedup();
        assert_eq!(words.len(), before, "{code} has duplicate words, which skews the draw");
    }
}

#[test]
fn every_word_is_drawable_in_principle() {
    // A word made only of combining marks, or of punctuation the font lacks, is
    // unusable. This does not prove the font HAS it - that needs the font, and
    // is checked at runtime - but it catches the obvious authoring mistakes.
    let bank = load_words(&read("words.tsv"));
    for (code, words) in &bank {
        for word in words {
            assert!(
                word.chars().any(|c| c.is_alphanumeric() || is_letter_like(c)),
                "{code}: {word:?} has no letters at all"
            );
            assert!(
                !word.chars().all(|c| !c.is_ascii() && !is_letter_like(c)),
                "{code}: {word:?} is entirely combining marks"
            );
        }
    }
}

#[test]
fn the_adult_set_is_a_separate_off_by_default_file() {
    // Kept apart on purpose: words on an unattended screen are a deliberate
    // choice, so they live in their own file and the saver only reads it when
    // the option is on.
    // Check the DATA rows, not the comments: words.tsv legitimately mentions
    // the adult set in its header to explain where it went.
    let main_text = read("words.tsv");
    let main_rows: Vec<&str> = main_text
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .collect();
    assert!(!main_rows.is_empty());
    let adult = read("words-adult.tsv");
    assert!(
        !main_rows.iter().any(|l| l.to_lowercase().contains("adult")),
        "the default word list must not contain the adult set"
    );
    // And the two files must not overlap, or "off by default" means nothing.
    let adult_words = load_words(&adult);
    let main_bank = load_words(&main_text);
    for (code, words) in &adult_words {
        if let Some(main_words) = main_bank.get(code) {
            let overlap: Vec<&String> =
                words.iter().filter(|w| main_words.contains(w)).collect();
            assert!(
                overlap.is_empty(),
                "{code}: these words are in BOTH files, so they are on by default: {overlap:?}"
            );
        }
    }
    assert!(
        adult.starts_with('#'),
        "the adult file must be commented so its intent is obvious in the file"
    );
}

#[test]
fn the_adult_set_parses_and_is_well_formed() {
    let bank = load_words(&read("words-adult.tsv"));
    if bank.is_empty() {
        return; // an empty, disabled set is valid
    }
    for (code, words) in &bank {
        for word in words {
            assert!(!word.chars().any(|c| c.is_control()));
            assert!(word.chars().count() <= 14, "{code}: {word:?} is too long");
        }
    }
}

fn is_letter_like(c: char) -> bool {
    c.is_alphabetic()
        // Scripts without case still have alphabetic blocks; these ranges cover
        // the ones a Latin-only is_alphabetic can miss depending on Unicode
        // tables, plus the marks a real word may begin with.
        || matches!(c as u32,
            0x0900..=0x0DFF | 0x0E00..=0x0FFF | 0x1000..=0x109F |
            0x1780..=0x17FF | 0x3040..=0x30FF | 0xAC00..=0xD7AF |
            0x0600..=0x06FF | 0x0590..=0x05FF)
}
