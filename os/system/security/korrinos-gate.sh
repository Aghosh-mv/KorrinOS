#!/bin/bash
set -u
PROFILE_FILE="${KORRINOS_PROFILE_FILE:-/var/lib/korrinos/profiles.db}"
cmd="${1:-status}"
user="${KORRINOS_USER:-$(id -un)}"

is_adult=0
if [ -r "$PROFILE_FILE" ]; then
  line="$(awk -F'\t' -v u="$user" '$1==u{print $2; exit}' "$PROFILE_FILE" 2>/dev/null || true)"
  if [ "$line" = "1" ]; then is_adult=1; fi
fi

case "$cmd" in
  is-adult)  exit $([ "$is_adult" -eq 1 ] && echo 0 || echo 1) ;;
  is-minor)  exit $([ "$is_adult" -eq 1 ] && echo 1 || echo 0) ;;
  profiles)  awk -F'\t' '{print}' "$PROFILE_FILE" ;;
  status)    echo "korrinos-gate: $user -> $([ $is_adult -eq 1 ] && echo adult || echo non-adult)" ;;
  *) echo "usage: korrinos-gate {is-adult|is-minor|profiles|status}" >&2; exit 2 ;;
esac
