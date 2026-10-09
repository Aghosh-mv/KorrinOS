# KorrinOS MASTER TODO

## THE ONE GOAL
Boot KorrinOS on a blank laptop with no OS installed, and use it with no bugs,
like a professional release. Do not claim it works until it does, verified.

## P0 — BLOCKERS (nothing else matters until these are done)
- [ ] Root-cause early-boot stall. Currently 0/5 boots. Get backtrace/lockdep,
      fix, verify 5/5.
- [ ] Multi-core stall at -smp 3+ (0/4). Every real machine and VM has >2 cores.
- [ ] B1 hyperdrive.c: sleeping sched_setscheduler() under rt_lock spinlock.
- [ ] B8 gamemode_apply(): for_each_process() with no rcu_read_lock -> UAF.
- [ ] 15 DANGEROUS os/ scripts (nft flush ruleset at boot; Security Center
      runs `ufw disable`; dd/mkfs on unsanitised devices).
- [ ] korrinos-firewall.sh:193 hardcodes `tcp dport`, so whole UDP gaming
      profile applies as TCP.
- [ ] Real-hardware boot verification: install to blank laptop, confirm
      GRUB -> kernel -> rootfs -> login -> desktop, then WiFi/audio/graphics.

## P1 — CORRECTNESS BUGS IN SHIPPING CODE
- [ ] B2 energy_sched: tinker_energy_account() never called => ratio always 0
      => cpufreq PEAK/SAVER dead by construction.
- [ ] B6 dead sscanf branches: driver_monitor.c, hw_cert.c,
      mobile_companion.c — load/test commands can NEVER run.
- [ ] B14/B15 oled_wear dims ordinary LCD backlights one-way after 1h uptime.
- [ ] B17 thermal_sched indexes cpu_temp_mc[] unguarded by possible-CPU loop.
- [ ] B10/B16 hardware_dna node is 0444 (write unreachable); thermal_sched node
      is 0644 with no write handler.
- [ ] B11 hyperdrive cpumask_set_cpu(1) with no nr_cpu_ids check.

## P2 — HONESTY OF THE FEATURE SET
- [ ] Decide fate of 31 instrumentation-only tinker modules: implement or delete.
      data_shredder reports success writing 0 bytes; hardware_dna fingerprint is
      always TINKER-OBFUSCATED-00000000 (get_cycles() & 0).
- [ ] Wire or delete 7 exported-but-never-called symbols.
- [ ] Top-10 user-space stubs (STUB-AUDIT.md).
- [ ] Compile hardware-tech C backends in build-distro.sh.
- [ ] Only 22% of os/ is reachable; 52% ships but can never be invoked.

## P3 — DISTRIBUTION
- [ ] Push 34 commits to origin (Aghosh-mv, admin confirmed).
- [ ] SourceForge: project `korrinos` returns 404. Need it created, OR grant
      aghoshpratheesh upload rights on existing `tinker`. Then upload ISO.

## P4 — UX / POLISH (after P0 and P1)
- [ ] Cursor themes: SweezyCursors .ani (animated) + .cur -> Xcursor, system-wide
      incl. GDM login + installer; Mouse Designs picker in Settings.
- [ ] Starfield loading screen.
- [ ] Instant reload preserving exact app state across shutdown.
- [ ] Alex floating assistant: rotating cube, Hermes backend, one continuous
      reconnecting session. Much more polished build, on Zegrate AI after
      training completes. PLAN ONLY for now.
- [ ] Evaluate smooth-ui library.

## FACTS (verified 2026-10)
- Our code ~53k lines / ~10.5k files. Repo 96,059 files, 85,541 are upstream Linux.
- Sole author: Aghosh-mv (aghoshpratheesh@gmail.com). All 78 commits. No AI
  co-authorship and no other contributors are claimed.
- Tests: kcommand 128/128, wordpen 94/94 (85 unit + 9 data), shell 467 bash -n,
  shellcheck error-clean, 49 Python clean.
- smp-bringup-test.sh and iso-boot-test.sh are the boot regression gates.
