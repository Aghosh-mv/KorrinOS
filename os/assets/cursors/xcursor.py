#!/usr/bin/env python3
"""
xcursor.py -- read and write Xcursor (.xcursor) files directly.

WHY NOT JUST SHELL OUT TO xcursorgen
------------------------------------
xcursorgen 1.0.7 (the version in the build image) is broken against libpng
1.6.x: it fails on *every* input, PNG or BMP, with

    xcursorgen: PNG error while reading <hotspot-x>

regardless of the image's format, size or validity. Verified on this build
host against a solid ImageMagick-generated PNG. Since KorrinOS has to be able
to build these themes on the build machine, we cannot depend on it.

THE ACTUAL ON-DISK LAYOUT
-------------------------
The obvious reading of Xcursor.h -- file header {magic, version, nimages,
nominal-size}, then per chunk {type, subtype, position, length} and a 28-byte
image header -- is what several implementations assume, and libXcursor
REJECTS every file written that way. The layout libXcursor 1.0.2 actually
accepts was determined empirically against the reference theme shipped by
GNOME (/usr/share/icons/Adwaita/cursors/default) and confirmed by loading
candidate files back through the real library:

    file header, 16 bytes
        0x00  magic   "Xcur"      -- four ASCII bytes, NOT 0xfffd0002
        0x04  header  16          -- size of this header
        0x08  version 0x00010000
        0x0c  nimages

    chunk directory, 12 bytes per image
        type      0xfffd0002
        subtype   nominal size
        position  byte offset of that image's body

    image body, at `position`
        length    36              -- size of the header block that follows
        type      0xfffd0002      -- repeated
        subtype   nominal size    -- repeated
        version   1
        width
        height
        xhot
        yhot
        delay                    -- ms until the next frame
        pixels    width*height ARGB32 words, row major

Three details are load-bearing and each one alone makes libXcursor reject the
whole file:

  * the magic is the ASCII string "Xcur"; the 0xfffd0002 constant that
    Xcursor.h advertises is only valid for the *chunk* type;
  * the image body repeats [length][type][subtype] before the image header;
  * the header block is 36 bytes, so every field including the hotspot is a
    32-bit word. XcursorDim is `unsigned int`, not `unsigned short`.

Animation is ordering: Xcursor advances through every image that shares a
nominal size, in file order, so this module groups by nominal size and keeps
the caller's frame order within each group.
"""

import hashlib
import os
import struct

# The ASCII magic is the only file magic libXcursor 1.0.2 accepts.
XCURSOR_MAGIC = b'Xcur'
XCURSOR_HEADER_SIZE = 16
XCURSOR_VERSION = 0x00010000
XCURSOR_IMAGE_VERSION = 1
XCURSOR_TYPE_IMAGE = 0xfffd0002

# Total size of the image body header block: length, type, subtype, version,
# width, height, xhot, yhot, delay.
XCURSOR_IMAGE_HEADER_LEN = 36
XCURSOR_CHUNK_DIR_ENTRY = 12

# Guard rails. These are what stop a corrupt or mis-sized input from producing
# a multi-gigabyte allocation or an out-of-bounds read in read().
MAX_IMAGES = 4096
MAX_DIMENSION = 1024


def _pixels(im):
    """PIL image -> list of 32-bit ARGB words.

    Xcursor stores pixels as CARD32 ARGB with no premultiplication, which is
    what xcursorgen writes and what every theme in the wild contains.
    """
    out = []
    for (r, g, b, a) in im.convert('RGBA').getdata():
        out.append(((a & 0xFF) << 24) | ((r & 0xFF) << 16) |
                   ((g & 0xFF) << 8) | (b & 0xFF))
    return out


def write(path, images):
    """Write an .xcursor file. Returns a stats dict.

    images : iterable of dicts with keys
        image   -- PIL image (RGBA, or convertible)
        hotx, hoty -- hotspot in pixels, clamped into the image
        delay   -- ms until the next frame; 0 for a static cursor
        nominal -- nominal size for matching; defaults to the image width

    Images are grouped by nominal size (ascending) and, within a group, kept
    in the order given. That ordering IS the animation.
    """
    groups = {}
    for idx, ent in enumerate(images):
        nom = int(ent.get('nominal') or ent['image'].size[0])
        if nom not in groups:
            groups[nom] = []
        groups[nom].append((idx, ent))
    order = sorted(groups)

    total = sum(len(groups[k]) for k in order)
    if total > MAX_IMAGES:
        raise ValueError("refusing to write %d images (max %d)"
                         % (total, MAX_IMAGES))

    # Build bodies first so the directory can carry their real offsets.
    bodies = []
    for nom in order:
        for _idx, ent in sorted(groups[nom], key=lambda t: t[0]):
            im = ent['image']
            w, h = im.size
            if w != h:
                raise ValueError("Xcursor images must be square, got %dx%d"
                                 % (w, h))
            if not (0 < w <= MAX_DIMENSION):
                raise ValueError("implausible Xcursor dimension %d" % w)
            hotx = max(0, min(int(ent.get('hotx', 0)), w - 1))
            hoty = max(0, min(int(ent.get('hoty', 0)), h - 1))
            delay = max(0, int(ent.get('delay', 0)))

            body = bytearray()
            body += struct.pack('<IIIIIIIII', XCURSOR_IMAGE_HEADER_LEN,
                                XCURSOR_TYPE_IMAGE, nom, XCURSOR_IMAGE_VERSION,
                                w, h, hotx, hoty, delay)
            body += struct.pack('<%dI' % (w * h), *_pixels(im))
            bodies.append(bytes(body))

    blob = bytearray()
    blob += XCURSOR_MAGIC
    blob += struct.pack('<III', XCURSOR_HEADER_SIZE, XCURSOR_VERSION, total)

    dir_size = XCURSOR_CHUNK_DIR_ENTRY * total
    off = len(blob) + dir_size
    positions = []
    for b in bodies:
        positions.append(off)
        off += len(b)

    for b, pos in zip(bodies, positions):
        subtype = struct.unpack_from('<I', b, 8)[0]
        blob += struct.pack('<III', XCURSOR_TYPE_IMAGE, subtype, pos)
    for b in bodies:
        blob += b

    with open(path, 'wb') as f:
        f.write(blob)
    return dict(path=path, images=total, sizes=order, bytes=len(blob))


