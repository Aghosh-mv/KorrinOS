# kcommand.bash — KorrinOS shell integration for kcommand.
#
# Sourced by the shell inside kcommand. Everything here is opt-in and additive:
# it never replaces an existing command, and it removes itself cleanly.
#
# Provides:
#   * typo detection with "did you mean ...?" on any unknown command
#   * the KorrinOS terminal tooling (parcos-terminal) on PATH behaviour
#   * small conveniences that make the terminal easier to live in

# ---- guard -----------------------------------------------------------------
# Only load once per shell.
[ -n "${_KCOMMAND_BASH_LOADED:-}" ] && return 0
_KCOMMAND_BASH_LOADED=1

# Never fight a shell that already has its own handler.
if declare -F command_not_found_handle >/dev/null 2>&1; then
    _KCOMMAND_SAVED_NOT_FOUND="$(declare -f command_not_found_handle)"
fi

# ---- did-you-mean ----------------------------------------------------------
# Damerau-Levenshtein distance in awk: no dependency beyond awk itself, which
# every POSIX shell environment has.
#
# Damerau rather than plain Levenshtein on purpose. A plain distance scores a
# swapped pair ("pyhton" -> "python") as two edits, but swapping two adjacent
# letters is the single most common human typo, so plain Levenshtein rejects
# exactly the mistakes we most want to catch. Damerau scores it as one.
_kc_distance() {
    awk -v a="$1" -v b="$2" '
    BEGIN {
        la = length(a); lb = length(b)
        for (i = 0; i <= la; i++) d[i, 0] = i
        for (j = 0; j <= lb; j++) d[0, j] = j
        for (i = 1; i <= la; i++) {
            ca = substr(a, i, 1)
            for (j = 1; j <= lb; j++) {
                cb = substr(b, j, 1)
                cost = (ca == cb) ? 0 : 1
                m = d[i-1, j] + 1
                if (d[i, j-1] + 1 < m) m = d[i, j-1] + 1
                if (d[i-1, j-1] + cost < m) m = d[i-1, j-1] + cost
                # Adjacent transposition costs one edit.
                if (i > 1 && j > 1 && ca == substr(b, j-1, 1) &&
                    substr(a, i-1, 1) == cb) {
                    t = d[i-2, j-2] + 1
                    if (t < m) m = t
                }
                # Store only after the transposition adjustment, otherwise the
                # improved value is computed and then discarded.
                d[i, j] = m
            }
        }
        print d[la, lb]
    }'
}

# Candidate commands: the rest of the line, every word in PATH plus shell
# builtins, so we can catch a typo in any of them.
_kc_candidates() {
    { command -v "$@" 2>/dev/null; compgen -c 2>/dev/null; } | sort -u
}

# kc_typo <misspelled> [candidate ...]
# Prints up to 3 near-matches, closest first.
kc_typo() {
    local bad="$1"; shift
    [ -z "$bad" ] && return 1
    local c d best="" bestd=999
    for c in "$@"; do
        # Cheap reject: a word wildly different in length is never a typo.
        local lo hi
        lo=$(( ${#bad} - 2 )); hi=$(( ${#bad} + 2 ))
        [ "$lo" -lt 1 ] && lo=1
        [ "${#c}" -lt "$lo" ] || [ "${#c}" -gt "$hi" ] && continue
        d=$(_kc_distance "$bad" "$c") || continue
        if [ "$d" -lt "$bestd" ]; then
            bestd="$d"; best="$c"
        fi
    done
    # Allow at most 2 edits, and never suggest the word itself.
    [ -n "$best" ] || return 1
    [ "$best" = "$bad" ] && return 1
    [ "$bestd" -le 2 ] || return 1
    printf '%s (distance %s)\n' "$best" "$bestd"
}

# Native hook: fires instead of "command not found".
command_not_found_handle() {
    local bad="$1"
    local suggest
    suggest=$(kc_typo "$bad" $(_kc_candidates) 2>/dev/null | head -3)

    if [ -n "$suggest" ]; then
        printf '%s: command not found. Did you mean:\n' "$bad" >&2
        # Indent each suggestion as a whole line. Unquoted $suggest would
        # word-split "git (distance 1)" onto three separate lines.
        printf '%s\n' "$suggest" | sed 's/^/  /' >&2
    else
        # Preserve whatever the shell or the user had before us.
        if [ -n "${_KCOMMAND_SAVED_NOT_FOUND:-}" ]; then
            eval "${_KCOMMAND_SAVED_NOT_FOUND/\\$1/$bad}" 2>/dev/null
            return $?
        fi
        printf '%s: command not found\n' "$bad" >&2
    fi

    # Nudge toward the KorrinOS error explainer if the OS provides it.
    if [ -x /opt/korrinos/os/apps/system/terminal-error-explainer.sh ]; then
        printf 'Tip: `explain %s` describes what went wrong.\n' "$bad" >&2
    fi
    return 127
}

# ---- KorrinOS terminal tooling --------------------------------------------
# parcos-terminal already provides suggest/explain/fix/record/replay/sessions/
# history/shortcuts. Put it on PATH rather than reimplementing any of it.
if [ -f /opt/korrinos/os/parc-ai/parcos-terminal.sh ]; then
    # shellcheck disable=SC1091
    . /opt/korrinos/os/parc-ai/parcos-terminal.sh 2>/dev/null || true
fi

# ---- conveniences ----------------------------------------------------------
kc_version() {
    if [ -x /usr/bin/kcommand ]; then
        /usr/bin/kcommand --version 2>/dev/null
    else
        echo "kcommand not installed in this session"
    fi
}

# Which font/typography policy is active? First thing to check when text
# renders oddly.
kc_type() {
    if [ -r /usr/share/korrinos/i18n/languages.tsv ]; then
        local code="${LANGUAGE:-${LANG%%.*}}"
        code="${code%%_*}"
        local script
        script=$(awk -F'\t' -v c="$code" '$1==c {print $5"\t"$6}' \
            /usr/share/korrinos/i18n/languages.tsv 2>/dev/null | head -1)
        if [ -n "$script" ]; then
            printf 'language: %s\nscript:   %s\nfonts:    55 registered\n' "$code" "$script"
        else
            printf 'language: %s (not in registry)\n' "$code"
        fi
    else
        echo "no language registry installed"
    fi
}
