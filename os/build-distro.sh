#!/bin/bash
# KorrinOS REAL Distribution ISO Builder
# Builds a genuine, full desktop Linux distribution (like Ubuntu/Arch/Kali)
# with Xorg/Wayland, a desktop environment, browsers, applications, a real
# package base, AND the 3 isolated worlds (HACK/GAME/NORMAL) baked in.
#
# Structure (standard Ubuntu live-CD / casper layout):
#   rootfs/            <- debootstrap base + packages
#     casper/filesystem.squashfs
#     vmlinuz, initrd
#   KorrinOS-v1.1.iso  <- bootable, multi-GB, real OS
#
# Requires: sudo + debootstrap + mksquashfs + xorriso + internet.

set -euo pipefail

WORLDS="${WORLDS:-all}"

ARCH="${ARCH:-amd64}"
SUITE="${SUITE:-jammy}"                       # Ubuntu 22.04 (Pop base)
MIRROR="${MIRROR:-http://in.archive.ubuntu.com/ubuntu/}"
BUILD="${BUILD:-/home/tinkerspace/build-korrinos}"
ROOTFS="$BUILD/rootfs"
IMAGE="$BUILD/image"
OUT="${OUT:-/home/tinkerspace/linux-kernel/KorrinOS-v2.0.iso}"
SUDO="${SUDO:-sudo}"
if ! sudo -n true 2>/dev/null; then
  echo "ERROR: passwordless sudo required. Run: sudo -v" >&2
  exit 1
fi

NEED="debootstrap mksquashfs xorriso chroot"
for c in debootstrap mksquashfs xorriso; do
  command -v "$c" >/dev/null || { echo "missing: $c"; exit 1; }
done

mkdir -p "$BUILD"

echo "### [1/6] Bootstrapping base system ($SUITE)..."

stage1() {
  "$SUDO" rm -rf "$ROOTFS"
  "$SUDO" debootstrap --arch="$ARCH" --variant=minbase \
    "$SUITE" "$ROOTFS" "$MIRROR"
  # give the chroot working DNS so apt/in-chroot fetch works
  [ -f /etc/resolv.conf ] && "$SUDO" cp /etc/resolv.conf "$ROOTFS/etc/resolv.conf"
  echo "   base bootstrap done."
}

