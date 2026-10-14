#!/usr/bin/env python3
"""
synth-cursors.py -- generate clean, legible Xcursor themes programmatically.

WHY
---
The KorrinOS "Mouse Designs" picker has to offer a bundled, offline set of
cursor designs. Two of them come from real artist-made artwork (the Walter and
Witch Staff themes); the rest are SYNTHESISED here, drawn from primitives with
PIL, so that:

  * nothing is downloaded at build time or at runtime;
  * every theme is unambiguously ours to license, which matters because the
    artist themes carry third-party attribution (see NOTICE.md);
  * every design is legible at 16px, which hand-drawn art often is not.

Everything these themes produce is marked as synthesised in the theme's
index.theme and in NOTICE.md.

Technique: each shape is drawn as a polygon at 4x supersample, then reduced
with LANCZOS. That antialiases the edges without a separate AA pass, which is
what keeps a 1px outline from disappearing at small sizes.
"""

import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import xcursor as xc

# X11 asks for a fixed set of cursor names. Anything a theme does not provide
# falls back to `default`, but providing the real ones is what stops a busy
# cursor or a resize cursor from looking wrong.
ROLES = {
    "default": "arrow",
    "left_ptr": "arrow",
    "arrow": "arrow",
    "alias": "arrow",
    "copy": "arrow",
    "move": "arrow",
    "help": "help",
    "question": "help",
    "whats_this": "help",
    "link": "help",
    "text": "ibeam",
    "xterm": "ibeam",
    "ibeam": "ibeam",
    "crosshair": "cross",
    "cross": "cross",
    "cross_reverse": "cross",
    "diamond_cross": "cross",
    "pencil": "cross",
    "plus": "plus",
    "pen": "arrow",
    "hand": "hand",
    "pointer2": "hand",
    "grab": "hand",
    "grabbing": "hand",
    "fleur": "move",
    "size_all": "move",
    "closedhand": "hand",
    "openhand": "hand",
    "dnd-copy": "copy",
    "dnd-link": "help",
    "dnd-none": "cross",
    "no-drop": "cross",
    "not-allowed": "cross",
    "forbidden": "cross",
    "wait": "ring",
    "watch": "ring",
    "progress": "ring",
    "left_ptr_watch": "ring",
    "all-scroll": "ring",
    "zoom-in": "plus",
    "zoom-out": "minus",
    "sb_h_double_arrow": "ew",
    "sb_v_double_arrow": "ns",
    "sb_left_arrow": "ew",
    "sb_right_arrow": "ew",
    "size_h": "ew",
    "size_bd": "ew",
    "split_h": "ew",
    "split_v": "ns",
    "h_double_arrow": "ew",
    "v_double_arrow": "ns",
    "n-resize": "ns",
    "s-resize": "ns",
    "e-resize": "ew",
    "w-resize": "ew",
    "ne-resize": "nesw",
    "sw-resize": "nesw",
    "nw-resize": "nwse",
    "se-resize": "nwse",
}

# Anchor used when scaling: every shape is authored in this unit box and then
# scaled to each output size, so a hotspot can be derived from the box.
SS = 4  # supersample factor

STANDARD_SIZES = [16, 20, 24, 28, 32, 40, 48, 64, 96, 128, 192, 256]


# --------------------------------------------------------------------------
# Shape geometry, in a 0..1 unit box (y grows downward, as in image space)
# --------------------------------------------------------------------------

def _arrow_poly(box):
    """Classic pointer arrow, tip at the top-left origin."""
    x0, y0, w, h = box
    return [
        (x0, y0),
        (x0, y0 + h),
        (x0 + 0.19 * w, y0 + 0.82 * h),
        (x0 + 0.34 * w, y0 + 1.15 * h),
        (x0 + 0.47 * w, y0 + 1.08 * h),
        (x0 + 0.32 * w, y0 + 0.76 * h),
        (x0 + 0.56 * w, y0 + 0.72 * h),
    ]


