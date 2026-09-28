# kcommand

**KorrinOS's terminal.** A fork of [Alacritty](https://alacritty.org) that
KorrinOS maintains as its own project.

`kcommand` is a **separate repository** on purpose. KorrinOS consumes it by
pinned commit and builds it from source; it is not vendored into the KorrinOS
tree. That keeps three things clean:

- **Licence boundary.** `kcommand` is Apache-2.0. KorrinOS's own licence is
  unaffected, and the Apache-2.0 code is never placed inside the GPL-2.0 kernel
  subtree. It is a separate program that talks to the kernel at arm's length,
  the same way every terminal on every Linux distribution does.
- **Upstream tracking.** The internal crate names are left as upstream
  (`alacritty`, `alacritty_terminal`, `alacritty_config`) so rebasing on new
  Alacritty releases stays tractable. Only the binary is renamed.
- **Independent usefulness.** `kcommand` builds and runs on its own. It does
  not require KorrinOS.

## What KorrinOS has changed

See [`NOTICE.md`](NOTICE.md) for the full Apache-2.0 §4 attribution and change
notice. In short:

1. Binary, window title, CLI, IPC name and config path renamed to `kcommand`.
2. Config now reads `~/.config/kcommand/kcommand.toml`, still falling back to
   the legacy Alacritty location so existing settings survive the rename.
3. A per-script typography policy layer (`kcommand_typography.rs`).

## The typography layer

This is the substantive KorrinOS work.

Alacritty measures fonts well — FreeType does the real work. But it takes its
line height from whichever face is primary, which is wrong for a multi-script
OS. A Devanagari or Thai face carries vowel signs and tone marks **above and
below** the baseline, so a Latin-derived line height clips them.

`kcommand` layers *policy* on top of the measurement rather than reimplementing
it:

| Script group | Line height | Why |
|---|---|---|
| Latin, Cyrillic, Greek | 1.00× | ascenders/descenders only, already measured |
| Arabic, Hebrew | 1.10× | marks above and below, modestly |
| Indic (Devanagari, Bengali, Tamil, Telugu, Gujarati, Kannada, Malayalam, Gurmukhi), Sinhala | 1.20× | matras stack above *and* below the glyph |
| Thai, Lao, Myanmar, Ethiopic | 1.20× | stacked tone marks and vowel signs |
| CJK, Hangul | 1.05× | tall, but not marked |

Policy is driven by the language registry (`share/i18n/languages.tsv`, 55
languages). The active language is read from the standard environment
(`LANGUAGE`, then `LANG`) — nothing is hardcoded to one locale.

The registry is **embedded in the binary** via `include_str!`, so `kcommand`
works standalone from any working directory. If a host OS provides
`/usr/share/korrinos/i18n/languages.tsv`, that copy wins, keeping the OS the
single source of truth.

## Build

```sh
cargo build --release --bin kcommand
```

Requires Rust (developed against 1.95) and the usual Alacritty system
dependencies: `fontconfig`, `freetype`, `libx11`, `libxkbcommon`,
`wayland-client`, `libepoxy`.

## Test

```sh
cargo test --release --bin kcommand kcommand_typography
```

The suite includes a test that parses the real 55-language registry and asserts
the contract: exactly 55 rows, every row has a script with a height policy, and
the RTL set is exactly Arabic, Hebrew, Persian and Urdu. If the registry and the
terminal ever disagree, the build fails rather than rendering wrong text.

## Diagnostics

On startup `kcommand` logs what it resolved, which is the first thing to check
when text renders incorrectly:

```
kcommand typography: 55 of 55 languages loaded; active=Malayalam (മലയാളം)
locale=ml_IN.UTF-8 script=mlym dir=ltr line-height=1.20x font-pkg=fonts-lohit-mlym
```

## Licence

Apache-2.0. Derived from Alacritty by Christian Duerr and Joe Wilm. See
[`NOTICE.md`](NOTICE.md), [`LICENSE`](LICENSE), and
[`README.upstream.md`](README.upstream.md).
