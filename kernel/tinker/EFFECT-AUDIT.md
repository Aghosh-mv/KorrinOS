# KorrinOS `kernel/tinker/` — Effect Audit

Audit of all 38 `kernel/tinker/*.c` modules: does each one have an observable
effect on real system behaviour, or is it a status readout / dead state?

Method: read every source file; `grep` the whole tree for callers; verify every
claim against `nm` and `objdump -d vmlinux` (call-site counts taken from the
actual disassembly, not from source reading alone).

Config in effect: `CONFIG_NR_CPUS=8192`, `CONFIG_SMP=y`, `CONFIG_NUMA=y`.

## Verdict summary

| Verdict | Count |
|---|---|
| REAL EFFECT (provably changes system behaviour) | **3** |
| PARTIAL (wired to a real subsystem, but gated or mostly inert) | **4** |
| INSTRUMENTATION-ONLY (stores a variable + counter; nothing reads it) | **31** |
| DEAD (exported symbol with zero references anywhere) | *7 symbols, inside the above* |

The headline: **only 3 of 38 modules do anything a user could observe, and one of
those 3 (`oled_wear`) is actively harmful.** The scheduler/cpufreq integration the
project has been advertising as "real kernel work" reduces to 5 call sites into 3
files, and one of the three subsystems involved (`energy_sched`) has a dead
automatic mode because its only data source is never called.

---

## The integration points that DO exist (verified)

These are real, compiled in, and reachable. Call-site counts are from
`objdump -d vmlinux`:

| Symbol | Call sites in vmlinux | Caller(s) |
|---|---|---|
| `tinker_task_boosted` | 3 | `kernel/sched/fair.c:9610`, `cpufreq_schedutil.c:478` (+inlined copy) |
| `tinker_thermal_is_hot` | 2 | `kernel/sched/fair.c:9621,9623` |
| `tinker_gamemode_enabled` | 2 | `kernel/sched/syscalls.c:543` |
| `tinker_gamemode_reap_finished` | 3 | `cpufreq_schedutil.c:452` |
| `tinker_energy_mode` | 1 | `cpufreq_schedutil.c:208` (`get_next_freq`) |
| `tinker_oled_get_dim` | 1 | `drivers/video/backlight/backlight.c:198` |
| `tinker_battery_lifespan` | 1 | `drivers/power/supply/power_supply_core.c:1330` |
| `tinker_battery_envelope` | 1 | `drivers/power/supply/power_supply_core.c:1332` |

All are guarded by `IS_ENABLED(CONFIG_TINKER_*)`, all of those Kconfig options are
enabled in this build (the call instructions exist in the binary), and none are
`static inline`d away. **The scheduler integration is real.**

### Exported symbols with ZERO references (confirmed at binary level)

```
callers of tinker_gamemode_request_boost : 0
callers of tinker_thermal_hint_hot_cpu   : 0
callers of tinker_energy_account         : 0   (grep: 0 outside kernel/tinker)
callers of tinker_energy_ratio           : 0
callers of tinker_oled_wear_seconds      : 0
callers of hd_pin_memory                 : 0
callers of hd_alloc_huge_pages           : 0
```

`tinker_gamemode_request_boost()` is **dead**, exactly as suspected. `gamemode_write()`
does not call it — it duplicates the logic inline (`gamemode.c:238-254`). The
documented "kernel-internal hook ... so no user-space involvement is required"
(`gamemode.c:78-82`) has no kernel-internal caller.

`tinker_thermal_hint_hot_cpu()` is also dead, which matters: it is the only API
that would let any other subsystem *feed* the heat map. Its absence is why
thermal_sched is only PARTIAL (see below).

---

## Per-module table

