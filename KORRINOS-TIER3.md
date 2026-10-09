# Tier 3 — Possible In Principle, Unsolved Or Done Badly Anywhere

> The third of three tiers. Tiers 1 and 2 are blocked by physics and law, where
> no amount of engineering changes the answer. **Tier 3 is different: these are
> things that could be built correctly, and are not.**
>
> Nothing in Tier 3 is a Linux *architectural* impossibility. Each is a genuine
> engineering failure — in the kernel, in every distribution, or in this
> project. That makes Tier 3 the honest list of "we all collectively haven't
> done this yet," and the only tier where an OS project can actually win by
> doing better.
>
> **This document does not claim any distribution is *architecturally blocked*
> from these.** Where a capability exists and merely works badly, that is said
> explicitly. The distinction matters: "unsolved" is a legitimate engineering
> target; "impossible" would be an excuse.

---

## How to read this

Each entry states:

- **The capability** — what "doing this properly" means
- **State today** — what actually exists, stated honestly
- **Why it isn't done** — the real reason, not "it's hard"
- **This kernel** — measured against KorrinOS's own `.config`, not assumed

Config values are read from the actual build. Verified against
`7.2.0-rc6-korrinos+` `.config`.

---

## A. Real-time and determinism

### 1. Hard real-time on a general-purpose kernel

**Capability:** jitter bounded in the tens of microseconds, for control loops,
audio DSP, motor control, PLC-style work.

**State today:** `PREEMPT_RT` exists and works on parts of the tree, but
non-RT drivers and subsystems still break the guarantee. Real deployments
either use RT kernels with an isolated RT subsystem, or move to a different
silicon.

**Why not done:** a single non-preemptible section anywhere — a vendor GPU
driver, a filesystem, a firmware call — invalidates every bound. Proving
absence of such code across ~85,000 files is not tractable, so the guarantee is
structurally unprovable rather than merely unmet.

**This kernel:** `CONFIG_PREEMPT_RT` **OFF**. KorrinOS offers no real-time
option today.

### 2. Bounded-latency networking end to end

**Capability:** application-to-wire latency with a hard ceiling, across NIC,
driver, stack, and application.

**State today:** XDP/eBPF can bypass the stack and get very close. But NIC
hardware queues, IRQ affinity, and driver completion paths are not
deterministic, and cross-NIC paths are worse. TSO/GSO and multi-queue
coalescing deliberately trade latency for throughput.

**Why not done:** the last microsecond lives in the NIC and in PCIe
completion ordering, neither of which software controls.

**This kernel:** `XDP_SOCKETS` BUILTIN, `BPF` BUILTIN, `NET_SCH_FQ` =m. Strong
starting point; the NIC-side half is missing.

### 3. Deterministic GPU scheduling

**Capability:** a GPU frame completes within a known bound.

**State today:** time-slicing exists (`CONFIG_DRM_SCHED` on). Hard deadlines do
not, because the GPU executes asynchronously and driver firmware decides
completion order.

**Why not done:** the GPU is a separate execution domain with its own firmware.
The kernel schedules command submission, not execution.

---

## B. Security and attestation

### 4. End-to-end measured boot to a remote verifier

**Capability:** every boot stage hashed and measured, attesting to a remote
service, with tamper evidence covering firmware, bootloader, kernel, and
userspace.

**State today:** dm-verity, IMA, Secure Boot, and TPM PCR extension all exist
and compose. What does not exist is a distribution that ships the *whole
chain* wired to *remote attestation* by default, with firmware support
guaranteed across the hardware it ships on.

**Why not done:** firmware quality varies enormously. Attestation is only as
strong as the earliest measurable stage, and many platforms report PCRs that
don't include what you'd need.

**This kernel:** `INTEGRITY` BUILTIN, `DM_VERITY` =m, `IMA_EVM` **absent**,
`SYSTEM_TRUSTED_KEYS=""`, `SECURITY_SELINUX_DEVELOP=y`. **KorrinOS currently
cannot do this at all** — and this is our defect, not a Linux limit.

### 5. Mandatory access control that users cannot disable

**Capability:** a system where policy is genuinely mandatory, not advisory.

**State today:** SELinux enforcing and AppArmor are both available and both
optional. Most distributions ship SELinux permissive. Container runtimes
routinely disable enforcement because it breaks them.

**Why not done:** the container ecosystem depends on processes being able to
mount, create device nodes, and load BPF. Making policy mandatory means
rearchitecting that ecosystem.

**This kernel:** `SECURITY_SELINUX_DEVELOP=y` — **logs, does not enforce**,
while also advertising `+SELINUX` in systemd's mode string. We look compliant
and are not. Fixing this is in `os/MASTER-TODO.md`.

### 6. Sandboxing the kernel from itself

**Capability:** a bug in one driver cannot compromise the whole system.