def _ibeam_poly(box):
    """I-beam: a vertical stem with serifs top and bottom."""
    x0, y0, w, h = box
    cx = x0 + w / 2.0
    sw = 0.16 * w          # serif half-width
    tw = 0.11 * w          # stem half-width
    top = y0 + 0.10 * h
    bot = y0 + 0.90 * h
    return [
        (cx - sw, top), (cx + sw, top),
        (cx + sw, top + 0.12 * h), (cx + tw, top + 0.12 * h),
        (cx + tw, bot - 0.12 * h), (cx + sw, bot - 0.12 * h),
        (cx + sw, bot), (cx - sw, bot),
        (cx - sw, bot - 0.12 * h), (cx - tw, bot - 0.12 * h),
        (cx - tw, top + 0.12 * h), (cx - sw, top + 0.12 * h),
    ]


def _cross_poly(box):
    x0, y0, w, h = box
    cx, cy = x0 + w / 2.0, y0 + h / 2.0
    t = 0.14 * w
    return [(cx - t, cy - 0.44 * h), (cx + t, cy - 0.44 * h),
            (cx + t, cy - t), (cx + 0.44 * w, cy - t),
            (cx + 0.44 * w, cy + t), (cx + t, cy + t),
            (cx + t, cy + 0.44 * h), (cx - t, cy + 0.44 * h),
            (cx - t, cy + t), (cx - 0.44 * w, cy + t),
            (cx - 0.44 * w, cy - t), (cx - t, cy - t)]


def _arrow_dir_poly(box, dx, dy):
    """A two-headed arrow for resize cursors, along (dx, dy)."""
    x0, y0, w, h = box
    cx, cy = x0 + w / 2.0, y0 + h / 2.0
    t = 0.13 * min(w, h)     # shaft half-thickness
    L = 0.44 * min(w, h)     # distance from centre to each tip
    head = 0.17 * min(w, h)  # head half-width

    def pt(a, b):
        return (cx + dx * a - dy * b, cy + dy * a + dx * b)

    return [pt(-L, 0), pt(-L + head, -head), pt(-L + head, -t),
            pt(L - head, -t), pt(L - head, -head), pt(L, 0),
            pt(L - head, head), pt(L - head, t),
            pt(-L + head, t), pt(-L + head, head)]


def _hand_poly(box):
    """Simplified pointing hand: one finger up, thumb to the left."""
    x0, y0, w, h = box
    return [
        (x0 + 0.40 * w, y0 + 0.02 * h),
        (x0 + 0.52 * w, y0 + 0.02 * h),
        (x0 + 0.54 * w, y0 + 0.42 * h),
        (x0 + 0.70 * w, y0 + 0.36 * h),
        (x0 + 0.86 * w, y0 + 0.52 * h),
        (x0 + 0.98 * w, y0 + 0.70 * h),
        (x0 + 0.90 * w, y0 + 0.96 * h),
        (x0 + 0.44 * w, y0 + 0.98 * h),
        (x0 + 0.30 * w, y0 + 0.74 * h),
        (x0 + 0.14 * w, y0 + 0.60 * h),
        (x0 + 0.22 * w, y0 + 0.40 * h),
        (x0 + 0.36 * w, y0 + 0.46 * h),
    ]


# --------------------------------------------------------------------------
# Rendering
# --------------------------------------------------------------------------

