# VOKK v4 — product mark

The assistant mark: a glowing four-lobed star with a small four-point sparkle
at the centre, white/cyan on near-black.

**Canonical name: VOKK v4.** See `os/docs/AI-NAMING.md` for the full naming
rule and the retired aliases (Tinker AI, TinkerAI, Parc AI, Zegrate). The user
confirmed on 2026-09-28 that VOKK v4, Parc AI, TinkerAI, tinkeria and Zegrate
are all names for this one product; VOKK v4 is the one to print.

## Files

| File | Purpose |
|------|---------|
| `vokk-v4-mark.png` | 302x299 crop of the mark, as delivered |
| `vokk-v4-banner-1200x630.jpg` | the original share preview |
| `mark-512/256/128/64/32/16.png` | square raster sizes, dark field (#090a10) |

## IMPORTANT — derived, not master artwork

The share link only exposed a **1200x630 JPEG preview**, not the original
vector. `vokk-v4-mark.png` is a **crop** of that preview and carries JPEG
artefacts.

**This mark is unusable below 32px.** Measured ink coverage:

| size | foreground px | usable? |
|------|---------------|---------|
| 64px | 270 (6.6%) | yes |
| 32px | 68 (6.6%) | marginal |
| 16px | **0 (0.0%)** | **no — renders blank** |

The outline is a ~1px hairline. It needs a **solid simplified variant** for
favicon, taskbar and app-grid use. `mark-16.png` and `mark-32.png` are
placeholders and should not be shipped.

## Still needed from the original art

1. master file (SVG or >=1024px PNG) to regenerate from
2. simplified solid mark for 16-48px
3. monochrome + solid-fill variant for GDM / GRUB / Plymouth (1-bit friendly)
4. inverted variant — the mark is near-white and vanishes on light backgrounds
5. favicon + app icon wiring (nothing in the repo references these yet)

## Do not

Do not recreate this mark in CSS or HTML. Rebuilding precise star geometry in
markup produces a worse result than the raster. Use the artwork.
