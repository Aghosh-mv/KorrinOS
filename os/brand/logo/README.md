# KorrinOS logo assets

Source: the "Neon Orbital Planet Emblem" artwork the user provided
(shared from a ChatGPT conversation, published 2026-09-28).

## What is here

| File | Purpose |
|------|---------|
| `korrinos-emblem.png` | 284x336 crop of the emblem, as delivered |
| `korrinos-banner-1200x630.jpg` | the original 1200x630 share preview |
| `emblem-512/256/128/64/32/16.png` | square raster sizes, dark field (#090a10) |

## IMPORTANT — these are derived, not master artwork

The share link only exposed a **1200x630 JPEG preview** rendered by ChatGPT,
not the original vector or full-resolution file. Consequences:

- the emblem occupies a small part of the 1200x630 canvas, so `korrinos-emblem.png`
  is a **crop** of that preview;
- JPEG artefacts mean the thin neon strokes are slightly soft and the flat
  background is not perfectly uniform;
- **32px and 16px are too small for this artwork.** The strokes are ~1px hair
  lines with a glow; at 16px they turn into a grey smudge. Do not use the small
  rasters as-is.

## Still needed from the original art

Ask the user for the master file (SVG or >=1024px PNG) and regenerate from it.
Until then the design is not production-ready, because:

1. **no small-size mark** — a hairline emblem needs a simplified, solid variant
   for favicon / taskbar / app-grid (16-48px)
2. **no monochrome variant** — required for the GDM greeter, GRUB, Plymouth
   (those are typically 1-bit or white-on-black) and for light themes
3. **no solid-fill variant** — the glow does not survive 1-bit rendering
4. **no inverted variant** — the mark is near-white; it is invisible on light
   backgrounds
5. **no favicon** — nothing in `brand/` or the repo is wired as a favicon or
   app icon yet

## Do not

Do not recreate this mark in CSS or HTML. The orbital ellipses and the three
blades are precise geometry; rebuilding them in markup produces a worse result
than the raster. Use the artwork.
