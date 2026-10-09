# KorrinOS — Vision & Technical Reference

> A complete, self-contained Linux distribution built by modifying the Linux
> kernel itself. Not a skin, not a theme, not a respin — real kernel code,
> compiled in, wired into real subsystems.
>
> **Document status: living reference.** Every "verified" claim in this document
> was checked by running something. Every unverified claim is marked as such.
> Numbers are real measurements, not aspirations. Where the project is behind,
> this document says so plainly.

---

## 1. The Mission

The founding request, verbatim:

> "get the code of latest linux from github and improve it .. feature by
> feature .. research what users wants .. and build them in .. code inside the
> linux code not as a separate code ... ask a lot of qs if wanted"

Three commitments that define the project:

1. **We modify the Linux kernel itself.** Features live in `kernel/`, `mm/`,
   `fs/`, `drivers/`, `arch/`, compiled into the kernel image. Not a
   user-space wrapper bolted on top.
2. **We build on real Linux, not a replacement for it.** Login is a themed
   GDM greeter, not a bespoke login screen. Boot splash is Plymouth + GRUB,
   both real. The kernel is upstream Linux plus our changes.
3. **Everything users need ships in the ISO.** The build host is a build host.
   It never runs the product.

---

## 2. What KorrinOS Is

| | |
|---|---|
| **Name** | KorrinOS |
| **Base** | Upstream Linux `7.2.0-rc6`, x86-64 |
| **Kernel banner** | `7.2.0-rc6-korrinos+` |
| **Distribution identity** | Ubuntu 22.04 user-space (casper live layout), rebranded |
| **Our feature namespace** | `kernel/tinker/` — 38 built-in modules |
| **Distribution shape** | Live ISO with installer; extensible to installed systems |
| **Licence** | GPL-2.0 (kernel), inheriting upstream Linux |

KorrinOS is aimed at **people and organisations who want an OS for their own
hardware** — one they control, with capabilities tuned to their machines rather
than to a vendor's product roadmap.

---

## 3. Architecture

```
┌──────────────────────────────────────────────────────────────┐
│  Upstream Linux 7.2.0-rc6  (85,541 files, unmodified base)   │
├──────────────────────────────────────────────────────────────┤
│  kernel/tinker/  ── 38 built-in modules, ~7,068 lines         │
│  ├── gamemode      scheduler + cpufreq + syscalls integration │
│  ├── thermal_sched  heat-aware CPU placement                 │
│  ├── energy_sched   cpufreq governor scaling                 │
│  ├── hyperdrive     GPU emulation helper                     │
│  ├── oled_wear      backlight lifetime management             │
│  └── ... 33 more                                              │
├──────────────────────────────────────────────────────────────┤
│  kernel/sched/  ── our changes to the stock scheduler        │
│    fair.c              thermal-aware + boost-aware placement  │
│    cpufreq_schedutil.c energy mode, boost accounting, reaping│
│    syscalls.c          gamemode-aware RT policy              │
├──────────────────────────────────────────────────────────────┤
│  os/  ── user-space Control Center, ~486 shell scripts       │
│    terminal/  kcommand + wordpen (Rust, fully tested)        │
│    apps/  system/  parc-ai/  hardware-tech/  territories/     │
├──────────────────────────────────────────────────────────────┤
│  os/build-distro.sh  ── casper ISO pipeline                   │
│  os/boot/live-init/ ── module-free live boot initramfs        │
└──────────────────────────────────────────────────────────────┘
```

### The critical build integration

Tinker is not a side project. Three points in the stock scheduler call into it:

| Call site | Function | Effect |
|---|---|---|
| `kernel/sched/fair.c` | `tinker_task_boosted()` | Boosted tasks bypass thermal demotion in CPU placement |
| `kernel/sched/fair.c` | `tinker_thermal_is_hot()` | CPU selection consults the heat map |
| `kernel/sched/cpufreq_schedutil.c` | `tinker_energy_mode()` | Frequency scaling ±25% by mode |
| `kernel/sched/cpufreq_schedutil.c` | `tinker_gamemode_reap_finished()` | Clears a boost whose process died |
| `kernel/sched/syscalls.c` | `tinker_gamemode_enabled()` | Allows RT scheduling during gamemode |

