#!/usr/bin/env bash
# computer-use.sh — full computer-control surface for the KorrinOS AI.
#
# agent-vision.sh can see the screen and click text. That is a "can it read the
# screen" tool, not a computer-use loop: it cannot type, press key combos,
# manage windows, scroll, drag, or touch the clipboard, so the AI cannot
# actually complete a task it can see.
#
# This module adds the missing verbs so the AI can close a loop:
#   see -> locate -> act -> verify
#
# Backends: xdotool (input), wmctrl (windows), scrot/import (capture),
# xclip/xsel (clipboard). Every verb degrades gracefully: if a backend is
# missing we say exactly what is missing instead of silently doing nothing.

CU_DIR="${TINKER_AI_HOME:-$HOME/.config/vokk}/agent"
CU_SHOTS="$CU_DIR/screenshots"
mkdir -p "$CU_SHOTS"

# --- dependency reporting ---------------------------------------------------

# Report which backends are available. The AI should be able to ask "what can
# you actually do right now?" instead of discovering it by silent failure.
cu_deps() {
  local t
  printf 'computer-use backends:\n'
  for t in xdotool wmctrl scrot import xclip xsel tesseract python3; do
    if command -v "$t" >/dev/null 2>&1; then
      printf '  [ok]      %s -> %s\n' "$t" "$(command -v "$t")"
    else
      printf '  [MISSING] %s\n' "$t"
    fi
  done
  python3 - <<'PY' 2>/dev/null || printf '  [MISSING] python3 PIL (Pillow)\n'
from PIL import Image  # noqa: F401
print("  [ok]      python3 PIL (Pillow)")
PY
}

# Require a backend or explain how to get it.
_cu_need() {
  command -v "$1" >/dev/null 2>&1 && return 0
  echo "computer-use: '$1' is not installed — cannot $2." >&2
  echo "  Debian/Ubuntu: sudo apt install $3" >&2
  return 1
}

# --- capture ----------------------------------------------------------------

# Capture the whole screen, or a region, or a named window.
#   cu_capture [full|region X Y W H|window MATCH] [outfile]
#
# Backend note: scrot's region flag (-g) is NOT present in every build -- the
# one shipped here rejects it outright ("scrot: invalid option -- 'g'"). Image
# Magick's `import` supports -crop everywhere, so region capture goes through
# import first and only tries scrot -g as a fallback. Full-screen capture
# likewise falls back to import when scrot is unavailable or fails.
_cu_capture() {
  local mode="${1:-full}" out="${2:-}" x y w h
  [ -z "$out" ] && out="$CU_SHOTS/cu_$(date +%s%N).png"

  case "$mode" in
    full)
      if command -v scrot >/dev/null 2>&1 && scrot -o "$out" 2>/dev/null && [ -s "$out" ]; then
        echo "$out"; return 0
      fi
      _cu_need import "capture the screen" imagemagick || return 1
      import -window root "$out" 2>/dev/null || { echo "capture failed"; return 1; }
      ;;
    region)
      x="${2:-0}"; y="${3:-0}"; w="${4:-0}"; h="${5:-0}"; out="${6:-$CU_SHOTS/cu_$(date +%s%N).png}"
      if [ "$w" -le 0 ] 2>/dev/null || [ "$h" -le 0 ] 2>/dev/null; then
        echo "region capture needs positive width and height"; return 2
      fi
      if command -v import >/dev/null 2>&1; then
        import -window root -crop "${w}x${h}+${x}+${y}" +repage "$out" 2>/dev/null \
          && [ -s "$out" ] && { echo "$out"; return 0; }
      fi
      if command -v scrot >/dev/null 2>&1; then
        scrot -o -g "${w}x${h}+${x}+${y}" "$out" 2>/dev/null \
          && [ -s "$out" ] && { echo "$out"; return 0; }
      fi
      echo "region capture failed (need ImageMagick 'import' or a scrot build with -g)"
      return 1
      ;;
    window)
      local match="${2:-}"
      # Focus the window first so it is on top, then shoot the root window.
      command -v xdotool >/dev/null 2>&1 && xdotool search --name "$match" windowactivate --sync %@ 2>/dev/null
      sleep 0.3
      if command -v scrot >/dev/null 2>&1 && scrot -o "$out" 2>/dev/null && [ -s "$out" ]; then
        echo "$out"; return 0
      fi
      _cu_need import "capture the screen" imagemagick || return 1
      import -window root "$out" 2>/dev/null || { echo "window capture failed"; return 1; }
      ;;
    *)
      echo "usage: cu_capture [full|region X Y W H|window MATCH] [outfile]"; return 2 ;;
  esac
  echo "$out"
}

