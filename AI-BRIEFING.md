# KorrinOS — Complete Project Briefing

> A single self-contained document for setting up another AI instance on this
> project. Read this first, then read `KORRINOS-VISION.md` for technical detail,
> then the audits named in §7.
>
> **This document distinguishes verified fact from intent throughout.** Do not
> present intent as capability.

---

## 1. What this project is

**KorrinOS** is a Linux distribution built by modifying the Linux kernel itself.
The founding request was verbatim:

> "get the code of latest linux from github and improve it .. feature by
> feature .. research what users wants .. and build them in .. code inside the
> linux code not as a separate code ... ask a lot of qs if wanted"

Three non-negotiable commitments:

1. **Kernel code, not user-space wrappers.** Features live in `kernel/tinker/`
   and are compiled into the kernel image (`core-y`). Modifications to
   `kernel/sched/` wire them into the stock scheduler.
2. **Built on real Linux, never replacing it.** Login is a themed GDM greeter.
   Boot splash is Plymouth + GRUB. The kernel is upstream plus our diff.
3. **Everything ships in the ISO.** The build host never runs the product.

**Target users:** people and organisations who want an OS for their own
hardware — controllable, tuned to their machines, not dictated by a vendor
roadmap.

---

## 2. Ground rules for any AI working on this

| Rule | Reason |
|---|---|
| **Never boot KorrinOS natively on the build host** | It is a build box. A native boot replaces the running OS. Use QEMU. |
| **Do not touch** `zegrate-training`, `DO-NOT-DELETE`, `Frame_Scan_AI`, `t2a`, `models` | Separate projects and training data. |
| **Do not `git add os/` wholesale** | The tree has untracked files (`os/scan_20261003_211525.pdf`, `territories/`) that do not belong in commits. |
| **`.config` is gitignored** | Required options live in `tools/korrinos-kernel-config.sh`. Do not "fix" boot issues by editing config and hoping — run that script. |
| **Never claim a fix without multiple runs** | A single successful boot is not a fix. Boot is flaky by nature here; see §5. |
| **QEMU is TCG only** | `/dev/kvm` exists but the host CPU lacks SVM, so emulation is ~100× slow. Budget accordingly. Expect multi-minute boots. |
| **Disk is tight** (~16 GB free, 97% used) | Never copy the 4.4 GB ISO or the kernel tree around. |
| **Sole attribution** | Attribution is the user only (Aghosh-mv / safarhashim007). No AI co-authorship and no other contributors are claimed. |

---

## 3. Project identity

| | |
|---|---|
| Product name | **KorrinOS** (formerly TinkerAI; `VOKK` is the AI subsystem) |
| Base kernel | Upstream Linux `7.2.0-rc6`, x86-64 |
| Banner | `7.2.0-rc6-korrinos+` |
| User-space base | Ubuntu 22.04 (systemd 249.11), casper live layout |
| Licence | GPL-2.0, inheriting upstream Linux |
| Repo | `github.com/Aghosh-mv/TinkerOS` (branch `master`) |

**Name history:** the AI component was renamed TinkerAI → VOKK v4. The OS itself
is KorrinOS. The repository is still named TinkerOS.

---

## 4. Architecture in brief

```
Upstream Linux 7.2.0-rc6 (85,541 files)
   └─ kernel/tinker/            38 built-in modules (~7,068 lines)
   └─ kernel/sched/             our diff to stock scheduler
         fair.c                 thermal-aware + boost-aware CPU placement
         cpufreq_schedutil.c    energy mode, boost accounting, reaping
         syscalls.c             gamemode-aware RT policy
   └─ os/                       user-space Control Center (~486 scripts)
         terminal/              kcommand + wordpen (Rust, fully tested)
         apps/ system/ parc-ai/ hardware-tech/ territories/
   └─ os/build-distro.sh        casper ISO pipeline
   └─ os/boot/live-init/        module-free live boot initramfs
```

**HyperDrive boundary (hard constraint):** HyperDrive emulates **only the GPU**.
CPU, RAM, storage, network, audio and input all run on real hardware. Zero
cloud, zero cost, pure local software.

---

## 5. Current status — read this before planning anything

### 5.1 The blocking problem

**KorrinOS does not reliably boot.**

| Test | Result |
|---|---|
| ISO → userspace | **0/5** baseline; 1/5 with `processor.max_cstate=0` |
| `-smp 1`, `-smp 2` | boots reliably |
| `-smp 3`, `4`, `6` | **stalls** (0/4 measured) |

The stall occurs immediately after:

```
ACPI: 1 ACPI AML tables successfully acquired and loaded
```

