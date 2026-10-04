# The state of `kernel/tinker/` — verified against a real build

Last verified: full `make` to `vmlinux` + `bzImage`, all 38 modules recompiled
from scratch, zero warnings, zero errors.

## This is a real, built KorrinOS kernel

The repo root is a complete Linux kernel tree — **7.2.0-rc6**, `CONFIG_LOCALVERSION="-korrinos"`,
`CONFIG_X86_64=y`, built with gcc 11.4. `vmlinux` is ~507 MB and `bzImage` ~17 MB.

`tinker/` is wired in at both ends:

- `Makefile:836` — `core-y += kernel/tinker/`
- `Kconfig:38` — `source "kernel/tinker/Kconfig"`

All 38 `CONFIG_TINKER_*` symbols are `=y` in `.config`. The modules are `obj-y`,
so they are **built into the image**, not loadable modules — which is the point:
this is kernel code, not a service running beside one.

## It is actually linked into the scheduler

This is the part worth knowing, because it is easy to assume the modules are
decorative. They are not. Three scheduler files call into `tinker/`:

**`kernel/sched/fair.c`** — CPU selection, two call sites:
- ~9609: a latency-critical boosted task waives thermal placement, so gamemode
  does not get throttled by the thermal governor
- ~9620: `select_task_rq_fair` avoids waking a task onto a thermally-hot CPU when
  the task's previous CPU is cool *and* valid for it — it only ever moves a task
  back to a CPU it was already allowed to run on, never overriding affinity

**`kernel/sched/cpufreq_schedutil.c`** — four call sites:
- ~204: energy mode caps the schedutil target frequency
- ~474/500/554: `sugov_tinker_gamemode_util()` adds boost headroom, with a
  crash-safe reap throttle (~1 call per 256 schedutil updates) so a dead game
  cannot pin a CPU forever if its cleanup hook never ran

**`kernel/sched/syscalls.c`** — ~536: while gamemode boost is active, RT policy
requests are permitted where they would normally be denied

Verified by symbol table, not by inspection: `fair.o` carries undefined
references to `tinker_task_boosted` and `tinker_thermal_is_hot`,
`cpufreq_schedutil.o` references three tinker symbols, `syscalls.o` one — and all
of them resolve against tinker code in the linked `vmlinux`. **150 live
tinker/hyperdrive symbols** are in the image.

## hyperdrive was orphaned, and that was real

`hyperdrive.c` (379 lines, the GPU emulation helper) had **no entry in either
`tinker/Makefile` or `tinker/Kconfig`**. No configuration could ever build it, so
`hd_boost_thread` was absent from `vmlinux` — confirmed by symbol probe before the
fix, and present in `vmlinux` after it. Both files now have the entry, in the
surrounding style, and `make kernel/tinker/` compiles `hyperdrive.o` and archives
it into `built-in.a`.

Makefile and Kconfig are cross-checked in both directions: 38 symbols matched, no
Makefile symbol without a Kconfig entry, no `.c` without a Makefile entry.

## API currency against 7.x

The box runs 7.0.11 and the tree is 7.2.0-rc6, so the older timer API is gone.
Verified against `/usr/src/linux-headers-7.0.11-76070011/include`:

| old | status in 7.0.11 | replacement |
|---|---|---|
| `from_timer()` | **absent** | `container_of()` |
| `del_timer_sync()` | **absent** | `timer_delete_sync()` |

Applied in `update_monitor.c` and `desktop_state.c`, then swept all 38 modules for
`del_timer`, `del_timer_interrupt` and `add_timer` — no remaining uses.
`ktime_to_timespec()` → `ktime_to_timespec64()` in `desktop_state.c`, since the
32-bit variant truncates a 64-bit `ktime`.

## Two real bugs fixed, both build-verified

**An unreachable branch in `cloud_sync.c`.** `sync-complete` was handled under
`ret >= 3`, but the only `sscanf` above it has two conversions (`"%31s %63s"`),
so `ret` could never reach 3. A finished sync therefore never moved the provider
back to `CLOUD_IDLE`, never credited `bytes_synced` and never bumped
`files_synced` — any provider that completed a sync stayed stuck in
`CLOUD_SYNCING` forever, waiting for an event that could not arrive. Now guarded
on the two fields it actually requires, with the optional byte count validated by
its own `sscanf` return check.

**A ~1 KiB kernel stack frame in `hyperdrive.c`.** `hd_boost_thread()` had
`cpumask_t mask;` as a local. `cpumask_t` is sized by `NR_CPUS`, so on a large
machine that is ~1 KiB, putting the function at a 1096-byte frame — past the 1 KiB
soft limit and a real overflow risk against the 16 KiB kernel stack. Moved to
`kzalloc(cpumask_size(), GFP_KERNEL)` with a NULL check, so pinning is skipped
rather than faulting if the allocation fails.

## How to rebuild

The tree is already configured, so an incremental build is enough after touching
anything in `tinker/`:

    touch kernel/tinker/*.c
    make kernel/tinker/ -j$(nproc)     # 38 modules -> built-in.a
    make -j$(nproc)                    # full link -> vmlinux, bzImage

To confirm tinker code reached the image:

    nm vmlinux | grep -E ' [tT] ' | grep -cE 'tinker_| hd_'    # 150

## Notes for anyone extending this

- `core-y += kernel/tinker/` means anything added to `tinker/` is in the boot
  image immediately. There is no module-load step and no `insmod` to forget.
- A new module needs **three** things, not one: the `.c`, an `obj-$(CONFIG_TINKER_*)`
  line in `tinker/Makefile`, and a `config` block in `tinker/Kconfig`. The
  hyperdrive bug was exactly a module missing the latter two while looking
  completely fine.
- Prefer hooking into `fair.c` / `cpufreq_schedutil.c` / `syscalls.c` over adding
  new scheduler entry points. The existing sites are chosen so that TinkerOS
  behaviour composes with the stock scheduler (it never overrides affinity, and
  the reap path is crash-safe) rather than fighting it.
- Build artifacts are heavy here: `drivers/` alone is 21 GB, of which 11.3 GB is
  20,362 stale `.o` files. `make clean` reclaims that if disk gets tight, at the
  cost of a long rebuild.
