//! `korrinos-wordpen` — the KorrinOS handwriting screen saver.
//!
//! Draws one word at a time, in one of the shipped languages, in a random bright
//! colour, on a black screen. Exits the instant the user touches anything: it
//! runs while you are away and is not a background process.
//!
//! Usage:
//!   korrinos-wordpen              run until dismissed
//!   korrinos-wordpen --preview N  draw N words then exit (for captures)
//!   korrinos-wordpen --info       report what can be drawn, then exit

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut preview: Option<usize> = None;
    let mut info = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--preview" => {
                index += 1;
                preview = args.get(index).and_then(|v| v.parse().ok());
            },
            "--info" => info = true,
            "--help" | "-h" => {
                println!("{}", HELP);
                return ExitCode::SUCCESS;
            },
            "--version" | "-V" => {
                println!("korrinos-wordpen {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            },
            other => {
                eprintln!("korrinos-wordpen: unknown argument {other:?}");
                eprintln!("{HELP}");
                return ExitCode::from(2);
            },
        }
        index += 1;
    }

    let settings = wordpen::screen::parse_settings(&wordpen::screen::read_optional(
        &wordpen::screen::config_path(),
    ));

    if info {
        let mut runner = wordpen::screen::Runner::new(settings);
        println!("data dir:   {:?}", runner.settings().data_dir);
        println!("languages:  {} registered", runner.language_count());
        println!("words:      {} available", runner.word_count());
        println!("drawable:   {} languages have a usable font", runner.drawable_languages());
        println!("adult set:  {}", if runner.settings().adult { "ENABLED" } else { "off (default)" });
        let missing = runner.undrawable_languages();
        if !missing.is_empty() {
            println!("
not drawable here (install the matching font package):");
            for (code, reason) in &missing {
                println!("  {code:<8} {reason}");
            }
        }
        return ExitCode::SUCCESS;
    }

    match wordpen::screen::run_on_display(settings, preview) {
        wordpen::screen::Outcome::Dismissed => ExitCode::SUCCESS,
        wordpen::screen::Outcome::Finished => ExitCode::SUCCESS,
        wordpen::screen::Outcome::NothingToDraw => ExitCode::FAILURE,
    }
}

const HELP: &str = "\
korrinos-wordpen — the KorrinOS handwriting screen saver

  (no arguments)   run until the user touches the keyboard or mouse
  --preview N      draw N words then exit, for capturing stills
  --info           report what this installation can draw, then exit
  --version        print the version

Configuration is read from $XDG_CONFIG_HOME/korrinos/wordpen.conf, or
$KORRINOS_WORDPEN_CONFIG if set. Keys:

  adult        true|false   include the opt-in adult word list (default false)
  seed         <integer>    fix the random sequence, for reproducible previews
  stroke_scale <float>      pen thickness as a fraction of screen height
  data_dir     <path>       where languages.tsv and words.tsv live";
