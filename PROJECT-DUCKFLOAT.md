# Project DuckFloat

**Codename for the complete creation of KorrinOS.**

Every decision that makes KorrinOS a real, shippable Linux distribution lives
under this project — the same way Mythos became Claude, a working name under
which the finished thing gets built.

- **Codename:** DuckFloat
- **Product:** KorrinOS
- **Kernel:** Linux `7.2.0-rc6-korrinos+`
- **Base:** Ubuntu 22.04 (casper), systemd 249
- **Repository:** `github.com/Aghosh-mv/KorrinOS`
- **Status:** booting, login proven, desktop in progress

---

## What DuckFloat is for

Not "add more features." DuckFloat is the job of making this thing **honest and
finished** — a Linux distribution anyone can download, install on a blank
laptop, and actually use.

## Decisions

These are settled and are not being revisited:

| Decision | Choice | Why it matters |
|---|---|---|
| **Release target** | **Public** | Anyone can download and install it. Claims must be accurate and the install experience must be clean. |
| **Testing** | **QEMU only** — no spare physical machine | Hardware-facing features cannot be proven here. They ship honestly labelled *unverified without real hardware*, never as working. |
| **Base distro** | **Ubuntu 22.04 — do not change** | The base provides the working substrate; which distro it is makes no difference to the goal. |

### What "public release" obligates us to do

1. **No claim may outrun its evidence.** The kernel's 38 advertised features
   were measured: 3 do anything observable. Publishing that list as-is would be
   a false claim to the public, which is the single worst outcome available to
   this project.
2. **No phone-home.** Nothing in KorrinOS may contact a server by default.
3. **Licence compliance.** The kernel is GPLv2 with the Linux kernel
   contributors' copyright intact; the KorrinOS additions are ours and must
   carry their own notices. Source for everything we ship must be available.
4. **The installer must be safe.** The audit flagged `installer.sh`,
   `korrinos-installer.sh`, `parcos-usb.sh` and the data shredder as dangerous
   — destructive `dd`/`mkfs`/`shred` on unsanitised devices. Those must be
   fixed before anyone else runs them.
5. **A README that is true.** Not a wish list.

---

## Where it stands

Measured, not assumed:

| | Status |
|---|---|
| Kernel boots, all cores | **Done** — `-smp 4` went 0/4 → 5/5; solid to 16 cores |
| Multi-core stall | **Fixed** — an added `pr_warn()` under two spinlocks in `affine_move_task()` |
| Clean-tree ISO build | **Fixed** — `kapt()` was defined inside a stage that runs *after* the preflight that checks for it |
| Boot to login shell | **Done** — bootloader → kernel → initramfs → squashfs → systemd → PAM → shell |
| Writable root | **Done** — overlayfs upper layer; read-only root was breaking logind, snapd and half the OS |
| `systemd-logind` | **active** |
| `lightdm` | **active**, but the screen is still black |
| Module signing | **Done** — `SYSTEM_TRUSTED_KEYS` was empty; the KorrinOS cert now loads |
| Graphical desktop | **Working** | Verified on-screen: OCR shows Applications launcher, clock, korrinos session. Root cause was render group (fixed + rebuilt). |
| Real hardware (Wi-Fi/audio/suspend) | **Not started** |

## The lesson that shaped DuckFloat

Every single failure in this project was **something that looked configured but
could not actually work**:

- a `pr_warn()` under two spinlocks → deadlocked the machine
- `SYSTEM_TRUSTED_KEYS=""` → no module could ever verify
- DRM display drivers built as modules → the ISO ships no modules, so X had no
  display
- `lightdm` installed but never enabled → no desktop, silently
- the live rootfs mounted read-only → every service that persists state failed
- 35 of 38 kernel modules compute a value that nobody ever reads

None of these announced themselves. That is the problem DuckFloat exists to
finish: **make things real, and make it visible when they are not.**

## The plan

### 1. Finish the desktop
LightDM starts, X does not draw. Find the missing piece and close it.

### 2. Ship kernel modules in the ISO
`make modules_install` appears nowhere in `os/build-distro.sh`. The image ships
a kernel with zero matching modules, which is why `systemd-modules-load` fails
and why so much was left built-in as a workaround.

### 3. Make the features real, split honestly

**Software-only — fully testable here, no special hardware:**
data shredder, hardware DNA, cache tiering, energy accounting, task boost, and
roughly a dozen more. Method: write the test first, watch it fail, implement
until it passes. No feature is "done" until a test distinguishes it from a
no-op.

**Hardware-facing — write them properly, verify on real hardware later:**
OLED wear, coil whine, dust dislodge, fan curves, battery life. QEMU has no
OLED, no microphone, no dust and no thermal zones. The code can be correct and
still be honestly labelled *unverified without real hardware*.

### 4. Ship
README that says exactly what works. No feature list that outruns the evidence.

---

*KorrinOS is the sole work of Aghosh-mv.*