with no panic, no oops, and no lockup report even with
`nmi_watchdog=1 hardlockup_panic=1 softlockup_panic=1`. Per-vCPU register dumps
show CPUs parked rather than spinning. **Root cause is unknown.**

Already ruled out: tinker code (a no-tinker kernel stalls identically),
`NR_CPUS` limits, the NMI watchdog, missing modules, `PARAVIRT_XXL`,
`PARAVIRT_SPINLOCKS`, and `CPU_IDLE_GOV_TEO`.

**Everything else is secondary until this is 5/5.**

### 5.2 Feature honesty

| Layer | Real | Partial | Instrumentation-only |
|---|---|---|---|
| 38 kernel modules | **3** | 4 | **31** |

Real effect: `gamemode`, `hyperdrive`, `oled_wear`. Partial: `thermal_sched`,
`energy_sched`, `battery_life`, `cache_tiering`.

User-space: **22%** genuinely reachable, **52%** ships but can never be invoked,
**15 scripts rated DANGEROUS**.

This matters: a `/proc` node that reports state is not a feature that works.
`data_shredder` reports a successful wipe while writing zero bytes.
`hardware_dna`'s fingerprint is always `TINKER-OBFUSCATED-00000000` because it
computes `get_cycles() & 0`.

### 5.3 Verified working

| Component | Evidence |
|---|---|
| `kcommand` | 128/128 tests |
| `wordpen` | 85 unit + 9 data tests; EN/TH/Cyrillic rendering verified |
| Shell scripts | 467 pass `bash -n`; ShellCheck error-level clean |
| Python | 49 files Pyflakes-clean |
| Kernel build | `make -j$(nproc)` clean, 0 warnings, 79 tinker symbols |
| Live boot chain | reaches `Welcome to KorrinOS!` (intermittently) |

### 5.4 Real bugs fixed

1. **gamemode self-deadlock** — writing `on <tgid>` hard-hung the kernel when a
   process boosted its own tgid (the normal case). PID 1 blocked 247s on a mutex
   it owned; RCU readers stalled. Fixed and regression-tested.
2. **BIOS bootloader silent hang** — the "isolinux" image is GRUB; config was
   embedded only in `efi.img` (UEFI-only). Serial went 0 bytes → 15 KB.
3. **ISO shipped the wrong kernel** — Ubuntu `5.15.0-1032-realtime`, 0 tinker
   symbols. Kernel staging now prefers the built `bzImage`.
4. **Live initramfs mismatch** — 5.15 initrd vs 7.2 kernel; all `modprobe` failed
   vermagic, udev never populated `/dev`, root scan spun silently.
5. **ISO9660 as a module** — the kernel must read its own boot medium.

---

## 6. Known defects, prioritised

Full list in `os/MASTER-TODO.md`.

**Ship-blockers**

| ID | Defect |
|---|---|
| — | Early-boot stall (0/5) |
| — | Multi-core stall at `-smp 3+` |
| B1 | `hyperdrive.c` calls sleeping `sched_setscheduler()` under the `rt_lock` spinlock — same class as the fixed gamemode deadlock |
| B8 | `gamemode_apply()` walks `for_each_process()` without `rcu_read_lock()` → UAF |
| — | `nft flush ruleset` runs at boot |
| — | Security Center runs `ufw disable` |

**Correctness**

| ID | Defect |
|---|---|
| B2 | `tinker_energy_account()` uncalled → cpufreq PEAK/SAVER dead by construction |
| B6 | `sscanf` guards demand more conversions than exist → commands unreachable |
| B14/15 | `oled_wear` dims ordinary LCDs one-way after 1 h |
| B17 | `thermal_sched` unguarded `cpu_temp_mc[]` indexing |
| B10/16 | Write-handler/permission mismatches on two proc nodes |
| B11 | `cpumask_set_cpu(1)` unchecked against `nr_cpu_ids` |
| — | `korrinos-firewall.sh:193` hardcodes `tcp dport`, breaking the whole UDP profile |

---

## 7. Where the truth lives

Read these before making claims about this project:

| Document | Contents |
|---|---|
| **`KORRINOS-VISION.md`** | Full technical + product reference |
| **`os/MASTER-TODO.md`** | Prioritised work queue with verified facts footer |
| **`kernel/tinker/EFFECT-AUDIT.md`** | Per-module verdict: real / partial / instrumentation / dead, plus 19 bugs |
| **`os/STUB-AUDIT.md`** | All 471 user-space scripts, verdict + danger rating |
| `README.md`, `os/README.md` | Project overview |

---

## 8. Commands

