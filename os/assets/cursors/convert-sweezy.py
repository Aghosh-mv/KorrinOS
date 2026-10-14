#!/usr/bin/env python3
"""
convert-sweezy.py -- convert SweezyCursors .ani/.cur assets into Xcursor themes.

WHY THIS EXISTS
---------------
SweezyCursors ships Windows cursor files:

  *.ani  RIFF/ACON container holding an animated cursor (multiple frames)
  *.cur  classic Windows cursor, often a multi-image .cur with PNG payloads

Neither format is usable by X11/Wayland. Every toolkit, terminal and browser
reads Xcursor (.xcursor), a completely different container. Shipping the
Windows files directly would leave KorrinOS with no cursor at all under X.

So this tool does the real work:

  .ani  parse RIFF/ACON -> anih header (frames, steps, JifRate, flags)
       -> LIST 'fram' -> per-icon chunks
       -> each icon is a .cur -> extract every image + its hotspot
  .cur  parse ICONDIR / ICONDIRENTRY -> extract every image + hotspot

then emits, per frame and size, a PNG, and writes an xcursorgen .config
describing the animation.

TWO THINGS THAT ACTUALLY BITE WHEN PARSING THESE FILES
------------------------------------------------------
1. The per-icon rate DWORD is a trap.
   With afIcon set, a well-formed ANI 'icon' chunk starts with a DWORD holding
   a per-frame rate in jiffies. SweezyCursors' .ani sets afIcon but writes no
   rate DWORD at all -- the chunk begins directly with the CUR header, so the
   first four bytes (00 00 02 00) read as "rate = 0x00020000" if you trust the
   flag blindly. This tool therefore *probes* both candidate offsets and only
   accepts a rate that is actually a plausible jiffies value; otherwise it
   falls back to the anih JifRate.

2. Delay units.
   ANI jiffies are 1/60 s. Xcursor delays are milliseconds. JifRate=6 means
   100 ms per frame. The conversion is done once, here, and never again.
"""

import argparse
import os
import struct
import sys

# xcursor.py lives beside this file; the build runs us from any cwd.
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

# Xcursor standard sizes. We always emit this ladder; sizes absent from the
# source are derived by resampling the nearest larger native image so we never
# have to invent detail at small sizes.
STANDARD_SIZES = [16, 20, 24, 28, 32, 40, 48, 64, 96, 128, 192, 256]

# An ANI rate is expressed in jiffies (1/60 s). Anything outside this window is
# not a rate -- it is a misaligned read of a CUR header.
JIF_FREQ = 60.0
MAX_PLAUSIBLE_JIFFIES = 10000  # ~166 s per frame; anything more is not a rate


def rd(b, off, fmt):
    return struct.unpack_from(fmt, b, off)


# --------------------------------------------------------------------------
# CUR / ICO parsing
# --------------------------------------------------------------------------

def parse_icon_dir(data, base=0):
    """Parse an ICO/CUR blob. Returns (typ, entries).

    Each entry dict carries width/height/hotspot/offset/size. For CUR the
    hotspot lives in the fields ICO uses for planes/bpp.
    """
    if len(data) < 6:
        raise ValueError("icon directory truncated")
    reserved, typ, count = rd(data, base, '<HHH')
    if reserved != 0 or count == 0:
        raise ValueError("not an icon directory (reserved=%d count=%d)"
                         % (reserved, count))
    if len(data) < base + 6 + count * 16:
        raise ValueError("icon directory claims %d entries but blob is short"
                         % count)
    entries = []
    for i in range(count):
        o = base + 6 + i * 16
        w, h, ncol, rsvd, planes, bpp, size, offset = rd(data, o, '<BBBBHHII')
        # A byte width of 0 means 256.
        if w == 0:
            w = 256
        if h == 0:
            h = 256
        if typ == 2:  # CUR: hotspot is absolute, in the planes/bpp fields
            hotx, hoty = planes, bpp
        else:  # ICO: no hotspot
            hotx = hoty = None
        abs_off = base + offset
        if offset > len(data) or abs_off + size > len(data) + 1:
            raise ValueError("entry %d points outside the blob "
                             "(offset=%d size=%d len=%d)"
                             % (i, offset, size, len(data)))
        entries.append(dict(w=w, h=h, hotx=hotx, hoty=hoty,
                            size=size, offset=abs_off))
    return typ, entries