**State today:** user namespaces, seccomp, BPF LSM, Landlock, and `mm_struct`
protections all exist. They constrain *userspace*. Nothing constrains a kernel
driver from the rest of the kernel — one bad driver is full kernel compromise.

**Why not done:** drivers are written against APIs that assume mutual trust.
Retrofitting isolation to ~85,000 files' worth of in-kernel interfaces is a
decade-scale project, actively being worked (rust-for-linux, hood, isolate).

**This kernel:** `BPF_LSM` BUILTIN, `SECCOMP` BUILTIN, `USER_NS` BUILTIN.
Genuinely capable here — this is one of KorrinOS's better positions.

---

## C. Storage and data

### 7. Crash-consistent storage without a battery or flush cache

**Capability:** data survives power loss with no cache flush needed.

**State today:** requires `flush`/`fsync` discipline in application code. `dm-flakey`, `bcache`, and FUA-capable NVMe help but do not eliminate the requirement.

**Why not done:** the drive cache is volatile; only the drive can flush it,
and only on command.

### 8. Guaranteed SSD endurance-aware placement

**Capability:** the filesystem knows wear state and places hot data accordingly.

**State today:** `fstrim`, discard, and zone awareness for **Zoned** SSDs.
Wear-leveling awareness for general consumer NVMe: absent — the drive does it
internally and exposes nothing.

**Why not done:** requires vendor cooperation or a standard SMART/zone
interface for wear data. The filesystem is deliberately kept ignorant.

**This kernel:** `SQUASHFS_ZSTD` BUILTIN, `NVME_CORE` =m.

### 9. Copy-on-write everywhere, including block devices

**Capability:** snapshot and rollback any block device without upper-layer
cooperation.

**State today:** btrfs, XFS with reflinks, ZFS, and LVM snapshots all do this
— at the filesystem or volume layer. Block-level COW for arbitrary devices
(`bcachefs`) remains in development.

**Why not done:** every block layer assumes a device has stable, linear
addressing semantics.

### 10. Durable memory semantics

**Capability:** store and load with crash consistency across power loss, in one
instruction sequence, on ordinary RAM.

**State today:** Intel TSX/AMX exist but are disabled by microcode on most
parts for security. Persistent memory (PMEM/Optane) supports real
load-store persistence via `pmem`/`libpmem`, but the ecosystem is small and
the hardware is nearly discontinued.

**Why not done:** hardware vendors removed the feature for good reason.

---

## D. Virtualization

### 11. Consumer-GPU passthrough with full performance

**Capability:** a physical GPU in a VM, with performance close to bare metal.

**State today:** works for AMD (much better) and Intel iGPU, with real
caveats. NVIDIA consumer cards reset on VM teardown and need module patching;
`vGPU` (the actual partitioning product) is licensed to enterprise customers.

**Why not done:** NVIDIA's driver deliberately resets the device when the
host driver unloads. Not a Linux policy.

**This kernel:** `VFIO` on, `INTEL_IOMMU` BUILTIN, `AMD_IOMMU` BUILTIN,
`AMD_VGPU` absent.

### 12. Live kernel migration with zero downtime

**Capability:** move a running VM between hosts without pausing it, with the
application unaware.

**State today:** CRIU works for a limited class of workloads. Live migration
requires that a kernel be able to checkpoint a process and restore it
elsewhere — including all its open files, sockets, and device state. Anything
not CRIU-aware breaks.

**Why not done:** the kernel holds vast amounts of state with no general
serialization mechanism. CRIU works by reverse-engineering that state per
subsystem, which does not scale.

### 13. Guaranteed teardown of a hostile VM

**Capability:** reclaim 100% of resources from a VM that has been compromised
or wedged, without host reboot.

**State today:** roughly yes for memory, no for passed-through devices and for
PCIe fault injection. `IOMMU` groups help; they do not solve it.

**Why not done:** a passed-through device is, by definition, outside kernel
control. IOMMU fault handling is improving but not complete.

---

## E. Observability and debugging

### 14. Kernel debugging without serial-console access

**Capability:** full post-mortem debugging of a system you cannot physically
reach, over the network.

**State today:** `kdump`/`kexec` requires configuring a crash kernel ahead of
time and needs memory reserved. `pstore`/`ramoops` captures panic logs to
persistent memory. System.map-based symbolication works. What does not exist is
a complete remote-debugging story for a locked-down machine.

**Why not done:** the harder problem is *symbol resolution*: shipping exact
kernel symbols matching a running binary, across distro rebuilds and
third-party module trees, requires coordinated build metadata no distro
centralises.

**This kernel:** `SOFTLOCKUP_DETECTOR` BUILTIN, `HARDLOCKUP_DETECTOR` BUILTIN,
`KEXEC_CRASH_DUMP_CRASHKERNEL` **absent**.

### 15. Provable absence of a race or use-after-free

**Capability:** prove a kernel subsystem has no memory-safety races, not merely
"we fuzzed it and found none."

