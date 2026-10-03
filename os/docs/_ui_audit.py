#!/usr/bin/env python3
"""
UI/UX professionalism audit.

This is deliberately NOT a code-style audit. It looks for the things that make
an interface read as unfinished or amateur: animation that has no duration or
easing budget, motion that never settles, decorative motion with no function,
unbounded glow/shadow, primary-colour palettes, placeholder-looking copy,
inconsistent radii and spacing, and mascot/character chrome with no content.
"""
import os
import re
import sys
from collections import defaultdict

ROOT = "/home/tinkerspace/linux-kernel/os"
SKIP = {".git", "node_modules", "dist", "build", "__pycache__"}
UI_EXT = {".css", ".html", ".tsx", ".ts", ".jsx", ".js", ".scss", ".mjs"}

f = defaultdict(list)  # category -> [(file, line, detail)]


def add(cat, path, line, detail):
    f[cat].append((path, line, detail))


ui_files = []
for dp, dn, fns in os.walk(ROOT):
    dn[:] = [d for d in dn if d not in SKIP]
    for fn in fns:
        if os.path.splitext(fn)[1] in UI_EXT:
            p = os.path.join(dp, fn)
            try:
                if os.path.getsize(p) > 1_500_000:
                    continue
                ui_files.append((p, open(p, encoding="utf-8", errors="replace").read()))
            except OSError:
                pass

print(f"UI files: {len(ui_files)}", file=sys.stderr)

# ---------------------------------------------------------------- animations
for p, t in ui_files:
    rel = os.path.relpath(p, ROOT)
    for i, ln in enumerate(t.split("\n"), 1):
        s = ln.strip()

        # 1) transitions/animation with NO duration -> instant or browser default
        m = re.search(r"\b(transition|animation)\s*:\s*([^;{}]+)", ln)
        if m:
            decl = m.group(2)
            ("s" in decl and re.search(r"\d", decl)) or "var(" in decl
            prop = m.group(1)
            if not re.search(r"\d", decl) and "var(" not in decl:
                add("U01 transition/animation with no duration", rel, i, s[:70])
            if "all" in decl.split(",")[0]:
                add("U02 transition: all (transitions unintended props)", rel, i, s[:70])

        # 2) bouncy / overshoot easing
        for bez in re.findall(r"cubic-bezier\(([^)]*)\)", ln):
            nums = [x.strip() for x in bez.split(",")]
            if len(nums) == 4:
                try:
                    y1 = float(nums[1])
                    if y1 > 1.6 or y1 < -0.2:
                        add("U03 overshoot/bounce easing", rel, i, f"cubic-bezier({bez})")
                except ValueError:
                    pass
        if re.search(r"cubic-bezier\([^)]*\b(back|bounce|elastic|overshoot)\b", ln, re.I):
            add("U03 overshoot/bounce easing", rel, i, s[:70])

        # 3) motion that never stops
        if re.search(r"animation[^;]*\binfinite\b", ln):
            add("U04 infinite animation (never settles)", rel, i, s[:70])
        if re.search(r"animation[^;]*\balternate\b", ln):
            add("U05 alternate animation (ping-pong motion)", rel, i, s[:70])

        # 4) decorative blink / sparkle / pulse
        for kw in ("blink", "sparkle", "twinkle", "shimmer", "flicker", "pulse-glow", "neon"):
            if re.search(rf"\b{kw}\b", ln, re.I):
                add("U06 decorative blink/sparkle motion", rel, i, s[:70])

        # 5) motion on a property that causes layout thrash.
        # Only look at the transition's OWN value, not the whole rule: a rule
        # like `.progress-fill{height:100%;transition:width .4s}` animates width
        # and merely SETS height, so matching the whole line was a false positive.
        for tm in re.finditer(r"transition(?:-property)?\s*:\s*([^;}]+)", ln):
            val = tm.group(1)
            for prop in ("width", "height", "top", "left", "right", "bottom", "margin", "padding"):
                if re.search(rf"(^|[ ,]){prop}\b", val):
                    add("U07 animating layout property (causes jank)", rel, i, s[:70])

        # 6) too many simultaneous animations in one rule block
        if ln.count("@keyframes") == 1 and len(re.findall(r"@keyframes", t)) < 3:
            add("U08 one-off keyframe for a trivial effect", rel, i, s[:60])

        # 7) excessive duration (>600ms feels sluggish for UI)
        for dur in re.findall(r"(\d*\.?\d+)s\b", ln):
            try:
                if float(dur) > 0.6 and ("transition" in ln or "animation" in ln):
                    add("U09 sluggish duration >600ms", rel, i, f"{dur}s  {s[:50]}")
            except ValueError:
                pass

        # 8) primary colours / rainbow
        for hexc in re.findall(r"#([0-9a-fA-F]{6})\b", ln):
            r_, g_, b_ = int(hexc[0:2], 16), int(hexc[2:4], 16), int(hexc[4:6], 16)
            if (r_ > 200 and g_ < 60 and b_ < 60) or (r_ < 60 and g_ > 200 and b_ < 60) or \
               (r_ > 200 and g_ > 200 and b_ < 60) or (r_ > 200 and g_ < 60 and b_ > 200):
                add("U10 primary/rainbow colour in UI", rel, i, f"#{hexc}  {s[:50]}")
        if re.search(r"hue-rotate\s*\(|conic-gradient|rainbow", ln, re.I):
            add("U10 primary/rainbow colour in UI", rel, i, s[:60])

        # 9) unbounded glow / heavy shadow stacks
        shadows = re.findall(r"box-shadow\s*:\s*([^;]+)", ln)
        for sh in shadows:
            layers = sh.count(",") + 1
            if layers >= 3:
                add("U11 shadow stack >=3 layers", rel, i, f"{layers} layers")
            for blur in re.findall(r"(\d+)px", sh):
                if int(blur) > 60:
                    add("U12 excessive blur radius", rel, i, f"{blur}px  {s[:50]}")

        # 10) glassmorphism without a fallback
        if re.search(r"backdrop-filter", ln) and "rgba(" not in t and "@supports" not in t:
            add("U13 backdrop-filter without fallback/@supports", rel, i, s[:60])

        # 11) emoji used as UI iconography
        if re.search(r"[\U0001F300-\U0001FAFF☀-➿]", ln) and not re.search(r"//|/\*|\* ", s):
            add("U14 emoji as UI iconography", rel, i, s[:60])

        # 12) text placeholder / lorem / TODO in shipped UI
        # Only prose counts. `id="todo"`, `.paintTodo()`, and a regex that
        # extracts the word TODO from a user's notes are all legitimate.
        if re.search(r"lorem ipsum|placeholder text|coming soon|under construction|not implemented yet", ln, re.I):
            add("U15 placeholder copy in shipped UI", rel, i, s[:60])

        # 13) inline styles in markup (hard to theme, smells)
        if p.endswith((".html", ".tsx", ".jsx")) and re.search(r'style\s*=\s*"\s*[^"]*:', ln):
            add("U16 inline style attribute in markup", rel, i, s[:60])

        # 14) !important overuse
        if "!important" in ln:
            add("U17 !important in stylesheet", rel, i, s[:60])

        # 15) fixed px font sizes (breaks user font scaling)
        if re.search(r"font-size\s*:\s*\d+px", ln):
            add("U18 fixed px font-size (ignores user scaling)", rel, i, s[:60])

        # 16) !important on transition/animation = fighting the cascade
        if re.search(r"(transition|animation)[^;]*!important", ln):
            add("U19 !important on motion property", rel, i, s[:60])

