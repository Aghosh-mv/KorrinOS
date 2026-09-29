#!/usr/bin/env python3
"""ask-history — search your own past prompts to this assistant.

Why this exists: the chat app does not show you your message history, so when
you half-remember something you said ("I told you about the worlds", "that
thing we discussed about security") you have no way to go and look it up.

This tool reads the assistant's own session database and searches YOUR messages
by text, printing them with timestamps. It only ever reads; it never writes,
never sends anything anywhere, and needs no account or network.

Usage:
  ./ask-history.py "worlds"              search for a word or phrase
  ./ask-history.py --regex "tinker|korrin"
  ./ask-history.py --days 3 "font"       limit to the last N days
  ./ask-history.py --all "3 things"      include every match, not just recent
  ./ask-history.py --mine "what did I say about X"   alias, same as plain search
  ./ask-history.py --stats               show what has been stored
"""

import argparse
import datetime
import json
import os
import re
import subprocess
import sys
from pathlib import Path

# Some Python builds (notably a source-built 3.14) ship without the sqlite3
# extension. Rather than fail with a bare ImportError, re-exec into an
# interpreter that has it.
try:
    import sqlite3
except ModuleNotFoundError:  # pragma: no cover - environment dependent
    if os.environ.get("_ASK_HISTORY_REEXEC"):
        sys.exit("error: no Python interpreter with sqlite3 support was found.")
    for alt in ("/usr/bin/python3", "/usr/bin/python3.11", "/usr/bin/python3.10",
                "/usr/bin/python3.12", "/usr/bin/python3.13"):
        if not os.path.isfile(alt):
            continue
        probe = subprocess.run([alt, "-c", "import sqlite3"], capture_output=True)
        if probe.returncode == 0:
            env = dict(os.environ, _ASK_HISTORY_REEXEC="1")
            os.execve(alt, [alt, os.path.abspath(__file__)] + sys.argv[1:], env)
    sys.exit("error: this Python has no sqlite3 module; install python3-sqlite3.")

DB = Path.home() / ".local/share/opencode/opencode.db"


def connect():
    if not DB.is_file():
        sys.exit(f"error: session database not found at {DB}")
    # Read-only: this tool must never be able to modify history.
    return sqlite3.connect(f"file:{DB}?mode=ro", uri=True)


def rows(db, since_ms=None):
    sql = (
        "SELECT p.time_created, p.data, m.data "
        "FROM part p JOIN message m ON m.id = p.message_id"
    )
    args = []
    if since_ms is not None:
        sql += " WHERE p.time_created >= ?"
        args.append(since_ms)
    for created, part, msg in db.execute(sql, args):
        try:
            pj, mj = json.loads(part), json.loads(msg)
        except (ValueError, TypeError):
            continue
        if mj.get("role") != "user" or pj.get("type") != "text":
            continue
        text = (pj.get("text") or "").strip()
        if not text:
            continue
        yield created, text


def ts(ms):
    return datetime.datetime.fromtimestamp(ms / 1000).strftime("%Y-%m-%d %H:%M")


def matches(text, needle, use_regex):
    if use_regex:
        try:
            return re.search(needle, text, re.I) is not None
        except re.error as exc:
            sys.exit(f"error: bad regex: {exc}")
    return needle.lower() in text.lower()


def main():
    ap = argparse.ArgumentParser(add_help=True)
    ap.add_argument("query", nargs="*", help="text to search for in your messages")
    ap.add_argument("--regex", action="store_true", help="treat the query as a regex")
    ap.add_argument("--days", type=float, help="only search the last N days")
    ap.add_argument("--all", action="store_true", help="show every match")
    ap.add_argument("--mine", dest="mine", nargs="*", help="alias for a plain search")
    ap.add_argument("--stats", action="store_true", help="show stored history summary")
    ap.add_argument("--max", type=int, default=25, help="max results to print (default 25)")
    args = ap.parse_args()

    db = connect()

    if args.stats:
        total = 0
        first = last = None
        for created, _ in rows(db):
            total += 1
            first = created if first is None else min(first, created)
            last = created if last is None else max(last, created)
        print(f"stored user messages: {total}")
        if first:
            print(f"oldest: {ts(first)}")
            print(f"newest: {ts(last)}")
        return

    query = " ".join(args.mine or args.query).strip()
    if not query:
        ap.print_help()
        return

    since = None
    if args.days:
        since = int((datetime.datetime.now() - datetime.timedelta(days=args.days)).timestamp() * 1000)

    found = [(c, t) for c, t in rows(db, since) if matches(t, query, args.regex)]

    if not found:
        scope = f"the last {args.days:g} days" if args.days else "all stored history"
        print(f'no match for "{query}" in {scope}.')
        return

    found.sort()
    shown = found if args.all else found[-args.max:]
    print(f'{len(found)} match(es) for "{query}" — showing {len(shown)} most recent:\n')
    for created, text in shown:
        flat = " ".join(text.split())
        limit = 400
        print(f"--- {ts(created)} ---")
        print(flat if len(flat) <= limit else flat[:limit] + " ...")
        print()


if __name__ == "__main__":
    main()