def write_group(paths, images):
    """Write one .xcursor and symlink the rest of `paths` onto it.

    A cursor theme asks X11 for ~59 distinct cursor names, but the artwork
    behind them is far less varied: every resize cursor is the same picture,
    and in the artist themes 57 of 59 roles are one image. Xcursor stores
    pixels as uncompressed 32-bit words, so writing all 59 out separately
    costs hundreds of megabytes for what is really two or three images.

    Roles whose image sets are byte-identical therefore share one file. The
    links are real symlinks, so Xcursor's plain open() resolves them.

    The REAL file is named after the most significant role in the group --
    `default` first, then `left_ptr`, then whatever comes first -- so that a
    theme's `cursors/default` is a genuine file and not a link to something
    obscure. Some tools stat or copy a theme's `default` directly, and a
    theme whose `default` is a symlink to `alias` is confusing to debug even
    though it resolves.

    Returns (stats, symlinked_paths).
    """
    ranks = ('default', 'left_ptr', 'pointer', 'arrow')
    order = sorted(paths, key=lambda p: (
        ranks.index(os.path.basename(p)) if os.path.basename(p) in ranks
        else len(ranks),
        p))
    target, rest = order[0], order[1:]

    stats = write(target, images)
    linked = []
    for p in rest:
        if os.path.islink(p) or os.path.exists(p):
            os.unlink(p)
        os.symlink(os.path.basename(target), p)
        linked.append(p)
    return stats, linked


def content_key(images):
    """Stable key identifying a set of images, for grouping identical roles.

    Hashes the declared geometry and the raw pixels, so two roles collide only
    when they are genuinely the same picture.
    """
    h = hashlib.sha256()
    for ent in images:
        h.update(b'%d,%d,%d,%d,%d;' % (int(ent.get('nominal') or 0),
                                      int(ent.get('hotx', 0)),
                                      int(ent.get('hoty', 0)),
                                      int(ent.get('delay', 0)),
                                      ent['image'].size[0]))
        px = _pixels(ent['image'])
        for v in px:
            h.update(struct.pack('<I', v))
    return h.hexdigest()


def read(path):
    """Parse an .xcursor file back into a dict, for verification.

    Raises ValueError on anything structurally wrong, including a body shorter
    than its declared length -- the failure mode a hand-rolled writer is most
    likely to have.
    """
    d = open(path, 'rb').read()
    if d[:4] != XCURSOR_MAGIC:
        raise ValueError("bad magic %r (want %r)" % (d[:4], XCURSOR_MAGIC))
    hsize, ver, nimages = struct.unpack_from('<III', d, 4)
    if hsize != XCURSOR_HEADER_SIZE:
        raise ValueError("header size %d, expected %d"
                         % (hsize, XCURSOR_HEADER_SIZE))
    if ver != XCURSOR_VERSION:
        raise ValueError("version %#x, expected %#x" % (ver, XCURSOR_VERSION))
    if not (0 < nimages <= MAX_IMAGES):
        raise ValueError("implausible image count %d" % nimages)
    if hsize + XCURSOR_CHUNK_DIR_ENTRY * nimages > len(d):
        raise ValueError("directory for %d images runs past end of file"
                         % nimages)

    out = []
    for i in range(nimages):
        o = hsize + i * XCURSOR_CHUNK_DIR_ENTRY
        ctype, sub, pos = struct.unpack_from('<3I', d, o)
        if ctype != XCURSOR_TYPE_IMAGE:
            raise ValueError("directory entry %d is not an image chunk (%#x)"
                             % (i, ctype))
        if pos + XCURSOR_IMAGE_HEADER_LEN > len(d):
            raise ValueError("entry %d body runs past end of file" % i)
        (length, btype, bsub, bver, w, h, hotx, hoty, delay) = \
            struct.unpack_from('<9I', d, pos)
        if length != XCURSOR_IMAGE_HEADER_LEN:
            raise ValueError("entry %d body header is %d bytes, expected %d"
                             % (i, length, XCURSOR_IMAGE_HEADER_LEN))
        if btype != XCURSOR_TYPE_IMAGE:
            raise ValueError("entry %d body type %#x" % (i, btype))
        if bsub != sub or bver != XCURSOR_IMAGE_VERSION:
            raise ValueError("entry %d body header disagrees with directory" % i)
        if sub != w:
            raise ValueError("entry %d nominal size %d != width %d"
                             % (i, sub, w))
        if w != h:
            raise ValueError("entry %d is %dx%d; cursors are square" % (i, w, h))
        if not (0 < w <= MAX_DIMENSION):
            raise ValueError("entry %d implausible dimension %d" % (i, w))
        if not (0 <= hotx < w and 0 <= hoty < h):
            raise ValueError("entry %d hotspot (%d,%d) outside %dx%d"
                             % (i, hotx, hoty, w, h))
        if pos + length + w * h * 4 > len(d):
            raise ValueError("entry %d pixel data runs past end of file" % i)
        out.append(dict(nominal=sub, hotx=hotx, hoty=hoty, w=w, h=h,
                        delay=delay, position=pos))
    return dict(version=ver, images=out, bytes=len(d))