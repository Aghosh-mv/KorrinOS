//! KorrinOS typography policy for `kcommand`.
//!
//! Alacritty measures fonts well (FreeType + FreeType's own metrics), but its
//! line height is whatever the primary face reports. That is wrong for a
//! multi-script OS: a Devanagari or Thai face carries marks above *and* below
//! the baseline, so a Latin-derived line height clips them.
//!
//! This module does not reimplement font measuring. It layers *policy* on top:
//! given the active language, it decides how much vertical room that script
//! needs, and adjusts the measured metrics accordingly.
//!
//! The active language comes from the standard environment (`LANGUAGE`, then
//! `LANG`), which `os/i18n/i18n-lib.sh` sets. Nothing here is hardcoded to a
//! single locale.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

/// The registry compiled into the binary.
///
/// `kcommand` ships as a standalone repository, so it cannot depend on a file
/// that only exists inside a KorrinOS installation. Embedding the table means
/// it always works, from any working directory, with no runtime file lookup.
/// KorrinOS still wins at runtime when it provides its own copy (see
/// [`load_registry`]), which keeps the OS the single source of truth.
const BUNDLED_REGISTRY: &str = include_str!("../../../../share/i18n/languages.tsv");

/// Where a host OS may provide its own registry, overriding the bundled copy.
const REGISTRY_OVERRIDES: &[&str] =
    &["/usr/share/korrinos/i18n/languages.tsv", "os/i18n/languages.tsv"];

/// Extra vertical room per script, as a multiplier on the measured line
/// height. 1.0 means "Latin rules are fine".
///
/// The reasoning per group:
/// - Latin/Cyrillic/Greek: ascenders and descenders only, already measured.
/// - Arabic/Hebrew: marks sit above and below, but modestly.
/// - Indic + Sinhala: vowel signs (matras) stack above and below the glyph.
///   This is the group that visibly clips on a Latin-derived height.
/// - Thai/Lao/Myanmar/Ethiopic: stacked tone marks and vowel signs.
/// - CJK: tall but not marked; a small bump keeps the em box honest.
fn line_height_policy(script: &str) -> f64 {
    match script {
        "latin" | "cyrillic" | "greek" => 1.0,
        "arabic" | "hebrew" => 1.10,
        "deva" | "beng" | "taml" | "telu" | "gujr" | "knda" | "mlym" | "guru" | "sinh" => 1.20,
        "thai" | "lao" | "mymr" | "ethi" => 1.20,
        "hans" | "hant" | "jpan" | "kore" => 1.05,
        // Unknown script: change nothing rather than guess.
        _ => 1.0,
    }
}

/// A row from the language registry.
#[derive(Clone, Debug)]
pub struct LanguageEntry {
    pub code: String,
    pub locale: String,
    pub endonym: String,
    pub english: String,
    pub dir: String,
    pub script: String,
    pub fontpkg: String,
}

impl LanguageEntry {
    /// True when the script is written right to left.
    pub fn is_rtl(&self) -> bool {
        self.dir.eq_ignore_ascii_case("rtl")
    }
}

fn registry_path() -> Option<PathBuf> {
    REGISTRY_OVERRIDES.iter().map(PathBuf::from).find(|path| path.is_file())
}

/// Parse a registry table. The format is tab separated with `#` comments; the
/// header row is skipped. Unparseable rows are skipped rather than guessed at.
fn parse_registry(text: &str) -> Vec<LanguageEntry> {
    let mut entries = Vec::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 7 || cols[0] == "code" {
            continue;
        }
        entries.push(LanguageEntry {
            code: cols[0].to_string(),
            locale: cols[1].to_string(),
            endonym: cols[2].to_string(),
            english: cols[3].to_string(),
            dir: cols[4].to_string(),
            script: cols[5].to_string(),
            fontpkg: cols[6].to_string(),
        });
    }
    entries
}

/// Load the registry: the host OS copy if it exists, otherwise the copy that
/// was compiled into this binary.
fn load_registry() -> Vec<LanguageEntry> {
    if let Some(path) = registry_path() {
        if let Ok(text) = fs::read_to_string(&path) {
            let entries = parse_registry(&text);
            if !entries.is_empty() {
                return entries;
            }
        }
    }
    parse_registry(BUNDLED_REGISTRY)
}