def dib_to_png(blob, width, height, bpp, hdr_size=40):
    """Decode a DIB (BMP without file header) to an RGBA PIL image.

    Handles the palette DIBs (1/4/8 bpp) and the 16/24/32 bpp truecolour ones,
    both of which SweezyCursors files use.
    """
    from PIL import Image

    if hdr_size < 40:
        raise ValueError("unsupported DIB header size %d" % hdr_size)

    # DIB rows are stored bottom-up. Walk them once, reversing into top-down.
    def rows_top_down(bytewidth, src_start, row_padded):
        out = bytearray()
        for y in range(height - 1, -1, -1):
            src = src_start + y * row_padded
            out += blob[src:src + bytewidth]
        return bytes(out)

    if bpp <= 8:
        # Colour table immediately follows the fixed header. Its length is at
        # offset 32, and it is only present when hdr_size == 40.
        ncol = rd(blob, 32, '<I')[0] if len(blob) >= 36 else 0
        if ncol == 0:
            ncol = 1 << bpp
        pal_off = 14 + hdr_size
        pal = blob[pal_off:pal_off + ncol * 4]
        if len(pal) < ncol * 4:
            raise ValueError("DIB palette truncated")
        row_padded = ((width + 31) // 32) * 4
        idx = rows_top_down(min(width, row_padded), hdr_size, row_padded)

        # 4bpp packs two pixels per byte; expand to one index per pixel.
        if bpp == 4:
            expanded = bytearray()
            for b in idx[:width * height]:
                expanded.append(b >> 4)
                expanded.append(b & 0x0F)
            idx = bytes(expanded)
        elif bpp == 1:
            expanded = bytearray()
            for byte in idx[:(width + 7) // 8 * height]:
                for bit in range(7, -1, -1):
                    expanded.append((byte >> bit) & 1)
            idx = bytes(expanded)

        # Palette entries are BGRA on disk; PIL wants RGB.
        flat = bytearray()
        for e in range(ncol):
            b, g, r = pal[e * 4], pal[e * 4 + 1], pal[e * 4 + 2]
            flat += bytes((r, g, b))
        im = Image.frombytes('P', (width, height), idx[:width * height])
        im.putpalette(bytes(flat) + b'\0' * (768 - len(flat)))
        return im.convert('RGBA')

    if bpp in (16, 24, 32):
        row_padded = ((width * bpp + 31) // 32) * 4
        raw = rows_top_down(width * (bpp // 8), hdr_size, row_padded)
        if bpp == 16:
            # X1R5G5B5, little endian. Scale each channel to 8 bits.
            px = bytearray()
            for i in range(width * height):
                v = raw[i * 2] | (raw[i * 2 + 1] << 8)
                px += bytes((((v >> 10) & 0x1F) * 255 // 31,
                             ((v >> 5) & 0x1F) * 255 // 31,
                             (v & 0x1F) * 255 // 31))
            return Image.frombytes('RGB', (width, height), bytes(px)
                                   ).convert('RGBA')
        mode = 'RGBA' if bpp == 32 else 'RGB'
        im = Image.frombytes(mode, (width, height), raw[:width * height * (bpp // 8)])
        return im.convert('RGBA')

    raise ValueError("unsupported bpp %d" % bpp)


def extract_image(data, entry):
    """Return (PIL RGBA image, hotx, hoty) for one icon-directory entry."""
    from PIL import Image
    import io

    blob = data[entry['offset']:entry['offset'] + entry['size']]
    if blob[:8] == b'\x89PNG\r\n\x1a\n':
        im = Image.open(io.BytesIO(blob))
        im.load()
    else:
        hdr_size = struct.unpack_from('<I', blob, 0)[0]
        width = struct.unpack_from('<i', blob, 4)[0]
        height = struct.unpack_from('<i', blob, 8)[0]
        height = abs(height) // 2  # XOR bitmap + AND mask are stacked
        bpp = struct.unpack_from('<H', blob, 14)[0]
        im = dib_to_png(blob, width, height, bpp, hdr_size)

    im = im.convert('RGBA')
    # The declared entry size and the decoded image can disagree; trust the
    # decoded pixels, but keep the cursor square as Xcursor expects.
    w, h = im.size
    side = max(w, h)
    if (w, h) != (side, side):
        canvas = Image.new('RGBA', (side, side), (0, 0, 0, 0))
        canvas.paste(im, (0, 0))
        im = canvas

    hotx = entry['hotx'] if entry['hotx'] is not None else side // 2
    hoty = entry['hoty'] if entry['hoty'] is not None else side // 2
    hotx = max(0, min(hotx, side - 1))
    hoty = max(0, min(hoty, side - 1))
    return im, hotx, hoty


# --------------------------------------------------------------------------
# ANI parsing
# --------------------------------------------------------------------------

def parse_ani(path):
    """Parse a RIFF/ACON animated cursor.

    Returns (frames, info) where frames is a list of frame dicts, each
    {images: {size: (PIL image, hotx, hoty)}, delay_ms: int} and info carries
    the raw anih fields for reporting.
    """
    d = open(path, 'rb').read()
    if d[0:4] != b'RIFF' or d[8:12] != b'ACON':
        raise ValueError("%s is not an ANI (RIFF/ACON) file" % path)

    off = 12
    anih = None
    fram_start = fram_end = None
    title = None
    while off + 8 <= len(d):
        cid = d[off:off + 4]
        csz = struct.unpack_from('<I', d, off + 4)[0]
        body = off + 8
        if cid == b'anih':
            anih = struct.unpack_from('<9I', d, body)
        elif cid == b'LIST':
            listtype = d[body:body + 4]
            if listtype == b'fram':
                fram_start, fram_end = body + 4, body + csz
            elif listtype == b'INFO':
                q = body + 4
                iend = body + csz
                while q + 8 <= iend:
                    sub = d[q:q + 4]
                    ssz = struct.unpack_from('<I', d, q + 4)[0]
                    if sub == b'INAM':
                        title = d[q + 8:q + 8 + ssz].split(b'\0')[0].decode(
                            'utf-8', 'replace')
                    q += 8 + ssz + (ssz & 1)
        off = body + csz + (csz & 1)

    if anih is None:
        raise ValueError("ANI has no anih header")

    (cbsize, cframes, csteps, cx, cy, cbit, cplanes, jifrate, flags) = anih
    af_icon = bool(flags & 1)
    if jifrate == 0:
        jifrate = 6  # spec default: 6 jiffies

    chunks = []
    p = fram_start
    while p is not None and p + 8 <= fram_end:
        cid = d[p:p + 4]
        csz = struct.unpack_from('<I', d, p + 4)[0]
        if cid == b'icon':
            chunks.append(d[p + 8:p + 8 + csz])
        p += 8 + csz + (csz & 1)

    if not chunks:
        raise ValueError("ANI 'fram' list contains no 'icon' chunks")

    frames = []
    for body in chunks:
        # Probe for the cursor payload. A well-formed afIcon chunk starts with
        # a rate DWORD; these files do not, so try offset 0 first and only
        # treat a leading DWORD as a rate if it is plausible jiffies.
        payload, rate, skip = None, None, None
        for candidate_skip in (0, 4):
            if af_icon and candidate_skip == 4:
                r = struct.unpack_from('<I', body, 0)[0] if len(body) >= 4 else 0
                if not (0 < r <= MAX_PLAUSIBLE_JIFFIES):
                    continue  # not a rate; try offset 0
            try:
                _typ, _entries = parse_icon_dir(body, candidate_skip)
            except Exception:
                continue
            payload = body[candidate_skip:]
            skip = candidate_skip
            if af_icon and candidate_skip == 4:
                rate = struct.unpack_from('<I', body, 0)[0]
            break

        if payload is None:
            raise ValueError("ANI icon chunk is not a readable CUR "
                             "(neither offset 0 nor 4 parses)")

        if rate is None:
            rate = jifrate
        delay_ms = int(round((rate / JIF_FREQ) * 1000.0))
        # Xcursor treats a non-positive delay as "never advance"; clamp so a
        # zero-rate source still animates instead of freezing on frame 1.
        delay_ms = max(delay_ms, 20)

        typ, entries = parse_icon_dir(payload, 0)
        images = {}
        for ent in entries:
            im, hx, hy = extract_image(payload, ent)
            images[im.size[0]] = (im, hx, hy)
        frames.append(dict(images=images, delay_ms=delay_ms, rate=rate,
                           payload_skip=skip))

    info = dict(frames_hdr=cframes, steps=csteps, jifrate=jifrate,
                af_icon=af_icon, title=title, size_hdr=(cx, cy))
    return frames, info


def parse_cur(path):
    """Parse a Windows .cur. Returns (frames, info); one frame, delay 0."""
    d = open(path, 'rb').read()
    typ, entries = parse_icon_dir(d, 0)
    images = {}
    for ent in entries:
        im, hx, hy = extract_image(d, ent)
        images[im.size[0]] = (im, hx, hy)
    info = dict(count=len(entries), animated=False)
    return [dict(images=images, delay_ms=0, rate=0, payload_skip=0)], info


# --------------------------------------------------------------------------
# Size ladder
# --------------------------------------------------------------------------

def build_size_ladder(frames, sizes=None, down_filter=None, up_filter=None):
    """Produce {size: [ (PIL image, hotx, hoty) per frame ]} for `sizes`.

    Sizes present natively in the source are used verbatim -- that is what
    keeps small sizes honest. Missing sizes are derived from the nearest
    larger native image, so we never scale *up* to reach a small size (which
    is how art gets destroyed).

    Hotspots are rescaled with the image, which the source format does not do
    for us and which silently breaks click accuracy if forgotten.
    """
    from PIL import Image

    sizes = sizes or STANDARD_SIZES
    native = sorted({s for f in frames for s in f['images']})
    if not native:
        raise ValueError("no native sizes in source")

    ladder = {}
    for target in sizes:
        if target in native:
            ladder[target] = [f['images'][target] for f in frames]
            continue
        # Derive from the nearest *larger* native size.
        larger = [s for s in native if s >= target]
        src = min(larger) if larger else max(native)
        if src == target:
            ladder[target] = [f['images'][target] for f in frames]
            continue
        filt = (down_filter if src > target else (up_filter or down_filter))
        ladder[target] = [_resize(f['images'][src], target, filt)
                          for f in frames]
    return ladder


def _resize(entry, size, filt):
    from PIL import Image
    im, hx, hy = entry
    scale = size / im.size[0]
    out = im.resize((size, size), filt)
    # Resample the hotspot in step with the pixels.
    nhx = int(round(hx * scale))
    nhy = int(round(hy * scale))
    return (out, max(0, min(nhx, size - 1)), max(0, min(nhy, size - 1)))


# --------------------------------------------------------------------------
# Emission
# --------------------------------------------------------------------------

def emit_theme(frames, name, outdir, sizes=None,
               down_filter=None, up_filter=None,
               keep_pngs=True, emit_config=False):
    """Write a real .xcursor for one cursor role, plus optional side products.

    The .xcursor is written by xcursor.py directly -- NOT by shelling out to
    xcursorgen, which is broken under libpng 1.6.x (see xcursor.py).

    keep_pngs   keep the intermediate PNGs on disk (needed when several roles
                share one animation, since each role needs its own images)
    emit_config also write an xcursorgen .config, for reference or for hosts
                where xcursorgen does work

    Returns a stats dict.
    """
    from PIL import Image
    import xcursor as xc

    down_filter = down_filter or Image.LANCZOS
    up_filter = up_filter or Image.LANCZOS

    ladder = build_size_ladder(frames, sizes, down_filter, up_filter)
    animated = len(frames) > 1

    os.makedirs(outdir, exist_ok=True)

    # Ordering matters and is load-bearing: Xcursor advances through the images
    # sharing a nominal size, in file order. So we walk sizes ascending and,
    # within a size, frames in their original order.
    images = []
    lines = []
    npng = 0
    for size in sorted(ladder):
        for fi, (im, hx, hy) in enumerate(ladder[size]):
            delay = frames[fi]['delay_ms'] if animated else 0
            images.append(dict(image=im, hotx=hx, hoty=hy,
                               delay=delay, nominal=size))
            if keep_pngs:
                png = os.path.join(outdir, "%s_%02d_%dx%d.png"
                                         % (name, fi, size, size))
                im.save(png, 'PNG')
                npng += 1
                lines.append("%d %d %d %s %s %s"
                             % (size, size, delay, hx, hy, png))

    out = os.path.join(outdir, name)
    written = xc.write(out, images)

    cfg = None
    if emit_config:
        cfg = os.path.join(outdir, name + ".config")
        with open(cfg, 'w') as f:
            f.write("\n".join(lines) + "\n")

    return dict(name=name, xcursor=out, config=cfg, pngs=npng,
                animated=animated, frames=len(frames),
                sizes=sorted(ladder), xcursor_images=written['images'],
                xcursor_bytes=written['bytes'],
                delay_ms=frames[0]['delay_ms'] if animated else None)


def main():
    ap = argparse.ArgumentParser(
        description="Convert Windows .ani/.cur cursors into Xcursor themes.")
    ap.add_argument('inputs', nargs='+')
    ap.add_argument('-o', '--outdir', required=True,
                    help="directory to receive PNGs and xcursorgen configs")
    ap.add_argument('-n', '--name', required=True,
                    help="xcursor base name (role, e.g. default or pointer)")
    ap.add_argument('--sizes', default=None,
                    help="comma-separated size ladder "
                         "(default: %s)" % ",".join(map(str, STANDARD_SIZES)))
    ap.add_argument('--nearest-upscale', action='store_true',
                    help="use NEAREST instead of LANCZOS when scaling up")
    ap.add_argument('--no-pngs', action='store_true',
                    help="do not keep the intermediate PNGs")
    ap.add_argument('--xcursorgen-config', action='store_true',
                    help="also emit an xcursorgen .config (reference only; "
                         "xcursorgen is broken under libpng 1.6.x)")
    args = ap.parse_args()

    from PIL import Image

    sizes = ([int(s) for s in args.sizes.split(',')] if args.sizes
             else list(STANDARD_SIZES))
    up = Image.NEAREST if args.nearest_upscale else Image.LANCZOS

    os.makedirs(args.outdir, exist_ok=True)
    rc = 0
    for path in args.inputs:
        base = os.path.basename(path)
        try:
            if path.lower().endswith('.ani'):
                frames, info = parse_ani(path)
                kind = "ANI"
            elif path.lower().endswith('.cur'):
                frames, info = parse_cur(path)
                kind = "CUR"
            else:
                print("  skip (unsupported container): %s" % base,
                      file=sys.stderr)
                continue

            stats = emit_theme(frames, args.name, args.outdir, sizes,
                               down_filter=Image.LANCZOS, up_filter=up,
                               keep_pngs=not args.no_pngs,
                               emit_config=args.xcursorgen_config)

            if kind == "ANI":
                print("  %s: ANI, %d frames, %d jiffies/step = %d ms/frame, "
                      "native sizes %s"
                      % (base, len(frames), frames[0]['rate'],
                         frames[0]['delay_ms'],
                         sorted({s for f in frames for s in f['images']})))
                if info.get('title'):
                    print("       title: %s" % info['title'])
            else:
                print("  %s: CUR, %d images, native sizes %s"
                      % (base, info['count'], stats['sizes']))
            print("       -> %s (%d Xcursor images, sizes %s, %d bytes)"
                  % (stats['xcursor'], stats['xcursor_images'],
                     stats['sizes'], stats['xcursor_bytes']))
        except Exception as e:
            print("  FAILED %s: %s" % (base, e), file=sys.stderr)
            rc = 1
    return rc


if __name__ == '__main__':
    sys.exit(main())