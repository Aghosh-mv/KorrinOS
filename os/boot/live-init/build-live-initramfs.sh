#!/usr/bin/env bash
# build-live-initramfs.sh — generate an initramfs that matches a freshly built
# KorrinOS kernel, for the live ISO.
#
# WHY THIS EXISTS
#
# The live ISO is a casper image: an ISO9660 medium whose real rootfs lives in
# casper/filesystem.squashfs. The boot chain therefore needs three things from
# the kernel before it can hand off to the distribution userspace:
#
#   1. ISO9660   to read the medium itself
#   2. BLK_DEV_LOOP + SQUASHFS to mount the squashfs root image
#   3. DEVTMPFS + procfs + sysfs so the real init can run
#
# All of those are configured built-in (see tools/korrinos-kernel-config.sh),
# which means the handoff needs NO modules at all.
#
# The obvious alternative — reusing an initramfs-tools image generated for a
# distribution kernel — does not work. Every modprobe inside it fails the
# vermagic check against a differently-versioned kernel, udev then never
# populates /dev, and the casper root-device scan spins forever with no error.
# That failure looks like "the ISO just hangs", which is exactly what it does.
#
# This initramfs is ~1 MB, has no kernel dependency at all, and boots the same
# rootfs with any kernel that has the options above built in.

set -euo pipefail

OUT="${1:-}"
BUSYBOX="${BUSYBOX:-$(command -v busybox || true)}"
KERNEL_SRC="${KERNEL_SRC:-/home/tinkerspace/linux-kernel}"

if [ -z "$OUT" ]; then
  OUT="${KERNEL_SRC}/os/boot/live-init/initrd.img"
fi

if [ -z "$BUSYBOX" ] || [ ! -x "$BUSYBOX" ]; then
  echo "error: busybox not found. Install it (apt install busybox-static) or set BUSYBOX=/path/to/busybox" >&2
  exit 1
fi

# A dynamic busybox would need its loader and libc copied in; require static so
# the initramfs stays self-contained.
if ldd "$BUSYBOX" >/dev/null 2>&1; then
  echo "error: $BUSYBOX is dynamically linked; a static busybox is required" >&2
  exit 1
fi

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INIT_SRC="$HERE/init"

[ -f "$INIT_SRC" ] || { echo "error: missing $INIT_SRC" >&2; exit 1; }

WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

mkdir -p "$WORK/root"/{bin,dev,proc,sys,cd,root}
cp "$BUSYBOX" "$WORK/root/bin/busybox"
install -m 0755 "$INIT_SRC" "$WORK/root/init"

mkdir -p "$(dirname "$OUT")"
( cd "$WORK/root" && find . | cpio -o -H newc --quiet ) | gzip -1 > "$OUT"

echo "live initramfs: $OUT ($(stat -c %s "$OUT") bytes)"
echo "  busybox      : $BUSYBOX"
echo "  init         : $INIT_SRC"
echo
echo "This initramfs requires the kernel to have, built in:"
echo "  CONFIG_ISO9660_FS=y  CONFIG_SQUASHFS=y  CONFIG_BLK_DEV_LOOP=y"
echo "  CONFIG_DEVTMPFS=y    CONFIG_PROC_FS=y  CONFIG_SYSFS=y  CONFIG_EXT4_FS=y"
echo "Run tools/korrinos-kernel-config.sh to enforce them."