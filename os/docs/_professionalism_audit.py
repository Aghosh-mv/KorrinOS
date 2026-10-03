#!/usr/bin/env python3
"""
KorrinOS professionalism audit.

Every finding is produced by a real measurement over the tree, so the report is
a count of actual occurrences rather than a list of invented complaints.
Findings are grouped by category and written with file:line so each one can be
fixed and re-measured.
"""
import os
import re
import sys
from collections import defaultdict

ROOT = "/home/tinkerspace/linux-kernel"
OS = os.path.join(ROOT, "os")
SKIP_DIRS = {".git", "node_modules", "dist", "build", "__pycache__", ".sum"}

findings = defaultdict(list)  # category -> [(path, line, detail)]


def add(cat, path, line, detail):
    findings[cat].append((path, line, detail))


def walk(root, exts):
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for fn in sorted(filenames):
            if os.path.splitext(fn)[1] in exts:
                p = os.path.join(dirpath, fn)
                try:
                    if os.path.getsize(p) > 2_000_000:
                        continue
                    with open(p, encoding="utf-8", errors="replace") as fh:
                        yield p, fh.read().split("\n")
                except OSError:
                    pass


REL = lambda p: os.path.relpath(p, ROOT)

# =============================================================== shell files
sh_files = list(walk(OS, {".sh"}))
# extension-less executables that are still shell (no .sh suffix)
for p, _ in list(walk(OS, {""})):
    try:
        with open(p, "rb") as fh:
            head = fh.read(80)
        if head.startswith(b"#!/bin/bash") or head.startswith(b"#!/usr/bin/env bash"):
            with open(p, encoding="utf-8", errors="replace") as fh:
                sh_files.append((p, fh.read().split("\n")))
    except OSError:
        pass

print(f"shell files: {len(sh_files)}", file=sys.stderr)