| # | Module | What the write handler actually does | Called from elsewhere? | Real subsystem? | Verdict | Bugs |
|---|---|---|---|---|---|---|
| 1 | `tinker.c` | none (core; creates `/proc/tinker`, read-only status) | n/a — core | `/proc` only | INSTRUMENTATION-ONLY | none |
| 2 | `adaptive_display.c` | stores `ad_mode`/`ad_refresh_hz`, `ad_frames++`. Includes `<linux/backlight.h>` but never calls it | No | No | INSTRUMENTATION-ONLY | — |
| 3 | `app_store.c` | `memmove`s elements in a local `installed_app[]` array, bumps counters | No | No | INSTRUMENTATION-ONLY | — |
| 4 | `battery_life.c` | sets `lifespan_mode`, clamps `charge_min`/`charge_max` | **Yes** — `power_supply_core.c:1330` | **Yes** — `__power_supply_set_property` | **PARTIAL** | see B7 |
| 5 | `cache_tiering.c` | Genuinely executes CPUID leaf 4 to detect L3 ways at init; write only sets `cache_l3_realtime_ways` | No | No — never programs CAT/MPAM; `<linux/cache.h>` unused | **PARTIAL** | detection is real, application is zero |
| 6 | `cloud_sync.c` | mutates `cloud_provider` struct array + counters | No | No | INSTRUMENTATION-ONLY | was fixed (see note) |
| 7 | `coil_whine.c` | flips booleans; a timer jitters `cw_freq_khz` | No | No PWM/VRM anywhere | INSTRUMENTATION-ONLY | **B3, B4** |
| 8 | `cxl_memory.c` | stores `cxl_enabled`, `cxl_pool_cxl_mb` | No | No CXL API | INSTRUMENTATION-ONLY | — |
| 9 | `data_shredder.c` | `shred_scrub_pages += 4096`, counter. Includes `<linux/swap.h>`, calls nothing | No | No — **zero bytes are ever scrubbed** | INSTRUMENTATION-ONLY | B5 |
| 10 | `desktop_state.c` | copies strings into a `desktop_state` struct; `workspace_count`, `monitor_count` | No | No | INSTRUMENTATION-ONLY | — |
| 11 | `driver_monitor.c` | mutates `driver_info[]` | No | No | INSTRUMENTATION-ONLY | **B6 (2 dead branches)** |
| 12 | `dust_dislodger.c` | extensive real envelope *arithmetic*; `run` only does `pr_info()` + `dust_runs++` | No | **No fan driver, no PWM, no hwmon** | INSTRUMENTATION-ONLY | elaborate no-op |
| 13 | `dvfs_shaver.c` | stores `dvfs_state`; `shave` bumps a counter | No | No cpufreq API | INSTRUMENTATION-ONLY | — |
| 14 | `energy_sched.c` | sets `energy_mode` | **Yes** — `cpufreq_schedutil.c:208` | **Yes** — `get_next_freq()` scales freq ±25% | **PARTIAL** | **B2** |
| 15 | `enterprise_state.c` | sets `domain_joined`/`sso_active`/`mfa_enabled`/… + audit log | No | No AD/SSO/MFA integration | INSTRUMENTATION-ONLY | — |
| 16 | `finance_audit.c` | `fa_last_seen_days[i]++` | No | No | INSTRUMENTATION-ONLY | — |
| 17 | `fpga_scaler.c` | stores `fpga_precision_bits` | No | No | INSTRUMENTATION-ONLY | — |
| 18 | **`gamemode.c`** | **applies `sched_setscheduler_nocheck(p, SCHED_FIFO, prio)` to every task in the tgid; sets `gamemode_tgid`** | **Yes ×8** | **Yes** — scheduler + cpufreq | **REAL EFFECT** | **B1, B8** |
| 19 | `hardware_dna.c` | sets `dna_obfuscated`, recomputes digest | No | No | INSTRUMENTATION-ONLY | **B9, B10** |
| 20 | `hardware_tuning.c` | stores `tune_governor`/`tune_fan_curve` | No | No cpufreq/pwm/hwmon | INSTRUMENTATION-ONLY | — |
| 21 | `hw_cert.c` | mutates `cert_profile[]` | No | No | INSTRUMENTATION-ONLY | **B6 (dead branch)** |
| 22 | **`hyperdrive.c`** | **`sched_setscheduler()` to SCHED_FIFO prio 50, `set_cpus_allowed_ptr()`, `kcompactd_run()`** | No (user-space driven) | **Yes** — scheduler, affinity, compaction | **REAL EFFECT** | **B1, B11, B12, B13** |
| 23 | `installer_state.c` | copies strings into `installer_state`; `phase`, `percent_complete` | No | No | INSTRUMENTATION-ONLY | — |
| 24 | `mobile_companion.c` | mutates `mobile_device[]` | No | No USB/net/bluetooth | INSTRUMENTATION-ONLY | **B6 (dead branch)** |
| 25 | `neural_audio.c` | stores `na_gain_pct`/`na_noise_floor`, `na_frames++` | No | No ALSA | INSTRUMENTATION-ONLY | — |
| 26 | `neural_super_res.c` | stores `nsr_scale`, `nsr_frames++` | No | No DRM | INSTRUMENTATION-ONLY | — |
| 27 | **`oled_wear.c`** | **sets `oled_dim_pct`; a 1 Hz workfn decrements it after 1 h of *uptime*** | **Yes** | **Yes** — `backlight_device_set_brightness()` | **REAL EFFECT (harmful)** | **B14, B15** |
| 28 | `pkg_tracker.c` | `strscpy` into `pkg_event` array, counters | No | No rpm/`pkg_*` | INSTRUMENTATION-ONLY | — |
| 29 | `predictive_prewarm.c` | stores `pw_target_pid`, `pw_hits++`/`pw_misses++` | No | No — does not touch a page cache or fault anything in | INSTRUMENTATION-ONLY | — |
| 30 | `predictive_render.c` | stores `pr_prewarm_frames`, hit/miss counters | No | No DRM/present | INSTRUMENTATION-ONLY | — |
| 31 | `ray_traced_audio.c` | stores room dims/reflectivity; computes a Sabine RT60 number | No | No ALSA/PCM | INSTRUMENTATION-ONLY | — |
| 32 | `remote_hardware_api.c` | permission bitmask bookkeeping, `rha_calls++` | No | No | INSTRUMENTATION-ONLY | — |
| 33 | `sdgpu.c` | stores `sdgpu_compute_share`, `sdgpu_power_cap_pct` | No | No DRM/PCI | INSTRUMENTATION-ONLY | — |
| 34 | `smart_power_grid.c` | stores `pg_profile`, `pg_avg_load_pct` | No | No | INSTRUMENTATION-ONLY | — |
| 35 | `thermal_sched.c` | **none** — `thermal_fops` has no `.proc_write` at all. A 5 s workfn re-reads `/sys/class/thermal/*/temp` via `filp_open`+`kernel_read` | **Yes** — `fair.c:9621` reads `is_hot` | **Yes** — scheduler. **No** thermal API | **PARTIAL** | **B16, B17, B10** |
| 36 | `unified_memory.c` | stores `um_enabled`/`um_local_dram_mb` | No | No memory tiering | INSTRUMENTATION-ONLY | — |
| 37 | `update_monitor.c` | sets an enum state + counters; `timer_setup` for an uptime timer | No | No reboot/`kernel_restart` | INSTRUMENTATION-ONLY | — |
| 38 | `zero_latency_input.c` | stores `zl_target_priority`, `zl_events++`. Header claims PM_QOS/sched_latency export; **neither exists** | No | No PM_QOS | INSTRUMENTATION-ONLY | misleading header |

