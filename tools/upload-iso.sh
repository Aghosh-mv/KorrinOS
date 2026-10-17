#!/bin/bash
# upload-iso.sh - publish a built KorrinOS ISO to SourceForge over SSH.
#
# Requires:
#   1. An SSH key registered on SourceForge (works today for aghoshpratheesh)
#   2. An existing /upload/<project> tree on frs.sourceforge.net.
#      SourceForge provisions this only after the first file is uploaded once
#      through the web "Files" UI, so do that first if /upload is ABSENT.
#
# Usage:  ./upload-iso.sh [ISO] [SUBDIR]
#   ISO      path to the ISO            (default: repo KorrinOS-v2.0.iso)
#   SUBDIR   destination folder name     (default: v2.0.1)
set -euo pipefail

SF_USER="${SF_USER:-aghoshpratheesh}"
SF_PROJECT="${SF_PROJECT:-tinkeros}"
SF_HOST="frs.sourceforge.net"

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ISO="${1:-$REPO/KorrinOS-v2.0.iso}"
SUBDIR="${2:-v2.0.1}"

[ -f "$ISO" ] || { echo "no such ISO: $ISO" >&2; exit 1; }
command -v sha256sum >/dev/null || { echo "sha256sum required" >&2; exit 1; }

echo "ISO      : $ISO"
echo "size     : $(du -h "$ISO" | cut -f1)"
echo "sha256   : $(sha256sum "$ISO" | cut -d' ' -f1)"
echo "target   : $SF_HOST:/upload/$SF_PROJECT/$SUBDIR/"
echo

# 1. Confirm the upload tree exists before spending 4.8 GB of bandwidth.
if ! timeout 25 sftp -o BatchMode=yes \
      "$SF_USER,$SF_PROJECT@$SF_HOST:/upload/$SF_PROJECT" <<< "ls" 2>&1 \
      | grep -qv "not found"; then
  cat >&2 <<EOF
ERROR: /upload/$SF_PROJECT does not exist on SourceForge yet.

Upload any small placeholder file once through the web UI:
  https://sourceforge.net/projects/$SF_PROJECT/files/
    -> "Add new release" / "Add files"

SourceForge only creates the SSH upload tree after the first web upload.
Re-run this script once that exists.
EOF
  exit 2
fi

echo "upload tree OK - streaming (this takes a while for 4.8 GB)..."
timeout 7200 sftp -o BatchMode=yes \
  "$SF_USER,$SF_PROJECT@$SF_HOST:/upload/$SF_PROJECT/$SUBDIR" <<EOF
mkdir $SUBDIR
put $ISO
EOF

echo
echo "uploaded. Publish the release here:"
echo "  https://sourceforge.net/projects/$SF_PROJECT/files/$SUBDIR/"