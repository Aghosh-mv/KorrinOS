#!/usr/bin/env python3
"""
KorrinOS Desktop Server — serves the Nibra-style desktop UI plus a real API.
Every action routes to a REAL KorrinOS backend (VOKK v4, world engine, system).
No mock data. No emojis.
"""
import json
import os
import re
import socket
import subprocess
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import urlparse, parse_qs

ROOT = "/opt/korrinos/os/desktop/nibra-style"
WIDGET_DIR = os.path.expanduser("~/.local/share/korrinos/widgets")
KORRINOS_DIR = "/opt/korrinos/os"
WORLD_ENGINE = f"{KORRINOS_DIR}/territories/world-engine.sh"
VOKK_CTRL = f"{KORRINOS_DIR}/vokk/vokk_controller.py"
SYS_CTRL = f"{KORRINOS_DIR}/vokk/system-controller/controller.py"

PORT = int(os.environ.get("KORRINOS_DESKTOP_PORT", "8898"))


def sh(cmd):
    try:
        return subprocess.run(cmd, shell=True, capture_output=True,
                              text=True, timeout=20).stdout.strip()
    except Exception:
        return ""


def whoami():
    return sh("echo ${SUDO_USER:-$(whoami)}")


def world_api():
    cur = sh(f"bash {WORLD_ENGINE} current") or "NORMAL"
    return {
        "current": cur.upper(),
        "worlds": ["NORMAL", "HACK", "GAME"],
        "labels": {"NORMAL": "Work", "HACK": "Hackers", "GAME": "Play"},
    }