These are verified present as real `call` instructions in the linked `vmlinux`.

---

## 4. HyperDrive — GPU Emulation Only

**Hard boundary, per project charter:** HyperDrive emulates **only the GPU**.
CPU, RAM, storage, network, audio and input all run on real physical hardware.
Zero cloud. Zero cost. Pure local software.

The goal: make a machine with no dedicated GPU feel as though it has one, via
adaptive resolution, frame prediction, memory compression and CPU
micro-optimisation.

---

## 5. Feature Status — The Honest Picture

This is the part most project documents hide. Here it is straight.

### 5.1 Kernel modules

Of 38 modules, **only 3 have real, verifiable effect**:

| Module | Real effect | Wired into |
|---|---|---|
| `gamemode` | `sched_setscheduler_nocheck(SCHED_FIFO)` across the process group | 8 call sites incl. all 3 sched files |
| `hyperdrive` | `sched_setscheduler()`, `set_cpus_allowed_ptr()`, `kcompactd_run()` | user-space driven |
| `oled_wear` | Rescales `backlight_device_set_brightness()` | real backlight path |

**4 are partial**: `thermal_sched`, `energy_sched`, `battery_life`,
`cache_tiering` (the last genuinely executes CPUID leaf 4 and detects L3
topology, but nothing ever programs CAT/MPAM).

**31 are instrumentation-only** — they expose a `/proc` node that reports
state, but nothing reads that state to change behaviour. Examples of the honest
kind of "not real":

- `data_shredder` — a privacy wipe that increments a counter and writes zero bytes
- `hardware_dna` — the fingerprint is always `TINKER-OBFUSCATED-00000000`
  because it computes `get_cycles() & (get_cycles() ? 0 : 0)`
- `dust_dislodger` — ~90 lines of careful fan-envelope arithmetic, then `pr_info()`
- `coil_whine` — runs a 10 ms timer forever, 100 wakeups/sec, jittering a value nothing reads

**7 exported symbols have zero callers anywhere in the tree**:
`tinker_gamemode_request_boost`, `tinker_thermal_hint_hot_cpu`,
`tinker_energy_account`, `tinker_energy_ratio`, `tinker_oled_wear_seconds`,
`hd_pin_memory`, `hd_alloc_huge_pages`.

Full audit: [`kernel/tinker/EFFECT-AUDIT.md`](kernel/tinker/EFFECT-AUDIT.md)

### 5.2 User-space

| Metric | Value |
|---|---|
| Shell scripts | 486 |
| Genuine reachable functionality | **22%** |
| Ships but can never be invoked | **52%** |
| Rated DANGEROUS | **15** |

The dangerous set is not theoretical: one script runs `nft flush ruleset` at
boot (wiping every firewall rule on the host), and a tool named *Security
Center* runs `ufw disable`.

Full audit: [`os/STUB-AUDIT.md`](os/STUB-AUDIT.md)

### 5.3 Boot status

**This is the critical gap.**

| Test | Result |
|---|---|
| ISO boots to userspace | **0/5 baseline**, 1/5 with `processor.max_cstate=0` |
| Multi-core (`-smp 4`) | **0/4** — stalls during bring-up |
| Multi-core (`-smp 1`, `2`) | boots reliably |

The kernel boots, mounts the ISO, `switch_root`s into the real distribution and
reaches `Welcome to KorrinOS!` — but only intermittently. The stall occurs
immediately after `ACPI: 1 ACPI AML tables successfully acquired and loaded`,
with no panic, no oops, and no lockup report even with `hardlockup_panic=1`.

**KorrinOS is not yet bootable on a blank machine.** Until it is 5/5, nothing
else matters more.

---

## 6. Verified Working

Everything in this section has passing tests behind it.

| Component | Test | Result |
|---|---|---|
| `kcommand` terminal | `cargo test` | **128/128 pass** |
| `wordpen` screensaver | unit + data | **85 + 9 pass** |
| WordPen rendering | PNG output, 3 scripts | English, Thai, Cyrillic verified |
| Shell scripts | `bash -n` | 467 pass |
| Shell quality | ShellCheck 0.10.0 | error-level clean |
| Python | `python3 -m pyflakes` | 49 files clean |
| Kernel build | `make -j$(nproc)` | clean, 0 warnings |
| Tinker symbols | `nm vmlinux` | 79 live text symbols |
| Live boot chain | QEMU | reaches real userspace (intermittent) |