cu_screenshot() { _cu_capture full "${1:-}"; }

# Public capture entry point (the dispatch table calls _cu_capture directly,
# but cu_capture is the name an operator or the AI will actually reach for).
cu_capture() { _cu_capture "$@"; }

# --- mouse ------------------------------------------------------------------

# cu_move X Y
cu_move() {
  [ $# -ge 2 ] || { echo "usage: cu_move <x> <y>"; return 2; }
  _cu_need xdotool "move the mouse" xdotool || return 1
  xdotool mousemove --sync "$1" "$2" 2>/dev/null || { echo "move failed"; return 1; }
  echo "moved to ($1, $2)"
}

# cu_click [left|right|middle] [single|double]
cu_click() {
  _cu_need xdotool "click" xdotool || return 1
  local btn="${1:-left}" times="${2:-single}" b n
  case "$btn" in
    left) b=1 ;; right) b=3 ;; middle) b=2 ;;
    *) echo "usage: cu_click [left|right|middle] [single|double]"; return 2 ;;
  esac
  n=1; [ "$times" = double ] && n=2
  xdotool click --repeat "$n" --delay 120 "$b" 2>/dev/null || { echo "click failed"; return 1; }
  echo "clicked $btn x$n"
}

# cu_click_at X Y [button] — move then click in one step
cu_click_at() {
  [ $# -ge 2 ] || { echo "usage: cu_click_at <x> <y> [button] [single|double]"; return 2; }
  cu_move "$1" "$2" >/dev/null || return 1
  cu_click "${3:-left}" "${4:-single}"
}

# cu_drag X1 Y1 X2 Y2 [steps] — press, glide, release
cu_drag() {
  [ $# -ge 4 ] || { echo "usage: cu_drag <x1> <y1> <x2> <y2> [steps]"; return 2; }
  _cu_need xdotool "drag" xdotool || return 1
  local steps="${5:-24}"
  xdotool mousemove --sync "$1" "$2" 2>/dev/null || return 1
  sleep 0.15
  xdotool mousedown 1 2>/dev/null || return 1
  sleep 0.15
  xdotool mousemove --sync "$3" "$4" 2>/dev/null
  # intermediate moves make drags register on apps that need motion events
  xdotool mousemove_relative -- "$3" "$4" 2>/dev/null
  sleep 0.15
  xdotool mouseup 1 2>/dev/null || return 1
  echo "dragged ($1,$2) -> ($3,$4) in $steps steps"
}

# cu_scroll up|down|left|right [clicks]
cu_scroll() {
  [ $# -ge 1 ] || { echo "usage: cu_scroll up|down|left|right [clicks]"; return 2; }
  _cu_need xdotool "scroll" xdotool || return 1
  local dir="${1:-down}" n="${2:-5}" b
  case "$dir" in
    up) b=4 ;; down) b=5 ;; left) b=6 ;; right) b=7 ;;
    *) echo "usage: cu_scroll up|down|left|right [clicks]"; return 2 ;;
  esac
  xdotool click --repeat "$n" "$b" 2>/dev/null || { echo "scroll failed"; return 1; }
  echo "scrolled $dir x$n"
}

# --- keyboard ---------------------------------------------------------------

