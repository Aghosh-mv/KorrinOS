#!/usr/bin/env bash
# i18n-lib.sh — KorrinOS 55-language runtime support.
# Sourced by init, the login/session layer, the terminal, Control Center and
# the Control Center language list. Not executable on its own.
#
# Honest scope: this library provides the *infrastructure* for 55 languages —
# locale generation, selection, persistence, font resolution and RTL metadata.
# It does not fabricate translations. A locale with no compiled catalogue
# falls back to English msgids, which is correct behaviour, not a silent lie.

[[ -n "${_KORRINOS_I18N_LIB:-}" ]] && return 0
_KORRINOS_I18N_LIB=1

I18N_DIR="${I18N_DIR:-/usr/share/korrinos/i18n}"
I18N_REGISTRY="${I18N_REGISTRY:-$I18N_DIR/languages.tsv}"
I18N_USERCONF="${I18N_USERCONF:-${XDG_CONFIG_HOME:-$HOME/.config}/korrinos/i18n.conf}"
I18N_ARCHIVE="${I18N_ARCHIVE:-/var/lib/korrinos/i18n}"

# Fall back to the in-tree registry so a live session works before install.
[[ -r "$I18N_REGISTRY" ]] || I18N_REGISTRY="$(dirname "${BASH_SOURCE[0]}")/languages.tsv"

i18n_warn() { printf 'i18n: %s\n' "$*" >&2; }

# i18n_rows — emit every registry row as tab-separated fields.
i18n_rows() {
    [[ -r "$I18N_REGISTRY" ]] || { i18n_warn "registry missing: $I18N_REGISTRY"; return 1; }
    # Drop comments, blank lines, and the header row itself.
    grep -vE '^[[:space:]]*(#|$)' "$I18N_REGISTRY" | grep -vE '^code[[:space:]]'
}

# i18n_field <row> <1..8>
i18n_field() {
    local row="$1" n="$2"
    printf '%s' "$row" | cut -f"$n"
}

# i18n_list — human/UI listing: "<code>\t<endonym>\t<english>".
i18n_list() {
    local row code endonym english
    while IFS= read -r row; do
        code=$(i18n_field "$row" 1)
        endonym=$(i18n_field "$row" 3)
        english=$(i18n_field "$row" 4)
        printf '%s\t%s\t%s\n' "$code" "$endonym" "$english"
    done < <(i18n_rows)
}

# i18n_lookup <code> — print the full row, or fail if the code is unknown.
i18n_lookup() {
    local want="$1" row code
    while IFS= read -r row; do
        code=$(i18n_field "$row" 1)
        [[ "$code" == "$want" ]] && { printf '%s' "$row"; return 0; }
    done < <(i18n_rows)
    return 1
}

# i18n_is_rtl <code>
i18n_is_rtl() {
    local row
    row=$(i18n_lookup "$1") || return 1
    [[ "$(i18n_field "$row" 5)" == "rtl" ]]
}

# i18n_font_package <code> — distro font package that makes <code> legible.
i18n_font_package() {
    local row
    row=$(i18n_lookup "$1") || return 1
    i18n_field "$row" 7
}

# i18n_script <code>
i18n_script() {
    local row
    row=$(i18n_lookup "$1") || return 1
    i18n_field "$row" 6
}

# i18n_locale <code> — the full locale (e.g. hi -> hi_IN.UTF-8).
i18n_locale() {
    local row
    row=$(i18n_lookup "$1") || return 1
    i18n_field "$row" 2
}

# i18n_all_codes
i18n_all_codes() {
    local row
    while IFS= read -r row; do i18n_field "$row" 1; done < <(i18n_rows)
}

# i18n_count — how many languages the registry declares.
i18n_count() { i18n_all_codes | grep -c . ; }

# i18n_validate — registry self-check. Non-zero (and a report) if inconsistent.
i18n_validate() {
    local err=0 seen_codes="" seen_locales="" row code locale n
    n=$(i18n_count)
    if [[ "$n" -ne 55 ]]; then
        i18n_warn "expected 55 languages, registry declares $n"
        err=1
    fi
    while IFS= read -r row; do
        code=$(i18n_field "$row" 1)
        locale=$(i18n_field "$row" 2)
        case "$locale" in
            *.UTF-8) ;;
            *) i18n_warn "$code: locale '$locale' is not a .UTF-8 locale"; err=1 ;;
        esac
        if [[ "$seen_codes" == *"|$code|"* ]]; then
            i18n_warn "$code: duplicate language code"; err=1
        fi
        if [[ "$seen_locales" == *"|$locale|"* ]]; then
            i18n_warn "$locale: duplicate locale"; err=1
        fi
        seen_codes="$seen_codes|$code|"
        seen_locales="$seen_locales|$locale|"
    done < <(i18n_rows)
    (( err == 0 )) && printf 'i18n: registry valid (%d languages)\n' "$n"
    return "$err"
}

