# KorrinOS — Age gating (profiles + content policy)

Status: spec. Published as optional feature because the gatingsystem ships **empty**
of gated content. No adult material ships in the ISO.

## Model

- Accounts are tagged `minor / standard / adult` in a local attributes file.
- An `AgeGate` service (D-Bus, local, no network) answers `IsAdult(session)`.
- Restricted packs carry a `restricted: true` + `min_age` manifest field and are
  stored under `/usr/share/korrinos/restricted/` with mode 0750, group `render`
  (reused for render access) but read only after a gate check passes.
- The wallpaper picker calls the gate before *listing* restricted items, and a
  converter run at first boot records whether each account is child-only per the
  household parent flow.
- A restricted item can only be set for one account and only remains applied
  while that account's session owns the screen.

## Why the pack is empty

Shipping any adult content — even with a gate — puts the bytes on every public
mirror and puts the legal exposure on the publisher rather than the downloader.
Self-attestation is not age assurance under UK OSRA and EU DSA, so a gating
toggle alone would not satisfy the stated public-release constraint
"no claim may outrun its evidence". The mechanism is real; the content
defaults to absent, and users self-source.

## Household flow

First adult account creation (TTY or GUI installer):
  1. "Is this a household/multi-user machine?"
  2. If yes: create admin parent account + child accounts.
  3. Child accounts never see restricted ranks; only adult-confirmed
     sessions can unlock packs.
  4. The toggle is audited in `/var/log/korrinos/account-lifecycle.log`
     (yes/no, timestamp, no pack identifiers).

## Testing in QEMU

- account create flow under lightdm
- restricted directory permissions
- gate cross-user isolation
- gate black-box audit

## Integration points (later)

- `/usr/share/korrinos/wallpapers/*` packs voluntary lock-down via manifest
- App store items use the same `min_age` field
- "Family" policy can be applied at session start by the first systemd user manager