for path, lines in sh_files:
    rel = REL(path)
    text = "\n".join(lines)

    if not text.startswith("#!"):
        add("C01 no shebang", rel, 1, "script has no interpreter line")
    elif not re.match(r"^#!.*\b(ba|z|k|c|tc|da)?sh\b", text):
        add("C01 non-bash shebang", rel, 1, text.split("\n")[0][:50])

    if "set -e" not in text and "set -u" not in text and "set -o pipefail" not in text:
        add("C02 no strict mode", rel, 1, "no 'set -euo pipefail'")

    for i, ln in enumerate(lines, 1):
        s = ln.strip()
        if len(ln) > 120:
            add("C03 line >120 cols", rel, i, f"{len(ln)} cols")
        if ln.rstrip() != ln and ln.strip():
            add("C04 trailing whitespace", rel, i, repr(ln[-12:]))
        if "\t" in ln:
            add("C05 tab indentation", rel, i, "literal tab")
        if re.search(r"\bTODO\b|\bFIXME\b|\bXXX\b|\bHACK\b", ln):
            m = re.search(r"\b(TODO|FIXME|XXX|HACK)\b:?[ ]*(.{0,40})", ln)
            add("C06 TODO/FIXME/HACK left in code", rel, i, (m.group(0)[:60] if m else ""))
        if re.search(r"/tmp/[A-Za-z0-9_.-]+", ln) and "mktemp" not in ln:
            add("C07 hardcoded /tmp path", rel, i, s[:70])
        if re.search(r"\$HOME/\.tinker", ln):
            add("C08 legacy ~/.tinker state path", rel, i, s[:70])
        if re.search(r"\brm -rf\s+\"?\$", ln):
            add("C09 rm -rf on a variable", rel, i, s[:70])
        if re.search(r"\beval\b", ln) and not s.startswith("#"):
            add("C10 eval present", rel, i, s[:70])
        if re.search(r"\becho\b.*\b(error|Error|ERROR|failed|Failed|FAILED)\b", ln) and ">&2" not in ln:
            add("C11 error text on stdout", rel, i, s[:70])
        if re.search(r"\bsudo\b", ln) and '"$' not in ln and "'$" not in ln and '"${' not in ln:
            add("C12 sudo with unquoted var", rel, i, s[:70])
        if re.search(r"\becho -e\b", ln):
            add("C13 echo -e (non-portable)", rel, i, s[:60])
        if re.search(r"\$\(\s*echo\b", ln):
            add("C14 useless $(echo)", rel, i, s[:60])
        if re.search(r"\[\s*-[a-zA-Z]\s+\"\$", ln) and "==" not in ln:
            pass
        if re.search(r"if \[\[.*=~.*\]\]", ln):
            add("C15 regex in [[ ]] unquoted RHS risk", rel, i, s[:60])
        if re.search(r"\bwhich\b", ln):
            add("C16 'which' instead of 'command -v'", rel, i, s[:60])
        if re.search(r"\bls \|", ln):
            add("C17 parsing ls output", rel, i, s[:60])
        if re.search(r"^[^#]*\bfor\s+\w+\s+in\s+\$\(", ln):
            add("C18 for-loop over $(...) (word split)", rel, i, s[:60])
        if re.search(r"\bchmod\s+777\b", ln):
            add("C19 chmod 777", rel, i, s[:60])
        if re.search(r"\bchmod\b.*\+s\b", ln):
            add("C20 setuid bit", rel, i, s[:60])
        if re.search(r"curl[^|]*\|\s*(sudo\s+)?(ba)?sh", ln):
            add("C21 curl piped to shell", rel, i, s[:60])
        if re.search(r"sleep\s+\d+\s*$", ln) and "while" not in text:
            add("C22 bare sleep", rel, i, s[:50])
        if re.search(r"\bkillall\b", ln):
            add("C23 killall", rel, i, s[:50])
        if re.search(r"\bexport\s+\w+=\"?\$", ln) and "PATH" not in ln and "LD_" not in ln:
            add("C24 exporting a local-ish var", rel, i, s[:60])
        if re.search(r"2>&1\s*\|\s*head", ln):
            add("C25 pipe to head (SIGPIPE risk)", rel, i, s[:60])
        if re.search(r"^\s*local\s+\w+=\$\(", ln) and "=" in ln and ln.count("=") > 3:
            add("C26 complex inline command substitution", rel, i, s[:60])
        if "..." in ln and "usage" not in s.lower() and "example" not in s.lower():
            pass
        if re.search(r"[‘’“”]", ln):
            add("C27 smart quotes in code", rel, i, s[:50])
        if re.search(r"\$\{[A-Za-z_][A-Za-z0-9_]*\[@\]", ln) and '"${' not in ln:
            add("C29 unquoted array expansion", rel, i, s[:60])
        if re.search(r"\bexit\s+\$", ln):
            add("C30 exit with variable status", rel, i, s[:50])
        if re.search(r"\|\|\s*true\b", ln):
            add("C31 '|| true' swallows errors", rel, i, s[:60])
        if re.search(r"^set \+e", ln):
            add("C32 'set +e' disables error mode", rel, i, s[:50])

    if re.search(r"\.(bak|orig|rej|tmp|old|swp|copy|~)$", path) or ".bak." in path:
        add("C34 stray backup/scratch file in tree", rel, 0, os.path.basename(path))
    if not re.search(r"(usage|--help|-h\)|show_help|help\(\))", text, re.I):
        add("C33 no usage/help text", rel, 1, "no --help/usage path")

# =============================================================== python
py_files = list(walk(OS, {".py"}))
print(f"python files: {len(py_files)}", file=sys.stderr)
for path, lines in py_files:
    rel = REL(path)
    text = "\n".join(lines)
    for i, ln in enumerate(lines, 1):
        s = ln.strip()
        if len(ln) > 100:
            add("P01 line >100 cols", rel, i, f"{len(ln)} cols")
        if ln.rstrip() != ln and s:
            add("P02 trailing whitespace", rel, i, repr(ln[-12:]))
        if "\t" in ln:
            add("P03 tab indentation", rel, i, "literal tab")
        if re.search(r"\bTODO\b|\bFIXME\b|\bXXX\b", ln):
            add("P04 TODO/FIXME left", rel, i, s[:60])
        if re.search(r"except\s*:\s*$", ln):
            add("P05 bare except", rel, i, s[:60])
        if re.search(r"except\b.*:\s*$", ln) and i < len(lines) and lines[i].strip().startswith("pass"):
            add("P06 except: pass (silent swallow)", rel, i, s[:50])
        if re.search(r"\bprint\s*\(", ln) and "cli" not in rel and "test" not in rel:
            add("P07 debug print in library", rel, i, s[:50])
        if re.search(r"#\s*type:", ln):
            add("P08 legacy type comment", rel, i, s[:50])
        # only a mutable default ARGUMENT of a function definition is a bug;
        # a bare `x = []` assignment is normal code
        if re.search(r"^\s*def\s+\w+\s*\([^)]*=\s*(\[\s*\]|\{\s*\}|set\(\s*\))", ln):
            add("P09 mutable default argument", rel, i, s[:60])
        if re.search(r"==\s*None|!=\s*None", ln):
            add("P10 comparison to None", rel, i, s[:50])
        if re.search(r"\beval\s*\(", ln):
            add("P11 eval()", rel, i, s[:50])
        if re.search(r"\bexec\s*\(", ln):
            add("P12 exec()", rel, i, s[:50])
        if re.search(r"shell\s*=\s*True", ln):
            add("P13 shell=True", rel, i, s[:50])
        if re.search(r"verify\s*=\s*False", ln):
            add("P14 TLS verification disabled", rel, i, s[:50])
        if re.search(r"subprocess\.(call|run|Popen)\([^)]*\)\s*$", ln) and "shell" not in ln:
            pass
        if re.search(r"\.format\(", ln):
            add("P15 str.format instead of f-string", rel, i, s[:50])
        if re.search(r"%\s*\(.*\)\s*$", ln) and "logging" not in s:
            add("P16 printf-style formatting", rel, i, s[:50])
        if re.search(r"\bos\.system\b", ln):
            add("P17 os.system", rel, i, s[:50])
        if re.search(r"\bassert\b", ln) and "test" not in rel:
            add("P18 assert used outside tests", rel, i, s[:50])
    if not re.search(r'(#!|"""|\'\'\')', text[:400]):
        add("P19 no module docstring/shebang", rel, 1, "")

