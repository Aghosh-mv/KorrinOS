# Installer Safety

Scripts under `os/system/installer/` and `os/parc-ai/parcos-usb.sh` use destructive
primitives (`parted`, `mkfs`, `dd`, `shred`). Public-release isolation requires
`KORRINOS_ACCEPT_DESTRUCTIVE=1` in the environment before any destructive step
runs. Verification of target device is printed, and the operator must approve.

Citation: apply after `lsblk` review.