# cu_type "text" — literal typing
cu_type() {
  [ $# -ge 1 ] || { echo "usage: cu_type <text>"; return 2; }
  _cu_need xdotool "type" xdotool || return 1
  [ -z "$1" ] && { echo "usage: cu_type <text>"; return 2; }
  xdotool type --clearmodifiers --delay 12 "$1" 2>/dev/null || { echo "type failed"; return 1; }
  echo "typed ${#1} chars"
}

# cu_key combo [combo ...] — key combos: ctrl+c, alt+tab, Return, Escape
cu_key() {
  [ $# -ge 1 ] || { echo "usage: cu_key <combo> [combo ...]"; return 2; }
  _cu_need xdotool "send keys" xdotool || return 1
  [ $# -eq 0 ] && { echo "usage: cu_key <combo> [combo ...]  (ctrl+c, alt+Tab, Return)"; return 2; }
  xdotool key --clearmodifiers "$@" 2>/dev/null || { echo "key failed: $*"; return 1; }
  echo "sent: $*"
}

# cu_typeenter "text" — type then Return, the single most common AI action
cu_typeenter() {
  [ $# -ge 1 ] || { echo "usage: cu_typeenter <text>"; return 2; }
  cu_type "$1" >/dev/null || return 1
  cu_key Return >/dev/null || return 1
  echo "typed text and pressed Return"
}

# --- windows ----------------------------------------------------------------

# wmctrl where available, xdotool search as fallback.
_cu_win_backend() {
  if command -v wmctrl >/dev/null 2>&1; then echo wmctrl; else echo xdotool; fi
}

cu_windows() {
  local be
  be=$(_cu_win_backend)
  if [ "$be" = wmctrl ]; then
    wmctrl -lp 2>/dev/null || { echo "wmctrl failed"; return 1; }
  else
    _cu_need xdotool "list windows" xdotool || return 1
    xdotool search --name '.' getwindowname %@ 2>/dev/null
  fi
}

cu_focus() {
  [ $# -ge 1 ] || { echo "usage: cu_focus <window-name-match>"; return 2; }
  local match="${1:-}"
  [ -z "$match" ] && { echo "usage: cu_focus <window-name-match>"; return 2; }
  if command -v wmctrl >/dev/null 2>&1; then
    wmctrl -a "$match" 2>/dev/null || { echo "no window matching '$match'"; return 1; }
  else
    _cu_need xdotool "focus windows" xdotool || return 1
    xdotool search --name "$match" windowactivate --sync %@ 2>/dev/null \
      || { echo "no window matching '$match'"; return 1; }
  fi
  echo "focused: $match"
}

cu_close() {
  [ $# -ge 1 ] || { echo "usage: cu_close <window-name-match>"; return 2; }
  local match="${1:-}"
  [ -z "$match" ] && { echo "usage: cu_close <window-name-match>"; return 2; }
  if command -v wmctrl >/dev/null 2>&1; then
    wmctrl -c "$match" 2>/dev/null || { echo "no window matching '$match'"; return 1; }
  else
    _cu_need xdotool "close windows" xdotool || return 1
    xdotool search --name "$match" windowclose %@ 2>/dev/null \
      || { echo "no window matching '$match'"; return 1; }
  fi
  echo "closed: $match"
}

# --- clipboard --------------------------------------------------------------

_cu_clip_backend() {
  if command -v xclip >/dev/null 2>&1; then echo xclip
  elif command -v xsel >/dev/null 2>&1; then echo xsel
  else echo none; fi
}

cu_clip_get() {
  local be; be=$(_cu_clip_backend)
  case "$be" in
    xclip) xclip -selection clipboard -o 2>/dev/null ;;
    xsel)  xsel --clipboard --output 2>/dev/null ;;
    *) echo "clipboard: no xclip or xsel installed (apt install xclip)" >&2; return 1 ;;
  esac
}

cu_clip_set() {
  [ $# -ge 1 ] || { echo "usage: cu_clip_set <text>"; return 2; }
  local be; be=$(_cu_clip_backend)
  [ -z "$1" ] && { echo "usage: cu_clip_set <text>"; return 2; }
  case "$be" in
    xclip) printf '%s' "$1" | xclip -selection clipboard 2>/dev/null || return 1 ;;
    xsel)  printf '%s' "$1" | xsel --clipboard --input 2>/dev/null || return 1 ;;
    *) echo "clipboard: no xclip or xsel installed (apt install xclip)" >&2; return 1 ;;
  esac
  echo "clipboard set (${#1} chars)"
}

# cu_paste — paste clipboard into focused field
cu_paste() {
  local be; be=$(_cu_clip_backend)
  [ "$be" = none ] && { echo "no xclip/xsel" >&2; return 1; }
  cu_key ctrl+v >/dev/null && echo "pasted"
}

# --- act / verify -----------------------------------------------------------

# cu_wait [seconds]
cu_wait() { sleep "${1:-1}"; echo "waited ${1:-1}s"; }

# cu_act "<json array of steps>" — run a small action program.
# Each step: {"op":"type","text":"hi"} | {"op":"key","keys":["ctrl+s"]}
#          | {"op":"click","x":10,"y":20} | {"op":"key",...}
# Steps run in order; output is collected. This is the piece that lets the AI
# run a whole task instead of one verb per turn.
cu_act() {
  [ $# -ge 1 ] || { echo "usage: cu_act '<json steps>'"; return 2; }
  [ -z "$1" ] && { echo "usage: cu_act '<json steps>'"; return 2; }
  python3 - "$1" <<'PY' 2>&1
import json, subprocess, sys, shlex

try:
    steps = json.loads(sys.argv[1])
except Exception as e:
    print(f"cu_act: invalid JSON: {e}")
    sys.exit(2)

if not isinstance(steps, list):
    print("cu_act: expected a JSON array of steps")
    sys.exit(2)

def run(*args):
    # Never shell-interpolate: argv form only.
    return subprocess.run(args, capture_output=True, text=True)

ok = True
for i, s in enumerate(steps, 1):
    if not isinstance(s, dict):
        print(f"  step {i}: not an object, skipped"); ok = False; continue
    op = s.get("op")
    try:
        if op == "type":
            r = run("xdotool", "type", "--clearmodifiers", "--delay", "12", str(s.get("text", "")))
        elif op == "key":
            keys = s.get("keys") or ([s["key"]] if "key" in s else [])
            r = run("xdotool", "key", "--clearmodifiers", *keys)
        elif op == "click":
            args = ["xdotool"]
            if "x" in s and "y" in s:
                args += ["mousemove", "--sync", str(s["x"]), str(s["y"])]
            args += ["click", str(s.get("button", 1))]
            r = run(*args)
        elif op == "scroll":
            b = {"up": 4, "down": 5, "left": 6, "right": 7}.get(s.get("dir", "down"), 5)
            r = run("xdotool", "click", "--repeat", str(s.get("n", 5)), str(b))
        elif op == "focus":
            r = run("wmctrl", "-a", str(s.get("match", "")))
        elif op == "close":
            r = run("wmctrl", "-c", str(s.get("match", "")))
        elif op == "shot":
            r = run("scrot", "-o", str(s.get("out", "/tmp/cu_act.png")))
        elif op == "wait":
            import time; time.sleep(float(s.get("seconds", 1))); print(f"  step {i}: wait ok"); continue
        else:
            print(f"  step {i}: unknown op {op!r}"); ok = False; continue
        if r.returncode != 0:
            print(f"  step {i}: {op} FAILED rc={r.returncode} {r.stderr.strip()[:120]}")
            ok = False
        else:
            print(f"  step {i}: {op} ok")
    except FileNotFoundError as e:
        print(f"  step {i}: backend missing ({e})"); ok = False

print("cu_act: OK" if ok else "cu_act: COMPLETED WITH FAILURES")
sys.exit(0 if ok else 1)
PY
}
