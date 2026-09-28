# NOTICE — kcommand

## This work is a derivative work

`kcommand` is a fork and derivative work of **Alacritty**.

- Upstream project: Alacritty — https://alacritty.org
- Upstream source: https://github.com/alacritty/alacritty
- Pinned upstream revision: `alacritty_terminal_v0.25.1` (package version `0.16.0`)
- Upstream authors: Christian Duerr `<contact@christianduerr.com>`, Joe Wilm `<joe@jwilm.com>`
- Upstream licence: **Apache-2.0** (the `alacritty` and `alacritty_terminal`
  crates). The `alacritty_config` crate is dual-licensed `MIT OR Apache-2.0`.

Both upstream licence texts are retained verbatim in this directory:

- `LICENSE-APACHE`
- `LICENSE-MIT`

Apache-2.0 §4(d) requires that recipients of a derivative work receive a copy
of the licence. Apache-2.0 §4(b)/(c) require attribution to the original
authors and notification of modification. This file satisfies those
obligations for the KorrinOS distribution.

## Modifications made by KorrinOS

The following changes have been made to the upstream source. This list is the
required "state changes" notice.

1. **Binary renamed to `kcommand`.**
   `alacritty/Cargo.toml` gains a `[[bin]]` override so the produced binary is
   `kcommand`. The internal crate names are deliberately left as upstream, so
   that rebasing on future Alacritty releases remains tractable.

2. **User-facing identity strings changed** from "Alacritty" to "kcommand":
   - `alacritty/src/config/window.rs` — `DEFAULT_NAME` (window title and class)
   - `alacritty/src/ipc.rs` — macOS IPC socket prefix
   - `alacritty/src/main.rs` — startup log line
   - `alacritty/src/cli.rs` — `#[clap(name = "kcommand", ...)]`

3. **Configuration path changed** to `kcommand`:
   `$XDG_CONFIG_HOME/kcommand/kcommand.toml`, `$HOME/.config/kcommand/kcommand.toml`,
   `/etc/kcommand/kcommand.toml`. The pre-rename Alacritty locations are still
   consulted as a fallback (see `legacy_alacritty_config` in
   `alacritty/src/config/mod.rs`) so an existing upstream configuration is not
   silently discarded.

4. **New module: per-script typography policy.**
   `alacritty/src/renderer/text/kcommand_typography.rs` adds a policy layer over
   the metrics that FreeType measures. It reads the canonical 55-language
   registry at `/usr/share/korrinos/i18n/languages.tsv` and scales the line
   height for the active language's script, so that Indic, Thai, Lao, Myanmar
   and Ethiopic marks are not clipped by a Latin-derived line height.
   It does not reimplement font measurement.

5. **Hook point** in `alacritty/src/renderer/text/glyph_cache.rs`
   (`load_font_metrics`) applies that policy to freshly measured metrics.

## Third-party components

This fork pulls in upstream Alacritty's own dependency tree, each under its own
licence. Notably `crossfont` (font rasterisation and metrics) is consumed
unmodified from crates.io and is **not** forked; KorrinOS layers policy on top
of it rather than vendoring or replacing it.

## KorrinOS

KorrinOS modifications are distributed under the same Apache-2.0 terms as the
upstream work they are based on.