---

## Hardware-specific modules (as requested)

Plain statement of whether each drives real hardware:

| Module | Real hardware/subsystem? |
|---|---|
| `oled_wear` | **Yes — and it is harmful.** `backlight_device_set_brightness()` genuinely rescales brightness. See B14. |
| `battery_life` | Partially. Only clamps `POWER_SUPPLY_PROP_CHARGE_CONTROL_END_THRESHOLD`, and only for drivers that implement that property. Never sets it, so it only *caps* thresholds others set. |
| `oled` (`/proc/tinker/oled`) | Same module as `oled_wear` above. |
| `coil_whine` | **No.** No PWM, no VRM, no MSR. `cw_freq_khz` is read by nothing but `cw_show`. |
| `dust_dislodger` | **No.** No fan driver, no `pwm`, no hwmon tach. `run` prints a message. |
| `fpga_scaler` | **No.** No FPGA, no fabric, no bitstream. One `unsigned int`. |
| `cxl_memory` | **No.** No `cxl_mem`, no `memdev`, no `cxl/core.h`. Four integers. |
| `sdgpu` | **No.** No DRM, no PCI, no `drm_dev`. Two integers. |
| `hardware_dna` | **No.** Folds 4 loop iterations of a constant salt. Reads no DMI, no serial, no vendor UUID, despite the header claiming "CPU, DMI, and serial components". |
| `data_shredder` | **No.** Includes `<linux/swap.h>`, calls nothing from it. Never writes a byte to swap. |
| `ray_traced_audio` | **No.** No ALSA, no PCM, no `snd_pcm`. Header admits "can run here or in a user-space audio daemon". |
| `neural_audio` | **No.** No ALSA, no DSP. |
| `adaptive_display` | **No.** Includes `<linux/backlight.h>`, calls nothing from it. |
| `unified_memory` | **No.** No memory tiering, no ZONE_DEVICE. |
| `zero_latency_input` | **No.** No PM_QOS, no input subsystem, no scheduler change. |
| `dust_dislodger` (2nd) | **No.** |

---

## BUGS FOUND

