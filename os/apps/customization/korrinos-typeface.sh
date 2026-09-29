#!/usr/bin/env bash
# ==============================================================================
#  korrinos-font — KorrinOS system-wide typeface control
# ------------------------------------------------------------------------------
#  Default typeface: Balsamiq Sans, rendered with a SLIGHT synthetic slant so
#  the whole desktop reads with one consistent, gently italicised voice.
#
#  Balsamiq Sans is licensed under the SIL Open Font License 1.1 (see
#  assets/fonts/OFL.txt) and is bundled with KorrinOS, so this works offline.
#
#  The "slight italic" is applied with a fontconfig `matrix` shear rather than
#  by switching to the real Italic face: a full italic is too aggressive for
#  body text at 10-11pt, whereas ~6 degrees reads as "slightly slanted" and
#  stays comfortable for long stretches of UI text.
#
#  Usage:
#    korrinos-font on       install fonts + apply as system default
#    korrinos-font off      restore the previous system default
#    korrinos-font toggle   flip on <-> off
#    korrinos-font status   show what is currently applied
#    korrinos-font preview  render a sample card in the current font
#    korrinos-font uninstall  remove fonts and the fontconfig override
# ==============================================================================
set -euo pipefail

FONT_FAMILY="Balsamiq Sans"
SLANT_DEG="${KORRINOS_FONT_SLANT:-6}"     # slight italic
FONT_SRC_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../assets/fonts" && pwd)"

# Where we install, in order of preference.
SYSTEM_FONT_DIR="/usr/share/fonts/korrinos"
LOCAL_FONT_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/fonts/korrinos"

# fontconfig drop-in directory.
CONF_DIR_SYSTEM="/etc/fonts/conf.d"
CONF_DIR_LOCAL="${XDG_CONFIG_HOME:-$HOME/.config}/fontconfig/conf.d"
CONF_NAME="61-korrinos-font.conf"
STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/korrinos"
STATE_FILE="$STATE_DIR/font-state"

have() { command -v "$1" >/dev/null 2>&1; }
log()  { printf '  %s\n' "$*"; }
ok()   { printf '\033[1;32m%s\033[0m\n' "$*"; }
warn() { printf '\033[1;33m%s\033[0m\n' "$*" >&2; }
die()  { printf '\033[1;31m%s\033[0m\n' "$*" >&2; exit 1; }

# Pick a writable font dir + conf dir. Returns "system" or "local".
target_scope() {
  if [ -w /usr/share/fonts ] && [ -w "$CONF_DIR_SYSTEM" ] 2>/dev/null; then
    echo system
  else
    echo local
  fi
}

font_dir_for()  { [ "$1" = system ] && echo "$SYSTEM_FONT_DIR"  || echo "$LOCAL_FONT_DIR"; }
conf_dir_for()  { [ "$1" = system ] && echo "$CONF_DIR_SYSTEM" || echo "$CONF_DIR_LOCAL"; }

# ---------------------------------------------------------------------------
#  install the bundled font files
# ---------------------------------------------------------------------------
install_fonts() {
  local scope dest
  scope=$(target_scope)
  dest=$(font_dir_for "$scope")

  [ -d "$FONT_SRC_DIR" ] || die "bundled font directory missing: $FONT_SRC_DIR"
  local found=0
  local f
  for f in "$FONT_SRC_DIR"/BalsamiqSans-*.ttf; do
    [ -f "$f" ] || continue
    found=1
    if [ -w "$dest" ] 2>/dev/null || install -d "$dest" 2>/dev/null; then
      cp -f "$f" "$dest/" 2>/dev/null || sudo -n cp -f "$f" "$dest/" 2>/dev/null \
        || die "cannot write fonts into $dest"
    else
      die "cannot create $dest"
    fi
    log "installed $(basename "$f")"
  done
  [ "$found" -eq 1 ] || die "no BalsamiqSans-*.ttf found in $FONT_SRC_DIR"

  have fc-cache && fc-cache -f >/dev/null 2>&1 && log "font cache rebuilt" || true
  echo "$dest"
}