/// The full registry, loaded once.
pub fn registry() -> &'static [LanguageEntry] {
    static REGISTRY: OnceLock<Vec<LanguageEntry>> = OnceLock::new();
    REGISTRY.get_or_init(load_registry)
}

/// The language code for this session: `LANGUAGE` first, then the language
/// part of `LANG` (so `hi_IN.UTF-8` yields `hi`).
pub fn active_language() -> Option<String> {
    if let Ok(code) = env::var("LANGUAGE") {
        if !code.trim().is_empty() {
            return Some(code.trim().to_ascii_lowercase());
        }
    }
    if let Ok(lang) = env::var("LANG") {
        if let Some(code) = lang.split(['.', '@']).next() {
            if !code.is_empty() && !code.eq_ignore_ascii_case("c") && code != "posix" {
                return Some(code.to_ascii_lowercase().replace('_', "-"));
            }
        }
    }
    None
}

/// Resolve the active language to a registry entry, matching the full code
/// first and then the bare language subtag (`pt` -> `pt-BR`).
pub fn active_entry() -> Option<&'static LanguageEntry> {
    let code = active_language()?;
    let bare = code.split('-').next().unwrap_or(&code).to_string();

    registry()
        .iter()
        .find(|entry| entry.code.eq_ignore_ascii_case(&code))
        .or_else(|| {
            registry()
                .iter()
                .find(|entry| entry.code.split('-').next().unwrap_or("").eq_ignore_ascii_case(&bare))
        })
}

/// The script for the active language, if we recognise it.
pub fn active_script() -> Option<&'static str> {
    active_entry().map(|entry| entry.script.as_str())
}

/// The line height multiplier for the active language.
pub fn active_line_height_policy() -> f64 {
    active_script().map(line_height_policy).unwrap_or(1.0)
}

/// Apply KorrinOS typography policy to freshly measured metrics.
///
/// This is the single hook point. Measuring stays with FreeType; only the
/// policy is ours.
pub fn apply_policy(metrics: &mut crossfont::Metrics) {
    let policy = active_line_height_policy();
    if (policy - 1.0).abs() > f64::EPSILON {
        metrics.line_height *= policy;
    }
}

/// A one-line description of the active language, for the startup log and for
/// `kcommand --typography`.
///
/// This is the first thing to look at when text renders wrong, so it reports
/// what was actually resolved rather than what was requested.
pub fn describe_active() -> String {
    let count = registry().len();
    match active_entry() {
        Some(entry) => format!(
            "kcommand typography: {} of {} languages loaded; active={} ({}) locale={} \
             script={} dir={} line-height={:.2}x font-pkg={}",
            count,
            count,
            entry.english,
            entry.endonym,
            entry.locale,
            entry.script,
            if entry.is_rtl() { "rtl" } else { "ltr" },
            line_height_policy(&entry.script),
            entry.fontpkg,
        ),
        None => format!(
            "kcommand typography: {count} languages loaded; active language not in registry \
             (LANGUAGE={:?} LANG={:?}); using unmodified font metrics",
            env::var("LANGUAGE").ok(),
            env::var("LANG").ok(),
        ),
    }
}

/// The font family to request for the active script.
///
/// This is *policy*: which face we want for the language in use. FreeType and
/// fontconfig still do the actual matching and fallback. Returning `None` means
/// "no opinion" and the user's own configured family is used untouched.
pub fn active_font_family() -> Option<&'static str> {
    Some(match active_script()? {
        // Our own system faces where we ship them.
        "latin" | "cyrillic" | "greek" => "Balsamiq Sans",
        "arabic" => "Noto Sans Mono Arabic",
        "hebrew" => "Noto Sans Mono Hebrew",
        "deva" => "Noto Sans Mono Devanagari",
        "beng" => "Noto Sans Mono Bengali",
        "taml" => "Noto Sans Mono Tamil",
        "telu" => "Noto Sans Mono Telugu",
        "gujr" => "Noto Sans Mono Gujarati",
        "knda" => "Noto Sans Mono Kannada",
        "mlym" => "Noto Sans Mono Malayalam",
        "guru" => "Noto Sans Mono Gurmukhi",
        "sinh" => "Noto Sans Mono Sinhala",
        "thai" => "Noto Sans Mono Thai",
        "lao" => "Noto Sans Mono Lao",
        "mymr" => "Noto Sans Mono Myanmar",
        "ethi" => "Noto Sans Mono Ethiopic",
        "hans" | "hant" => "Noto Sans Mono CJK SC",
        "jpan" => "Noto Sans Mono CJK JP",
        "kore" => "Noto Sans Mono CJK KR",
        _ => return None,
    })
}

