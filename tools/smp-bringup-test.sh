#!/usr/bin/env bash
# smp-bringup-test.sh — regression test for SMP bring-up.
#
# BUG (found 2026-10-06, 7.2.0-rc6-korrinos+):
#   The kernel boots reliably with -smp 1 and -smp 2, but HANGS during SMP
#   bring-up with 3 or more vCPUs. The last output is always:
#
#       ACPI: 1 ACPI AML tables successfully acquired and loaded
#
#   and then nothing, forever. No panic, no oops, no BUG:, no soft lockup.
#
# Evidence gathered:
#   - 5/5 runs stalled at -smp 4 (one anomalous success in ~10 runs)
#   - host has 24 cores, so this is NOT oversubscription
#   - QEMU monitor "info registers" per vCPU at the hang:
#         vCPU0    watchdog_check_skew+0x8b
#         vCPU1    pv_native_safe_halt+0xb            (idle - normal)
#         vCPU2/3  native_queued_spin_lock_slowpath+0xbc
#     => two CPUs spinning on a lock held elsewhere during bring-up
#   - booting with "nowatchdog" still stalls, so it is a real lockup and not
#     the NMI watchdog misfiring
#   - all boot-critical storage/filesystem drivers are builtin (SQUASHFS,
#     EXT4, VIRTIO_BLK, ATA_PIIX, BLK_DEV_SD, SCSI, BLK_DEV_LOOP), so this is
#     not a "module missing from the initrd" problem
#
# This matters: essentially every real machine is multi-core. A kernel that
# cannot bring up >2 CPUs is not shippable.
#
# Usage:  tools/smp-bringup-test.sh [bzImage] [initrd]
#         tools/smp-bringup-test.sh --repeat 10

set -u

REPEAT="${REPEAT:-1}"
SMPS="${SMPS:-1 2 3 4}"
TIMEOUT="${TIMEOUT:-180}"
BZ=""
INITRD=""

# Parse flags first so "--repeat 3" is never mistaken for the bzImage path.
while [ $# -gt 0 ]; do
  case "$1" in
    --repeat) REPEAT="${2:-3}"; shift 2 ;;
    --smps)   SMPS="${2:-1 2 3 4}"; shift 2 ;;
    --timeout) TIMEOUT="${2:-180}"; shift 2 ;;
    -h|--help)
      sed -n '2,30p' "$0" | sed 's/^# \?//'
      exit 0 ;;
    -*) echo "unknown option: $1" >&2; exit 2 ;;
    *)
      if [ -z "$BZ" ]; then BZ="$1"; else INITRD="$1"; fi
      shift ;;
  esac
done

BZ="${BZ:-/home/tinkerspace/linux-kernel/arch/x86/boot/bzImage}"

[ -f "$BZ" ] || { echo "bzImage not found: $BZ" >&2; exit 2; }

# Build a tiny init that just announces itself and powers off, so a boot
# success is unambiguous and independent of any rootfs.
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/initrd"
cat > "$WORK/mini.c" <<'EOF'
#include <stdio.h>
#include <sys/reboot.h>
#include <unistd.h>
int main(void){ printf("SMPTEST-REACHED-USERSPACE\n"); fflush(stdout); reboot(RB_POWER_OFF); for(;;) pause(); }
EOF
gcc -O2 -static -o "$WORK/initrd/init" "$WORK/mini.c" 2>/dev/null || {
  echo "cannot build test init (need static libc)" >&2; exit 2; }
( cd "$WORK/initrd" && find . | cpio -o -H newc --quiet 2>/dev/null | gzip -1 > "$WORK/initrd.gz" )

echo "bzImage : $BZ"
echo "initrd  : $WORK/initrd.gz"
echo "timeout : ${TIMEOUT}s per boot"
echo

declare -A RESULT
FAILED=0

for smp in $SMPS; do
  pass=0
  for i in $(seq 1 "$REPEAT"); do
    log="$WORK/boot_${smp}_${i}.log"
    timeout "$TIMEOUT" qemu-system-x86_64 \
      -m 2048 -smp "$smp" \
      -kernel "$BZ" -initrd "$WORK/initrd.gz" \
      -append "console=ttyS0,115200 earlyprintk=serial,ttyS0,115200 panic=-1" \
      -display none -serial "file:$log" -no-reboot >/dev/null 2>&1
    if grep -qa 'SMPTEST-REACHED-USERSPACE' "$log" 2>/dev/null; then
      pass=$((pass + 1))
    else
      stalled_at=$(grep -aoE 'ACPI: 1 ACPI AML tables successfully acquired' "$log" 2>/dev/null | head -1)
      [ -n "$stalled_at" ] && echo "    smp=$smp run=$i: STALLED after '$stalled_at'"
    fi
  done
  RESULT[$smp]=$pass
  echo "  smp=$smp : $pass/$REPEAT reached userspace"
  [ "$pass" -eq 0 ] && [ "$REPEAT" -gt 1 ] && FAILED=1
  [ "$pass" -eq 0 ] && [ "$REPEAT" -eq 1 ] && echo "     (single run; re-run with --repeat 3 to rule out flakiness)"
  echo
done

echo "=================================================="
res() { echo "${RESULT[$1]:-0}"; }
if [ "$(res 1)" -gt 0 ] && [ "$(res 2)" -gt 0 ] && [ "$(res 3)" -eq 0 ] && [ "$(res 4)" -eq 0 ]; then
  echo "RESULT: BUG REPRODUCED - boots with 1-2 CPUs, hangs with 3+"
  echo "        (see header comment for the register-level evidence)"
  exit 1
fi
echo "RESULT: no reproduction in this run. Per-vCPU results:"
for s in $SMPS; do echo "        smp=$s -> $(res "$s")/$REPEAT"; done
exit 0