# ---------------------------------------------------------------------------
#  the fontconfig override that makes it the default, with a slight slant
# ---------------------------------------------------------------------------
write_conf() {
  local scope cdir file slant
  scope=$(target_scope)
  cdir=$(conf_dir_for "$scope")
  file="$cdir/$CONF_NAME"
  slant="${SLANT_DEG}"

  if ! mkdir -p "$cdir" 2>/dev/null; then
    sudo -n mkdir -p "$cdir" 2>/dev/null || die "cannot create $cdir"
  fi

  # cos/sin of the slant angle, computed once so the matrix stays readable.

  # A fontconfig <matrix> holds exactly FOUR <double> values describing a 2x2
  # affine transform:  <double>a</double> <double>b</double>
  #                     <double>c</double> <double>d</double>
  # i.e.  x' = a*x + b*y ,  y' = c*x + d*y
  # A slight italic is a shear, so a=1, b=0, c=tan(theta), d=1.
  # Supplying 16 <name> values instead makes fontconfig log
  # "wrong number of matrix elements" and silently drop the whole edit.
  local tmp; tmp=$(mktemp)
  local tanv
  tanv=$(awk -v d="$slant" 'BEGIN{printf "%.6f", (d*3.14159265358979/180)}' 2>/dev/null)
  tanv=$(awk -v d="$slant" 'BEGIN{printf "%.6f", sin(d*3.14159265358979/180)/cos(d*3.14159265358979/180)}')
  cat > "$tmp" <<EOF
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "urn:fontconfig:fonts.dtd">
<!-- KorrinOS default typeface: $FONT_FAMILY with a ${slant}deg slight slant.
     Installed by korrinos-font. Remove this file to revert. -->
<fontconfig>

  <!-- Direct matches: any request for the family resolves to our copy. -->
  <match target="pattern">
    <test name="family"><string>$FONT_FAMILY</string></test>
    <edit name="family" mode="assign" binding="strong"><string>$FONT_FAMILY</string></edit>
  </match>

  <!-- Make it the default for the generic interface families. -->
  <alias><family>sans-serif</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>serif</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>system-ui</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>ui-sans-serif</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>desktop</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>monospace</family><prefer><family>$FONT_FAMILY</family></prefer></alias>
  <alias><family>Monospace</family><prefer><family>$FONT_FAMILY</family></prefer></alias>

  <!-- Slight italic: shear the glyph outlines instead of using the real
       Italic face, which is far too strong for body text.
       2x2 shear matrix: 1 0 / tan(6deg) 1 -->
  <match target="font">
    <test name="family"><string>$FONT_FAMILY</string></test>
    <edit name="matrix" mode="assign">
      <matrix>
        <double>1</double>
        <double>0</double>
        <double>$tanv</double>
        <double>1</double>
      </matrix>
    </edit>
  </match>

</fontconfig>
EOF

  if [ -w "$cdir" ] 2>/dev/null; then
    cp -f "$tmp" "$file"
  else
    sudo -n cp -f "$tmp" "$file" 2>/dev/null || { rm -f "$tmp"; die "cannot write $file"; }
  fi
  rm -f "$tmp"
  have fc-cache && fc-cache -f >/dev/null 2>&1 || true
  echo "$file"
}

conf_path() {
  local scope; scope=$(target_scope)
  echo "$(conf_dir_for "$scope")/$CONF_NAME"
}

# ---------------------------------------------------------------------------
#  state
# ---------------------------------------------------------------------------
load_state() { [ -f "$STATE_FILE" ] && cat "$STATE_FILE" || echo off; }
save_state() { mkdir -p "$STATE_DIR"; printf '%s\n' "$1" > "$STATE_FILE"; }

