#!/bin/bash
# capture-desktop.sh - prove the ISO actually reaches a graphical desktop.
#
# A booting kernel and a reachable login shell are not a desktop. This boots the
# real ISO under QEMU with a virtual VGA framebuffer, lets LightDM autologin and
# XFCE come up, then pulls the framebuffer out through the QEMU monitor's
# `screendump` and converts it to PNG so a human can actually look at it.
#
# Textual proof of boot is easy to fool yourself with; a screenshot is not.
# This is what caught the image sitting at a text login prompt despite LightDM
# and XFCE both being installed.
#
# Usage: tools/capture-desktop.sh [seconds-before-first-shot] [gap-seconds]
# Requires: qemu-system-x86_64, and the built ISO at ../KorrinOS-v2.0.iso.
# Chain: wait for the GUI kernel -> restage it into the ISO -> boot -> capture
# the framebuffer twice, so we can see whether a desktop actually appears.
export PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ART=/home/tinkerspace/.korrinos-artifacts
KDIR=/home/tinkerspace/linux-kernel

# 1. wait for the kernel build
for i in $(seq 1 120); do
  grep -q KERNEL-OK "$ART/kbuild-gui.log" 2>/dev/null && break
  sleep 10
done
[ -f "$ART/bzImage-GUI" ] || { echo "FAIL: kernel never built"; exit 1; }
echo "STEP1 kernel ready"

# 2. restage the new kernel into the image and rebuild the ISO
cd "$KDIR" || exit 1
bash os/build-distro.sh finalize > "$ART/isobuild-gui.log" 2>&1
grep -q "BUILT:" "$ART/isobuild-gui.log" && echo "STEP2 ISO rebuilt with GUI kernel" || {
  echo "FAIL: ISO rebuild"; tail -5 "$ART/isobuild-gui.log"; exit 1; }

# 3. boot and capture the framebuffer
cd "$ART/smp" || exit 1
rm -f gui3.sock gui3.log shot1.ppm shot2.ppm

qemu-system-x86_64 -m 8192 -smp 4 \
  -kernel "$ART/bzImage-GUI" \
  -initrd /home/tinkerspace/build-korrinos/image/casper/initrd \
  -append "console=tty0 console=ttyS0,115200 panic=-1" \
  -display none -vga std \
  -serial file:./gui3.log \
  -monitor unix:./gui3.sock,server,nowait \
  -no-reboot -cdrom "$KDIR/KorrinOS-v2.0.iso" &
QPID=$!

for i in $(seq 1 60); do [ -S ./gui3.sock ] && break; sleep 2; done

snap() {
python3 - "$1" <<'PY'
import socket, time, sys
s=socket.socket(socket.AF_UNIX); s.settimeout(15); s.connect('./gui3.sock')
def send(c,w=6):
    s.sendall((c+'\n').encode()); time.sleep(w)
    buf=b''
    try:
        while True:
            d=s.recv(65536)
            if not d: break
            buf+=d
    except socket.timeout: pass
send('')
send('screendump %s' % sys.argv[1], 10)
PY
}

sleep "${1:-500}"
snap "$ART/smp/shot1.ppm"
sleep "${2:-260}"
snap "$ART/smp/shot2.ppm"

kill $QPID 2>/dev/null; wait 2>/dev/null
for f in shot1 shot2; do
  [ -f "$ART/smp/$f.ppm" ] && python3 "$ART/ppm2png.py" "$ART/smp/$f.ppm" "$ART/smp/$f.png"
done
echo "STEP3 captures done"
echo "--- lightdm/x errors in serial ---"
grep -aiE "lightdm|Xorg|drm|GPU|failed to start" gui3.log | tail -12
echo "GUI3-DONE"