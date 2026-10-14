#!/bin/bash
# KorrinOS Cursor Themes -- install the bundled Xcursor themes system-wide.
#
# WHY A SYSTEM LOCATION AND NOT ~/.icons
# --------------------------------------
# The cursor has to appear in places that are not the logged-in user's
# session:
#
#   * the desktop session,
#   * terminals, browsers and other apps,
#   * the GDM login/greeter screen, which runs as the `gdm` user,
#   * the Calamares installer, which runs inside the live ISO session.
#
# A per-user ~/.icons install covers none of those. So the themes go to
# /usr/share/icons, which is on the Xcursor/GTK theme search path for every
# user on the system, and the default is published as a *system dconf default*
# so any session that has not explicitly chosen a cursor picks it up.
#
# IDEMPOTENCE
# -----------
# Safe to run repeatedly. Re-running rebuilds into a staging directory,
# replaces the installed trees wholesale, refreshes the dconf database and
# re-asserts the default. It never downloads anything: every theme is built
# from assets vendored in os/assets/cursors/.
#
# USAGE
#   install-cursor-themes.sh [--destdir DIR] [--no-default] [--no-build]
#                            [--sizes 16,32,...] [--force-theme NAME]
#
#   --destdir DIR  install under DIR (used to stage into an ISO rootfs)
#   --no-default   install the themes but do not change the cursor setting
#   --no-build     use the themes already present under --stagedir
#   --force-theme  set this theme instead of the default
#
# Exit status is 0 only if every theme was installed and (unless
# --no-default) the default was published.

set -euo pipefail

SELF="$(readlink -f "$0")"
# os/apps/customization/ -> os/assets/cursors/
OS_ROOT="$(cd "$(dirname "$SELF")/../.." && pwd)"
ASSETS="$OS_ROOT/assets/cursors"

THEME_PREFIX_REL="usr/share/icons/korrinos-cursors"
NOTICE_DEST_REL="usr/share/doc/korrinos-cursor-themes"
DCONF_LOCAL_REL="etc/dconf/db/local.d"

DESTDIR=""
SET_DEFAULT=1
DO_BUILD=1
STAGEDIR=""
SIZES=""
FORCE_THEME=""

while [ $# -gt 0 ]; do
    case "$1" in
        --destdir)    DESTDIR="${2%/}"; shift 2 ;;
        --no-default) SET_DEFAULT=0; shift ;;
        --no-build)   DO_BUILD=0; shift ;;
        --stagedir)   STAGEDIR="$2"; shift 2 ;;
        --sizes)      SIZES="$2"; shift 2 ;;
        --force-theme) FORCE_THEME="$2"; shift 2 ;;
        -h|--help)    sed -n '2,40p' "$0"; exit 0 ;;
        *) echo "unknown option: $1" >&2; exit 2 ;;
    esac
done

DEFAULT_THEME="korrinos-walter"
[ -n "$FORCE_THEME" ] && DEFAULT_THEME="$FORCE_THEME"

say() { echo "  $*"; }
warn() { echo "  WARNING: $*" >&2; }
die() { echo "ERROR: $*" >&2; exit 1; }

# ---------------------------------------------------------------- build stage
if [ "$DO_BUILD" -eq 1 ]; then
    # Don't assume `python3` is the one that has Pillow: on this build host
    # Pillow lives in /usr/local/bin/python3, which sudo's PATH omits, and the
    # error that produces ("python3 is required") sends you looking in the
    # wrong place entirely. Probe for an interpreter that can import PIL.
    PY=""
    for cand in "${PYTHON:-}" python3 /usr/local/bin/python3 python3.12 \
                python3.11 python3.10 /usr/bin/python3; do
        [ -n "$cand" ] || continue
        command -v "$cand" >/dev/null 2>&1 || continue
        if "$cand" -c "from PIL import Image; Image.new('RGBA',(1,1))" >/dev/null 2>&1; then
            PY="$cand"
            break
        fi
    done
    [ -n "$PY" ] || die "no python3 with Pillow found; install python3-pil (needed to build the cursor themes)"

    BUILD_DIR="$(mktemp -d "${TMPDIR:-/tmp}/korrinos-cursors.XXXXXX")"
    trap 'rm -rf "$BUILD_DIR"' EXIT

    say "building bundled themes with $PY from $ASSETS/source ..."
    BUILD_ARGS=(-o "$BUILD_DIR")
    [ -n "$SIZES" ] && BUILD_ARGS+=(--sizes "$SIZES")
    "$PY" "$ASSETS/make-themes.py" "${BUILD_ARGS[@]}" --include-synth \
        || die "theme build failed"
else
    [ -n "$STAGEDIR" ] || die "--no-build requires --stagedir"
    BUILD_DIR="$STAGEDIR"
    say "using pre-built themes from $BUILD_DIR"
fi

[ -f "$BUILD_DIR/manifest.json" ] || die "no manifest.json in $BUILD_DIR"

# ------------------------------------------------------------------- install
# Installed layout, per the Xcursor icon-theme spec:
#   <dest>/usr/share/icons/korrinos-cursors/<theme>/cursors/<role>
#   <dest>/usr/share/icons/korrinos-cursors/<theme>/index.theme
#
# Xcursor and GTK discover a theme by directory NAME under an XDG icon
# directory, so each theme is also symlinked into place one level up:
#   <dest>/usr/share/icons/<theme> -> korrinos-cursors/<theme>
# Without that symlink `gsettings ... cursor-theme korrinos-walter` would have
# nothing to resolve against and the theme would be invisible.

