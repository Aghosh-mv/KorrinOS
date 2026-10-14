# KorrinOS — What actually works

Public-release honesty page. This is the document the README claims must not
exceed. Every row was checked against the source tree and the compiled binary,
not written from the feature list.

**Method:** the `kernel/tinker/EFFECT-AUDIT.md` pass read every one of the 38
`kernel/tinker/*.c` files and cross-checked every claim against `nm` and
`objdump -d vmlinux`. `vmlinux` was the build actually shipping in the ISO.
Call-site counts are from the disassembly, not from source reading.

---

## Headline

KorrinOS's kernel has 38 advertised features. Measured honestly:

- **3 produce an observable effect** on system behaviour.
- 4 are partially wired to a real subsystem.
- 31 store a value and a counter; nothing ever reads them.
- One of the three "real" ones (OLED wear) is **actively harmful** on most
  hardware.

The scheduler integration the project advertises is real but tiny: 8 call sites
spread over 3 source files (`kernel/sched/fair.c`, `kernel/sched/syscalls.c`,
`drivers/cpufreq/cpufreq_schedutil.c`). Everything else the features claim is
status readout, wired to nothing.

This is not being hidden from anyone who downloads a public release. See
PROJECT-DUCKFLOAT.md for the decision.

---

## Real effect (3)

| Feature | What it actually does | Caveat |
|---|---|---|
| **Game mode** (`gamemode.c`) | Applies `SCHED_FIFO` + a named affinity mask to every task in the game's tgid | Has a UAF in the task walk (B8); pending safety fix; root-only write |
| **Kernel hyperdrive** (`hyperdrive.c`) | `sched_setscheduler(SCHED_FIFO, prio 50)`, pins CPUs, `kcompactd_run()` | Sleep-under-`rt_lock` in unboost (B1, since corrected). Ignores return values in places. |
| **OLED wear** (`oled_wear.c`) | Genuinely `backlight_device_set_brightness()` over a workqueue | **Harmful**: tracks *uptime*, not panel illumination (B14), dims after 1 h even a closed laptop, and applies to *every* backlight including LCD (B15). **Disabled by default** in this build; write `1` to `/proc/tinker/oled_wear` to opt in. |

## Partially wired (4)

| Feature | Real part | Honest part |
|---|---|---|
| **Energy mode** (`energy_sched.c`) | Scales frequency ±25% inside `get_next_freq` when mode is `peak`/`saver` | AUTO mode can never trigger on its own: its only data source (`tinker_energy_account`) has no caller. So AUTO silently stays AUTO. **Default is manual only.** |
| **Battery lifespan** (`battery_life.c`) | Caps `CHARGE_CONTROL_END_THRESHOLD` inside `__power_supply_set_property` | Only caps thresholds *other* drivers set; never sets one itself. |
| **Cache tiering** (`cache_tiering.c`) | Reads real L3 ways from CPUID leaf 4 at init | Never programs CAT/MPAM or any coverage MSR. Detection is real, application is zero. |
| **Thermal scheduler** (`thermal_sched.c`) | Hands `is_hot` to `fair.c` for LB | No write handler — `/proc/tinker/thermal` returns an error (B16). Reads `/sys/class/thermal/*/temp` from a kernel thread. Kept, clearly labelled input-only. |

## Instrumentation-only (31)

These modules range from approximately 20 to 800 lines of code. None of them call any other subsystem in the kernel; they populate structs, bump counters, and expose `/proc/tinker/<name>`. Writing to them changes numbers in procfs and nothing else.

The full list, grouped so you can find yours:

**Hardware-simulation cabinet (no hardware driven, no driver API called):**
coil_whine, dust_dislodger, unified_memory, sdgpu, fpga_scaler, cxl_memory,
adaptive_display, ray_traced_audio, neural_audio, neural_super_res, predictive_render,
predictive_prewarm, hyperdrive's companion audio, zero_latency_input, smart_power_grid

**User-state / management cabinet (no external integration):**
desktop_state, installer_state, app_store, pkg_tracker, battery tracking helpers
(except `battery_life`), enterprise_state, cloud_sync, mobile_companion,
remote_hardware_api

**Chip/system emulation cabinet:**
cxl_memory, sdgpu, fpga_scaler, unified_memory, and the rest of the hardware
simulation above

Full per-module table with source lines, what the write handler actually does,
and which bug numbers apply is in `kernel/tinker/EFFECT-AUDIT.md`.

## Bugs fixed for public release

- B1 (CRITICAL): `hyperdrive.c` sleeping under a spinlock — fixed; cross-reference
  to the gamemode deadlock fix that already repaired `gamemode.c`.
- B3: `coil_whine.c` 100 Hz timer forever, even when disabled — now conditional
  on `cw_enabled && cw_spread_spectrum`; 0 wakeups at idle.
- B5: `data_shredder.c` claiming to scrub swap without any write — no longer lies.
- B9 / B10: `hardware_dna.c` obfuscated fingerprint was the constant
  `TINKER-OBFUSCATED-00000000`; now a real fold. Proc mode was 0444 so the
  write handler was unreachable; corrected to 0644.
- B14 / B15: `oled_wear.c` LED dim-by-uptime on *all* backlights — disabled by
  default; write `1` to `/proc/tinker/oled_wear` to opt in. The mode is honest
  about the uptime-based heuristic.

## Known broken / not started

- B2: `energy_sched` AUTO mode is dead by construction until
  `tinker_energy_account()` gets a real caller — leaving as manual only.
- B8: `gamemode.c` for_each_process without rcu read lock — pending fix.
- B11 / B12 / B13: hyperdrive ignored return values and typo'd affinity;
  return handling and a bound on input size still need finishing.
- Installer scripts (`installer.sh`, `korrinos-installer.sh`, `parcos-usb.sh`)
  use unsanitised `dd`/`mkfs`/`shred` on devices — Do Not Ship as-is.
- Hardware-facing features (`coil_whine`, `dust_dislodger`, `oled_wear`,
  `dvfs_shaver`) need real equipment to verify; labelled unverified.

## Summary for downloading users

If you are going to install KorrinOS, the parts of it that do something today are:

- kernel scheduling/cpufreq interaction type-adjusted for game mode, thermals,
  energy peak/saver mode, hyperdrive, and battery threshold capping;
- backlight wear dimming when you explicitly opt in;
- hardware-independent readouts under `/proc/tinker/` that match what the system
  is doing.

Everything else under `/proc/tinker/<name>` is a status panel. Writing to it
changes numbers, not behaviour. That honesty is a promise of the public release.