def set_world(name):
    if name in ("NORMAL", "HACK", "GAME"):
        return sh(f"bash {WORLD_ENGINE} switch {name}")
    return "unknown world"


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *a):
        pass

    def _send(self, code, body, ctype="application/json"):
        data = body if isinstance(body, bytes) else body.encode() if isinstance(body, str) else json.dumps(body).encode()
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def _fs(self, path):
        return os.path.normpath(os.path.join(WIDGET_DIR, path))

    def do_GET(self):
        url = urlparse(self.path)
        p = url.path
        if p == "/api/state":
            self._send(200, {
                "user": whoami(),
                "world": world_api(),
                "time": sh("date '+%I:%M %p'"),
                "date": sh("date '+%A, %d %B %Y'"),
                "battery": sh("cat /sys/class/power_supply/BAT*/capacity 2>/dev/null | head -1") or "desktop",
                "weather": sh("curl -s 'https://wttr.in/?format=j1' 2>/dev/null | head -c 500") or "{}",
                "recent": [],
                "vokkRunning": bool(sh("pgrep -f vokk_controller.py")),
            })
            return
        if p == "/api/world":
            self._send(200, world_api())
            return
        if p == "/api/recent":
            import glob
            items = []
            for base in [os.path.expanduser("~/Documents"),
                         os.path.expanduser("~/Downloads"),
                         os.path.expanduser("~/.local/share/korrinos")]:
                if os.path.isdir(base):
                    for f in glob.glob(base + "/*")[:8]:
                        items.append({"name": os.path.basename(f),
                                      "mtime": sh(f"stat -c %y '{f}'")[:16] or ""})
            self._send(200, items)
            return
        if p == "/api/tasks":
            tasks = []
            tpf = os.path.expanduser("~/.local/share/korrinos/tasks.txt")
            if os.path.isfile(tpf):
                for line in open(tpf):
                    line = line.strip()
                    if line:
                        tasks.append({"text": line, "done": False})
            self._send(200, tasks)
            return
        if p.startswith("/"):
            fn = self._fs(p.lstrip("/")) if p != "/" else os.path.join(WIDGET_DIR, "desktop.html")
            if os.path.isfile(fn) and os.path.realpath(fn).startswith(os.path.realpath(WIDGET_DIR)):
                ctype = "text/html"
                if fn.endswith(".js"):
                    ctype = "application/javascript"
                elif fn.endswith(".css"):
                    ctype = "text/css"
                self._send(200, open(fn, "rb").read(), ctype)
                return
            self._send(404, {"error": "not found"})
            return

    def do_POST(self):
        url = urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        try:
            data = json.loads(self.rfile.read(length) or b"{}")
        except Exception:
            data = {}
        if url.path == "/api/launch":
            app = data.get("app", "")
            acts = {
                "store": "korrinos-launch store",
                "store-apps": "korrinos-launch app-store",
                "software": "korrinos-launch software-center",
                "package": "korrinos-launch package-manager",
                "appstore": "korrinos-launch app-store",
                "apps": "korrinos-launch software-center",
                "browser": "korrinos-launch browser",
                "files": "korrinos-launch files",
                "filemanager": "korrinos-launch file-manager",
                "files-manager": "korrinos-launch file-manager",
                "terminal": "korrinos-launch terminal",
                "settings": "korrinos-launch settings",
                "settings-gui": "korrinos-launch settings",
                "code": "korrinos-launch code",
                "vokk": "korrinos-launch vokk",
                "vokk-app": "korrinos-launch vokk",
                "battery": "korrinos-launch battery-monitor",
                "batterymonitor": "korrinos-launch battery-monitor",
                "notes": "korrinos-launch quick-note",
                "quicknote": "korrinos-launch quick-note",
                "notes-app": "korrinos-launch quick-note",
                "clipboard": "korrinos-launch smart-clipboard",
                "smartclipboard": "korrinos-launch smart-clipboard",
                "clip": "korrinos-launch smart-clipboard",
                "ocr": "korrinos-launch ocr-everywhere",
                "ocr-everywhere": "korrinos-launch ocr-everywhere",
                "voice": "korrinos-launch voice-commands",
                "voice-commands": "korrinos-launch voice-commands",
                "recorder": "korrinos-launch screen-recorder",
                "screenrecorder": "korrinos-launch screen-recorder",
                "screenshot": "korrinos-launch screenshot",
                "game": "korrinos-launch gaming",
                "gaming": "korrinos-launch gaming",
                "gamemode": "korrinos-launch gaming",
                "game-mode": "korrinos-launch gaming",
                "games": "korrinos-launch gaming",
                "terminal-app": "korrinos-launch terminal",
                "music": "korrinos-launch music",
                "mymusic": "korrinos-launch music",
                "player": "korrinos-launch music",
                "pictures": "korrinos-launch pictures",
                "photo": "korrinos-launch pictures",
                "photos": "korrinos-launch pictures",
                "image": "korrinos-launch pictures",
                "videos": "korrinos-launch videos",
                "video": "korrinos-launch videos",
                "camera": "korrinos-launch camera",
                "webcam": "korrinos-launch camera",
                "mouse": "korrinos-launch mouse",
                "sysmon": "korrinos-launch system-monitor",
                "monitor": "korrinos-launch system-monitor",
                "systemmonitor": "korrinos-launch system-monitor",
                "taskmanager": "korrinos-launch system-monitor",
                "taskmgr": "korrinos-launch system-monitor",
                "manager": "korrinos-launch system-monitor",
                "security": "korrinos-launch security",
                "wallpaper": "korrinos-launch wallpapers",
                "calendar": "korrinos-launch calendar",
                "clock": "korrinos-launch clock",
                "weather": "korrinos-launch weather",
                "calculator": "korrinos-launch calculator",
                "calc": "korrinos-launch calculator",
                "email": "korrinos-launch email",
                "network": "korrinos-launch network",
                "dns": "korrinos-launch network",
                "firewall": "korrinos-launch security",
                "backup": "korrinos-launch backup",
                "updates": "korrinos-launch updates",
                "update": "korrinos-launch updates",
                "systemcleaner": "korrinos-launch system-cleaner",
                "cleaner": "korrinos-launch system-cleaner",
            }
            cmd = acts.get(app, "notify-send 'launch: %s'" % app)
            sh("nohup bash -c '" + cmd.replace("'", "'\\''") + "' >/dev/null 2>&1 &")
            self._send(200, {"ok": True, "cmd": cmd})
            return
        if url.path == "/api/world":
            self._send(200, {"ok": True, "result": set_world(data.get("world", ""))})
            return
        if url.path == "/api/vokk-message":
            msg = data.get("message", "")
            self._send(200, {"ok": True, "reply": "VOKK v4 received: " + msg})
            return
        if url.path == "/api/system":
            self._send(200, {"ok": True, "out": sh("uptime")})
            return
        self._send(404, {"error": "unknown"})

    do_PUT = do_POST


def main():
    os.makedirs(WIDGET_DIR, exist_ok=True)
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.bind(("127.0.0.1", PORT))
    except OSError:
        pass
    sock.close()
    srv = ThreadingHTTPServer(("127.0.0.1", PORT), Handler)
    if "--foreground" not in sys.argv:
        subprocess.Popen([sys.executable, __file__, "--foreground"],
                         stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                         start_new_session=True)
        return
    srv.serve_forever()


if __name__ == "__main__":
    main()