# ---------------------------------------------------------------- mascots
for p, t in ui_files:
    rel = os.path.relpath(p, ROOT)
    for i, ln in enumerate(t.split("\n"), 1):
        for kw in ("mascot", "pet", "companion", "duck", "sprite", "character"):
            if re.search(rf"\b{kw}\b", ln, re.I) and re.search(
                r"(blinky|eyes|animation|walk|idle|hop|waddle|bounce|follow)", ln, re.I
            ):
                add("U20 character/mascot with idle motion", rel, i, s[:60] if (s := ln.strip()) else "")

# mascot shells with no content
for dp, dn, fns in os.walk(ROOT):
    dn[:] = [d for d in dn if d not in SKIP]
    for fn in fns:
        if "mascot" in fn.lower() or "pet" in fn.lower():
            p = os.path.join(dp, fn)
            try:
                t = open(p, encoding="utf-8", errors="replace").read()
            except OSError:
                continue
            if "NO CONTENT YET" in t or "waiting for user" in t.lower():
                add("U21 mascot framework shipped with no content", os.path.relpath(p, ROOT), 1, fn)

# ---------------------------------------------------------------- consistency
# collect every border-radius and animation-duration used, to show the spread
radii, durs = set(), set()
for p, t in ui_files:
    for r in re.findall(r"border-radius\s*:\s*([^;]+)", t):
        radii.add(r.strip()[:24])
    for d in re.findall(r"(?:transition|animation)[^;{]*?(\d*\.?\d+)m?s", t):
        durs.add(d)
if len(radii) > 6:
    f["U22 inconsistent corner radii (no scale)"].append(
        ("<tree-wide>", 0, f"{len(radii)} distinct values: {', '.join(sorted(radii)[:10])}"))
if len(durs) > 8:
    f["U23 inconsistent motion durations (no scale)"].append(
        ("<tree-wide>", 0, f"{len(durs)} distinct values: {', '.join(sorted(durs)[:12])}"))

total = sum(len(v) for v in f.values())
out = ["# KorrinOS — UI/UX Professionalism Audit", "",
       "Generated by `docs/_ui_audit.py`. Every count is MEASURED over the real UI files",
       "(css/html/tsx/ts/js). This is about how the interface FEELS, not code style:", "",
       "motion that has no duration or easing budget, motion that never settles,",
       "decorative motion with no function, unbounded glow, primary-colour palettes,",
       "placeholder copy, icon-as-emoji, and mascot chrome with no content.", "",
       f"## Total: {total}", "", "| Category | Count |", "|---|---|"]
for c in sorted(f, key=lambda x: -len(f[x])):
    out.append(f"| {c[3:]} | {len(f[c])} |")
out += ["", "## Detail", ""]
for c in sorted(f, key=lambda x: -len(f[x])):
    out += [f"### {c} — {len(f[c])}", "", "| file | line | detail |", "|---|---|---|"]
    for p, l, d in f[c]:
        out.append(f"| `{p}` | {l} | {str(d).replace('|', '/')[:80]} |")
    out.append("")

with open(os.path.join(ROOT, "docs", "UI-AUDIT.md"), "w", encoding="utf-8") as fh:
    fh.write("\n".join(out))
import shutil
shutil.copy(os.path.abspath(__file__), os.path.join(ROOT, "docs", "_ui_audit.py"))

print(f"TOTAL UI FINDINGS: {total}")
for c in sorted(f, key=lambda x: -len(f[x])):
    print(f"  {len(f[c]):4d}  {c}")
print("wrote docs/UI-AUDIT.md")