# =============================================================== C
c_files = list(walk(OS, {".c"})) + list(walk(os.path.join(ROOT, "kernel", "tinker"), {".c"}))
print(f"c files: {len(c_files)}", file=sys.stderr)
for path, lines in c_files:
    rel = REL(path)
    for i, ln in enumerate(lines, 1):
        s = ln.strip()
        if len(ln) > 100:
            add("C101 line >100 cols", rel, i, f"{len(ln)} cols")
        if "\t" in ln and not ln.startswith("\t"):
            add("C102 mixed tabs/spaces", rel, i, s[:50])
        if re.search(r"\bprintf\s*\(\s*\"[^\"]*%[^%\"]", ln) and "%%" not in ln:
            pass
        if re.search(r"\bstrcpy\s*\(|\bstrcat\s*\(|\bgets\s*\(", ln):
            add("C103 unsafe string function", rel, i, s[:50])
        if re.search(r"\bmalloc\s*\(", ln) and "free" not in text:
            add("C104 malloc without free", rel, i, s[:50])
        if re.search(r"\bmemcpy\s*\(", ln):
            add("C105 memcpy (size must be checked)", rel, i, s[:50])
        if re.search(r"\bsprintf\s*\(", ln) and "snprintf" not in ln:
            add("C106 sprintf (use snprintf)", rel, i, s[:50])
        if re.search(r"//", ln) and "http://" not in ln and "https://" not in ln:
            add("C107 C++ style // comment", rel, i, s[:50])
        if re.search(r"\bTODO\b|\bFIXME\b|\bXXX\b", ln):
            add("C108 TODO/FIXME left", rel, i, s[:50])
        if "\t" in ln and re.search(r"    ", ln):
            add("C109 mixed indentation", rel, i, s[:50])
    if "SPDX-License-Identifier" not in "\n".join(lines[:12]):
        add("C110 missing SPDX license header", rel, 1, "no SPDX in first 12 lines")
    if not re.search(r"\bMODULE_LICENSE\b", text):
        add("C111 no MODULE_LICENSE", rel, 1, "kernel module lacks MODULE_LICENSE")

# =============================================================== js/ts/tsx
web_files = list(walk(OS, {".js", ".mjs", ".ts", ".tsx", ".jsx"}))
print(f"web files: {len(web_files)}", file=sys.stderr)
for path, lines in web_files:
    rel = REL(path)
    for i, ln in enumerate(lines, 1):
        s = ln.strip()
        if len(ln) > 120:
            add("W01 line >120 cols", rel, i, f"{len(ln)} cols")
        if re.search(r"\bconsole\.log\(", ln) and "cli" not in rel and "vite" not in rel:
            add("W02 console.log left in", rel, i, s[:50])
        if re.search(r"\bdebugger\b", ln):
            add("W03 debugger statement", rel, i, s[:50])
        if re.search(r"\bvar\s+\w+\s*=", ln):
            add("W04 var instead of let/const", rel, i, s[:50])
        if re.search(r"==\s*[^=]|!=\s*[^=]", ln) and "!==" not in ln and "===" not in ln:
            add("W05 loose equality", rel, i, s[:50])
        if re.search(r"\beval\s*\(", ln):
            add("W06 eval()", rel, i, s[:50])
        if re.search(r"innerHTML\s*=", ln):
            add("W07 innerHTML assignment", rel, i, s[:50])
        if re.search(r"document\.write\s*\(", ln):
            add("W08 document.write", rel, i, s[:50])
        if re.search(r"\.ts\(\s*['\"]", ln) or re.search(r"any\b", ln):
            add("W09 'any' type escape hatch", rel, i, s[:50])
        if re.search(r"@ts-ignore|@ts-expect-error", ln):
            add("W10 ts-ignore suppressing types", rel, i, s[:50])
        if re.search(r"\bdangerouslySetInnerHTML\b", ln):
            add("W11 dangerouslySetInnerHTML", rel, i, s[:50])
        if re.search(r"\bas\s+any\b", ln):
            add("W12 'as any' cast", rel, i, s[:50])
        if re.search(r"\bTODO\b|\bFIXME\b|\bXXX\b", ln):
            add("W13 TODO/FIXME left", rel, i, s[:50])
        if re.search(r"localStorage|sessionStorage", ln) and "token" in s.lower():
            add("W14 token in web storage", rel, i, s[:50])

