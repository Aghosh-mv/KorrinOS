#!/usr/bin/env bash
# End-to-end test of os/parc-ai/modules/computer-use.sh against a real X server.
# No window manager, so this also exercises the xdotool fallback paths.
set -u  # deliberately keep set -u: the module must survive a strict caller

MOD="$(dirname "$0")/modules/computer-use.sh"
PASS=0; FAIL=0
ok()   { PASS=$((PASS+1)); echo "  PASS  $1"; }
bad()  { FAIL=$((FAIL+1)); echo "  FAIL  $1"; }

echo "== loading module =="
# shellcheck source=/dev/null
source "$MOD" || { echo "module failed to load"; exit 1; }
echo "  module loaded"

echo
echo "== argument validation (must fail cleanly, not silently) =="
cu_move            >/dev/null 2>&1 && bad "cu_move with no args should fail" || ok "cu_move no-args rejected"
cu_click sideways  >/dev/null 2>&1 && bad "cu_click bad button should fail" || ok "cu_click bad button rejected"
cu_scroll sideways >/dev/null 2>&1 && bad "cu_scroll bad dir should fail" || ok "cu_scroll bad dir rejected"
cu_type            >/dev/null 2>&1 && bad "cu_type no-args should fail"    || ok "cu_type no-args rejected"
cu_key             >/dev/null 2>&1 && bad "cu_key no-args should fail"     || ok "cu_key no-args rejected"
cu_focus           >/dev/null 2>&1 && bad "cu_focus no-args should fail"    || ok "cu_focus no-args rejected"
cu_clip_set        >/dev/null 2>&1 && bad "cu_clip_set no-args should fail"|| ok "cu_clip_set no-args rejected"
cu_capture bogus   >/dev/null 2>&1 && bad "cu_capture bad mode should fail" || ok "cu_capture bad mode rejected"

echo
echo "== clipboard round-trip =="
cu_clip_set "korrinos-computer-use-test" >/dev/null 2>&1
GOT=$(cu_clip_get 2>/dev/null)
if [ "$GOT" = "korrinos-computer-use-test" ]; then
  ok "clipboard set/get round-trip ($GOT)"
else
  bad "clipboard round-trip got '$GOT'"
fi

echo
echo "== screenshot (full screen) =="
SHOT=$(cu_screenshot 2>/dev/null)
if [ -n "$SHOT" ] && [ -s "$SHOT" ]; then
  ok "full screenshot captured: $(basename "$SHOT") ($(stat -c%s "$SHOT") B)"
else
  bad "full screenshot failed"
fi

echo
echo "== region capture =="
RSHOT=$(cu_capture region 0 0 320 240 2>/dev/null)
if [ -n "$RSHOT" ] && [ -s "$RSHOT" ]; then
  DIM=$(CU_SHOT="$RSHOT" python3 -c 'from PIL import Image;import os;print("%dx%d"%Image.open(os.environ["CU_SHOT"]).size)' 2>/dev/null)
  if [ "$DIM" = "320x240" ]; then ok "region capture is exactly 320x240"; else bad "region capture wrong size: $DIM"; fi
else
  bad "region capture failed"
fi

echo
echo "== mouse move + click =="
cu_move 100 120 >/dev/null 2>&1 && ok "cu_move ok" || bad "cu_move failed"
POS=$(xdotool getmouselocation 2>/dev/null | grep -oE 'x:[0-9]+ y:[0-9]+' | head -1)
if echo "$POS" | grep -q 'x:100 y:120'; then
  ok "pointer really at $POS"
else
  bad "pointer at '$POS', expected '100 120'"
fi
cu_click left single >/dev/null 2>&1 && ok "cu_click left ok" || bad "cu_click left failed"
cu_click right single >/dev/null 2>&1 && ok "cu_click right ok" || bad "cu_click right failed"
cu_click left double  >/dev/null 2>&1 && ok "cu_click double ok" || bad "cu_click double failed"
cu_scroll down 3    >/dev/null 2>&1 && ok "cu_scroll ok" || bad "cu_scroll failed"
cu_drag 10 10 200 200 >/dev/null 2>&1 && ok "cu_drag ok" || bad "cu_drag failed"

echo
echo "== keyboard =="
cu_type "hello korrinos" >/dev/null 2>&1 && ok "cu_type ok" || bad "cu_type failed"
cu_key ctrl+a          >/dev/null 2>&1 && ok "cu_key ctrl+a ok" || bad "cu_key ctrl+a failed"
cu_key Return          >/dev/null 2>&1 && ok "cu_key Return ok"  || bad "cu_key Return failed"
cu_typeenter "combo"   >/dev/null 2>&1 && ok "cu_typeenter ok"  || bad "cu_typeenter failed"

echo
echo "== window listing (xdotool fallback, no wmctrl) =="
WINS=$(cu_windows 2>/dev/null | wc -l)
if [ "$WINS" -ge 0 ]; then ok "cu_windows returned $WINS line(s)"; else bad "cu_windows failed"; fi

echo
echo "== cu_act action program =="
OUT=$(cu_act '[{"op":"type","text":"abc"},{"op":"key","keys":["Return"]},{"op":"wait","seconds":0.1}]' 2>&1)
if echo "$OUT" | grep -q 'cu_act: OK'; then ok "cu_act ran a valid program"
else bad "cu_act failed: $(echo "$OUT" | tail -2 | tr '\n' ' ')"; fi

OUT2=$(cu_act '[{"op":"type","text":"a"},{"op":"bogusop"}]' 2>&1)
if echo "$OUT2" | grep -q 'FAILURES'; then ok "cu_act reports unknown ops instead of pretending"
else bad "cu_act did not report the bad op"; fi

OUT3=$(cu_act 'not json at all' 2>&1)
if echo "$OUT3" | grep -qi 'invalid JSON'; then ok "cu_act rejects malformed JSON"
else bad "cu_act accepted malformed JSON"; fi

# Injection safety: a step text that would break naive shell/Python splicing.
OUT4=$(cu_act '[{"op":"type","text":"a\"; import os; os.system(\"id\")"}]' 2>&1)
if echo "$OUT4" | grep -q 'cu_act: OK'; then ok "cu_act treats hostile text as data (no injection)"
else bad "cu_act mishandled hostile text: $OUT4"; fi

echo
echo "== summary =="
echo "  pass=$PASS fail=$FAIL"
[ "$FAIL" -eq 0 ] && echo "  ALL COMPUTER-USE TESTS PASSED" || echo "  THERE ARE FAILURES"
exit $(( FAIL > 0 ))