**State today:** KASAN, KCSAN, UBSAN, and lockdep find bugs. They do not prove
absence. Concurrency reasoning is manual.

**Why not done:** unbounded state space. This is a formal-verification problem
that tooling does not solve at kernel scale.

**This kernel:** worth auditing — `CONFIG_UBSAN=y` was observed earlier, and
`CONFIG_DEBUG_INFO=y` with DWARF5, which is unusually heavy for a shipping
kernel and inflates build time and size.

---

## F. Observability of userspace behaviour

### 16. Explain why the system is slow, always

**Capability:** always answer "what is the system waiting on," down to the
offending line.

**State today:** `perf`, `ftrace`, `bpftrace`, `eBPF` are excellent and
thorough. What is missing is *always-on, low-overhead, historical* causality
after the fact — most tracing tools are armed for a window, not continuous.

**Why not done:** continuous high-fidelity tracing has real cost. Ring buffers
are finite; the kernel cannot keep unlimited history at zero overhead.

**This kernel:** `BPF_JIT` BUILTIN, `CGROUP_BPF` BUILTIN, `BPF` BUILTIN.
Strong instrumentation base.

### 17. Unified timeline across kernel, firmware, and userspace

**Capability:** one chronologically consistent trace covering firmware,
kernel, driver, and application, correlatable without manual offset tuning.

**State today:** achievable by hand with expertise and matching clocks.
Turnkey, always-correct, cross-layer correlation: absent. Every layer has its
own clock source, and PTP over firmware is rarely wired up.

**Why not done:** needs a shared timebase spanning firmware, kernel, and
userspace, which requires firmware cooperation.

---

## G. Energy and thermal

### 18. Predictive thermal scheduling that is actually predictive

**Capability:** know a thermal throttle will happen 2 seconds out and schedule
around it.

**State today:** thermal throttling is reactive — the hardware hits a limit and
the kernel backs off. Prediction requires accurate future power modelling, and
that requires knowing the workload.

**Why not done:** general power modelling is unsolved. A generic CPU cannot
know what the code it is about to run will cost.

**KorrinOS specifically:** `thermal_sched` maintains a heat map that the
scheduler consults, but in a VM with no thermal zones it is **permanently
inert** — every temperature decays to zero and `tinker_thermal_is_hot()`
returns false forever. The `fair.c` hook is verified reachable but has never
actually fired in testing.

### 19. Guaranteed power envelope under thermal pressure

**Capability:** a device that never exceeds its thermal design power, even with
all workloads running.

**State today:** cap-based throttling exists and does bound power. What it does
not do is *guarantee* a bound — it reacts, so transient overshoot occurs.

**Why not done:** prediction again.

---

## H. Driver and hardware

### 20. A driver model that encourages out-of-tree development

**Capability:** write a modern driver outside the kernel tree, in a modern
language, without it being second-class.

**State today:** `rust-for-linux` is real, substantial, and upstream-merged for
several subsystems. Bindings, abstractions, and the supporting infrastructure
remain incomplete, so C in-tree is still the only path with full subsystem
support.

**Why not done:** the kernel's core interfaces were designed for C, are
performance-critical, and are depended upon by every driver in existence.

**This kernel:** `BPF_LSM`, `IO_URING`-adjacent async paths are BUILTIN. Rust
driver support not audited here.

---

## I. Summary: where KorrinOS is strong and where it is weak

**Genuinely above-average (most distros do not have these builtin):**

- `BPF`, `BPF_JIT`, `BPF_LSM`, `CGROUP_BPF`, `XDP_SOCKETS` — BUILTIN
- `SOFTLOCKUP_DETECTOR`, `HARDLOCKUP_DETECTOR` — BUILTIN
- `PREEMPT_DYNAMIC` — BUILTIN
- `USER_NS`, `SECCOMP`, `SECCOMP_FILTER` — BUILTIN
- `INTEGRITY` — BUILTIN

**Clearly behind:**

- `PREEMPT_RT` **OFF** — no real-time story at all
- `IMA_EVM` **absent** — cannot do measured boot
- `SYSTEM_TRUSTED_KEYS=""` — cannot do Secure Boot
- `SECURITY_SELINUX_DEVELOP=y` — enforcement is off
- `KEXEC_CRASH_DUMP` **absent** — no post-mortem debugging
- `DM_VERITY` =m only — verity volumes need a module load

**The strategic read:** KorrinOS's kernel is built like a modern cloud/observability
kernel, which is a defensible choice. It is *not* built like a security-hardened
or real-time kernel, and Tier 3 items 4, 5, 14, and 1 are exactly where that
shows. Those four are concrete, fixable, and are on the roadmap.

---

## J. The one-line summary

Tier 1 cannot be engineered. Tier 2 cannot be licensed. **Tier 3 is simply
unfinished work** — and it is the only tier where this project can be judged on
what it builds rather than what it inherits.