# ---------------------------------------------------------------------------
#  actions
# ---------------------------------------------------------------------------
do_on() {
  log "Installing $FONT_FAMILY ..."
  install_fonts >/dev/null
  log "Applying as system default (${SLANT_DEG}deg slight slant) ..."
  local c; c=$(write_conf)
  log "wrote $c"
  save_state on
  ok "$FONT_FAMILY is now the KorrinOS default typeface (slight italic)."
  log "Open a new app window, or log out/in, for every window to pick it up."
  log "Revert any time with: korrinos-font off"
}

do_off() {
  local c; c=$(conf_path)
  if [ -f "$c" ]; then
    if [ -w "$(dirname "$c")" ] 2>/dev/null; then rm -f "$c"; else sudo -n rm -f "$c" 2>/dev/null || true; fi
    log "removed $c"
  else
    log "no override present (already off)"
  fi
  have fc-cache && fc-cache -f >/dev/null 2>&1 || true
  save_state off
  ok "Restored the previous system default typeface."
}

do_toggle() {
  if [ "$(load_state)" = on ] || [ -f "$(conf_path)" ]; then do_off; else do_on; fi
}

do_status() {
  local st; st=$(load_state)
  local c; c=$(conf_path)
  printf 'KorrinOS typeface status\n'
  printf '  %-22s %s\n' "default family:" "$FONT_FAMILY"
  printf '  %-22s %s\n' "slant:" "${SLANT_DEG}deg (slight italic)"
  printf '  %-22s %s\n' "state:" "$st"
  printf '  %-22s %s\n' "fontconfig override:" "$([ -f "$c" ] && echo present || echo absent)"
  # NOTE: this must be a real mktemp. A predictable path such as /tmp/.kfc.$$
  # is a symlink-attack vector: another local user can pre-create that name as
  # a symlink and make the script (running as this user) clobber their target.
  # Buffering through a pipe into grep -q is NOT an option either, because
  # `grep -q` exits on the first match, fc-list takes SIGPIPE, and under
  # `set -o pipefail` that turns a successful match into a failure.
  if have fc-list; then
    local _fclist
    if _fclist=$(mktemp); then
      if fc-list > "$_fclist" 2>/dev/null && grep -qi "Balsamiq" "$_fclist"; then
        st=installed
      else
        st="not installed"
      fi
      rm -f "$_fclist"
    else
      st="unknown (mktemp failed)"
    fi
  else
    st="unknown (fontconfig not available)"
  fi
  printf '  %-22s %s\n' "font files:" "$st"
  if have fc-match; then
    printf '  %-22s %s\n' "fc-match sans-serif:" "$(fc-match sans-serif 2>/dev/null)"
  fi
}

do_preview() {
  if ! have fc-match; then
    warn "fontconfig not available; cannot render a preview."
    return 0
  fi
  printf '\n'
  printf '  The quick brown fox jumps over the lazy dog. 0123456789\n'
  printf ' abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ\n'
  printf '  KorrinOS — Balsamiq Sans, slightly italic.\n'
  printf '\n'
  printf '  resolved family: %s\n' "$(fc-match sans-serif 2>/dev/null)"
  printf '  target family:   %s\n' "$FONT_FAMILY"
  printf '\n'
  log "To see it for real, open a text editor or run:"
  log "  fc-match sans-serif   # should name $FONT_FAMILY"
}

do_uninstall() {
  do_off
  for d in "$SYSTEM_FONT_DIR" "$LOCAL_FONT_DIR"; do
    if [ -d "$d" ]; then
      if [ -w "$d" ]; then rm -rf "$d"; else sudo -n rm -rf "$d" 2>/dev/null || true; fi
      log "removed $d"
    fi
  done
  have fc-cache && fc-cache -f >/dev/null 2>&1 || true
  ok "Balsamiq Sans removed from the system."
}

usage() {
  sed -n '2,20p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

case "${1:-status}" in
  on|enable)        do_on ;;
  off|disable)      do_off ;;
  toggle)           do_toggle ;;
  status)           do_status ;;
  preview|sample)   do_preview ;;
  uninstall)        do_uninstall ;;
  help|-h|--help)   usage ;;
  *)                usage; exit 1 ;;
esac
