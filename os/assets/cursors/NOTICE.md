# KorrinOS Cursor Themes — attribution and provenance

This directory builds the cursor themes that ship with KorrinOS. Some of the
artwork in it is **third-party**, and this file records where each piece came
from so the project is not shipping unattributed art.

Nothing here is downloaded at build time or at runtime. Every theme is built
from assets vendored under `os/assets/cursors/source/`.

---

## Third-party assets

### 1. Walter White Heisenberg Sketch — animated cursor

| | |
|---|---|
| **Source filename** | `Walter White Heisenberg Sketch Animated Cursor--cursor--SweezyCursors.ani` |
| **Vendored as** | `source/walter-animated.ani` |
| **Format** | Windows animated cursor (RIFF/ACON), 57584 bytes, 4 frames |
| **Embedded title** | `Walter White Heisenberg Sketch Animated Cursor` |
| **Embedded author** | `Sweezy Cursors` (the `IART` chunk) |
| **Origin** | SweezyCursors cursor pack |
| **Used by themes** | `korrinos-walter` (as `default`), `korrinos-walter-only` |

Character depicted is Walter White from *Breaking Bad*. Walter White, the name
and likeness, and the visual language of the show are the property of their
respective rights holders; this is a fan-made cursor used as system
skinning, not an official asset.

### 2. Witch Crystal Staff & Skeleton Hand — pointer cursor

| | |
|---|---|
| **Source filename** | `Witch Crystal Staff & Skeleton Hand Animated Cursor--pointer--SweezyCursors.cur` |
| **Vendored as** | `source/witch-staff.cur` |
| **Format** | Windows cursor (ICONDIR), 15502 bytes, 5 images at 16/32/48/64/128 |
| **Embedded author** | `Sweezy Cursors` |
| **Origin** | SweezyCursors cursor pack |
| **Used by themes** | `korrinos-walter` (as `pointer`/`left_ptr`), `korrinos-witch-only` |

Note: despite the original filename containing the word "Animated", the file
contains **no** animation — it is five sizes of a single static image. It is
installed as a static cursor.

### Licensing status — ACTION REQUIRED

The SweezyCursors artwork is redistributed here **without a verified licence**.
The distribution that supplied these files did not carry a licence file, and
Sweezy Cursors' redistribution terms for its packs are not reproduced in this
repository. Before KorrinOS is published, the licence for both assets must be
checked and recorded here, and the NOTICE file in the shipped image should
carry the terms alongside this provenance record.

If the terms cannot be confirmed, these two themes must be removed from the
default build; the themes described below are unaffected.

---

## First-party (synthesised) assets

These themes are **generated programmatically** by
`os/assets/cursors/synth-cursors.py` from geometric primitives drawn with
Pillow. They contain no third-party artwork and are covered by the KorrinOS
project licence. They are marked as synthesised in three places: the theme's
`index.theme` (`X-KorrinOS-Synthesised=true`), the installed `manifest.json`
(`"origin": "synthesised"`), and in the Settings picker UI.

| Theme | Description |
|---|---|
| `korrinos-cursor-hc` | High Contrast — white fill, black outline |
| `korrinos-cursor-large` | Large Target — larger and bolder, for low vision |
| `korrinos-cursor-mono` | Monochrome — near-black and off-white only |

---

## What each theme contains

All six themes provide the same 59 X11 cursor names at 12 sizes
(16, 20, 24, 28, 32, 40, 48, 64, 96, 128, 192, 256), so the theme looks right
for text selection, resizing, drag-and-drop and busy states rather than only
under the mouse pointer.

| Theme | Origin | `default` | `pointer` / `left_ptr` |
|---|---|---|---|
| `korrinos-walter` | artist | Walter White, animated, 4 frames @ 100 ms | Witch Crystal Staff, static |
| `korrinos-walter-only` | artist | Walter White, animated | Walter White, animated |
| `korrinos-witch-only` | artist | Witch Crystal Staff, static | Witch Crystal Staff, static |
| `korrinos-cursor-hc` | synthesised | synthesised | synthesised |
| `korrinos-cursor-large` | synthesised | synthesised | synthesised |
| `korrinos-cursor-mono` | synthesised | synthesised | synthesised |

`korrinos-walter` is the KorrinOS default.

---

## How these were built

X11, Wayland, GTK and Qt do not read Windows cursor files. They read Xcursor
(`.xcursor`), a different container, so the Windows files had to be converted:

* `convert-sweezy.py` parses the RIFF/ACON `.ani` and the ICONDIR `.cur`,
  extracts each image and its hotspot (PNG and DIB/BMP payloads, 1/4/8 bpp
  palette and 16/24/32 bpp truecolour), and emits one image per frame and size.
  It also resizes to the standard size ladder and rescales the hotspot to
  match. Native source sizes are used byte-for-byte; only genuinely absent
  sizes are resampled, and always from a *larger* source image, so no small
  size is ever invented by scaling up.
* `synth-cursors.py` draws the first-party themes.
* `xcursor.py` writes the `.xcursor` container. It does **not** shell out to
  `xcursorgen`: `xcursorgen` 1.0.7 fails on every input under libpng 1.6.x
  (see the module docstring), so the container is written directly against the
  layout libXcursor actually accepts.
* `make-themes.py` assembles the six themes and writes `manifest.json`.

The animated cursor's frame delay comes from the `.ani` header: `JifRate` is 6
jiffies and one jiffy is 1/60 s, giving **100 ms per frame**. The per-icon rate
DWORD in these files is absent/garbage (`0x80800005`), so the tool validates
candidate offsets rather than trusting the `afIcon` flag, and falls back to
`JifRate`.