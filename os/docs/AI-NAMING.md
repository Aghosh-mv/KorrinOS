# KorrinOS AI Naming Registry
#
# SINGLE SOURCE OF TRUTH for every AI-related name in the codebase.
# Read this before adding any new AI surface, script, module or UI string.
#
# -----------------------------------------------------------------------------
# CANONICAL PRODUCT NAME:  VOKK v4
# -----------------------------------------------------------------------------
# `vokk/` directory            - the AI engine ("VOKK v4") + its modules
# `vokk/vokk.sh`              - the launcher (execs vokk.py)
# `vokk/vokk.py`              - the CLI entry point
# `vokk/vokk_search.py`       - search index module  (underscore: it is imported)
# `vokk/vokk-v4-engine.py`    - the VokkV4 / VokkV4Server engine
# `vokk/vokk-ui.py`           - optional tkinter front end
#
# Module naming rule:
#   - a module that is `import`ed MUST use underscores (vokk_search.py).
#   - a module that is only ever executed as a script MAY use hyphens.
#
# -----------------------------------------------------------------------------
# WHAT "PARC" IS (a different layer - do not rename to VOKK)
# -----------------------------------------------------------------------------
# `parc-ai/` is the desktop shell / Control Center that *hosts* VOKK v4.
# It is a UI and integration layer, not the assistant itself. It legitimately
# keeps the "Parc" name. Its user-facing title is "Parc Control Center".
#
# -----------------------------------------------------------------------------
# WHAT "TINKER" IS (internal paths only - never user-facing)
# -----------------------------------------------------------------------------
# `~/.tinker/` remains the on-disk state/config directory. It is an internal
# implementation detail; renaming it would invalidate existing user state.
# Do NOT put the word "Tinker" in anything the user reads.
#
# -----------------------------------------------------------------------------
# USER-FACING VOICE
# -----------------------------------------------------------------------------
#   Correct:   "VOKK v4"   |  "VOKK"  |  "Parc Control Center"
#   Wrong:     "TinkerAI"  |  "Tinker AI"  |  "tinker-ai"  |  "Zegrate"
#
# Capitalisation: the product is written "VOKK v4" - capital VOKK, lowercase v4.
# The command is `vokk`. Never "Vokk V4" or "VOKK V4" in user-facing text.
# -----------------------------------------------------------------------------
# ALIASES
# -----------------------------------------------------------------------------
# "VOKK", "VOKK v4" and "VOKK4" are the same thing. Accept all three as input.
# "Parc", "Parc AI" and "Parc Control Center" refer to the shell, not the brain.
# -----------------------------------------------------------------------------
# RETIRED
# -----------------------------------------------------------------------------
#   tinker-ai.py         -> vokk/vokk-v4-engine.py
#   tinker_search_ai.py  -> vokk/vokk_search.py
#   tinker_ui.py         -> vokk/vokk-ui.py
#   "TinkerAI" strings   -> "VOKK v4"
#   "Zegrate"            -> never adopted; do not reintroduce
