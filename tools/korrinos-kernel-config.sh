#!/usr/bin/env bash
# korrinos-kernel-config.sh — enforce the kernel options KorrinOS depends on.
#
# .config is gitignored, so the live-boot requirements live here instead of
# being rediscovered the hard way. Run this after a fresh clone, or from
# os/build-distro.sh before the kernel build.
#
# THE CRITICAL ONE
#
#   CONFIG_ISO9660_FS must be =y (built in).
#
# The live ISO is a casper image: the kernel must read its own boot medium to
# find casper/filesystem.squashfs. Left as =m, that read fails unless the
# matching module can be loaded, and the only initramfs on hand is generated
# for a different kernel -- so every modprobe fails vermagic, udev never
# populates /dev, and the boot spins in the root-device scan with no message.
# The symptom is an ISO that boots and then does nothing at all.
#
# Everything else listed here is needed for the live handoff to succeed, and all
# of it is deliberately built in so the handoff needs no modules whatsoever.

set -euo pipefail

CONFIG="${1:-.config}"

[ -f "$CONFIG" ] || { echo "error: $CONFIG not found (run from the kernel source root)" >&2; exit 1; }
command -v scripts/config >/dev/null 2>&1 || {
  echo "error: run this from the kernel source root (scripts/config not found)" >&2; exit 1; }

# --- required for the live ISO to boot at all -------------------------------
# name=value, one per line. ISO9660 is listed first deliberately.
REQUIRED=(
  "ISO9660_FS=y"        # read the boot medium (casper ISO)
  "SQUASHFS=y"          # casper/filesystem.squashfs is the real rootfs
  "BLK_DEV_LOOP=y"      # loop-mount that squashfs
  "DEVTMPFS=y"          # /dev without udev in the initramfs
  "PROC_FS=y"
  "SYSFS=y"
  "EXT4_FS=y"           # the real rootfs writes here
  "BLK_DEV_INITRD=y"    # we boot an initramfs
  "BLK_DEV_SR=y"        # the medium appears as /dev/sr0
  "CDROM=y"
  "SCSI=y"
  "ATA=y"
  "ATA_PIIX=y"          # QEMU's default IDE controller
  "VIRTIO_BLK=y"        # virtio disk, used by most cloud hypervisors
  "VIRTIO_PCI=y"
  "VIRTIO=y"
  "TMPFS=y"
  "UNIX=y"              # needed for systemd
)

# --- the KorrinOS feature set ------------------------------------------------
# Derived from kernel/tinker/Makefile rather than hardcoded, so adding a tinker
# module cannot leave this script enabling a symbol that does not exist (or,
# worse, missing one that does).
TINKER_MK="kernel/tinker/Makefile"
if [ ! -f "$TINKER_MK" ]; then
  echo "error: $TINKER_MK not found (run from the kernel source root)" >&2
  exit 1
fi
TINKER_MODULES=(
  $(grep -oE 'obj-\$\(CONFIG_[A-Z0-9_]+\)' "$TINKER_MK" \
    | sed 's/obj-\$(CONFIG_//; s/)//' | sort -u)
)

CHANGED=0
apply() {
  local opt="$1" want="$2" cur
  cur="$(grep -E "^CONFIG_${opt}=" "$CONFIG" 2>/dev/null | head -1 | cut -d= -f2- || true)"
  [ -z "$cur" ] && cur="$(grep -E "^# CONFIG_${opt} is not set" "$CONFIG" >/dev/null 2>&1 && echo n || echo '')"
  if [ "$cur" != "$want" ]; then
    echo "  $opt: ${cur:-<unset>} -> $want"
    if [ "$want" = "n" ]; then
      scripts/config --disable "$opt" >/dev/null 2>&1 || true
    else
      scripts/config --enable "$opt" >/dev/null 2>&1 || true
    fi
    CHANGED=$((CHANGED + 1))
  fi
}

echo "enforcing KorrinOS kernel requirements in $CONFIG"

echo "live-boot requirements:"
for entry in "${REQUIRED[@]}"; do
  apply "${entry%%=*}" "${entry##*=}"
done

echo "tinker feature set (all built in):"
for m in "${TINKER_MODULES[@]}"; do
  apply "$m" y
done
# The umbrella gate that pulls kernel/tinker/ into the build.
apply TINKER_FEATURES y

if [ "$CHANGED" -eq 0 ]; then
  echo "  already satisfied, nothing to change"
else
  echo "  changed $CHANGED option(s) - rebuild required:"
  echo "    make -j\$(nproc)"
fi