### B1 — Sleeping in atomic context (CRITICAL) — `hyperdrive.c:149`, `:358`

`hd_unboost_thread()` calls `sched_setscheduler()`, `put_task_struct()` and
`kfree()` **while holding `rt_lock`, a raw spinlock** (`spin_lock(&rt_lock)` at
`hyperdrive.c:143`, `spin_unlock` only at `:167`):

```c
spin_lock(&rt_lock);
list_for_each_entry_safe(rt, tmp, &render_threads, list) {
    if (rt->pid == pid) {
        list_del(&rt->list);
        sched_setscheduler(rt->task, SCHED_NORMAL, &param);  /* :149 SLEEPS */
        set_cpus_allowed_ptr(rt->task, cpu_all_mask);
        put_task_struct(rt->task);
        kfree(rt);
        spin_unlock(&rt_lock);
```

`__sched_setscheduler()` takes `tasklist_lock` (an rwsem) and allocates; it can
sleep. `put_task_struct()` can drop the last reference and run
`delayed_put_task_struct()`. Both are illegal under a spinlock — this will
produce `scheduling while atomic` / BUG splats. **The exact same pattern repeats in
`hyperdrive_exit()` at `hyperdrive.c:354-362`**, which additionally holds the
spinlock across `sched_setscheduler()` during module teardown.

Note this is the *opposite* of the gamemode deadlock that was recently fixed
carefully in `gamemode.c` — `gamemode_apply()` correctly drops `gamemode_lock`
before calling `sched_setscheduler_nocheck()`. `hyperdrive.c` has the identical
class of bug and was not fixed.

### B2 — `energy_sched` AUTO mode is dead by construction — `energy_sched.c:70-95`

`tinker_energy_account()` has **zero callers**, so `energy_ticks_idle` and
`energy_ticks_busy` are permanently 0. Therefore
`energy_idle_busy_ratio()` returns 0 forever, and in `tinker_energy_mode()` the
`mode == ENERGY_MODE_AUTO` path hits `if (ratio == 0) { /* stay auto */ }` and
returns `AUTO`. Since `AUTO` is both the initial value and the value users get by
writing `auto`, **the cpufreq PEAK/SAVER scaling at `cpufreq_schedutil.c:208-214`
is dead unless someone explicitly writes `peak` or `saver`.** The automatic
behaviour the module is named for can never trigger.

### B3 — Unconditional 10 ms timer — `coil_whine.c:110-128`

`cw_spread_timer_fn()` re-arms itself with `mod_timer(..., msecs_to_jiffies(10))`
at the `resched:` label on **every** path, including the early-out when the
feature is disabled. `timer_setup()` + `mod_timer()` run unconditionally in
`tinker_cw_init()`. That is **100 timer wakeups per second, forever, on every
boot**, to jitter an integer that nothing reads. It also holds no reference to
the module.

### B4 — `cw_spread_khz` is unreachable and jitter can go negative — `coil_whine.c:78,120`

The `"spread N"` command parses `val` but only ever sets the boolean
`cw_spread_spectrum`; `cw_spread_khz` is therefore permanently 50 and can never be
configured. Relatedly `cw_freq_khz = cw_base_freq_khz + jitter - cw_spread_khz`
at `:120` ranges over `[base-50, base+49]` — an asymmetric window that can drive
the "frequency" below the declared base, the opposite of the stated policy.

### B5 — `data_shredder` claims to scrub swap and does nothing — `data_shredder.c:80-83`

```c
} else if (!strcmp(cmd, "run")) {
    /* best-effort: scrub a bounded number of free swap pages */
    shred_scrub_pages += 4096;
    atomic64_inc(&shred_ops);
```

No swap function is called. Not one byte of swap is written. The counter is
incremented anyway and reported as `scrub_pages` in `/proc/tinker/shredder`,
so the readout actively asserts that scrubbing happened. Given the module's
stated purpose is a *privacy wipe*, a readout that lies about having wiped is
the worst failure mode available.

### B6 — Dead branches: `sscanf` conversion count < `ret >= N` guard

The conversion count and the guard disagree, so the branch can never execute.
This exact bug class was already found and fixed in `cloud_sync.c` (see the
comment at `cloud_sync.c:151-157`); **it was never checked in these three files.**