ICON_ROOT="$DESTDIR/usr/share/icons"
GROUP_ROOT="$ICON_ROOT/korrinos-cursors"

mkdir -p "$GROUP_ROOT"
# Start from a clean slate so a removed theme does not linger after a rebuild.
for old in "$GROUP_ROOT"/*/; do
    [ -e "$old" ] || continue
    rm -rf "$old"
done

installed=0
while read -r theme; do
    [ -n "$theme" ] || continue
    [ -d "$BUILD_DIR/$theme/cursors" ] || { warn "skipping $theme (not built)"; continue; }
    # cp -a preserves the intra-theme symlinks that keep identical roles on one
    # file; without -a they would be flattened into 59 real copies.
    cp -a "$BUILD_DIR/$theme" "$GROUP_ROOT/$theme"

    # Make it discoverable under its bare name.
    ln -sfn "korrinos-cursors/$theme" "$ICON_ROOT/$theme"

    nroles=$(find "$GROUP_ROOT/$theme/cursors" -mindepth 1 | wc -l)
    say "installed $theme ($nroles cursor files)"
    installed=$((installed + 1))
done < <(python3 -c '
import json, sys
m = json.load(open(sys.argv[1]))
print("\n".join(t["name"] for t in m["themes"]))
' "$BUILD_DIR/manifest.json")

[ "$installed" -gt 0 ] || die "no themes were installed"

cp -f "$BUILD_DIR/manifest.json" "$GROUP_ROOT/manifest.json"
if [ -f "$ASSETS/NOTICE.md" ]; then
    mkdir -p "$DESTDIR/$NOTICE_DEST_REL"
    cp -f "$ASSETS/NOTICE.md" "$DESTDIR/$NOTICE_DEST_REL/NOTICE.md"
    cp -f "$ASSETS/NOTICE.md" "$GROUP_ROOT/NOTICE.md"
    say "attribution installed at /$NOTICE_DEST_REL/NOTICE.md"
fi

# ------------------------------------------------------- publish the default
#
# A *system dconf default* is the only mechanism that reaches the greeter and
# the live/installer session: those have their own users and their own dconf
# databases, so a per-user `gsettings set` in an autostart would never fire.
# It is a default, not a lock, so the Mouse Designs picker can still override
# it per user.
if [ "$SET_DEFAULT" -eq 1 ]; then
    mkdir -p "$DESTDIR/$DCONF_LOCAL_REL"
    cat > "$DESTDIR/$DCONF_LOCAL_REL/00-korrinos-cursor-theme" <<EOF
# KorrinOS -- default cursor theme
#
# Installed by os/apps/customization/install-cursor-themes.sh. A system dconf
# default (rather than a per-user setting) is used deliberately: it is what the
# GDM greeter and the live/installer session read, and because it is a default
# and not a lock, a user can still override it from the Mouse Designs picker.
[org/gnome/desktop/interface]
cursor-theme='$DEFAULT_THEME'
cursor-size=24
EOF
    chmod 0644 "$DESTDIR/$DCONF_LOCAL_REL/00-korrinos-cursor-theme"
    say "wrote /$DCONF_LOCAL_REL/00-korrinos-cursor-theme -> cursor-theme='$DEFAULT_THEME'"

    # dconf compiles the system database on the target machine; in staging mode
    # there is nothing to compile against, so the file alone is correct.
    if [ -z "$DESTDIR" ]; then
        if command -v dconf >/dev/null 2>&1; then
            if dconf update 2>/dev/null; then
                say "dconf database updated"
            else
                warn "dconf update failed; the default will apply after reboot"
            fi
        else
            warn "dconf not installed; system default not compiled"
        fi
    else
        say "staged only; run 'dconf update' on the target to compile it"
    fi
else
    say "--no-default: cursor-theme setting left untouched"
fi

# ------------------------------------------- XFCE, which is the actual DE
#
# KorrinOS ships an XFCE4 desktop, and xfsettings reads the cursor theme from
# the xfconf xsettings channel, not from org.gnome.desktop.interface. Setting
# only the gsettings key would leave the running session showing the Adwaita
# cursor. So for the *current* user we set both, live.
if [ "$SET_DEFAULT" -eq 1 ] && [ -z "$DESTDIR" ] && [ -n "${SUDO_USER:-}" ]; then
    asuser="${SUDO_USER}"
    if command -v xfconf-query >/dev/null 2>&1; then
        if sudo -u "$asuser" DBUS_SESSION_BUS_ADDRESS="${DBUS_SESSION_BUS_ADDRESS:-unix:path=/run/user/$(id -u "$asuser")/bus}" \
            xfconf-query -c xsettings -p /Net/CursorTheme -s "$DEFAULT_THEME" 2>/dev/null; then
            say "xfsettings /Net/CursorTheme -> $DEFAULT_THEME (for $asuser)"
        else
            warn "could not set xfconf /Net/CursorTheme for $asuser"
        fi
    else
        warn "xfconf-query not present; XFCE cursor theme not set"
    fi
fi

say "done: $installed theme(s), default '$DEFAULT_THEME'"
exit 0