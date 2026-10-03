#!/bin/sh
# xscreensaver entry point for the KorrinOS wordpen saver.
#
# xscreensaver runs this with:
#   $1 = the X display to use
#   $2 = the window id to draw into (unused: we open our own fullscreen window)
#   $3 = the screensaver mode: "preview", "run", or "background"
#
# "preview" is the small thumbnail in the settings dialog, "run" is the real
# thing. There is no "background" mode here on purpose: the saver does not stay
# resident, which is the whole point.
set -eu

DISPLAY_VALUE="${1:-${DISPLAY:-}}"
MODE="${3:-run}"

export DISPLAY="$DISPLAY_VALUE"
export KORRINOS_WORDPEN_DATA="${KORRINOS_WORDPEN_DATA:-/usr/share/korrinos/wordpen}"

if [ -x /usr/bin/korrinos-wordpen ]; then
    exec /usr/bin/korrinos-wordpen
fi

# During an ISO build the binary may live in the staging root rather than on the
# live filesystem, so look there before giving up.
for candidate in \
    /usr/lib/korrinos/wordpen/korrinos-wordpen \
    /usr/local/lib/korrinos/wordpen/korrinos-wordpen
do
    if [ -x "$candidate" ]; then
        exec "$candidate"
    fi
done

printf 'wordpen: korrinos-wordpen is not installed\n' >&2
exit 1