```bash
# Kernel
make -j$(nproc) bzImage
./tools/korrinos-kernel-config.sh .config      # enforce boot-critical options

# Live initramfs (must match the kernel)
./os/boot/live-init/build-live-initramfs.sh out/initrd.img

# Full ISO
./os/build-distro.sh

# Regression gates — use these, not ad-hoc QEMU
./tools/smp-bringup-test.sh --repeat 3 --smps "1 2 4"
./tools/iso-boot-test.sh <iso> <bzImage> <initrd> 5 800

# Isolated verification only. Never natively.
qemu-system-x86_64 -m 4096 -smp 2 -cdrom <iso> \
  -kernel arch/x86/boot/bzImage -initrd <initrd> \
  -append "console=ttyS0,115200 panic=-1" \
  -display none -serial file:/tmp/boot.log -no-reboot
```

Per-vCPU register dumps (no gdb installed):
```bash
# start qemu with: -monitor unix:/tmp/mon.sock,server,nowait
printf 'cpu 2\ninfo registers\n' | socat - UNIX-CONNECT:/tmp/mon.sock
```
Resolve addresses against `vmlinux`, **not** the bzImage-extracted ELF — the
latter is stripped.

---

## 9. Distribution status

- **GitHub:** `gh auth login` completed as `Aghosh-mv`, admin rights. 35 commits
  pushed.
- **SourceForge:** SSH key works and authenticates, but project `korrinos`
  returns **404 — never created**. The project that exists is `tinker`, to which
  this account has **no write access**. Publishing is blocked on the user
  creating `korrinos` or granting upload rights on `tinker`.

---

## 10. Roadmap

**Phase 0 — Make it boot** (blocking)
1. Root-cause the boot stall; 5/5 verified.
2. Fix multi-core; 5/5 at `-smp 4`.
3. Fix B1, B8.

**Phase 1 — Earn the claims**
4. Implement or delete 31 instrumentation-only modules.
5. Wire or remove 7 uncalled exports.
6. Finish top-10 user-space stubs; neutralise 15 dangerous scripts.

**Phase 2 — Real hardware**
7. Install to a blank laptop; verify boot, WiFi, audio, graphics, suspend.

**Phase 3 — Distribution**
8. Publish ISO. Secure Boot + update channel are prerequisites beyond
   enthusiast use.

**Phase 4 — Experience**
9. Cursor themes — animated, system-wide including login screen and installer;
   Mouse Designs picker in Settings.
10. Starfield loading screen.
11. Instant reload preserving exact app state across shutdown.
12. **Alex** — floating desktop assistant, rotating cube, Hermes backend, one
    continuous reconnecting session. Far more polished, on the Zegrate AI
    backend once training completes. *Planned only.*
13. Evaluate the smooth-ui library.

---

## 11. Summary judgement

**Sound:** the kernel integrates cleanly into the stock scheduler. A serious,
user-reachable kernel defect was found by booting the OS, root-caused with
register evidence, fixed, and regression-tested. The bootloader defect was found
and fixed the same way. `kcommand` and `wordpen` are finished products. The build
pipeline is reproducible.

**Not sound yet:** most kernel modules change nothing; most user-space code
cannot launch; boot is unreliable; multi-core is broken; nothing has run on real
hardware; nothing has shipped.

**The gap is a specific, tracked defect list — not a conceptual gap.** It is in
`os/MASTER-TODO.md` and it is shrinking.

---

## 12. Prompt for a fresh AI instance

> You are working on **KorrinOS**, a Linux distribution built by modifying the
> Linux kernel itself (upstream 7.2.0-rc6 + 38 built-in modules in
> `kernel/tinker/` wired into the stock scheduler).
>
> **Read these first, in order:** `KORRINOS-VISION.md`, `os/MASTER-TODO.md`,
> `kernel/tinker/EFFECT-AUDIT.md`, `os/STUB-AUDIT.md`.
>
> **The single goal:** be able to state with maximum confidence that this OS
> boots on a blank laptop with no OS installed and can be used without bugs.
> Right now it cannot — it boots 0/5. Do not claim otherwise until
> `./tools/iso-boot-test.sh` reports 5/5.
>
> **Hard rules:** never boot natively (build host); use QEMU; do not touch
> `zegrate-training`, `DO-NOT-DELETE`, `Frame_Scan_AI`, `t2a`, `models`; do not
> `git add os/` wholesale; `.config` is gitignored so run
> `tools/korrinos-kernel-config.sh`; never claim a fix from a single successful
> run; TCG emulation is ~100× slow, so budget minutes per boot; disk is tight.
>
> **Be honest over agreeable.** Roughly two thirds of the kernel modules and
> user-space scripts do not do what they appear to. Say so rather than implying
> capability. The user has explicitly asked for this project to be treated as a
> real professional OS — the fastest route to that is an accurate picture.