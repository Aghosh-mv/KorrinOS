# The state of `kernel/` — read this before touching the kernel work

Written 2026-10-03 after auditing the tree rather than trusting the file listing.

## `kernel/` is not a buildable Linux kernel

It contains **real upstream kernel source** — `acct.c`, `audit.c` and the rest are
genuine, with correct SPDX headers and plausible line counts (audit.c is 2,859
lines, which is about right for the real file). That is not the problem.

The problem is that **19 of the 19 directories a kernel build needs are missing**:

    include  arch  drivers  fs  mm  kernel  block  net  security  lib
    ipc  crypto  init  samples  scripts  tools  sound

There is no `Kconfig` at the kernel root either. The 24 directories that *are*
present — `bpf cgroup configs debug dma entry events futex gcov irq kcsan
livepatch liveupdate locking module power printk rcu sched time tinker trace
unwind` — are all leaf subdirectories, with none of the build machinery that ties
them together.

So `kernel/` is a **partial snapshot of kernel source**, not a kernel. Nothing in
it can be compiled, linked, or booted.

## The `tinker/` modules are real, good code — with nowhere to go

All 38 modules in `kernel/tinker/` are genuine KorrinOS kernel modules: correct
`#include <linux/...>` headers, `tinker_core.h`, `/proc/tinker/*` interfaces,
proper `MODULE_LICENSE`, `module_init`/`module_exit`. `gamemode.c` mirrors the
Feral GameMode design; the registry-facing ones implement the roadmap's thermal,
energy, battery, OLED, cache-tiering, dust, shredder, CXL, DVFS, ray-traced audio
and neural-audio ideas. This is the "code inside the linux code" the project
mission asks for, and it is written to a normal standard.

`kernel/tinker/Makefile` already carries the right `obj-$(CONFIG_TINKER_*)` lines
— 37 of them, one per module. The wiring is correct; the tree it points into is
not there.

**Nothing in `tinker/` is called from the real kernel.** Only `tinker_proc_root`
is referenced anywhere outside `tinker/` (4 references). `tinker_gamemode_request_boost`,
`tinker_energy_mode`, `tinker_battery_envelope` and the rest are defined, never
invoked. They sit next to the scheduler and mm and do nothing.

## Why the modules cannot be compile-checked here

Ubuntu's installed `linux-headers-*` packages have an **empty
`include/generated/`** — every generated file is stripped, including
`asm/cpufeaturemasks.h`, which `arch/x86/include/asm/cpufeature.h` includes
unconditionally. Without a configured kernel source tree there is no way to
produce that file, so every module fails at the first x86 header.

Checked against all three installed header versions (6.16.3, 6.17.9, 7.0.11);
all have the same empty `include/generated/`.

**So the "38 compiled `.o` files" in `tinker/` are not evidence of anything.**
They date from 2026-09-28 and were produced by some earlier setup that no longer
exists in this tree. Do not read them as proof the modules compile.

## What is actually needed

In order:

1. **A real kernel source tree.** Not vendored into this repo — fetched at build
   time into a separate directory, so the repo does not carry 1.4 GB of someone
   else's code.
2. **A configuration**, then `make prepare` (or `make defconfig && make prepare`)
   to generate `include/generated/`.
3. **`tinker/` wired into that tree's `Makefile`** — one line, `obj-y += tinker/`
   next to the other top-level entries.
4. **`tinker/Kconfig`** sourced from the tree's `Kconfig`, defining the
   `CONFIG_TINKER_*` symbols that `tinker/Makefile` already references.
5. **Actual call sites.** A module that builds still does nothing. The first real
   integration should be `tinker_gamemode_request_boost()` called from
   `kernel/sched/core.c`'s `pick_next_task`, because that is the single function
   that makes 38 decorative modules into live kernel behaviour.

Only after all five can any claim of "it works in the kernel" be honest.

## Disk note

This machine had **9.7 GB free at 98% full** when this was written. A kernel
source is ~1.4 GB compressed and rather more extracted. Fetching one is
possible but tight, and it should not be done while the Zegrate training run is
writing checkpoints.