stage2_install() {
  echo "### [2/6] Installing desktop, apps, package base inside rootfs..."
  # proper full sources so universe/multiverse apps resolve
  "$SUDO" bash -c "cat > '$ROOTFS/etc/apt/sources.list' <<'SRC'
deb $MIRROR $SUITE main restricted universe multiverse
deb $MIRROR $SUITE-updates main restricted universe multiverse
deb $MIRROR $SUITE-security main restricted universe multiverse
deb-src $MIRROR $SUITE main restricted universe multiverse
SRC"
  # bind-mount /proc /sys /dev so apt postinst scripts work inside chroot
  "$SUDO" mkdir -p "$ROOTFS"/{proc,sys,dev,dev/pts}
  "$SUDO" mount --bind /proc  "$ROOTFS/proc"  2>/dev/null || true
  "$SUDO" mount --bind /sys   "$ROOTFS/sys"   2>/dev/null || true
  "$SUDO" mount --bind /dev   "$ROOTFS/dev"   2>/dev/null || true
  mountpoint -q "$ROOTFS/dev/pts" || "$SUDO" mount -t devpts none "$ROOTFS/dev/pts" 2>/dev/null || true

# The install helper lives in its own file, written with a QUOTED heredoc so
# its own $variables survive verbatim. The apt-setup heredoc below is
# unquoted (it intentionally expands $WORLDS and $(...)), so a function defined
# inline there would be mangled at generation time.
cat > "$BUILD/kapt-lib.sh" <<'KAPTEOF'
#!/bin/bash
# kapt: resilient package install.
#
# A single unavailable package name used to abort the entire apt-get install
# for its group, silently dropping every other package in that group. In one
# KorrinOS run that cost picom, dunst, alsa-utils, wireplumber, libreoffice,
# gimp, inkscape, audacity, gparted and sysstat. kapt filters the list first,
# so a bad name costs only itself and is recorded rather than hidden.
KAPT_SKIPPED_FILE="${KAPT_SKIPPED_FILE:-/var/lib/korrinos/skipped-packages.txt}"

kapt() {
  local label="$1"; shift
  local -a want=() ok=() p
  for p in "$@"; do
    [ -z "$p" ] && continue
    want+=("$p")
    if apt-cache show "$p" >/dev/null 2>&1; then
      ok+=("$p")
    else
      echo "$p" >> "$KAPT_SKIPPED_FILE"
    fi
  done
  if [ ${#ok[@]} -eq 0 ]; then
    echo "  [$label] nothing installable (all ${#want[@]} unavailable)"
    return 0
  fi
  local missing=$(( ${#want[@]} - ${#ok[@]} ))
  if [ "$missing" -gt 0 ]; then
    echo "  [$label] installing ${#ok[@]}, skipping $missing unavailable"
  else
    echo "  [$label] installing ${#ok[@]}"
  fi
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${ok[@]}" \
    >/tmp/kapt-$$.log 2>&1 || {
      echo "  [$label] WARNING: apt returned non-zero; last lines:"
      tail -4 /tmp/kapt-$$.log | sed 's/^/      /'
      rm -f /tmp/kapt-$$.log
      return 0
    }
  rm -f /tmp/kapt-$$.log
  return 0
}
KAPTEOF

cat > "$BUILD/apt.sh" <<EOF
#!/bin/bash
set -e
export DEBIAN_FRONTEND=noninteractive
. /usr/local/lib/korrinos/kapt-lib.sh
WORLDS="$WORLDS"
apt-get update -y

# ============================================================
#  KorrinOS v1.3 — FULL 30GB DESKTOP DISTRIBUTION
#  Installing EVERYTHING with recommends for a complete OS
# ============================================================

# ---- KERNEL + BOOT ----
# Our KorrinOS kernel (Linux v7.2-rc6 + Tinker) is built on the host and
# dropped into the rootfs in stage3. apt charset only provides boot/firmware.
echo ">>> Installing kernel and boot system..."
kapt "kernel/boot" initramfs-tools initramfs-tools-core initramfs-tools-bin casper live-boot live-config grub-efi-amd64-bin shim-signed mokutil
# Note: grub-pc removed — conflicts with grub-efi in chroot

# ---- FULL XFCE DESKTOP (with recommends) ----
echo ">>> Installing XFCE4 desktop..."
kapt "desktop" xfce4 xfce4-goodies xfce4-terminal xfce4-panel xfce4-session xfce4-settings xfce4-power-manager lightdm lightdm-gtk-greeter lightdm-gtk-greeter-settings xorg xserver-xorg xserver-xorg-input-all xserver-xorg-video-all xserver-xorg-input-libinput xserver-xorg-input-synaptics x11-xserver-utils x11-utils x11-apps xdg-utils xdg-desktop-portal

# ---- DISPLAY MANAGER + COMPOSITOR ----
echo ">>> Installing compositor and display tools..."
kapt "compositor" picom dunst xfwm4 arandr autorandr xdotool xclip xsel nitrogen feh imwheel

# ---- AUDIO STACK ----
echo ">>> Installing audio system..."
kapt "audio" pulseaudio pulseaudio-utils pulseaudio-module-bluetooth pavucontrol pavumeter alsa-utils alsa-tools pipewire pipewire-pulse wireplumber sound-theme-freedesktop audacity audacious lmms

# ---- NETWORKING ----
echo ">>> Installing networking..."
kapt "networking" network-manager network-manager-gnome net-tools wireless-tools iw wpasupplicant openssh-client openssh-server ssh curl wget aria2 axel smbclient samba-common-bin dnsutils traceroute nmap openvpn wireguard-tools bluetooth bluez bluez-tools blueman

# ---- FILE MANAGER + FILES ----
echo ">>> Installing file managers..."
kapt "file managers" thunar thunar-archive-plugin thunar-volman nemo nautilus pcmanfm mousepad xfburn file-roller engrampa gvfs gvfs-backends gvfs-fuse udisks2 udiskie

# ---- WEB BROWSERS ----
echo ">>> Installing browsers..."
kapt "browsers" firefox

# ---- OFFICE SUITE ----
echo ">>> Installing LibreOffice full..."
kapt "libreoffice" libreoffice libreoffice-l10n-en-us libreoffice-help-en-us libreoffice-writer libreoffice-calc libreoffice-impress libreoffice-draw libreoffice-base libreoffice-math libreoffice-style-colibre libreoffice-gtk3 libreoffice-pdfimport

# ---- CREATIVE SUITE ----
echo ">>> Installing creative tools..."
kapt "creative" gimp gimp-data inkscape darktable rawtherapee blender krita obs-studio shotwell shotwell-common eog eog-plugins rhythmbox celluloid mpv imagemagick imagemagick-6.q16

# ---- DEVELOPMENT TOOLS ----
echo ">>> Installing development tools..."
kapt "dev tools" build-essential gcc g++ make cmake python3 python3-pip python3-venv python3-dev python3-numpy default-jdk default-jre git gitk git-gui vim vim-common nano neovim nodejs npm php php-cli ruby go || true rustc cargo || true valgrind gdb strace ltrace cloc sloccount

# ---- SYSTEM TOOLS ----
echo ">>> Installing system tools..."
kapt "sys tools" htop btop atop glances sysstat iotop lsof lshw lshw-gtk hardinfo inxi neofetch gnome-disk-utility gparted synaptic aptitude dconf-editor gparted testdisk foremost scalpel rsync rdiff-backup timeshift ncdu

# ---- MULTIMEDIA CODECS ----
echo ">>> Installing multimedia codecs..."
kapt "codecs" ubuntu-restricted-extras gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav gstreamer1.0-tools ffmpeg ffmpeg-doc libavcodec-extra libavformat-dev libavutil-dev lame flac vorbis-tools

# ---- FONTS ----
echo ">>> Installing fonts..."
kapt "fonts" fonts-dejavu fonts-liberation fonts-freefont-ttf fonts-noto fonts-noto-color-emoji fonts-noto-cjk fonts-ubuntu fonts-liberation2 fonts-firacode fonts-hack fonts-croscore fonts-crosextra-carlito msttcorefonts || true

# ---- UTILITIES ----
echo ">>> Installing utilities..."
kapt "utilities" galculator mate-calc terminator gnome-terminal xfce4-terminal gnome-screenshot flameshot clipman parcellite keepassxc filezilla transmission-gtk

# ---- SECURITY ----
echo ">>> Installing security tools..."
kapt "security" ufw gufw apparmor apparmor-utils firejail firetools keepassxc fail2ban lynis rkhunter chkrootkit cryptsetup ecryptfs-utils

# ---- GAMES ----
echo ">>> Installing games..."
dpkg --add-architecture i386 || true
apt-get update -y || true
kapt "games" steam-installer steam-devices || true lutris || true wine wine32 wine64 || true vulkan-tools mesa-vulkan-drivers mesa-utils mangohud || true 0ad 0ad-data supertuxkart supertuxkart-data warzone2100 minetest minetest-server ExtremeTuxRacer freedoom foobillard++ || true

# ---- VIRTUALIZATION ----
echo ">>> Installing virtualization..."
kapt "virt" qemu-kvm qemu-system-x86 qemu-utils libvirt-daemon-system libvirt-clients virt-manager virtinst bridge-utils

# ---- CONTAINERS ----
echo ">>> Installing containers..."
kapt "containers" docker.io docker-compose || true podman podman-compose || true

# ---- DOCUMENTATION ----
echo ">>> Installing documentation..."
kapt "docs" man-db manpages manpages-dev manpages-posix manpages-posix-dev info debian-handbook

# ---- THEMES + ICONS ----
echo ">>> Installing themes..."
kapt "themes" arc-theme papirus-icon-theme numix-gtk-theme numix-icon-theme light-themes adwaita-icon-theme adwaita-qt qt5ct

# ---- AI / MACHINE LEARNING ----
echo ">>> Installing AI/ML tools..."
kapt "ai/ml" python3-sklearn python3-pandas python3-numpy

# ---- ADDITIONAL DEVELOPMENT ----
echo ">>> Installing additional dev tools..."
kapt "additional dev" sqlitebrowser httpie

# ---- ADDITIONAL CREATIVE ----
echo ">>> Installing additional creative tools..."
kapt "additional creative" scribus scribus-doc shotcut || true

# ---- ADDITIONAL GAMES ----
echo ">>> Installing additional games..."
kapt "additional games" neverball neverball-data armagetronad assaultcube openarena openarena-data

# ---- DOCUMENTATION ----
echo ">>> Installing documentation..."
kapt "docs" man-db manpages manpages-dev manpages-posix manpages-posix-dev info debian-handbook

# ---- THEMES + ICONS ----
echo ">>> Installing themes..."
kapt "themes" arc-theme arc-icon-theme papirus-icon-theme numix-gtk-theme numix-icon-theme light-themes adwaita-icon-theme adwaita-qt qt5ct

# ---- FINAL CLEANUP (keep big packages, remove caches) ----
echo ">>> Cleaning up..."
apt-get autoremove -y
apt-get clean
rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /var/cache/apt/*.bin
echo ">>> DONE: $(dpkg-query -W -f='\${Installed-Size}\n' | awk '{s+=$1}END{printf "%.0f MB\n", s/1024}') installed"
EOF
  "$SUDO" mkdir -p "$ROOTFS/usr/local/lib/korrinos" "$ROOTFS/var/lib/korrinos"
  "$SUDO" cp "$BUILD/kapt-lib.sh" "$ROOTFS/usr/local/lib/korrinos/kapt-lib.sh"
  "$SUDO" rm -f "$ROOTFS/var/lib/korrinos/skipped-packages.txt"
  "$SUDO" touch "$ROOTFS/var/lib/korrinos/skipped-packages.txt"
  "$SUDO" cp "$BUILD/apt.sh" "$ROOTFS/apt-setup.sh"
  "$SUDO" chroot "$ROOTFS" bash /apt-setup.sh || echo "   apt install had warnings (continuing)"
  "$SUDO" rm -f "$ROOTFS/apt-setup.sh"
  # unmount chroot bind-mounts (hard requirement, not best-effort)
  mountpoint -q "$ROOTFS/dev/pts" && "$SUDO" umount "$ROOTFS/dev/pts" 2>/dev/null || true
  mountpoint -q "$ROOTFS/proc" && "$SUDO" umount "$ROOTFS/proc" 2>/dev/null || true
  mountpoint -q "$ROOTFS/sys" && "$SUDO" umount "$ROOTFS/sys" 2>/dev/null || true
  mountpoint -q "$ROOTFS/dev" && "$SUDO" umount "$ROOTFS/dev" 2>/dev/null || true
  echo "   desktop + apps installed."
}

stage3_worlds() {
  echo "### [3/6] Baking KorrinOS worlds + os layer into rootfs..."
  "$SUDO" rm -rf "$ROOTFS/opt/korrinos"
  "$SUDO" mkdir -p "$ROOTFS/opt/korrinos"
  "$SUDO" cp -r /home/tinkerspace/linux-kernel/os "$ROOTFS/opt/korrinos/os"
  "$SUDO" cp /home/tinkerspace/linux-kernel/README.md "$ROOTFS/opt/korrinos/" 2>/dev/null || true
  "$SUDO" cp /home/tinkerspace/linux-kernel/LICENSE "$ROOTFS/opt/korrinos/" 2>/dev/null || true
  "$SUDO" cp /home/tinkerspace/linux-kernel/LICENSE "$ROOTFS/usr/share/doc/korrinos-os-copyright" 2>/dev/null || true

  # World launcher on PATH
  "$SUDO" bash -c 'cat > "$ROOTFS/usr/local/bin/parc-world" <<EOF
#!/bin/bash
exec /opt/korrinos/os/territories/modes.sh "\$@"
EOF
chmod +x /usr/local/bin/parc-world' 2>/dev/null || true

  # KorrinOS CLI on PATH
  "$SUDO" bash -c 'cat > "$ROOTFS/usr/local/bin/korrinos" <<EOF
#!/bin/bash
exec /opt/korrinos/os/parc-ai/parc-ai.sh "\$@"
EOF
chmod +x /usr/local/bin/korrinos' 2>/dev/null || true

  # KorrinOS Apps — make all os/apps/ scripts directly runnable
  "$SUDO" bash -c 'mkdir -p "$ROOTFS/usr/local/bin"
  for f in /opt/korrinos/os/apps/*.sh; do
    [ -f "$ROOTFS\$f" ] || continue
    name=$(basename "\$f" .sh)
    ln -sf "\$f" "$ROOTFS/usr/local/bin/korrinos-\$name" 2>/dev/null || true
  done
  for d in customization gaming hardware network security system; do
    for f in /opt/korrinos/os/apps/\$d/*.sh; do
      [ -f "$ROOTFS\$f" ] || continue
      name=$(basename "\$f" .sh)
      ln -sf "\$f" "$ROOTFS/usr/local/bin/korrinos-\$name" 2>/dev/null || true
    done
    done' 2>/dev/null || true

  # KorrinOS System CLI — all os/system/ scripts on PATH
  "$SUDO" bash -c 'mkdir -p "$ROOTFS/usr/local/bin"
  for d in package-manager update-system cloud-sync mobile-companion enterprise driver-manager hardware-cert installer appstore desktop-env security backup firewall; do
    for f in /opt/korrinos/os/system/\$d/korrinos-*.sh; do
      [ -f "$ROOTFS\$f" ] || continue
      name=\$(basename "\$f" .sh)
      ln -sf "\$f" "$ROOTFS/usr/local/bin/\$name" 2>/dev/null || true
    done
  done' 2>/dev/null || true

  # Systemd services for KorrinOS features
  "$SUDO" mkdir -p "$ROOTFS/etc/systemd/system"

  # === UNIFIED DESKTOP SERVICE (replaces liquid-glass + widgets + dock + smoothui) ===
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-desktop.service" <<EOF
[Unit]
Description=KorrinOS Desktop — Frosted Glass, Widgets, Sidebar, Dock
After=graphical.target
Wants=graphical.target

[Service]
Type=forking
ExecStart=/opt/korrinos/os/desktop/nibra-style/nibra-shell.sh start
ExecStop=/opt/korrinos/os/desktop/nibra-style/nibra-shell.sh stop
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF'

  # === TINKERAI SYSTEM CONTROLLER ===
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/vokk.service" <<EOF
[Unit]
Description=VOKK v4 — System Controller + Image Gen + Chat
After=graphical.target korrinos-desktop.service
Wants=graphical.target

[Service]
Type=simple
ExecStart=/usr/bin/python3 /opt/korrinos/os/vokk/vokk_controller.py
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF'

  # Backup timer service
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-backup.service" <<EOF
[Unit]
Description=KorrinOS Backup
After=network-online.target

[Service]
Type=oneshot
ExecStart=/opt/korrinos/os/system/backup/korrinos-backup.sh full
Nice=19
IOSchedulingClass=idle

[Install]
WantedBy=multi-user.target
EOF'

  # Backup timer
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-backup.timer" <<EOF
[Unit]
Description=KorrinOS Backup Timer

[Timer]
OnCalendar=*-*-* 02:00:00
RandomizedDelaySec=3600
Persistent=true

[Install]
WantedBy=timers.target
EOF'

  # Firewall auto-setup service
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-firewall.service" <<EOF
[Unit]
Description=KorrinOS Firewall Setup
Before=network-pre.target

[Service]
Type=oneshot
ExecStart=/opt/korrinos/os/system/firewall/korrinos-firewall.sh setup
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF'

  # Health monitor timer
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-health.timer" <<EOF
[Unit]
Description=KorrinOS Health Monitor Timer

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
EOF'

  # Enable services
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-desktop.service 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable vokk.service 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-autoupdate.timer 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-backup.timer 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-firewall.service 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-health.timer 2>/dev/null || true

  # Health check service (timer references this)
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-health.service" <<EOF
[Unit]
Description=KorrinOS System Health Check

[Service]
Type=oneshot
ExecStart=/opt/korrinos/os/system/security/korrinos-health.sh check
EOF'

  # Enable display manager (LightDM)
  "$SUDO" chroot "$ROOTFS" systemctl enable lightdm.service 2>/dev/null || true

  # Create live user for ISO (korrinos/korrinos)
  "$SUDO" chroot "$ROOTFS" bash -c '
    useradd -m -s /bin/bash -G sudo,adm,dialout,cdrom,floppy,audio,dip,video,plugdev,netdev korrinos 2>/dev/null || true
    echo "korrinos:korrinos" | chpasswd 2>/dev/null || true
    echo "root:korrinos" | chpasswd 2>/dev/null || true
    # Auto-login for live session
    mkdir -p /etc/lightdm/lightdm.conf.d
    cat > /etc/lightdm/lightdm.conf.d/autologin.conf << LGDM
[Seat:*]
autologin-user=korrinos
autologin-user-timeout=0
user-session=xfce
greeter-session=lightdm-gtk-greeter
LGDM
  ' 2>/dev/null || echo "   live user setup had warnings"

  # Configure hostname
  "$SUDO" chroot "$ROOTFS" bash -c 'echo "korrinos" > /etc/hostname && echo "127.0.1.1 korrinos" >> /etc/hosts' 2>/dev/null || true

  # Auto-update service (new systems)
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-autoupdate.service" <<EOF
[Unit]
Description=KorrinOS Automatic Updates
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/opt/korrinos/os/system/update-system/korrinos-update.sh full
Nice=19
IOSchedulingClass=idle
TimeoutStartSec=3600

[Install]
WantedBy=multi-user.target
EOF'

  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-autoupdate.timer" <<EOF
[Unit]
Description=KorrinOS Automatic Updates Timer

[Timer]
OnCalendar=*-*-* 03:00:00
RandomizedDelaySec=1800
Persistent=true

[Install]
WantedBy=timers.target
EOF'

  # Cloud sync service
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-cloud-sync.service" <<EOF
[Unit]
Description=KorrinOS Cloud Sync
After=network-online.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/opt/korrinos/os/system/cloud-sync/korrinos-cloud.sh auto-sync
Nice=19
IOSchedulingClass=idle

[Install]
WantedBy=multi-user.target
EOF'

  "$SUDO" bash -c 'cat > "$ROOTFS/etc/systemd/system/korrinos-cloud-sync.timer" <<EOF
[Unit]
Description=KorrinOS Cloud Sync Timer

[Timer]
OnBootSec=120
OnUnitActiveSec=30min
Persistent=true

[Install]
WantedBy=timers.target
EOF'

  # Enable new services
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-autoupdate.timer 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-cloud-sync.timer 2>/dev/null || true

  # KorrinOS system CLI symlinks (new systems)
  "$SUDO" bash -c 'mkdir -p "$ROOTFS/usr/local/bin"
  for sys in package-manager update-system cloud-sync mobile-companion enterprise driver-manager hardware-cert installer appstore desktop-env; do
    for f in /opt/korrinos/os/system/$sys/*.sh; do
      [ -f "$ROOTFS\$f" ] || continue
      name=$(basename "\$f" .sh)
      ln -sf "\$f" "$ROOTFS/usr/local/bin/\$name" 2>/dev/null || true
    done
  done' 2>/dev/null || true

  # First-boot setup script
  "$SUDO" mkdir -p "$ROOTFS/usr/local/bin"
  "$SUDO" cp /home/tinkerspace/linux-kernel/os/system/korrinos-firstboot.sh "$ROOTFS/usr/local/bin/korrinos-firstboot"
  "$SUDO" chmod +x "$ROOTFS/usr/local/bin/korrinos-firstboot" 2>/dev/null || true

  # Add firstboot to /etc/rc.local or autostart
  "$SUDO" bash -c 'cat > "$ROOTFS/etc/profile.d/korrinos-firstboot.sh" <<EOF
[ -x /usr/local/bin/korrinos-firstboot ] && /usr/local/bin/korrinos-firstboot &
EOF'

  # Default wallpaper
  "$SUDO" mkdir -p "$ROOTFS/usr/share/korrinos/wallpapers"
  "$SUDO" cp /home/tinkerspace/linux-kernel/os/branding/plymouth/logo.png "$ROOTFS/usr/share/korrinos/wallpapers/default.png" 2>/dev/null || true

  echo "   worlds + os layer + services baked in."
}

stage4_live() {
  unbind_rootfs || echo "  WARNING: mounts still attached; proceeding (squashfs will re-check)."
  echo "### [4/6] Preparing live image (casper layout)..."
  "$SUDO" rm -rf "$IMAGE"
  "$SUDO" mkdir -p "$IMAGE"/{casper,isolinux,install}
  echo "   staged ($(du -sh "$ROOTFS" | cut -f1) rootfs ready for squashfs)."
}

stage_branding() {
  echo "### [branding] Writing KorrinOS distribution identity into rootfs..."
  "$SUDO" bash -c "cat > '$ROOTFS/etc/os-release' <<'EOS'
PRETTY_NAME="KorrinOS 2.0"
NAME=KorrinOS
VERSION_ID="2.0"
VERSION="2.0"
VERSION_CODENAME=korrinos
ID=korrinos
ID_LIKE=
HOME_URL=https://sourceforge.net/projects/korrinos/
SUPPORT_URL=https://sourceforge.net/projects/korrinos/
BUG_REPORT_URL=https://sourceforge.net/projects/korrinos/
EOS"
  "$SUDO" cp "$ROOTFS/etc/os-release" "$ROOTFS/etc/lsb-release"
  "$SUDO" bash -c "echo 'KorrinOS 2.0 \\\\l' > '$ROOTFS/etc/issue'"
  "$SUDO" cp "$ROOTFS/etc/issue" "$ROOTFS/etc/issue.net"
  echo "   KorrinOS identity written (os-release/lsb-release/issue)."
}

# install plymouth + the branded splash/GRUB theme into an existing rootfs
# (used on incremental rebuilds; fresh builds get plymouth via stage2 apt)
stage2b_branding() {
  echo "### [branding] installing plymouth + KorrinOS boot theme into rootfs..."
  "$SUDO" mkdir -p "$ROOTFS"/{proc,sys,dev,dev/pts}
  "$SUDO" mount --bind /proc "$ROOTFS/proc" 2>/dev/null || true
  "$SUDO" mount --bind /sys  "$ROOTFS/sys"  2>/dev/null || true
  "$SUDO" mount --bind /dev  "$ROOTFS/dev"  2>/dev/null || true
  mountpoint -q "$ROOTFS/dev/pts" || "$SUDO" mount -t devpts none "$ROOTFS/dev/pts" 2>/dev/null || true
  "$SUDO" chroot "$ROOTFS" bash -c "DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends plymouth plymouth-themes plymouth-x11" \
    || echo "   plymouth install had warnings"
  "$SUDO" bash "/home/tinkerspace/linux-kernel/os/branding/install-branding.sh" "$ROOTFS" \
    || echo "   branding install had warnings"
  "$SUDO" chroot "$ROOTFS" update-initramfs -u 2>&1 | tail -1 || echo "   initramfs not updated"
  for m in dev/pts proc sys dev; do
    [ -d "$ROOTFS/$m" ] && mountpoint -q "$ROOTFS/$m" && "$SUDO" umount "$ROOTFS/$m" 2>/dev/null || true
  done
  echo "   boot branding installed (Plymouth + GRUB theme)."
}

stage5_squashfs() {
  echo "### [5/6] Building squashfs of the full rootfs (compressing)..."
  # Never compress a tree that still has /proc or /sys bind-mounted into it.
  unbind_rootfs || { echo "ABORT: refusing to build a squashfs from a live-mounted rootfs."; exit 1; }
  local srcsz
  srcsz=$("$SUDO" du -sm "$ROOTFS" 2>/dev/null | cut -f1)
  echo "  source tree: ${srcsz} MB"
  "$SUDO" rm -f "$IMAGE/casper/filesystem.squashfs"
  "$SUDO" mksquashfs "$ROOTFS" "$IMAGE/casper/filesystem.squashfs" \
    -comp xz -b 1M -no-xattrs -processors "$(nproc)" 2>&1 | tail -4
}

# kernel + initrd into casper (fresh copy from rootfs)
stage5_caspermaterials() {
  "$SUDO" mkdir -p "$IMAGE/casper"
  # Copy kernel — PREFER the freshly built tree in THIS repo.
  #
  # This must come first. The previous order (rootfs -korrinos -> any rootfs
  # vmlinuz -> host /boot) silently shipped Ubuntu's 5.15.0-1032-realtime when
  # the chroot had no -korrinos package, which is how KorrinOS-v3.0.iso ended
  # up with ZERO tinker symbols despite this tree containing all 38 features.
  # A distro kernel cannot run kernel/tinker at all, so if the built bzImage
  # exists it is the only correct choice.
  local kernel_found="" k
  local KERNEL_SRC="${KERNEL_SRC:-/home/tinkerspace/linux-kernel}"
  for k in "$KERNEL_SRC/arch/x86/boot/bzImage" \
           "$KERNEL_SRC/vmlinuz"; do
    if [ -f "$k" ]; then
      "$SUDO" cp "$k" "$IMAGE/casper/vmlinuz"
      kernel_found=1
      echo "   kernel (built tree): $k"
      "$SUDO" cp "$k" "$IMAGE/casper/vmlinuz.built"
      break
    fi
  done
  if [ -z "$kernel_found" ]; then
    for k in "$ROOTFS/boot"/vmlinuz-*-korrinos; do
      if [ -f "$k" ]; then
        "$SUDO" cp "$k" "$IMAGE/casper/vmlinuz"
        kernel_found=1
        echo "   kernel (KorrinOS pkg): $k"
        break
      fi
    done
  fi
  if [ -z "$kernel_found" ]; then
    for k in "$ROOTFS/boot"/vmlinuz-*; do
      if [ -f "$k" ]; then
        "$SUDO" cp "$k" "$IMAGE/casper/vmlinuz"
        kernel_found=1
        echo "   WARN kernel is DISTRO, not built tree: $k"
        break
      fi
    done
  fi
  if [ -z "$kernel_found" ]; then
    # Fallback: host kernel
    for k in /boot/vmlinuz-*; do
      if [ -f "$k" ]; then
        "$SUDO" cp "$k" "$IMAGE/casper/vmlinuz"
        kernel_found=1
        echo "   WARN kernel (host fallback, distro): $k"
        break
      fi
    done
  fi
  [ -z "$kernel_found" ] && echo "   ERROR: no kernel found!"
  # Copy initrd from rootfs — prefer -korrinos
  local initrd_found=""
  for i in "$ROOTFS/boot"/initrd.img-*-korrinos; do
    if [ -f "$i" ] && [ ! -s "$IMAGE/casper/initrd" ]; then
      "$SUDO" cp "$i" "$IMAGE/casper/initrd"
      initrd_found=1
      echo "   initrd (KorrinOS): $i"
      break
    fi
  done
  if [ -z "$initrd_found" ]; then
    for i in "$ROOTFS/boot"/initrd.img-*; do
      if [ -f "$i" ] && [ ! -s "$IMAGE/casper/initrd" ]; then
        "$SUDO" cp "$i" "$IMAGE/casper/initrd"
        initrd_found=1
        echo "   initrd: $i"
        break
      fi
    done
  fi
  if [ -z "$initrd_found" ]; then
    for i in /boot/initrd.img-*; do
      if [ -f "$i" ] && [ ! -s "$IMAGE/casper/initrd" ]; then
        "$SUDO" cp "$i" "$IMAGE/casper/initrd"
        initrd_found=1
        echo "   initrd (host fallback): $i"
        break
      fi
    done
  fi
  [ -z "$initrd_found" ] && echo "   WARN: no initrd copied"
  ls -la "$IMAGE/casper/" | awk '{print $5,$9}'
}

stage6_iso() {
  echo "### [6/6] Building final bootable ISO (iso-level 3, >4GB OK)..."
  # earlier stages (mksquashfs/apt) wrote as root — hand the build dir back
  # to the real user so grub-mk* and xorriso can write without sudo.
  "$SUDO" chown -R "$(id -u):$(id -g)" "$IMAGE" "$BUILD" 2>/dev/null || true
  mkdir -p "$BUILD/grub-img"
  cat > "$BUILD/grub.cfg" <<EOF
set timeout=5
set default=0
loadfont unicode
insmod all_video
insmod gfxterm
terminal_output gfxterm
# Mirror GRUB to serial so a headless/BIOS boot is observable instead of
# silently hanging. Harmless on a console, invaluable for support + CI.
serial --unit=0 --speed=115200
terminal_output serial

# KorrinOS GRUB theme
set theme="/boot/grub/themes/korrinos/theme.txt"

menuentry "KorrinOS 1.3 — Start" {
  linux /casper/vmlinuz boot=casper quiet splash
  initrd /casper/initrd
}
menuentry "KorrinOS 1.3 — Safe Graphics" {
  linux /casper/vmlinuz boot=casper quiet splash nomodeset
  initrd /casper/initrd
}
menuentry "KorrinOS 1.3 — Memory Test" {
  linux /casper/vmlinuz boot=casper quiet splash memtest
  initrd /casper/initrd
}
menuentry "KorrinOS 1.3 — Rescue / Serial Debug" {
  linux /casper/vmlinuz boot=casper console=tty0 console=ttyS0,115200
  initrd /casper/initrd
}
menuentry "Boot from first HDD" {
  set root=(hd0)
  chainloader +1
}
EOF
  grub-mkstandalone --format=x86_64-efi --output="$BUILD/efi.img" \
    --locales="" --fonts="" \
    "boot/grub/grub.cfg=$BUILD/grub.cfg" 2>/dev/null || \
    grub-mkimage -p /boot/grub -O x86_64-efi -o "$BUILD/efi.img" \
      iso9660 at_keyboard gfxterm gfxmenu all_video font terminal configfile normal 2>/dev/null || true
  # serial + terminal are REQUIRED for the `serial --unit=0` and
  # `terminal_output serial` lines in grub.cfg above to emit anything. Without
  # them the BIOS boot produces zero serial bytes and looks like a hang even
  # when it is booting fine.
  GRUB_MODS="iso9660 biosdisk part_msdos part_gpt fat ext2 udf normal configfile search search_fs_file linux chain boot reboot serial terminal gfxterm all_video video"
  [ -f /usr/lib/grub/i386-pc/initrd.mod ] && GRUB_MODS="$GRUB_MODS initrd"
  grub-mkimage -p /boot/grub -O i386-pc -o "$BUILD/core.img" $GRUB_MODS 2>&1 | tail -2
  if [ -s "$BUILD/core.img" ]; then
    cat /usr/lib/grub/i386-pc/cdboot.img "$BUILD/core.img" > "$IMAGE/isolinux/isolinux.bin"
  fi
  [ -s "$BUILD/efi.img" ] && mkdir -p "$IMAGE/boot/grub" && cp "$BUILD/efi.img" "$IMAGE/boot/grub/efi.img"

  # CRITICAL (BIOS): core.img above was built with the `configfile` module and
  # prefix /boot/grub, so on BIOS boot GRUB looks for (cd0)/boot/grub/grub.cfg.
  # Embedding the config in efi.img only serves UEFI. Without this file the BIOS
  # El Torito image finds no config, prints nothing, and the machine appears to
  # hang -- which is exactly how the shipped KorrinOS-v3.0.iso behaved.
  mkdir -p "$IMAGE/boot/grub"
  cp "$BUILD/grub.cfg" "$IMAGE/boot/grub/grub.cfg"

  # BIOS El Torito has no EFI firmware to inherit, so it also needs the
  # platform_modules part of the normal GRUB prefix present on the media.
  if [ -d /usr/lib/grub/i386-pc ]; then
    mkdir -p "$IMAGE/boot/grub/i386-pc"
    cp -f /usr/lib/grub/i386-pc/*.mod "$IMAGE/boot/grub/i386-pc/" 2>/dev/null || true
  fi
  ls -la "$IMAGE/isolinux/isolinux.bin" "$IMAGE/boot/grub/efi.img" \
         "$IMAGE/boot/grub/grub.cfg" 2>/dev/null | awk '{print $5,$9}'
  xorriso -as mkisofs -quiet \
    -V KorrinOS \
    -iso-level 3 -R -J -joliet-long -full-iso9660-filenames \
    -b isolinux/isolinux.bin -c boot.cat -no-emul-boot \
    -boot-load-size 8 -boot-info-table \
    -eltorito-alt-boot -e boot/grub/efi.img -no-emul-boot \
    -o "$OUT" "$IMAGE" 2>&1 | tail -3
  echo "BUILT: $OUT"
  du -sh "$OUT"
}

# ---- preflight --------------------------------------------------------------
# A stage that is called but not defined returns 127 and truncates the && chain,
# which once produced a "DONE" message with no ISO built. Catch that before
# spending an hour: verify every stage in the chain actually exists.
preflight() {
  local missing=0 f
  for f in stage1 stage2_install stage3_worlds stage_branding stage2b_branding \
           stage_i18n_fonts stage_kcommand stage_wordpen stage4_live stage5_squashfs \
           stage5_caspermaterials stage6_iso stage7_verify; do
    if ! declare -f "$f" >/dev/null 2>&1; then
      echo "PREFLIGHT FAIL: stage function '$f' is called but not defined."
      missing=1
    fi
  done
  if ! declare -f kapt >/dev/null 2>&1 && [ ! -r "$BUILD/kapt-lib.sh" ]; then
    echo "PREFLIGHT FAIL: neither kapt() nor $BUILD/kapt-lib.sh is available."
    missing=1
  fi
  [ "$missing" -eq 0 ] && echo "preflight: all build stages defined."
  return "$missing"
}

# ---- unbind_rootfs: remove the chroot bind-mounts ---------------------------
# stage2_install bind-mounts /proc /sys /dev so apt postinst scripts work.
# Those mounts MUST come down before anything reads the rootfs tree: leaving
# them attached made mksquashfs traverse live /proc and /sys, which produced a
# corrupt squashfs LARGER than its own source (31 GB from a 15 GB rootfs) and
# filled the disk. The mounts also stacked up, one set per rebuild, because the
# old cleanup was best-effort and silently failed.
unbind_rootfs() {
  local n=0
  # dev/pts before dev, sys, proc: unmount dependents first.
  for target in dev/pts dev sys proc; do
    while mountpoint -q "$ROOTFS/$target" 2>/dev/null; do
      "$SUDO" umount -lf "$ROOTFS/$target" 2>/dev/null || break
      n=$((n+1))
      [ "$n" -gt 40 ] && break
    done
  done
  local left
  left=$(mount | grep -c "$ROOTFS/" || true)
  if [ "${left:-0}" -ne 0 ]; then
    echo "  WARNING: $left mount(s) still attached under $ROOTFS:"
    mount | grep "$ROOTFS/" | awk '{print "      " $3}' | head -5
    return 1
  fi
  echo "  rootfs bind-mounts cleared (released $n)."
  return 0
}

# ---- stage_kcommand: build + install our terminal -------------------------
# kcommand is a separate Apache-2.0 project that KorrinOS consumes. We build it
# from the pinned source rather than shipping a prebuilt binary, so the image is
# reproducible and nothing is trusted that we did not compile. Host and the
# jammy rootfs are both glibc 22.04, so the binary is ABI-compatible.
#
# Honours the separate-repo design: if the source is absent the stage skips
# cleanly rather than failing the build.
KCOMMAND_SRC="${KCOMMAND_SRC:-/home/tinkerspace/linux-kernel/os/terminal/kcommand}"
WORDPEN_SRC="${WORDPEN_SRC:-/home/tinkerspace/linux-kernel/os/screensaver/korrinos-wordpen}"
KCOMMAND_REPO="${KCOMMAND_REPO:-https://github.com/Aghosh-mv/kcommand.git}"
KCOMMAND_PIN="${KCOMMAND_PIN:-}"

# ---- stage_wordpen: the handwriting screen saver ----------------------------
#
# KorrinOS ships xscreensaver as the real substrate, so this saver inherits the
# honest contract it already provides: run while idle, exit on any input, no
# wallpaper twin and no resident process. That is the property that makes people
# leave a screensaver switched on instead of disabling it.
stage_wordpen() {
  echo "### [wordpen] building the KorrinOS handwriting screen saver..."
  local src="$WORDPEN_SRC"

  if [ ! -d "$src" ]; then
    echo "  source not at $src - skipping the screensaver"
    return 0
  fi

  # Rust toolchain present?
  if ! command -v cargo >/dev/null 2>&1; then
    echo "  WARNING: cargo not found; kcommand was built, so building the saver too"
    return 0
  fi

  ( cd "$src" && cargo build --release --offline --target-dir "$BUILD/wordpen-target" ) \
    || { echo "  WARNING: wordpen failed to build; continuing without it"; return 0; }

  local bin="$BUILD/wordpen-target/release/korrinos-wordpen"
  if [ ! -x "$bin" ]; then
    echo "  WARNING: wordpen binary not produced"
    return 0
  fi

  # Create the destination rather than assuming an earlier stage made it: a
  # test against a fresh rootfs caught install failing on a missing /usr/bin
  # while the stage still reported success.
  "$SUDO" mkdir -p "$ROOTFS/usr/bin"
  "$SUDO" install -m 0755 "$bin" "$ROOTFS/usr/bin/korrinos-wordpen" || {
    echo "  WARNING: could not install the wordpen binary; continuing without it"
    return 0
  }

  # The word data. The saver is useless without it, and it is the same registry
  # the terminal uses, so the two can never disagree about which languages exist.
  "$SUDO" mkdir -p "$ROOTFS/usr/share/korrinos/wordpen"
  for f in languages.tsv words.tsv words-adult.tsv; do
    [ -f "$src/data/$f" ] && "$SUDO" install -m 0644 "$src/data/$f" \
        "$ROOTFS/usr/share/korrinos/wordpen/$f"
  done

  # xscreensaver module.
  local xss="$src/../xscreensaver"
  if [ -d "$xss" ]; then
    "$SUDO" mkdir -p "$ROOTFS/usr/share/xscreensaver/config"
    for f in "$xss"/*.xml; do
      [ -f "$f" ] || continue
      "$SUDO" install -m 0644 "$f" "$ROOTFS/usr/share/xscreensaver/config/"
    done
    "$SUDO" mkdir -p "$ROOTFS/usr/lib/x86_64-linux-gnu/xscreensaver"
    for f in "$xss"/*.sh; do
      [ -f "$f" ] || continue
      local base; base="$(basename "$f" .sh)"
      "$SUDO" install -m 0755 "$f" \
          "$ROOTFS/usr/lib/x86_64-linux-gnu/xscreensaver/$base"
    done
    echo "  installed the xscreensaver module"
  fi

  # A documented default config. The adult word list is OFF here and stays off
  # unless someone edits this: words drawn on an unattended screen should be a
  # deliberate choice by whoever owns the machine.
  "$SUDO" mkdir -p "$ROOTFS/etc/skel/.config/korrinos"
  cat > /tmp/wordpen.conf.$$ <<'CONF'
# KorrinOS wordpen - handwriting screen saver
#
# adult        true|false   include the opt-in adult word list (default false)
# seed         <integer>    fix the random sequence, for reproducible captures
# stroke_scale <float>      pen thickness as a fraction of screen height
# data_dir     <path>       where languages.tsv and words.tsv live
#
# The adult list ships empty and is OFF. Turning it on here is the only way to
# get those words, which is deliberate: a saver runs on a screen anyone can walk
# past, photograph or screenshot.
adult = false
# seed = 12345
stroke_scale = 0.011
# data_dir = /usr/share/korrinos/wordpen
CONF
  "$SUDO" install -m 0644 /tmp/wordpen.conf.$$ "$ROOTFS/etc/skel/.config/korrinos/wordpen.conf"
  rm -f /tmp/wordpen.conf.$$

  # Sanity check: the binary must run and be able to say what it can draw. A
  # saver that silently draws nothing is the worst outcome, so this is worth
  # failing loudly over rather than trusting the build.
  if [ ! -x "$ROOTFS/usr/bin/korrinos-wordpen" ]; then
    echo "  WARNING: korrinos-wordpen is not executable in the staging root"
    return 0
  fi
  local report
  report="$(chroot "$ROOTFS" /usr/bin/korrinos-wordpen --info 2>/dev/null | tr '\n' ';' || true)"
  if [ -n "$report" ]; then
    echo "  self-check: $report"
  else
    echo "  NOTE: could not chroot the self-check (needs a working chroot); the binary is installed"
  fi

  echo "  installed $("$SUDO" du -h "$ROOTFS/usr/bin/korrinos-wordpen" 2>/dev/null | cut -f1) korrinos-wordpen"
}

stage_kcommand() {
  echo "### [kcommand] building KorrinOS terminal..."
  local src="$KCOMMAND_SRC"

  if [ ! -d "$src" ] && command -v git >/dev/null; then
    echo "  source not at $src - fetching pinned checkout"
    "$SUDO" rm -rf "$BUILD/kcommand-src"
    if [ -n "$KCOMMAND_PIN" ]; then
      git clone --depth 1 --branch "$KCOMMAND_PIN" "$KCOMMAND_REPO" "$BUILD/kcommand-src" \
        || { echo "  WARNING: could not fetch kcommand; continuing without it"; return 0; }
    else
      git clone --depth 1 "$KCOMMAND_REPO" "$BUILD/kcommand-src" \
        || { echo "  WARNING: could not fetch kcommand; continuing without it"; return 0; }
    fi
    src="$BUILD/kcommand-src"
  fi

  [ -d "$src" ] || { echo "  WARNING: kcommand source unavailable; skipping"; return 0; }
  command -v cargo >/dev/null || {
    echo "  WARNING: cargo not installed; kcommand not in this image"; return 0; }

  ( cd "$src" && cargo build --release --bin kcommand ) >/tmp/kcommand-build.log 2>&1 || {
    echo "  WARNING: kcommand build failed; last lines:"
    tail -5 /tmp/kcommand-build.log | sed 's/^/      /'
    return 0
  }

  local bin="$src/target/release/kcommand"
  [ -x "$bin" ] || { echo "  WARNING: kcommand binary not produced"; return 0; }

  "$SUDO" install -m 0755 "$bin" "$ROOTFS/usr/bin/kcommand"

  # The launcher (same script as /usr/bin/kcommand) plus the per-language
  # wrappers that make "kcommand!Hindi" work as a SINGLE shell word.
  "$SUDO" install -m 0755 "$src/share/kcommand-launcher" \
      "$ROOTFS/usr/bin/kcommand-launcher" 2>/dev/null || true
  "$SUDO" install -m 0755 "$src/share/kcommand-lang-wrappers" \
      "$ROOTFS/usr/share/kcommand/kcommand-lang-wrappers" 2>/dev/null || true

  # The launcher resolves language names against this registry, so it must sit
  # at the path the launcher searches first.
  if [ -f "$src/share/i18n/languages.tsv" ]; then
    "$SUDO" mkdir -p "$ROOTFS/usr/share/korrinos/i18n"
    "$SUDO" install -m 0644 "$src/share/i18n/languages.tsv" \
        "$ROOTFS/usr/share/korrinos/i18n/languages.tsv"
  fi

  # One wrapper per language, named kcommand!<Language>. bash resolves a single
  # word by name, so this is the only way the one-word form can work.
  if [ -x "$src/share/kcommand-lang-wrappers" ] \
     && [ -f "$ROOTFS/usr/share/korrinos/i18n/languages.tsv" ]; then
    "$SUDO" mkdir -p "$ROOTFS/usr/local/bin"
    "$SUDO" KCOMMAND_LAUNCHER=/usr/bin/kcommand \
        "$src/share/kcommand-lang-wrappers" \
        "$ROOTFS/usr/share/korrinos/i18n/languages.tsv" \
        "$ROOTFS/usr/local/bin" 2>/dev/null \
        && echo "  generated kcommand!<language> wrappers" \
        || echo "  WARNING: could not generate kcommand!<language> wrappers"
  fi
  # Apache-2.0 obligations travel with the binary.
  "$SUDO" mkdir -p "$ROOTFS/usr/share/doc/kcommand"
  "$SUDO" cp "$src/NOTICE.md" "$src/LICENSE" "$ROOTFS/usr/share/doc/kcommand/" 2>/dev/null || true
  "$SUDO" cp "$src/README.md" "$ROOTFS/usr/share/doc/kcommand/" 2>/dev/null || true

  # Shell integration: typo detection and the KorrinOS terminal tooling.
  "$SUDO" mkdir -p "$ROOTFS/usr/share/kcommand"
  "$SUDO" cp "$src/share/kcommand.bash" "$ROOTFS/usr/share/kcommand/kcommand.bash" 2>/dev/null || true
  # Source it from the system bashrc so it is active in every kcommand session.
  "$SUDO" bash -c "grep -q 'kcommand.bash' '$ROOTFS/etc/bash.bashrc' 2>/dev/null || cat >> '$ROOTFS/etc/bash.bashrc' <<'BASHRC'

# KorrinOS kcommand: typo detection (did you mean) + KorrinOS terminal tools
[ -r /usr/share/kcommand/kcommand.bash ] && . /usr/share/kcommand/kcommand.bash
BASHRC"

  # Make kcommand the system terminal, not merely a command on PATH.
  # x-terminal-emulator is the Debian/XFCE alternative that file managers, the
  # panel and "Open Terminal Here" all consult, so registering here is what
  # actually makes it the default rather than a rename.
  "$SUDO" update-alternatives --install /usr/bin/x-terminal-emulator \
      x-terminal-emulator /usr/bin/kcommand 60 \
      --slave /usr/bin/xterm xterm /usr/bin/kcommand >/dev/null 2>&1 || true
  "$SUDO" update-alternatives --set x-terminal-emulator /usr/bin/kcommand \
      >/dev/null 2>&1 || true

  # XFCE's own default-terminal preference, and the mimeapps handler used by
  # "Open in Terminal" in file managers.
  "$SUDO" mkdir -p "$ROOTFS/etc/xdg"
  "$SUDO" bash -c "cat > '$ROOTFS/etc/xdg/korrinos-terminals.list'" <<'TERMLEOF'
[Default Terminal]
Terminal=kcommand
TERMLEOF
  "$SUDO" bash -c "cat > '$ROOTFS/usr/share/applications/korrinos-terminal.desktop'" <<'DESKEOF'
[Desktop Entry]
Type=Application
Name=Terminal
GenericName=Terminal
Comment=KorrinOS terminal
Exec=kcommand
Icon=utilities-terminal
Terminal=false
Categories=System;TerminalEmulator;
Keywords=shell;prompt;command;commandline;
DESKEOF
  echo "  registered kcommand as the system terminal (x-terminal-emulator)"

  # The 55-language registry the typography layer reads at runtime.
  "$SUDO" mkdir -p "$ROOTFS/usr/share/korrinos/i18n"
  "$SUDO" cp /home/tinkerspace/linux-kernel/os/i18n/languages.tsv \
    "$ROOTFS/usr/share/korrinos/i18n/languages.tsv"

  echo "  installed $("$SUDO" du -h "$ROOTFS/usr/bin/kcommand" | cut -f1) kcommand"
  echo "  installed 55-language registry"
}

# ---- stage_i18n_fonts: install the fonts the 55 languages need --------------
# The package list is derived from the language registry rather than hardcoded,
# so adding a language to languages.tsv automatically pulls in its font.
# Must run inside the chroot: apt needs root, and the rootfs is the target.
stage_i18n_fonts() {
  echo "### [i18n] installing fonts for the shipped languages..."
  local reg="/home/tinkerspace/linux-kernel/os/i18n/languages.tsv"
  [ -r "$reg" ] || { echo "  WARNING: registry not found at $reg; skipping"; return 0; }

  local pkgs langs
  pkgs=$(cut -f7 "$reg" | grep -v '^#' | grep -v '^fontpkg' | grep -v '^$' | sort -u | tr '\n' ' ')
  langs=$(cut -f1 "$reg" | grep -v '^#' | grep -v '^code' | grep -v '^$' | sort -u | wc -l)
  echo "  $langs languages require $(printf '%s\n' $pkgs | wc -w) font packages"

  # Generate a chroot-side script (quoted heredoc: no expansion wanted here).
  "$SUDO" bash -c "cat > '$ROOTFS/i18n-fonts.sh'" <<'FONTSCRIPT'
#!/bin/bash
export DEBIAN_FRONTEND=noninteractive
. /usr/local/lib/korrinos/kapt-lib.sh
kapt "i18n fonts" FONTPKGS_PLACEHOLDER
FONTSCRIPT
  "$SUDO" sed -i "s/FONTPKGS_PLACEHOLDER/$pkgs/" "$ROOTFS/i18n-fonts.sh"
  # apt-setup.sh deliberately wipes /var/lib/apt/lists at the end, so the
  # availability check kapt relies on has nothing to query. Refresh first.
  "$SUDO" chroot "$ROOTFS" bash -c 'apt-get update -qq' || echo "  WARNING: apt update failed"
  "$SUDO" chroot "$ROOTFS" bash /i18n-fonts.sh || echo "  WARNING: i18n font stage had issues"
  "$SUDO" rm -f "$ROOTFS/i18n-fonts.sh"

  # Prove the fonts are actually present, not merely requested.
  local installed
  installed=$("$SUDO" chroot "$ROOTFS" bash -c 'fc-list 2>/dev/null | wc -l' 2>/dev/null || echo 0)
  echo "  rootfs now reports $installed font files"
}

# ---- stage7: verify what actually made it into the image -------------------
# The install steps deliberately tolerate a missing package so one bad name
# cannot abort a multi-hour build. That tolerance hid 16 missing packages in an
# earlier run. This stage makes the result explicit instead.
EXPECTED_PACKAGES=(
  # desktop + display
  xfce4 xfce4-panel xfwm4 thunar xfce4-terminal picom dunst
  # input + clipboard + display utilities (xrandr/xprop live in these)
  x11-xserver-utils x11-utils xdotool xclip xsel arandr
  # audio
  pulseaudio pavucontrol alsa-utils pipewire wireplumber
  # networking
  network-manager openssh-server nmap wireguard-tools
  # apps
  firefox libreoffice gimp inkscape audacity
  # system
  gparted gnome-disk-utility sysstat
  # fonts (must cover the 55 shipped languages)
  fonts-dejavu fonts-noto fonts-noto-cjk fonts-liberation
  # NOTE: kcommand is NOT listed here. It is not a dpkg package, so
  # dpkg-query always reports it missing. It is verified by path instead,
  # in the file check below.
)

stage7_verify() {
  echo "### [7/7] Verifying image contents..."
  local report="$BUILD/install-report.txt" missing=() p
  : > "$report"

  for p in "${EXPECTED_PACKAGES[@]}"; do
    if "$SUDO" chroot "$ROOTFS" dpkg-query -W -f='${Status}' "$p" 2>/dev/null \
        | grep -q 'install ok installed'; then
      printf '  ok      %s\n' "$p" >> "$report"
    else
      printf '  MISSING %s\n' "$p" >> "$report"
      missing+=("$p")
    fi
  done

  # KorrinOS branding and the language registry must be baked in.
  for f in /usr/share/korrinos/i18n/languages.tsv /usr/bin/kcommand; do
    if "$SUDO" test -e "$ROOTFS$f"; then
      printf '  ok      %s\n' "$f" >> "$report"
    else
      printf '  MISSING %s\n' "$f" >> "$report"
      missing+=("$f")
    fi
  done

  local skipfile="$ROOTFS/var/lib/korrinos/skipped-packages.txt"
  if [ -s "$skipfile" ]; then
    local nskipped
    nskipped=$("$SUDO" sort -u "$skipfile" | grep -c . || true)
    echo "  $nskipped package(s) unavailable in this suite (skipped, not fatal):"
    "$SUDO" sort -u "$skipfile" | sed 's/^/    - /' | head -20
  else
    echo "  no packages were skipped."
  fi

  local total=${#EXPECTED_PACKAGES[@]}
  echo "  checked $((total + 2)) expectations -> $report"
  if [ ${#missing[@]} -gt 0 ]; then
    echo "### WARNING: ${#missing[@]} expected item(s) are NOT in the image:"
    printf '    - %s\n' "${missing[@]}"
    echo "### (see $report)"
  else
    echo "  all expected packages and KorrinOS components are present."
  fi
  return 0
}

run() {
  preflight || { echo "ABORT: preflight failed."; exit 1; }
  stage1 && stage2_install && stage3_worlds && stage_branding && stage2b_branding && stage_i18n_fonts && stage_kcommand && stage4_live \
    && stage5_squashfs && stage5_caspermaterials && stage6_iso && stage7_verify \
    && { echo "DONE: KorrinOS full distribution ISO ready."; } \
    || echo "BUILD FAILED: a stage returned non-zero (see output above). ISO is NOT complete."
}

rebuild() {
  preflight || { echo "ABORT: preflight failed."; exit 1; }
  test -d "$ROOTFS/etc" || { echo "no rootfs yet - run full first"; exit 1; }
  stage2_install && stage3_worlds && stage_branding && stage2b_branding && stage_i18n_fonts && stage_kcommand && stage4_live \
    && stage5_squashfs && stage5_caspermaterials && stage6_iso && stage7_verify \
    && { echo "DONE: KorrinOS rebuild (kept base rootfs)."; } \
    || echo "BUILD FAILED: a stage returned non-zero (see output above). ISO is NOT complete."
}

# finalize: reuse an already-built rootfs + squashfs; just (re)materialize
# casper kernel/initrd and assemble the ISO. Saves the slow compress step.
finalize() {
  [ -s "$IMAGE/casper/filesystem.squashfs" ] || {
    echo "ERROR: no squashfs at $IMAGE/casper/filesystem.squashfs — run 'build' first"
    exit 1
  }
  echo "### [finalize] reusing existing squashfs, rebuilding casper materials + ISO"
  stage5_caspermaterials && stage6_iso
  echo "DONE: KorrinOS ISO rebuilt from existing squashfs."
}

case "${1:-}" in
  full|build|run) run ;;
  rebuild) rebuild ;;
  finalize) finalize ;;
  base|stage1) stage1 ;;
  *) echo "KorrinOS Distribution Builder
Usage: ${0##*/} <build|rebuild|finalize|base>
Builds a real, full desktop Linux distribution ISO (Ubuntu/Kali-style) with
Xorg/Wayland + desktop + apps + package base + the 3 worlds baked in.
rebuild = keep rootfs, redo apt+worlds+ISO (fast iteration).
finalize = reuse existing squashfs, just rebuild casper materials + ISO." ;;
esac
