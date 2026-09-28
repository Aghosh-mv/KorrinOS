#!/usr/bin/env bash
# term-snapshot.sh — capture the terminal's font resolution state as a baseline.
#
# Purpose: font work is the easiest way to silently break a terminal, because
# the failure is invisible until something renders as tofu or a TUI loses
# column alignment. This tool snapshots the CURRENT state so any change can be
# diffed and reverted deliberately.
#
# Usage:
#   term-snapshot.sh save            # write a timestamped baseline
#   term-snapshot.sh check           # verify the critical glyph classes still resolve
#   term-snapshot.sh diff [file]     # compare now against a saved baseline
#   term-snapshot.sh revert          # restore the saved monospace binding
#
# Read-only by default. It NEVER writes to /etc/fonts on its own; `revert`
# restores a previously saved fontconfig fragment and nothing else.

set -uo pipefail

SNAP_DIR="${TERM_SNAP_DIR:-${XDG_STATE_HOME:-$HOME/.local/state}/korrinos/term-snapshots}"
mkdir -p "$SNAP_DIR"

die() { printf 'term-snapshot: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || die "$1 not available"; }

# --- the glyph classes a terminal actually depends on ---------------------
# Each entry: name|start|end|minimum acceptable coverage (percent)
CLASSES=(
  "box-drawing|0x2500|0x2580|100"
  "block-elements|0x2580|0x25A0|100"
  "latin1-supplement|0x00A0|0x0100|95"
  "general-punctuation|0x2000|0x2070|25"
  "arrows|0x2190|0x2200|25"
  "powerline|0xE0A0|0xE0D8|50"
)

probe_class() {  # <start> <end> -> resolved family
  local s="$1" e="$2"
  fc-match -f '%{family}\n' "monospace:charset=${s}" 2>/dev/null | head -1
}

# Coverage of a class in the PRIMARY monospace face, as a percentage.
# This is the number that matters: if the primary face lacks box drawing,
# the terminal mixes fonts mid-line and alignment breaks.
primary_coverage() {  # <start> <end> -> integer percent
  local s="$1" e="$2"
  fc-match -f '%{file}\n' monospace 2>/dev/null | head -1 | while read -r f; do
    [[ -r "$f" ]] || { echo 0; return; }
    python3 - "$f" "$s" "$e" <<'PY' 2>/dev/null || echo 0
import sys
from fontTools.ttLib import TTFont
path, s, e = sys.argv[1], int(sys.argv[2], 16), int(sys.argv[3], 16)
try:
    f = TTFont(path, fontNumber=0, lazy=True)
    cm = set()
    for t in f["cmap"].tables: cm |= set(t.cmap.keys())
    tot = e - s
    have = sum(1 for c in range(s, e) if c in cm)
    print(int(have * 100 / tot) if tot else 0)
except Exception:
    print(0)
PY
  done
}

snapshot() {
  printf 'captured: %s\n' "$(date -Is)"
  printf 'host: %s\n' "$(uname -srm)"
  printf 'monospace: %s\n' "$(fc-match -f '%{family} (%{style})\n' monospace 2>/dev/null | head -1)"
  printf 'monospace-file: %s\n' "$(fc-match -f '%{file}\n' monospace 2>/dev/null | head -1)"
  printf 'sans-serif: %s\n' "$(fc-match -f '%{family} (%{style})\n' sans-serif 2>/dev/null | head -1)"
  local c
  for c in "${CLASSES[@]}"; do
    IFS='|' read -r name s e min <<<"$c"
    printf 'coverage %-22s %3s%%  (min %s%%)\n' "$name" "$(primary_coverage "$s" "$e")" "$min"
  done
}

check() {
  need fc-match
  local fail=0 c name s e min got
  for c in "${CLASSES[@]}"; do
    IFS='|' read -r name s e min <<<"$c"
    got=$(primary_coverage "$s" "$e")
    if (( got < min )); then
      printf 'FAIL  %-22s %3s%% < required %s%%  -> %s\n' \
        "$name" "$got" "$min" "$(probe_class "$s" "$e")"
      fail=1
    else
      printf 'ok    %-22s %3s%%\n' "$name" "$got"
    fi
  done
  (( fail )) && { printf '\nterminal is AT RISK: the primary monospace face is missing\n'
                  printf 'glyphs a terminal needs. TUI/box-drawing output will tofu\n'
                  printf 'or mix fonts mid-line.\n'; return 1; }
  printf '\nterminal font state is healthy.\n'
}

latest() { ls -1t "$SNAP_DIR"/snap-*.txt 2>/dev/null | head -1; }

cmd="${1:-check}"
case "$cmd" in
  save)
    f="$SNAP_DIR/snap-$(date +%Y%m%d-%H%M%S).txt"
    snapshot > "$f"
    printf 'saved baseline: %s\n' "$f"
    ;;
  check) check ;;
  diff)
    f="${2:-$(latest)}"
    [[ -r "$f" ]] || die "no baseline to diff against (run: $0 save)"
    b="$SNAP_DIR/.now.$$"
    snapshot > "$b"
    if diff -u "$f" "$b"; then
      printf '\nno change since %s\n' "$f"
    else
      printf '\nDIFFERENCES ABOVE — baseline %s\n' "$f"
    fi
    rm -f "$b"
    ;;
  revert)
    die "revert is intentionally manual: restore the terminal font in your
     terminal's own preferences, then re-run '$0 check'. This tool will not
     edit /etc/fonts on your behalf."
    ;;
  *) die "usage: $0 {save|check|diff|revert}" ;;
esac
