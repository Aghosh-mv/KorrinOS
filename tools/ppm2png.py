#!/usr/bin/env python3
"""Convert a QEMU screendump PPM (P6) to PNG so it can actually be looked at."""
import struct
import sys
import zlib

src, dst = sys.argv[1], sys.argv[2]
with open(src, "rb") as f:
    assert f.readline().strip() == b"P6", "not a P6 PPM"
    w, h = map(int, f.readline().split())
    f.readline()  # maxval
    data = f.read()

raw = b"".join(b"\x00" + data[y * w * 3:(y + 1) * w * 3] for y in range(h))


def chunk(tag, payload):
    body = tag + payload
    return struct.pack(">I", len(payload)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)


png = (b"\x89PNG\r\n\x1a\n"
       + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
       + chunk(b"IDAT", zlib.compress(raw, 6))
       + chunk(b"IEND", b""))

with open(dst, "wb") as f:
    f.write(png)
print(f"wrote {dst} ({len(png)} bytes, {w}x{h})")