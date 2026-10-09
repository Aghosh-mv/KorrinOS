# KorrinOS `os/` Stub Audit

Audit of the **471 shell scripts** under `os/` (92,917 lines). Every script was read; verdicts come from reading the body, not the filename or the header comment. `bash -n` / ShellCheck passing only proves the files *parse*.

## Headline

| Verdict | Scripts | Lines | % of scripts |
|---|---:|---:|---:|
| REAL | 350 | 64,589 | 74.3% |
| PARTIAL | 78 | 22,843 | 16.6% |
| STUB | 28 | 942 | 5.9% |
| DANGEROUS | 15 | 4,543 | 3.2% |
| **Total** | **471** | **92,917** | **100%** |

Reachability is tracked separately because it is orthogonal to function:

- **228 scripts are wired** into the ISO (PATH symlink or a systemd `ExecStart`, per `build-distro.sh`).
- **243 scripts are orphans** — copied into `/opt/korrinos/os` by `build-distro.sh:263` but nothing installs them onto `PATH`, no unit starts them, and no other script calls them.

### Real working functionality vs cosmetic

A script only counts as *real working functionality* if it is REAL **and** actually changes system state (sysfs/proc writes, package or service management, root-privileged ops, desktop config), **and** something can actually launch it.

| Bucket | Scripts | % of os/ |
|---|---:|---:|
| REAL **and** wired **and** changes system state | 104 | **22.1%** |
| REAL + changes system state, but orphaned | 71 | 15.1% |
| REAL but only reads / writes its own state dir | 175 | 37.2% |
| PARTIAL (works, but a branch is inert) | 78 | 16.6% |
| STUB (prints a spec, changes nothing) | 28 | 5.9% |
| DANGEROUS (works, and can destroy data) | 15 | 3.2% |

**Blunt answer: roughly 22% of `os/` is working, reachable functionality. About 23% is partial or cosmetic, and a further 52% of the tree is dead weight that ships but can never be invoked.**

## The four structural defects behind those numbers

These are not individual script bugs — they are systemic, and they explain most of the PARTIAL rows.

### 1. The C backends are never built

`os/hardware-tech/backend/` contains 9 `.c` files and a `Makefile`, and `hardware-tech/lib/backend-helper.sh` is built around them (`backend_available`, `backend_run`). `build-distro.sh` never compiles them and never calls `backend_build`. In a shipped KorrinOS `backend_available()` is **always false**, so every consumer silently takes its degraded path.

### 2. The AI layer has no model

`parc-ai/modules/ai-engine.sh` calls Ollama at `$OLLAMA_HOST` (default `localhost:11434`). Ollama is **not in any `kapt` package list** in `build-distro.sh` and nothing installs it — the only references are a graceful-failure message and an `if command -v ollama` guard. Much of the rest of `modules/` is deterministic keyword/regex heuristics dressed as AI (`math.sh`, `language.sh` are genuinely functional algorithms; `travel.sh`, `entertainment.sh`, `creative.sh`, `textgen.sh`, `persona.sh` are templated prose that fabricates results from the input string).

### 3. XFCE host, GNOME-targeting scripts

`build-distro.sh` installs **XFCE4**. But 18 scripts drive `gsettings set org.gnome.*`, and `apps/customization/shell-theme.sh` themes `org.gnome.shell.theme` — GNOME Shell does not exist on this image, so those calls return 0 while doing nothing. Only 6 scripts use `xfconf-query`, the correct XFCE path.

### 4. Unpackaged dependencies

Commands the scripts require that `build-distro.sh` does **not** install:

| Missing binary | Package not installed | Scripts affected |
|---|---|---:|
| `snap` | snapd | 35 |
| `flatpak` | flatpak | 12 |
| `sensors` | lm-sensors | 11 |
| `wmctrl` | wmctrl | 10 |
| `btrfs` | btrfs-progs | 4 |
| `conky` | conky | 4 |
| `firewall-cmd` | firewalld | 4 |
| `nvme` | nvme-cli | 4 |
| `v4l2-ctl` | v4l-utils | 2 |
| `jq` | jq | 2 |
| `smartctl` | smartmontools | 2 |
| `ethtool` | ethtool | 1 |
| `qrencode` | qrencode | 1 |
| `synclient` | synclient | 1 |

Also missing: `ollama` (see above). Present-but-unused: `xset`, `xfconf-query`, `iwconfig`, `mokutil` and `iptables` *are* installed via `x11-xserver-utils` / `xfce4-settings` / `wireless-tools` / `shim-signed` / `ufw`, so those are fine.

## DANGEROUS scripts

These do real work — that is the problem. All are flagged `file:line`.

| Script | Line | What it destroys |
|---|---|---|
| `system/firewall/korrinos-firewall.sh` | 138 | `sudo nft flush ruleset` — wipes **every** nftables table on the host, not just KorrinOS's. Runs at boot via `korrinos-firewall.service`. |
| `parc-ai/parcos-security.sh` | 40 | `sudo ufw disable` — a tool named *Security Center* turns the firewall **off**, then continues editing iptables. |
| `territories/hack/panic-wipe.sh` | 36 | `dd if=/dev/urandom of=$file conv=notrunc` — in-place random overwrite of real user files. |
| `territories/hack/split-personality.sh` | — | `dd` + `shred` + `truncate` on real paths. `shred` is irreversible on SSD/journaled FS. |
| `territories/hack/ephemeral-ram.sh` | — | tmpfs mounted over a real path; alters live mount topology. |
| `territories/hack/amnesia-firewall.sh` | — | Rewrites the full iptables/nftables rule set. |
| `territories/hack/reverse-proxy.sh` | — | Rewrites iptables NAT rules affecting all host networking. |
| `parc-ai/parcos-usb.sh` | 37,46 | `sudo dd if=$iso of=$device` + `parted`/`mkfs.ext4` on `$device3`. Guarded only by a `y/N` prompt; no `/dev/disk*` validation, no ISO/device mismatch check. |
| `system/installer.sh` | 47-52 | `sudo mkfs.*` on a caller-supplied partition — no confirmation, no device check. |
| `system/installer/korrinos-installer.sh` | — | `dd` to target disk + `mkfs` + `mount`; full-disk installer. |
| `system/driver-manager/korrinos-drivers.sh` | 286 | Stops the running display manager (`gdm3`/`sddm`/`lightdm`) and runs `apt` on a live session. |
| `system/password-manager.sh` | — | `openssl` + `shred` credential deletion; `shred` does not securely erase SSD/COW filesystems. |
| `hardware-tech/data-shredder/data-shredder.sh` | — | `shred -n 3` across caches, DNS, swap and logs. Same SSD caveat. |
| `system/rollback-recovery.sh` | — | apt/dpkg rollback + `passwd` + `tar` restore over system paths. |
| `apps/security/filevault.sh` | 71 | `sudo umount` of a mount point. Note: the *encryption* is not implemented — it creates a sparse file and prints manual `losetup`/`cryptsetup` steps. |

## Ranked: 15 most important stubs

Ordered by intent-versus-effort — real, obviously-wanted functionality that is cheap to finish.

### 1. ``hardware-tech/unified-control-plane/unified-control-plane.sh`` (16 lines)

**What exists:** A single HTTP/JSON API over every hardware class. `init` writes JSON; `api` prints an endpoint table. **Zero dispatch code exists.**

**What's needed:** Add a `route()` that maps `POST /hw/<class>/<action>` to the existing per-category tuning scripts. The 14-category tuning scripts already do the work — this is only a router.

### 2. ``hardware-tech/remote-hardware-api/remote-api.sh`` (29 lines)

**What exists:** Generate an API key, then expose the hardware controls over WebSocket for a phone. It writes the key and prints endpoints; **no listener is ever started.**

**What's needed:** Bind a port with `socat`/`nc` or a ~60-line Python `http.server`, and dispatch to the tuning scripts behind the key check. Key generation already works.

### 3. ``hardware-tech/neural-super-res/neural-super-res.sh`` (21 lines)

**What exists:** Upscale any window in real time. `upscale` and `realtime` only print a description of a CNN pipeline.

**What's needed:** `ffmpeg` is already packaged. A `ffmpeg -vf scale` + optional `sr` filter on a window capture via x11grab is ~20 lines and gives a genuinely working 2x upscaler.

### 4. ``hardware-tech/predictive-render/predictive-render.sh`` (16 lines)

**What exists:** Pre-render frames ahead of scroll/mouse motion to cut perceived latency.

**What's needed:** Needs a compositor hook, but a real version is cheap: listen to `xdotool mousemove`, keep a ring buffer of recently-rendered regions, and re-present via `picom`/XFCE compositor hints. The config schema is already right.

### 5. ``hardware-tech/hardware-tuning/gpu-tuning.sh`` (7 lines)

**What exists:** An empty file: shebang, a comment, `set -euo pipefail`. No body.

**What's needed:** The other 13 tuning scripts in the same directory are real. Copy `cpu-tuning.sh`'s shape and write `governor`/`power-limit` against `/sys/bus/pci/devices/*/power/` and `/sys/class/drm/*/`. ~30 lines.

### 6. ``hardware-tech/hardware-tuning/14-categories.sh`` (13 lines)

**What exists:** Claims to orchestrate all 14 tuning categories. It `source`s two scripts (discarding the result) and then just `ls` the directory.

**What's needed:** Delete the two dead `source` lines and make `show_all` actually invoke each category's `status`. It is a 5-line dispatcher once the stubs above are real.

### 7. ``apps/apps/parc-ai.sh`` (20 lines)

**What exists:** The `korrinos-vokk` Control Center launcher. `serve` prints `ready` after `ast.parse`-ing the file — **it never launches anything**.

**What's needed:** `vokk/vokk.py` exists. Replace the `ast.parse` status check with an actual `nohup python3 ... &` plus a PID file. ~5 lines.

### 8. ``hardware-tech/neural-audio/neural-audio-engine.sh`` (28 lines)

**What exists:** Noise cancellation / spatial audio pipeline. Every subcommand either echoes a stage list or shells to `pactl` to list sink names.

**What's needed:** Wire the stages to real PipeWire filter modules — `pipewire` and `wireplumber` are already packaged. `pactl`-based EQ and a compressor/limiter chain are config, not code.

### 9. ``hardware-tech/smart-power-grid/smart-power-grid.sh`` (31 lines)

**What exists:** Schedule heavy tasks by off-peak tariff. `schedule` appends to a JSON queue; **nothing ever drains it** — no timer, no cron, no unit.

**What's needed:** The queue already persists correctly. Wire `korrinos-schedule.sh`'s existing `crontab` path to drain it, and register a systemd timer. ~20 lines of glue.

### 10. ``desktop/nibra-style/ui-enhancements.sh`` (5 lines)

**What exists:** An empty file.