| Location | `sscanf` | max `ret` | Guard | Effect |
|---|---|---|---|---|
| `driver_monitor.c:135` | `"%31s %63s"` (`:129`) | 2 | `ret >= 3` | `load` command **never runs** |
| `driver_monitor.c:164` | `"%31s %63s"` (`:129`) | 2 | `ret >= 3` | `update-available` **never runs** |
| `hw_cert.c:111` | `"%31s %63s"` (`:105`) | 2 | `ret >= 5` | `test` command **never runs** |
| `mobile_companion.c:185` | `"%31s %63s"` (`:132`) | 2 | `ret >= 3` | `battery` command **never runs** |

These commands are accepted by the parser and then silently dropped into the
final `else { return -EINVAL; }`. `driver_monitor`'s entire purpose is tracking
driver load state and it cannot record a load event.

### B7 — `tinker_gamemode_request_boost()` is dead code — `gamemode.c:83-95`

Exported, has a detailed comment describing a kernel-internal caller
("Called from the scheduler/fair integration when a process is detected, so no
user-space involvement is required", `:78-82`), and has **zero call sites in
`vmlinux`**. The described integration was never written.

### B8 — Unprotected tasklist walk (use-after-free) — `gamemode.c:67-75`

```c
for_each_process(p) {
    if (task_tgid_nr(p) == tgid) {
        ...
        sched_setscheduler_nocheck(p, SCHED_FIFO, &param);
    }
}
```

`for_each_process()` is defined at `include/linux/sched/signal.h:640-641` as
`for (p = &init_task; (p = next_task(p)) != &init_task; )` — a bare
`next_task()` walk with **no `rcu_read_lock()`**. The comment at `gamemode.c:61-66`
argues this is safe because `tgid` is snapshotted and because
`sched_setscheduler_nocheck()` takes its own reference — but `task_tgid_nr(p)` is
evaluated on `p` *before* that reference is taken, on an unreferenced task
pointer. If the process group is exiting concurrently, this is a UAF.
`gamemode_show()` (`:187-194`) and `tinker_gamemode_reap_finished()` (`:158-165`)
in the *same file* correctly use `rcu_read_lock()`; `gamemode_apply()` does not.
It should simply hold `rcu_read_lock()` across the walk.

### B9 — Obfuscated hardware DNA is a constant — `hardware_dna.c:41`

```c
unsigned int mix = (unsigned int)get_cycles() & (get_cycles() ? 0 : 0);
```

`& 0` — `mix` is unconditionally 0. So the "obfuscated" fingerprint at `:52-53` is
always the literal string `TINKER-OBFUSCATED-00000000`. Obfuscation mode
destroys the fingerprint instead of hiding it. (The `get_cycles() ? 0 : 0` is
also a no-op that a compiler is entitled to fold away entirely.)

### B10 — `hardware_dna` write path is unreachable — `hardware_dna.c:117`

```c
proc_create("hwdna", 0444, tinker_proc_root, &hwdna_fops)
```

Mode `0444` grants no write bit to anyone, but the file has a `.proc_write`
handler. **`/proc/tinker/hwdna` cannot be written at all.** Any QEMU test that
"wrote" this node did not actually reach `dna_write()`. The same mismatch
exists in reverse in `thermal_sched` (below).

### B11 — Silent affinity failure / invalid CPU on small machines — `hyperdrive.c:120-123`

```c
cpumask_set_cpu(0, mask);
cpumask_set_cpu(1, mask);
set_cpus_allowed_ptr(task, mask);
```

CPU 1 is set unconditionally, with no `cpu < nr_cpu_ids` check. On a
single-vCPU machine (exactly the QEMU configuration used for the boot tests)
`set_cpus_allowed_ptr()` sanitises against `cpu_possible_mask`, strips the
non-existent CPU 1, and is left with an **empty mask → `-EINVAL`**. The return
value is discarded. So on the test machine the pinning step is a no-op that
silently fails while the code proceeds to report success.

### B12 — Ignored return values — `hyperdrive.c:110`, `:122`, `:157`

`sched_setscheduler()` and both `set_cpus_allowed_ptr()` calls have their return
values discarded. If `SCHED_FIFO` prio 50 is refused, `rt->priority_boosted` is
still set to 1 at `:106` and the thread is still listed as boosted. `hd_proc_show()`
then prints a **hardcoded** `"  PID %d: priority=50, cpus=0,1\n"` at `:247`
regardless of what actually happened — so `/proc/hyperdrive/status` misreports
state even when every call failed.

### B13 — Permanent memory leak + unbounded allocation order — `hyperdrive.c:206`

`hd_alloc_huge_pages()` calls `alloc_pages(GFP_HIGHUSER|__GFP_COMP, order_base_2(count * 2))`
and never frees the pages. `order_base_2()` on an attacker-influenced `count`
with no upper clamp can exceed `MAX_PAGE_ORDER`. Both are currently latent
only because the function has **zero callers** (see the dead-export list) — it is
reachable only if a future out-of-tree module calls the exported symbol.

### B14 — OLED wear tracks uptime, not panel illumination — `oled_wear.c:113-124`

`oled_wear_workfn` runs every second from boot and does `oled_wear_seconds++`
unconditionally. It has no idea whether the panel is on, whether the lid is
shut, or whether the machine is docked. After **one hour of system uptime** it
starts clamping `oled_dim_pct` down toward 70%, and the value is
**one-way** — nothing ever raises it back. A laptop left closed in a bag for an
hour and then opened comes back with a dimmed backlight.

### B15 — The wear dim is applied to *every* backlight, including LCD — `backlight.c:194-201`

```c
extern unsigned int tinker_oled_get_dim(void);
unsigned int dim = tinker_oled_get_dim();
if (dim > 0 && dim < 100)
    brightness = (brightness * dim) / 100;
```

This sits inside `backlight_device_set_brightness()` with **no panel-type
check**. It scales the requested brightness of every backlight device in the
system, so combined with B14 an ordinary **LCD** laptop's backlight is silently
reduced to 70% after an hour of uptime. The module is named for OLED burn-in;
on non-OLED hardware it is a pure, invisible brightness regression.

### B16 — `thermal_sched` has no write handler at all — `thermal_sched.c:158-163`

`thermal_fops` omits `.proc_write`, yet the node is created `0644`. Writing to
`/proc/tinker/thermal` returns an error. The claim that all 38 nodes "accept
writes" is false for this one, and there is nothing to accept — the heat map has
no manual input path at all.

### B17 — `cpu_temp_mc[]` indexed by possible-CPU, sized by `NR_CPUS` — `thermal_sched.c:115-123`, `:145-147`

`heat.cpu_temp_mc` is `[NR_CPUS]`, but both `thermal_decay_workfn()` and
`thermal_show()` iterate `for_each_possible_cpu(cpu)` and write
`heat.cpu_temp_mc[cpu]` unguarded. If `nr_possible_cpus > NR_CPUS` this is an
out-of-bounds write into the adjacent static `struct tinker_heat_map` fields
(`cpu_hot_thresh_mc`, `zone_temps[]`, `enabled`) — which, given the field order
at `:37-40`, means a runaway CPU index can overwrite `enabled` and the threshold
and thus drive the whole scheduler heuristic arbitrarily. The other accessors
(`tinker_thermal_hint_hot_cpu` `:53`, `tinker_thermal_is_hot` `:69`) *do* clamp
against `nr_cpu_ids`; these two loops do not. Note `CONFIG_NR_CPUS=8192` here,
and `cpu_temp_mc` alone is 64 KB of BSS.

### B18 — Obsolete procfs API on the removal path — `hyperdrive.c:339`, `:366`

`remove_proc_entry(HD_PROC_DIR, NULL)` passes the literal string `"hyperdrive"`
as a name and `NULL` as the parent. This API is deprecated (the modern call is
`proc_remove(hd_proc_dir)`), and `include/linux/proc_fs.h:216` shows it is
compiled to a no-op in some configurations. Since `hd_proc_dir` is already held
in a variable, `proc_remove()` is both correct and available.

### B19 — `thermal_sched` opens sysfs files from a workqueue every 5 s — `thermal_sched.c:93-111`

Up to 8 × (`filp_open` + `kernel_read` + `filp_close`) every 5 seconds, with a
`char path[64]` and `snprintf` per zone, in a `delayed_work`. Legal but slow,
and it completely bypasses the thermal framework that already exists
(`thermal_zone_device_register()` / `thermal_zone_get_temp()` / the
`thermal_notifier` chain). It also means the whole feature depends on
`/sys/class/thermal` having non-contiguous zone indices, and it silently
degrades to all-zero on any machine with no thermal zones — which is the QEMU
case, so `tinker_thermal_is_hot()` returns `false` forever in every test run.

---

## Note on `cloud_sync.c`

`cloud_sync.c` is the one place where the B6 bug class was already caught and
fixed — the comment block at `cloud_sync.c:151-157` documents the original
`ret >= 3` dead branch and the fix that moved it to `ret >= 2`. The audit found
the identical unfixed bug in `driver_monitor.c`, `hw_cert.c` and
`mobile_companion.c` (B6). Whatever review pass produced that fix did not
generalise it.
