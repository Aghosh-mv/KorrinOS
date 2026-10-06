#!/usr/bin/env bash
# agent-vision.sh — computer vision: read screen, find elements, visual understanding

AGENT_DIR="${TINKER_AI_HOME:-$HOME/.config/vokk}/agent"
mkdir -p "$AGENT_DIR/screenshots"

# Take screenshot and read all text
agent_vision_read() {
  local screenshot="${1:-}"
  if [ -z "$screenshot" ]; then
    screenshot="$AGENT_DIR/screenshots/vision_$(date +%s).png"
    scrot -o "$screenshot" 2>/dev/null
  fi
  
  if [ ! -f "$screenshot" ]; then
    echo "Screenshot failed"; return 1
  fi
  
  # Get image info (path passed via env, not interpolated into the source)
  local info
  info=$(CU_SHOT="$screenshot" python3 <<'PY' 2>/dev/null
import os
from PIL import Image
img = Image.open(os.environ["CU_SHOT"])
print(f'Size: {img.size[0]}x{img.size[1]}')
PY
)
  
  # OCR all text
  local text
  text=$(tesseract "$screenshot" - 2>/dev/null)
  
  echo "=== Screen Analysis ==="
  echo "$info"
  echo ""
  echo "Text on screen:"
  echo "$text"
}

# Find where something is on screen (returns coordinates)
# NOTE: the path and search target are passed via the environment, not
# interpolated into the Python source. The previous version spliced them into
# single-quoted string literals, so any target containing a quote broke the
# script and a crafted target could inject arbitrary Python.
agent_vision_find() {
  local target="$1"
  if [ -z "$target" ]; then
    echo "usage: agent_vision_find <text>"; return 2
  fi
  local screenshot="$AGENT_DIR/screenshots/find_$(date +%s%N).png"
  scrot -o "$screenshot" 2>/dev/null
  if [ ! -f "$screenshot" ]; then
    echo "Screenshot failed"; return 1
  fi

  CU_SHOT="$screenshot" CU_TARGET="$target" python3 <<'PY' 2>/dev/null
import os, subprocess
from PIL import Image

shot = os.environ["CU_SHOT"]
target = os.environ["CU_TARGET"].lower().strip()

img = Image.open(shot)
img_w, img_h = img.size

result = subprocess.run(["tesseract", shot, "-", "--dpi", "96", "tsv"],
                        capture_output=True, text=True)
lines = result.stdout.strip().split("\n")

found = []  # (word, cx, cy)
for line in lines[1:]:
    parts = line.split("\t")
    if len(parts) < 12:
        continue
    word = parts[11].strip()
    if not word or target not in word.lower():
        continue
    x, y = int(parts[6]), int(parts[7])
    bw, bh = int(parts[8]), int(parts[9])
    if bw <= 0 or bh <= 0:          # OCR noise rows have zero-size boxes
        continue
    found.append((word, x + bw // 2, y + bh // 2))

if found:
    for word, cx, cy in found:
        print(f'Found "{word}" at ({cx}, {cy})')
    avg_x = sum(c[1] for c in found) // len(found)
    avg_y = sum(c[2] for c in found) // len(found)
    print(f'Click target: ({avg_x}, {avg_y})')
else:
    print(f'"{target}" not found on screen')
PY
}

# Click on text found on screen
agent_vision_click() {
  local target="$1"
  local coords
  coords=$(agent_vision_find "$target" 2>/dev/null | grep "Click target:" | grep -oP '\(\d+, \d+\)' | tail -1)
  
  if [ -n "$coords" ]; then
    local x=$(echo "$coords" | grep -oP '\d+' | head -1)
    local y=$(echo "$coords" | grep -oP '\d+' | tail -1)
    xdotool mousemove --sync "$x" "$y" 2>/dev/null
    sleep 0.2
    xdotool click 1 2>/dev/null
    echo "Clicked on \"$target\" at ($x, $y)"
  else
    echo "Could not find \"$target\" on screen"
  fi
}

# Describe what's on screen (uses LLM for vision)
agent_vision_describe() {
  local screenshot="${1:-}"
  if [ -z "$screenshot" ]; then
    screenshot="$AGENT_DIR/screenshots/describe_$(date +%s).png"
    scrot -o "$screenshot" 2>/dev/null
  fi
  
  # Get text content
  local text
  text=$(tesseract "$screenshot" - 2>/dev/null)
  
# Get image properties (env-passed, not interpolated)
  local props
  props=$(CU_SHOT="$screenshot" python3 <<'PY' 2>/dev/null
import os
from PIL import Image
img = Image.open(os.environ["CU_SHOT"])
w, h = img.size
pixels = list(img.getdata())[:10000]
r_avg = sum(p[0] for p in pixels) // len(pixels)
g_avg = sum(p[1] for p in pixels) // len(pixels)
b_avg = sum(p[2] for p in pixels) // len(pixels)
brightness = (r_avg + g_avg + b_avg) / 3
print(f'Dimensions: {w}x{h}')
print(f'Brightness: {brightness:.0f}/255 ({"bright" if brightness > 128 else "dark"})')
PY
)
  
  echo "=== Screen Description ==="
  echo "$props"
  echo ""
  echo "Visible text:"
  echo "$text"
  
  # If ollama available, ask for visual description
  if curl -s http://localhost:11434/api/tags >/dev/null 2>&1; then
    echo ""
    echo "AI Analysis:"
    curl -s http://localhost:11434/api/generate \
      -d "{\"model\":\"llama3.1:8b\",\"prompt\":\"Describe what you see on this computer screen based on the following text extracted via OCR. Be concise and note any buttons, menus, input fields, or important UI elements:\\n\\n$text\",\"stream\":false}" 2>/dev/null | \
      python3 -c "import json,sys; print(json.load(sys.stdin).get('response',''))" 2>/dev/null
  fi
}