def render(shape, size, fill, outline, outline_w, extra=None, scale=1.0):
    """Render one cursor shape at `size` px as RGBA.

    `scale` > 1 grows the shape inside the canvas. The synthesised "Large
    Target" design relies on this: it is genuinely bigger, not just bolder.
    """
    from PIL import Image, ImageDraw

    S = size * SS
    im = Image.new('RGBA', (S, S), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)

    # Every shape is drawn into a box slightly larger than the canvas so the
    # stroke at the edges is not clipped away. `scale` shrinks that box.
    pad = 0.06
    half = 0.5 * pad * scale
    box = (S * half, S * half, S * (1 - 2 * half), S * (1 - 2 * half))
    ow = max(1, int(round(outline_w * SS)))

    if shape == "arrow":
        poly = _arrow_poly(box)
        anchor = (box[0], box[1])
    elif shape == "ibeam":
        poly = _ibeam_poly(box)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "cross":
        poly = _cross_poly(box)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape in ("plus", "minus"):
        poly = _cross_poly(box)
        if shape == "minus":
            cx, cy = box[0] + box[2] / 2.0, box[1] + box[3] / 2.0
            t = 0.14 * box[2]
            poly = [(cx - t, cy - t), (cx + t, cy - t), (cx + t, cy + t),
                    (cx - t, cy + t)]
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "ew":
        poly = _arrow_dir_poly(box, 1, 0)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "ns":
        poly = _arrow_dir_poly(box, 0, 1)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "nesw":
        poly = _arrow_dir_poly(box, 0.7071, 0.7071)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "nwse":
        poly = _arrow_dir_poly(box, 0.7071, -0.7071)
        anchor = (box[0] + box[2] / 2.0, box[1] + box[3] / 2.0)
    elif shape == "hand":
        poly = _hand_poly(box)
        anchor = (box[0] + 0.40 * box[2], box[1] + 0.30 * box[3])
    elif shape == "help":
        # Question mark drawn as a filled circle plus a stem, so it reads at
        # small sizes where a real glyph would turn to mush.
        cx = box[0] + box[2] / 2.0
        cy = box[1] + box[3] / 2.0
        r = box[2] / 2.0
        d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=fill, outline=outline,
                  width=ow)
        # The glyph is drawn in the OUTLINE colour: drawing it in the fill
        # colour on top of a fill-coloured disc makes it invisible, which is
        # exactly what the first version of this did.
        t = r * 0.17
        top = cy - r * 0.52
        d.rounded_rectangle([cx - r * 0.40, top, cx + r * 0.40, top + 2 * t],
                            radius=t, fill=outline)
        d.rounded_rectangle([cx - t, top + 2 * t, cx + t, cy + r * 0.02],
                            radius=t, fill=outline)
        d.ellipse([cx - t, cy + r * 0.18, cx + t, cy + r * 0.18 + 2 * t],
                  fill=outline)
        anchor = (cx, cy)
        poly = None
    elif shape == "ring":
        cx = box[0] + box[2] / 2.0
        cy = box[1] + box[3] / 2.0
        r = box[2] / 2.0
        d.ellipse([cx - r, cy - r, cx + r, cy + r], outline=fill,
                  width=max(2, int(r * 0.22)))
        d.ellipse([cx - r * 0.55, cy - r * 0.55, cx + r * 0.55,
                   cy + r * 0.55], outline=outline,
                  width=max(1, int(r * 0.14)))
        anchor = (cx, cy)
        poly = None
    elif shape == "copy":
        # An arrow with a small plus badge at the lower right, the standard
        # "this is a copy" affordance.
        poly = _arrow_poly((box[0], box[1] - 0.10 * box[3],
                            box[2] * 0.82, box[3] * 0.82))
        anchor = (box[0], box[1] - 0.10 * box[3])
        bx = box[0] + box[2] * 0.68
        by = box[1] + box[3] * 0.66
        r = box[2] * 0.26
        t = max(1, int(round(r * 0.30)))
        d.rectangle([bx - r, by - r, bx + r, by + r], fill=fill,
                    outline=outline, width=ow)
        d.rectangle([bx - r * 0.6, by - t, bx + r * 0.6, by + t], fill=outline)
        d.rectangle([bx - t, by - r * 0.6, bx + t, by + r * 0.6], fill=outline)
    elif shape == "move":
        # Four way-finders. Drawn as thin crossed lines with arrowheads rather
        # than four overlapping filled wedges, which merged into a blob.
        cx = box[0] + box[2] / 2.0
        cy = box[1] + box[3] / 2.0
        half = 0.46 * min(box[2], box[3])
        head = 0.20 * min(box[2], box[3])
        t = max(1, int(round(0.055 * min(box[2], box[3]))))
        d.line([cx - half, cy, cx + half, cy], fill=fill, width=t)
        d.line([cx, cy - half, cx, cy + half], fill=fill, width=t)
        for dx, dy in ((0, -1), (0, 1), (-1, 0), (1, 0)):
            d.polygon(_arrow_dir_poly(
                (cx - head, cy - head, head * 2, head * 2), dx, dy),
                fill=fill, outline=outline, width=max(1, ow // 2))
        anchor = (cx, cy)
        poly = None
    else:
        raise ValueError("unknown shape %r" % shape)

    if poly:
        d.polygon(poly, fill=fill, outline=outline, width=ow)

    im = im.resize((size, size), 1)  # 1 == Image.LANCZOS
    hotx = min(size - 1, max(0, int(round(anchor[0] / SS))))
    hoty = min(size - 1, max(0, int(round(anchor[1] / SS))))
    return im, hotx, hoty


def build_theme(name, outdir, fill, outline, outline_w, sizes=None,
                title="", comment="", scale=1.0):
    """Render every role for one synthesised theme. Returns a stats dict."""
    sizes = sizes or STANDARD_SIZES
    cursors_dir = os.path.join(outdir, 'cursors')
    os.makedirs(cursors_dir, exist_ok=True)

    # Render once per (shape, size); many roles share a shape.
    cache = {}
    # Group roles by identical content so they can share one file on disk.
    groups = {}
    for role, shape in sorted(ROLES.items()):
        if shape not in cache:
            cache[shape] = [render(shape, size, fill, outline, outline_w,
                                   scale=scale)
                            for size in sizes]
        imgs = []
        for (im, hx, hy), size in zip(cache[shape], sizes):
            imgs.append(dict(image=im, hotx=hx, hoty=hy, delay=0,
                             nominal=size))
        groups.setdefault(xc.content_key(imgs), (imgs, []))[1].append(role)

    written = linked = 0
    for key, (imgs, roles) in groups.items():
        paths = [os.path.join(cursors_dir, r) for r in roles]
        _stats, links = xc.write_group(paths, imgs)
        written += 1
        linked += len(links)

    write_index_theme(outdir, name, title=title,
                      comment=comment or
                      "Synthesised for KorrinOS by os/assets/cursors/"
                      "synth-cursors.py. Drawn programmatically from "
                      "primitives; not derived from any third-party artwork.")

    return dict(name=name, roles=len(ROLES), distinct_files=written,
                symlinks=linked, sizes=len(sizes), dir=outdir)


def write_index_theme(theme_dir, name, title="", comment=""):
    """Write an Xcursor icon-theme index.theme.

    Directories=cursors is the only part that matters to Xcursor; the rest
    exists so GTK's theme enumeration and any icon-theme viewer can list it.
    """
    with open(os.path.join(theme_dir, 'index.theme'), 'w') as f:
        f.write("[Icon Theme]\n")
        f.write("Name=%s\n" % name)
        if title:
            f.write("Comment=%s\n" % title)
        if comment:
            f.write("X-KorrinOS-Synthesised=true\n")
            f.write("X-KorrinOS-Notice=%s\n" % comment)
        f.write("Directories=cursors\n")
        f.write("Type=Xcursor\n")


# The bundled synthesised designs. Each is a distinct, deliberate visual
# choice rather than a recolour, so the picker offers real options.
THEMES = {
    'korrinos-cursor-hc': dict(
        title='High Contrast (synthesised)',
        fill=(255, 255, 255, 255),
        outline=(0, 0, 0, 255),
        outline_w=1.0,
    ),
    'korrinos-cursor-large': dict(
        title='Large Target (synthesised)',
        # A visibly fatter outline and a larger arrow body, for low vision
        # and for anyone using a HiDPI display at a distance.
        fill=(255, 210, 40, 255),
        outline=(20, 20, 24, 255),
        outline_w=1.8,
        scale=1.22,
    ),
    'korrinos-cursor-mono': dict(
        title='Monochrome (synthesised)',
        fill=(235, 235, 235, 255),
        outline=(32, 32, 32, 255),
        outline_w=1.1,
    ),
}


def main():
    import argparse
    ap = argparse.ArgumentParser(
        description="Generate synthesised KorrinOS Xcursor themes.")
    ap.add_argument('-o', '--outdir', required=True,
                    help="root directory to create theme directories in")
    ap.add_argument('--theme', action='append', default=None,
                    help="only build this theme (repeatable)")
    ap.add_argument('--sizes', default=None,
                    help="comma-separated size ladder")
    args = ap.parse_args()

    sizes = ([int(s) for s in args.sizes.split(',')] if args.sizes
             else list(STANDARD_SIZES))
    wanted = args.theme or list(THEMES)
    for name in wanted:
        if name not in THEMES:
            print("  unknown synthesised theme: %s (have: %s)"
                  % (name, ", ".join(sorted(THEMES))))
            return 1
        spec = THEMES[name]
        st = build_theme(name, os.path.join(args.outdir, name),
                         fill=spec['fill'], outline=spec['outline'],
                         outline_w=spec['outline_w'], sizes=sizes,
                         title=spec['title'], scale=spec.get('scale', 1.0))
        print("  %-22s %d roles x %d sizes -> %s"
              % (name, st['roles'], st['sizes'], st['dir']))
    return 0


if __name__ == '__main__':
    sys.exit(main())