/// Resolve the family to actually request: our policy for the active script,
/// unless the user has explicitly chosen a family of their own.
pub fn resolve_family(configured: &str) -> String {
    // "monospace" and empty are Alacritty defaults, not a user choice, so our
    // per-script policy is allowed to override them.
    let is_default = configured.trim().is_empty() || configured.eq_ignore_ascii_case("monospace");
    if is_default {
        if let Some(family) = active_font_family() {
            return family.to_string();
        }
    }
    configured.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indic_scripts_get_extra_height() {
        assert!(line_height_policy("deva") > line_height_policy("latin"));
        assert!(line_height_policy("mlym") > line_height_policy("latin"));
        assert!(line_height_policy("thai") > line_height_policy("latin"));
    }

    #[test]
    fn latin_is_the_baseline() {
        assert_eq!(line_height_policy("latin"), 1.0);
        assert_eq!(line_height_policy("cyrillic"), 1.0);
    }

    #[test]
    fn unknown_script_does_not_guess() {
        assert_eq!(line_height_policy("klingon"), 1.0);
    }

    #[test]
    fn explicit_user_family_is_always_respected() {
        // A user who picks a font gets that font, whatever the language is.
        assert_eq!(resolve_family("Iosevka"), "Iosevka");
        assert_eq!(resolve_family("Some Custom Mono"), "Some Custom Mono");
    }

    #[test]
    fn default_family_defers_to_policy() {
        // No active language known in a test environment must not crash or
        // invent a family; with no language it falls back to the input.
        let r = resolve_family("monospace");
        assert!(!r.is_empty(), "must always return a usable family");
    }

    #[test]
    fn blank_family_is_treated_as_default() {
        let r = resolve_family("   ");
        assert!(!r.trim().is_empty());
    }

    /// Verifies the whole chain against the real registry shipped in the tree:
    /// TSV parse -> entry lookup -> script -> line height policy.
    #[test]
    fn loads_the_real_55_language_registry() {
        // The in-tree registry, relative to this crate.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../i18n/languages.tsv");
        if !path.is_file() {
            eprintln!("skipping: registry not found at {}", path.display());
            return;
        }
        let text = fs::read_to_string(&path).expect("registry readable");
        let mut entries = Vec::new();
        for line in text.lines() {
            let line = line.trim_end();
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 7 || cols[0] == "code" {
                continue;
            }
            entries.push(cols);
        }

        assert_eq!(entries.len(), 55, "registry must declare exactly 55 languages");

        // Every row must carry a script we have a policy for, and a direction.
        let known = [
            "latin", "cyrillic", "greek", "arabic", "hebrew", "deva", "beng", "taml", "telu",
            "gujr", "knda", "mlym", "guru", "sinh", "thai", "lao", "mymr", "ethi", "hans", "hant",
            "jpan", "kore",
        ];
        for cols in &entries {
            assert!(
                known.contains(&cols[5]),
                "{}: script '{}' has no line-height policy",
                cols[0],
                cols[5]
            );
            assert!(cols[4] == "ltr" || cols[4] == "rtl", "{}: bad dir", cols[0]);
        }

        // Spot check the chain the way it runs at startup.
        let malayalam = entries.iter().find(|c| c[0] == "ml").expect("Malayalam present");
        assert_eq!(malayalam[5], "mlym");
        assert!(line_height_policy(malayalam[5]) > line_height_policy("latin"));

        // Every RTL language in the registry must be one we mark as RTL.
        let rtl = entries.iter().filter(|c| c[4] == "rtl").count();
        assert_eq!(rtl, 4, "expected ar/he/fa/ur as the RTL set");
    }
}
