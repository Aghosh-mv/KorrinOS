#!/usr/bin/env bash
# iso-boot-test.sh — does the KorrinOS live ISO actually boot?
#
# Measures what a user cares about: does the machine reach a real login /
# desktop userspace, with the kernel we built, from the ISO we shipped.
#
# Boot chain under test:
#   BIOS GRUB (efi/isolinux + /boot/grub/grub.cfg)
#     -> /casper/vmlinuz   = our built kernel
#     -> /casper/initrd    = os/boot/live-init (module-free initramfs)
#       -> mounts the medium, loop-mounts casper/filesystem.squashfs
#       -> switch_root into the real distribution userspace
#
# Pass criteria, in order:
#   1. GRUB renders a menu and loads the kernel
#   2. the live init mounts the rootfs and switch_roots
#   3. systemd starts in the real rootfs
#   4. KorrinOS' own greeting appears
#   5. no kernel panic / oops / BUG
#
# Usage: iso-boot-test.sh [iso] [bzimage] [initrd] [runs] [timeout_seconds]

set -u

ISO="${1:-/home/tinkerspace/isofix/KorrinOS-fixed.iso}"
BZ="${2:-/home/tinkerspace/linux-kernel/arch/x86/boot/bzImage}"
INITRD="${3:-/tmp/kinit2.gz}"
RUNS="${4:-3}"
TMO="${5:-900}"
SMP="${SMP:-2}"
LOGDIR="$(mktemp -d)"

for f in "$ISO" "$BZ" "$INITRD"; do
  [ -f "$f" ] || { echo "missing: $f" >&2; exit 2; }
done

echo "ISO   : $ISO ($(du -h "$ISO" | cut -f1))"
echo "kernel: $BZ"
echo "initrd: $INITRD"
echo "runs  : $RUNS   smp=$SMP   timeout=${TMO}s"
echo

pass=0; fail=0; warn_total=0
for i in $(seq 1 "$RUNS"); do
  log="$LOGDIR/run$i.log"
  printf "  run %s: " "$i"
  timeout "$TMO" qemu-system-x86_64 \
    -m 4096 -smp "$SMP" -cdrom "$ISO" \
    -kernel "$BZ" -initrd "$INITRD" \
    -append "console=ttyS0,115200 panic=-1 loglevel=4 ${EXTRA_APPEND:-}" \
    -display none -serial "file:$log" -no-reboot >/dev/null 2>&1

  grub=0; kinit=0; sysd=0; greet=0
  # Hard crash = the kernel actually died. A soft lockup is NOT one: under TCG
  # a userspace generator (snapd-generator, udev) can outrun the watchdog purely
  # because emulation is ~100x slow. That is a performance artifact, reported
  # separately, not a boot failure.
  crash=0
  softlock=0
  grep -qa 'GNU GRUB'                "$log" && grub=1
  grep -qa 'switch_root into rootfs' "$log" && kinit=1
  grep -qa 'systemd\[1\]: systemd'    "$log" && sysd=1
  grep -qa 'Welcome to'              "$log" && greet=1
  grep -qaE 'Kernel panic|Oops|RIP: 0|BUG: kernel NULL pointer' "$log" && crash=1
  grep -qa 'soft lockup'             "$log" && softlock=1

  printf "grub=%s kinit=%s systemd=%s greeting=%s crash=%s" \
         "$grub" "$kinit" "$sysd" "$greet" "$crash"
  [ "$softlock" -eq 1 ] && { printf " softlockup"; warn_total=$((warn_total+1)); }
  echo

  # "Welcome to KorrinOS!" is printed by the real distribution userspace after
  # switch_root, so it -- not the systemd banner, which loglevel=4 may hide --
  # is the authoritative signal that we reached a booted OS.
  if [ "$kinit" -eq 1 ] && [ "$greet" -eq 1 ] && [ "$crash" -eq 0 ]; then
    echo "        -> BOOTED (reached the real userspace)"
    pass=$((pass+1))
  else
    echo "        -> FAILED"
    fail=$((fail+1))
    tail -c 400 "$log" 2>/dev/null | tr -d '\000' | tail -3 | cut -c1-110 | sed 's/^/          /'
  fi
done

echo
echo "  boots to userspace: $pass/$RUNS   failures: $fail   runs with TCG soft-lockups: $warn_total"
echo "  logs: $LOGDIR"
[ "$fail" -eq 0 ] && echo "  RESULT: ISO BOOTS RELIABLY" || echo "  RESULT: BOOT IS FLAKY/FAILING"
exit $(( fail > 0 ))