### Bugs found and fixed

**gamemode self-deadlock** (kernel/tinker/gamemode.c) — writing `on <tgid>` to
`/proc/tinker/gamemode` hard-hung the kernel whenever a process boosted its own
tgid, which is the normal case: the game enables gamemode on itself. Observed as
PID 1 blocked 247 seconds on a mutex it owned, with RCU readers stalled behind
it. Three compounding causes: `rcu_read_lock()` held across a sleeping
`sched_setscheduler_nocheck()`; two scheduler predicates taking a mutex from
inside the scheduler; and callers invoking the apply function while holding that
same mutex. Fixed, and verified in QEMU: `on 1` now returns in milliseconds with
zero kernel warnings.

**BIOS bootloader hang** (os/build-distro.sh) — the ISO's "isolinux" image is
actually GRUB. Its `core.img` was built with the `configfile` module and prefix
`/boot/grub`, but the script only embedded the config into `efi.img`, which
serves UEFI alone. BIOS found no config and died silently. Serial output went
from **0 bytes to 15 KB**, GRUB renders its menu, and the kernel loads.

**ISO shipped the wrong kernel** (os/build-distro.sh) — kernel staging fell
through to Ubuntu `5.15.0-1032-realtime`, which cannot run `kernel/tinker` at
all. The ISO kernel had **0** tinker symbols against the built kernel's 79.

**Live initramfs mismatch** (os/boot/live-init) — the ISO's initrd was generated
for a different kernel release. Every `modprobe` failed vermagic, udev never
populated `/dev`, and the root-device scan spun forever with no message. Fixed
with a ~1.1 MB module-free busybox initramfs that needs nothing from the kernel
but built-in drivers.

**ISO9660 as a module** — the live kernel must read its own boot medium. Left as
`=m`, that read fails silently and the ISO appears to hang. Now enforced built-in
by `tools/korrinos-kernel-config.sh`.

---

## 7. Known Defects

Ordered by severity. Full list in [`os/MASTER-TODO.md`](os/MASTER-TODO.md).

### Ship-blockers

| ID | Defect | Impact |
|---|---|---|
| — | Early-boot stall, 0/5 | OS does not reliably boot |
| — | Multi-core stall at `-smp 3+`, 0/4 | Unusable on real hardware; every real CPU and VM has >2 cores |
| B1 | `hyperdrive.c` calls sleeping `sched_setscheduler()` while holding the `rt_lock` spinlock | Kernel splat / crash — same bug class as the fixed gamemode deadlock, missed there |
| B8 | `gamemode_apply()` walks `for_each_process()` with no `rcu_read_lock()` | Use-after-free on `task_struct` |
| — | `nft flush ruleset` at boot | Wipes all host firewall rules |
| — | Security Center runs `ufw disable` | Actively disables the firewall |

### Correctness bugs

| ID | Defect | Impact |
|---|---|---|
| B2 | `tinker_energy_account()` never called | Ratio permanently 0, so the cpufreq PEAK/SAVER path is dead by construction |
| B6 | `sscanf` guards compare against more conversions than exist (`ret >= 3` with 2) | `driver_monitor` load and `hw_cert` test commands can never execute |
| B14/15 | `oled_wear` clamps brightness one-way with no panel-type check | Dims ordinary LCD backlights after 1 hour uptime |
| B17 | `thermal_sched` indexes `cpu_temp_mc[]` via `for_each_possible_cpu()` unguarded | Runaway index can overwrite `enabled` and the hot threshold |
| B10/16 | `hardware_dna` node is `0444` but has a write handler; `thermal_sched` node is `0644` with no write handler | Write-path claims were wrong in both directions |
| B11 | `cpumask_set_cpu(1)` with no `nr_cpu_ids` check | Affinity pinning silently failed in every test config |
| — | `korrinos-firewall.sh:193` hardcodes `tcp dport` | The entire UDP gaming profile applies as TCP |

---

## 8. Testing Infrastructure