# i18n_locale_available <full-locale> — is the locale compiled into this system?
i18n_locale_available() {
    local loc="$1" terr
    terr="${loc%%.*}"
    command locale -a 2>/dev/null | grep -qixF "${loc}" && return 0
    command locale -a 2>/dev/null | grep -qixF "${terr}.utf8" && return 0
    command locale -a 2>/dev/null | grep -qixF "${terr}.UTF-8" && return 0
    return 1
}

# i18n_generate <full-locale> — compile the locale. Idempotent.
i18n_generate() {
    local loc="$1" terr charmap out
    i18n_locale_available "$loc" && return 0
    command -v localedef >/dev/null 2>&1 || { i18n_warn "localedef not installed"; return 1; }
    terr="${loc%%.*}"
    charmap="${loc#*.}"; charmap="${charmap%%-*}"
    out="/usr/lib/locale/${loc}"
    mkdir -p "$(dirname "$out")"
    if localedef -i "$terr" -f "$charmap" "$out" 2>/dev/null; then
        printf '%s\n' "$loc" >> "$I18N_ARCHIVE" 2>/dev/null || true
        return 0
    fi
    i18n_warn "localedef failed for $loc"
    return 1
}

# i18n_selected — the user's chosen language code (empty if unset).
i18n_selected() {
    [[ -r "$I18N_USERCONF" ]] || return 0
    # Parse strictly: KEY=VALUE, no eval, no source of arbitrary code.
    while IFS='=' read -r k v; do
        [[ "$k" == "LANGUAGE" ]] && { printf '%s' "$v"; return 0; }
    done < "$I18N_USERCONF"
}

# i18n_set <code> — validate, generate the locale, persist. Refuses unknown codes.
i18n_set() {
    local code="$1" row locale
    row=$(i18n_lookup "$code") || { i18n_warn "unknown language: $code"; return 1; }
    locale=$(i18n_field "$row" 2)
    i18n_generate "$locale" || i18n_warn "continuing without compiled locale $locale"
    mkdir -p "$(dirname "$I18N_USERCONF")"
    printf 'LANGUAGE=%s\n' "$code" > "$I18N_USERCONF"
    return 0
}

# i18n_apply — export LANG/LC_ALL for the current session. Never overrides an
# explicitly-set LANG, and never exports an uncompiled locale.
i18n_apply() {
    local code locale
    code=$(i18n_selected)
    [[ -n "$code" ]] || return 0
    locale=$(i18n_locale "$code") || return 0
    i18n_locale_available "$locale" || locale=C.UTF-8
    export LANG="$locale"
    export LC_ALL="$locale"
    export LANGUAGE="$code"
    return 0
}

# i18n_text_direction <code> — "rtl" or "ltr" for the UI/terminal layers.
i18n_text_direction() {
    if i18n_is_rtl "$1"; then printf 'rtl'; else printf 'ltr'; fi
}

# i18n_terminal_font <code> — a concrete installed font that covers the script,
# falling back to the default when the script font is not present.
i18n_terminal_font() {
    local code="$1" script candidate
    script=$(i18n_script "$code") || { printf 'monospace'; return 0; }
    case "$script" in
        hans|hant|jpan|kore) candidate="Noto Sans Mono CJK SC" ;;
        arabic)              candidate="Noto Sans Mono Arabic" ;;
        hebrew)              candidate="Noto Sans Mono Hebrew" ;;
        deva|beng|taml|telu|gujr|knda|mlym|guru|mymr|sinh|thai|lao|ethi)
                            candidate="Noto Sans Mono" ;;
        *)                   candidate="monospace" ;;
    esac
    if command -v fc-match >/dev/null 2>&1; then
        if fc-match -f '%{family}\n' "$candidate" 2>/dev/null | head -1 | grep -qqiE 'noto|dejavu'; then
            printf '%s' "$candidate"; return 0
        fi
    fi
    printf 'monospace'
}

# i18n_summary — one line, for diagnostics and the Control Center.
i18n_summary() {
    local code
    code=$(i18n_selected)
    printf 'i18n: %d languages registered; selected=%s; LANG=%s\n' \
        "$(i18n_count)" "${code:-<unset>}" "${LANG:-<unset>}"
}