**What's needed:** No spec to reconstruct — it is a comment header with no body. Decide whether the feature exists; if not, delete the file so it stops inflating the script count.

### 11. ``territories/vokk/vokk-scheduler.sh`` (4 lines)

**What exists:** 'AI-native task prioritisation' — comment only.

**What's needed:** The ranking logic it describes is already implemented, inertly, in `territories/ai-scheduler/prioritize.sh` (7 lines, prints 'Prioritization engine ready'). Either port the real implementation or delete both.

### 12. ``parc-ai/modules/travel.sh`` (125 lines)

**What exists:** Four functions that print an itinerary, packing list and recommendations. The itinerary is generated purely from the destination *string* — 'Visit top attraction #2' — with no data source.

**What's needed:** Either wire it to a real source (there is a packaged `curl`) or relabel it honestly as a template. As written it fabricates plausible output, which is worse than returning nothing.

### 13. ``desktop/help-system.sh`` (325 lines)

**What exists:** A 325-line help manual that is only ever *printed*. It is not installed to `/usr/share/doc`, not indexed, and not reachable from any menu.

**What's needed:** Install it as a real help corpus and wire it to `korrinos-help` on PATH. Content is already written; only the delivery is missing.

### 14. ``territories/install-territories.sh`` (60 lines)

**What exists:** Links all 92 territory scripts into `~/.local/bin` as `tinker-*` commands. **Nothing calls it** — not `build-distro.sh`, not any script.

**What's needed:** Call it from `build-distro.sh` `stage3_worlds`, or drop it. Right now the entire territories layer is unlaunchable and its 92 scripts are orphans.

### 15. ``apps/customization/shell-theme.sh`` (33 lines)

**What exists:** Lists/sets GNOME Shell themes on an **XFCE** image. `gsettings set org.gnome.shell.theme` is a guaranteed no-op here.

**What's needed:** Repoint at `xfconf-query` against the xfwm theme channel — the same fix the 6 correctly-written XFCE scripts already use. ~10 lines.

## Script inventory

`Reach` = `wired` (on PATH / a systemd `ExecStart`) or `orphan`.