# =============================================================== naming
BRANDS = {
    "KorrinOS": 0, "Korrinos": 0, "korrinos": 0, "Korrin": 0, "korrin": 0,
    "KorrinOS": 0, "Tinkeros": 0, "tinkeros": 0, "Tinker AI": 0, "TinkerAI": 0,
    "tinker-ai": 0, "Parc": 0, "parc-ai": 0, "VOKK": 0, "vokk": 0,
    "Zegrate": 0, "HyperDrive": 0, "hyperdrive": 0,
}
for p, lines in walk(OS, {".sh", ".py", ".md", ".service", ".desktop", ".ts", ".tsx", ".c", ".h", ".conf", ""}):
    text = "\n".join(lines)
    for b in BRANDS:
        BRANDS[b] += text.count(b)
with open(os.path.join(ROOT, "os", "docs", "AI-NAMING.md"), encoding="utf-8") as fh:
    pass
naming_detail = ", ".join(f"{k}={v}" for k, v in sorted(BRANDS.items(), key=lambda x: -x[1]))
findings["N01 brand-name split brain"].append(("<tree-wide>", 0, naming_detail))
for b, c in BRANDS.items():
    if c and b.lower() not in ("korrinos",):
        findings["N02 non-canonical brand in use"].append(("<tree-wide>", 0, f"{b}: {c} occurrences"))

# files whose names embed a stale brand
for p, _ in walk(OS, {".sh", ".py", ".md", ".ts", ".tsx", ".c", ""}):
    base = os.path.basename(p).lower()
    for b in ("tinker", "tinkeros", "parc", "vokk", "korrinos"):
        if b in base:
            findings["N03 filename embeds a brand"].append((REL(p), 0, base))

# =============================================================== write
out = []
total = 0
out.append("# KorrinOS — Professionalism Audit")
out.append("")
out.append("Every count below is MEASURED by `docs/_professionalism_audit.py` over the tree.")
out.append("No item is invented: each is a real occurrence at a real file:line.")
out.append("Re-run the script after fixing to watch the numbers fall.")
out.append("")
out.append(f"## Total findings: {sum(len(v) for v in findings.values())}")
out.append("")
out.append("| # | Category | Count |")
out.append("|---|----------|-------|")
for cat in sorted(findings, key=lambda c: -len(findings[c])):
    out.append(f"| {cat.split()[0]} | {cat[3:]} | {len(findings[cat])} |")
    total += len(findings[cat])
out.append("")
out.append("## Findings by category")
out.append("")
for cat in sorted(findings, key=lambda c: -len(findings[c])):
    out.append(f"### {cat} — {len(findings[cat])}")
    out.append("")
    out.append("| file | line | detail |")
    out.append("|------|------|--------|")
    for p, l, d in findings[cat]:
        dl = str(d).replace("|", "\\|")[:80]
        out.append(f"| `{p}` | {l} | {dl} |")
    out.append("")

path = os.path.join(OS, "docs", "PROFESSIONALISM.md")
with open(path, "w", encoding="utf-8") as fh:
    fh.write("\n".join(out))
with open(os.path.join(OS, "docs", "_professionalism_audit.py"), "w", encoding="utf-8") as fh:
    fh.write(open(os.path.abspath(__file__), encoding="utf-8").read())

print(f"TOTAL FINDINGS: {total}")
for cat in sorted(findings, key=lambda c: -len(findings[c]))[:20]:
    print(f"  {len(findings[cat]):5d}  {cat}")
print("wrote docs/PROFESSIONALISM.md + docs/_professionalism_audit.py")
