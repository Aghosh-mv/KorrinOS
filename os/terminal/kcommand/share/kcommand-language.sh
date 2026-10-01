# kcommand!<language> — language switching without a file per language.
#
# KorrinOS owns this shell, so the `kcommand!Hindi` form does not need 158
# generated wrapper scripts. `kcommand!Hindi` is a single word to bash, and bash
# resolves a word by NAME, so it looks for an executable called
# "kcommand!Hindi" and fails:
#
#     $ kcommand() { echo caught; }; kcommand!Hindi
#     bash: kcommand!Hindi: command not found
#
# It also never reaches a function named `kcommand`, for the same reason.
#
# But bash does offer exactly the hook this needs: when a word cannot be
# resolved, bash calls `command_not_found_handle` with that word as $1. So one
# function intercepts the whole form, and only when the text after the "!" is
# actually a shipped locale. Every other command is untouched and falls through
# to the normal not-found behaviour.

# shellcheck shell=bash

# kcommand_not_found <word> <args...>
#
# Called by bash for any unresolvable command. We only claim words of the form
# kcommand!<language>; everything else is passed on unchanged so the real
# "command not found" error still appears.
kcommand_not_found() {
    local word="${1:-}"
    shift 2>/dev/null || true

    case "$word" in
        kcommand\!*) ;;
        *)
            # Not ours. Reproduce the standard message and status.
            printf '%s: command not found\n' "$word" >&2
            return 127
            ;;
    esac

    local language="${word#kcommand!}"

    # Only intercept a REAL language. Anything else is a genuine typo and must
    # keep failing like any other unknown command.
    if ! /usr/bin/kcommand --is-language "$language" >/dev/null 2>&1; then
        printf '%s: command not found\n' "$word" >&2
        printf "Did you mean 'kcommand!%s'? Run 'kcommand --languages' for the list.\n" \
            "$language" >&2
        return 127
    fi

    exec /usr/bin/kcommand "!$language" "$@"
}

command_not_found_handle() {
    kcommand_not_found "$@"
}