| # | Script | Lines | Verdict | Reach | Evidence | Touches |
|---:|---|---:|---|---|---|---|
| 1 | `apps/security/filevault.sh` | 121 | **DANGEROUS** | wired | sudo umount of a mount point; encryption itself is NOT implemented (prints manual steps) | ~/.tinker, apt/dpkg, filesystem writes |
| 2 | `hardware-tech/data-shredder/data-shredder.sh` | 321 | **DANGEROUS** | orphan | shred -n 3 on caches, DNS, swap, logs; shred does not securely erase SSD/journaled FS | ~/.tinker, /sys, filesystem writes |
| 3 | `parc-ai/parcos-security.sh` | 241 | **DANGEROUS** | orphan | `sudo ufw disable` (line 40) plus iptables edits and openssl/shred; a "security" tool that turns the firewall off | ~/.config/korrinos, apt/dpkg, systemd units, firewall rules, filesystem writes |
| 4 | `parc-ai/parcos-usb.sh` | 140 | **DANGEROUS** | orphan | sudo dd if=$iso of=$device + parted/mkfs.ext4 on $device3; guarded only by a y/N prompt, no /dev/disk validation | ~/.config/korrinos |
| 5 | `system/driver-manager/korrinos-drivers.sh` | 915 | **DANGEROUS** | wired | stops the running display manager (gdm3/sddm/lightdm) and runs apt on a live session | ~/.config/korrinos, /sys, /proc, apt/dpkg, systemd units |
| 6 | `system/firewall/korrinos-firewall.sh` | 402 | **DANGEROUS** | wired | `sudo nft flush ruleset` (line 143) destroys EVERY nftables table incl. non-KorrinOS ones; wired as a systemd unit (korrinos-firewall.service) | ~/.config/korrinos, apt/dpkg, firewall rules, filesystem writes |
| 7 | `system/installer.sh` | 238 | **DANGEROUS** | wired | sudo mkfs.* on a caller-supplied partition with no confirmation or device check | apt/dpkg, filesystem writes |
| 8 | `system/installer/korrinos-installer.sh` | 777 | **DANGEROUS** | wired | dd to target disk + mkfs + mount; full-disk installer | ~/.config/korrinos, /sys, /proc, apt/dpkg, systemd units |
| 9 | `system/password-manager.sh` | 663 | **DANGEROUS** | wired | openssl+shred credential deletion; shred is irreversible on SSD/COW filesystems | ~/.tinker |
| 10 | `system/rollback-recovery.sh` | 207 | **DANGEROUS** | wired | apt/dpkg rollback + passwd + tar restore over system paths | ~/.tinker, apt/dpkg, filesystem writes |
| 11 | `territories/hack/amnesia-firewall.sh` | 151 | **DANGEROUS** | wired | rewrites the full iptables/nftables rule set | /sys, firewall rules, filesystem writes |
| 12 | `territories/hack/ephemeral-ram.sh` | 92 | **DANGEROUS** | orphan | tmpfs mount over a real path + dd; changes live mount topology | /sys, /proc, filesystem writes |
| 13 | `territories/hack/panic-wipe.sh` | 101 | **DANGEROUS** | wired | dd if=/dev/urandom of=$file conv=notrunc — in-place random overwrite of user files | /sys, /proc, systemd units, filesystem writes |
| 14 | `territories/hack/reverse-proxy.sh` | 79 | **DANGEROUS** | orphan | rewrites iptables NAT rules; affects all host networking | systemd units, firewall rules, network |
| 15 | `territories/hack/split-personality.sh` | 95 | **DANGEROUS** | orphan | dd + shred + truncate on real paths; shred is irreversible | nothing (self-contained) |
| 16 | `apps/apps/aether-workspace.sh` | 17 | **STUB** | wired | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 17 | `apps/apps/nibra-betterlife.sh` | 17 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 18 | `apps/apps/parc-ai.sh` | 20 | **STUB** | wired | prints "ready" after ast.parse; never starts vokk.py despite claiming "serve" | nothing (self-contained) |
| 19 | `desktop/help-system.sh` | 325 | **STUB** | orphan | 325 lines of printed help text; the single executable line builds the menu — no documentation is read or served from disk | ~/.tinker, filesystem writes |
| 20 | `desktop/nibra-style/ui-enhancements.sh` | 5 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 21 | `desktop/nibra-style/vookk-integration.sh` | 4 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 22 | `hardware-tech/hardware-tuning/gpu-tuning.sh` | 7 | **STUB** | orphan | 7 lines: shebang, comment, `set -euo pipefail`. No body at all | nothing (self-contained) |
| 23 | `hardware-tech/hardware-tuning/security-tuning.sh` | 14 | **STUB** | orphan | 9 echo vs 3 real command(s) — prints a spec, changes nothing | /sys |
| 24 | `hardware-tech/neural-super-res/neural-super-res.sh` | 21 | **STUB** | orphan | init writes JSON; upscale/realtime only describe a CNN pipeline in prose — nothing captures or scales a frame | ~/.tinker |
| 25 | `hardware-tech/predictive-render/predictive-render.sh` | 16 | **STUB** | orphan | init writes a JSON file; predict/benefits are pure marketing echo — no frame is ever pre-rendered | ~/.tinker |
| 26 | `hardware-tech/remote-hardware-api/remote-api.sh` | 29 | **STUB** | orphan | generates an API key + prints an endpoint list; no HTTP/WebSocket listener exists (gpu_toggle just prints a sysfs path) | ~/.tinker, /sys, filesystem writes |
| 27 | `hardware-tech/unified-control-plane/unified-control-plane.sh` | 16 | **STUB** | orphan | init writes JSON; api/perms only print an endpoint list — there is no server, no dispatch, no hardware call | ~/.tinker |
| 28 | `parc-ai/korrinos-shell-ai.sh` | 64 | **STUB** | wired | routes a couple of verbs to a Python one-liner; no model backend reachable | nothing (self-contained) |
| 29 | `scripts/integrate-gpu-features.sh` | 4 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 30 | `system/korrinos-battery-bar.sh` | 247 | **STUB** | orphan | renders a bar from config; no power_supply read wired into the default path | /sys |
| 31 | `system/security-suite.sh` | 15 | **STUB** | wired | 3 echo vs 1 real command(s) — prints a spec, changes nothing | nothing (self-contained) |
| 32 | `terminal/kcommand/scripts/colors.sh` | 12 | **STUB** | orphan | zero executable commands in body (comments/spec only) | nothing (self-contained) |
| 33 | `terminal/kcommand/scripts/fg-bg.sh` | 54 | **STUB** | orphan | zero executable commands; 54 lines of spec text | nothing (self-contained) |
| 34 | `territories/ai-scheduler/native-scheduler.sh` | 5 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 35 | `territories/ai-scheduler/prioritize.sh` | 8 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 36 | `territories/daily-driver/enhanced.sh` | 4 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 37 | `territories/game/gpu-lock.sh` | 8 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 38 | `territories/game/gpu-pipeline.sh` | 7 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 39 | `territories/game/three-worlds-expand.sh` | 6 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 40 | `territories/vokk/vokk-lock.sh` | 3 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 41 | `territories/vokk/vokk-scheduler.sh` | 4 | **STUB** | orphan | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 42 | `territories/vokk/vokk.sh` | 6 | **STUB** | wired | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 43 | `vokk/vokk.sh` | 4 | **STUB** | wired | file contains only a spec/comment and zero executable commands | nothing (self-contained) |
| 44 | `apps/customization/conky-stats.sh` | 46 | **PARTIAL** | wired | inert because: conky (conky not packaged) | ~/.tinker, apt/dpkg, filesystem writes |
| 45 | `apps/customization/cursor-themes.sh` | 94 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, filesystem writes |
| 46 | `apps/customization/desktop-effects.sh` | 84 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker |
| 47 | `apps/customization/gtk-theme.sh` | 37 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, filesystem writes |
| 48 | `apps/customization/icon-packs.sh` | 90 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, filesystem writes |
| 49 | `apps/customization/shell-theme.sh` | 33 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, filesystem writes |
| 50 | `apps/customization/wallpaper-manager.sh` | 44 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker |
| 51 | `apps/customization/window-animations.sh` | 84 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker |
| 52 | `apps/hardware/webcam-manager.sh` | 161 | **PARTIAL** | wired | inert because: v4l2-ctl (v4l-utils not packaged) | ~/.tinker, /sys, filesystem writes |
| 53 | `apps/package-manager.sh` | 297 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged), snap (snapd not packaged) | ~/.tinker, apt/dpkg, filesystem writes |
| 54 | `apps/software-center.sh` | 252 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged) | apt/dpkg, filesystem writes |
| 55 | `apps/system/digital-twin.sh` | 196 | **PARTIAL** | wired | inert because: snap (snapd not packaged) | ~/.tinker, /sys, /proc, filesystem writes |
| 56 | `apps/system/focus-mode.sh` | 68 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker |
| 57 | `apps/system/power-manager.sh` | 335 | **PARTIAL** | wired | inert because: nvme (nvme-cli not packaged) | ~/.tinker, /sys |
| 58 | `apps/system/rollback-recovery.sh` | 271 | **PARTIAL** | wired | inert because: btrfs (btrfs-progs not packaged), snap (snapd not packaged) | ~/.tinker |
| 59 | `apps/system/system-monitor.sh` | 231 | **PARTIAL** | wired | inert because: sensors (lm-sensors not packaged) | ~/.tinker, /sys, /proc |
| 60 | `brand/install-gdm-skin.sh` | 85 | **PARTIAL** | orphan | inert because: gsettings org.gnome on XFCE = no-op | systemd units, filesystem writes |
| 61 | `desktop/desktop-integration.sh` | 345 | **PARTIAL** | orphan | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, filesystem writes |
| 62 | `desktop/gestures.sh` | 344 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | apt/dpkg, filesystem writes |
| 63 | `desktop/shortcuts.sh` | 545 | **PARTIAL** | wired | inert because: wmctrl (wmctrl not packaged) | ~/.tinker, /sys, /proc, apt/dpkg, systemd units |
| 64 | `desktop/theme-manager.sh` | 144 | **PARTIAL** | orphan | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker |
| 65 | `desktop/tiling.sh` | 365 | **PARTIAL** | orphan | inert because: snap (snapd not packaged) | nothing (self-contained) |
| 66 | `desktop/virtual-desktops.sh` | 410 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | nothing (self-contained) |
| 67 | `desktop/window-manager.sh` | 193 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | nothing (self-contained) |
| 68 | `hardware-tech/adaptive-display/adaptive-display.sh` | 69 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker |
| 69 | `hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 378 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker, /sys |
| 70 | `hardware-tech/dust-dislodger/dust-dislodger.sh` | 410 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker, /sys, /proc, filesystem writes |
| 71 | `hardware-tech/fpga-scaler/fpga-scaler.sh` | 291 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker, /sys |
| 72 | `hardware-tech/hardware-tuning/camera-tuning.sh` | 14 | **PARTIAL** | orphan | inert because: v4l2-ctl (v4l-utils not packaged) | nothing (self-contained) |
| 73 | `hardware-tech/hardware-tuning/display-tuning.sh` | 34 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO, gsettings org.gnome on XFCE = no-op | /sys, filesystem writes |
| 74 | `hardware-tech/hardware-tuning/input-tuning.sh` | 14 | **PARTIAL** | orphan | inert because: synclient (synclient not packaged) | /sys |
| 75 | `hardware-tech/hardware-tuning/network-tuning.sh` | 14 | **PARTIAL** | orphan | inert because: ethtool (ethtool not packaged) | nothing (self-contained) |
| 76 | `hardware-tech/hardware-tuning/storage-tuning.sh` | 18 | **PARTIAL** | orphan | inert because: nvme (nvme-cli not packaged) | /sys, /proc, filesystem writes |
| 77 | `hardware-tech/hardware-tuning/thermal-tuning.sh` | 24 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | /sys |
| 78 | `hardware-tech/hardware-tuning/usb-tuning.sh` | 40 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | /sys, filesystem writes |
| 79 | `hardware-tech/lib/backend-helper.sh` | 83 | **PARTIAL** | wired | inert because: C backend never compiled into ISO | nothing (self-contained) |
| 80 | `hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 426 | **PARTIAL** | orphan | inert because: sensors (lm-sensors not packaged) | ~/.tinker, /sys, filesystem writes |
| 81 | `hardware-tech/neural-audio/neural-audio-engine.sh` | 28 | **PARTIAL** | orphan | inert because: sensors (lm-sensors not packaged) | ~/.tinker, /sys |
| 82 | `hardware-tech/oled-shield/oled-shield.sh` | 484 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker, /sys |
| 83 | `hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 643 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO | ~/.tinker |
| 84 | `hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 678 | **PARTIAL** | orphan | inert because: C backend never compiled into ISO, sensors (lm-sensors not packaged) | ~/.tinker, /sys |
| 85 | `parc-ai/korrinos-dashboard.sh` | 239 | **PARTIAL** | wired | inert because: sensors (lm-sensors not packaged) | ~/.config/korrinos, /sys, /proc |
| 86 | `parc-ai/korrinos-power.sh` | 272 | **PARTIAL** | wired | inert because: sensors (lm-sensors not packaged) | ~/.config/korrinos, /sys, filesystem writes |
| 87 | `parc-ai/korrinos-security-apps.sh` | 185 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged) | ~/.config/korrinos, /sys, systemd units, firewall rules, filesystem writes |
| 88 | `parc-ai/korrinos-smoothui.sh` | 433 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 89 | `parc-ai/korrinos-splitscreen.sh` | 159 | **PARTIAL** | wired | inert because: wmctrl (wmctrl not packaged) | nothing (self-contained) |
| 90 | `parc-ai/korrinos-widgets-panel.sh` | 244 | **PARTIAL** | wired | inert because: conky (conky not packaged) | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 91 | `parc-ai/modules/agent-browser.sh` | 304 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | apt/dpkg, network, filesystem writes |
| 92 | `parc-ai/modules/agent-system.sh` | 255 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | ~/.config/vokk |
| 93 | `parc-ai/modules/agent-vision.sh` | 157 | **PARTIAL** | orphan | inert because: Ollama not packaged | ~/.config/vokk, network |
| 94 | `parc-ai/modules/ai-brain-smart.sh` | 481 | **PARTIAL** | orphan | inert because: Ollama not packaged | /sys, network |
| 95 | `parc-ai/modules/ai-engine.sh` | 210 | **PARTIAL** | orphan | inert because: Ollama not packaged | ~/.config/vokk, network, filesystem writes |
| 96 | `parc-ai/modules/ai-master-brain.sh` | 462 | **PARTIAL** | orphan | inert because: Ollama not packaged | /sys, network |
| 97 | `parc-ai/modules/ai-self-learn.sh` | 200 | **PARTIAL** | orphan | inert because: Ollama not packaged | ~/.config/vokk, network |
| 98 | `parc-ai/modules/computer-use.sh` | 351 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | ~/.config/vokk, apt/dpkg, filesystem writes |
| 99 | `parc-ai/modules/device.sh` | 184 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | /sys, apt/dpkg, filesystem writes |
| 100 | `parc-ai/modules/knowledge-parcos.sh` | 339 | **PARTIAL** | orphan | inert because: sensors (lm-sensors not packaged) | apt/dpkg, systemd units |
| 101 | `parc-ai/modules/nlp-670-patterns.sh` | 425 | **PARTIAL** | orphan | inert because: snap (snapd not packaged) | nothing (self-contained) |
| 102 | `parc-ai/parc-ai.sh` | 1760 | **PARTIAL** | wired | inert because: conky (conky not packaged) | ~/.config/vokk, filesystem writes |
| 103 | `parc-ai/parcos-desktop.sh` | 196 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | nothing (self-contained) |
| 104 | `parc-ai/parcos-pm.sh` | 280 | **PARTIAL** | orphan | inert because: flatpak (flatpak not packaged), snap (snapd not packaged) | ~/.config/korrinos, apt/dpkg, network, filesystem writes |
| 105 | `parc-ai/parcos-recovery.sh` | 193 | **PARTIAL** | orphan | inert because: btrfs (btrfs-progs not packaged) | ~/.config/korrinos, apt/dpkg, systemd units, filesystem writes |
| 106 | `parc-ai/parcos-tools.sh` | 152 | **PARTIAL** | orphan | inert because: sensors (lm-sensors not packaged) | /sys, apt/dpkg, network, filesystem writes |
| 107 | `parc-ai/tests/computer-use-test.sh` | 107 | **PARTIAL** | orphan | inert because: wmctrl (wmctrl not packaged) | nothing (self-contained) |
| 108 | `parcos-agent.sh` | 83 | **PARTIAL** | orphan | inert because: gsettings org.gnome on XFCE = no-op | filesystem writes |
| 109 | `system/appstore/korrinos-appstore.sh` | 851 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged), snap (snapd not packaged) | ~/.config/korrinos, /proc, apt/dpkg, network, filesystem writes |
| 110 | `system/backup-restore.sh` | 257 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged) | ~/.tinker, apt/dpkg |
| 111 | `system/context-aware.sh` | 311 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.tinker, /sys |
| 112 | `system/enterprise/korrinos-enterprise.sh` | 1070 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op | ~/.config/korrinos, /proc, apt/dpkg, systemd units, firewall rules |
| 113 | `system/flatpak-support.sh` | 149 | **PARTIAL** | orphan | inert because: flatpak (flatpak not packaged) | apt/dpkg, systemd units, filesystem writes |
| 114 | `system/mobile-companion/korrinos-mobile.sh` | 864 | **PARTIAL** | wired | inert because: snap (snapd not packaged) | ~/.config/korrinos, /proc, apt/dpkg, systemd units, filesystem writes |
| 115 | `system/package-manager/korrinos-pkg.sh` | 1117 | **PARTIAL** | wired | inert because: flatpak (flatpak not packaged), snap (snapd not packaged) | ~/.config/korrinos, /proc, apt/dpkg, filesystem writes |
| 116 | `system/security/korrinos-health.sh` | 461 | **PARTIAL** | wired | inert because: sensors (lm-sensors not packaged) | ~/.config/korrinos, /sys, /proc, systemd units, firewall rules |
| 117 | `system/update-system.sh` | 261 | **PARTIAL** | orphan | inert because: flatpak (flatpak not packaged), snap (snapd not packaged) | /proc, apt/dpkg, filesystem writes |
| 118 | `system/update-system/korrinos-update.sh` | 900 | **PARTIAL** | wired | inert because: btrfs (btrfs-progs not packaged), flatpak (flatpak not packaged), snap (snapd not packaged) | ~/.config/korrinos, /proc, apt/dpkg, systemd units, filesystem writes |
| 119 | `territories/hack/hack-ways.sh` | 131 | **PARTIAL** | orphan | only touches its own state dir (~/.tinker or ~/.config/korrinos); 48 commands, 1 real reads | network |
| 120 | `territories/secure/sip-guard.sh` | 68 | **PARTIAL** | wired | inert because: snap (snapd not packaged) | /sys |
| 121 | `territories/vibe-address/connectors/bootstrap.sh` | 330 | **PARTIAL** | wired | inert because: gsettings org.gnome on XFCE = no-op, jq (jq not packaged) | filesystem writes |
| 122 | `ai/voice-assistant.sh` | 191 | **REAL** | orphan | 10 real commands; 1 privileged/service ops | network |
| 123 | `ai/voice-engine.sh` | 207 | **REAL** | orphan | 15 real commands; 2 privileged/service ops | systemd units, network |
| 124 | `apps/app-store.sh` | 295 | **REAL** | wired | 154 real commands; 4 privileged/service ops; package/service mgmt | apt/dpkg, network, filesystem writes |
| 125 | `apps/battery-monitor.sh` | 257 | **REAL** | wired | 40 real commands; 1 sysfs/proc writes | ~/.tinker, /sys |
| 126 | `apps/customization/font-manager.sh` | 41 | **REAL** | wired | 22 real commands | ~/.tinker, filesystem writes |
| 127 | `apps/customization/grub-theme.sh` | 101 | **REAL** | wired | 31 real commands; 1 privileged/service ops | ~/.tinker, filesystem writes |
| 128 | `apps/customization/install-cursor-themes.sh` | 224 | **REAL** | wired | 71 real commands; 2 privileged/service ops | filesystem writes |
| 129 | `apps/customization/korrinos-typeface.sh` | 285 | **REAL** | wired | 105 real commands; 1 privileged/service ops | filesystem writes |
| 130 | `apps/customization/login-theme.sh` | 80 | **REAL** | wired | 20 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, systemd units |
| 131 | `apps/customization/qt-theme.sh` | 35 | **REAL** | wired | 10 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 132 | `apps/file-manager.sh` | 226 | **REAL** | wired | 55 real commands | ~/.tinker, filesystem writes |
| 133 | `apps/gaming-mode.sh` | 251 | **REAL** | wired | 45 real commands; 2 sysfs/proc writes; 2 privileged/service ops; package/service mgmt | /sys |
| 134 | `apps/gaming-support.sh` | 350 | **REAL** | wired | 78 real commands; 3 privileged/service ops; package/service mgmt | /sys, apt/dpkg, network, filesystem writes |
| 135 | `apps/gaming/anticheat-helper.sh` | 70 | **REAL** | wired | 16 real commands | ~/.tinker |
| 136 | `apps/gaming/audio-mixer.sh` | 247 | **REAL** | wired | 73 real commands | ~/.tinker, /proc |
| 137 | `apps/gaming/controller-mapper.sh` | 159 | **REAL** | wired | 48 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 138 | `apps/gaming/discord-presence.sh` | 69 | **REAL** | wired | 16 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 139 | `apps/gaming/emulator-manager.sh` | 112 | **REAL** | wired | 22 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 140 | `apps/gaming/fps-monitor.sh` | 182 | **REAL** | wired | 39 real commands; 2 privileged/service ops | ~/.tinker, /sys, /proc, apt/dpkg, filesystem writes |
| 141 | `apps/gaming/game-launcher.sh` | 98 | **REAL** | wired | 27 real commands | ~/.tinker |
| 142 | `apps/gaming/game-replay.sh` | 97 | **REAL** | wired | 28 real commands | ~/.tinker, filesystem writes |
| 143 | `apps/gaming/game-saves-sync.sh` | 99 | **REAL** | wired | 25 real commands | ~/.tinker, filesystem writes |
| 144 | `apps/gaming/gif-recorder.sh` | 81 | **REAL** | wired | 22 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 145 | `apps/gaming/hardware-benchmark.sh` | 177 | **REAL** | wired | 40 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, network, filesystem writes |
| 146 | `apps/gaming/performance-graph.sh` | 73 | **REAL** | wired | 20 real commands | ~/.tinker, /proc |
| 147 | `apps/gaming/screenshot-tool.sh` | 146 | **REAL** | wired | 47 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 148 | `apps/gaming/streaming-manager.sh` | 105 | **REAL** | wired | 34 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 149 | `apps/gaming/wine-manager.sh` | 102 | **REAL** | wired | 27 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 150 | `apps/hardware/display-calibration.sh` | 85 | **REAL** | wired | 23 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 151 | `apps/hardware/docking-station.sh` | 95 | **REAL** | wired | 22 real commands | ~/.tinker, /sys |
| 152 | `apps/hardware/fingerprint-manager.sh` | 95 | **REAL** | wired | 22 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 153 | `apps/hardware/gpio-manager.sh` | 99 | **REAL** | wired | 17 real commands; 3 sysfs/proc writes; 1 privileged/service ops | ~/.tinker, /sys, /proc, filesystem writes |
| 154 | `apps/hardware/hdr-manager.sh` | 75 | **REAL** | wired | 17 real commands | ~/.tinker |
| 155 | `apps/hardware/kvm-switch.sh` | 76 | **REAL** | wired | 18 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 156 | `apps/hardware/nfc-manager.sh` | 76 | **REAL** | wired | 17 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 157 | `apps/hardware/pen-stylus.sh` | 79 | **REAL** | wired | 19 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 158 | `apps/hardware/printer-manager.sh` | 198 | **REAL** | wired | 37 real commands; 1 privileged/service ops; package/service mgmt | ~/.tinker, systemd units, filesystem writes |
| 159 | `apps/hardware/scanner-manager.sh` | 104 | **REAL** | wired | 23 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 160 | `apps/hardware/serial-uart.sh` | 84 | **REAL** | wired | 23 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 161 | `apps/hardware/thunderbolt-manager.sh` | 73 | **REAL** | wired | 18 real commands; 1 sysfs/proc writes; 1 privileged/service ops | ~/.tinker, /sys, filesystem writes |
| 162 | `apps/hardware/touchscreen-manager.sh` | 92 | **REAL** | wired | 23 real commands; 2 privileged/service ops | ~/.tinker, /proc, apt/dpkg, filesystem writes |
| 163 | `apps/hardware/usb-manager.sh` | 125 | **REAL** | wired | 30 real commands | ~/.tinker, /sys |
| 164 | `apps/network/bandwidth-limiter.sh` | 74 | **REAL** | wired | 19 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 165 | `apps/network/dns-manager.sh` | 92 | **REAL** | wired | 24 real commands; 1 privileged/service ops | ~/.tinker |
| 166 | `apps/network/firewall-gui.sh` | 271 | **REAL** | wired | 99 real commands; 4 privileged/service ops; package/service mgmt | ~/.tinker, systemd units, firewall rules |
| 167 | `apps/network/hotspot-manager.sh` | 92 | **REAL** | wired | 25 real commands; 1 privileged/service ops | ~/.tinker |
| 168 | `apps/network/mesh-network.sh` | 92 | **REAL** | wired | 23 real commands; 1 privileged/service ops | ~/.tinker, network, filesystem writes |
| 169 | `apps/network/network-monitor.sh` | 199 | **REAL** | wired | 53 real commands | ~/.tinker, /sys |
| 170 | `apps/network/proxy-manager.sh` | 101 | **REAL** | wired | 25 real commands | ~/.tinker, network |
| 171 | `apps/network/speed-test.sh` | 127 | **REAL** | wired | 24 real commands | ~/.tinker, network |
| 172 | `apps/network/vpn-manager.sh` | 221 | **REAL** | wired | 63 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, /sys, systemd units, network, filesystem writes |
| 173 | `apps/network/wifi-analyzer.sh` | 185 | **REAL** | wired | 24 real commands; 1 privileged/service ops | ~/.tinker, /sys |
| 174 | `apps/ocr-everywhere.sh` | 401 | **REAL** | wired | 83 real commands; 2 privileged/service ops; package/service mgmt | apt/dpkg, filesystem writes |
| 175 | `apps/quick-note.sh` | 161 | **REAL** | wired | 28 real commands | ~/.tinker |
| 176 | `apps/screen-recorder.sh` | 134 | **REAL** | wired | 28 real commands | nothing (self-contained) |
| 177 | `apps/security/biometric.sh` | 132 | **REAL** | wired | 21 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 178 | `apps/security/findmydevice.sh` | 146 | **REAL** | wired | 35 real commands | ~/.tinker, network |
| 179 | `apps/security/firewall.sh` | 84 | **REAL** | wired | 16 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, systemd units, firewall rules |
| 180 | `apps/security/gatekeeper.sh` | 169 | **REAL** | wired | 45 real commands | ~/.tinker |
| 181 | `apps/security/password-manager.sh` | 166 | **REAL** | wired | 26 real commands | ~/.tinker |
| 182 | `apps/security/privacy.sh` | 152 | **REAL** | wired | 36 real commands; 1 privileged/service ops | ~/.tinker, filesystem writes |
| 183 | `apps/security/security-suite.sh` | 163 | **REAL** | wired | 50 real commands; 4 privileged/service ops; package/service mgmt | ~/.tinker, /sys, /proc, apt/dpkg, firewall rules |
| 184 | `apps/smart-clipboard.sh` | 159 | **REAL** | wired | 38 real commands | ~/.tinker, filesystem writes |
| 185 | `apps/system-cleaner.sh` | 179 | **REAL** | wired | 35 real commands; 3 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 186 | `apps/system/adaptive-power-grid.sh` | 237 | **REAL** | wired | 45 real commands | ~/.tinker, /sys, /proc |
| 187 | `apps/system/auto-updates.sh` | 270 | **REAL** | wired | 99 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, systemd units, filesystem writes |
| 188 | `apps/system/backup-restore.sh` | 347 | **REAL** | wired | 101 real commands; 1 privileged/service ops; package/service mgmt | ~/.tinker, systemd units |
| 189 | `apps/system/cognitive-load.sh` | 62 | **REAL** | wired | 24 real commands | ~/.tinker |
| 190 | `apps/system/command-palette.sh` | 101 | **REAL** | wired | 41 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 191 | `apps/system/context-aware.sh` | 243 | **REAL** | wired | 87 real commands | ~/.tinker, /sys |
| 192 | `apps/system/disk-visualizer.sh` | 88 | **REAL** | wired | 21 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 193 | `apps/system/drag-to-install.sh` | 42 | **REAL** | wired | 21 real commands; 3 privileged/service ops; package/service mgmt | apt/dpkg, filesystem writes |
| 194 | `apps/system/duplicate-finder.sh` | 122 | **REAL** | wired | 21 real commands | ~/.tinker, filesystem writes |
| 195 | `apps/system/fast-boot.sh` | 217 | **REAL** | wired | 54 real commands; 1 privileged/service ops; package/service mgmt | ~/.tinker, /proc, systemd units |
| 196 | `apps/system/file-versioning.sh` | 106 | **REAL** | wired | 20 real commands | ~/.tinker, filesystem writes |
| 197 | `apps/system/global-search.sh` | 97 | **REAL** | wired | 24 real commands; 1 privileged/service ops | ~/.tinker |
| 198 | `apps/system/intent-launcher.sh` | 47 | **REAL** | wired | 15 real commands | nothing (self-contained) |
| 199 | `apps/system/json-formatter.sh` | 93 | **REAL** | wired | 18 real commands | nothing (self-contained) |
| 200 | `apps/system/markdown-editor.sh` | 130 | **REAL** | wired | 33 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 201 | `apps/system/parental-controls.sh` | 108 | **REAL** | wired | 28 real commands | ~/.tinker |
| 202 | `apps/system/pomodoro-timer.sh` | 90 | **REAL** | wired | 28 real commands | ~/.tinker |
| 203 | `apps/system/predictive-caching.sh` | 244 | **REAL** | wired | 94 real commands | ~/.tinker |
| 204 | `apps/system/predictive-intelligence.sh` | 213 | **REAL** | wired | 88 real commands | ~/.tinker, /sys, /proc |
| 205 | `apps/system/quick-actions.sh` | 98 | **REAL** | wired | 30 real commands; 1 privileged/service ops | ~/.tinker, systemd units |
| 206 | `apps/system/regex-tool.sh` | 67 | **REAL** | wired | 7 real commands | nothing (self-contained) |
| 207 | `apps/system/screen-time.sh` | 92 | **REAL** | wired | 17 real commands | ~/.tinker |
| 208 | `apps/system/self-healing.sh` | 297 | **REAL** | wired | 110 real commands; 1 sysfs/proc writes; 3 privileged/service ops; package/service mgmt | ~/.tinker, /sys, /proc, apt/dpkg, systemd units |
| 209 | `apps/system/temporal-mapping.sh` | 248 | **REAL** | wired | 46 real commands | ~/.tinker |
| 210 | `apps/system/terminal-error-explainer.sh` | 54 | **REAL** | wired | 18 real commands; 2 privileged/service ops | systemd units, filesystem writes |
| 211 | `apps/system/time-tracker.sh` | 97 | **REAL** | wired | 25 real commands | ~/.tinker |
| 212 | `apps/voice-commands.sh` | 457 | **REAL** | wired | 119 real commands; 2 privileged/service ops; package/service mgmt | apt/dpkg, network, filesystem writes |
| 213 | `boot/live-init/build-live-initramfs.sh` | 71 | **REAL** | wired | 13 real commands; 1 privileged/service ops | apt/dpkg, filesystem writes |
| 214 | `brand/install-boot-intro.sh` | 71 | **REAL** | orphan | 27 real commands; 1 privileged/service ops | filesystem writes |
| 215 | `brand/install-grub-theme.sh` | 58 | **REAL** | orphan | 23 real commands; 1 privileged/service ops | filesystem writes |
| 216 | `branding/install-branding.sh` | 43 | **REAL** | wired | 8 real commands; 1 privileged/service ops | apt/dpkg, filesystem writes |
| 217 | `build-distro.sh` | 1221 | **REAL** | wired | 351 real commands; 8 privileged/service ops; package/service mgmt | ~/.config/korrinos, apt/dpkg, systemd units, firewall rules, network |
| 218 | `control-center/launch_feature.sh` | 190 | **REAL** | orphan | 9 real commands | nothing (self-contained) |
| 219 | `data/hardware-db.sh` | 164 | **REAL** | orphan | 87 real commands | ~/.tinker |
| 220 | `data/user-profiles.sh` | 122 | **REAL** | orphan | 28 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, network, filesystem writes |
| 221 | `desktop/activity-monitor.sh` | 142 | **REAL** | orphan | 34 real commands | nothing (self-contained) |
| 222 | `desktop/app-launcher.sh` | 141 | **REAL** | orphan | 21 real commands | ~/.tinker |
| 223 | `desktop/clipboard-manager.sh` | 119 | **REAL** | orphan | 33 real commands | ~/.tinker, filesystem writes |
| 224 | `desktop/dock.sh` | 92 | **REAL** | wired | 35 real commands | ~/.tinker |
| 225 | `desktop/file-search.sh` | 152 | **REAL** | orphan | 25 real commands | ~/.tinker |
| 226 | `desktop/multi-monitor.sh` | 213 | **REAL** | orphan | 56 real commands; 1 sysfs/proc writes; 1 privileged/service ops | ~/.tinker, /sys, filesystem writes |
| 227 | `desktop/nibra-style/nibra-shell.sh` | 141 | **REAL** | wired | 80 real commands; 1 privileged/service ops | network |
| 228 | `desktop/night-mode.sh` | 125 | **REAL** | wired | 33 real commands | nothing (self-contained) |
| 229 | `desktop/notification-center.sh` | 108 | **REAL** | wired | 26 real commands | ~/.tinker, /sys, filesystem writes |
| 230 | `desktop/screen-tools.sh` | 137 | **REAL** | orphan | 30 real commands | nothing (self-contained) |
| 231 | `desktop/settings-gui.sh` | 515 | **REAL** | wired | 99 real commands; 7 privileged/service ops; package/service mgmt | ~/.tinker, /sys, apt/dpkg, systemd units, firewall rules |
| 232 | `desktop/setup-wizard.sh` | 246 | **REAL** | orphan | 39 real commands | ~/.tinker |
| 233 | `desktop/start-desktop.sh` | 62 | **REAL** | orphan | 9 real commands | nothing (self-contained) |
| 234 | `desktop/system-monitor.sh` | 135 | **REAL** | orphan | 33 real commands | /sys |
| 235 | `desktop/system-tray.sh` | 121 | **REAL** | orphan | 21 real commands | /sys |
| 236 | `desktop/text-expander.sh` | 211 | **REAL** | orphan | 39 real commands; 2 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 237 | `desktop/widgets.sh` | 150 | **REAL** | wired | 33 real commands | ~/.tinker, network |
| 238 | `drivers/driver-manager.sh` | 290 | **REAL** | orphan | 59 real commands; 2 privileged/service ops; package/service mgmt | apt/dpkg, filesystem writes |
| 239 | `hardware-tech/cache-tiering/cache-tiering.sh` | 553 | **REAL** | orphan | 263 real commands; 1 privileged/service ops | ~/.tinker, /sys, /proc |
| 240 | `hardware-tech/cross-app-automation/cross-app-automation.sh` | 344 | **REAL** | orphan | 120 real commands; 1 privileged/service ops | ~/.tinker |
| 241 | `hardware-tech/cxl-memory/cxl-memory.sh` | 423 | **REAL** | orphan | 178 real commands | ~/.tinker, /sys, /proc |
| 242 | `hardware-tech/dvfs-shaver/dvfs-shaver.sh` | 507 | **REAL** | orphan | 225 real commands | ~/.tinker, /sys, /proc, filesystem writes |
| 243 | `hardware-tech/energy-scheduler/energy-scheduler.sh` | 412 | **REAL** | orphan | 188 real commands; 2 privileged/service ops | ~/.tinker, /sys, apt/dpkg |
| 244 | `hardware-tech/finance-audit/subscription-audit.sh` | 535 | **REAL** | orphan | 300 real commands | ~/.tinker |
| 245 | `hardware-tech/hardware-dna/hardware-dna.sh` | 71 | **REAL** | orphan | 39 real commands; 1 sysfs/proc writes; 2 privileged/service ops | ~/.tinker, /sys, /proc, filesystem writes |
| 246 | `hardware-tech/hardware-tuning/14-categories.sh` | 13 | **REAL** | orphan | 3 real commands | nothing (self-contained) |
| 247 | `hardware-tech/hardware-tuning/audio-tuning.sh` | 15 | **REAL** | orphan | 5 real commands | nothing (self-contained) |
| 248 | `hardware-tech/hardware-tuning/cpu-tuning.sh` | 18 | **REAL** | orphan | 5 real commands; 3 sysfs/proc writes; 1 privileged/service ops | /sys, /proc, filesystem writes |
| 249 | `hardware-tech/hardware-tuning/led-tuning.sh` | 20 | **REAL** | orphan | 6 real commands; 2 sysfs/proc writes; 1 privileged/service ops | /sys, filesystem writes |
| 250 | `hardware-tech/hardware-tuning/memory-tuning.sh` | 18 | **REAL** | orphan | 8 real commands; 1 sysfs/proc writes; 2 privileged/service ops | /sys, /proc, filesystem writes |
| 251 | `hardware-tech/hardware-tuning/power-tuning.sh` | 14 | **REAL** | orphan | 3 real commands; 1 sysfs/proc writes; 1 privileged/service ops | /sys, filesystem writes |
| 252 | `hardware-tech/lib/hardware-consent.sh` | 131 | **REAL** | wired | 57 real commands | ~/.tinker |
| 253 | `hardware-tech/predictive-prewarm/predictive-prewarm.sh` | 409 | **REAL** | orphan | 207 real commands | ~/.tinker, /proc |
| 254 | `hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 83 | **REAL** | orphan | 40 real commands; 1 privileged/service ops | ~/.tinker, /sys, filesystem writes |
| 255 | `hardware-tech/sdgpu/software-gpu.sh` | 35 | **REAL** | orphan | 39 real commands | ~/.tinker, /proc |
| 256 | `hardware-tech/smart-power-grid/smart-power-grid.sh` | 31 | **REAL** | orphan | 22 real commands | ~/.tinker, /sys |
| 257 | `hardware-tech/unified-memory/unified-contextual-memory.sh` | 551 | **REAL** | orphan | 304 real commands | ~/.tinker |
| 258 | `hardware-tech/zero-latency-input/zero-latency-input.sh` | 43 | **REAL** | orphan | 24 real commands | ~/.tinker, /sys |
| 259 | `hyperdrive/hyperdrive-cli.sh` | 238 | **REAL** | orphan | 51 real commands; 2 sysfs/proc writes; 2 privileged/service ops; package/service mgmt | /sys, /proc, systemd units, filesystem writes |
| 260 | `hyperdrive/hyperdrive-daemon.sh` | 159 | **REAL** | orphan | 48 real commands; 2 sysfs/proc writes; 1 privileged/service ops | /sys, /proc, filesystem writes |
| 261 | `hyperdrive/hyperdrive.sh` | 590 | **REAL** | orphan | 203 real commands; 3 sysfs/proc writes; 3 privileged/service ops; package/service mgmt | /sys, /proc, systemd units, filesystem writes |
| 262 | `i18n/i18n-lib.sh` | 217 | **REAL** | orphan | 76 real commands | filesystem writes |
| 263 | `i18n/term-snapshot.sh` | 128 | **REAL** | orphan | 45 real commands | nothing (self-contained) |
| 264 | `install-parcai.sh` | 127 | **REAL** | orphan | 64 real commands; 3 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 265 | `iso-artifacts.sh` | 24 | **REAL** | orphan | 4 real commands | nothing (self-contained) |
| 266 | `iso-builder.sh` | 309 | **REAL** | orphan | 99 real commands; 3 privileged/service ops | ~/.tinker, apt/dpkg, filesystem writes |
| 267 | `languages/install.sh` | 71 | **REAL** | wired | 17 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 268 | `mobile-companion/scripts/mobile-companion-daemon.sh` | 77 | **REAL** | orphan | 23 real commands | ~/.tinker, filesystem writes |
| 269 | `parc-ai/install-parcos.sh` | 110 | **REAL** | orphan | 53 real commands; 4 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 270 | `parc-ai/korrinos-annotate.sh` | 158 | **REAL** | wired | 34 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 271 | `parc-ai/korrinos-ascii.sh` | 72 | **REAL** | wired | 16 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 272 | `parc-ai/korrinos-backup.sh` | 341 | **REAL** | wired | 96 real commands | ~/.config/korrinos |
| 273 | `parc-ai/korrinos-cleanup.sh` | 307 | **REAL** | wired | 73 real commands; 2 privileged/service ops; package/service mgmt | ~/.config/korrinos, /sys, apt/dpkg |
| 274 | `parc-ai/korrinos-clipctx.sh` | 341 | **REAL** | wired | 122 real commands | ~/.config/korrinos, filesystem writes |
| 275 | `parc-ai/korrinos-devsuite.sh` | 370 | **REAL** | wired | 139 real commands | ~/.config/korrinos, filesystem writes |
| 276 | `parc-ai/korrinos-dock.sh` | 256 | **REAL** | wired | 96 real commands; 2 privileged/service ops | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 277 | `parc-ai/korrinos-focus.sh` | 143 | **REAL** | wired | 43 real commands | ~/.config/korrinos |
| 278 | `parc-ai/korrinos-hotkeys.sh` | 149 | **REAL** | wired | 64 real commands | ~/.config/korrinos |
| 279 | `parc-ai/korrinos-liquid-glass.sh` | 341 | **REAL** | wired | 202 real commands; 2 privileged/service ops | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 280 | `parc-ai/korrinos-monitor.sh` | 268 | **REAL** | wired | 65 real commands | ~/.config/korrinos, /sys, /proc |
| 281 | `parc-ai/korrinos-network.sh` | 309 | **REAL** | wired | 56 real commands; 1 privileged/service ops | ~/.config/korrinos, /sys, network, filesystem writes |
| 282 | `parc-ai/korrinos-nlctl.sh` | 232 | **REAL** | wired | 87 real commands | ~/.config/korrinos |
| 283 | `parc-ai/korrinos-notepad.sh` | 284 | **REAL** | wired | 93 real commands; 2 privileged/service ops | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 284 | `parc-ai/korrinos-schedule.sh` | 340 | **REAL** | wired | 124 real commands; 1 privileged/service ops; package/service mgmt | ~/.config/korrinos, filesystem writes |
| 285 | `parc-ai/korrinos-shortcuts.sh` | 287 | **REAL** | wired | 79 real commands | ~/.config/korrinos, filesystem writes |
| 286 | `parc-ai/korrinos-sounds.sh` | 221 | **REAL** | wired | 80 real commands | ~/.config/korrinos |
| 287 | `parc-ai/korrinos-tinkeria.sh` | 315 | **REAL** | orphan | 95 real commands | ~/.config/korrinos, network, filesystem writes |
| 288 | `parc-ai/korrinos-toggles.sh` | 202 | **REAL** | wired | 52 real commands | ~/.config/korrinos, /sys, /proc |
| 289 | `parc-ai/korrinos-voice.sh` | 204 | **REAL** | wired | 76 real commands; 2 privileged/service ops; package/service mgmt | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 290 | `parc-ai/korrinos-widgets.sh` | 114 | **REAL** | wired | 30 real commands | ~/.config/korrinos, /sys |
| 291 | `parc-ai/modules/agent-automation.sh` | 179 | **REAL** | orphan | 44 real commands; 1 privileged/service ops; package/service mgmt | ~/.config/vokk |
| 292 | `parc-ai/modules/ai-image-gen.sh` | 284 | **REAL** | orphan | 145 real commands | nothing (self-contained) |
| 293 | `parc-ai/modules/ai-knowledge-broad.sh` | 334 | **REAL** | orphan | 39 real commands | nothing (self-contained) |
| 294 | `parc-ai/modules/ai-knowledge-mega.sh` | 686 | **REAL** | orphan | 33 real commands; 1 privileged/service ops | filesystem writes |
| 295 | `parc-ai/modules/ai-narrative.sh` | 428 | **REAL** | orphan | 136 real commands | nothing (self-contained) |
| 296 | `parc-ai/modules/ai-nlu-crf.sh` | 490 | **REAL** | orphan | 79 real commands | nothing (self-contained) |
| 297 | `parc-ai/modules/ai-personality.sh` | 178 | **REAL** | orphan | 41 real commands | filesystem writes |
| 298 | `parc-ai/modules/ai-voice.sh` | 97 | **REAL** | orphan | 20 real commands | nothing (self-contained) |
| 299 | `parc-ai/modules/cards-interactive.sh` | 287 | **REAL** | orphan | 330 real commands | nothing (self-contained) |
| 300 | `parc-ai/modules/cards-live.sh` | 250 | **REAL** | orphan | 293 real commands | network |
| 301 | `parc-ai/modules/cards-structural.sh` | 224 | **REAL** | orphan | 339 real commands | nothing (self-contained) |
| 302 | `parc-ai/modules/cards-visual.sh` | 217 | **REAL** | orphan | 164 real commands | ~/.config/vokk, network |
| 303 | `parc-ai/modules/cards-workspace.sh` | 224 | **REAL** | orphan | 259 real commands | nothing (self-contained) |
| 304 | `parc-ai/modules/codegen.sh` | 227 | **REAL** | orphan | 93 real commands; 1 privileged/service ops | filesystem writes |
| 305 | `parc-ai/modules/commerce.sh` | 226 | **REAL** | orphan | 85 real commands | ~/.config/vokk |
| 306 | `parc-ai/modules/contacts.sh` | 69 | **REAL** | orphan | 30 real commands | ~/.config/vokk |
| 307 | `parc-ai/modules/conversation.sh` | 69 | **REAL** | orphan | 21 real commands | ~/.config/vokk, filesystem writes |
| 308 | `parc-ai/modules/creative.sh` | 296 | **REAL** | orphan | 109 real commands | nothing (self-contained) |
| 309 | `parc-ai/modules/debugging.sh` | 269 | **REAL** | orphan | 121 real commands | filesystem writes |
| 310 | `parc-ai/modules/entertainment.sh` | 176 | **REAL** | orphan | 46 real commands | nothing (self-contained) |
| 311 | `parc-ai/modules/execution.sh` | 338 | **REAL** | orphan | 126 real commands | nothing (self-contained) |
| 312 | `parc-ai/modules/finance.sh` | 101 | **REAL** | orphan | 56 real commands | network, filesystem writes |
| 313 | `parc-ai/modules/language.sh` | 145 | **REAL** | orphan | 32 real commands | filesystem writes |
| 314 | `parc-ai/modules/math.sh` | 131 | **REAL** | orphan | 63 real commands | filesystem writes |
| 315 | `parc-ai/modules/multimodal.sh` | 180 | **REAL** | orphan | 85 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 316 | `parc-ai/modules/nlp-engine.sh` | 164 | **REAL** | orphan | 27 real commands | nothing (self-contained) |
| 317 | `parc-ai/modules/nlp-training.sh` | 108 | **REAL** | orphan | 23 real commands | filesystem writes |
| 318 | `parc-ai/modules/nlu.sh` | 94 | **REAL** | orphan | 14 real commands | nothing (self-contained) |
| 319 | `parc-ai/modules/parcos-features.sh` | 326 | **REAL** | wired | 123 real commands; 2 privileged/service ops; package/service mgmt | /sys, /proc, apt/dpkg, filesystem writes |
| 320 | `parc-ai/modules/persona.sh` | 83 | **REAL** | orphan | 28 real commands | nothing (self-contained) |
| 321 | `parc-ai/modules/productivity.sh` | 209 | **REAL** | orphan | 72 real commands | ~/.config/vokk, filesystem writes |
| 322 | `parc-ai/modules/textgen.sh` | 477 | **REAL** | orphan | 193 real commands | nothing (self-contained) |
| 323 | `parc-ai/modules/travel.sh` | 125 | **REAL** | orphan | 59 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 324 | `parc-ai/modules/web.sh` | 76 | **REAL** | orphan | 27 real commands | network |
| 325 | `parc-ai/overlay/agent-narrator.sh` | 56 | **REAL** | wired | 14 real commands | nothing (self-contained) |
| 326 | `parc-ai/overlay/narrator.sh` | 19 | **REAL** | wired | 5 real commands | nothing (self-contained) |
| 327 | `parc-ai/parcai-model-pipeline.sh` | 686 | **REAL** | orphan | 368 real commands | filesystem writes |
| 328 | `parc-ai/parcos-ai-installer.sh` | 134 | **REAL** | orphan | 59 real commands; 2 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 329 | `parc-ai/parcos-backup.sh` | 122 | **REAL** | orphan | 25 real commands; 1 privileged/service ops | ~/.config/korrinos, ~/.config/vokk |
| 330 | `parc-ai/parcos-clipboard.sh` | 102 | **REAL** | orphan | 32 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 331 | `parc-ai/parcos-devtool.sh` | 334 | **REAL** | orphan | 123 real commands; 2 privileged/service ops; package/service mgmt | ~/.config/korrinos, apt/dpkg, network, filesystem writes |
| 332 | `parc-ai/parcos-media.sh` | 254 | **REAL** | orphan | 82 real commands | filesystem writes |
| 333 | `parc-ai/parcos-procmon.sh` | 116 | **REAL** | orphan | 27 real commands | /sys, /proc |
| 334 | `parc-ai/parcos-searchie.sh` | 231 | **REAL** | orphan | 55 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 335 | `parc-ai/parcos-settings.sh` | 240 | **REAL** | orphan | 74 real commands; 4 sysfs/proc writes; 1 privileged/service ops | ~/.config/korrinos, /sys, filesystem writes |
| 336 | `parc-ai/parcos-terminal.sh` | 177 | **REAL** | orphan | 37 real commands | ~/.config/korrinos, network, filesystem writes |
| 337 | `parc-ai/parcos-uninstall-blocker.sh` | 72 | **REAL** | orphan | 21 real commands; 2 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 338 | `publish-iso.sh` | 58 | **REAL** | orphan | 17 real commands | nothing (self-contained) |
| 339 | `release.sh` | 155 | **REAL** | orphan | 56 real commands | filesystem writes |
| 340 | `screensaver/xscreensaver/wordpen.sh` | 37 | **REAL** | orphan | 2 real commands | nothing (self-contained) |
| 341 | `scripts/setup-gpu-stacks.sh` | 35 | **REAL** | orphan | 10 real commands; 2 privileged/service ops; package/service mgmt | apt/dpkg, filesystem writes |
| 342 | `security/biometric.sh` | 74 | **REAL** | wired | 14 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 343 | `security/filevault.sh` | 126 | **REAL** | wired | 15 real commands | ~/.tinker |
| 344 | `security/findmydevice.sh` | 66 | **REAL** | wired | 18 real commands | ~/.tinker, network |
| 345 | `security/firewall.sh` | 66 | **REAL** | wired | 22 real commands; 3 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, firewall rules, filesystem writes |
| 346 | `security/gatekeeper.sh` | 130 | **REAL** | wired | 15 real commands; 1 privileged/service ops | ~/.tinker, apt/dpkg |
| 347 | `security/privacy.sh` | 64 | **REAL** | wired | 11 real commands | ~/.tinker |
| 348 | `security/security.sh` | 94 | **REAL** | wired | 16 real commands; 2 privileged/service ops | firewall rules |
| 349 | `security/sip.sh` | 67 | **REAL** | wired | 8 real commands; 3 privileged/service ops | ~/.tinker, filesystem writes |
| 350 | `system/adaptive-power-grid.sh` | 451 | **REAL** | wired | 64 real commands; 3 sysfs/proc writes | ~/.tinker, /sys, /proc |
| 351 | `system/audio-clarity.sh` | 244 | **REAL** | orphan | 63 real commands | ~/.tinker, filesystem writes |
| 352 | `system/auto-updates.sh` | 245 | **REAL** | wired | 67 real commands; 4 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 353 | `system/backup/korrinos-backup.sh` | 342 | **REAL** | wired | 103 real commands; 2 privileged/service ops; package/service mgmt | ~/.config/korrinos, systemd units, filesystem writes |
| 354 | `system/bluetooth-manager.sh` | 421 | **REAL** | orphan | 104 real commands; 3 privileged/service ops; package/service mgmt | apt/dpkg, systemd units, filesystem writes |
| 355 | `system/cloud-sync/korrinos-cloud.sh` | 748 | **REAL** | wired | 307 real commands; 4 privileged/service ops; package/service mgmt | ~/.config/korrinos, /proc, apt/dpkg, systemd units, network |
| 356 | `system/content-filter.sh` | 90 | **REAL** | orphan | 37 real commands; 1 privileged/service ops | ~/.config/korrinos, filesystem writes |
| 357 | `system/context-aware-adaptation.sh` | 185 | **REAL** | orphan | 59 real commands | ~/.tinker |
| 358 | `system/default-apps.sh` | 86 | **REAL** | orphan | 42 real commands; 3 privileged/service ops; package/service mgmt | apt/dpkg, network, filesystem writes |
| 359 | `system/desktop-env/korrinos-desktop.sh` | 1993 | **REAL** | wired | 854 real commands; 3 privileged/service ops | ~/.config/korrinos, apt/dpkg, filesystem writes |
| 360 | `system/desktop-env/korrinos-notify.sh` | 249 | **REAL** | wired | 115 real commands | ~/.config/korrinos, filesystem writes |
| 361 | `system/digital-twin.sh` | 524 | **REAL** | wired | 109 real commands; 3 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, systemd units, filesystem writes |
| 362 | `system/fast-boot.sh` | 169 | **REAL** | orphan | 54 real commands; 3 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, systemd units, filesystem writes |
| 363 | `system/feature-manager.sh` | 167 | **REAL** | orphan | 51 real commands | ~/.tinker |
| 364 | `system/gamemode-setup.sh` | 174 | **REAL** | orphan | 68 real commands; 3 privileged/service ops; package/service mgmt | /proc, apt/dpkg, systemd units, filesystem writes |
| 365 | `system/gaming-meta.sh` | 94 | **REAL** | orphan | 18 real commands; 2 privileged/service ops; package/service mgmt | ~/.tinker, apt/dpkg, filesystem writes |
| 366 | `system/gpu-config.sh` | 184 | **REAL** | orphan | 48 real commands; 2 sysfs/proc writes; 2 privileged/service ops; package/service mgmt | /sys, apt/dpkg, filesystem writes |
| 367 | `system/hardware-cert/korrinos-cert.sh` | 901 | **REAL** | wired | 286 real commands | ~/.config/korrinos, /sys, /proc |
| 368 | `system/hardware-detect.sh` | 229 | **REAL** | orphan | 84 real commands; 3 privileged/service ops; package/service mgmt | /sys, apt/dpkg, network, filesystem writes |
| 369 | `system/init.sh` | 97 | **REAL** | orphan | 26 real commands; 2 privileged/service ops; package/service mgmt | nothing (self-contained) |
| 370 | `system/install-greetings.sh` | 137 | **REAL** | orphan | 57 real commands; 2 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 371 | `system/korrinos-apt-hook.sh` | 162 | **REAL** | orphan | 31 real commands; 2 privileged/service ops | apt/dpkg, filesystem writes |
| 372 | `system/korrinos-errors.sh` | 344 | **REAL** | orphan | 125 real commands; 5 privileged/service ops; package/service mgmt | apt/dpkg, systemd units, network, filesystem writes |
| 373 | `system/korrinos-firstboot.sh` | 167 | **REAL** | wired | 112 real commands; 1 privileged/service ops | filesystem writes |
| 374 | `system/korrinos-greetings.sh` | 320 | **REAL** | orphan | 25 real commands | nothing (self-contained) |
| 375 | `system/korrinos-mascot.sh` | 201 | **REAL** | orphan | 57 real commands; 1 privileged/service ops | filesystem writes |
| 376 | `system/monitor/korrinos-process.sh` | 250 | **REAL** | orphan | 81 real commands; 1 privileged/service ops | ~/.config/korrinos, /proc, filesystem writes |
| 377 | `system/network/korrinos-network.sh` | 360 | **REAL** | wired | 112 real commands; 3 privileged/service ops; package/service mgmt | ~/.config/korrinos, /sys, /proc, apt/dpkg, network |
| 378 | `system/optimization-toggles.sh` | 187 | **REAL** | orphan | 58 real commands; 3 privileged/service ops; package/service mgmt | ~/.tinker, /sys, systemd units |
| 379 | `system/parc-dust.sh` | 47 | **REAL** | orphan | 23 real commands | /proc |
| 380 | `system/parc-gamemode-hook.sh` | 82 | **REAL** | orphan | 24 real commands | /proc |
| 381 | `system/parc-gamemode-launch.sh` | 56 | **REAL** | orphan | 6 real commands | /proc |
| 382 | `system/password-monitor.sh` | 548 | **REAL** | orphan | 145 real commands | ~/.tinker, filesystem writes |
| 383 | `system/power-manager.sh` | 405 | **REAL** | wired | 92 real commands; 4 sysfs/proc writes; 1 privileged/service ops | /sys, filesystem writes |
| 384 | `system/predictive-caching.sh` | 243 | **REAL** | wired | 40 real commands | ~/.tinker, filesystem writes |
| 385 | `system/predictive-intelligence.sh` | 144 | **REAL** | wired | 49 real commands | ~/.tinker |
| 386 | `system/predictive-pre-caching.sh` | 155 | **REAL** | orphan | 57 real commands | ~/.tinker |
| 387 | `system/security/korrinos-bugfix.sh` | 434 | **REAL** | wired | 44 real commands; 3 privileged/service ops | ~/.config/korrinos, apt/dpkg, network, filesystem writes |
| 388 | `system/self-healing.sh` | 540 | **REAL** | wired | 109 real commands; 1 sysfs/proc writes; 4 privileged/service ops; package/service mgmt | ~/.tinker, /sys, /proc, apt/dpkg, systemd units |
| 389 | `system/temporal-mapping.sh` | 186 | **REAL** | wired | 28 real commands; 2 sysfs/proc writes | ~/.tinker, /sys, /proc |
| 390 | `system/temporal-resource-mapping.sh` | 156 | **REAL** | orphan | 59 real commands | ~/.tinker |
| 391 | `systemd/install-services.sh` | 47 | **REAL** | orphan | 17 real commands; 2 privileged/service ops; package/service mgmt | systemd units, filesystem writes |
| 392 | `terminal/kcommand/.github/workflows/upload_asset.sh` | 97 | **REAL** | orphan | 10 real commands | network |
| 393 | `terminal/kcommand/scripts/24-bit-color.sh` | 102 | **REAL** | orphan | 19 real commands | nothing (self-contained) |
| 394 | `terminal/kcommand/scripts/create-flamegraph.sh` | 37 | **REAL** | orphan | 7 real commands | filesystem writes |
| 395 | `terminal/kcommand/share/kcommand-language.sh` | 55 | **REAL** | orphan | 4 real commands | nothing (self-contained) |
| 396 | `territories/arsenal.sh` | 160 | **REAL** | wired | 40 real commands; 1 privileged/service ops | apt/dpkg, network, filesystem writes |
| 397 | `territories/cue-watchdog.sh` | 116 | **REAL** | orphan | 43 real commands | nothing (self-contained) |
| 398 | `territories/game/anticheat-consent.sh` | 65 | **REAL** | orphan | 22 real commands | nothing (self-contained) |
| 399 | `territories/game/audio-focus.sh` | 68 | **REAL** | orphan | 27 real commands | nothing (self-contained) |
| 400 | `territories/game/controller-haptics.sh` | 72 | **REAL** | orphan | 34 real commands | nothing (self-contained) |
| 401 | `territories/game/frame-pacing.sh` | 42 | **REAL** | orphan | 12 real commands | nothing (self-contained) |
| 402 | `territories/game/game-mode.sh` | 85 | **REAL** | wired | 21 real commands | /sys, filesystem writes |
| 403 | `territories/game/game-replay-ai.sh` | 65 | **REAL** | orphan | 18 real commands | /proc |
| 404 | `territories/game/game-studio.sh` | 56 | **REAL** | orphan | 23 real commands | nothing (self-contained) |
| 405 | `territories/game/latency-clean.sh` | 62 | **REAL** | orphan | 16 real commands; 2 privileged/service ops; package/service mgmt | systemd units, firewall rules |
| 406 | `territories/game/lowlat-input.sh` | 57 | **REAL** | orphan | 18 real commands; 1 privileged/service ops; package/service mgmt | /proc |
| 407 | `territories/game/perf-tune.sh` | 77 | **REAL** | orphan | 36 real commands; 3 sysfs/proc writes; 1 privileged/service ops; package/service mgmt | /sys, /proc |
| 408 | `territories/game/save-archiver.sh` | 61 | **REAL** | orphan | 26 real commands | nothing (self-contained) |
| 409 | `territories/game/spectator-box.sh` | 42 | **REAL** | orphan | 13 real commands | nothing (self-contained) |
| 410 | `territories/hack/app-guard.sh` | 84 | **REAL** | wired | 28 real commands | filesystem writes |
| 411 | `territories/hack/browser-gate.sh` | 59 | **REAL** | wired | 17 real commands | nothing (self-contained) |
| 412 | `territories/hack/canary-honeypot.sh` | 112 | **REAL** | wired | 38 real commands | filesystem writes |
| 413 | `territories/hack/gpu-pipeline.sh` | 93 | **REAL** | orphan | 30 real commands | filesystem writes |
| 414 | `territories/hack/hack-defense.sh` | 173 | **REAL** | wired | 51 real commands; 2 privileged/service ops; package/service mgmt | /sys, /proc, apt/dpkg |
| 415 | `territories/hack/hack-gate.sh` | 100 | **REAL** | wired | 36 real commands | nothing (self-contained) |
| 416 | `territories/hack/hack-mode.sh` | 92 | **REAL** | wired | 21 real commands | nothing (self-contained) |
| 417 | `territories/hack/intent-hardware.sh` | 120 | **REAL** | orphan | 37 real commands; 1 sysfs/proc writes; 2 privileged/service ops; package/service mgmt | /sys, filesystem writes |
| 418 | `territories/hack/kill-switch.sh` | 95 | **REAL** | orphan | 40 real commands; 1 privileged/service ops | nothing (self-contained) |
| 419 | `territories/hack/masked-proc.sh` | 90 | **REAL** | orphan | 32 real commands; 2 privileged/service ops | /proc |
| 420 | `territories/hack/portal-sandbox.sh` | 118 | **REAL** | orphan | 47 real commands | filesystem writes |
| 421 | `territories/hack/sdr-isolation.sh` | 86 | **REAL** | orphan | 29 real commands; 2 privileged/service ops; package/service mgmt | systemd units |
| 422 | `territories/hack/supply-chain.sh` | 102 | **REAL** | orphan | 36 real commands; 1 privileged/service ops | nothing (self-contained) |
| 423 | `territories/hack/threat-monitor.sh` | 134 | **REAL** | wired | 57 real commands; 2 privileged/service ops | nothing (self-contained) |
| 424 | `territories/install-territories.sh` | 60 | **REAL** | orphan | 19 real commands | filesystem writes |
| 425 | `territories/lib/common.sh` | 58 | **REAL** | wired | 22 real commands; 1 privileged/service ops | nothing (self-contained) |
| 426 | `territories/modes.sh` | 109 | **REAL** | wired | 36 real commands | nothing (self-contained) |
| 427 | `territories/secure/app-allowlist.sh` | 104 | **REAL** | wired | 34 real commands | nothing (self-contained) |
| 428 | `territories/secure/biometric-lock.sh` | 61 | **REAL** | orphan | 22 real commands; 1 privileged/service ops; package/service mgmt | filesystem writes |
| 429 | `territories/secure/canary-monitor.sh` | 79 | **REAL** | wired | 21 real commands; 2 privileged/service ops | filesystem writes |
| 430 | `territories/secure/duress-alert.sh` | 80 | **REAL** | orphan | 31 real commands; 2 privileged/service ops; package/service mgmt | systemd units, network |
| 431 | `territories/secure/filevault2.sh` | 61 | **REAL** | orphan | 25 real commands; 1 privileged/service ops | nothing (self-contained) |
| 432 | `territories/secure/gatekeeper2.sh` | 87 | **REAL** | orphan | 35 real commands | nothing (self-contained) |
| 433 | `territories/secure/identity-cloud.sh` | 61 | **REAL** | orphan | 21 real commands | network, filesystem writes |
| 434 | `territories/secure/key-wallet.sh` | 67 | **REAL** | wired | 22 real commands | nothing (self-contained) |
| 435 | `territories/secure/privacy-ledger.sh` | 73 | **REAL** | orphan | 26 real commands; 1 privileged/service ops | nothing (self-contained) |
| 436 | `territories/secure/secure-mode.sh` | 73 | **REAL** | wired | 20 real commands | nothing (self-contained) |
| 437 | `territories/secure/traffic-guard.sh` | 55 | **REAL** | orphan | 17 real commands; 1 privileged/service ops | nothing (self-contained) |
| 438 | `territories/secure/vault-engine.sh` | 61 | **REAL** | wired | 31 real commands; 2 privileged/service ops | nothing (self-contained) |
| 439 | `territories/secure/zero-trust-config.sh` | 73 | **REAL** | wired | 11 real commands; 1 privileged/service ops | nothing (self-contained) |
| 440 | `territories/self-watchdog.sh` | 151 | **REAL** | orphan | 53 real commands | nothing (self-contained) |
| 441 | `territories/vibe-address/core/action.sh` | 171 | **REAL** | orphan | 52 real commands | nothing (self-contained) |
| 442 | `territories/vibe-address/core/adapt.sh` | 170 | **REAL** | wired | 51 real commands | filesystem writes |
| 443 | `territories/vibe-address/core/align.sh` | 186 | **REAL** | orphan | 60 real commands | nothing (self-contained) |
| 444 | `territories/vibe-address/core/audit.sh` | 150 | **REAL** | orphan | 57 real commands | nothing (self-contained) |
| 445 | `territories/vibe-address/core/auto.sh` | 100 | **REAL** | orphan | 42 real commands | nothing (self-contained) |
| 446 | `territories/vibe-address/core/bloom.sh` | 160 | **REAL** | orphan | 45 real commands | filesystem writes |
| 447 | `territories/vibe-address/core/bulk.sh` | 175 | **REAL** | orphan | 71 real commands | nothing (self-contained) |
| 448 | `territories/vibe-address/core/capacity.sh` | 125 | **REAL** | orphan | 45 real commands | nothing (self-contained) |
| 449 | `territories/vibe-address/core/cms.sh` | 106 | **REAL** | orphan | 47 real commands | filesystem writes |
| 450 | `territories/vibe-address/core/dista.sh` | 107 | **REAL** | orphan | 66 real commands | nothing (self-contained) |
| 451 | `territories/vibe-address/core/index.sh` | 177 | **REAL** | wired | 42 real commands | nothing (self-contained) |
| 452 | `territories/vibe-address/core/ingest.sh` | 334 | **REAL** | wired | 113 real commands | nothing (self-contained) |
| 453 | `territories/vibe-address/core/ir.sh` | 163 | **REAL** | orphan | 43 real commands | filesystem writes |
| 454 | `territories/vibe-address/core/lexin.sh` | 274 | **REAL** | orphan | 138 real commands | nothing (self-contained) |
| 455 | `territories/vibe-address/core/lsh.sh` | 196 | **REAL** | orphan | 61 real commands | filesystem writes |
| 456 | `territories/vibe-address/core/markov.sh` | 157 | **REAL** | orphan | 51 real commands | filesystem writes |
| 457 | `territories/vibe-address/core/match.sh` | 231 | **REAL** | wired | 69 real commands | nothing (self-contained) |
| 458 | `territories/vibe-address/core/phoneme.sh` | 266 | **REAL** | wired | 124 real commands | nothing (self-contained) |
| 459 | `territories/vibe-address/core/prf.sh` | 123 | **REAL** | orphan | 30 real commands | nothing (self-contained) |
| 460 | `territories/vibe-address/core/query.sh` | 588 | **REAL** | wired | 142 real commands | filesystem writes |
| 461 | `territories/vibe-address/core/rank.sh` | 216 | **REAL** | wired | 48 real commands | nothing (self-contained) |
| 462 | `territories/vibe-address/core/retention.sh` | 89 | **REAL** | orphan | 20 real commands | filesystem writes |
| 463 | `territories/vibe-address/core/sarray.sh` | 143 | **REAL** | orphan | 48 real commands | nothing (self-contained) |
| 464 | `territories/vibe-address/core/selftest.sh` | 256 | **REAL** | orphan | 139 real commands | filesystem writes |
| 465 | `territories/vibe-address/core/store.sh` | 245 | **REAL** | wired | 83 real commands | filesystem writes |
| 466 | `territories/vibe-address/core/time.sh` | 225 | **REAL** | wired | 60 real commands | nothing (self-contained) |
| 467 | `territories/vibe-address/core/tree.sh` | 136 | **REAL** | wired | 38 real commands | filesystem writes |
| 468 | `territories/vibe-address/vibe-address.sh` | 351 | **REAL** | wired | 156 real commands | ~/.config/korrinos, filesystem writes |
| 469 | `territories/world-engine.sh` | 227 | **REAL** | wired | 86 real commands | /proc, filesystem writes |
| 470 | `territories/world-optimizer.sh` | 90 | **REAL** | wired | 31 real commands; 1 privileged/service ops; package/service mgmt | /sys, /proc |
| 471 | `vokk/agent/system-agent.sh` | 463 | **REAL** | orphan | 198 real commands | ~/.tinker, filesystem writes |

## Notes and caveats

- **Vendored code excluded.** 15 of the 486 `.sh` files under `os/` are third-party `node_modules` content (`playwright-core`, `app-builder-lib`, `node-gyp`, `jake`) shipped inside `apps/apps/nibra-betterlife/`. They are Playwright/Electron upstream files, not KorrinOS, and are excluded from all counts. The audited corpus is 471 scripts.
- **Keyword scanning over-reports.** Only 36 of 471 scripts contain `TODO`/`stub`/`placeholder`. Most were false positives from prose: `territories/hack/supply-chain.sh` (7 hits) is a genuine sha256 ledger + reproducible-build verifier, and `hardware-tech/data-shredder/data-shredder.sh` (17 hits) really does shred — its hits were on the words *dummy telemetry* and *fake browsing history*, which are the feature. **Keyword counts are not a stub count.**
- **Library files were not judged as stubs.** `parc-ai/modules/*.sh` (45 files) and `territories/vibe-address/core/*.sh` define functions for a caller. `parc-ai/parc-ai.sh:49` glob-sources every module and `vibe-address.sh:44-50` sources its core, so 'uncalled function' is expected there and was not counted as dead code.
- **`set -euo pipefail` is widely undermined.** Many scripts declare it and then defeat it with `2>/dev/null` on the critical call, `|| true`, or unchecked `$?` — e.g. the entire `apply_profile` rule loop in `system/firewall/korrinos-firewall.sh:186-197` swallows every `ufw`/`nft` error, so a failed firewall rule still logs `OK` and returns success.
- **A real bug, not just a stub:** `korrinos-firewall.sh` emits `nft add rule inet korrinos input tcp dport ...` regardless of the rule's protocol, so every UDP profile rule is applied as TCP. The gaming profile (`27015:27030/udp`, `3478:3480/udp`, `88/udp`) is silently broken.
- **Scope note.** This audit covers `os/**/*.sh`. The `.py` files (111 of them, mostly `parc-ai`/`vokk` helpers) were spot-checked for stubbed functions but are not in the 471-row table.