Two purpose-built regression gates exist, both committed:

```bash
# Does the kernel bring up N CPUs?
./tools/smp-bringup-test.sh --repeat 3 --smps "1 2 4"

# Does the ISO actually reach userspace?
./tools/iso-boot-test.sh <iso> <bzImage> <initrd> 5 800
```

`iso-boot-test.sh` watches serial output and aborts stalled boots early rather
than burning the full timeout, and scores on a real boot signal — the KorrinOS
greeting printed by the distribution userspace after `switch_root`.

Kernel config is enforced, not remembered:

```bash
./tools/korrinos-kernel-config.sh .config
```

`.config` is gitignored, so requirements that make the ISO bootable would
otherwise be lost on a fresh clone. The tinker symbol list is derived from
`kernel/tinker/Makefile` so it cannot drift.

---

## 9. Roadmap

### Phase 0 — Make it boot (blocking everything)
1. Root-cause the early-boot stall; verify 5/5.
2. Fix multi-core bring-up; verify 5/5 at `-smp 4`.
3. Fix B1 and B8.

### Phase 1 — Earn the feature claims
4. Implement or delete the 31 instrumentation-only modules.
5. Wire or remove the 7 uncalled exports.
6. Finish the top-10 user-space stubs.
7. Remove or sandbox the 15 dangerous scripts.

### Phase 2 — Real hardware
8. Install to a blank laptop; verify boot, then WiFi, audio, graphics, suspend.
9. Verify the hardware-facing modules against real devices — impossible in a VM,
   which is why so many remain unproven.

### Phase 3 — Distribution
10. Publish ISO. Secure Boot signing and a real update channel are prerequisites
    for anything beyond enthusiast use.

### Phase 4 — Experience
11. Cursor themes (animated, system-wide including login screen and installer).
12. Starfield boot loader screen.
13. Instant reload preserving exact application state across shutdown.
14. **Alex** — floating desktop assistant: rotating cube character, Hermes
    backend, one continuous session that reconnects. Rebuilt far more polished,
    on the Zegrate AI backend once training completes. *Planned, not started.*

---

## 10. Honest Self-Assessment

Where KorrinOS genuinely stands:

**Real.** The kernel integrates cleanly into the stock scheduler — those call
sites are not decorative. A serious class of kernel defect (a self-deadlock
reachable from ordinary use) was found by actually booting the thing, root-caused
with register-level evidence, fixed, and regression-tested. The bootloader
defect was found and fixed the same way. `kcommand` and `wordpen` are finished,
tested products. The build pipeline is reproducible.

**Not real yet.** Most of the 38 kernel modules change nothing. Most of the
user-space layer cannot be launched. The OS does not reliably boot. Multi-core
is broken. Nothing has run on real hardware. Nothing has been published.

**The honest summary:** the foundation is sound and the verification method is
now rigorous — but "a professional, reliable OS" is not yet demonstrable, and
this document does not claim it is.

The gap is not conceptual. It is a specific list of defects, tracked in
`os/MASTER-TODO.md`, and it is shrinking.

---

## 11. Quick Reference

```bash
# Kernel
make -j$(nproc) bzImage
./tools/korrinos-kernel-config.sh .config      # enforce required options

# Live initramfs (must match the kernel)
./os/boot/live-init/build-live-initramfs.sh out/initrd.img

# Full ISO
./os/build-distro.sh

# Regression gates
./tools/smp-bringup-test.sh --repeat 3 --smps "1 2 4"
./tools/iso-boot-test.sh <iso> <bzImage> <initrd> 5 800

# Audits
kernel/tinker/EFFECT-AUDIT.md
os/STUB-AUDIT.md
os/MASTER-TODO.md
```

**Key paths**

| Path | What |
|---|---|
| `kernel/tinker/` | 38 built-in feature modules |
| `kernel/sched/{fair,cpufreq_schedutil,syscalls}.c` | Scheduler integration |
| `os/boot/live-init/` | Module-free live boot initramfs |
| `os/build-distro.sh` | ISO pipeline |
| `tools/` | Regression tests and config enforcement |
| `os/terminal/` | kcommand + wordpen (Rust) |
| `os/MASTER-TODO.md` | Prioritised work queue |