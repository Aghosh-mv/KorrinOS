# KorrinOS — Professionalism Audit

Every count below is MEASURED by `docs/_professionalism_audit.py` over the tree.
No item is invented: each is a real occurrence at a real file:line.
Re-run the script after fixing to watch the numbers fall.

## Total findings: 6857

| # | Category | Count |
|---|----------|-------|
| C03 |  line >120 cols | 1296 |
| C31 |  '|| true' swallows errors | 1006 |
| C12 |  sudo with unquoted var | 805 |
| C13 |  echo -e (non-portable) | 530 |
| C14 |  useless $(echo) | 524 |
| P01 |  line >100 cols | 319 |
| P07 |  debug print in library | 290 |
| C109 | 9 mixed indentation | 261 |
| W01 |  line >120 cols | 228 |
| C08 |  legacy ~/.tinker state path | 197 |
| C07 |  hardcoded /tmp path | 169 |
| C11 |  error text on stdout | 135 |
| C02 |  no strict mode | 131 |
| N03 |  filename embeds a brand | 124 |
| C33 |  no usage/help text | 111 |
| C06 |  TODO/FIXME/HACK left in code | 86 |
| C102 | 2 mixed tabs/spaces | 64 |
| C111 | 1 no MODULE_LICENSE | 49 |
| C04 |  trailing whitespace | 40 |
| C16 |  'which' instead of 'command -v' | 39 |
| C107 | 7 C++ style // comment | 37 |
| C09 |  rm -rf on a variable | 36 |
| C18 |  for-loop over $(...) (word split) | 33 |
| P06 |  except: pass (silent swallow) | 33 |
| P13 |  shell=True | 29 |
| C15 |  regex in [[ ]] unquoted RHS risk | 28 |
| C24 |  exporting a local-ish var | 27 |
| W09 |  'any' type escape hatch | 22 |
| P05 |  bare except | 21 |
| C10 |  eval present | 20 |
| P17 |  os.system | 20 |
| C101 | 1 line >100 cols | 20 |
| P02 |  trailing whitespace | 16 |
| N02 |  non-canonical brand in use | 15 |
| C22 |  bare sleep | 13 |
| C110 | 0 missing SPDX license header | 12 |
| P11 |  eval() | 8 |
| P18 |  assert used outside tests | 8 |
| C25 |  pipe to head (SIGPIPE risk) | 7 |
| P12 |  exec() | 7 |
| P04 |  TODO/FIXME left | 6 |
| C105 | 5 memcpy (size must be checked) | 6 |
| W02 |  console.log left in | 6 |
| C21 |  curl piped to shell | 5 |
| C26 |  complex inline command substitution | 4 |
| W07 |  innerHTML assignment | 3 |
| C23 |  killall | 2 |
| C103 | 3 unsafe string function | 2 |
| W13 |  TODO/FIXME left | 2 |
| W12 |  'as any' cast | 2 |
| P15 |  str.format instead of f-string | 1 |
| C106 | 6 sprintf (use snprintf) | 1 |
| N01 |  brand-name split brain | 1 |

## Findings by category

### C03 line >120 cols — 1296

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 399 | 168 cols |
| `os/build-distro.sh` | 541 | 131 cols |
| `os/build-distro.sh` | 610 | 145 cols |
| `os/build-distro.sh` | 630 | 145 cols |
| `os/build-distro.sh` | 671 | 150 cols |
| `os/build-distro.sh` | 797 | 145 cols |
| `os/iso-builder.sh` | 53 | 128 cols |
| `os/parcos-agent.sh` | 52 | 146 cols |
| `os/parcos-agent.sh` | 55 | 190 cols |
| `os/parcos-agent.sh` | 56 | 205 cols |
| `os/parcos-agent.sh` | 57 | 203 cols |
| `os/parcos-agent.sh` | 58 | 224 cols |
| `os/apps/app-store.sh` | 120 | 129 cols |
| `os/apps/app-store.sh` | 131 | 191 cols |
| `os/apps/app-store.sh` | 218 | 126 cols |
| `os/apps/app-store.sh` | 219 | 130 cols |
| `os/apps/app-store.sh` | 220 | 130 cols |
| `os/apps/gaming-support.sh` | 221 | 141 cols |
| `os/apps/gaming-support.sh` | 227 | 130 cols |
| `os/apps/apps/aether-workspace.sh` | 7 | 168 cols |
| `os/apps/apps/nibra-betterlife.sh` | 7 | 168 cols |
| `os/apps/apps/parc-ai.sh` | 12 | 156 cols |
| `os/apps/gaming/audio-mixer.sh` | 171 | 158 cols |
| `os/apps/gaming/audio-mixer.sh` | 195 | 133 cols |
| `os/apps/gaming/hardware-benchmark.sh` | 125 | 129 cols |
| `os/apps/gaming/hardware-benchmark.sh` | 127 | 137 cols |
| `os/apps/gaming/performance-graph.sh` | 49 | 121 cols |
| `os/apps/gaming/screenshot-tool.sh` | 76 | 125 cols |
| `os/apps/customization/cursor-themes.sh` | 50 | 124 cols |
| `os/apps/customization/wallpaper-manager.sh` | 16 | 146 cols |
| `os/apps/customization/wallpaper-manager.sh` | 22 | 138 cols |
| `os/apps/customization/wallpaper-manager.sh` | 28 | 131 cols |
| `os/apps/security/firewall.sh` | 50 | 126 cols |
| `os/apps/security/firewall.sh` | 60 | 129 cols |
| `os/apps/security/gatekeeper.sh` | 47 | 133 cols |
| `os/apps/security/password-manager.sh` | 85 | 131 cols |
| `os/apps/security/password-manager.sh` | 107 | 127 cols |
| `os/apps/security/privacy.sh` | 48 | 146 cols |
| `os/apps/security/privacy.sh` | 76 | 158 cols |
| `os/apps/security/privacy.sh` | 97 | 122 cols |
| `os/apps/security/security-suite.sh` | 76 | 129 cols |
| `os/apps/security/security-suite.sh` | 98 | 132 cols |
| `os/apps/security/security-suite.sh` | 108 | 125 cols |
| `os/apps/security/security-suite.sh` | 135 | 142 cols |
| `os/apps/security/security-suite.sh` | 141 | 132 cols |
| `os/apps/hardware/usb-manager.sh` | 14 | 121 cols |
| `os/apps/hardware/usb-manager.sh` | 39 | 127 cols |
| `os/apps/hardware/webcam-manager.sh` | 19 | 127 cols |
| `os/apps/hardware/webcam-manager.sh` | 74 | 145 cols |
| `os/apps/system/auto-updates.sh` | 152 | 129 cols |
| `os/apps/system/auto-updates.sh` | 223 | 159 cols |
| `os/apps/system/auto-updates.sh` | 227 | 145 cols |
| `os/apps/system/context-aware.sh` | 118 | 165 cols |
| `os/apps/system/context-aware.sh` | 132 | 121 cols |
| `os/apps/system/digital-twin.sh` | 145 | 135 cols |
| `os/apps/system/duplicate-finder.sh` | 46 | 130 cols |
| `os/apps/system/predictive-intelligence.sh` | 132 | 169 cols |
| `os/apps/system/rollback-recovery.sh` | 151 | 130 cols |
| `os/apps/system/self-healing.sh` | 60 | 128 cols |
| `os/apps/system/self-healing.sh` | 197 | 128 cols |
| `os/apps/system/system-monitor.sh` | 93 | 138 cols |
| `os/apps/system/system-monitor.sh` | 156 | 143 cols |
| `os/apps/network/firewall-gui.sh` | 34 | 133 cols |
| `os/apps/network/firewall-gui.sh` | 232 | 176 cols |
| `os/apps/network/network-monitor.sh` | 91 | 125 cols |
| `os/apps/network/speed-test.sh` | 39 | 143 cols |
| `os/apps/network/vpn-manager.sh` | 85 | 139 cols |
| `os/apps/network/vpn-manager.sh` | 107 | 154 cols |
| `os/apps/network/wifi-analyzer.sh` | 112 | 131 cols |
| `os/hyperdrive/hyperdrive-cli.sh` | 187 | 124 cols |
| `os/hyperdrive/hyperdrive-daemon.sh` | 101 | 126 cols |
| `os/hyperdrive/hyperdrive.sh` | 65 | 128 cols |
| `os/parc-ai/korrinos-backup.sh` | 35 | 166 cols |
| `os/parc-ai/korrinos-backup.sh` | 95 | 122 cols |
| `os/parc-ai/korrinos-backup.sh` | 109 | 167 cols |
| `os/parc-ai/korrinos-backup.sh` | 120 | 122 cols |
| `os/parc-ai/korrinos-backup.sh` | 141 | 166 cols |
| `os/parc-ai/korrinos-backup.sh` | 163 | 166 cols |
| `os/parc-ai/korrinos-backup.sh` | 210 | 166 cols |
| `os/parc-ai/korrinos-backup.sh` | 271 | 166 cols |
| `os/parc-ai/korrinos-backup.sh` | 284 | 166 cols |
| `os/parc-ai/korrinos-cleanup.sh` | 194 | 147 cols |
| `os/parc-ai/korrinos-cleanup.sh` | 195 | 147 cols |
| `os/parc-ai/korrinos-cleanup.sh` | 196 | 147 cols |
| `os/parc-ai/korrinos-clipctx.sh` | 55 | 176 cols |
| `os/parc-ai/korrinos-clipctx.sh` | 68 | 122 cols |
| `os/parc-ai/korrinos-dashboard.sh` | 71 | 143 cols |
| `os/parc-ai/korrinos-dashboard.sh` | 82 | 137 cols |
| `os/parc-ai/korrinos-dashboard.sh` | 116 | 183 cols |
| `os/parc-ai/korrinos-dashboard.sh` | 147 | 125 cols |
| `os/parc-ai/korrinos-dock.sh` | 157 | 163 cols |
| `os/parc-ai/korrinos-focus.sh` | 47 | 132 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 55 | 127 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 57 | 126 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 59 | 124 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 61 | 133 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 63 | 137 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 65 | 125 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 67 | 133 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 69 | 124 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 200 | 149 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 201 | 142 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 202 | 146 cols |
| `os/parc-ai/korrinos-liquid-glass.sh` | 203 | 145 cols |
| `os/parc-ai/korrinos-monitor.sh` | 35 | 124 cols |
| `os/parc-ai/korrinos-monitor.sh` | 142 | 170 cols |
| `os/parc-ai/korrinos-monitor.sh` | 148 | 165 cols |
| `os/parc-ai/korrinos-monitor.sh` | 240 | 126 cols |
| `os/parc-ai/korrinos-monitor.sh` | 243 | 140 cols |
| `os/parc-ai/korrinos-network.sh` | 137 | 122 cols |
| `os/parc-ai/korrinos-network.sh` | 168 | 134 cols |
| `os/parc-ai/korrinos-network.sh` | 216 | 132 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 180 | 130 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 181 | 130 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 184 | 139 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 187 | 147 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 188 | 143 cols |
| `os/parc-ai/korrinos-nlctl.sh` | 189 | 153 cols |
| `os/parc-ai/korrinos-notepad.sh` | 48 | 121 cols |
| `os/parc-ai/korrinos-power.sh` | 115 | 121 cols |
| `os/parc-ai/korrinos-power.sh` | 198 | 125 cols |
| `os/parc-ai/korrinos-shell-ai.sh` | 30 | 138 cols |
| `os/parc-ai/korrinos-shortcuts.sh` | 120 | 130 cols |
| `os/parc-ai/korrinos-smoothui.sh` | 49 | 123 cols |
| `os/parc-ai/korrinos-sounds.sh` | 57 | 122 cols |
| `os/parc-ai/korrinos-sounds.sh` | 81 | 124 cols |
| `os/parc-ai/korrinos-sounds.sh` | 124 | 144 cols |
| `os/parc-ai/korrinos-sounds.sh` | 126 | 143 cols |
| `os/parc-ai/korrinos-sounds.sh` | 128 | 142 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 37 | 125 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 79 | 160 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 82 | 150 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 85 | 153 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 88 | 175 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 91 | 161 cols |
| `os/parc-ai/korrinos-tinkeria.sh` | 94 | 157 cols |
| `os/parc-ai/korrinos-toggles.sh` | 29 | 137 cols |
| `os/parc-ai/korrinos-toggles.sh` | 52 | 138 cols |
| `os/parc-ai/korrinos-toggles.sh` | 103 | 136 cols |
| `os/parc-ai/korrinos-voice.sh` | 98 | 132 cols |
| `os/parc-ai/korrinos-voice.sh` | 99 | 136 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 47 | 154 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 48 | 155 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 49 | 159 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 50 | 159 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 93 | 122 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 134 | 157 cols |
| `os/parc-ai/korrinos-widgets-panel.sh` | 135 | 154 cols |
| `os/parc-ai/parc-ai.sh` | 28 | 126 cols |
| `os/parc-ai/parc-ai.sh` | 279 | 231 cols |
| `os/parc-ai/parc-ai.sh` | 280 | 180 cols |
| `os/parc-ai/parc-ai.sh` | 347 | 225 cols |
| `os/parc-ai/parc-ai.sh` | 349 | 149 cols |
| `os/parc-ai/parc-ai.sh` | 410 | 152 cols |
| `os/parc-ai/parc-ai.sh` | 411 | 154 cols |
| `os/parc-ai/parc-ai.sh` | 412 | 136 cols |
| `os/parc-ai/parc-ai.sh` | 442 | 155 cols |
| `os/parc-ai/parc-ai.sh` | 444 | 261 cols |
| `os/parc-ai/parc-ai.sh` | 496 | 182 cols |
| `os/parc-ai/parc-ai.sh` | 507 | 134 cols |
| `os/parc-ai/parc-ai.sh` | 515 | 153 cols |
| `os/parc-ai/parc-ai.sh` | 534 | 223 cols |
| `os/parc-ai/parc-ai.sh` | 553 | 192 cols |
| `os/parc-ai/parc-ai.sh` | 625 | 257 cols |
| `os/parc-ai/parcai-model-pipeline.sh` | 365 | 148 cols |
| `os/parc-ai/parcai-model-pipeline.sh` | 511 | 148 cols |
| `os/parc-ai/parcos-settings.sh` | 186 | 129 cols |
| `os/parc-ai/parcos-settings.sh` | 187 | 127 cols |
| `os/parc-ai/parcos-settings.sh` | 188 | 127 cols |
| `os/parc-ai/parcos-tools.sh` | 55 | 122 cols |
| `os/parc-ai/parcos-uninstall-blocker.sh` | 67 | 194 cols |
| `os/parc-ai/modules/agent-browser.sh` | 26 | 156 cols |
| `os/parc-ai/modules/agent-browser.sh` | 46 | 176 cols |
| `os/parc-ai/modules/agent-browser.sh` | 87 | 150 cols |
| `os/parc-ai/modules/agent-browser.sh` | 187 | 183 cols |
| `os/parc-ai/modules/agent-vision.sh` | 142 | 270 cols |
| `os/parc-ai/modules/ai-brain-smart.sh` | 11 | 126 cols |
| `os/parc-ai/modules/ai-brain-smart.sh` | 94 | 121 cols |
| `os/parc-ai/modules/ai-brain-smart.sh` | 263 | 121 cols |
| `os/parc-ai/modules/ai-brain-smart.sh` | 423 | 136 cols |
| `os/parc-ai/modules/ai-engine.sh` | 13 | 898 cols |
| `os/parc-ai/modules/ai-engine.sh` | 118 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 12 | 175 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 13 | 189 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 14 | 209 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 15 | 193 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 16 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 17 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 18 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 19 | 165 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 20 | 181 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 21 | 153 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 24 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 26 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 27 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 28 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 29 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 30 | 181 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 31 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 32 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 33 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 36 | 174 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 37 | 177 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 38 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 39 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 40 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 41 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 42 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 43 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 44 | 172 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 45 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 48 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 49 | 169 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 50 | 156 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 51 | 174 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 52 | 184 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 53 | 181 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 54 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 55 | 179 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 56 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 57 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 62 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 63 | 175 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 64 | 186 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 65 | 190 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 66 | 184 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 67 | 186 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 68 | 186 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 69 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 70 | 174 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 71 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 74 | 201 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 75 | 178 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 76 | 171 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 77 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 78 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 79 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 80 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 81 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 82 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 83 | 188 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 86 | 165 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 87 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 88 | 162 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 89 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 90 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 91 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 92 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 93 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 94 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 95 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 100 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 101 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 102 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 103 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 104 | 156 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 105 | 165 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 106 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 107 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 108 | 208 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 109 | 178 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 112 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 113 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 114 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 115 | 198 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 116 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 117 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 118 | 179 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 119 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 120 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 121 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 124 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 125 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 127 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 128 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 129 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 130 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 131 | 203 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 132 | 156 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 133 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 138 | 178 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 139 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 140 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 141 | 177 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 142 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 143 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 144 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 145 | 169 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 146 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 147 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 150 | 191 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 151 | 174 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 152 | 181 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 153 | 176 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 154 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 155 | 200 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 156 | 169 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 157 | 184 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 158 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 159 | 182 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 163 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 164 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 165 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 166 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 167 | 178 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 168 | 171 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 169 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 170 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 171 | 171 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 172 | 177 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 176 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 177 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 178 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 179 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 180 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 181 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 182 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 183 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 184 | 160 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 185 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 190 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 191 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 192 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 193 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 194 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 195 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 196 | 179 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 197 | 200 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 198 | 182 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 199 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 202 | 153 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 203 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 204 | 175 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 205 | 169 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 206 | 176 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 207 | 156 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 208 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 209 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 210 | 176 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 211 | 173 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 215 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 216 | 167 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 217 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 218 | 197 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 219 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 220 | 180 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 221 | 205 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 222 | 169 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 223 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 224 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 227 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 228 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 229 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 230 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 231 | 179 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 232 | 160 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 233 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 234 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 235 | 156 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 236 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 239 | 176 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 240 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 241 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 242 | 162 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 243 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 244 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 245 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 246 | 178 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 247 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 248 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 251 | 165 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 253 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 254 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 255 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 256 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 257 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 258 | 166 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 259 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 260 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 12 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 13 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 14 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 15 | 158 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 16 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 17 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 18 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 19 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 20 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 22 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 23 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 24 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 25 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 26 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 29 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 30 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 31 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 32 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 33 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 34 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 35 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 36 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 37 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 38 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 39 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 40 | 165 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 41 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 44 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 49 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 50 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 51 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 54 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 55 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 56 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 59 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 60 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 62 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 63 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 66 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 67 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 69 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 71 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 72 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 73 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 74 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 75 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 76 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 77 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 78 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 79 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 80 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 81 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 82 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 83 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 84 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 85 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 87 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 88 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 89 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 90 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 93 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 94 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 96 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 97 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 98 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 99 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 100 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 101 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 102 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 103 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 104 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 106 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 107 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 108 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 109 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 110 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 111 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 112 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 117 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 118 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 119 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 120 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 121 | 157 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 122 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 123 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 124 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 125 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 126 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 127 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 128 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 129 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 130 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 131 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 132 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 133 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 134 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 135 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 136 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 137 | 159 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 138 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 139 | 160 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 140 | 163 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 141 | 155 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 142 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 143 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 144 | 155 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 145 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 146 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 149 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 152 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 153 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 156 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 157 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 158 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 159 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 160 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 161 | 155 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 162 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 163 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 164 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 165 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 166 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 167 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 168 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 171 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 172 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 173 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 175 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 176 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 177 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 178 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 179 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 183 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 184 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 186 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 187 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 188 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 189 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 190 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 197 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 198 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 203 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 204 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 205 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 206 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 207 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 210 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 211 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 212 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 213 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 214 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 221 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 223 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 227 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 234 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 235 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 239 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 244 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 245 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 249 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 251 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 255 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 256 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 257 | 155 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 258 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 261 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 265 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 266 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 267 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 268 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 269 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 270 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 271 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 272 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 273 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 274 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 275 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 277 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 278 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 279 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 280 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 285 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 286 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 287 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 288 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 289 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 290 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 291 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 292 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 293 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 294 | 161 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 295 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 296 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 297 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 298 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 299 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 300 | 162 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 301 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 302 | 172 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 303 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 304 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 307 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 308 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 309 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 310 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 311 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 312 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 313 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 314 | 148 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 315 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 316 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 317 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 318 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 319 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 320 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 321 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 322 | 164 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 323 | 153 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 324 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 325 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 326 | 151 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 330 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 334 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 339 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 340 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 341 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 342 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 343 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 344 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 345 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 346 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 348 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 349 | 153 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 360 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 366 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 370 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 375 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 377 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 378 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 379 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 380 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 381 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 382 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 383 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 384 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 385 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 387 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 389 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 391 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 405 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 409 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 411 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 413 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 414 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 415 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 416 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 417 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 420 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 421 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 423 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 425 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 426 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 427 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 428 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 429 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 430 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 431 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 432 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 433 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 434 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 436 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 438 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 439 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 443 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 446 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 447 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 448 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 451 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 452 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 453 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 454 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 455 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 457 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 460 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 461 | 150 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 465 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 471 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 475 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 477 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 478 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 479 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 480 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 481 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 482 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 483 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 492 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 493 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 495 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 501 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 502 | 149 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 503 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 504 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 508 | 154 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 509 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 510 | 168 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 511 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 512 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 514 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 515 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 516 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 517 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 519 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 520 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 521 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 522 | 124 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 523 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 524 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 525 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 526 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 527 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 531 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 532 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 533 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 534 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 535 | 147 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 536 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 537 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 538 | 145 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 539 | 152 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 540 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 541 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 542 | 127 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 543 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 544 | 144 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 545 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 546 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 547 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 549 | 162 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 552 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 554 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 556 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 557 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 558 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 559 | 128 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 560 | 143 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 563 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 564 | 146 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 565 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 566 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 567 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 568 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 569 | 140 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 571 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 574 | 142 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 576 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 577 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 578 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 579 | 135 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 580 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 581 | 121 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 582 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 583 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 584 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 585 | 139 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 586 | 136 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 587 | 130 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 588 | 129 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 589 | 134 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 590 | 133 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 591 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 592 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 593 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 596 | 137 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 598 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 599 | 132 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 600 | 122 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 604 | 126 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 607 | 123 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 609 | 131 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 611 | 125 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 612 | 141 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 613 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 614 | 138 cols |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 615 | 128 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 38 | 122 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 48 | 141 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 58 | 141 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 60 | 144 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 62 | 139 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 68 | 121 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 86 | 123 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 96 | 130 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 98 | 171 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 107 | 134 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 138 | 190 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 293 | 142 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 294 | 145 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 295 | 141 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 296 | 140 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 297 | 145 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 298 | 137 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 304 | 122 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 340 | 138 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 408 | 124 cols |
| `os/parc-ai/modules/ai-master-brain.sh` | 440 | 142 cols |
| `os/parc-ai/modules/ai-narrative.sh` | 78 | 143 cols |
| `os/parc-ai/modules/ai-narrative.sh` | 86 | 123 cols |
| `os/parc-ai/modules/ai-narrative.sh` | 87 | 145 cols |
| `os/parc-ai/modules/ai-narrative.sh` | 374 | 126 cols |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 132 | 121 cols |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 153 | 180 cols |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 475 | 121 cols |
| `os/parc-ai/modules/ai-personality.sh` | 51 | 136 cols |
| `os/parc-ai/modules/ai-personality.sh` | 57 | 145 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 21 | 193 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 28 | 178 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 29 | 136 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 54 | 294 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 58 | 132 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 63 | 294 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 80 | 127 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 81 | 206 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 96 | 239 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 98 | 142 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 113 | 128 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 116 | 235 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 117 | 235 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 118 | 235 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 119 | 235 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 195 | 133 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 198 | 143 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 206 | 125 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 208 | 125 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 218 | 165 cols |
| `os/parc-ai/modules/cards-interactive.sh` | 248 | 128 cols |
| `os/parc-ai/modules/cards-live.sh` | 12 | 146 cols |
| `os/parc-ai/modules/cards-live.sh` | 15 | 248 cols |
| `os/parc-ai/modules/cards-live.sh` | 16 | 288 cols |
| `os/parc-ai/modules/cards-live.sh` | 19 | 164 cols |
| `os/parc-ai/modules/cards-live.sh` | 32 | 146 cols |
| `os/parc-ai/modules/cards-live.sh` | 37 | 227 cols |
| `os/parc-ai/modules/cards-live.sh` | 45 | 189 cols |
| `os/parc-ai/modules/cards-live.sh` | 47 | 162 cols |
| `os/parc-ai/modules/cards-live.sh` | 93 | 155 cols |
| `os/parc-ai/modules/cards-live.sh` | 94 | 137 cols |
| `os/parc-ai/modules/cards-live.sh` | 195 | 137 cols |
| `os/parc-ai/modules/cards-live.sh` | 196 | 186 cols |
| `os/parc-ai/modules/cards-live.sh` | 211 | 157 cols |
| `os/parc-ai/modules/cards-live.sh` | 212 | 139 cols |
| `os/parc-ai/modules/cards-live.sh` | 216 | 169 cols |
| `os/parc-ai/modules/cards-live.sh` | 220 | 181 cols |
| `os/parc-ai/modules/cards-live.sh` | 242 | 150 cols |
| `os/parc-ai/modules/cards-structural.sh` | 29 | 230 cols |
| `os/parc-ai/modules/cards-structural.sh` | 33 | 164 cols |
| `os/parc-ai/modules/cards-structural.sh` | 40 | 213 cols |
| `os/parc-ai/modules/cards-structural.sh` | 81 | 191 cols |
| `os/parc-ai/modules/cards-structural.sh` | 87 | 127 cols |
| `os/parc-ai/modules/cards-structural.sh` | 102 | 146 cols |
| `os/parc-ai/modules/cards-structural.sh` | 142 | 146 cols |
| `os/parc-ai/modules/cards-structural.sh` | 146 | 138 cols |
| `os/parc-ai/modules/cards-structural.sh` | 148 | 158 cols |
| `os/parc-ai/modules/cards-structural.sh` | 159 | 219 cols |
| `os/parc-ai/modules/cards-structural.sh` | 160 | 217 cols |
| `os/parc-ai/modules/cards-structural.sh` | 162 | 249 cols |
| `os/parc-ai/modules/cards-structural.sh` | 163 | 272 cols |
| `os/parc-ai/modules/cards-structural.sh` | 165 | 126 cols |
| `os/parc-ai/modules/cards-structural.sh` | 166 | 157 cols |
| `os/parc-ai/modules/cards-structural.sh` | 178 | 166 cols |
| `os/parc-ai/modules/cards-structural.sh` | 179 | 165 cols |
| `os/parc-ai/modules/cards-structural.sh` | 180 | 145 cols |
| `os/parc-ai/modules/cards-structural.sh` | 184 | 163 cols |
| `os/parc-ai/modules/cards-structural.sh` | 185 | 149 cols |
| `os/parc-ai/modules/cards-structural.sh` | 189 | 206 cols |
| `os/parc-ai/modules/cards-structural.sh` | 190 | 189 cols |
| `os/parc-ai/modules/cards-structural.sh` | 214 | 158 cols |
| `os/parc-ai/modules/cards-visual.sh` | 29 | 146 cols |
| `os/parc-ai/modules/cards-visual.sh` | 31 | 231 cols |
| `os/parc-ai/modules/cards-visual.sh` | 38 | 161 cols |
| `os/parc-ai/modules/cards-visual.sh` | 98 | 138 cols |
| `os/parc-ai/modules/cards-visual.sh` | 100 | 162 cols |
| `os/parc-ai/modules/cards-visual.sh` | 131 | 138 cols |
| `os/parc-ai/modules/cards-visual.sh` | 136 | 254 cols |
| `os/parc-ai/modules/cards-visual.sh` | 137 | 261 cols |
| `os/parc-ai/modules/cards-visual.sh` | 160 | 219 cols |
| `os/parc-ai/modules/cards-visual.sh` | 161 | 272 cols |
| `os/parc-ai/modules/cards-visual.sh` | 186 | 233 cols |
| `os/parc-ai/modules/cards-visual.sh` | 189 | 124 cols |
| `os/parc-ai/modules/cards-visual.sh` | 211 | 199 cols |
| `os/parc-ai/modules/cards-visual.sh` | 212 | 192 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 15 | 143 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 61 | 143 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 70 | 164 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 73 | 164 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 75 | 136 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 93 | 146 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 96 | 211 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 97 | 214 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 98 | 225 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 99 | 212 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 100 | 209 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 103 | 180 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 106 | 143 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 113 | 157 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 119 | 161 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 157 | 121 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 158 | 157 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 199 | 128 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 204 | 258 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 205 | 255 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 215 | 128 cols |
| `os/parc-ai/modules/cards-workspace.sh` | 217 | 139 cols |
| `os/parc-ai/modules/commerce.sh` | 15 | 125 cols |
| `os/parc-ai/modules/commerce.sh` | 85 | 127 cols |
| `os/parc-ai/modules/commerce.sh` | 110 | 125 cols |
| `os/parc-ai/modules/commerce.sh` | 154 | 126 cols |
| `os/parc-ai/modules/commerce.sh` | 163 | 135 cols |
| `os/parc-ai/modules/commerce.sh` | 173 | 140 cols |
| `os/parc-ai/modules/contacts.sh` | 31 | 134 cols |
| `os/parc-ai/modules/conversation.sh` | 61 | 126 cols |
| `os/parc-ai/modules/creative.sh` | 87 | 164 cols |
| `os/parc-ai/modules/creative.sh` | 88 | 167 cols |
| `os/parc-ai/modules/creative.sh` | 175 | 130 cols |
| `os/parc-ai/modules/creative.sh` | 266 | 159 cols |
| `os/parc-ai/modules/creative.sh` | 267 | 156 cols |
| `os/parc-ai/modules/creative.sh` | 268 | 150 cols |
| `os/parc-ai/modules/creative.sh` | 269 | 165 cols |
| `os/parc-ai/modules/creative.sh` | 270 | 166 cols |
| `os/parc-ai/modules/creative.sh` | 271 | 151 cols |
| `os/parc-ai/modules/creative.sh` | 272 | 162 cols |
| `os/parc-ai/modules/debugging.sh` | 228 | 143 cols |
| `os/parc-ai/modules/entertainment.sh` | 27 | 140 cols |
| `os/parc-ai/modules/entertainment.sh` | 52 | 126 cols |
| `os/parc-ai/modules/entertainment.sh` | 58 | 123 cols |
| `os/parc-ai/modules/entertainment.sh` | 63 | 129 cols |
| `os/parc-ai/modules/entertainment.sh` | 94 | 137 cols |
| `os/parc-ai/modules/execution.sh` | 152 | 122 cols |
| `os/parc-ai/modules/execution.sh` | 153 | 138 cols |
| `os/parc-ai/modules/execution.sh` | 154 | 124 cols |
| `os/parc-ai/modules/execution.sh` | 155 | 164 cols |
| `os/parc-ai/modules/execution.sh` | 156 | 133 cols |
| `os/parc-ai/modules/finance.sh` | 10 | 170 cols |
| `os/parc-ai/modules/knowledge-parcos.sh` | 303 | 124 cols |
| `os/parc-ai/modules/knowledge-parcos.sh` | 309 | 123 cols |
| `os/parc-ai/modules/language.sh` | 13 | 180 cols |
| `os/parc-ai/modules/language.sh` | 14 | 161 cols |
| `os/parc-ai/modules/language.sh` | 15 | 149 cols |
| `os/parc-ai/modules/language.sh` | 16 | 141 cols |
| `os/parc-ai/modules/language.sh` | 17 | 134 cols |
| `os/parc-ai/modules/language.sh` | 18 | 145 cols |
| `os/parc-ai/modules/language.sh` | 25 | 135 cols |
| `os/parc-ai/modules/math.sh` | 80 | 163 cols |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 74 | 160 cols |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 76 | 129 cols |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 90 | 242 cols |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 91 | 131 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 63 | 204 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 67 | 152 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 71 | 126 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 80 | 167 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 91 | 159 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 98 | 143 cols |
| `os/parc-ai/modules/nlp-engine.sh` | 105 | 180 cols |
| `os/parc-ai/modules/nlp-training.sh` | 52 | 152 cols |
| `os/parc-ai/modules/nlp-training.sh` | 57 | 158 cols |
| `os/parc-ai/modules/nlu.sh` | 15 | 151 cols |
| `os/parc-ai/modules/nlu.sh` | 23 | 188 cols |
| `os/parc-ai/modules/nlu.sh` | 62 | 159 cols |
| `os/parc-ai/modules/nlu.sh` | 73 | 154 cols |
| `os/parc-ai/modules/nlu.sh` | 74 | 143 cols |
| `os/parc-ai/modules/nlu.sh` | 88 | 123 cols |
| `os/parc-ai/modules/parcos-features.sh` | 325 | 129 cols |
| `os/parc-ai/modules/persona.sh` | 10 | 163 cols |
| `os/parc-ai/modules/persona.sh` | 12 | 153 cols |
| `os/parc-ai/modules/persona.sh` | 14 | 138 cols |
| `os/parc-ai/modules/persona.sh` | 16 | 135 cols |
| `os/parc-ai/modules/persona.sh` | 18 | 132 cols |
| `os/parc-ai/modules/persona.sh` | 20 | 150 cols |
| `os/parc-ai/modules/persona.sh` | 22 | 127 cols |
| `os/parc-ai/modules/persona.sh` | 24 | 133 cols |
| `os/parc-ai/modules/persona.sh` | 26 | 122 cols |
| `os/parc-ai/modules/persona.sh` | 28 | 145 cols |
| `os/parc-ai/modules/persona.sh` | 30 | 124 cols |
| `os/parc-ai/modules/persona.sh` | 39 | 128 cols |
| `os/parc-ai/modules/textgen.sh` | 124 | 135 cols |
| `os/parc-ai/modules/textgen.sh` | 127 | 122 cols |
| `os/parc-ai/modules/textgen.sh` | 136 | 134 cols |
| `os/parc-ai/modules/textgen.sh` | 183 | 136 cols |
| `os/parc-ai/modules/textgen.sh` | 185 | 138 cols |
| `os/parc-ai/modules/textgen.sh` | 386 | 218 cols |
| `os/parc-ai/modules/textgen.sh` | 399 | 153 cols |
| `os/parc-ai/modules/travel.sh` | 76 | 129 cols |
| `os/parc-ai/modules/travel.sh` | 77 | 126 cols |
| `os/parc-ai/modules/travel.sh` | 78 | 127 cols |
| `os/system/content-filter.sh` | 53 | 158 cols |
| `os/system/context-aware-adaptation.sh` | 42 | 168 cols |
| `os/system/default-apps.sh` | 40 | 125 cols |
| `os/system/default-apps.sh` | 49 | 256 cols |
| `os/system/flatpak-support.sh` | 31 | 136 cols |
| `os/system/gamemode-setup.sh` | 49 | 127 cols |
| `os/system/hardware-detect.sh` | 18 | 129 cols |
| `os/system/hardware-detect.sh` | 26 | 151 cols |
| `os/system/hardware-detect.sh` | 27 | 123 cols |
| `os/system/hardware-detect.sh` | 32 | 128 cols |
| `os/system/korrinos-errors.sh` | 18 | 135 cols |
| `os/system/korrinos-errors.sh` | 22 | 123 cols |
| `os/system/korrinos-errors.sh` | 162 | 219 cols |
| `os/system/korrinos-errors.sh` | 172 | 190 cols |
| `os/system/korrinos-errors.sh` | 243 | 177 cols |
| `os/system/korrinos-greetings.sh` | 300 | 127 cols |
| `os/system/korrinos-greetings.sh` | 303 | 131 cols |
| `os/system/korrinos-greetings.sh` | 306 | 127 cols |
| `os/system/korrinos-greetings.sh` | 309 | 123 cols |
| `os/system/korrinos-greetings.sh` | 312 | 129 cols |
| `os/system/parc-dust.sh` | 24 | 129 cols |
| `os/system/parc-dust.sh` | 41 | 158 cols |
| `os/system/parc-dust.sh` | 42 | 153 cols |
| `os/system/parc-dust.sh` | 43 | 245 cols |
| `os/system/parc-dust.sh` | 45 | 128 cols |
| `os/system/password-manager.sh` | 314 | 200 cols |
| `os/system/password-manager.sh` | 338 | 137 cols |
| `os/system/password-manager.sh` | 377 | 128 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 145 | 142 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 189 | 122 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 286 | 129 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 337 | 132 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 390 | 121 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 771 | 143 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 872 | 125 cols |
| `os/system/driver-manager/korrinos-drivers.sh` | 873 | 137 cols |
| `os/system/package-manager/korrinos-pkg.sh` | 461 | 121 cols |
| `os/system/package-manager/korrinos-pkg.sh` | 893 | 145 cols |
| `os/system/package-manager/korrinos-pkg.sh` | 921 | 168 cols |
| `os/system/appstore/korrinos-appstore.sh` | 355 | 130 cols |
| `os/system/appstore/korrinos-appstore.sh` | 359 | 129 cols |
| `os/system/appstore/korrinos-appstore.sh` | 363 | 121 cols |
| `os/system/appstore/korrinos-appstore.sh` | 370 | 143 cols |
| `os/system/monitor/korrinos-process.sh` | 23 | 141 cols |
| `os/system/monitor/korrinos-process.sh` | 84 | 305 cols |
| `os/system/monitor/korrinos-process.sh` | 85 | 307 cols |
| `os/system/monitor/korrinos-process.sh` | 169 | 134 cols |
| `os/system/monitor/korrinos-process.sh` | 179 | 124 cols |
| `os/system/security/korrinos-bugfix.sh` | 346 | 144 cols |
| `os/system/security/korrinos-bugfix.sh` | 403 | 148 cols |
| `os/system/security/korrinos-health.sh` | 211 | 134 cols |
| `os/system/security/korrinos-health.sh` | 395 | 192 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 39 | 135 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 40 | 133 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 42 | 121 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 43 | 131 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 55 | 125 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 153 | 149 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 165 | 139 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 213 | 151 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 254 | 121 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 255 | 125 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 256 | 152 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 282 | 135 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 332 | 145 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 365 | 150 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 403 | 122 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 404 | 149 cols |
| `os/system/cloud-sync/korrinos-cloud.sh` | 511 | 155 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 220 | 128 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 695 | 128 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 823 | 146 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 828 | 167 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 853 | 163 cols |
| `os/system/hardware-cert/korrinos-cert.sh` | 864 | 176 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 41 | 133 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 245 | 124 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 271 | 121 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 699 | 132 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 708 | 135 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1075 | 176 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1088 | 161 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1249 | 121 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1252 | 145 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1566 | 140 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1730 | 134 cols |
| `os/system/desktop-env/korrinos-desktop.sh` | 1770 | 148 cols |
| `os/system/update-system/korrinos-update.sh` | 239 | 127 cols |
| `os/system/update-system/korrinos-update.sh` | 751 | 152 cols |
| `os/system/update-system/korrinos-update.sh` | 783 | 122 cols |
| `os/system/update-system/korrinos-update.sh` | 813 | 125 cols |
| `os/system/update-system/korrinos-update.sh` | 816 | 130 cols |
| `os/system/installer/korrinos-installer.sh` | 284 | 121 cols |
| `os/system/installer/korrinos-installer.sh` | 324 | 129 cols |
| `os/system/mobile-companion/korrinos-mobile.sh` | 202 | 134 cols |
| `os/system/enterprise/korrinos-enterprise.sh` | 394 | 138 cols |
| `os/system/enterprise/korrinos-enterprise.sh` | 400 | 166 cols |
| `os/system/enterprise/korrinos-enterprise.sh` | 404 | 158 cols |
| `os/system/enterprise/korrinos-enterprise.sh` | 405 | 134 cols |
| `os/system/enterprise/korrinos-enterprise.sh` | 910 | 143 cols |
| `os/control-center/launch_feature.sh` | 129 | 198 cols |
| `os/control-center/launch_feature.sh` | 155 | 308 cols |
| `os/control-center/launch_feature.sh` | 159 | 310 cols |
| `os/control-center/launch_feature.sh` | 163 | 296 cols |
| `os/control-center/launch_feature.sh` | 167 | 199 cols |
| `os/control-center/launch_feature.sh` | 171 | 244 cols |
| `os/control-center/launch_feature.sh` | 175 | 152 cols |
| `os/control-center/launch_feature.sh` | 179 | 206 cols |
| `os/control-center/launch_feature.sh` | 183 | 272 cols |
| `os/territories/cue-watchdog.sh` | 40 | 153 cols |
| `os/territories/cue-watchdog.sh` | 60 | 164 cols |
| `os/territories/self-watchdog.sh` | 60 | 213 cols |
| `os/territories/secure/app-allowlist.sh` | 67 | 156 cols |
| `os/territories/secure/app-allowlist.sh` | 87 | 137 cols |
| `os/territories/secure/canary-monitor.sh` | 49 | 135 cols |
| `os/territories/secure/sip-guard.sh` | 36 | 149 cols |
| `os/territories/secure/zero-trust-config.sh` | 58 | 139 cols |
| `os/territories/secure/zero-trust-config.sh` | 59 | 125 cols |
| `os/territories/vibe-address/vibe-address.sh` | 46 | 178 cols |
| `os/territories/vibe-address/vibe-address.sh` | 201 | 182 cols |
| `os/territories/vibe-address/vibe-address.sh` | 235 | 156 cols |
| `os/territories/vibe-address/core/adapt.sh` | 164 | 133 cols |
| `os/territories/vibe-address/core/adapt.sh` | 165 | 132 cols |
| `os/territories/vibe-address/core/align.sh` | 44 | 121 cols |
| `os/territories/vibe-address/core/align.sh` | 66 | 124 cols |
| `os/territories/vibe-address/core/align.sh` | 85 | 195 cols |
| `os/territories/vibe-address/core/align.sh` | 100 | 190 cols |
| `os/territories/vibe-address/core/align.sh` | 109 | 206 cols |
| `os/territories/vibe-address/core/align.sh` | 158 | 128 cols |
| `os/territories/vibe-address/core/capacity.sh` | 115 | 145 cols |
| `os/territories/vibe-address/core/index.sh` | 155 | 225 cols |
| `os/territories/vibe-address/core/ingest.sh` | 88 | 121 cols |
| `os/territories/vibe-address/core/ingest.sh` | 307 | 128 cols |
| `os/territories/vibe-address/core/ir.sh` | 123 | 123 cols |
| `os/territories/vibe-address/core/lexin.sh` | 35 | 303 cols |
| `os/territories/vibe-address/core/lexin.sh` | 93 | 339 cols |
| `os/territories/vibe-address/core/lexin.sh` | 245 | 155 cols |
| `os/territories/vibe-address/core/phoneme.sh` | 63 | 130 cols |
| `os/territories/vibe-address/core/prf.sh` | 112 | 133 cols |
| `os/territories/vibe-address/core/query.sh` | 80 | 124 cols |
| `os/territories/vibe-address/core/query.sh` | 138 | 125 cols |
| `os/territories/vibe-address/core/query.sh` | 139 | 148 cols |
| `os/territories/vibe-address/core/selftest.sh` | 45 | 133 cols |
| `os/territories/vibe-address/core/selftest.sh` | 66 | 134 cols |
| `os/territories/vibe-address/core/selftest.sh` | 89 | 125 cols |
| `os/territories/vibe-address/core/selftest.sh` | 96 | 192 cols |
| `os/territories/vibe-address/core/selftest.sh` | 134 | 277 cols |
| `os/territories/vibe-address/core/selftest.sh` | 143 | 137 cols |
| `os/territories/vibe-address/core/selftest.sh` | 194 | 171 cols |
| `os/territories/vibe-address/core/selftest.sh` | 201 | 163 cols |
| `os/territories/vibe-address/core/store.sh` | 95 | 121 cols |
| `os/territories/vibe-address/core/store.sh` | 210 | 167 cols |
| `os/territories/game/frame-pacing.sh` | 28 | 123 cols |
| `os/territories/game/frame-pacing.sh` | 31 | 130 cols |
| `os/territories/game/game-studio.sh` | 28 | 127 cols |
| `os/territories/game/game-studio.sh` | 37 | 125 cols |
| `os/territories/game/perf-tune.sh` | 23 | 152 cols |
| `os/territories/game/save-archiver.sh` | 15 | 127 cols |
| `os/territories/hack/amnesia-firewall.sh` | 137 | 125 cols |
| `os/territories/hack/app-guard.sh` | 35 | 148 cols |
| `os/territories/hack/app-guard.sh` | 54 | 163 cols |
| `os/territories/hack/app-guard.sh` | 58 | 160 cols |
| `os/territories/hack/ephemeral-ram.sh` | 76 | 124 cols |
| `os/territories/hack/intent-hardware.sh` | 43 | 136 cols |
| `os/territories/hack/intent-hardware.sh` | 89 | 160 cols |
| `os/territories/hack/intent-hardware.sh` | 98 | 132 cols |
| `os/territories/hack/supply-chain.sh` | 81 | 137 cols |
| `os/territories/hack/threat-monitor.sh` | 53 | 197 cols |
| `os/territories/hack/threat-monitor.sh` | 82 | 130 cols |
| `os/territories/hack/threat-monitor.sh` | 128 | 154 cols |
| `os/branding/install-branding.sh` | 37 | 123 cols |
| `os/branding/install-branding.sh` | 40 | 132 cols |
| `os/data/user-profiles.sh` | 14 | 165 cols |
| `os/ai/voice-engine.sh` | 178 | 146 cols |
| `os/brand/install-boot-intro.sh` | 31 | 134 cols |
| `os/brand/install-boot-intro.sh` | 59 | 152 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 11 | 198 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 15 | 146 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 17 | 185 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 19 | 261 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 21 | 412 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 23 | 301 cols |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 25 | 131 cols |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 32 | 146 cols |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 46 | 147 cols |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 73 | 155 cols |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 10 | 170 cols |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 20 | 809 cols |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 22 | 375 cols |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 24 | 173 cols |
| `os/hardware-tech/smart-power-grid/smart-power-grid.sh` | 5 | 169 cols |
| `os/hardware-tech/smart-power-grid/smart-power-grid.sh` | 9 | 445 cols |
| `os/hardware-tech/cross-app-automation/cross-app-automation.sh` | 245 | 122 cols |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 275 | 121 cols |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 394 | 140 cols |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 432 | 151 cols |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 558 | 141 cols |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 595 | 140 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 16 | 138 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 351 | 128 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 433 | 136 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 441 | 155 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 463 | 145 cols |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 499 | 170 cols |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 453 | 244 cols |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 456 | 247 cols |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 11 | 206 cols |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 15 | 194 cols |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 17 | 253 cols |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 38 | 248 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 489 | 127 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 490 | 126 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 491 | 132 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 492 | 132 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 493 | 136 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 494 | 127 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 495 | 126 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 528 | 274 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 531 | 253 cols |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 534 | 289 cols |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 64 | 131 cols |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 83 | 129 cols |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 296 | 168 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 10 | 194 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 13 | 270 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 28 | 166 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 29 | 169 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 30 | 221 cols |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 32 | 127 cols |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 18 | 238 cols |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 30 | 170 cols |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 41 | 122 cols |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 66 | 267 cols |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 56 | 121 cols |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 296 | 146 cols |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 404 | 259 cols |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 407 | 262 cols |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 258 | 135 cols |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 392 | 141 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 15 | 217 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 177 | 147 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 254 | 151 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 298 | 170 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 300 | 147 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 321 | 176 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 323 | 147 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 358 | 127 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 360 | 147 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 401 | 128 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 416 | 124 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 442 | 135 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 446 | 213 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 450 | 131 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 459 | 128 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 464 | 154 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 539 | 163 cols |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 540 | 155 cols |
| `os/hardware-tech/predictive-prewarm/predictive-prewarm.sh` | 389 | 258 cols |
| `os/hardware-tech/predictive-prewarm/predictive-prewarm.sh` | 392 | 261 cols |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 62 | 121 cols |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 193 | 124 cols |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 218 | 126 cols |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 5 | 207 cols |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 9 | 1888 cols |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 11 | 329 cols |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 186 | 151 cols |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 385 | 139 cols |
| `os/hardware-tech/predictive-render/predictive-render.sh` | 5 | 148 cols |
| `os/hardware-tech/predictive-render/predictive-render.sh` | 9 | 416 cols |
| `os/hardware-tech/predictive-render/predictive-render.sh` | 11 | 267 cols |
| `os/hardware-tech/neural-super-res/neural-super-res.sh` | 10 | 174 cols |
| `os/hardware-tech/neural-super-res/neural-super-res.sh` | 14 | 254 cols |
| `os/hardware-tech/neural-super-res/neural-super-res.sh` | 16 | 315 cols |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 8 | 214 cols |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 13 | 345 cols |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 32 | 325 cols |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 35 | 214 cols |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 46 | 226 cols |
| `os/hardware-tech/hardware-tuning/14-categories.sh` | 8 | 188 cols |
| `os/hardware-tech/hardware-tuning/camera-tuning.sh` | 7 | 132 cols |
| `os/hardware-tech/hardware-tuning/camera-tuning.sh` | 9 | 127 cols |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 11 | 126 cols |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 13 | 175 cols |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 14 | 164 cols |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 15 | 181 cols |
| `os/hardware-tech/hardware-tuning/display-tuning.sh` | 15 | 126 cols |
| `os/hardware-tech/hardware-tuning/display-tuning.sh` | 28 | 211 cols |
| `os/hardware-tech/hardware-tuning/display-tuning.sh` | 30 | 124 cols |
| `os/hardware-tech/hardware-tuning/display-tuning.sh` | 31 | 172 cols |
| `os/hardware-tech/hardware-tuning/input-tuning.sh` | 6 | 169 cols |
| `os/hardware-tech/hardware-tuning/input-tuning.sh` | 9 | 130 cols |
| `os/hardware-tech/hardware-tuning/led-tuning.sh` | 15 | 125 cols |
| `os/hardware-tech/hardware-tuning/led-tuning.sh` | 17 | 154 cols |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 15 | 157 cols |
| `os/hardware-tech/hardware-tuning/network-tuning.sh` | 9 | 121 cols |
| `os/hardware-tech/hardware-tuning/network-tuning.sh` | 11 | 126 cols |
| `os/hardware-tech/hardware-tuning/power-tuning.sh` | 8 | 140 cols |
| `os/hardware-tech/hardware-tuning/power-tuning.sh` | 10 | 163 cols |
| `os/hardware-tech/hardware-tuning/power-tuning.sh` | 11 | 151 cols |
| `os/hardware-tech/hardware-tuning/security-tuning.sh` | 11 | 134 cols |
| `os/hardware-tech/hardware-tuning/storage-tuning.sh` | 14 | 192 cols |
| `os/hardware-tech/hardware-tuning/storage-tuning.sh` | 15 | 182 cols |
| `os/hardware-tech/hardware-tuning/usb-tuning.sh` | 16 | 141 cols |
| `os/hardware-tech/hardware-tuning/usb-tuning.sh` | 27 | 152 cols |
| `os/hardware-tech/hardware-tuning/usb-tuning.sh` | 37 | 141 cols |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 180 | 143 cols |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 181 | 161 cols |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 343 | 135 cols |
| `os/desktop/desktop-integration.sh` | 218 | 126 cols |
| `os/desktop/file-search.sh` | 15 | 243 cols |
| `os/desktop/gestures.sh` | 174 | 156 cols |
| `os/desktop/multi-monitor.sh` | 43 | 133 cols |
| `os/desktop/multi-monitor.sh` | 53 | 133 cols |
| `os/desktop/multi-monitor.sh` | 63 | 138 cols |
| `os/desktop/multi-monitor.sh` | 68 | 130 cols |
| `os/desktop/settings-gui.sh` | 268 | 126 cols |
| `os/desktop/settings-gui.sh` | 320 | 140 cols |
| `os/desktop/settings-gui.sh` | 333 | 125 cols |
| `os/desktop/settings-gui.sh` | 366 | 122 cols |
| `os/desktop/settings-gui.sh` | 376 | 123 cols |
| `os/desktop/settings-gui.sh` | 406 | 131 cols |
| `os/desktop/shortcuts.sh` | 290 | 122 cols |
| `os/desktop/system-tray.sh` | 98 | 124 cols |
| `os/bin/korrinos-launch` | 29 | 124 cols |

### C31 '|| true' swallows errors — 1006

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 61 | "$SUDO" mount --bind /proc  "$ROOTFS/proc"  2>/dev/null \|\| t |
| `os/build-distro.sh` | 62 | "$SUDO" mount --bind /sys   "$ROOTFS/sys"   2>/dev/null \|\| t |
| `os/build-distro.sh` | 63 | "$SUDO" mount --bind /dev   "$ROOTFS/dev"   2>/dev/null \|\| t |
| `os/build-distro.sh` | 64 | mountpoint -q "$ROOTFS/dev/pts" \|\| "$SUDO" mount -t devpts n |
| `os/build-distro.sh` | 170 | code \|\| true \ |
| `os/build-distro.sh` | 174 | go \|\| true \ |
| `os/build-distro.sh` | 175 | rustc cargo \|\| true \ |
| `os/build-distro.sh` | 214 | msttcorefonts \|\| true \ |
| `os/build-distro.sh` | 241 | dpkg --add-architecture i386 \|\| true |
| `os/build-distro.sh` | 242 | apt-get update -y \|\| true |
| `os/build-distro.sh` | 244 | steam-installer steam-devices \|\| true \ |
| `os/build-distro.sh` | 245 | lutris \|\| true \ |
| `os/build-distro.sh` | 246 | wine wine32 wine64 \|\| true \ |
| `os/build-distro.sh` | 248 | mangohud \|\| true \ |
| `os/build-distro.sh` | 255 | foobillard++ \|\| true \ |
| `os/build-distro.sh` | 270 | docker.io docker-compose \|\| true \ |
| `os/build-distro.sh` | 271 | podman podman-compose \|\| true \ |
| `os/build-distro.sh` | 310 | shotcut \|\| true \ |
| `os/build-distro.sh` | 352 | mountpoint -q "$ROOTFS/dev/pts" && "$SUDO" umount "$ROOTFS/d |
| `os/build-distro.sh` | 353 | mountpoint -q "$ROOTFS/proc" && "$SUDO" umount "$ROOTFS/proc |
| `os/build-distro.sh` | 354 | mountpoint -q "$ROOTFS/sys" && "$SUDO" umount "$ROOTFS/sys"  |
| `os/build-distro.sh` | 355 | mountpoint -q "$ROOTFS/dev" && "$SUDO" umount "$ROOTFS/dev"  |
| `os/build-distro.sh` | 364 | "$SUDO" cp /home/tinkerspace/linux-kernel/README.md "$ROOTFS |
| `os/build-distro.sh` | 365 | "$SUDO" cp /home/tinkerspace/linux-kernel/LICENSE "$ROOTFS/o |
| `os/build-distro.sh` | 366 | "$SUDO" cp /home/tinkerspace/linux-kernel/LICENSE "$ROOTFS/u |
| `os/build-distro.sh` | 373 | chmod +x /usr/local/bin/parc-world' 2>/dev/null \|\| true |
| `os/build-distro.sh` | 380 | chmod +x /usr/local/bin/korrinos' 2>/dev/null \|\| true |
| `os/build-distro.sh` | 387 | ln -sf "\$f" "$ROOTFS/usr/local/bin/korrinos-\$name" 2>/dev/ |
| `os/build-distro.sh` | 393 | ln -sf "\$f" "$ROOTFS/usr/local/bin/korrinos-\$name" 2>/dev/ |
| `os/build-distro.sh` | 395 | done' 2>/dev/null \|\| true |
| `os/build-distro.sh` | 403 | ln -sf "\$f" "$ROOTFS/usr/local/bin/\$name" 2>/dev/null \|\| t |
| `os/build-distro.sh` | 405 | done' 2>/dev/null \|\| true |
| `os/build-distro.sh` | 504 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-desktop.s |
| `os/build-distro.sh` | 505 | "$SUDO" chroot "$ROOTFS" systemctl enable vokk.service 2>/de |
| `os/build-distro.sh` | 506 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-autoupdat |
| `os/build-distro.sh` | 507 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-backup.ti |
| `os/build-distro.sh` | 508 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-firewall. |
| `os/build-distro.sh` | 509 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-health.ti |
| `os/build-distro.sh` | 522 | "$SUDO" chroot "$ROOTFS" systemctl enable lightdm.service 2> |
| `os/build-distro.sh` | 526 | useradd -m -s /bin/bash -G sudo,adm,dialout,cdrom,floppy,aud |
| `os/build-distro.sh` | 527 | echo "korrinos:korrinos" \| chpasswd 2>/dev/null \|\| true |
| `os/build-distro.sh` | 528 | echo "root:korrinos" \| chpasswd 2>/dev/null \|\| true |
| `os/build-distro.sh` | 541 | "$SUDO" chroot "$ROOTFS" bash -c 'echo "korrinos" > /etc/hos |
| `os/build-distro.sh` | 605 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-autoupdat |
| `os/build-distro.sh` | 606 | "$SUDO" chroot "$ROOTFS" systemctl enable korrinos-cloud-syn |
| `os/build-distro.sh` | 614 | ln -sf "\$f" "$ROOTFS/usr/local/bin/\$name" 2>/dev/null \|\| t |
| `os/build-distro.sh` | 616 | done' 2>/dev/null \|\| true |
| `os/build-distro.sh` | 621 | "$SUDO" chmod +x "$ROOTFS/usr/local/bin/korrinos-firstboot"  |
| `os/build-distro.sh` | 630 | "$SUDO" cp /home/tinkerspace/linux-kernel/os/branding/plymou |
| `os/build-distro.sh` | 667 | "$SUDO" mount --bind /proc "$ROOTFS/proc" 2>/dev/null \|\| tru |
| `os/build-distro.sh` | 668 | "$SUDO" mount --bind /sys  "$ROOTFS/sys"  2>/dev/null \|\| tru |
| `os/build-distro.sh` | 669 | "$SUDO" mount --bind /dev  "$ROOTFS/dev"  2>/dev/null \|\| tru |
| `os/build-distro.sh` | 670 | mountpoint -q "$ROOTFS/dev/pts" \|\| "$SUDO" mount -t devpts n |
| `os/build-distro.sh` | 677 | [ -d "$ROOTFS/$m" ] && mountpoint -q "$ROOTFS/$m" && "$SUDO" |
| `os/build-distro.sh` | 762 | "$SUDO" chown -R "$(id -u):$(id -g)" "$IMAGE" "$BUILD" 2>/de |
| `os/build-distro.sh` | 796 | iso9660 at_keyboard gfxterm gfxmenu all_video font terminal  |
| `os/install-parcai.sh` | 96 | read -r cmd < /tmp/korrinos_ai_pipe \|\| true |
| `os/install-parcai.sh` | 105 | mkfifo /tmp/korrinos_ai_pipe 2>/dev/null \|\| true |
| `os/install-parcai.sh` | 106 | chmod 666 /tmp/korrinos_ai_pipe 2>/dev/null \|\| true |
| `os/install-parcai.sh` | 114 | chattr +i "$INSTALL_DIR" 2>/dev/null \|\| true |
| `os/install-parcai.sh` | 115 | chattr +i "$SERVICE_DIR/korrinos-ai.service" 2>/dev/null \|\|  |
| `os/install-parcai.sh` | 118 | echo "korrinos-ai" >> /etc/korrinos/.protected 2>/dev/null \| |
| `os/iso-builder.sh` | 147 | sudo cp /boot/vmlinuz-* "$BUILD_DIR/casper/vmlinuz" 2>/dev/n |
| `os/iso-builder.sh` | 148 | sudo cp /boot/initrd.img-* "$BUILD_DIR/casper/initrd" 2>/dev |
| `os/iso-builder.sh` | 161 | sudo cp /usr/lib/grub/x86_64-efi/*.mod "$BUILD_DIR/boot/grub |
| `os/iso-builder.sh` | 182 | dd if=/dev/zero of="$efiimg" bs=1M count=6 status=none 2>/de |
| `os/iso-builder.sh` | 193 | cp /usr/lib/grub/x86_64-efi/*.mod "$mdir/boot/grub/" 2>/dev/ |
| `os/iso-builder.sh` | 194 | cp /usr/lib/grub/x86_64-efi/unicode.pf2 "$mdir/boot/grub/" 2 |
| `os/iso-builder.sh` | 195 | mcopy -i "$efiimg" -s "$mdir/EFI" "::EFI" 2>/dev/null \|\| tru |
| `os/iso-builder.sh` | 196 | mcopy -i "$efiimg" -s "$mdir/boot" "::boot" 2>/dev/null \|\| t |
| `os/parcos-agent.sh` | 41 | pkill -HUP xbindkeys 2>/dev/null \|\| true |
| `os/parcos-agent.sh` | 55 | gsettings set org.gnome.settings-daemon.plugins.media-keys c |
| `os/parcos-agent.sh` | 56 | gsettings set org.gnome.settings-daemon.plugins.media-keys.c |
| `os/parcos-agent.sh` | 57 | gsettings set org.gnome.settings-daemon.plugins.media-keys.c |
| `os/parcos-agent.sh` | 58 | gsettings set org.gnome.settings-daemon.plugins.media-keys.c |
| `os/parcos-agent.sh` | 70 | sed -i '/# KorrinOS Agent : Ctrl+AltGr/,+1d' "$HOME/.xbindke |
| `os/parcos-agent.sh` | 71 | sed -i '/control + alt + ISO_Level3_Shift/,+1d' "$HOME/.conf |
| `os/release.sh` | 91 | git add os/build-distro.sh README.md os/docs/RELEASE-*.md 2> |
| `os/release.sh` | 92 | git commit -q -m "release: bump to v${NEW_VER_FULL}" \|\| true |
| `os/release.sh` | 123 | git commit -q -m "release: v${NEW_VER_FULL} ISO SHA256 ${SHA |
| `os/release.sh` | 141 | git tag -a "v${NEW_VER_FULL}" -m "Release v${NEW_VER_FULL}"  |
| `os/release.sh` | 142 | git push origin "v${NEW_VER_FULL}" --quiet 2>/dev/null \|\| tr |
| `os/apps/battery-monitor.sh` | 95 | notify-send -u critical "Battery Low" "Battery at ${capacity |
| `os/apps/battery-monitor.sh` | 103 | notify-send -u critical "Battery CRITICAL" "Battery at ${cap |
| `os/apps/battery-monitor.sh` | 111 | notify-send -u critical "Battery Hot" "Temperature: $((temp  |
| `os/apps/battery-monitor.sh` | 186 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/apps/battery-monitor.sh` | 192 | echo ondemand > $cpu 2>/dev/null \|\| true |
| `os/apps/battery-monitor.sh` | 198 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/apps/battery-monitor.sh` | 205 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/apps/battery-monitor.sh` | 208 | echo 30 > /sys/class/backlight/*/brightness 2>/dev/null \|\| t |
| `os/apps/battery-monitor.sh` | 210 | bluetoothctl power off 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 36 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 60 | echo on > $dev 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 72 | kill -STOP $(cat /tmp/tinker-notifications) 2>/dev/null \|\| t |
| `os/apps/gaming-mode.sh` | 98 | cat $CPU_GOVERNOR_FILE > $cpu 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 103 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 123 | echo auto > $dev 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 134 | kill -CONT $(cat /tmp/tinker-notifications) 2>/dev/null \|\| t |
| `os/apps/gaming-support.sh` | 243 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/apps/gaming-support.sh` | 259 | echo schedutil > $cpu 2>/dev/null \|\| true |
| `os/apps/gaming-support.sh` | 263 | killall gamemoded 2>/dev/null \|\| true |
| `os/apps/package-manager.sh` | 159 | sudo snap refresh 2>/dev/null \|\| true |
| `os/apps/package-manager.sh` | 164 | flatpak update -y 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 23 | sudo apt clean 2>/dev/null \|\| sudo pacman -Sc --noconfirm 2> |
| `os/apps/system-cleaner.sh` | 31 | sudo find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 38 | sudo journalctl --vacuum-time=3d 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 43 | rm -rf ~/.cache/thumbnails/* 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 61 | rm -rf ~/.cache/google-chrome/Default/Cache/* 2>/dev/null \|\| |
| `os/apps/system-cleaner.sh` | 62 | rm -rf ~/.mozilla/firefox/*/Cache/* 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 67 | rm -rf ~/.cache/* 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 72 | sudo apt autoremove --purge -y 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 77 | rm -rf ~/.local/share/Trash/* 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 82 | find "$BACKUP_DIR" -type f -mtime +30 -delete 2>/dev/null \|\| |
| `os/apps/system-cleaner.sh` | 97 | cp -r ~/.config "$backup/" 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 98 | cp -r ~/.tinker "$backup/" 2>/dev/null \|\| true |
| `os/apps/gaming/emulator-manager.sh` | 89 | sudo apt install libretro-* 2>/dev/null \|\| true |
| `os/apps/gaming/fps-monitor.sh` | 88 | echo "$(date +%s)\|$cpu\|$mem\|$threads" >> "$LOG_FILE" 2>/dev/ |
| `os/apps/gaming/game-launcher.sh` | 66 | steam steam://rungameid/$game 2>/dev/null \|\| true |
| `os/apps/gaming/game-launcher.sh` | 69 | lutris $game 2>/dev/null \|\| true |
| `os/apps/gaming/game-replay.sh` | 64 | kill $(cat "$REPLAY_DIR/recording.pid") 2>/dev/null \|\| true |
| `os/apps/gaming/game-saves-sync.sh` | 54 | cp -r "$HOME/.local/share/Steam/userdata" "$backup/" 2>/dev/ |
| `os/apps/customization/cursor-themes.sh` | 46 | gsettings set org.gnome.desktop.interface cursor-theme "$the |
| `os/apps/customization/cursor-themes.sh` | 47 | gsettings set org.gnome.desktop.interface cursor-size $size  |
| `os/apps/customization/cursor-themes.sh` | 50 | sed -i "s/gtk-cursor-theme-name=.*/gtk-cursor-theme-name=\"$ |
| `os/apps/customization/cursor-themes.sh` | 51 | sed -i "s/gtk-cursor-theme-size=.*/gtk-cursor-theme-size=$si |
| `os/apps/customization/desktop-effects.sh` | 27 | gsettings set org.gnome.mutter compositing true 2>/dev/null  |
| `os/apps/customization/desktop-effects.sh` | 37 | xfconf-query -c xfwm4 -p /general/use_shade -s true 2>/dev/n |
| `os/apps/customization/desktop-effects.sh` | 49 | xfconf-query -c xfwm4 -p /general/use_compositing -s "$state |
| `os/apps/customization/font-manager.sh` | 13 | cp "$file" "$LOCAL_FONTS/" && fc-cache -f 2>/dev/null \|\| tru |
| `os/apps/customization/gtk-theme.sh` | 16 | gsettings set org.gnome.desktop.interface gtk-theme "$theme" |
| `os/apps/customization/icon-packs.sh` | 44 | gsettings set org.gnome.desktop.interface icon-theme "$theme |
| `os/apps/customization/icon-packs.sh` | 47 | sed -i "s/gtk-icon-theme-name=.*/gtk-icon-theme-name=\"$them |
| `os/apps/customization/korrinos-typeface.sh` | 83 | have fc-cache && fc-cache -f >/dev/null 2>&1 && log "font ca |
| `os/apps/customization/korrinos-typeface.sh` | 160 | have fc-cache && fc-cache -f >/dev/null 2>&1 \|\| true |
| `os/apps/customization/korrinos-typeface.sh` | 193 | if [ -w "$(dirname "$c")" ] 2>/dev/null; then rm -f "$c"; el |
| `os/apps/customization/korrinos-typeface.sh` | 198 | have fc-cache && fc-cache -f >/dev/null 2>&1 \|\| true |
| `os/apps/customization/korrinos-typeface.sh` | 263 | if [ -w "$d" ]; then rm -rf "$d"; else sudo -n rm -rf "$d" 2 |
| `os/apps/customization/korrinos-typeface.sh` | 267 | have fc-cache && fc-cache -f >/dev/null 2>&1 \|\| true |
| `os/apps/customization/login-theme.sh` | 57 | sudo sed -i "s/^greeter-session=.*/greeter-session=$theme/"  |
| `os/apps/customization/shell-theme.sh` | 14 | gsettings set org.gnome.shell.theme name "$theme" 2>/dev/nul |
| `os/apps/customization/window-animations.sh` | 30 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/apps/customization/window-animations.sh` | 33 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/apps/customization/window-animations.sh` | 34 | gsettings set org.gnome.shell enabled-extensions "['animatio |
| `os/apps/customization/window-animations.sh` | 48 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/apps/customization/window-animations.sh` | 50 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/apps/security/password-manager.sh` | 40 | echo "[]" > "$VAULT_FILE" 2>/dev/null \|\| true |
| `os/apps/hardware/gpio-manager.sh` | 64 | echo "$pin" \| sudo tee /sys/class/gpio/export > /dev/null 2> |
| `os/apps/hardware/gpio-manager.sh` | 65 | echo "out" \| sudo tee /sys/class/gpio/gpio$pin/direction > / |
| `os/apps/hardware/gpio-manager.sh` | 66 | echo "$value" \| sudo tee /sys/class/gpio/gpio$pin/value > /d |
| `os/apps/hardware/serial-uart.sh` | 33 | setserial -g /dev/tty* 2>/dev/null \| grep -v "unknown" \|\| tr |
| `os/apps/system/adaptive-power-grid.sh` | 123 | echo 80 > "$c" 2>/dev/null \|\| true |
| `os/apps/system/adaptive-power-grid.sh` | 130 | echo performance > "$cpu" 2>/dev/null \|\| true |
| `os/apps/system/adaptive-power-grid.sh` | 136 | echo schedutil > "$cpu" 2>/dev/null \|\| echo ondemand > "$cpu |
| `os/apps/system/adaptive-power-grid.sh` | 162 | echo 85 > "$c" 2>/dev/null \|\| true |
| `os/apps/system/adaptive-power-grid.sh` | 170 | echo $capacity > "$c" 2>/dev/null \|\| true |
| `os/apps/system/auto-updates.sh` | 115 | timeshift --create --comments "Pre-update $(date)" --tags D  |
| `os/apps/system/auto-updates.sh` | 117 | snapper create --description "Pre-update $(date)" 2>/dev/nul |
| `os/apps/system/auto-updates.sh` | 152 | grep -q "NOTIFY_AFTER_INSTALL=true" "$CONFIG_FILE" && notify |
| `os/apps/system/auto-updates.sh` | 201 | systemctl daemon-reload 2>/dev/null \|\| true |
| `os/apps/system/auto-updates.sh` | 202 | systemctl enable --now tinker-auto-update.timer 2>/dev/null  |
| `os/apps/system/auto-updates.sh` | 223 | grep -q "NOTIFY_BEFORE_INSTALL=true" "$CONFIG_FILE" && notif |
| `os/apps/system/auto-updates.sh` | 227 | grep -q "NOTIFY_BEFORE_INSTALL=true" "$CONFIG_FILE" && notif |
| `os/apps/system/backup-restore.sh` | 143 | \| "${TAR_COMPRESS[@]}" \| "${TAR_ENCRYPT[@]}" > "$out" && ok= |
| `os/apps/system/backup-restore.sh` | 146 | \| "${TAR_COMPRESS[@]}" > "$out" && ok=1 \|\| true |
| `os/apps/system/backup-restore.sh` | 174 | grep -q "NOTIFICATIONS=true" "$CONFIG_FILE" && notify-send " |
| `os/apps/system/backup-restore.sh` | 316 | systemctl daemon-reload 2>/dev/null \|\| true |
| `os/apps/system/backup-restore.sh` | 317 | systemctl enable --now tinker-backup.timer 2>/dev/null \|\| tr |
| `os/apps/system/cognitive-load.sh` | 29 | cls=$(xdotool getactivewindow getwindowclassname 2>/dev/null |
| `os/apps/system/context-aware.sh` | 140 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/context-aware.sh` | 141 | notify-send "Context: Work" "Balanced performance mode" 2>/d |
| `os/apps/system/context-aware.sh` | 144 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/context-aware.sh` | 145 | bash /home/tinkerspace/linux-kernel/os/apps/apps/gaming-mode |
| `os/apps/system/context-aware.sh` | 146 | notify-send "Context: Gaming" "Performance mode + Gaming mod |
| `os/apps/system/context-aware.sh` | 149 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/context-aware.sh` | 150 | notify-send "Context: Media" "Optimized for playback" 2>/dev |
| `os/apps/system/context-aware.sh` | 153 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/context-aware.sh` | 154 | pactl set-sink-mute @DEFAULT_SINK@ toggle 2>/dev/null \|\| tru |
| `os/apps/system/context-aware.sh` | 155 | notify-send "Context: Meeting" "Notifications muted" 2>/dev/ |
| `os/apps/system/context-aware.sh` | 158 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/context-aware.sh` | 159 | notify-send "Context: Travel" "Power saving mode" 2>/dev/nul |
| `os/apps/system/context-aware.sh` | 162 | bash /home/tinkerspace/linux-kernel/os/apps/system/power-man |
| `os/apps/system/fast-boot.sh` | 117 | systemctl daemon-reload 2>/dev/null \|\| true |
| `os/apps/system/file-versioning.sh` | 83 | diff "$VERSIONS_DIR/$id/$v1" "$VERSIONS_DIR/$id/$v2" \|\| true |
| `os/apps/system/focus-mode.sh` | 43 | gsettings set org.gnome.desktop.notifications show-banners f |
| `os/apps/system/focus-mode.sh` | 57 | gsettings set org.gnome.desktop.notifications show-banners t |
| `os/apps/system/parental-controls.sh` | 62 | notify-send -u critical "Bedtime" "Computer should be off!"  |
| `os/apps/system/pomodoro-timer.sh` | 41 | notify-send "Pomodoro" "Work session $count started!" 2>/dev |
| `os/apps/system/pomodoro-timer.sh` | 43 | notify-send "Pomodoro" "Work session complete!" 2>/dev/null  |
| `os/apps/system/pomodoro-timer.sh` | 48 | notify-send "Pomodoro" "Long break time!" 2>/dev/null \|\| tru |
| `os/apps/system/pomodoro-timer.sh` | 52 | notify-send "Pomodoro" "Short break time!" 2>/dev/null \|\| tr |
| `os/apps/system/pomodoro-timer.sh` | 68 | notify-send "Timer" "Starting $minutes minute timer" 2>/dev/ |
| `os/apps/system/pomodoro-timer.sh` | 70 | notify-send "Timer" "Time's up!" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 109 | echo "$cpu_governor" > "$cpu" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 117 | echo "$cpu_min_freq" > "$cpu" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 122 | echo "$cpu_max_freq" > "$cpu" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 129 | echo "on" > /sys/class/drm/card0/device/power_dpm_force_perf |
| `os/apps/system/power-manager.sh` | 131 | echo "auto" > /sys/class/drm/card0/device/power_dpm_force_pe |
| `os/apps/system/power-manager.sh` | 138 | echo "$disk_scheduler" > "$disk" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 146 | iw dev "$iface" set power_save "$wifi_powersave" 2>/dev/null |
| `os/apps/system/power-manager.sh` | 153 | bluetoothctl power "$bluetooth" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 161 | [ -n "$max" ] && echo $((max * backlight / 100)) > "$bl" 2>/ |
| `os/apps/system/power-manager.sh` | 169 | echo 1000 > "$usb" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 172 | echo auto > "$usb" 2>/dev/null \|\| true |
| `os/apps/system/power-manager.sh` | 181 | notify-send "Power Profile" "Switched to $profile" 2>/dev/nu |
| `os/apps/system/rollback-recovery.sh` | 103 | grep -q "NOTIFICATIONS=true" "$CONFIG_FILE" && notify-send " |
| `os/apps/system/rollback-recovery.sh` | 151 | grep -q "NOTIFICATIONS=true" "$CONFIG_FILE" && notify-send " |
| `os/apps/system/system-monitor.sh` | 155 | [ $cpu -gt ${alert_cpu:-90} ] && notify-send -u critical "CP |
| `os/apps/system/system-monitor.sh` | 156 | [ $(echo "$mem" \| cut -d. -f1) -gt ${alert_mem:-90} ] && not |
| `os/apps/system/system-monitor.sh` | 157 | [ $disk -gt ${alert_disk:-90} ] && notify-send -u critical " |
| `os/apps/network/bandwidth-limiter.sh` | 50 | sudo tc qdisc del dev eth0 root 2>/dev/null \|\| true |
| `os/apps/network/firewall-gui.sh` | 81 | iptables) systemctl enable --now iptables 2>&1 \|\| true ;; |
| `os/apps/network/firewall-gui.sh` | 96 | iptables) systemctl stop iptables 2>&1 \|\| true ;; |
| `os/apps/network/firewall-gui.sh` | 209 | echo "$(date +%s)\|policy\|$policy" >> "$LOG_FILE" 2>/dev/null |
| `os/apps/network/firewall-gui.sh` | 223 | iptables -A INPUT -m recent --name portscan --rcheck --secon |
| `os/apps/network/firewall-gui.sh` | 224 | iptables -A FORWARD -m recent --name portscan --rcheck --sec |
| `os/apps/network/hotspot-manager.sh` | 46 | nmcli connection down Hotspot 2>/dev/null \|\| true |
| `os/apps/network/proxy-manager.sh` | 67 | sed -i '/http_proxy/d' ~/.bashrc 2>/dev/null \|\| true |
| `os/apps/network/proxy-manager.sh` | 68 | sed -i '/https_proxy/d' ~/.bashrc 2>/dev/null \|\| true |
| `os/apps/network/speed-test.sh` | 87 | echo "$(date +%s)\|${d1}" >> "$RESULTS_FILE" 2>/dev/null \|\| t |
| `os/apps/network/vpn-manager.sh` | 110 | (wg-quick up "$profile" 2>/dev/null \|\| true) &>/dev/null |
| `os/apps/network/vpn-manager.sh` | 139 | sudo ip link set "$iface" down 2>/dev/null \|\| true |
| `os/apps/network/vpn-manager.sh` | 147 | sudo wg-quick down "$name" 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive-cli.sh` | 83 | sudo insmod /lib/modules/$(uname -r)/extra/hyperdrive.ko 2>/ |
| `os/hyperdrive/hyperdrive-cli.sh` | 124 | sudo renice -n -20 -p "$pid" 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive-cli.sh` | 153 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpu |
| `os/hyperdrive/hyperdrive-cli.sh` | 157 | echo always \| sudo tee /proc/sys/vm/nr_hugepages >/dev/null  |
| `os/hyperdrive/hyperdrive-cli.sh` | 161 | echo 1 \| sudo tee /proc/sys/vm/compact_memory >/dev/null 2>& |
| `os/hyperdrive/hyperdrive-cli.sh` | 166 | echo 3 \| sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 \| |
| `os/hyperdrive/hyperdrive-cli.sh` | 170 | echo never \| sudo tee /sys/kernel/mm/transparent_hugepage/en |
| `os/hyperdrive/hyperdrive-cli.sh` | 192 | dd if=/dev/zero of=/tmp/bench bs=1M count=1024 2>&1 \| grep - |
| `os/hyperdrive/hyperdrive-daemon.sh` | 61 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpu |
| `os/hyperdrive/hyperdrive-daemon.sh` | 101 | # xrandr --output $(xrandr \| grep connected \| head -1 \| awk  |
| `os/hyperdrive/hyperdrive-daemon.sh` | 111 | echo 3 \| sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 \| |
| `os/hyperdrive/hyperdrive-daemon.sh` | 114 | echo 1 \| sudo tee /proc/sys/vm/compact_memory >/dev/null 2>& |
| `os/hyperdrive/hyperdrive-daemon.sh` | 126 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpu |
| `os/hyperdrive/hyperdrive-daemon.sh` | 129 | echo always \| sudo tee /proc/sys/vm/nr_hugepages >/dev/null  |
| `os/hyperdrive/hyperdrive-daemon.sh` | 132 | sudo renice -n -10 $$ 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive-daemon.sh` | 147 | kill $(cat "$HD_STATE") 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 211 | sudo swapoff /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 212 | sudo echo "lz4" > /sys/block/zram0/comp_algorithm 2>/dev/nul |
| `os/hyperdrive/hyperdrive.sh` | 213 | sudo echo "2G" > /sys/block/zram0/disksize 2>/dev/null \|\| tr |
| `os/hyperdrive/hyperdrive.sh` | 214 | sudo mkswap /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 215 | sudo swapon /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 269 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpu |
| `os/parc-ai/install-parcos.sh` | 27 | cp "$SCRIPT_DIR"/modules/*.sh "$INSTALL_DIR/modules/" 2>/dev |
| `os/parc-ai/install-parcos.sh` | 28 | [ -d "$SCRIPT_DIR/overlay" ] && cp -r "$SCRIPT_DIR/overlay/" |
| `os/parc-ai/install-parcos.sh` | 29 | [ -d "$SCRIPT_DIR/model" ] && cp -r "$SCRIPT_DIR/model/"* "$ |
| `os/parc-ai/install-parcos.sh` | 62 | systemctl start ${SERVICE_NAME}.service 2>/dev/null \|\| true |
| `os/parc-ai/install-parcos.sh` | 69 | chattr +i "$INSTALL_DIR/bin/vokk" 2>/dev/null \|\| true |
| `os/parc-ai/install-parcos.sh` | 70 | chattr +i "$INSTALL_DIR" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-backup.sh` | 82 | rsync -a --progress $exclude_args "$src/" "$backup_path/$dir |
| `os/parc-ai/korrinos-backup.sh` | 197 | rsync -a --progress "$dir/" "$target/" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-backup.sh` | 254 | rsync -rc "$src/" "$new_path/$dir_name/" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-backup.sh` | 259 | rsync -a "$src/" "$new_path/$dir_name/" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 26 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 27 | sudo apt autoremove -y 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 39 | find ~/.cache -type f -name "*.tmp" -delete 2>/dev/null \|\| t |
| `os/parc-ai/korrinos-cleanup.sh` | 40 | find ~/.cache -type f -name "*.log" -delete 2>/dev/null \|\| t |
| `os/parc-ai/korrinos-cleanup.sh` | 41 | find ~/.cache -type d -name "thumbnails" -exec rm -rf {} + 2 |
| `os/parc-ai/korrinos-cleanup.sh` | 52 | find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 53 | find ~/.local/tmp -type f -atime +7 -delete 2>/dev/null \|\| t |
| `os/parc-ai/korrinos-cleanup.sh` | 64 | sudo find /var/log -type f -name "*.gz" -delete 2>/dev/null  |
| `os/parc-ai/korrinos-cleanup.sh` | 65 | sudo find /var/log -type f -name "*.old" -delete 2>/dev/null |
| `os/parc-ai/korrinos-cleanup.sh` | 66 | sudo journalctl --vacuum-time=3d 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 77 | rm -rf ~/.local/share/Trash/* 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 87 | sudo apt autoremove --purge -y 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 94 | docker system prune -f 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 128 | find "$cache_dir" -type f -name "*.tmp" -delete 2>/dev/null  |
| `os/parc-ai/korrinos-cleanup.sh` | 129 | find "$cache_dir" -type f -name "*.cache" -delete 2>/dev/nul |
| `os/parc-ai/korrinos-cleanup.sh` | 144 | find "$cache_dir" -type f -atime +30 -delete 2>/dev/null \|\|  |
| `os/parc-ai/korrinos-cleanup.sh` | 235 | fc-cache -f 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 240 | sudo mandb -q 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 245 | sudo journalctl --vacuum-size=100M 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 254 | sudo fstrim -av 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-dock.sh` | 200 | pkill plank 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-dock.sh` | 204 | pkill -f "plank --preferences" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-dock.sh` | 217 | pkill plank 2>/dev/null && echo "Plank dock stopped" \|\| true |
| `os/parc-ai/korrinos-dock.sh` | 218 | pkill -f "korrinos/custom/dock.sh" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-focus.sh` | 55 | "<false>" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-focus.sh` | 58 | pkill -f dunst 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-focus.sh` | 105 | "<true>" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-focus.sh` | 109 | pkill -f "focus_ambient" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-focus.sh` | 112 | [ -f "$FOCUS_DIR/timer.pid" ] && kill "$(cat "$FOCUS_DIR/tim |
| `os/parc-ai/korrinos-liquid-glass.sh` | 190 | pkill picom 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-power.sh` | 45 | echo performance 2>/dev/null \| sudo tee "$gov" > /dev/null 2 |
| `os/parc-ai/korrinos-power.sh` | 50 | nvidia-smi -pl 200 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-power.sh` | 58 | echo powersave 2>/dev/null \| sudo tee "$gov" > /dev/null 2>& |
| `os/parc-ai/korrinos-power.sh` | 62 | nvidia-smi -pl 150 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-power.sh` | 70 | echo powersave 2>/dev/null \| sudo tee "$gov" > /dev/null 2>& |
| `os/parc-ai/korrinos-power.sh` | 74 | nvidia-smi -pl 80 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-power.sh` | 78 | echo 1 2>/dev/null \| sudo tee "$iface" > /dev/null 2>&1 \|\| t |
| `os/parc-ai/korrinos-smoothui.sh` | 84 | sudo apt install -y xdotool 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-smoothui.sh` | 90 | sudo apt install -y libinput-tools 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-smoothui.sh` | 227 | pkill picom 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-smoothui.sh` | 238 | gsettings set org.gnome.desktop.peripherals.touchpad natural |
| `os/parc-ai/korrinos-smoothui.sh` | 239 | gsettings set org.gnome.desktop.peripherals.mouse natural-sc |
| `os/parc-ai/korrinos-smoothui.sh` | 245 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/parc-ai/korrinos-smoothui.sh` | 251 | gsettings set org.gnome.desktop.interface font-antialiasing  |
| `os/parc-ai/korrinos-smoothui.sh` | 252 | gsettings set org.gnome.desktop.interface font-hinting 'slig |
| `os/parc-ai/korrinos-smoothui.sh` | 284 | gsettings set org.gnome.desktop.peripherals.touchpad natural |
| `os/parc-ai/korrinos-smoothui.sh` | 285 | gsettings set org.gnome.desktop.peripherals.touchpad two-fin |
| `os/parc-ai/korrinos-smoothui.sh` | 286 | gsettings set org.gnome.desktop.peripherals.touchpad edge-sc |
| `os/parc-ai/korrinos-smoothui.sh` | 302 | gsettings set org.gnome.desktop.wm.keybindings show-desktop  |
| `os/parc-ai/korrinos-smoothui.sh` | 303 | gsettings set org.gnome.shell.keybindings toggle-overview "[ |
| `os/parc-ai/korrinos-smoothui.sh` | 310 | gsettings set org.gnome.desktop.wm.keybindings toggle-messag |
| `os/parc-ai/korrinos-smoothui.sh` | 316 | gsettings set org.gnome.shell.keybindings toggle-application |
| `os/parc-ai/korrinos-smoothui.sh` | 322 | gsettings set org.gnome.desktop.interface enable-hot-corners |
| `os/parc-ai/korrinos-smoothui.sh` | 346 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/parc-ai/korrinos-smoothui.sh` | 347 | gsettings set org.gnome.shell.extensions.jupiter rollback-an |
| `os/parc-ai/korrinos-smoothui.sh` | 352 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/parc-ai/korrinos-smoothui.sh` | 353 | gsettings set org.gnome.shell.extensions.jupiter rollback-an |
| `os/parc-ai/korrinos-smoothui.sh` | 358 | gsettings set org.gnome.desktop.interface enable-animations  |
| `os/parc-ai/korrinos-widgets-panel.sh` | 187 | pkill -f "korrinos-left.conf" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-widgets-panel.sh` | 188 | pkill -f "korrinos-right.conf" 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-widgets-panel.sh` | 206 | pkill -f "korrinos-left.conf" 2>/dev/null && echo "Left pane |
| `os/parc-ai/korrinos-widgets-panel.sh` | 207 | pkill -f "korrinos-right.conf" 2>/dev/null && echo "Right pa |
| `os/parc-ai/parc-ai.sh` | 195 | shift 2>/dev/null \|\| true |
| `os/parc-ai/parc-ai.sh` | 232 | bash "$VIBE_ENGINE/vibe-address.sh" ask "$query" 2>/dev/null |
| `os/parc-ai/parc-ai.sh` | 246 | history 2>/dev/null \| grep -i "$query" \| tail -5 \|\| true |
| `os/parc-ai/parc-ai.sh` | 370 | ai_learn_teach "$query" "$answer" 2>/dev/null \|\| true |
| `os/parc-ai/parc-ai.sh` | 375 | ai_narrate_creative "$query" "$answer" 2>/dev/null \|\| true |
| `os/parc-ai/parc-ai.sh` | 397 | ai_learn_teach "$query" "$web_answer" 2>/dev/null \|\| true |
| `os/parc-ai/parcai-model-pipeline.sh` | 603 | cp "$CHECKPOINT_DIR/base_model.pt" "$RELEASE_DIR/" 2>/dev/nu |
| `os/parc-ai/parcai-model-pipeline.sh` | 604 | cp "$CHECKPOINT_DIR/tokenizer.json" "$RELEASE_DIR/" 2>/dev/n |
| `os/parc-ai/parcai-model-pipeline.sh` | 605 | cp "$LORA_DIR/lora_weights.pt" "$RELEASE_DIR/" 2>/dev/null \| |
| `os/parc-ai/parcai-model-pipeline.sh` | 606 | cp "$MODEL_DIR/release/korrinos_ai.onnx" "$RELEASE_DIR/" 2>/ |
| `os/parc-ai/parcos-ai-installer.sh` | 106 | "$INSTALL_DIR/config/uninstall-blocker.sh" --setup-cron 2>/d |
| `os/parc-ai/parcos-backup.sh` | 23 | 2>/dev/null \|\| true |
| `os/parc-ai/parcos-backup.sh` | 40 | 2>/dev/null \|\| true |
| `os/parc-ai/parcos-backup.sh` | 55 | 2>/dev/null \|\| true |
| `os/parc-ai/parcos-backup.sh` | 73 | 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 49 | / 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 133 | sudo apt --fix-broken install 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 152 | sudo dpkg --configure -a 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 153 | sudo apt --fix-broken install -y 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 157 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 161 | sudo chmod 1777 /tmp 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 165 | sudo systemctl daemon-reload 2>/dev/null \|\| true |
| `os/parc-ai/parcos-settings.sh` | 157 | redshift -O 2>/dev/null \|\| true |
| `os/parc-ai/parcos-settings.sh` | 159 | redshift -x 2>/dev/null \|\| true |
| `os/parc-ai/parcos-settings.sh` | 164 | amixer set Master "$val%" 2>/dev/null \|\| true |
| `os/parc-ai/parcos-settings.sh` | 168 | amixer set Master toggle 2>/dev/null \|\| true |
| `os/parc-ai/parcos-settings.sh` | 172 | nmcli radio wifi on 2>/dev/null \|\| rfkill unblock wifi 2>/de |
| `os/parc-ai/parcos-settings.sh` | 174 | nmcli radio wifi off 2>/dev/null \|\| rfkill block wifi 2>/dev |
| `os/parc-ai/parcos-settings.sh` | 179 | bluetoothctl power on 2>/dev/null \|\| rfkill unblock bluetoot |
| `os/parc-ai/parcos-settings.sh` | 181 | bluetoothctl power off 2>/dev/null \|\| rfkill block bluetooth |
| `os/parc-ai/parcos-settings.sh` | 186 | performance) echo performance \| sudo tee /sys/devices/system |
| `os/parc-ai/parcos-settings.sh` | 187 | powersave)   echo powersave \| sudo tee /sys/devices/system/c |
| `os/parc-ai/parcos-settings.sh` | 188 | balanced)    echo schedutil \| sudo tee /sys/devices/system/c |
| `os/parc-ai/parcos-tools.sh` | 111 | sudo apt autoremove -y 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 112 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 117 | sudo find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 121 | rm -rf ~/.cache/thumbnails/* 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 122 | rm -rf ~/.cache/pip/http 2>/dev/null \|\| true |
| `os/parc-ai/parcos-uninstall-blocker.sh` | 21 | local bad=$(cd / && md5sum -c "$CHECKSUM_FILE" 2>&1 \| grep - |
| `os/parc-ai/parcos-usb.sh` | 33 | sudo umount "${device}"* 2>/dev/null \|\| true |
| `os/parc-ai/parcos-usb.sh` | 68 | sudo umount "${device}"* 2>/dev/null \|\| true |
| `os/parc-ai/modules/device.sh` | 99 | playerctl metadata 2>/dev/null \| head -5 \|\| true |
| `os/system/adaptive-power-grid.sh` | 206 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 212 | echo ondemand > $cpu 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 218 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 226 | nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 227 | nvidia-smi -ac 5001,1500 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 231 | nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 235 | nvidia-smi -pm 0 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 243 | iw dev wlan0 set power_save on 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 246 | iw dev wlan0 set power_save off 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 252 | echo 2 > /sys/module/usbcore/parameters/autosuspend 2>/dev/n |
| `os/system/adaptive-power-grid.sh` | 259 | xrandr --output $(xrandr \| grep connected \| head -1 \| awk '{ |
| `os/system/adaptive-power-grid.sh` | 297 | echo ondemand > $cpu 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 299 | nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 304 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/system/adaptive-power-grid.sh` | 327 | echo "mq-deadline" > $disk 2>/dev/null \|\| true |
| `os/system/auto-updates.sh` | 106 | cp -r /etc/apt "$backup_dir/" 2>/dev/null \|\| true |
| `os/system/backup-restore.sh` | 67 | 2>/dev/null \|\| true |
| `os/system/backup-restore.sh` | 78 | flatpak list --app --columns=application > "$backup_path/fla |
| `os/system/content-filter.sh` | 50 | sudo cp /etc/hosts /etc/hosts.tinkos-cf.bak 2>/dev/null \|\| t |
| `os/system/content-filter.sh` | 51 | cat "$backup" $'\n' >/dev/null 2>&1 \|\| true |
| `os/system/content-filter.sh` | 57 | sudo resolvectl dns "$(resolvectl status 2>/dev/null \| awk ' |
| `os/system/content-filter.sh` | 64 | sudo rm -f "$CF_HOSTS_MARK" /tmp/tinkos-hosts 2>/dev/null \|\| |
| `os/system/context-aware.sh` | 182 | echo "$cpu_governor" > $cpu 2>/dev/null \|\| true |
| `os/system/context-aware.sh` | 188 | nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/context-aware.sh` | 189 | nvidia-smi -ac 5001,1500 2>/dev/null \|\| true |
| `os/system/context-aware.sh` | 192 | nvidia-smi -pm 0 2>/dev/null \|\| true |
| `os/system/context-aware.sh` | 200 | quiet) echo 80 > $fan 2>/dev/null \|\| true ;; |
| `os/system/context-aware.sh` | 201 | balanced) echo 150 > $fan 2>/dev/null \|\| true ;; |
| `os/system/context-aware.sh` | 202 | aggressive) echo 255 > $fan 2>/dev/null \|\| true ;; |
| `os/system/context-aware.sh` | 211 | echo $target > /sys/class/backlight/*/brightness 2>/dev/null |
| `os/system/context-aware.sh` | 218 | gsettings set org.gnome.desktop.notifications show-banners f |
| `os/system/context-aware.sh` | 222 | gsettings set org.gnome.desktop.notifications show-banners t |
| `os/system/context-aware.sh` | 226 | gsettings set org.gnome.desktop.notifications show-banners t |
| `os/system/context-aware.sh` | 234 | pkill -f "update-manager" 2>/dev/null \|\| true |
| `os/system/context-aware.sh` | 235 | pkill -f "tracker-miner" 2>/dev/null \|\| true |
| `os/system/default-apps.sh` | 24 | (ls /usr/bin/apt >/dev/null 2>&1 && apt-get install -y gh) \| |
| `os/system/default-apps.sh` | 31 | && dpkg -i /tmp/vscode.deb \|\| true) |
| `os/system/default-apps.sh` | 41 | && dpkg -i /tmp/brave.deb \|\| true) |
| `os/system/digital-twin.sh` | 107 | systemctl list-units --type=service > "$state_file/services. |
| `os/system/digital-twin.sh` | 115 | pacman -Q > "$state_file/packages.txt" 2>/dev/null \|\| true |
| `os/system/digital-twin.sh` | 270 | pacman -S --dry-run "$package" > "$sim_dir/pacman_dryrun.txt |
| `os/system/digital-twin.sh` | 274 | du -sh /var/cache/apt/archives/ >> "$sim_dir/impact.txt" 2>/ |
| `os/system/digital-twin.sh` | 290 | nginx -t -c "$config" > "$sim_dir/config_test.txt" 2>&1 \|\| t |
| `os/system/digital-twin.sh` | 303 | systemctl status "$service" > "$sim_dir/service_status.txt"  |
| `os/system/digital-twin.sh` | 306 | systemctl list-dependencies "$service" > "$sim_dir/dependenc |
| `os/system/digital-twin.sh` | 321 | rpm -qa \| grep kernel > "$sim_dir/available_kernels.txt" 2>/ |
| `os/system/digital-twin.sh` | 334 | diff "$state1/processes.txt" "$state2/processes.txt" >> "$si |
| `os/system/digital-twin.sh` | 337 | diff "$state1/memory.txt" "$state2/memory.txt" >> "$sim_dir/ |
| `os/system/digital-twin.sh` | 340 | diff "$state1/disk.txt" "$state2/disk.txt" >> "$sim_dir/comp |
| `os/system/digital-twin.sh` | 408 | diff "$old_procs" "$new_procs" \| grep "^>" \| awk '{print $2} |
| `os/system/fast-boot.sh` | 81 | sudo systemctl disable "$service" 2>/dev/null \|\| true |
| `os/system/fast-boot.sh` | 92 | sudo apt clean 2>/dev/null \|\| true |
| `os/system/fast-boot.sh` | 93 | sudo pacman -Sc --noconfirm 2>/dev/null \|\| true |
| `os/system/fast-boot.sh` | 103 | sudo apt install -y preload 2>/dev/null \|\| true |
| `os/system/flatpak-support.sh` | 33 | sudo apt install -y plasma-discover-backend-flatpak 2>/dev/n |
| `os/system/gaming-meta.sh` | 55 | sudo apt remove -y steam-installer steam lutris wine mangohu |
| `os/system/gpu-config.sh` | 85 | sudo nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/gpu-config.sh` | 88 | sudo nvidia-smi -pl 100 2>/dev/null \|\| true |
| `os/system/install-greetings.sh` | 106 | plymouth display-message --text="$MSG" 2>/dev/null \|\| true |
| `os/system/install-greetings.sh` | 108 | plymouth hide-message --text="$MSG" 2>/dev/null \|\| true |
| `os/system/install-greetings.sh` | 118 | /opt/korrinos/os/system/korrinos-greetings.sh boot 2>/dev/nu |
| `os/system/install-greetings.sh` | 125 | sudo systemctl enable korrinos-greet-boot.service 2>/dev/nul |
| `os/system/install-greetings.sh` | 126 | sudo systemctl enable korrinos-greet-wake.target 2>/dev/null |
| `os/system/installer.sh` | 176 | sudo systemd-machine-id-setup --root=/mnt 2>/dev/null \|\| tru |
| `os/system/installer.sh` | 193 | "grub-install --target=x86_64-efi --efi-directory=/boot --bo |
| `os/system/installer.sh` | 194 | grub-mkconfig -o /boot/grub/grub.cfg \|\| true" |
| `os/system/installer.sh` | 203 | sudo umount /mnt/proc /mnt/sys /mnt/dev /mnt/run 2>/dev/null |
| `os/system/korrinos-apt-hook.sh` | 112 | echo "[$(date '+%Y-%m-%d %H:%M:%S')] $action: $message" >> " |
| `os/system/korrinos-firstboot.sh` | 22 | -s "$WALLPAPER_DIR/default.jpg" 2>/dev/null \|\| true |
| `os/system/korrinos-firstboot.sh` | 24 | -s 5 2>/dev/null \|\| true |
| `os/system/korrinos-firstboot.sh` | 29 | xfconf-query -c xsettings -p /Net/ThemeName -s "Adwaita-dark |
| `os/system/korrinos-firstboot.sh` | 30 | xfconf-query -c xsettings -p /Net/IconThemeName -s "Adwaita" |
| `os/system/korrinos-firstboot.sh` | 31 | xfconf-query -c xsettings -p /Gtk/FontName -s "Sans 10" 2>/d |
| `os/system/korrinos-mascot.sh` | 24 | echo "[$(date '+%H:%M:%S')] Hook registered: $event  $callba |
| `os/system/korrinos-mascot.sh` | 117 | echo "[$(date '+%H:%M:%S')] Event triggered: $event" >> "$MA |
| `os/system/korrinos-mascot.sh` | 119 | echo "[$(date '+%H:%M:%S')] Unknown event: $event" >> "$MASC |
| `os/system/optimization-toggles.sh` | 53 | echo schedutil > $cpu 2>/dev/null \|\| true |
| `os/system/optimization-toggles.sh` | 59 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/system/optimization-toggles.sh` | 120 | echo $scheduler > $disk 2>/dev/null \|\| true |
| `os/system/parc-gamemode-hook.sh` | 20 | hook_log() { echo "$(date '+%F %T') $*" >> "$LOG" 2>/dev/nul |
| `os/system/power-manager.sh` | 141 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 156 | echo deadline > $disk 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 161 | echo on > $dev 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 170 | echo schedutil > $cpu 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 185 | echo cfq > $disk 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 190 | echo auto > $dev 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 199 | echo powersave > $cpu 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 214 | echo cfq > $disk 2>/dev/null \|\| true |
| `os/system/power-manager.sh` | 219 | echo auto > $dev 2>/dev/null \|\| true |
| `os/system/predictive-caching.sh` | 144 | cp "$file" "$CACHE_DIR/" 2>/dev/null \|\| true |
| `os/system/predictive-caching.sh` | 190 | find "$CACHE_DIR" -type f -mmin +60 -delete 2>/dev/null \|\| t |
| `os/system/predictive-intelligence.sh` | 68 | " 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 28 | cp -r /etc/apt "$snapshot_dir/" 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 29 | cp /etc/fstab "$snapshot_dir/" 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 30 | cp /etc/hostname "$snapshot_dir/" 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 33 | cp /etc/default/grub "$snapshot_dir/" 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 39 | .bashrc .profile .config 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 82 | tar -xzf "$snapshot_dir/home-backup/configs.tar.gz" -C "$HOM |
| `os/system/rollback-recovery.sh` | 139 | sudo grub-install /dev/sda 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 140 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 148 | sudo fsck -f /dev/sda1 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 252 | kill -15 "$heavy_proc" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 256 | swapoff -a && swapon -a 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 266 | find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 269 | journalctl --vacuum-time=3d 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 272 | apt clean 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 273 | pacman -Sc --noconfirm 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 276 | find /home -name "*.snap" -mtime +30 -delete 2>/dev/null \|\|  |
| `os/system/self-healing.sh` | 290 | renice +10 "$heavy_proc" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 296 | kill -15 "$heavy_proc" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 311 | ifdown "$interface" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 313 | ifup "$interface" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 317 | systemd-resolve --flush-caches 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 331 | systemctl restart "$service" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 349 | kill -SIGCHLD "$parent" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 368 | kill -9 "$proc" 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 390 | sysctl -p 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 399 | sysctl -p 2>/dev/null \|\| true |
| `os/system/self-healing.sh` | 404 | sysctl -p 2>/dev/null \|\| true |
| `os/system/temporal-mapping.sh` | 130 | echo performance > $cpu 2>/dev/null \|\| true |
| `os/system/temporal-mapping.sh` | 135 | echo ondemand > $cpu 2>/dev/null \|\| true |
| `os/system/temporal-resource-mapping.sh` | 71 | " 2>/dev/null \|\| true |
| `os/system/update-system.sh` | 155 | cp -r /etc/tinker $backup_dir/ 2>/dev/null \|\| true |
| `os/system/update-system.sh` | 156 | cp /etc/fstab $backup_dir/ 2>/dev/null \|\| true |
| `os/system/update-system.sh` | 174 | find /var/backup/tinker -maxdepth 1 -type d -mtime +30 -exec |
| `os/system/update-system.sh` | 206 | sudo cp -r $backup_dir/tinker/* /etc/tinker/ 2>/dev/null \|\|  |
| `os/system/driver-manager/korrinos-drivers.sh` | 270 | sudo add-apt-repository -y ppa:graphics-drivers/ppa 2>/dev/n |
| `os/system/driver-manager/korrinos-drivers.sh` | 286 | sudo systemctl stop gdm3 2>/dev/null \|\| sudo systemctl stop  |
| `os/system/driver-manager/korrinos-drivers.sh` | 298 | sudo apt-get install -y nvidia-cuda-toolkit nvidia-cudnn 2>/ |
| `os/system/driver-manager/korrinos-drivers.sh` | 309 | sudo apt-get install -y vulkan-tools libvulkan1 mesa-vulkan- |
| `os/system/driver-manager/korrinos-drivers.sh` | 318 | sudo apt-get install -y libnvidia-encode-535 libnvidia-decod |
| `os/system/driver-manager/korrinos-drivers.sh` | 337 | sudo systemctl start gdm3 2>/dev/null \|\| sudo systemctl star |
| `os/system/driver-manager/korrinos-drivers.sh` | 368 | mesa-va-drivers mesa-vdpau-drivers 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 371 | sudo apt-get install -y vulkan-tools libvulkan1 2>/dev/null  |
| `os/system/driver-manager/korrinos-drivers.sh` | 419 | libgl1-mesa-dri libgl1-mesa-glx 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 454 | firmware-brcm80211 firmware-misc-nonfree 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 487 | sudo modprobe -r iwlwifi 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 488 | sudo modprobe iwlwifi 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 489 | sudo modprobe ath9k 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 490 | sudo modprobe rtl8xxxu 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 499 | sudo apt-get install -y fprintd libpam-fprintd 2>/dev/null \| |
| `os/system/driver-manager/korrinos-drivers.sh` | 504 | sudo apt-get install -y python3-validity 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 505 | sudo systemctl enable --now open-fprintd 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 506 | sudo systemctl enable --now python3-validity 2>/dev/null \|\|  |
| `os/system/driver-manager/korrinos-drivers.sh` | 512 | sudo apt-get install -y python3-goodix 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 520 | fprintd-enroll "$USER" 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 530 | sudo apt-get install -y bluez blueman pulseaudio-module-blue |
| `os/system/driver-manager/korrinos-drivers.sh` | 531 | sudo systemctl enable bluetooth 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 532 | sudo systemctl start bluetooth 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 540 | sudo apt-get install -y cups cups-client printer-driver-gute |
| `os/system/driver-manager/korrinos-drivers.sh` | 541 | sudo systemctl enable cups 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 546 | printer-driver-escpos printer-driver-brlaser 2>/dev/null \|\|  |
| `os/system/driver-manager/korrinos-drivers.sh` | 551 | sudo apt-get install -y hplip hplip-gui 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 591 | sudo prime-select intel 2>/dev/null \|\| sudo prime-select off |
| `os/system/driver-manager/korrinos-drivers.sh` | 592 | sudo rm -f /etc/X11/xorg.conf.d/20-nvidia.conf 2>/dev/null \| |
| `os/system/driver-manager/korrinos-drivers.sh` | 596 | sudo prime-select on-demand 2>/dev/null \|\| sudo prime-select |
| `os/system/driver-manager/korrinos-drivers.sh` | 660 | dkms status > "$backup_path/dkms.txt" 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 682 | awk '{print $2}' "$backup_path/packages.txt" \| xargs sudo ap |
| `os/system/backup/korrinos-backup.sh` | 148 | "Full backup complete: $size in ${duration}s" 2>/dev/null \|\| |
| `os/system/package-manager/korrinos-pkg.sh` | 118 | sudo dnf check-update -y 2>&1 \| tail -5 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 137 | snap refresh "$s" 2>/dev/null \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 143 | flatpak update --appstream 2>/dev/null \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 199 | sudo snap refresh 2>&1 \| tail -5 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 204 | flatpak update -y 2>&1 \| tail -5 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 209 | apt)    sudo apt-get autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 210 | dnf)    sudo dnf autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 221 | cat /var/run/reboot-required 2>/dev/null \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 539 | dnf check-update 2>/dev/null \| tail -30 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 602 | grep -v "^$pkg\|" "$PKG_DB" > "$PKG_DB.tmp" 2>/dev/null \|\| tr |
| `os/system/package-manager/korrinos-pkg.sh` | 603 | mv "$PKG_DB.tmp" "$PKG_DB" 2>/dev/null \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 687 | echo "$to_remove" \| xargs sudo apt-get remove -y 2>/dev/null |
| `os/system/package-manager/korrinos-pkg.sh` | 694 | sudo pacman -S --noconfirm $(cat "$snap_dir/packages.txt" \|  |
| `os/system/appstore/korrinos-appstore.sh` | 141 | cp "$tmpdir/repodata.json" "$APP_CACHE/${name}_repodata.json |
| `os/system/appstore/korrinos-appstore.sh` | 174 | " 2>/dev/null \|\| true |
| `os/system/appstore/korrinos-appstore.sh` | 376 | sudo apt-get install -y "$dep" 2>/dev/null \|\| true |
| `os/system/appstore/korrinos-appstore.sh` | 398 | gpg --import /usr/share/korrinos/keys/repo-signing.gpg 2>/de |
| `os/system/appstore/korrinos-appstore.sh` | 429 | flatpak install -y flathub "$app_name" 2>/dev/null \|\| true |
| `os/system/appstore/korrinos-appstore.sh` | 432 | sudo snap install "$app_name" 2>/dev/null \|\| true |
| `os/system/appstore/korrinos-appstore.sh` | 501 | rm -f "$HOME/.local/share/applications/korrinos-${app_name}. |
| `os/system/appstore/korrinos-appstore.sh` | 767 | gpg --default-key "$gpg_key" --sign "$pkg_file" 2>/dev/null  |
| `os/system/monitor/korrinos-process.sh` | 209 | kill -SIGCHLD "$ppid" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 24 | sed -i '1a set -euo pipefail' "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 36 | sed -i 's/for $var/for "$var"/g' "$script" 2>/dev/null \|\| tr |
| `os/system/security/korrinos-bugfix.sh` | 37 | sed -i 's/\[ $var/\[ "$var"/g' "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 77 | }' "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 89 | sed -i 's/\$(command -v \([^)]*\))/$(command -v \1 2>/dev/nu |
| `os/system/security/korrinos-bugfix.sh` | 90 | sed -i 's/\$(which \([^)]*\))/$(which \1 2>/dev/null)/g' "$s |
| `os/system/security/korrinos-bugfix.sh` | 115 | sed -i 's\|> /tmp/korrinos-\([^ ]*\)\|> "$(mktemp /tmp/korrino |
| `os/system/security/korrinos-bugfix.sh` | 157 | sed -i 's/curl -s/curl -s --connect-timeout 10 --max-time 60 |
| `os/system/security/korrinos-bugfix.sh` | 160 | sed -i 's/wget /wget --timeout=30 --tries=3 /g' "$script" 2> |
| `os/system/security/korrinos-bugfix.sh` | 184 | }' "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 199 | sed -i 's/python3 -c "/python3 -c "/g' "$script" 2>/dev/null |
| `os/system/security/korrinos-bugfix.sh` | 210 | sed -i 's/^  adb /  command -v adb \&>\/dev/null \&\& adb /g |
| `os/system/security/korrinos-bugfix.sh` | 221 | sed -i 's/echo "ERROR:/echo "ERROR:/g' "$script" 2>/dev/null |
| `os/system/security/korrinos-bugfix.sh` | 248 | sed -i '/^trap.*EXIT/i trap "echo Cleaning up...; exit 1" SI |
| `os/system/security/korrinos-bugfix.sh` | 272 | grep -c "<<\s*['\"]" "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 284 | sed -i '1a export LC_ALL=C.UTF-8 2>/dev/null \|\| true' "$scri |
| `os/system/security/korrinos-bugfix.sh` | 310 | sed -i 's/scp /scp -o ConnectTimeout=10 -o ServerAliveInterv |
| `os/system/security/korrinos-bugfix.sh` | 311 | sed -i 's/ssh /ssh -o ConnectTimeout=10 -o ServerAliveInterv |
| `os/system/security/korrinos-bugfix.sh` | 321 | sed -i 's/adb devices/adb devices 2>/dev/null/g' "$script" 2 |
| `os/system/security/korrinos-bugfix.sh` | 346 | sed -i '/bluetooth.*install/a\\n  # Recover Bluetooth if nee |
| `os/system/security/korrinos-bugfix.sh` | 370 | chmod 755 "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 380 | find "$logfile" -name "*.log" -size +10M -exec truncate -s 1 |
| `os/system/security/korrinos-bugfix.sh` | 404 | cp "$config" "${config}.bak" 2>/dev/null \|\| true |
| `os/system/security/korrinos-health.sh` | 401 | "System health is ${total_score}/100. Check korrinos-health. |
| `os/system/cloud-sync/korrinos-cloud.sh` | 271 | [ "$remote_dir" != "." ] && curl -s -u "$user:$pass" -X MKCO |
| `os/system/firewall/korrinos-firewall.sh` | 252 | sudo ufw delete deny "$port/$proto" 2>/dev/null \|\| true |
| `os/system/firewall/korrinos-firewall.sh` | 326 | ufw)    sudo ufw delete deny from "$ip" 2>/dev/null \|\| true  |
| `os/system/hardware-cert/korrinos-cert.sh` | 454 | " 2>/dev/null \|\| true |
| `os/system/hardware-cert/korrinos-cert.sh` | 873 | init)            init_drivers 2>/dev/null \|\| true; echo "Har |
| `os/system/desktop-env/korrinos-desktop.sh` | 225 | xfce4-panel --quit 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 226 | pkill xfce4-panel 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 369 | update-desktop-database "$launcher_dir" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 451 | pkill dunst 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 566 | pkill picom 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 585 | xfconf-query -c xfwm4 -p /general/titlebar_layout -s "CHM" 2 |
| `os/system/desktop-env/korrinos-desktop.sh` | 586 | xfconf-query -c xfwm4 -p /general/title_font -s "Ubuntu Sans |
| `os/system/desktop-env/korrinos-desktop.sh` | 587 | xfconf-query -c xfwm4 -p /general/title_alignment -s "left"  |
| `os/system/desktop-env/korrinos-desktop.sh` | 588 | xfconf-query -c xfwm4 -p /general/title_horizontal_offset -s |
| `os/system/desktop-env/korrinos-desktop.sh` | 589 | xfconf-query -c xfwm4 -p /general/title_vertical_offset -s 2 |
| `os/system/desktop-env/korrinos-desktop.sh` | 592 | xfconf-query -c xfwm4 -p /general/click_to_focus -s true 2>/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 593 | xfconf-query -c xfwm4 -p /general/focus_new -s true 2>/dev/n |
| `os/system/desktop-env/korrinos-desktop.sh` | 594 | xfconf-query -c xfwm4 -p /general/activate_action -s "bring" |
| `os/system/desktop-env/korrinos-desktop.sh` | 595 | xfconf-query -c xfwm4 -p /general/focus_delay -s 200 2>/dev/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 598 | xfconf-query -c xfwm4 -p /general/snap_to_border -s true 2>/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 599 | xfconf-query -c xfwm4 -p /general/snap_to_windows -s true 2> |
| `os/system/desktop-env/korrinos-desktop.sh` | 600 | xfconf-query -c xfwm4 -p /general/snap_width_maximized -s 16 |
| `os/system/desktop-env/korrinos-desktop.sh` | 601 | xfconf-query -c xfwm4 -p /general/snap_width_moving -s 16 2> |
| `os/system/desktop-env/korrinos-desktop.sh` | 602 | xfconf-query -c xfwm4 -p /general/edge_tiling -s true 2>/dev |
| `os/system/desktop-env/korrinos-desktop.sh` | 605 | xfconf-query -c xfwm4 -p /general/use_compositing -s true 2> |
| `os/system/desktop-env/korrinos-desktop.sh` | 606 | xfconf-query -c xfwm4 -p /general/show_frame_shadow -s true  |
| `os/system/desktop-env/korrinos-desktop.sh` | 607 | xfconf-query -c xfwm4 -p /general/show_popup_shadow -s false |
| `os/system/desktop-env/korrinos-desktop.sh` | 610 | xfconf-query -c xfwm4 -p /general/placement_ratio -s 50 2>/d |
| `os/system/desktop-env/korrinos-desktop.sh` | 611 | xfconf-query -c xfwm4 -p /general/placement -s "center" 2>/d |
| `os/system/desktop-env/korrinos-desktop.sh` | 614 | xfconf-query -c xfwm4 -p /general/box_move -s true 2>/dev/nu |
| `os/system/desktop-env/korrinos-desktop.sh` | 615 | xfconf-query -c xfwm4 -p /general/box_resize -s true 2>/dev/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 616 | xfconf-query -c xfwm4 -p /general/raise_on_focus -s true 2>/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 617 | xfconf-query -c xfwm4 -p /general/raise_delay -s 250 2>/dev/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 620 | xfconf-query -c xfwm4 -p /general/button_layout -s "CHM" 2>/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 621 | xfconf-query -c xfwm4 -p /general/button_offset -s 3 2>/dev/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 622 | xfconf-query -c xfwm4 -p /general/button_spacing -s 4 2>/dev |
| `os/system/desktop-env/korrinos-desktop.sh` | 665 | -n -t string -s "$keyval" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 686 | -s "$wallpaper" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 688 | -s 5 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1133 | xfconf-query -c xsettings -p /Net/ThemeName -s "KorrinOS-Dar |
| `os/system/desktop-env/korrinos-desktop.sh` | 1134 | xfconf-query -c xsettings -p /Net/IconThemeName -s "KorrinOS |
| `os/system/desktop-env/korrinos-desktop.sh` | 1135 | xfconf-query -c xsettings -p /Gtk/FontName -s "Ubuntu Sans 1 |
| `os/system/desktop-env/korrinos-desktop.sh` | 1136 | xfconf-query -c xsettings -p /Gtk/MonospaceFontName -s "JetB |
| `os/system/desktop-env/korrinos-desktop.sh` | 1184 | top-right) xfce4-panel --toggle-window panel-2 2>/dev/null \| |
| `os/system/desktop-env/korrinos-desktop.sh` | 1185 | bottom-left) xfce4-send-keys --name show-desktop 2>/dev/null |
| `os/system/desktop-env/korrinos-desktop.sh` | 1212 | xfconf-query -c xfce4-workspaces -p /general/workspace-count |
| `os/system/desktop-env/korrinos-desktop.sh` | 1217 | xfconf-query -c xfce4-workspaces -p "/workspace-$((i+1))/nam |
| `os/system/desktop-env/korrinos-desktop.sh` | 1220 | xfconf-query -c xfce4-workspaces -p /general/wrap-workspaces |
| `os/system/desktop-env/korrinos-desktop.sh` | 1268 | chmod +x /usr/local/bin/korrinos-screenshot 2>/dev/null \|\| t |
| `os/system/desktop-env/korrinos-desktop.sh` | 1323 | chmod +x /usr/local/bin/korrinos-clip 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1336 | xfconf-query -c xfce4-screensaver -p /saver/enabled -s true  |
| `os/system/desktop-env/korrinos-desktop.sh` | 1337 | xfconf-query -c xfce4-screensaver -p /saver/timeout -s 600 2 |
| `os/system/desktop-env/korrinos-desktop.sh` | 1338 | xfconf-query -c xfce4-screensaver -p /lock/enabled -s false  |
| `os/system/desktop-env/korrinos-desktop.sh` | 1343 | xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 1344 | xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 1345 | xfconf-query -c xfce4-power-manager -p /xfce4-power-manager/ |
| `os/system/desktop-env/korrinos-desktop.sh` | 1376 | xrandr --output "$primary" --primary --auto --output "$secon |
| `os/system/desktop-env/korrinos-desktop.sh` | 1392 | xfconf-query -c xsettings -p /Net/ThemeName -s "KorrinOS-Dar |
| `os/system/desktop-env/korrinos-desktop.sh` | 1395 | xfconf-query -c xsettings -p /Gtk/CursorThemeSize -n -t int  |
| `os/system/desktop-env/korrinos-desktop.sh` | 1398 | xfconf-query -c xsettings -p /Xft/DPI -n -t int -s 96 2>/dev |
| `os/system/desktop-env/korrinos-desktop.sh` | 1495 | chmod +x "$icon_dir"/*.desktop 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1593 | EOF" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1607 | sudo mkdir -p "$grub_theme_dir" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1659 | sudo mkdir -p "$plymouth_dir" 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-desktop.sh` | 1837 | xfconf-query -c xsettings -p /Net/ThemeName -s "KorrinOS-Lig |
| `os/system/desktop-env/korrinos-desktop.sh` | 1843 | xfconf-query -c xsettings -p /Net/ThemeName -s "KorrinOS-Dar |
| `os/system/desktop-env/korrinos-desktop.sh` | 1844 | xfconf-query -c xsettings -p /Net/IconThemeName -s "KorrinOS |
| `os/system/desktop-env/korrinos-notify.sh` | 172 | killall dunst 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 140 | cp "$UPDATE_CONFIG" "$snap_dir/config.json" 2>/dev/null \|\| t |
| `os/system/update-system/korrinos-update.sh` | 146 | cat /etc/default/grub 2>/dev/null > "$snap_dir/grub-default. |
| `os/system/update-system/korrinos-update.sh` | 149 | [ -f /etc/apt/sources.list ] && cp /etc/apt/sources.list "$s |
| `os/system/update-system/korrinos-update.sh` | 155 | sudo btrfs subvolume snapshot / "/.snapshots/$snap_id" 2>/de |
| `os/system/update-system/korrinos-update.sh` | 198 | create_snapshot > /dev/null 2>&1 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 213 | echo "$to_remove" \| xargs sudo apt-get remove -y 2>/dev/null |
| `os/system/update-system/korrinos-update.sh` | 221 | echo "$to_install" \| xargs sudo apt-get install -y 2>/dev/nu |
| `os/system/update-system/korrinos-update.sh` | 231 | sudo pacman -S --noconfirm $saved 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 239 | sudo $(if [ "$mgr" = "dnf" ]; then echo "dnf install -y"; el |
| `os/system/update-system/korrinos-update.sh` | 246 | sudo cp "$snap_dir/grub-default.txt" /etc/default/grub 2>/de |
| `os/system/update-system/korrinos-update.sh` | 247 | sudo update-grub 2>/dev/null \|\| sudo grub2-mkconfig -o /boot |
| `os/system/update-system/korrinos-update.sh` | 313 | "$pre_script" 2>&1 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 330 | sudo dnf check-update -y 2>&1 \| tail -5 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 417 | flatpak update -y 2>&1 \| tail -5 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 428 | sudo snap refresh 2>&1 \| tail -5 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 449 | sudo apt-get install -y linux-image-generic linux-headers-ge |
| `os/system/update-system/korrinos-update.sh` | 476 | apt)    sudo apt-get autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 477 | dnf)    sudo dnf autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 478 | pacman) sudo pacman -Sc --noconfirm 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 484 | apt)    sudo apt-get clean 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 485 | dnf)    sudo dnf clean all 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 502 | "Update complete in ${duration}s.\nMode: $mode\nSnapshot: $s |
| `os/system/update-system/korrinos-update.sh` | 505 | paplay /usr/share/sounds/freedesktop/stereo/complete.oga 2>/ |
| `os/system/update-system/korrinos-update.sh` | 513 | cat /var/run/reboot-required 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 517 | "System packages updated. Please reboot." 2>/dev/null \|\| tru |
| `os/system/update-system/korrinos-update.sh` | 535 | "$post_script" 2>&1 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 565 | dnf check-update 2>/dev/null \| tail -20 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 653 | sudo systemctl stop korrinos-autoupdate.timer 2>/dev/null \|\| |
| `os/system/update-system/korrinos-update.sh` | 654 | sudo systemctl disable korrinos-autoupdate.timer 2>/dev/null |
| `os/system/update-system/korrinos-update.sh` | 718 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 735 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 738 | sudo pacman -R --noconfirm "linux-$target" 2>/dev/null \|\| tr |
| `os/system/update-system/korrinos-update.sh` | 813 | sudo apt-get download $(apt list --upgradable 2>/dev/null \|  |
| `os/system/update-system/korrinos-update.sh` | 816 | sudo dnf download --resolve $(dnf check-update 2>/dev/null \| |
| `os/system/installer/korrinos-installer.sh` | 139 | umount "${device}"* 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 140 | swapoff "${device}"* 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 144 | wipefs -a "$device" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 145 | dd if=/dev/zero of="$device" bs=1M count=100 2>/dev/null \|\|  |
| `os/system/installer/korrinos-installer.sh` | 240 | umount -R "$TARGET" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 267 | swapon /tmp/korrinos-swap 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 317 | mount --bind /dev "$TARGET/dev" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 318 | mount --bind /dev/pts "$TARGET/dev/pts" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 319 | mount --bind /proc "$TARGET/proc" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 320 | mount --bind /sys "$TARGET/sys" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 376 | chroot "$TARGET" useradd -m -s /bin/bash -c "KorrinOS User"  |
| `os/system/installer/korrinos-installer.sh` | 377 | chroot "$TARGET" usermod -aG sudo,docker,video,audio,plugdev |
| `os/system/installer/korrinos-installer.sh` | 394 | mkdir -p "$TARGET/etc/sddm.conf.d" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 406 | chroot "$TARGET" ufw --force enable 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 410 | chroot "$TARGET" systemctl enable NetworkManager 2>/dev/null |
| `os/system/installer/korrinos-installer.sh` | 411 | chroot "$TARGET" systemctl enable bluetooth 2>/dev/null \|\| t |
| `os/system/installer/korrinos-installer.sh` | 412 | chroot "$TARGET" systemctl enable sddm 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 413 | chroot "$TARGET" systemctl enable cups 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 416 | chroot "$TARGET" systemctl enable korrinos-autoupdate.timer  |
| `os/system/installer/korrinos-installer.sh` | 419 | umount -R "$TARGET/dev" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 420 | umount -R "$TARGET/proc" 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 421 | umount -R "$TARGET/sys" 2>/dev/null \|\| true |
| `os/system/mobile-companion/korrinos-mobile.sh` | 104 | adb start-server 2>/dev/null \|\| true |
| `os/system/mobile-companion/korrinos-mobile.sh` | 337 | adb shell am broadcast -a clipboardmanager --es text "$curre |
| `os/system/mobile-companion/korrinos-mobile.sh` | 371 | adb $args shell mkdir -p /sdcard/KorrinOS/ 2>/dev/null \|\| tr |
| `os/system/mobile-companion/korrinos-mobile.sh` | 378 | --es filename "$(basename "$file")" 2>/dev/null \|\| true |
| `os/system/mobile-companion/korrinos-mobile.sh` | 441 | "com.korrinos.notificationforwarder/.ListenerService" 2>/dev |
| `os/system/mobile-companion/korrinos-mobile.sh` | 488 | adb pull /sdcard/DCIM/Camera/ ~/Pictures/PhoneCamera/ 2>/dev |
| `os/system/mobile-companion/korrinos-mobile.sh` | 519 | kdeconnect-cli --list-devices 2>/dev/null \|\| true |
| `os/system/mobile-companion/korrinos-mobile.sh` | 641 | adb shell service call audio 10 i32 3 i32 1 i32 0 2>/dev/nul |
| `os/system/mobile-companion/korrinos-mobile.sh` | 774 | adb start-server 2>/dev/null \|\| true |
| `os/system/network/korrinos-network.sh` | 184 | sudo resolvconf -a <(echo "nameserver $s") 2>/dev/null \|\| tr |
| `os/system/network/korrinos-network.sh` | 190 | [ -n "$dev" ] && nmcli device modify "$dev" ipv4.dns "$serve |
| `os/system/enterprise/korrinos-enterprise.sh` | 142 | echo "korrinos enterprise: $action $detail" \| auditctl -w /e |
| `os/system/enterprise/korrinos-enterprise.sh` | 159 | samba-common-bin packagekit oddjob oddjob-mkhomedir 2>/dev/n |
| `os/system/enterprise/korrinos-enterprise.sh` | 226 | sudo pam-auth-update --enable mkhomedir 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 246 | sudo systemctl stop sssd 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 265 | sudo apt-get install -y libnss-ldap libpam-ldap ldap-utils s |
| `os/system/enterprise/korrinos-enterprise.sh` | 316 | sudo systemctl restart nslcd sssd 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 396 | gsettings set org.gnome.desktop.background picture-uri "file |
| `os/system/enterprise/korrinos-enterprise.sh` | 401 | gsettings set org.gnome.desktop.session idle-delay $((lock_t |
| `os/system/enterprise/korrinos-enterprise.sh` | 405 | echo "password requisite pam_pwquality.so minlen=$min_pwd_le |
| `os/system/enterprise/korrinos-enterprise.sh` | 479 | sudo apt-get install -y libpam-google-authenticator 2>/dev/n |
| `os/system/enterprise/korrinos-enterprise.sh` | 486 | google-authenticator -u -t -d -i "KorrinOS HOTP" -H "$userna |
| `os/system/enterprise/korrinos-enterprise.sh` | 499 | sudo apt-get install -y libpam-u2f 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 605 | sudo useradd -m -s /bin/bash "$username" 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 622 | sudo apt-get install -y wireguard 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 633 | sudo apt-get install -y openvpn 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 642 | sudo apt-get install -y strongswan 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 655 | echo "# Split tunnel routes" \| sudo tee -a /etc/wireguard/wg |
| `os/system/enterprise/korrinos-enterprise.sh` | 756 | [ -n "$grp" ] && sudo usermod -aG "$grp" "$username" 2>/dev/ |
| `os/system/enterprise/korrinos-enterprise.sh` | 799 | sudo gpasswd -d "$username" "$groupname" 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 840 | sudo sed -i "s/^PASS_MAX_DAYS.*/PASS_MAX_DAYS   $max_age/" / |
| `os/system/enterprise/korrinos-enterprise.sh` | 841 | sudo sed -i "s/^PASS_MIN_DAYS.*/PASS_MIN_DAYS   1/" /etc/log |
| `os/system/enterprise/korrinos-enterprise.sh` | 842 | sudo sed -i "s/^PASS_WARN_AGE.*/PASS_WARN_AGE   14/" /etc/lo |
| `os/territories/cue-watchdog.sh` | 36 | /tmp/opencode/tiocsti "$tty" "cue on" >/dev/null 2>&1 \|\| tru |
| `os/territories/cue-watchdog.sh` | 40 | printf '\n\x07\x07\x07CUE ON — %s — next real tool call now  |
| `os/territories/cue-watchdog.sh` | 44 | wall -n "CUE ON — $(date -Iseconds) — KorrinOS agent: next r |
| `os/territories/cue-watchdog.sh` | 60 | sed -i 's/^## SELF-WATCHDOG (READ THIS EVERY SESSION).*/## S |
| `os/territories/cue-watchdog.sh` | 63 | notify_ui 2>/dev/null \|\| true |
| `os/territories/self-watchdog.sh` | 60 | sed -i 's/^## SELF-WATCHDOG (READ THIS EVERY SESSION)/## SEL |
| `os/territories/self-watchdog.sh` | 78 | sed -i 's/  <-- STALL INJECTED .*$/  /' "$AGENTS_FILE" 2>/de |
| `os/territories/world-optimizer.sh` | 16 | need_root \|\| true |
| `os/territories/world-optimizer.sh` | 23 | [ -w "$g" ] && echo "$1" > "$g" 2>/dev/null \|\| true |
| `os/territories/world-optimizer.sh` | 30 | sysctl -w -q vm.swappiness=10 kernel.randomize_va_space=2 ke |
| `os/territories/world-optimizer.sh` | 37 | sysctl -w -q vm.swappiness=0 kernel.randomize_va_space=2 2>/ |
| `os/territories/world-optimizer.sh` | 50 | kernel.nmi_watchdog=0 2>/dev/null \|\| true |
| `os/territories/world-optimizer.sh` | 54 | renice -n -10 -p $$ 2>/dev/null \|\| true |
| `os/territories/world-optimizer.sh` | 70 | sysctl -w -q vm.swappiness=20 2>/dev/null \|\| true |
| `os/territories/secure/duress-alert.sh` | 44 | lock) loginctl lock-session 2>/dev/null \|\| true ;; |
| `os/territories/secure/duress-alert.sh` | 45 | wipe) "$(dirname "${BASH_SOURCE[0]}")/../hack/panic-wipe.sh" |
| `os/territories/secure/duress-alert.sh` | 47 | poweroff) sudo systemctl poweroff 2>/dev/null \|\| sudo powero |
| `os/territories/secure/duress-alert.sh` | 48 | notify:*) local url="${act#notify:}"; curl -s --max-time 5 " |
| `os/territories/secure/secure-mode.sh` | 25 | "$TERR_ROOT/secure/vault-engine.sh" open 2>/dev/null \|\| true |
| `os/territories/secure/secure-mode.sh` | 57 | "$TERR_ROOT/secure/zero-trust-config.sh" apply 2>/dev/null \| |
| `os/territories/secure/sip-guard.sh` | 28 | mount -o remount,rw,bind "$d" 2>/dev/null \|\| true |
| `os/territories/secure/sip-guard.sh` | 44 | find /etc /usr -xdev -perm -0002 -type f 2>/dev/null \| head  |
| `os/territories/secure/traffic-guard.sh` | 21 | sudo timeout 5 nethogs -c 3 2>/dev/null \| head -20 \|\| true |
| `os/territories/secure/vault-engine.sh` | 41 | sudo umount "$MOUNT_BASE/$name" 2>/dev/null \|\| true |
| `os/territories/secure/vault-engine.sh` | 42 | sudo cryptsetup close "tinker_$name" 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 25 | "$T/app-allowlist.sh" init 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 26 | "$T/app-allowlist.sh" enforce 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 32 | "$T/sip-guard.sh" audit 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 36 | "$T/canary-monitor.sh" plant 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 37 | "$T/canary-monitor.sh" baseline 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 52 | "$T/canary-monitor.sh" check 2>/dev/null \|\| true |
| `os/territories/secure/zero-trust-config.sh` | 53 | "$H/threat-monitor.sh" threats 2>/dev/null \| head -20 \|\| tru |
| `os/territories/vibe-address/vibe-address.sh` | 85 | sed -i "s/^$key=.*/$key=$((n+1))/" "$dir/meta.conf" 2>/dev/n |
| `os/territories/vibe-address/core/action.sh` | 91 | done <<< "$(echo "$ranked")" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/action.sh` | 134 | find "$VIBE_EVENTS" -name '*.log' -type f -exec sed -i "/\|$f |
| `os/territories/vibe-address/core/action.sh` | 136 | find "$VIBE_TREE" -name index -type f -exec sed -i "/ $fp$/d |
| `os/territories/vibe-address/core/action.sh` | 138 | find "$VIBE_INDEX/inv" -type f -exec sed -i "/ ${fp} /d" {}  |
| `os/territories/vibe-address/core/action.sh` | 139 | find "$VIBE_INDEX/inv" -type f -exec sed -i "/^${fp} /d" {}  |
| `os/territories/vibe-address/core/action.sh` | 141 | find "$VIBE_INDEX/time" -type f -exec sed -i "/ ${fp}$/d" {} |
| `os/territories/vibe-address/core/action.sh` | 143 | rm -f "$VIBE_INDEX/fp/$fp" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/action.sh` | 144 | sed -i "/^$fp /d" "$VIBE_INDEX/dedup.hash" 2>/dev/null \|\| tr |
| `os/territories/vibe-address/core/adapt.sh` | 36 | grep -v "^$vtype " "$tf" 2>/dev/null > "$tf.tmp" \|\| true |
| `os/territories/vibe-address/core/adapt.sh` | 44 | grep -v "^$top1 " "$cf" 2>/dev/null > "$cf.tmp" \|\| true |
| `os/territories/vibe-address/core/adapt.sh` | 52 | grep -v "^$depth " "$df" 2>/dev/null > "$df.tmp" \|\| true |
| `os/territories/vibe-address/core/adapt.sh` | 62 | local qc; qc=$(cat "$sf" 2>/dev/null \| grep -c . 2>/dev/null |
| `os/territories/vibe-address/core/adapt.sh` | 88 | local with_cat; with_cat=$(echo "$window" \| grep -c '\|yes\|'  |
| `os/territories/vibe-address/core/adapt.sh` | 89 | local with_time; with_time=$(echo "$window" \| grep -c '\|yes\| |
| `os/territories/vibe-address/core/align.sh` | 22 | srctokens=$(ve_ingest_source_tokens "$source") \|\| true |
| `os/territories/vibe-address/core/align.sh` | 23 | typetokens=$(ve_ingest_type_tokens "$vtype") \|\| true |
| `os/territories/vibe-address/core/align.sh` | 42 | srctokens=$(ve_ingest_source_tokens "$source") \|\| true |
| `os/territories/vibe-address/core/align.sh` | 43 | typetokens=$(ve_ingest_type_tokens "$vtype") \|\| true |
| `os/territories/vibe-address/core/align.sh` | 44 | ve_markov_observe "$(printf '%s %s %s %s' "$nametokens" "$sr |
| `os/territories/vibe-address/core/align.sh` | 45 | ve_lsh_index "$fp" "$(printf '%s %s %s' "$nametokens" "$srct |
| `os/territories/vibe-address/core/align.sh` | 162 | [ "$dry" = 0 ] && { shift; "$@" 2>&1 \| sed 's/^/    /' \|\| tr |
| `os/territories/vibe-address/core/align.sh` | 175 | rm -rf "$VIBE_STATE/markov" "$(ve_lsh_dir)" 2>/dev/null \|\| t |
| `os/territories/vibe-address/core/audit.sh` | 47 | n=$(ve_ir_df "$t" 2>/dev/null \|\| true) |
| `os/territories/vibe-address/core/audit.sh` | 54 | local dlen; dlen=$(cat "$(ve_ir_dlen_f "$fp")" 2>/dev/null \| |
| `os/territories/vibe-address/core/audit.sh` | 58 | local wr; wr=$(grep -F "\|$fp\|" "$VIBE_STATE/sarray/owners" 2 |
| `os/territories/vibe-address/core/audit.sh` | 74 | local sig; sig=$(cat "$(ve_lsh_sigdir)/$fp" 2>/dev/null \|\| t |
| `os/territories/vibe-address/core/audit.sh` | 79 | [ -n "$pk" ] && echo "    estimate  : count(token='$pk') ≈ $ |
| `os/territories/vibe-address/core/audit.sh` | 82 | ve_retention_standings "$fp" 2>/dev/null \| sed 's/^/    /' \| |
| `os/territories/vibe-address/core/audit.sh` | 95 | pos_raw=$(echo "$ops" \| grep '^POS=' \| cut -d= -f2- \|\| true) |
| `os/territories/vibe-address/core/audit.sh` | 96 | qneg_raw=$(echo "$ops" \| grep '^NEG=' \| cut -d= -f2- \|\| true |
| `os/territories/vibe-address/core/audit.sh` | 103 | intent_time=$(echo "$intent" \| grep -oP 'TIME=\K\S+' \|\| true |
| `os/territories/vibe-address/core/audit.sh` | 104 | intent_cat=$(echo "$intent" \| grep -oP 'CAT=\K\S+' \|\| true) |
| `os/territories/vibe-address/core/audit.sh` | 105 | intent_src=$(echo "$intent" \| grep -oP 'SOURCE=\K\S+' \|\| tru |
| `os/territories/vibe-address/core/bloom.sh` | 69 | grep "^$idx " "$map" 2>/dev/null \| awk '{print $2}' \| head - |
| `os/territories/vibe-address/core/bulk.sh` | 88 | { echo "$(date +%s)${fpstr// / $VIBE_BULK }" ; } >> /dev/nul |
| `os/territories/vibe-address/core/bulk.sh` | 167 | ' "$tmp/clean.tsv" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/index.sh` | 55 | ve_bloom_tok_add "$tfile" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/index.sh` | 103 | done <<< "$(echo "$tokens" \| tr ' ' '\n' \| sed '/^$/d')" > " |
| `os/territories/vibe-address/core/index.sh` | 114 | [ -f "$tb/$key" ] && cut -d' ' -f2 "$tb/$key" \|\| true |
| `os/territories/vibe-address/core/index.sh` | 152 | local srctokens;  srctokens=$(ve_ingest_source_tokens "$sour |
| `os/territories/vibe-address/core/index.sh` | 153 | local typetokens; typetokens=$(ve_ingest_type_tokens "$vtype |
| `os/territories/vibe-address/core/index.sh` | 164 | bucket_m=$(ve_time_epoch_bucket "$epoch" 1) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 165 | bucket_h=$(ve_time_epoch_bucket "$epoch" 2) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 166 | bucket_d=$(ve_time_epoch_bucket "$epoch" 3) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 167 | bucket_w=$(ve_time_epoch_bucket "$epoch" 4) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 168 | bucket_mo=$(ve_time_epoch_bucket "$epoch" 5) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 169 | bucket_q=$(ve_time_epoch_bucket "$epoch" 6) \|\| true |
| `os/territories/vibe-address/core/index.sh` | 170 | bucket_y=$(ve_time_epoch_bucket "$epoch" 7) \|\| true |
| `os/territories/vibe-address/core/ingest.sh` | 88 | ve_markov_observe "$(printf '%s %s %s %s' "$nametokens" "$sr |
| `os/territories/vibe-address/core/ingest.sh` | 91 | ve_lsh_index "$fp" "$(printf '%s %s %s' "$nametokens" "$srct |
| `os/territories/vibe-address/core/ingest.sh` | 179 | ve_ingest_record NET download "$file" "$(md5sum "$file" 2>/d |
| `os/territories/vibe-address/core/ingest.sh` | 260 | picks=$("$vah" markov complete "$typed" 3 2>/dev/null \|\| tru |
| `os/territories/vibe-address/core/ingest.sh` | 268 | awk -F'[ ]' '{for(i=NF;i>=1;i--) if($i ~ /^[0-9]+%$/) {print |
| `os/territories/vibe-address/core/ingest.sh` | 282 | --column="" --column="Phrase" 2>/dev/null \| tail -1 \|\| true) |
| `os/territories/vibe-address/core/ingest.sh` | 289 | extended=$("$vah" markov chain "$typed" 2 2>/dev/null \|\| tru |
| `os/territories/vibe-address/core/ingest.sh` | 296 | "Extend: $extended" 2>/dev/null \| tail -1 \|\| true) |
| `os/territories/vibe-address/core/lexin.sh` | 90 | # whole query planner.  The trailing `\|\| true` neutralizes o |
| `os/territories/vibe-address/core/lexin.sh` | 95 | sort -u \| grep -v '^$' \| tr '\n' ' ' \| sed 's/ $//' \| sed 's |
| `os/territories/vibe-address/core/lexin.sh` | 207 | grep -Fx "$tok:$tok" "$ringfile" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/lsh.sh` | 189 | grep -v "\|$fp\|" "$f" 2>/dev/null > "$f.prune" \|\| true |
| `os/territories/vibe-address/core/markov.sh` | 34 | { [ -f "$file" ] && cat "$file"; } \|\| true |
| `os/territories/vibe-address/core/query.sh` | 216 | local intent_time; intent_time=$(echo "$intent" \| grep -oP ' |
| `os/territories/vibe-address/core/query.sh` | 217 | local intent_cat;  intent_cat=$(echo "$intent" \| grep -oP 'C |
| `os/territories/vibe-address/core/query.sh` | 218 | local intent_src;  intent_src=$(echo "$intent" \| grep -oP 'S |
| `os/territories/vibe-address/core/retention.sh` | 84 | zcat "$gz" 2>/dev/null \| ve_ingest_bulk > /dev/null 2>&1 \|\|  |
| `os/territories/vibe-address/core/retention.sh` | 86 | mv "$gz" "${gz%.gz}.done.gz" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/sarray.sh` | 21 | ve_sarray_fps() { ls "$VIBE_INDEX/fp" 2>/dev/null \|\| true; } |
| `os/territories/vibe-address/core/selftest.sh` | 44 | ve_tree_insert "ref-march" "projects:work:march" "$(date +%s |
| `os/territories/vibe-address/core/selftest.sh` | 69 | ve_bloom_tok_add "needle" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 75 | ve_ingest_record "File" "files" "$f" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 114 | local mchain; mchain=$(ve_markov_chain "meeting" 4 2>/dev/nu |
| `os/territories/vibe-address/core/selftest.sh` | 138 | local r; r=$(SEARCHIE_TERSE=1 ve_query_run "meeting notes" 2 |
| `os/territories/vibe-address/core/selftest.sh` | 147 | local staged; staged=$(ve_action_delete "the missing thing"  |
| `os/territories/vibe-address/core/selftest.sh` | 151 | ve_auto_compile "hpho" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 152 | local ac; ac=$(ve_auto_scan_text "beachphotojpg note" 2>/dev |
| `os/territories/vibe-address/core/selftest.sh` | 167 | ve_cms_add "needle" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 168 | ve_cms_add "needle" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 173 | ve_store_append "1780000000\|photo\|file\|/st/align_probe.txt\|1 |
| `os/territories/vibe-address/core/selftest.sh` | 174 | ve_align_fix >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 186 | ve_align_fix --dry-run >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 201 | ve_store_append "$t0\|${vtypes[$((rnd_i % 5))]}\|file\|/st/fuzz |
| `os/territories/vibe-address/core/selftest.sh` | 203 | ve_align_fix >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 206 | local fuzzq; fuzzq=$(ve_query_run "kilo" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 213 | local tl_out; tl_out=$(ve_store_timeline 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 220 | ve_store_append "1780000001\|photo\|file\|/st/align_probe.txt\|2 |
| `os/territories/vibe-address/core/selftest.sh` | 221 | ve_align_fix >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 227 | ve_index_rebuild >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/selftest.sh` | 230 | local qout; qout=$(ve_query_run "notes" 2>/dev/null \|\| true) |
| `os/territories/vibe-address/core/store.sh` | 37 | sha256sum "$logfile" > "$idir/$fname.sha256" 2>/dev/null \|\|  |
| `os/territories/vibe-address/core/store.sh` | 184 | ve_lsh_dupe_prune_burn >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/store.sh` | 198 | ve_lsh_sigdir >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/store.sh` | 212 | ve_lsh_index "$f" "$toks" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/core/tree.sh` | 50 | printf '%s %s %s\n' "$epoch" "$pathref" "$norm" >> "$cur/ind |
| `os/territories/vibe-address/core/tree.sh` | 121 | printf '%s\n' "$alias" >> "$d/_syn" 2>/dev/null \|\| true |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 299 | \|\| gsettings set org.gnome.settings-daemon.plugins.media-key |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 304 | "['$path']" >/dev/null 2>&1 \|\| true |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 305 | gsettings set "$scheme:$path/" name "KorrinOS Searchie" >/de |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 306 | gsettings set "$scheme:$path/" command "$cmd" >/dev/null 2>& |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 307 | gsettings set "$scheme:$path/" binding "<Control><Shift>spac |
| `os/territories/game/audio-focus.sh` | 16 | pactl set-sink-mute "$sink" 0 2>/dev/null \|\| true |
| `os/territories/game/audio-focus.sh` | 17 | pactl set-sink-volume "$sink" 100% 2>/dev/null \|\| true |
| `os/territories/game/audio-focus.sh` | 41 | pactl set-source-mute "$s" 1 2>/dev/null \|\| true |
| `os/territories/game/game-mode.sh` | 35 | echo performance > "$g" 2>/dev/null \|\| true |
| `os/territories/game/game-mode.sh` | 40 | renice -n -5 -p $$ 2>/dev/null \|\| true |
| `os/territories/game/game-replay-ai.sh` | 51 | ai=$(command -v vokk \|\| command -v vokk \|\| true) |
| `os/territories/game/latency-clean.sh` | 31 | pkill -f "$app" 2>/dev/null && echo "  closed $app" \|\| true |
| `os/territories/game/lowlat-input.sh` | 21 | 2>/dev/null \|\| true |
| `os/territories/game/lowlat-input.sh` | 37 | sysctl -w -q "kernel.nohz_full"= 2>/dev/null \|\| true |
| `os/territories/game/perf-tune.sh` | 23 | has dd && echo "Disk write (1G): $(dd if=/dev/zero of=/tmp/p |
| `os/territories/game/perf-tune.sh` | 24 | has smartctl && smartctl --health /dev/sda 2>/dev/null \| tai |
| `os/territories/game/perf-tune.sh` | 33 | echo performance > /sys/devices/system/cpu/cpu*/cpufreq/scal |
| `os/territories/game/perf-tune.sh` | 34 | sysctl -w vm.swappiness=20 >/dev/null 2>&1 \|\| true |
| `os/territories/game/perf-tune.sh` | 38 | echo powersave > /sys/devices/system/cpu/cpu*/cpufreq/scalin |
| `os/territories/game/perf-tune.sh` | 39 | sysctl -w vm.swappiness=10 >/dev/null 2>&1 \|\| true |
| `os/territories/game/perf-tune.sh` | 52 | echo noop > /sys/block/sda/queue/scheduler 2>/dev/null \|\| tr |
| `os/territories/game/perf-tune.sh` | 53 | sysctl -w vm.swappiness=0 >/dev/null 2>&1 \|\| true |
| `os/territories/hack/amnesia-firewall.sh` | 39 | nft -f - <<'NFT' 2>/dev/null \|\| true |
| `os/territories/hack/amnesia-firewall.sh` | 47 | nft add rule inet tinker_amnesia input  ct state established |
| `os/territories/hack/amnesia-firewall.sh` | 48 | nft add rule inet tinker_amnesia input  iif lo accept \|\| tru |
| `os/territories/hack/amnesia-firewall.sh` | 49 | nft add rule inet tinker_amnesia output ct state established |
| `os/territories/hack/amnesia-firewall.sh` | 50 | nft add rule inet tinker_amnesia output oif lo accept \|\| tru |
| `os/territories/hack/amnesia-firewall.sh` | 71 | in\|both) nft add rule inet tinker_amnesia input  "$proto" dp |
| `os/territories/hack/amnesia-firewall.sh` | 74 | out\|both) nft add rule inet tinker_amnesia output "$proto" d |
| `os/territories/hack/amnesia-firewall.sh` | 116 | sudo macchanger -r "$iface" >/dev/null 2>&1 \|\| true |
| `os/territories/hack/amnesia-firewall.sh` | 136 | command -v nft >/dev/null 2>&1 && nft delete table inet tink |
| `os/territories/hack/app-guard.sh` | 27 | sed -i "/^${app}$/d" "$APPROVED" 2>/dev/null \|\| true |
| `os/territories/hack/browser-gate.sh` | 30 | local real; real="$(command -v "$b" 2>/dev/null \|\| true)" |
| `os/territories/hack/canary-honeypot.sh` | 32 | echo "-----BEGIN PRIVATE KEY----- (bait) -----"   > "$base/. |
| `os/territories/hack/canary-honeypot.sh` | 33 | chmod 700 "$base" 2>/dev/null \|\| true |
| `os/territories/hack/canary-honeypot.sh` | 87 | pkill -f "socat -v TCP-LISTEN" 2>/dev/null \|\| true |
| `os/territories/hack/canary-honeypot.sh` | 88 | pkill -f "inotifywait -m" 2>/dev/null \|\| true |
| `os/territories/hack/ephemeral-ram.sh` | 47 | echo 0 \| sudo tee /proc/sys/vm/swapiness >/dev/null 2>&1 \|\|  |
| `os/territories/hack/ephemeral-ram.sh` | 48 | sudo swapoff -a 2>/dev/null && echo "All swap disabled (RAM- |
| `os/territories/hack/ephemeral-ram.sh` | 61 | dd if=/dev/urandom of="$f" bs=1M conv=notrunc 2>/dev/null \|\| |
| `os/territories/hack/ephemeral-ram.sh` | 64 | sudo umount "$RAM_MNT" 2>/dev/null \|\| true |
| `os/territories/hack/gpu-pipeline.sh` | 25 | clinfo 2>/dev/null \| grep -iE "Device Name\|Platform Name" \|  |
| `os/territories/hack/gpu-pipeline.sh` | 27 | glxinfo -B 2>/dev/null \| grep -iE "OpenGL renderer\|OpenGL ve |
| `os/territories/hack/gpu-pipeline.sh` | 29 | nvidia-smi --query-gpu=name,memory.total --format=csv 2>/dev |
| `os/territories/hack/gpu-pipeline.sh` | 31 | lspci \| grep -iE "VGA\|3D" \|\| true |
| `os/territories/hack/gpu-pipeline.sh` | 60 | clOpenCL/klee >/dev/null 2>&1 \|\| true |
| `os/territories/hack/gpu-pipeline.sh` | 70 | hashcat -b --benchmark-all 2>&1 \| head -25 \|\| true |
| `os/territories/hack/hack-defense.sh` | 33 | sysctl -a > "$BACKUP" 2>/dev/null \|\| true |
| `os/territories/hack/hack-defense.sh` | 49 | 2>/dev/null \|\| true |
| `os/territories/hack/hack-defense.sh` | 70 | > /etc/sysctl.d/99-tinker-hard.conf 2>/dev/null \|\| true |
| `os/territories/hack/hack-defense.sh` | 71 | sysctl --system >/dev/null 2>&1 \|\| true |
| `os/territories/hack/hack-defense.sh` | 79 | auditctl -a always,exit -F arch=b64 -S execve -k tinker_exec |
| `os/territories/hack/hack-defense.sh` | 80 | auditctl -w /etc/passwd -p wa -k tinker_auth 2>/dev/null \|\|  |
| `os/territories/hack/hack-defense.sh` | 81 | auditctl -w /etc/shadow -p wa -k tinker_auth 2>/dev/null \|\|  |
| `os/territories/hack/hack-defense.sh` | 82 | auditctl -w /etc/sudoers -p wa -k tinker_sudo 2>/dev/null \|\| |
| `os/territories/hack/hack-defense.sh` | 108 | ulimit -c 0 2>/dev/null \|\| true |
| `os/territories/hack/hack-defense.sh` | 109 | printf 'kernel.core_pattern=\|/bin/true\n' >/etc/sysctl.d/50- |
| `os/territories/hack/hack-defense.sh` | 110 | sysctl -w kernel.core_pattern="\|/bin/true" >/dev/null 2>&1 \| |
| `os/territories/hack/hack-defense.sh` | 142 | [ -f "$BACKUP" ] && sysctl -p "$BACKUP" >/dev/null 2>&1 \|\| t |
| `os/territories/hack/hack-defense.sh` | 143 | rm -f /etc/sysctl.d/99-tinker-hard.conf /etc/sysctl.d/50-tin |
| `os/territories/hack/hack-defense.sh` | 144 | sysctl --system >/dev/null 2>&1 \|\| true |
| `os/territories/hack/intent-hardware.sh` | 29 | pactl list sources short 2>/dev/null \| grep -i input \| awk ' |
| `os/territories/hack/intent-hardware.sh` | 43 | echo "${d%/}" > "${d%/}-driver" 2>/dev/null && echo "${d%/}" |
| `os/territories/hack/intent-hardware.sh` | 45 | sudo modprobe -r uvcvideo 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 50 | sudo modprobe uvcvideo 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 58 | pactl set-source-mute @DEFAULT_SOURCE@ 1 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 59 | for s in $(find_audio_in); do pactl set-source-mute "$s" 1 2 |
| `os/territories/hack/intent-hardware.sh` | 66 | pactl set-source-mute @DEFAULT_SOURCE@ 0 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 102 | pkill -f "v4l2 /dev/video10" 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 103 | pkill -f "fakemic" 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 23 | sudo ip link set "$iface" down 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 25 | sudo ip route flush dev "$iface" 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 31 | sudo ip netns add "$ns" 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 47 | sudo ip link set "$iface" down 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 51 | sudo ip link set "$iface" up 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 57 | sudo ip link set "$iface" down 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 58 | sudo iw dev "$iface" set type managed 2>/dev/null \|\| sudo ai |
| `os/territories/hack/kill-switch.sh` | 59 | sudo ip link set "$iface" up 2>/dev/null \|\| true |
| `os/territories/hack/kill-switch.sh` | 66 | sudo nmap -sn 192.168.1.0/24 2>/dev/null \| head -20 \|\| true |
| `os/territories/hack/panic-wipe.sh` | 23 | pkill -f "tinker-" 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 24 | pkill -f "rtl_fm" 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 25 | pkill -f "hashcat" 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 26 | pkill -f "socat -v TCP-LISTEN" 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 36 | dd if=/dev/urandom of="$f" bs=1M conv=notrunc 2>/dev/null \|\| |
| `os/territories/hack/panic-wipe.sh` | 44 | echo 3 2>/dev/null > /proc/sys/vm/drop_caches \|\| true |
| `os/territories/hack/panic-wipe.sh` | 50 | history -c 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 51 | > /tmp/tinker-amnesia.log 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 52 | rm -f "${TINKER_STATE}/territory.log" 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 54 | ssh-add -D 2>/dev/null \|\| true |
| `os/territories/hack/panic-wipe.sh` | 62 | sudo systemctl poweroff 2>/dev/null \|\| sudo poweroff 2>/dev/ |
| `os/territories/hack/portal-sandbox.sh` | 67 | picked=$(zenity --file-selection --title="KorrinOS — choose  |
| `os/territories/hack/portal-sandbox.sh` | 69 | picked=$(kdialog --getopenfilename . 2>/dev/null \|\| true) |
| `os/territories/hack/reverse-proxy.sh` | 22 | sudo ip netns add "$NS" 2>/dev/null \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 23 | sudo ip rule add fwmark $MARK table $TABLE pref 100 2>/dev/n |
| `os/territories/hack/reverse-proxy.sh` | 34 | sudo iptables -t mangle -A OUTPUT -m owner --uid-owner "$(wh |
| `os/territories/hack/reverse-proxy.sh` | 40 | sudo iptables -t mangle -A OUTPUT -m mark --mark $MARK -j DR |
| `os/territories/hack/reverse-proxy.sh` | 47 | has tor && { echo "Starting tor service"; sudo systemctl sta |
| `os/territories/hack/reverse-proxy.sh` | 54 | sudo ip rule show 2>/dev/null \| grep -E "0x\|$TABLE" \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 55 | sudo ip route show table $TABLE 2>/dev/null \| head \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 56 | sudo iptables -t mangle -L OUTPUT 2>/dev/null \| grep -i mark |
| `os/territories/hack/reverse-proxy.sh` | 62 | sudo ip rule del fwmark $MARK 2>/dev/null \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 63 | sudo iptables -t mangle -D OUTPUT -m mark --mark $MARK -j DR |
| `os/territories/hack/reverse-proxy.sh` | 64 | sudo ip netns del "$NS" 2>/dev/null \|\| true |
| `os/territories/hack/sdr-isolation.sh` | 42 | sudo ip netns add "$ns" 2>/dev/null \|\| true |
| `os/territories/hack/sdr-isolation.sh` | 53 | > "${TINKER_STATE}/capture.wav" \|\| true |
| `os/territories/hack/sdr-isolation.sh` | 63 | sudo hcitool lescan --passive --duplicates 2>&1 \| head -20 \| |
| `os/territories/hack/split-personality.sh` | 62 | sudo cryptsetup close "$name" 2>&1 \|\| true |
| `os/territories/hack/split-personality.sh` | 69 | sudo shred -v -n1 "$path" 2>/dev/null \|\| true |
| `os/territories/hack/split-personality.sh` | 79 | sudo cryptsetup status 2>/dev/null \|\| lsblk \| grep -i crypt  |
| `os/territories/hack/supply-chain.sh` | 39 | ( cd "$srcroot" && make BUILD_PATH="$BUILDROOT/out" 2>&1 \| t |
| `os/territories/hack/threat-monitor.sh` | 45 | county=$(grep -c "Failed password" /var/log/auth.log 2>/dev/ |
| `os/territories/hack/threat-monitor.sh` | 47 | grep "Failed password" /var/log/auth.log 2>/dev/null \| tail  |
| `os/territories/hack/threat-monitor.sh` | 53 | ps -eo user,pid,comm 2>/dev/null \| awk '$1=="root"' \| grep - |
| `os/territories/hack/threat-monitor.sh` | 59 | ls -la --time-style=full-iso /etc/passwd /etc/shadow /etc/su |
| `os/territories/hack/threat-monitor.sh` | 76 | && debug "risky pattern in $f (eval / destructive)" \|\| true |
| `os/territories/hack/threat-monitor.sh` | 84 | && debug "injection-risk pattern in $f" \|\| true |
| `os/branding/install-branding.sh` | 37 | sudo sed -i 's\|^#*GRUB_THEME=.*\|GRUB_THEME="/boot/grub/theme |
| `os/branding/install-branding.sh` | 38 | sudo update-grub 2>/dev/null \| tail -1 \|\| true |
| `os/branding/install-branding.sh` | 40 | sudo sed -i 's\|^#*GRUB_THEME=.*\|GRUB_THEME="/boot/grub/theme |
| `os/brand/install-boot-intro.sh` | 31 | sed -i 's/GRUB_CMDLINE_LINUX_DEFAULT="[^"]*"/GRUB_CMDLINE_LI |
| `os/brand/install-boot-intro.sh` | 37 | update-initramfs -u \|\| true |
| `os/brand/install-boot-intro.sh` | 39 | dracut --force \|\| true |
| `os/brand/install-boot-intro.sh` | 50 | plymouth-set-default-theme --reset \|\| plymouth-set-default-t |
| `os/brand/install-boot-intro.sh` | 58 | plymouth-set-default-theme --list 2>/dev/null \|\| true |
| `os/brand/install-gdm-skin.sh` | 45 | gsettings set org.gnome.desktop.background picture-uri "file |
| `os/brand/install-gdm-skin.sh` | 46 | gsettings set org.gnome.desktop.background picture-uri-dark  |
| `os/brand/install-gdm-skin.sh` | 47 | gsettings set org.gnome.desktop.background primary-color "#0 |
| `os/brand/install-grub-theme.sh` | 25 | sed -i "s#^GRUB_THEME=.*#GRUB_THEME=\"$THEME_DST/theme.txt\" |
| `os/brand/install-grub-theme.sh` | 30 | update-grub \|\| true |
| `os/brand/install-grub-theme.sh` | 32 | /usr/sbin/grub-mkconfig -o /boot/grub/grub.cfg \|\| true |
| `os/brand/install-grub-theme.sh` | 41 | sed -i '/^GRUB_THEME=/d' /etc/default/grub 2>/dev/null \|\| tr |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 27 | xrandr --verbose 2>/dev/null \| grep "Brightness" \| head -1 \| |
| `os/languages/install.sh` | 57 | cp examples/*.kor /opt/korrinos/os/languages/demo/ 2>/dev/nu |
| `os/languages/install.sh` | 58 | cp examples/*.kui /opt/korrinos/os/languages/demo/ 2>/dev/nu |
| `os/desktop/desktop-integration.sh` | 68 | xfce4-panel --add-item 2>/dev/null \|\| true |
| `os/desktop/desktop-integration.sh` | 209 | xfconf-query -c xfce4-keyboard-shortcuts -p "/custom/Custom0 |
| `os/desktop/desktop-integration.sh` | 213 | gsettings set org.gnome.desktop.wm.keybindings show-desktop  |
| `os/desktop/desktop-integration.sh` | 214 | gsettings set org.gnome.settings-daemon.plugins.media-keys h |
| `os/desktop/desktop-integration.sh` | 218 | kwriteconfig5 --file kwinrc --group ModifierOnlyShortcuts -- |
| `os/desktop/desktop-integration.sh` | 262 | xfce4-panel --add-item 2>/dev/null \|\| true |
| `os/desktop/settings-gui.sh` | 281 | sudo systemctl mask sleep.target suspend.target hibernate.ta |
| `os/desktop/settings-gui.sh` | 307 | sudo modprobe -r uvcvideo 2>/dev/null \|\| true; save_setting  |
| `os/desktop/settings-gui.sh` | 309 | sudo modprobe uvcvideo 2>/dev/null \|\| true; save_setting PRI |
| `os/desktop/settings-gui.sh` | 314 | sudo modprobe -r snd_usb_audio snd_hda_intel 2>/dev/null \|\|  |
| `os/desktop/settings-gui.sh` | 316 | sudo modprobe snd_usb_audio snd_hda_intel 2>/dev/null \|\| tru |
| `os/desktop/shortcuts.sh` | 146 | pkill -f xbindkeys 2>/dev/null \|\| true |
| `os/desktop/text-expander.sh` | 139 | done < <(grep -v "^#" "$SNIPPETS_FILE" 2>/dev/null \|\| true) |
| `os/desktop/theme-manager.sh` | 112 | gsettings set org.gnome.desktop.interface gtk-theme 'Adwaita |
| `os/desktop/theme-manager.sh` | 113 | gsettings set org.gnome.desktop.interface color-scheme 'pref |
| `os/desktop/theme-manager.sh` | 115 | gsettings set org.gnome.desktop.interface gtk-theme 'Adwaita |
| `os/desktop/theme-manager.sh` | 116 | gsettings set org.gnome.desktop.interface color-scheme 'pref |
| `os/desktop/nibra-style/nibra-shell.sh` | 62 | xfconf-query -c xfce4-desktop -p /backdrop/screen0/monitor0/ |
| `os/desktop/nibra-style/nibra-shell.sh` | 72 | pkill picom 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 112 | pkill -f korrinos-liquid-glass 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 113 | pkill -f korrinos-widgets-panel 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 114 | pkill -f korrinos-dock 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 115 | pkill -f korrinos-smoothui 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 127 | pkill picom 2>/dev/null \|\| true |
| `os/desktop/nibra-style/nibra-shell.sh` | 128 | pkill -f "nibra-style/server.py" 2>/dev/null \|\| true |

### C12 sudo with unquoted var — 805

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 13 | # Requires: sudo + debootstrap + mksquashfs + xorriso + internet. |
| `os/build-distro.sh` | 27 | if ! sudo -n true 2>/dev/null; then |
| `os/build-distro.sh` | 28 | echo "ERROR: passwordless sudo required. Run: sudo -v" >&2 |
| `os/build-distro.sh` | 526 | useradd -m -s /bin/bash -G sudo,adm,dialout,cdrom,floppy,audio,dip,vid |
| `os/build-distro.sh` | 761 | # to the real user so grub-mk* and xorriso can write without sudo. |
| `os/iso-builder.sh` | 65 | echo "Install with: sudo apt install$missing" |
| `os/apps/app-store.sh` | 96 | sudo apt install -y firefox \|\| sudo dnf install -y firefox \|\| sudo pac |
| `os/apps/app-store.sh` | 99 | sudo apt install -y chromium-browser \|\| sudo dnf install -y chromium \| |
| `os/apps/app-store.sh` | 102 | sudo apt install -y thunderbird \|\| sudo dnf install -y thunderbird \|\|  |
| `os/apps/app-store.sh` | 105 | sudo apt install -y libreoffice \|\| sudo dnf install -y libreoffice \|\|  |
| `os/apps/app-store.sh` | 108 | sudo apt install -y gimp \|\| sudo dnf install -y gimp \|\| sudo pacman -S |
| `os/apps/app-store.sh` | 111 | sudo apt install -y blender \|\| sudo dnf install -y blender \|\| sudo pac |
| `os/apps/app-store.sh` | 116 | sudo dpkg --add-architecture i386 |
| `os/apps/app-store.sh` | 117 | sudo apt update |
| `os/apps/app-store.sh` | 118 | sudo apt install -y steam |
| `os/apps/app-store.sh` | 120 | sudo dnf install -y https://mirrors.rpmfusion.org/free/fedora/rpmfusio |
| `os/apps/app-store.sh` | 121 | sudo dnf install -y steam |
| `os/apps/app-store.sh` | 123 | sudo pacman -S --noconfirm steam |
| `os/apps/app-store.sh` | 130 | sudo install -D -o root -g root -m 644 packages.microsoft.gpg /etc/apt |
| `os/apps/app-store.sh` | 131 | echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/packages.microsoft.g |
| `os/apps/app-store.sh` | 132 | sudo apt update |
| `os/apps/app-store.sh` | 133 | sudo apt install -y code |
| `os/apps/app-store.sh` | 139 | sudo apt install -y docker.io docker-compose |
| `os/apps/app-store.sh` | 140 | sudo usermod -aG docker $USER |
| `os/apps/app-store.sh` | 142 | sudo dnf install -y docker docker-compose |
| `os/apps/app-store.sh` | 143 | sudo usermod -aG docker $USER |
| `os/apps/app-store.sh` | 145 | sudo pacman -S --noconfirm docker docker-compose |
| `os/apps/app-store.sh` | 146 | sudo usermod -aG docker $USER |
| `os/apps/app-store.sh` | 150 | sudo apt install -y obs-studio \|\| sudo dnf install -y obs-studio \|\| su |
| `os/apps/app-store.sh` | 153 | sudo apt install -y vlc \|\| sudo dnf install -y vlc \|\| sudo pacman -S - |
| `os/apps/app-store.sh` | 158 | curl -sS https://download.spotify.com/debian/pubkey_C85668DF69375001.g |
| `os/apps/app-store.sh` | 159 | echo "deb http://repository.spotify.com stable non-free" \| sudo tee /e |
| `os/apps/app-store.sh` | 160 | sudo apt update |
| `os/apps/app-store.sh` | 161 | sudo apt install -y spotify |
| `os/apps/app-store.sh` | 168 | sudo dpkg -i /tmp/discord.deb |
| `os/apps/app-store.sh` | 169 | sudo apt install -f -y |
| `os/apps/app-store.sh` | 176 | sudo dpkg -i /tmp/zoom.deb |
| `os/apps/app-store.sh` | 177 | sudo apt install -f -y |
| `os/apps/app-store.sh` | 181 | sudo apt install -y filezilla \|\| sudo dnf install -y filezilla \|\| sudo |
| `os/apps/app-store.sh` | 184 | sudo apt install -y virtualbox \|\| sudo dnf install -y VirtualBox \|\| su |
| `os/apps/app-store.sh` | 187 | sudo apt install -y gparted \|\| sudo dnf install -y gparted \|\| sudo pac |
| `os/apps/app-store.sh` | 190 | sudo apt install -y htop \|\| sudo dnf install -y htop \|\| sudo pacman -S |
| `os/apps/app-store.sh` | 193 | sudo apt install -y neofetch \|\| sudo dnf install -y neofetch \|\| sudo p |
| `os/apps/app-store.sh` | 196 | sudo apt install -y cmatrix \|\| sudo dnf install -y cmatrix \|\| sudo pac |
| `os/apps/app-store.sh` | 217 | firefox) sudo apt remove -y firefox \|\| sudo dnf remove -y firefox \|\| s |
| `os/apps/app-store.sh` | 218 | chromium) sudo apt remove -y chromium-browser \|\| sudo dnf remove -y ch |
| `os/apps/app-store.sh` | 219 | thunderbird) sudo apt remove -y thunderbird \|\| sudo dnf remove -y thun |
| `os/apps/app-store.sh` | 220 | libreoffice) sudo apt remove -y libreoffice \|\| sudo dnf remove -y libr |
| `os/apps/app-store.sh` | 221 | gimp) sudo apt remove -y gimp \|\| sudo dnf remove -y gimp \|\| sudo pacma |
| `os/apps/app-store.sh` | 274 | sudo apt update && sudo apt upgrade -y |
| `os/apps/app-store.sh` | 276 | sudo dnf upgrade -y |
| `os/apps/app-store.sh` | 278 | sudo pacman -Syu --noconfirm |
| `os/apps/gaming-mode.sh` | 47 | sudo nvidia-smi -pm 1 |
| `os/apps/gaming-mode.sh` | 48 | sudo nvidia-smi -pl 100 |
| `os/apps/gaming-mode.sh` | 65 | sudo sysctl -w vm.swappiness=10 |
| `os/apps/gaming-mode.sh` | 66 | sudo sysctl -w net.core.rmem_max=16777216 |
| `os/apps/gaming-mode.sh` | 67 | sudo sysctl -w net.core.wmem_max=16777216 |
| `os/apps/gaming-mode.sh` | 110 | sudo nvidia-smi -pm 0 |
| `os/apps/gaming-mode.sh` | 111 | sudo nvidia-smi -pl 100 |
| `os/apps/gaming-mode.sh` | 127 | sudo sysctl -w vm.swappiness=60 |
| `os/apps/gaming-mode.sh` | 128 | sudo sysctl -w net.core.rmem_max=212992 |
| `os/apps/gaming-mode.sh` | 129 | sudo sysctl -w net.core.wmem_max=212992 |
| `os/apps/gaming-support.sh` | 10 | sudo dpkg --add-architecture i386 |
| `os/apps/gaming-support.sh` | 11 | sudo apt update |
| `os/apps/gaming-support.sh` | 12 | sudo apt install -y steam-installer |
| `os/apps/gaming-support.sh` | 14 | sudo dnf install -y steam |
| `os/apps/gaming-support.sh` | 16 | sudo pacman -S --noconfirm steam |
| `os/apps/gaming-support.sh` | 26 | sudo apt install -y lutris |
| `os/apps/gaming-support.sh` | 28 | sudo dnf install -y lutris |
| `os/apps/gaming-support.sh` | 30 | sudo pacman -S --noconfirm lutris |
| `os/apps/gaming-support.sh` | 104 | echo "  Also consider 'protonup-qt': sudo apt install protonup-qt" |
| `os/apps/gaming-support.sh` | 127 | sudo apt install -y mangohud |
| `os/apps/gaming-support.sh` | 129 | sudo dnf install -y mangohud |
| `os/apps/gaming-support.sh` | 131 | sudo pacman -S --noconfirm mangohud |
| `os/apps/gaming-support.sh` | 141 | sudo apt install -y gamemode |
| `os/apps/gaming-support.sh` | 143 | sudo dnf install -y gamemode |
| `os/apps/gaming-support.sh` | 145 | sudo pacman -S --noconfirm gamemode |
| `os/apps/gaming-support.sh` | 155 | sudo apt install -y gamescope |
| `os/apps/gaming-support.sh` | 157 | sudo dnf install -y gamescope |
| `os/apps/gaming-support.sh` | 159 | sudo pacman -S --noconfirm gamescope |
| `os/apps/gaming-support.sh` | 169 | sudo dpkg --add-architecture i386 |
| `os/apps/gaming-support.sh` | 170 | sudo apt update |
| `os/apps/gaming-support.sh` | 171 | sudo apt install -y wine64 wine32 |
| `os/apps/gaming-support.sh` | 173 | sudo dnf install -y wine |
| `os/apps/gaming-support.sh` | 175 | sudo pacman -S --noconfirm wine |
| `os/apps/gaming-support.sh` | 220 | sudo apt update |
| `os/apps/gaming-support.sh` | 221 | sudo apt install -y nvidia-driver vulkan-tools libvulkan1 libvulkan1:i |
| `os/apps/gaming-support.sh` | 224 | sudo dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda vulkan-loade |
| `os/apps/gaming-support.sh` | 225 | echo "  Note: run 'sudo grub2-mkconfig' or reboot to build akmod modul |
| `os/apps/gaming-support.sh` | 227 | sudo pacman -S --noconfirm nvidia nvidia-utils lib32-nvidia-utils vulk |
| `os/apps/ocr-everywhere.sh` | 293 | sudo apt install -y tesseract-ocr-$lang |
| `os/apps/ocr-everywhere.sh` | 295 | sudo dnf install -y tesseract-langpack-$lang |
| `os/apps/ocr-everywhere.sh` | 297 | sudo pacman -S --noconfirm tesseract-data-$lang |
| `os/apps/package-manager.sh` | 144 | sudo apt update && sudo apt upgrade -y |
| `os/apps/package-manager.sh` | 147 | sudo dnf upgrade -y |
| `os/apps/package-manager.sh` | 150 | sudo pacman -Syu --noconfirm |
| `os/apps/package-manager.sh` | 153 | sudo zypper update -y |
| `os/apps/package-manager.sh` | 159 | sudo snap refresh 2>/dev/null \|\| true |
| `os/apps/package-manager.sh` | 237 | sudo apt clean |
| `os/apps/package-manager.sh` | 238 | sudo apt autoremove -y |
| `os/apps/package-manager.sh` | 241 | sudo dnf clean all |
| `os/apps/package-manager.sh` | 244 | sudo pacman -Sc --noconfirm |
| `os/apps/package-manager.sh` | 247 | sudo zypper clean |
| `os/apps/software-center.sh` | 198 | sudo apt update -qq 2>/dev/null |
| `os/apps/system-cleaner.sh` | 23 | sudo apt clean 2>/dev/null \|\| sudo pacman -Sc --noconfirm 2>/dev/null  |
| `os/apps/system-cleaner.sh` | 31 | sudo find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 38 | sudo journalctl --vacuum-time=3d 2>/dev/null \|\| true |
| `os/apps/system-cleaner.sh` | 72 | sudo apt autoremove --purge -y 2>/dev/null \|\| true |
| `os/apps/voice-commands.sh` | 41 | sudo apt install -y espeak espeak-ng python3-pip |
| `os/apps/voice-commands.sh` | 44 | sudo dnf install -y espeak-ng python3-pip |
| `os/apps/voice-commands.sh` | 47 | sudo pacman -S --noconfirm espeak-ng python-pip |
| `os/apps/voice-commands.sh` | 217 | sudo shutdown -h 1 |
| `os/apps/voice-commands.sh` | 222 | sudo reboot |
| `os/apps/gaming/controller-mapper.sh` | 126 | echo "  sudo apt install xboxdrv" |
| `os/apps/gaming/controller-mapper.sh` | 129 | echo "  sudo pip install ds4drv" |
| `os/apps/gaming/controller-mapper.sh` | 132 | echo "  sudo apt install joystick" |
| `os/apps/gaming/discord-presence.sh` | 33 | echo "  Install: sudo apt install discord" |
| `os/apps/gaming/emulator-manager.sh` | 86 | sudo apt install retroarch |
| `os/apps/gaming/emulator-manager.sh` | 89 | sudo apt install libretro-* 2>/dev/null \|\| true |
| `os/apps/gaming/fps-monitor.sh` | 34 | echo "  Install: sudo apt install mangohud" |
| `os/apps/gaming/fps-monitor.sh` | 114 | echo "  Try: sudo apt install glmark2" |
| `os/apps/gaming/gif-recorder.sh` | 36 | echo "Install ffmpeg: sudo apt install ffmpeg" |
| `os/apps/gaming/hardware-benchmark.sh` | 60 | echo "  sysbench not installed (sudo apt install sysbench)" |
| `os/apps/gaming/screenshot-tool.sh` | 38 | echo "  Install one: sudo apt install grim scrot imagemagick" |
| `os/apps/gaming/screenshot-tool.sh` | 112 | echo "No annotation tool. Install: sudo apt install flameshot" |
| `os/apps/gaming/streaming-manager.sh` | 34 | echo "  Install: sudo apt install obs-studio" |
| `os/apps/gaming/wine-manager.sh` | 33 | echo "  Install: sudo apt install wine" |
| `os/apps/gaming/wine-manager.sh` | 68 | sudo apt install winetricks |
| `os/apps/customization/conky-stats.sh` | 32 | echo "Install conky: sudo apt install conky-all" |
| `os/apps/customization/grub-theme.sh` | 44 | sudo sed -i "s/^GRUB_THEME=.*/GRUB_THEME=\"\/usr\/share\/grub\/themes\ |
| `os/apps/customization/grub-theme.sh` | 46 | sudo update-grub 2>/dev/null \|\| sudo grub-mkconfig -o /boot/grub/grub. |
| `os/apps/customization/grub-theme.sh` | 74 | sudo sed -i "s/^GRUB_TIMEOUT=.*/GRUB_TIMEOUT=$timeout/" /etc/default/g |
| `os/apps/customization/grub-theme.sh` | 76 | sudo update-grub 2>/dev/null \|\| sudo grub-mkconfig -o /boot/grub/grub. |
| `os/apps/customization/login-theme.sh` | 57 | sudo sed -i "s/^greeter-session=.*/greeter-session=$theme/" /etc/light |
| `os/apps/customization/qt-theme.sh` | 23 | echo "Install qt5ct: sudo apt install qt5ct" |
| `os/apps/security/biometric.sh` | 38 | echo "  To integrate with login (requires sudo):" |
| `os/apps/security/biometric.sh` | 39 | echo "    sudo pam-auth-update  # enable 'fprintd'" |
| `os/apps/security/biometric.sh` | 43 | echo "  Install: sudo apt install fprintd libpam-fprintd" |
| `os/apps/security/biometric.sh` | 68 | echo "    sudo apt install fprintd libpam-fprintd" |
| `os/apps/security/filevault.sh` | 22 | command -v $tool &>/dev/null \|\| { echo "Missing: $tool (sudo apt insta |
| `os/apps/security/filevault.sh` | 42 | echo "Setting up loop device (requires sudo)..." |
| `os/apps/security/filevault.sh` | 45 | echo "  sudo losetup /dev/loop0 $VAULT_DIR/$name.img" |
| `os/apps/security/filevault.sh` | 46 | echo "  sudo cryptsetup luksFormat /dev/loop0" |
| `os/apps/security/filevault.sh` | 47 | echo "  sudo cryptsetup open /dev/loop0 $name" |
| `os/apps/security/filevault.sh` | 48 | echo "  sudo mkfs.${FS:-ext4} /dev/mapper/$name" |
| `os/apps/security/filevault.sh` | 49 | echo "  sudo mount /dev/mapper/$name $MOUNT_POINT" |
| `os/apps/security/filevault.sh` | 61 | echo "  sudo cryptsetup open $VAULT_DIR/$name.img $name" |
| `os/apps/security/filevault.sh` | 62 | echo "  sudo mount /dev/mapper/$name $MOUNT_POINT" |
| `os/apps/security/filevault.sh` | 64 | echo "  (Interactive. Run the sudo command above to complete.)" |
| `os/apps/security/security-suite.sh` | 53 | echo "Users in sudo group:" |
| `os/apps/security/security-suite.sh` | 54 | getent group sudo 2>/dev/null \| cut -d: -f4 \| tr ',' '\n' \| sed 's/^/  |
| `os/apps/security/security-suite.sh` | 135 | [ -f /etc/ssh/sshd_config ] && sudo sed -i 's/^#PermitRootLogin.*/Perm |
| `os/apps/security/security-suite.sh` | 138 | command -v ufw &>/dev/null && sudo ufw enable 2>/dev/null && echo "    |
| `os/apps/security/security-suite.sh` | 141 | sudo bash -c 'echo "hard core 0" > /etc/security/limits.d/core.conf' 2 |
| `os/apps/hardware/display-calibration.sh` | 41 | echo "Install xcalib: sudo apt install xcalib" |
| `os/apps/hardware/fingerprint-manager.sh` | 36 | echo "Install: sudo apt install fprintd libpam-fprintd" |
| `os/apps/hardware/gpio-manager.sh` | 65 | echo "out" \| sudo tee /sys/class/gpio/gpio$pin/direction > /dev/null 2 |
| `os/apps/hardware/kvm-switch.sh` | 33 | echo "  sudo apt install barrier" |
| `os/apps/hardware/nfc-manager.sh` | 30 | echo "Install: sudo apt install libnfc-utils" |
| `os/apps/hardware/pen-stylus.sh` | 57 | echo "Install xournalpp: sudo apt install xournalpp" |
| `os/apps/hardware/scanner-manager.sh` | 32 | echo "Install: sudo apt install sane" |
| `os/apps/hardware/scanner-manager.sh` | 47 | echo "Install sane: sudo apt install sane" |
| `os/apps/hardware/scanner-manager.sh` | 75 | echo "  sudo apt install xsane" |
| `os/apps/hardware/scanner-manager.sh` | 77 | echo "  sudo apt install simple-scan" |
| `os/apps/hardware/serial-uart.sh` | 52 | echo "  sudo apt install screen" |
| `os/apps/hardware/thunderbolt-manager.sh` | 52 | echo 1 \| sudo tee /sys/bus/thunderbolt/devices/$device/authorized 2>/d |
| `os/apps/hardware/touchscreen-manager.sh` | 42 | echo "Install: sudo apt install xinput-calibrator" |
| `os/apps/hardware/webcam-manager.sh` | 136 | echo "    sudo usermod -aG video $USER" |
| `os/apps/system/command-palette.sh` | 50 | echo "Install fzf for interactive mode: sudo apt install fzf" |
| `os/apps/system/disk-visualizer.sh` | 61 | echo "Install ncdu for interactive visualization: sudo apt install ncd |
| `os/apps/system/markdown-editor.sh` | 55 | echo "  sudo apt install grip" |
| `os/apps/system/terminal-error-explainer.sh` | 17 | echo "  Fix:   sudo or add user to the owning group; check file perms  |
| `os/apps/network/bandwidth-limiter.sh` | 31 | echo "Install trickle: sudo apt install trickle" |
| `os/apps/network/bandwidth-limiter.sh` | 50 | sudo tc qdisc del dev eth0 root 2>/dev/null \|\| true |
| `os/apps/network/dns-manager.sh` | 39 | sudo bash -c "cat > /etc/resolv.conf" << EOF |
| `os/apps/network/mesh-network.sh` | 40 | sudo tailscale up |
| `os/apps/network/mesh-network.sh` | 55 | echo "Install: curl -s https://install.zerotier.com \| sudo bash" |
| `os/apps/network/wifi-analyzer.sh` | 157 | echo "      sudo iw dev $iface set power_save off" |
| `os/scripts/setup-gpu-stacks.sh` | 3 | # Run AFTER OS boots. Requires sudo. |
| `os/hyperdrive/hyperdrive-cli.sh` | 83 | sudo insmod /lib/modules/$(uname -r)/extra/hyperdrive.ko 2>/dev/null \| |
| `os/hyperdrive/hyperdrive-cli.sh` | 88 | sudo systemctl start hyperdrive |
| `os/hyperdrive/hyperdrive-cli.sh` | 96 | sudo systemctl stop hyperdrive |
| `os/hyperdrive/hyperdrive-cli.sh` | 153 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scali |
| `os/hyperdrive/hyperdrive-cli.sh` | 157 | echo always \| sudo tee /proc/sys/vm/nr_hugepages >/dev/null 2>&1 \|\| tr |
| `os/hyperdrive/hyperdrive-cli.sh` | 161 | echo 1 \| sudo tee /proc/sys/vm/compact_memory >/dev/null 2>&1 \|\| true |
| `os/hyperdrive/hyperdrive-cli.sh` | 166 | echo 3 \| sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 \|\| true |
| `os/hyperdrive/hyperdrive-cli.sh` | 170 | echo never \| sudo tee /sys/kernel/mm/transparent_hugepage/enabled >/de |
| `os/hyperdrive/hyperdrive-daemon.sh` | 61 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scali |
| `os/hyperdrive/hyperdrive-daemon.sh` | 111 | echo 3 \| sudo tee /proc/sys/vm/drop_caches >/dev/null 2>&1 \|\| true |
| `os/hyperdrive/hyperdrive-daemon.sh` | 114 | echo 1 \| sudo tee /proc/sys/vm/compact_memory >/dev/null 2>&1 \|\| true |
| `os/hyperdrive/hyperdrive-daemon.sh` | 126 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scali |
| `os/hyperdrive/hyperdrive-daemon.sh` | 129 | echo always \| sudo tee /proc/sys/vm/nr_hugepages >/dev/null 2>&1 \|\| tr |
| `os/hyperdrive/hyperdrive-daemon.sh` | 132 | sudo renice -n -10 $$ 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 6 | # Install: sudo ./hyperdrive.sh install |
| `os/hyperdrive/hyperdrive.sh` | 211 | sudo swapoff /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 212 | sudo echo "lz4" > /sys/block/zram0/comp_algorithm 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 213 | sudo echo "2G" > /sys/block/zram0/disksize 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 214 | sudo mkswap /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 215 | sudo swapon /dev/zram0 2>/dev/null \|\| true |
| `os/hyperdrive/hyperdrive.sh` | 269 | echo performance \| sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scali |
| `os/hyperdrive/hyperdrive.sh` | 509 | sudo tee /etc/systemd/system/hyperdrive.service > /dev/null << 'SERVIC |
| `os/hyperdrive/hyperdrive.sh` | 528 | sudo systemctl daemon-reload |
| `os/hyperdrive/hyperdrive.sh` | 529 | sudo systemctl enable hyperdrive.service |
| `os/hyperdrive/hyperdrive.sh` | 530 | sudo systemctl start hyperdrive.service |
| `os/hyperdrive/hyperdrive.sh` | 574 | start) sudo systemctl start hyperdrive ;; |
| `os/hyperdrive/hyperdrive.sh` | 575 | stop) sudo systemctl stop hyperdrive ;; |
| `os/hyperdrive/hyperdrive.sh` | 576 | status) sudo systemctl status hyperdrive ;; |
| `os/parc-ai/korrinos-annotate.sh` | 23 | echo "No screenshot tool. Install: sudo apt install scrot" |
| `os/parc-ai/korrinos-annotate.sh` | 41 | echo "Install ImageMagick: sudo apt install imagemagick" |
| `os/parc-ai/korrinos-ascii.sh` | 15 | echo "Install figlet: sudo apt install figlet" |
| `os/parc-ai/korrinos-cleanup.sh` | 26 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 27 | sudo apt autoremove -y 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 64 | sudo find /var/log -type f -name "*.gz" -delete 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 65 | sudo find /var/log -type f -name "*.old" -delete 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 66 | sudo journalctl --vacuum-time=3d 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 87 | sudo apt autoremove --purge -y 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 164 | sudo apt list --installed 2>/dev/null \| grep -i "autoinstall" \| head - |
| `os/parc-ai/korrinos-cleanup.sh` | 168 | sudo apt list --installed 2>/dev/null \| awk -F/ 'NR>1 && !seen[$1]++ { |
| `os/parc-ai/korrinos-cleanup.sh` | 240 | sudo mandb -q 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 245 | sudo journalctl --vacuum-size=100M 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-cleanup.sh` | 254 | sudo fstrim -av 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-dock.sh` | 211 | echo "Install plank for full experience: sudo apt install plank" |
| `os/parc-ai/korrinos-liquid-glass.sh` | 209 | echo "Error: picom not installed. Install with: sudo apt install picom |
| `os/parc-ai/korrinos-network.sh` | 180 | echo "nameserver 1.1.1.1" \| sudo tee /etc/resolv.conf > /dev/null |
| `os/parc-ai/korrinos-network.sh` | 181 | echo "nameserver 1.0.0.1" \| sudo tee -a /etc/resolv.conf > /dev/null |
| `os/parc-ai/korrinos-network.sh` | 185 | echo "nameserver 8.8.8.8" \| sudo tee /etc/resolv.conf > /dev/null |
| `os/parc-ai/korrinos-network.sh` | 186 | echo "nameserver 8.8.4.4" \| sudo tee -a /etc/resolv.conf > /dev/null |
| `os/parc-ai/korrinos-network.sh` | 190 | echo "nameserver 9.9.9.9" \| sudo tee /etc/resolv.conf > /dev/null |
| `os/parc-ai/korrinos-network.sh` | 191 | echo "nameserver 149.112.112.112" \| sudo tee -a /etc/resolv.conf > /de |
| `os/parc-ai/korrinos-network.sh` | 195 | echo "nameserver 208.67.222.222" \| sudo tee /etc/resolv.conf > /dev/nu |
| `os/parc-ai/korrinos-network.sh` | 196 | echo "nameserver 208.67.220.220" \| sudo tee -a /etc/resolv.conf > /dev |
| `os/parc-ai/korrinos-notepad.sh` | 246 | echo "Install yad or zenity for GUI: sudo apt install yad" |
| `os/parc-ai/korrinos-security-apps.sh` | 114 | sudo ufw status 2>/dev/null \|\| echo "  UFW not configured" |
| `os/parc-ai/korrinos-smoothui.sh` | 74 | sudo apt install -y picom 2>/dev/null \|\| \ |
| `os/parc-ai/korrinos-smoothui.sh` | 75 | sudo pacman -S picom 2>/dev/null \|\| \ |
| `os/parc-ai/korrinos-smoothui.sh` | 76 | sudo dnf install -y picom 2>/dev/null \|\| \ |
| `os/parc-ai/korrinos-smoothui.sh` | 84 | sudo apt install -y xdotool 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-smoothui.sh` | 90 | sudo apt install -y libinput-tools 2>/dev/null \|\| true |
| `os/parc-ai/korrinos-voice.sh` | 123 | echo "Install arecord: sudo apt install alsa-utils" |
| `os/parc-ai/korrinos-widgets-panel.sh` | 182 | echo "Error: conky not installed. Install with: sudo apt install conky |
| `os/parc-ai/parcos-ai-installer.sh` | 22 | fail "Run with sudo: sudo bash korrinos-ai-installer.sh" |
| `os/parc-ai/parcos-clipboard.sh` | 23 | echo "No clipboard tool found. Install xclip: sudo apt install xclip" |
| `os/parc-ai/parcos-devtool.sh` | 157 | sudo apt install -y python3 python3-pip python3-venv |
| `os/parc-ai/parcos-devtool.sh` | 160 | curl -fsSL https://deb.nodesource.com/setup_20.x \| sudo -E bash - |
| `os/parc-ai/parcos-devtool.sh` | 161 | sudo apt install -y nodejs |
| `os/parc-ai/parcos-devtool.sh` | 167 | sudo apt install -y golang-go |
| `os/parc-ai/parcos-devtool.sh` | 170 | sudo apt install -y default-jdk |
| `os/parc-ai/parcos-devtool.sh` | 173 | sudo apt install -y ruby-full |
| `os/parc-ai/parcos-devtool.sh` | 176 | sudo apt install -y php-cli |
| `os/parc-ai/parcos-pm.sh` | 142 | sudo apt update && sudo apt upgrade -y |
| `os/parc-ai/parcos-pm.sh` | 150 | sudo snap refresh |
| `os/parc-ai/parcos-pm.sh` | 223 | sudo apt install -y \ |
| `os/parc-ai/parcos-pm.sh` | 230 | sudo apt install -y \ |
| `os/parc-ai/parcos-pm.sh` | 237 | sudo apt install -y \ |
| `os/parc-ai/parcos-pm.sh` | 243 | sudo apt install -y \ |
| `os/parc-ai/parcos-recovery.sh` | 60 | sudo btrfs subvolume list / 2>/dev/null \| grep snapshot |
| `os/parc-ai/parcos-recovery.sh` | 77 | sudo reboot |
| `os/parc-ai/parcos-recovery.sh` | 108 | sudo grub-mkconfig -o /boot/grub/grub.cfg 2>/dev/null \|\| echo "GRUB up |
| `os/parc-ai/parcos-recovery.sh` | 112 | sudo cat /etc/fstab |
| `os/parc-ai/parcos-recovery.sh` | 116 | sudo update-initramfs -u 2>/dev/null \|\| echo "initramfs update failed" |
| `os/parc-ai/parcos-recovery.sh` | 129 | sudo dpkg --verify 2>/dev/null \| head -20 \|\| echo "dpkg verify not ava |
| `os/parc-ai/parcos-recovery.sh` | 133 | sudo apt --fix-broken install 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 137 | sudo fsck -n / 2>/dev/null \|\| echo "fsck check (read-only)" |
| `os/parc-ai/parcos-recovery.sh` | 152 | sudo dpkg --configure -a 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 153 | sudo apt --fix-broken install -y 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 157 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 161 | sudo chmod 1777 /tmp 2>/dev/null \|\| true |
| `os/parc-ai/parcos-recovery.sh` | 165 | sudo systemctl daemon-reload 2>/dev/null \|\| true |
| `os/parc-ai/parcos-searchie.sh` | 147 | echo "tesseract not installed. Run: sudo apt install tesseract-ocr" |
| `os/parc-ai/parcos-security.sh` | 16 | sudo ufw status verbose |
| `os/parc-ai/parcos-security.sh` | 21 | echo "No firewall found. Install ufw: sudo apt install ufw" |
| `os/parc-ai/parcos-security.sh` | 28 | sudo ufw enable |
| `os/parc-ai/parcos-security.sh` | 29 | sudo ufw default deny incoming |
| `os/parc-ai/parcos-security.sh` | 30 | sudo ufw default allow outgoing |
| `os/parc-ai/parcos-security.sh` | 33 | echo "Install ufw: sudo apt install ufw" |
| `os/parc-ai/parcos-security.sh` | 40 | sudo ufw disable |
| `os/parc-ai/parcos-security.sh` | 177 | sudo iptables -L -n 2>/dev/null \| head -20 \|\| echo "Cannot read iptabl |
| `os/parc-ai/parcos-settings.sh` | 186 | performance) echo performance \| sudo tee /sys/devices/system/cpu/cpu*/ |
| `os/parc-ai/parcos-settings.sh` | 187 | powersave)   echo powersave \| sudo tee /sys/devices/system/cpu/cpu*/cp |
| `os/parc-ai/parcos-settings.sh` | 188 | balanced)    echo schedutil \| sudo tee /sys/devices/system/cpu/cpu*/cp |
| `os/parc-ai/parcos-tools.sh` | 100 | echo "No screenshot tool found. Install scrot: sudo apt install scrot" |
| `os/parc-ai/parcos-tools.sh` | 111 | sudo apt autoremove -y 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 112 | sudo apt clean 2>/dev/null \|\| true |
| `os/parc-ai/parcos-tools.sh` | 117 | sudo find /tmp -type f -atime +7 -delete 2>/dev/null \|\| true |
| `os/parc-ai/modules/agent-browser.sh` | 243 | sudo apt install -y mpv &>/dev/null |
| `os/parc-ai/modules/agent-system.sh` | 35 | # Execute with sudo |
| `os/parc-ai/modules/codegen.sh` | 131 | suggestions.append('Insufficient permissions — try sudo or check file  |
| `os/parc-ai/modules/device.sh` | 179 | echo "No screenshot tool found. Install: sudo apt install scrot" |
| `os/parc-ai/modules/knowledge-parcos.sh` | 252 | - Restart: sudo systemctl restart NetworkManager |
| `os/parc-ai/modules/knowledge-parcos.sh` | 257 | - Restart: sudo systemctl restart bluetooth |
| `os/parc-ai/modules/knowledge-parcos.sh` | 272 | - Reinstall: sudo apt reinstall package-name |
| `os/parc-ai/modules/multimodal.sh` | 55 | echo "Install tesseract: sudo apt install tesseract-ocr" |
| `os/parc-ai/modules/multimodal.sh` | 142 | echo "No audio tools found. Install: sudo apt install ffmpeg" |
| `os/parc-ai/modules/multimodal.sh` | 156 | echo "No TTS engine found. Install: sudo apt install espeak-ng" |
| `os/parc-ai/modules/parcos-features.sh` | 50 | sudo apt-get clean 2>/dev/null |
| `os/parc-ai/modules/parcos-features.sh` | 57 | sudo apt-get update && sudo apt-get upgrade -y |
| `os/parc-ai/modules/travel.sh` | 122 | print('(For real-time weather, install wttr: sudo apt install wttr)') |
| `os/systemd/install-services.sh` | 23 | sudo systemctl daemon-reload |
| `os/systemd/install-services.sh` | 26 | sudo systemctl enable korrinos-desktop.service |
| `os/systemd/install-services.sh` | 27 | sudo systemctl enable korrinos-monitor.service |
| `os/systemd/install-services.sh` | 28 | sudo systemctl enable korrinos-heal.service |
| `os/systemd/install-services.sh` | 29 | sudo systemctl enable korrinos-power.service |
| `os/systemd/install-services.sh` | 30 | sudo systemctl enable korrinos-backup.timer |
| `os/systemd/install-services.sh` | 31 | sudo systemctl enable korrinos-update.timer |
| `os/systemd/install-services.sh` | 32 | sudo systemctl enable korrinos-cleanup.timer |
| `os/systemd/install-services.sh` | 37 | echo "  sudo systemctl start korrinos-desktop" |
| `os/systemd/install-services.sh` | 38 | echo "  sudo systemctl start korrinos-monitor" |
| `os/systemd/install-services.sh` | 39 | echo "  sudo systemctl start korrinos-heal" |
| `os/systemd/install-services.sh` | 40 | echo "  sudo systemctl start korrinos-power" |
| `os/systemd/install-services.sh` | 43 | echo "  sudo systemctl enable korrinos-desktop" |
| `os/systemd/install-services.sh` | 44 | echo "  sudo systemctl enable korrinos-monitor" |
| `os/systemd/install-services.sh` | 45 | echo "  sudo systemctl enable korrinos-heal" |
| `os/systemd/install-services.sh` | 46 | echo "  sudo systemctl enable korrinos-power" |
| `os/security/biometric.sh` | 23 | sudo apt install -y fprintd libpam-fprintd |
| `os/security/biometric.sh` | 44 | sudo howdy add |
| `os/security/firewall.sh` | 14 | sudo ufw --force enable |
| `os/security/firewall.sh` | 15 | sudo ufw default deny incoming |
| `os/security/firewall.sh` | 16 | sudo ufw default allow outgoing |
| `os/security/firewall.sh` | 17 | sudo ufw deny from any to any port 23 |
| `os/security/firewall.sh` | 18 | sudo ufw deny from any to any port 113 |
| `os/security/firewall.sh` | 21 | sudo firewall-cmd --permanent --set-default-zone=drop |
| `os/security/firewall.sh` | 22 | sudo firewall-cmd --reload |
| `os/security/firewall.sh` | 26 | sudo apt install -y ufw |
| `os/security/firewall.sh` | 49 | sudo ufw status verbose |
| `os/security/firewall.sh` | 58 | sudo ufw status numbered |
| `os/security/security.sh` | 52 | if sudo ufw status 2>/dev/null \| grep -q "active"; then |
| `os/security/sip.sh` | 18 | "/usr/bin/sudo" "/usr/bin/passwd" |
| `os/system/auto-updates.sh` | 47 | sudo apt update -qq 2>/dev/null |
| `os/system/auto-updates.sh` | 52 | sudo pacman -Sy --quiet 2>/dev/null |
| `os/system/auto-updates.sh` | 68 | sudo apt upgrade -y -qq |
| `os/system/auto-updates.sh` | 70 | sudo dnf upgrade -y -q |
| `os/system/auto-updates.sh` | 72 | sudo pacman -Syu --noconfirm --quiet |
| `os/system/auto-updates.sh` | 84 | sudo apt upgrade -y -qq -o Dir::Etc::SourceList=/etc/apt/sources.list. |
| `os/system/auto-updates.sh` | 86 | sudo dnf upgrade --security -y -q |
| `os/system/auto-updates.sh` | 148 | sudo reboot |
| `os/system/auto-updates.sh` | 189 | sudo apt-get dselect-upgrade -y |
| `os/system/backup-restore.sh` | 151 | sudo apt-get dselect-upgrade -y |
| `os/system/bluetooth-manager.sh` | 69 | echo "Install with: sudo apt install bluez bluez-tools" |
| `os/system/bluetooth-manager.sh` | 75 | sudo systemctl start bluetooth |
| `os/system/content-filter.sh` | 50 | sudo cp /etc/hosts /etc/hosts.tinkos-cf.bak 2>/dev/null \|\| true |
| `os/system/content-filter.sh` | 54 | sudo cp /tmp/tinkos-hosts /etc/hosts |
| `os/system/content-filter.sh` | 63 | sudo cp /etc/hosts.tinkos-cf.bak /etc/hosts 2>/dev/null \|\| sudo sed -i |
| `os/system/fast-boot.sh` | 47 | sudo sed -i "s/GRUB_TIMEOUT=.*/GRUB_TIMEOUT=$timeout/" /etc/default/gr |
| `os/system/fast-boot.sh` | 48 | sudo update-grub 2>/dev/null \|\| sudo grub-mkconfig -o /boot/grub/grub. |
| `os/system/fast-boot.sh` | 58 | sudo mkdir -p /etc/systemd/system.conf.d/ |
| `os/system/fast-boot.sh` | 64 | sudo mv /tmp/parallel-boot.conf /etc/systemd/system.conf.d/ |
| `os/system/fast-boot.sh` | 91 | sudo journalctl --vacuum-time=3d |
| `os/system/fast-boot.sh` | 92 | sudo apt clean 2>/dev/null \|\| true |
| `os/system/fast-boot.sh` | 93 | sudo pacman -Sc --noconfirm 2>/dev/null \|\| true |
| `os/system/fast-boot.sh` | 103 | sudo apt install -y preload 2>/dev/null \|\| true |
| `os/system/flatpak-support.sh` | 19 | sudo apt install -y flatpak |
| `os/system/flatpak-support.sh` | 21 | sudo dnf install -y flatpak |
| `os/system/flatpak-support.sh` | 23 | sudo pacman -S --noconfirm flatpak |
| `os/system/flatpak-support.sh` | 32 | sudo apt install -y gnome-software-plugin-flatpak 2>/dev/null \|\| \ |
| `os/system/flatpak-support.sh` | 33 | sudo apt install -y plasma-discover-backend-flatpak 2>/dev/null \|\| tru |
| `os/system/gamemode-setup.sh` | 28 | sudo apt install -y -qq gamemode gamemode-daemon 2>/dev/null \| tail -1 |
| `os/system/gamemode-setup.sh` | 29 | sudo apt install -y -qq gamemode 2>/dev/null \| tail -1 |
| `os/system/gamemode-setup.sh` | 31 | sudo dnf install -y -q gamemode 2>/dev/null \| tail -1 |
| `os/system/gamemode-setup.sh` | 33 | sudo pacman -S --noconfirm gamemode 2>/dev/null \| tail -1 |
| `os/system/gamemode-setup.sh` | 119 | sudo systemctl enable gamemode 2>/dev/null && log "gamemode system ser |
| `os/system/gaming-meta.sh` | 40 | sudo apt update |
| `os/system/gaming-meta.sh` | 41 | sudo apt install -y steam lutris wine mangohud gamemode gamescope |
| `os/system/gaming-meta.sh` | 55 | sudo apt remove -y steam-installer steam lutris wine mangohud gamemode |
| `os/system/gpu-config.sh` | 40 | sudo apt update |
| `os/system/gpu-config.sh` | 41 | sudo apt install -y nvidia-driver nvidia-cuda-toolkit nvidia-settings |
| `os/system/gpu-config.sh` | 43 | sudo dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda |
| `os/system/gpu-config.sh` | 45 | sudo pacman -S --noconfirm nvidia nvidia-utils nvidia-settings |
| `os/system/gpu-config.sh` | 55 | sudo apt update |
| `os/system/gpu-config.sh` | 56 | sudo apt install -y mesa-vulkan-drivers libvdpau-va-gl1 firmware-amd-g |
| `os/system/gpu-config.sh` | 58 | sudo dnf install -y mesa-vulkan-drivers mesa-va-drivers xorg-x11-drv-a |
| `os/system/gpu-config.sh` | 60 | sudo pacman -S --noconfirm mesa vulkan-radeon libva-mesa-driver xf86-v |
| `os/system/gpu-config.sh` | 70 | sudo apt update |
| `os/system/gpu-config.sh` | 71 | sudo apt install -y intel-media-va-driver mesa-vulkan-drivers firmware |
| `os/system/gpu-config.sh` | 73 | sudo dnf install -y intel-media-driver mesa-vulkan-drivers |
| `os/system/gpu-config.sh` | 75 | sudo pacman -S --noconfirm intel-media-driver vulkan-intel |
| `os/system/gpu-config.sh` | 85 | sudo nvidia-smi -pm 1 2>/dev/null \|\| true |
| `os/system/gpu-config.sh` | 88 | sudo nvidia-smi -pl 100 2>/dev/null \|\| true |
| `os/system/gpu-config.sh` | 98 | echo "auto" \| sudo tee /sys/class/drm/card0/device/power_dpm_force_per |
| `os/system/gpu-config.sh` | 109 | echo "1" \| sudo tee /sys/class/drm/card0/device/power_dpm_rc6_enable |
| `os/system/hardware-detect.sh` | 85 | sudo apt update -qq 2>/dev/null |
| `os/system/hardware-detect.sh` | 86 | sudo apt install -y -qq \ |
| `os/system/hardware-detect.sh` | 97 | sudo dpkg --add-architecture i386 |
| `os/system/hardware-detect.sh` | 98 | sudo apt update -qq 2>/dev/null |
| `os/system/hardware-detect.sh` | 99 | sudo apt install -y -qq \ |
| `os/system/hardware-detect.sh` | 112 | sudo dpkg --add-architecture i386 |
| `os/system/hardware-detect.sh` | 113 | sudo apt update -qq 2>/dev/null |
| `os/system/hardware-detect.sh` | 114 | sudo apt install -y -qq \ |
| `os/system/hardware-detect.sh` | 125 | sudo apt install -y -qq nvidia-prime switcharoo 2>/dev/null \| tail -1 |
| `os/system/hardware-detect.sh` | 134 | sudo apt install -y -qq mesa-amdgpu-drivers 2>/dev/null \| tail -1 |
| `os/system/hardware-detect.sh` | 144 | sudo apt install -y -qq mesa-utils libgl1-mesa-dri 2>/dev/null \| tail  |
| `os/system/hardware-detect.sh` | 153 | sudo apt install -y -qq \ |
| `os/system/install-greetings.sh` | 12 | sudo mkdir -p /opt/korrinos/os/system |
| `os/system/install-greetings.sh` | 26 | sudo tee /etc/systemd/system/korrinos-greet-boot.service > /dev/null < |
| `os/system/install-greetings.sh` | 46 | sudo tee /etc/systemd/system/korrinos-greet-wake.service > /dev/null < |
| `os/system/install-greetings.sh` | 64 | sudo tee /etc/systemd/system/korrinos-greet-shutdown.service > /dev/nu |
| `os/system/install-greetings.sh` | 84 | sudo tee /etc/plymouth/boot-greeting.sh > /dev/null << 'EOF' |
| `os/system/install-greetings.sh` | 110 | sudo chmod +x /etc/plymouth/boot-greeting.sh |
| `os/system/install-greetings.sh` | 115 | sudo tee /etc/profile.d/korrinos-greeting.sh > /dev/null << 'EOF' |
| `os/system/install-greetings.sh` | 121 | sudo chmod +x /etc/profile.d/korrinos-greeting.sh |
| `os/system/install-greetings.sh` | 124 | sudo systemctl daemon-reload |
| `os/system/install-greetings.sh` | 125 | sudo systemctl enable korrinos-greet-boot.service 2>/dev/null \|\| true |
| `os/system/install-greetings.sh` | 126 | sudo systemctl enable korrinos-greet-wake.target 2>/dev/null \|\| true |
| `os/system/installer.sh` | 162 | sudo apt-get update -qq |
| `os/system/installer.sh` | 163 | sudo apt-get install -y -qq debootstrap |
| `os/system/installer.sh` | 168 | sudo mkdir -p /mnt/proc /mnt/sys /mnt/dev /mnt/run |
| `os/system/installer.sh` | 169 | sudo mount --bind /proc /mnt/proc |
| `os/system/installer.sh` | 170 | sudo mount --bind /sys /mnt/sys |
| `os/system/installer.sh` | 171 | sudo mount --bind /dev /mnt/dev |
| `os/system/installer.sh` | 172 | sudo mount --bind /run /mnt/run |
| `os/system/installer.sh` | 175 | echo "korrinos" \| sudo tee /mnt/etc/hostname >/dev/null |
| `os/system/installer.sh` | 176 | sudo systemd-machine-id-setup --root=/mnt 2>/dev/null \|\| true |
| `os/system/installer.sh` | 180 | \| sudo tee /mnt/etc/fstab >/dev/null |
| `os/system/installer.sh` | 185 | sudo cp -a /home/tinkerspace/linux-kernel/os /mnt/opt/korrinos 2>/dev/ |
| `os/system/installer.sh` | 192 | sudo chroot /mnt /bin/bash -c \ |
| `os/system/installer.sh` | 203 | sudo umount /mnt/proc /mnt/sys /mnt/dev /mnt/run 2>/dev/null \|\| true |
| `os/system/korrinos-errors.sh` | 19 | ERR_FIX[1]="Try: sudo <command> (run as admin) or chmod +x <file> (mak |
| `os/system/korrinos-errors.sh` | 47 | ERR_FIX[30]="Try: sudo mount -o remount,rw <mount_point> to remount as |
| `os/system/korrinos-errors.sh` | 56 | ERR_FIX[127]="Try: sudo apt install <package> to install it, or check  |
| `os/system/korrinos-errors.sh` | 106 | ERR_FIX[100]="Try: sudo apt update && sudo apt install <package>" |
| `os/system/korrinos-errors.sh` | 110 | ERR_FIX[101]="Try: sudo apt --fix-broken install or use --force-unsafe |
| `os/system/korrinos-errors.sh` | 114 | ERR_FIX[102]="Try: sudo apt clean, sudo apt autoremove, or free disk s |
| `os/system/korrinos-errors.sh` | 119 | ERR_FIX[126]="Try: sudo <command> or add yourself to the right group" |
| `os/system/korrinos-errors.sh` | 123 | ERR_FIX[13]="Check: sudo aa-status or audit log for details" |
| `os/system/korrinos-errors.sh` | 132 | ERR_FIX[126]="Try: sudo ubuntu-drivers install or use nomodeset boot o |
| `os/system/korrinos-errors.sh` | 137 | ERR_FIX[12]="Try: free -h, close programs, add swap: sudo fallocate -l |
| `os/system/korrinos-errors.sh` | 287 | sudo tee /etc/systemd/system/korrinos-errors.service > /dev/null << 'E |
| `os/system/korrinos-errors.sh` | 300 | sudo systemctl daemon-reload |
| `os/system/korrinos-errors.sh` | 301 | sudo systemctl enable korrinos-errors.service |
| `os/system/korrinos-mascot.sh` | 127 | sudo tee /etc/systemd/system/korrinos-mascot.service > /dev/null << 'E |
| `os/system/optimization-toggles.sh` | 99 | sudo systemctl stop tlp 2>/dev/null |
| `os/system/optimization-toggles.sh` | 103 | sudo systemctl start tlp 2>/dev/null |
| `os/system/optimization-toggles.sh` | 111 | sudo sysctl -w vm.swappiness=$value |
| `os/system/optimization-toggles.sh` | 128 | sudo sysctl -w net.core.rmem_max=16777216 |
| `os/system/optimization-toggles.sh` | 129 | sudo sysctl -w net.core.wmem_max=16777216 |
| `os/system/optimization-toggles.sh` | 130 | sudo sysctl -w net.ipv4.tcp_congestion_control=bbr |
| `os/system/power-manager.sh` | 146 | sudo nvidia-smi -pm 1 |
| `os/system/power-manager.sh` | 147 | sudo nvidia-smi -pl 100 |
| `os/system/power-manager.sh` | 175 | sudo nvidia-smi -pm 0 |
| `os/system/power-manager.sh` | 176 | sudo nvidia-smi -pl 100 |
| `os/system/power-manager.sh` | 204 | sudo nvidia-smi -pm 1 |
| `os/system/power-manager.sh` | 205 | sudo nvidia-smi -pl 50 |
| `os/system/power-manager.sh` | 227 | echo $dim \| sudo tee /sys/class/backlight/$backlight/brightness |
| `os/system/rollback-recovery.sh` | 69 | sudo apt-get dselect-upgrade -y |
| `os/system/rollback-recovery.sh` | 139 | sudo grub-install /dev/sda 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 140 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 148 | sudo fsck -f /dev/sda1 2>/dev/null \|\| true |
| `os/system/rollback-recovery.sh` | 151 | sudo -i |
| `os/system/update-system.sh` | 34 | sudo apt update 2>/dev/null |
| `os/system/update-system.sh` | 41 | sudo pacman -Sy 2>/dev/null |
| `os/system/update-system.sh` | 75 | sudo apt upgrade -y 2>/dev/null |
| `os/system/update-system.sh` | 77 | sudo dnf upgrade -y 2>/dev/null |
| `os/system/update-system.sh` | 79 | sudo pacman -Syu --noconfirm 2>/dev/null |
| `os/system/update-system.sh` | 124 | sudo apt install -y firmware-linux firmware-linux-nonfree 2>/dev/null |
| `os/system/update-system.sh` | 126 | sudo dnf install -y linux-firmware 2>/dev/null |
| `os/system/update-system.sh` | 128 | sudo pacman -S --noconfirm linux-firmware 2>/dev/null |
| `os/system/update-system.sh` | 144 | sudo snap refresh 2>/dev/null |
| `os/system/update-system.sh` | 164 | sudo apt autoremove -y |
| `os/system/update-system.sh` | 165 | sudo apt autoclean |
| `os/system/update-system.sh` | 167 | sudo dnf autoremove -y |
| `os/system/update-system.sh` | 168 | sudo dnf clean all |
| `os/system/update-system.sh` | 170 | sudo pacman -Sc --noconfirm |
| `os/system/update-system.sh` | 206 | sudo cp -r $backup_dir/tinker/* /etc/tinker/ 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 270 | sudo add-apt-repository -y ppa:graphics-drivers/ppa 2>/dev/null \|\| tru |
| `os/system/driver-manager/korrinos-drivers.sh` | 271 | sudo apt-get update -qq |
| `os/system/driver-manager/korrinos-drivers.sh` | 286 | sudo systemctl stop gdm3 2>/dev/null \|\| sudo systemctl stop sddm 2>/de |
| `os/system/driver-manager/korrinos-drivers.sh` | 290 | sudo apt-get install -y "nvidia-driver-$recommended" "nvidia-utils-$re |
| `os/system/driver-manager/korrinos-drivers.sh` | 298 | sudo apt-get install -y nvidia-cuda-toolkit nvidia-cudnn 2>/dev/null \| |
| `os/system/driver-manager/korrinos-drivers.sh` | 309 | sudo apt-get install -y vulkan-tools libvulkan1 mesa-vulkan-drivers 2> |
| `os/system/driver-manager/korrinos-drivers.sh` | 318 | sudo apt-get install -y libnvidia-encode-535 libnvidia-decode-535 2>/d |
| `os/system/driver-manager/korrinos-drivers.sh` | 321 | sudo mkdir -p /etc/X11/xorg.conf.d |
| `os/system/driver-manager/korrinos-drivers.sh` | 322 | sudo tee /etc/X11/xorg.conf.d/20-nvidia.conf >/dev/null << 'XEOF' |
| `os/system/driver-manager/korrinos-drivers.sh` | 337 | sudo systemctl start gdm3 2>/dev/null \|\| sudo systemctl start sddm 2>/ |
| `os/system/driver-manager/korrinos-drivers.sh` | 365 | sudo apt-get install -y \ |
| `os/system/driver-manager/korrinos-drivers.sh` | 371 | sudo apt-get install -y vulkan-tools libvulkan1 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 389 | wget -qO - https://repo.radeon.com/rocm/rocm.gpg.key \| sudo apt-key ad |
| `os/system/driver-manager/korrinos-drivers.sh` | 390 | echo "deb [arch=amd64] https://repo.radeon.com/rocm/latest/ ubuntu mai |
| `os/system/driver-manager/korrinos-drivers.sh` | 391 | sudo apt-get update -qq |
| `os/system/driver-manager/korrinos-drivers.sh` | 392 | sudo apt-get install -y rocm-dev 2>&1 \| tail -5 |
| `os/system/driver-manager/korrinos-drivers.sh` | 415 | sudo apt-get install -y \ |
| `os/system/driver-manager/korrinos-drivers.sh` | 427 | sudo gpg --dearmor -o /usr/share/keyrings/intel-archive-keyring.gpg 2> |
| `os/system/driver-manager/korrinos-drivers.sh` | 429 | sudo tee /etc/apt/sources.list.d/intel-oneAPI.list |
| `os/system/driver-manager/korrinos-drivers.sh` | 430 | sudo apt-get update -qq |
| `os/system/driver-manager/korrinos-drivers.sh` | 431 | sudo apt-get install -y intel-oneapi-runtime 2>&1 \| tail -5 |
| `os/system/driver-manager/korrinos-drivers.sh` | 452 | sudo apt-get install -y \ |
| `os/system/driver-manager/korrinos-drivers.sh` | 460 | sudo apt-get install -y realtek-rtl88xxau-dkms 2>/dev/null \|\| { |
| `os/system/driver-manager/korrinos-drivers.sh` | 466 | sudo make install 2>&1 \| tail -3 |
| `os/system/driver-manager/korrinos-drivers.sh` | 472 | sudo apt-get install -y rtl8821ce-dkms 2>/dev/null \|\| { |
| `os/system/driver-manager/korrinos-drivers.sh` | 487 | sudo modprobe -r iwlwifi 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 488 | sudo modprobe iwlwifi 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 489 | sudo modprobe ath9k 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 490 | sudo modprobe rtl8xxxu 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 499 | sudo apt-get install -y fprintd libpam-fprintd 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 504 | sudo apt-get install -y python3-validity 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 505 | sudo systemctl enable --now open-fprintd 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 506 | sudo systemctl enable --now python3-validity 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 512 | sudo apt-get install -y python3-goodix 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 530 | sudo apt-get install -y bluez blueman pulseaudio-module-bluetooth 2>/d |
| `os/system/driver-manager/korrinos-drivers.sh` | 531 | sudo systemctl enable bluetooth 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 532 | sudo systemctl start bluetooth 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 540 | sudo apt-get install -y cups cups-client printer-driver-gutenprinter 2 |
| `os/system/driver-manager/korrinos-drivers.sh` | 541 | sudo systemctl enable cups 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 544 | sudo apt-get install -y \ |
| `os/system/driver-manager/korrinos-drivers.sh` | 551 | sudo apt-get install -y hplip hplip-gui 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 578 | sudo prime-select nvidia 2>/dev/null \|\| { |
| `os/system/driver-manager/korrinos-drivers.sh` | 579 | sudo tee /etc/X11/xorg.conf.d/20-nvidia.conf >/dev/null << 'XEOF' |
| `os/system/driver-manager/korrinos-drivers.sh` | 591 | sudo prime-select intel 2>/dev/null \|\| sudo prime-select off 2>/dev/nu |
| `os/system/driver-manager/korrinos-drivers.sh` | 592 | sudo rm -f /etc/X11/xorg.conf.d/20-nvidia.conf 2>/dev/null \|\| true |
| `os/system/driver-manager/korrinos-drivers.sh` | 596 | sudo prime-select on-demand 2>/dev/null \|\| sudo prime-select hybrid 2> |
| `os/system/driver-manager/korrinos-drivers.sh` | 620 | echo "DKMS not installed. Run: sudo apt install dkms" |
| `os/system/driver-manager/korrinos-drivers.sh` | 630 | sudo dkms autoinstall 2>&1 \| tail -10 |
| `os/system/driver-manager/korrinos-drivers.sh` | 724 | sudo rm -f "/etc/apt/preferences.d/korrinos-pin-$package" |
| `os/system/driver-manager/korrinos-drivers.sh` | 731 | sudo apt-get update -qq 2>/dev/null |
| `os/system/backup/korrinos-backup.sh` | 277 | sudo tee /etc/systemd/system/korrinos-backup.service >/dev/null << EOF |
| `os/system/backup/korrinos-backup.sh` | 292 | sudo tee /etc/systemd/system/korrinos-backup.timer >/dev/null << EOF |
| `os/system/backup/korrinos-backup.sh` | 305 | sudo systemctl daemon-reload |
| `os/system/backup/korrinos-backup.sh` | 306 | sudo systemctl enable korrinos-backup.timer |
| `os/system/backup/korrinos-backup.sh` | 307 | sudo systemctl start korrinos-backup.timer |
| `os/system/package-manager/korrinos-pkg.sh` | 111 | sudo apt-get update -y 2>&1 \| tail -5 |
| `os/system/package-manager/korrinos-pkg.sh` | 118 | sudo dnf check-update -y 2>&1 \| tail -5 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 121 | sudo pacman -Sy --noconfirm 2>&1 \| tail -5 |
| `os/system/package-manager/korrinos-pkg.sh` | 124 | sudo zypper refresh 2>&1 \| tail -5 |
| `os/system/package-manager/korrinos-pkg.sh` | 127 | sudo apk update 2>&1 \| tail -5 |
| `os/system/package-manager/korrinos-pkg.sh` | 130 | sudo xbps-install -Su 2>&1 \| tail -5 |
| `os/system/package-manager/korrinos-pkg.sh` | 175 | sudo apt-get upgrade -y -o Dpkg::Options::="--force-confdef" \ |
| `os/system/package-manager/korrinos-pkg.sh` | 177 | sudo apt-get dist-upgrade -y -o Dpkg::Options::="--force-confdef" \ |
| `os/system/package-manager/korrinos-pkg.sh` | 181 | sudo dnf upgrade -y --allowerasing 2>&1 \| tail -10 |
| `os/system/package-manager/korrinos-pkg.sh` | 184 | sudo pacman -Syu --noconfirm 2>&1 \| tail -10 |
| `os/system/package-manager/korrinos-pkg.sh` | 187 | sudo zypper update -y 2>&1 \| tail -10 |
| `os/system/package-manager/korrinos-pkg.sh` | 190 | sudo apk upgrade 2>&1 \| tail -10 |
| `os/system/package-manager/korrinos-pkg.sh` | 193 | sudo xbps-install -Su 2>&1 \| tail -10 |
| `os/system/package-manager/korrinos-pkg.sh` | 199 | sudo snap refresh 2>&1 \| tail -5 \|\| true |
| `os/system/package-manager/korrinos-pkg.sh` | 209 | apt)    sudo apt-get autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 210 | dnf)    sudo dnf autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 531 | sudo apt-get update -qq 2>/dev/null |
| `os/system/package-manager/korrinos-pkg.sh` | 542 | sudo pacman -Sy 2>/dev/null |
| `os/system/package-manager/korrinos-pkg.sh` | 738 | apt)    sudo apt-get autoremove -y 2>/dev/null ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 739 | dnf)    sudo dnf autoremove -y 2>/dev/null ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 740 | pacman) sudo pacman -Rns $(pacman -Qtdq 2>/dev/null \| tr '\n' ' ') --n |
| `os/system/package-manager/korrinos-pkg.sh` | 759 | apt)    sudo apt-get clean && sudo apt-get autoremove -y 2>/dev/null ; |
| `os/system/package-manager/korrinos-pkg.sh` | 760 | dnf)    sudo dnf clean all 2>/dev/null ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 761 | pacman) sudo pacman -Sc --noconfirm 2>/dev/null ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 762 | zypper) sudo zypper clean --all 2>/dev/null ;; |
| `os/system/package-manager/korrinos-pkg.sh` | 840 | echo "deb $repo_url" \| sudo tee /etc/apt/sources.list.d/korrinos-extra |
| `os/system/package-manager/korrinos-pkg.sh` | 842 | sudo apt-get update -qq |
| `os/system/appstore/korrinos-appstore.sh` | 408 | sudo apt-get install -f -y 2>/dev/null |
| `os/system/security/korrinos-bugfix.sh` | 100 | # Remove unnecessary sudo from non-critical operations |
| `os/system/security/korrinos-bugfix.sh` | 101 | # Keep sudo only for actual system modifications |
| `os/system/cloud-sync/korrinos-cloud.sh` | 92 | sudo apt-get install -y rclone 2>&1 \| tail -3 |
| `os/system/cloud-sync/korrinos-cloud.sh` | 94 | sudo dnf install -y rclone 2>&1 \| tail -3 |
| `os/system/cloud-sync/korrinos-cloud.sh` | 96 | sudo pacman -S --noconfirm rclone 2>&1 \| tail -3 |
| `os/system/cloud-sync/korrinos-cloud.sh` | 98 | curl -s https://rclone.org/install.sh \| sudo bash 2>&1 \| tail -3 |
| `os/system/cloud-sync/korrinos-cloud.sh` | 328 | command -v aws &>/dev/null \|\| { echo "aws-cli not installed. Run: sudo |
| `os/system/cloud-sync/korrinos-cloud.sh` | 572 | sudo tee /etc/systemd/system/korrinos-cloud-sync.service >/dev/null << |
| `os/system/cloud-sync/korrinos-cloud.sh` | 588 | sudo tee /etc/systemd/system/korrinos-cloud-sync.timer >/dev/null << E |
| `os/system/cloud-sync/korrinos-cloud.sh` | 601 | sudo systemctl daemon-reload |
| `os/system/cloud-sync/korrinos-cloud.sh` | 602 | sudo systemctl enable korrinos-cloud-sync.timer |
| `os/system/cloud-sync/korrinos-cloud.sh` | 603 | sudo systemctl start korrinos-cloud-sync.timer |
| `os/system/firewall/korrinos-firewall.sh` | 108 | sudo apt-get install -y ufw 2>&1 \| tail -3 |
| `os/system/firewall/korrinos-firewall.sh` | 124 | sudo ufw default deny routed 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 132 | sudo ufw --force enable 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 138 | sudo nft flush ruleset 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 139 | sudo nft add table inet korrinos 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 140 | sudo nft add chain inet korrinos input '{ type filter hook input prior |
| `os/system/firewall/korrinos-firewall.sh` | 141 | sudo nft add chain inet korrinos output '{ type filter hook output pri |
| `os/system/firewall/korrinos-firewall.sh` | 143 | sudo nft add rule inet korrinos input ct state established,related acc |
| `os/system/firewall/korrinos-firewall.sh` | 145 | sudo nft add rule inet korrinos input iif lo accept 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 275 | sudo ufw status verbose 2>/dev/null |
| `os/system/firewall/korrinos-firewall.sh` | 278 | sudo nft list ruleset 2>/dev/null \| head -30 |
| `os/system/firewall/korrinos-firewall.sh` | 281 | sudo iptables -L -n 2>/dev/null \| head -30 |
| `os/system/firewall/korrinos-firewall.sh` | 345 | sudo ufw limit ssh comment "SSH rate limit" 2>/dev/null |
| `os/system/desktop-env/korrinos-desktop.sh` | 569 | echo "picom not installed. Install: sudo apt install picom" |
| `os/system/update-system/korrinos-update.sh` | 155 | sudo btrfs subvolume snapshot / "/.snapshots/$snap_id" 2>/dev/null \|\|  |
| `os/system/update-system/korrinos-update.sh` | 231 | sudo pacman -S --noconfirm $saved 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 247 | sudo update-grub 2>/dev/null \|\| sudo grub2-mkconfig -o /boot/grub2/gru |
| `os/system/update-system/korrinos-update.sh` | 327 | sudo apt-get update -y 2>&1 \| tail -5 |
| `os/system/update-system/korrinos-update.sh` | 330 | sudo dnf check-update -y 2>&1 \| tail -5 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 333 | sudo pacman -Sy --noconfirm 2>&1 \| tail -5 |
| `os/system/update-system/korrinos-update.sh` | 336 | sudo zypper refresh 2>&1 \| tail -5 |
| `os/system/update-system/korrinos-update.sh` | 339 | sudo apk update 2>&1 \| tail -3 |
| `os/system/update-system/korrinos-update.sh` | 342 | sudo xbps-install -Su 2>&1 \| tail -3 |
| `os/system/update-system/korrinos-update.sh` | 365 | sudo apt-get upgrade -y \ |
| `os/system/update-system/korrinos-update.sh` | 371 | sudo apt-get dist-upgrade -y \ |
| `os/system/update-system/korrinos-update.sh` | 380 | sudo dnf upgrade --security -y 2>&1 \| tail -10 |
| `os/system/update-system/korrinos-update.sh` | 382 | sudo dnf upgrade -y --allowerasing 2>&1 \| tail -10 |
| `os/system/update-system/korrinos-update.sh` | 387 | sudo pacman -Syu --noconfirm 2>&1 \| tail -10 |
| `os/system/update-system/korrinos-update.sh` | 391 | sudo zypper update -y 2>&1 \| tail -10 |
| `os/system/update-system/korrinos-update.sh` | 395 | sudo apk upgrade 2>&1 \| tail -5 |
| `os/system/update-system/korrinos-update.sh` | 399 | sudo xbps-install -Su 2>&1 \| tail -5 |
| `os/system/update-system/korrinos-update.sh` | 428 | sudo snap refresh 2>&1 \| tail -5 \|\| true |
| `os/system/update-system/korrinos-update.sh` | 449 | sudo apt-get install -y linux-image-generic linux-headers-generic 2>&1 |
| `os/system/update-system/korrinos-update.sh` | 453 | echo "Run: sudo apt-get install linux-image-generic linux-headers-gene |
| `os/system/update-system/korrinos-update.sh` | 476 | apt)    sudo apt-get autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 477 | dnf)    sudo dnf autoremove -y 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 478 | pacman) sudo pacman -Sc --noconfirm 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 484 | apt)    sudo apt-get clean 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 485 | dnf)    sudo dnf clean all 2>/dev/null \|\| true ;; |
| `os/system/update-system/korrinos-update.sh` | 524 | sudo shutdown -r +1 "KorrinOS update requires reboot" |
| `os/system/update-system/korrinos-update.sh` | 552 | sudo apt-get update -qq 2>/dev/null |
| `os/system/update-system/korrinos-update.sh` | 568 | sudo pacman -Sy 2>/dev/null |
| `os/system/update-system/korrinos-update.sh` | 611 | sudo tee /etc/systemd/system/korrinos-autoupdate.service >/dev/null << |
| `os/system/update-system/korrinos-update.sh` | 630 | sudo tee /etc/systemd/system/korrinos-autoupdate.timer >/dev/null << T |
| `os/system/update-system/korrinos-update.sh` | 643 | sudo systemctl daemon-reload |
| `os/system/update-system/korrinos-update.sh` | 644 | sudo systemctl enable korrinos-autoupdate.timer |
| `os/system/update-system/korrinos-update.sh` | 645 | sudo systemctl start korrinos-autoupdate.timer |
| `os/system/update-system/korrinos-update.sh` | 653 | sudo systemctl stop korrinos-autoupdate.timer 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 654 | sudo systemctl disable korrinos-autoupdate.timer 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 717 | sudo apt-get install -y "linux-image-$target" "linux-headers-$target"  |
| `os/system/update-system/korrinos-update.sh` | 718 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 721 | echo "Edit /etc/grub.d/40_custom or use: sudo grub-mkconfig -o /boot/g |
| `os/system/update-system/korrinos-update.sh` | 734 | sudo apt-get remove -y "linux-image-$target" "linux-headers-$target" 2 |
| `os/system/update-system/korrinos-update.sh` | 735 | sudo update-grub 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 738 | sudo pacman -R --noconfirm "linux-$target" 2>/dev/null \|\| true |
| `os/system/update-system/korrinos-update.sh` | 812 | sudo apt-get update -qq 2>/dev/null |
| `os/system/update-system/korrinos-update.sh` | 813 | sudo apt-get download $(apt list --upgradable 2>/dev/null \| grep upgra |
| `os/system/update-system/korrinos-update.sh` | 816 | sudo dnf download --resolve $(dnf check-update 2>/dev/null \| awk '{pri |
| `os/system/update-system/korrinos-update.sh` | 832 | echo "Install trickle for bandwidth limiting: sudo apt install trickle |
| `os/system/installer/korrinos-installer.sh` | 541 | - sudo |
| `os/system/installer/korrinos-installer.sh` | 554 | sudoersGroup: sudo |
| `os/system/installer/korrinos-installer.sh` | 659 | echo "Create ISO manually with: sudo xorriso or sudo genisoimage" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 266 | sudo apt-get install -y scrcpy 2>/dev/null \|\| { |
| `os/system/mobile-companion/korrinos-mobile.sh` | 267 | echo "Install manually: sudo apt install scrcpy" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 274 | echo "Install: sudo apt install scrcpy adb" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 285 | echo "ADB not found. Install: sudo apt install adb" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 701 | echo "Install on PC: sudo apt install kdeconnect" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 720 | echo "Install KDE Connect: sudo apt install kdeconnect" |
| `os/system/mobile-companion/korrinos-mobile.sh` | 732 | echo "  sudo apt install adb scrcpy" |
| `os/system/network/korrinos-network.sh` | 184 | sudo resolvconf -a <(echo "nameserver $s") 2>/dev/null \|\| true |
| `os/system/network/korrinos-network.sh` | 209 | sudo sysctl -w net.core.rmem_max=16777216 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 210 | sudo sysctl -w net.core.wmem_max=16777216 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 211 | sudo sysctl -w net.ipv4.tcp_rmem="4096 87380 16777216" 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 212 | sudo sysctl -w net.ipv4.tcp_wmem="4096 65536 16777216" 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 213 | sudo sysctl -w net.core.netdev_max_backlog=5000 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 218 | sudo sysctl -w net.ipv4.tcp_fastopen=3 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 222 | sudo sysctl -w net.ipv4.tcp_keepalive_time=600 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 223 | sudo sysctl -w net.ipv4.tcp_keepalive_intvl=30 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 224 | sudo sysctl -w net.ipv4.tcp_keepalive_probes=5 2>/dev/null |
| `os/system/network/korrinos-network.sh` | 305 | echo "Install speedtest-cli: sudo apt install python3-speedtest-cli" |
| `os/system/enterprise/korrinos-enterprise.sh` | 101 | "default_groups": ["sudo", "docker", "video", "audio"], |
| `os/system/enterprise/korrinos-enterprise.sh` | 158 | sudo apt-get install -y realmd sssd sssd-tools adcli krb5-user \ |
| `os/system/enterprise/korrinos-enterprise.sh` | 181 | sudo tee /etc/sssd/sssd.conf >/dev/null << SSSDEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 185 | services = nss, pam, ssh, sudo |
| `os/system/enterprise/korrinos-enterprise.sh` | 213 | sudo chmod 600 /etc/sssd/sssd.conf |
| `os/system/enterprise/korrinos-enterprise.sh` | 215 | # Configure SSSD for sudo |
| `os/system/enterprise/korrinos-enterprise.sh` | 216 | sudo tee /etc/sudoers.d/domain-admins >/dev/null << 'SUDOEOF' |
| `os/system/enterprise/korrinos-enterprise.sh` | 219 | sudo chmod 440 /etc/sudoers.d/domain-admins |
| `os/system/enterprise/korrinos-enterprise.sh` | 222 | sudo systemctl restart sssd |
| `os/system/enterprise/korrinos-enterprise.sh` | 223 | sudo systemctl enable sssd |
| `os/system/enterprise/korrinos-enterprise.sh` | 226 | sudo pam-auth-update --enable mkhomedir 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 245 | sudo realm leave --verbose 2>&1 \| tail -5 |
| `os/system/enterprise/korrinos-enterprise.sh` | 246 | sudo systemctl stop sssd 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 247 | sudo rm -f /etc/sssd/sssd.conf |
| `os/system/enterprise/korrinos-enterprise.sh` | 265 | sudo apt-get install -y libnss-ldap libpam-ldap ldap-utils sssd sssd-l |
| `os/system/enterprise/korrinos-enterprise.sh` | 268 | sudo tee /etc/nslcd.conf >/dev/null << NSLCDEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 276 | sudo chmod 600 /etc/nslcd.conf |
| `os/system/enterprise/korrinos-enterprise.sh` | 279 | sudo tee /etc/nsswitch.conf >/dev/null << 'NSSWEOF' |
| `os/system/enterprise/korrinos-enterprise.sh` | 296 | sudo tee /etc/sssd/sssd.conf >/dev/null << LDAPSSSDEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 314 | sudo chmod 600 /etc/sssd/sssd.conf |
| `os/system/enterprise/korrinos-enterprise.sh` | 316 | sudo systemctl restart nslcd sssd 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 405 | echo "password requisite pam_pwquality.so minlen=$min_pwd_len retry=3" |
| `os/system/enterprise/korrinos-enterprise.sh` | 423 | sudo tee /etc/krb5.conf >/dev/null << KRBEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 479 | sudo apt-get install -y libpam-google-authenticator 2>/dev/null \|\| tru |
| `os/system/enterprise/korrinos-enterprise.sh` | 491 | echo "auth required pam_google_authenticator.so" \| sudo tee -a /etc/pa |
| `os/system/enterprise/korrinos-enterprise.sh` | 496 | echo "  1. Install: sudo apt install libpam-u2f" |
| `os/system/enterprise/korrinos-enterprise.sh` | 498 | echo "  3. Test with: sudo pamu2fcfg -n" |
| `os/system/enterprise/korrinos-enterprise.sh` | 499 | sudo apt-get install -y libpam-u2f 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 622 | sudo apt-get install -y wireguard 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 627 | sudo chmod 600 /etc/wireguard/wg0.conf |
| `os/system/enterprise/korrinos-enterprise.sh` | 628 | sudo systemctl enable wg-quick@wg0 |
| `os/system/enterprise/korrinos-enterprise.sh` | 633 | sudo apt-get install -y openvpn 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 637 | sudo systemctl enable openvpn@client |
| `os/system/enterprise/korrinos-enterprise.sh` | 642 | sudo apt-get install -y strongswan 2>/dev/null \|\| true |
| `os/system/enterprise/korrinos-enterprise.sh` | 655 | echo "# Split tunnel routes" \| sudo tee -a /etc/wireguard/wg0.conf 2>/ |
| `os/system/enterprise/korrinos-enterprise.sh` | 672 | wireguard)  sudo wg-quick up wg0 ;; |
| `os/system/enterprise/korrinos-enterprise.sh` | 673 | openvpn)    sudo systemctl start openvpn@client ;; |
| `os/system/enterprise/korrinos-enterprise.sh` | 684 | wireguard)  sudo wg-quick down wg0 ;; |
| `os/system/enterprise/korrinos-enterprise.sh` | 685 | openvpn)    sudo systemctl stop openvpn@client ;; |
| `os/system/enterprise/korrinos-enterprise.sh` | 754 | default_groups=$(cfg "provisioning.default_groups" "sudo,docker,video, |
| `os/system/enterprise/korrinos-enterprise.sh` | 827 | sudo tee /etc/security/pwquality.conf >/dev/null << PWEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 840 | sudo sed -i "s/^PASS_MAX_DAYS.*/PASS_MAX_DAYS   $max_age/" /etc/login. |
| `os/system/enterprise/korrinos-enterprise.sh` | 841 | sudo sed -i "s/^PASS_MIN_DAYS.*/PASS_MIN_DAYS   1/" /etc/login.defs 2> |
| `os/system/enterprise/korrinos-enterprise.sh` | 842 | sudo sed -i "s/^PASS_WARN_AGE.*/PASS_WARN_AGE   14/" /etc/login.defs 2 |
| `os/system/enterprise/korrinos-enterprise.sh` | 846 | echo "password requisite pam_pwhistory.so remember=$history" \| sudo te |
| `os/system/enterprise/korrinos-enterprise.sh` | 862 | sudo tee /etc/profile.d/korrinos-session.sh >/dev/null << TMEOF |
| `os/system/enterprise/korrinos-enterprise.sh` | 867 | sudo chmod 644 /etc/profile.d/korrinos-session.sh |
| `os/system/enterprise/korrinos-enterprise.sh` | 889 | admin_users=$(getent group sudo 2>/dev/null \| cut -d: -f4 \| tr ',' '\n |
| `os/system/enterprise/korrinos-enterprise.sh` | 891 | echo "Admin (sudo) users: $admin_users" |
| `os/system/enterprise/korrinos-enterprise.sh` | 902 | sudo iptables -L -n 2>/dev/null \| head -10 \|\| echo "  iptables not con |
| `os/system/enterprise/korrinos-enterprise.sh` | 941 | status=$(sudo realm list 2>/dev/null \| head -5) |
| `os/territories/secure/canary-monitor.sh` | 32 | for f in /etc/passwd /etc/shadow /etc/sudoers /bin/bash /usr/bin/sudo  |
| `os/territories/secure/duress-alert.sh` | 47 | poweroff) sudo systemctl poweroff 2>/dev/null \|\| sudo poweroff 2>/dev/ |
| `os/territories/secure/privacy-ledger.sh` | 40 | sudo journalctl -k 2>/dev/null \| grep -iE "avc\|apparmor\|DENIED\|audit"  |
| `os/territories/secure/privacy-ledger.sh` | 41 | sudo grep -iE "audit\|denied" /var/log/kern.log 2>/dev/null \| tail -15  |
| `os/territories/secure/sip-guard.sh` | 51 | for f in /etc/passwd /etc/shadow /etc/sudoers /bin/bash /usr/bin/sudo; |
| `os/territories/secure/traffic-guard.sh` | 21 | sudo timeout 5 nethogs -c 3 2>/dev/null \| head -20 \|\| true |
| `os/territories/secure/vault-engine.sh` | 42 | sudo cryptsetup close "tinker_$name" 2>/dev/null \|\| true |
| `os/territories/lib/common.sh` | 30 | echo "This operation requires root. Re-run with sudo: ${0##*/} ..." |
| `os/territories/hack/ephemeral-ram.sh` | 23 | if has sudo; then |
| `os/territories/hack/ephemeral-ram.sh` | 47 | echo 0 \| sudo tee /proc/sys/vm/swapiness >/dev/null 2>&1 \|\| true |
| `os/territories/hack/ephemeral-ram.sh` | 48 | sudo swapoff -a 2>/dev/null && echo "All swap disabled (RAM-world cann |
| `os/territories/hack/intent-hardware.sh` | 45 | sudo modprobe -r uvcvideo 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 50 | sudo modprobe uvcvideo 2>/dev/null \|\| true |
| `os/territories/hack/intent-hardware.sh` | 74 | [ -e /dev/video-loopback0 ] \|\| sudo modprobe v4l2loopback video_nr=10  |
| `os/territories/hack/kill-switch.sh` | 34 | echo "  Run commands inside: sudo ip netns exec $ns <cmd>" |
| `os/territories/hack/kill-switch.sh` | 66 | sudo nmap -sn 192.168.1.0/24 2>/dev/null \| head -20 \|\| true |
| `os/territories/hack/masked-proc.sh` | 54 | sudo unshare --mount bash -c " |
| `os/territories/hack/panic-wipe.sh` | 62 | sudo systemctl poweroff 2>/dev/null \|\| sudo poweroff 2>/dev/null \|\| tr |
| `os/territories/hack/reverse-proxy.sh` | 23 | sudo ip rule add fwmark $MARK table $TABLE pref 100 2>/dev/null \|\| tru |
| `os/territories/hack/reverse-proxy.sh` | 40 | sudo iptables -t mangle -A OUTPUT -m mark --mark $MARK -j DROP 2>/dev/ |
| `os/territories/hack/reverse-proxy.sh` | 47 | has tor && { echo "Starting tor service"; sudo systemctl start tor 2>/ |
| `os/territories/hack/reverse-proxy.sh` | 54 | sudo ip rule show 2>/dev/null \| grep -E "0x\|$TABLE" \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 55 | sudo ip route show table $TABLE 2>/dev/null \| head \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 56 | sudo iptables -t mangle -L OUTPUT 2>/dev/null \| grep -i mark \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 62 | sudo ip rule del fwmark $MARK 2>/dev/null \|\| true |
| `os/territories/hack/reverse-proxy.sh` | 63 | sudo iptables -t mangle -D OUTPUT -m mark --mark $MARK -j DROP 2>/dev/ |
| `os/territories/hack/sdr-isolation.sh` | 63 | sudo hcitool lescan --passive --duplicates 2>&1 \| head -20 \|\| true |
| `os/territories/hack/sdr-isolation.sh` | 72 | sudo systemctl is-active bluetooth 2>/dev/null \| sed 's/^/  bluetooth: |
| `os/territories/hack/split-personality.sh` | 57 | echo "Opened as /dev/mapper/$name (mount with: sudo mount ...)" |
| `os/territories/hack/split-personality.sh` | 79 | sudo cryptsetup status 2>/dev/null \|\| lsblk \| grep -i crypt \|\| true |
| `os/territories/hack/supply-chain.sh` | 70 | sudo unshare --mount --pid --net --fork --kill-child \ |
| `os/branding/install-branding.sh` | 16 | S=sudo |
| `os/branding/install-branding.sh` | 25 | sudo plymouth-set-default-theme korrinosplymouth && echo "  default sp |
| `os/branding/install-branding.sh` | 37 | sudo sed -i 's\|^#*GRUB_THEME=.*\|GRUB_THEME="/boot/grub/themes/korrinos |
| `os/branding/install-branding.sh` | 38 | sudo update-grub 2>/dev/null \| tail -1 \|\| true |
| `os/data/user-profiles.sh` | 89 | sudo apt update |
| `os/data/user-profiles.sh` | 90 | sudo apt install -y $packages |
| `os/data/user-profiles.sh` | 92 | sudo dnf install -y $packages |
| `os/data/user-profiles.sh` | 94 | sudo pacman -S --noconfirm $packages |
| `os/ai/voice-assistant.sh` | 25 | ["shutdown"]="sudo shutdown -h 1" |
| `os/ai/voice-assistant.sh` | 26 | ["reboot"]="sudo reboot" |
| `os/ai/voice-engine.sh` | 24 | ["shutdown"]="sudo shutdown -h 1" |
| `os/ai/voice-engine.sh` | 25 | ["reboot"]="sudo reboot" |
| `os/drivers/driver-manager.sh` | 95 | sudo apt install -y nvidia-driver nvidia-cuda-toolkit |
| `os/drivers/driver-manager.sh` | 97 | sudo dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda |
| `os/drivers/driver-manager.sh` | 99 | sudo pacman -S --noconfirm nvidia nvidia-utils |
| `os/drivers/driver-manager.sh` | 109 | sudo apt install -y mesa-vulkan-drivers libvdpau-va-gl1 |
| `os/drivers/driver-manager.sh` | 111 | sudo dnf install -y mesa-vulkan-drivers mesa-va-drivers |
| `os/drivers/driver-manager.sh` | 113 | sudo pacman -S --noconfirm mesa vulkan-radeon libva-mesa-driver |
| `os/drivers/driver-manager.sh` | 123 | sudo apt install -y intel-media-va-driver mesa-vulkan-drivers |
| `os/drivers/driver-manager.sh` | 125 | sudo dnf install -y intel-media-driver mesa-vulkan-drivers |
| `os/drivers/driver-manager.sh` | 127 | sudo pacman -S --noconfirm intel-media-driver vulkan-intel |
| `os/drivers/driver-manager.sh` | 137 | sudo apt install -y firmware-iwlwifi firmware-realtek |
| `os/drivers/driver-manager.sh` | 139 | sudo dnf install -y linux-firmware |
| `os/drivers/driver-manager.sh` | 141 | sudo pacman -S --noconfirm linux-firmware |
| `os/drivers/driver-manager.sh` | 151 | sudo apt install -y bluez bluez-tools |
| `os/drivers/driver-manager.sh` | 153 | sudo dnf install -y bluez bluez-tools |
| `os/drivers/driver-manager.sh` | 155 | sudo pacman -S --noconfirm bluez bluez-utils |
| `os/drivers/driver-manager.sh` | 267 | sudo apt update && sudo apt upgrade -y |
| `os/drivers/driver-manager.sh` | 269 | sudo dnf upgrade -y |
| `os/drivers/driver-manager.sh` | 271 | sudo pacman -Syu --noconfirm |
| `os/brand/install-boot-intro.sh` | 64 | install\|apply\|on) need_root && install_theme \|\| echo "Re-run with sudo |
| `os/brand/install-boot-intro.sh` | 65 | uninstall\|remove\|off) need_root && uninstall_theme \|\| echo "Re-run wit |
| `os/brand/install-gdm-skin.sh` | 49 | echo "KorrinOS GDM login skin installed. (Restart gdm3 to apply: sudo  |
| `os/brand/install-gdm-skin.sh` | 78 | install\|apply\|on) need_root && install_theme \|\| echo "Re-run with sudo |
| `os/brand/install-gdm-skin.sh` | 79 | uninstall\|remove\|off) need_root && uninstall_theme \|\| echo "Re-run wit |
| `os/brand/install-grub-theme.sh` | 51 | install\|apply\|on) need_root && install_theme \|\| echo "Re-run with sudo |
| `os/brand/install-grub-theme.sh` | 52 | uninstall\|remove\|off) need_root && uninstall_theme \|\| echo "Re-run wit |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 247 | # Try with sudo |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 249 | subprocess.run(["sudo", "sh", "-c", f"echo {freq_khz * 1000} > {pwm_fr |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 253 | print(f"  Set {pwm_id}: {freq_khz}kHz (via sudo)") |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 183 | result = subprocess.run(["sudo", "rdmsr", "0xC8F"], capture_output=Tru |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 383 | subprocess.run(["sudo", "wrmsr", hex(msr), hex(mask)], |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 90 | sudo dmidecode -s system-serial-number 2>/dev/null \| head -1 \| sed 's/ |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 198 | sudo systemd-resolve --flush-caches 2>/dev/null && echo "  systemd-res |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 199 | sudo resolvectl flush-caches 2>/dev/null && echo "  resolvectl: flushe |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 200 | sudo /etc/init.d/dns-clean 2>/dev/null |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 207 | sudo swapoff -a 2>/dev/null && sudo swapon -a 2>/dev/null && echo "  S |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 213 | sudo journalctl --vacuum-time=1h 2>/dev/null && echo "  journalctl: va |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 214 | sudo find /var/log -name "*.log" -mmin -1440 -exec truncate -s 0 {} \; |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 185 | subprocess.run(["sudo", "tee", "/sys/kernel/debug/ec/0/io"], |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 11 | perms(){ echo "=== Permission Model ==="; echo "  Tier 1 (read-only):  |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 297 | subprocess.run(["sudo", "kill", "-SIGSTOP", pid], |
| `os/hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 454 | subprocess.run(["sudo", "taskset", "-p", hex(mask), pid], |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 14 | cores) for i in $(seq ${2:-3} ${3:-7}); do echo 0 \| sudo tee /sys/devi |
| `os/hardware-tech/hardware-tuning/led-tuning.sh` | 15 | echo $val \| sudo tee /sys/class/leds/$led/brightness > /dev/null 2>&1  |
| `os/hardware-tech/hardware-tuning/led-tuning.sh` | 17 | [ -f "/sys/devices/platform/leds/leds/rgb:kbd/mode" ] && echo ${2:-sta |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 12 | swappiness) echo ${2:-60} \| sudo tee /proc/sys/vm/swappiness > /dev/nu |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 13 | hugepages) echo ${2:-1024} \| sudo tee /proc/sys/vm/nr_hugepages > /dev |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 14 | ksm) echo ${2:-1} \| sudo tee /sys/kernel/mm/ksm/run > /dev/null 2>&1 & |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 15 | zram) sudo modprobe zram 2>/dev/null && sudo zramctl /dev/zram0 --algo |
| `os/hardware-tech/hardware-tuning/usb-tuning.sh` | 37 | echo ${2:-500} \| sudo tee /sys/bus/usb/devices/*/power/max_power > /de |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 225 | subprocess.run(["sudo", "tee", pwm], input=b"1", capture_output=True,  |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 289 | subprocess.run(["sudo", "tee", pwm_file], |
| `os/languages/install.sh` | 23 | echo "Warning: GCC not found. Install with: sudo apt install gcc" |
| `os/desktop/gestures.sh` | 68 | sudo apt install -y libinput-tools |
| `os/desktop/gestures.sh` | 70 | sudo dnf install -y libinput |
| `os/desktop/gestures.sh` | 72 | sudo pacman -S --noconfirm libinput |
| `os/desktop/gestures.sh` | 77 | sudo libinput debug-events --device /dev/input/event* 2>/dev/null \| wh |
| `os/desktop/settings-gui.sh` | 281 | sudo systemctl mask sleep.target suspend.target hibernate.target hybri |
| `os/desktop/settings-gui.sh` | 307 | sudo modprobe -r uvcvideo 2>/dev/null \|\| true; save_setting PRIVACY_CA |
| `os/desktop/settings-gui.sh` | 309 | sudo modprobe uvcvideo 2>/dev/null \|\| true; save_setting PRIVACY_CAMER |
| `os/desktop/settings-gui.sh` | 314 | sudo modprobe -r snd_usb_audio snd_hda_intel 2>/dev/null \|\| true; save |
| `os/desktop/settings-gui.sh` | 316 | sudo modprobe snd_usb_audio snd_hda_intel 2>/dev/null \|\| true; save_se |
| `os/desktop/settings-gui.sh` | 333 | echo "  Firewall: $(command -v ufw >/dev/null 2>&1 && sudo ufw status  |
| `os/desktop/settings-gui.sh` | 344 | sudo ufw enable 2>/dev/null; echo "Firewall enabled" |
| `os/desktop/settings-gui.sh` | 349 | (command -v apt >/dev/null 2>&1 && sudo apt update -qq && sudo apt upg |
| `os/desktop/settings-gui.sh` | 350 | (command -v dnf >/dev/null 2>&1 && sudo dnf upgrade -y -q) |
| `os/desktop/settings-gui.sh` | 449 | sudo timedatectl set-ntp true 2>/dev/null |
| `os/desktop/settings-gui.sh` | 450 | sudo systemctl restart systemd-timesyncd 2>/dev/null |
| `os/desktop/shortcuts.sh` | 130 | echo "xbindkeys not installed — install with: sudo apt install xbindke |
| `os/desktop/shortcuts.sh` | 266 | sudo shutdown -h now |
| `os/desktop/shortcuts.sh` | 270 | sudo reboot |
| `os/desktop/shortcuts.sh` | 274 | sudo systemctl suspend |
| `os/desktop/shortcuts.sh` | 281 | echo "xdotool not installed — install with: sudo apt install xdotool" |
| `os/desktop/shortcuts.sh` | 416 | echo $new \| sudo tee /sys/class/backlight/$backlight/brightness |
| `os/desktop/text-expander.sh` | 110 | echo "xdotool not installed — install with: sudo apt install xdotool" |

### C13 echo -e (non-portable) — 530

| file | line | detail |
|------|------|--------|
| `os/install-parcai.sh` | 18 | log() { echo -e "${GREEN}[VOKK v4]${NC} $1"; } |
| `os/install-parcai.sh` | 19 | warn() { echo -e "${YELLOW}[VOKK v4]${NC} $1"; } |
| `os/install-parcai.sh` | 20 | fail() { echo -e "${RED}[VOKK v4]${NC} $1"; exit 1; } |
| `os/apps/app-store.sh` | 43 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/apps/app-store.sh` | 44 | echo -e "${BLUE}║                   KORRINOS APP STORE       |
| `os/apps/app-store.sh` | 45 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/apps/app-store.sh` | 50 | echo -e "${YELLOW}Available Apps:${NC}" |
| `os/apps/app-store.sh` | 54 | echo -e "  ${GREEN}${NC} $app - ${APPS[$app]}" |
| `os/apps/app-store.sh` | 56 | echo -e "  ${RED}○${NC} $app - ${APPS[$app]}" |
| `os/apps/app-store.sh` | 91 | echo -e "${YELLOW}Installing $app...${NC}" |
| `os/apps/app-store.sh` | 199 | echo -e "${RED}Unknown app: $app${NC}" |
| `os/apps/app-store.sh` | 205 | echo -e "${GREEN} $app installed successfully!${NC}" |
| `os/apps/app-store.sh` | 207 | echo -e "${RED} Failed to install $app${NC}" |
| `os/apps/app-store.sh` | 214 | echo -e "${YELLOW}Uninstalling $app...${NC}" |
| `os/apps/app-store.sh` | 222 | *) echo -e "${RED}Unknown app: $app${NC}" ;; |
| `os/apps/app-store.sh` | 226 | echo -e "${GREEN} $app uninstalled!${NC}" |
| `os/apps/app-store.sh` | 255 | echo -e "${RED}Please specify an app to install${NC}" |
| `os/apps/app-store.sh` | 264 | echo -e "${RED}Please specify an app to uninstall${NC}" |
| `os/apps/app-store.sh` | 272 | echo -e "${YELLOW}Updating all apps...${NC}" |
| `os/apps/app-store.sh` | 280 | echo -e "${GREEN} All apps updated!${NC}" |
| `os/apps/app-store.sh` | 287 | echo -e "${YELLOW}Welcome to KorrinOS App Store!${NC}" |
| `os/apps/gaming-mode.sh` | 19 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/apps/gaming-mode.sh` | 20 | echo -e "${BLUE}║                  KORRINOS GAMING MODE      |
| `os/apps/gaming-mode.sh` | 21 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/apps/gaming-mode.sh` | 26 | echo -e "${YELLOW}Enabling Gaming Mode...${NC}" |
| `os/apps/gaming-mode.sh` | 34 | echo -e "  Setting CPU to performance mode..." |
| `os/apps/gaming-mode.sh` | 40 | echo -e "  Disabling screen tearing..." |
| `os/apps/gaming-mode.sh` | 46 | echo -e "  Setting NVIDIA to maximum performance..." |
| `os/apps/gaming-mode.sh` | 53 | echo -e "  Setting AMD to high performance..." |
| `os/apps/gaming-mode.sh` | 58 | echo -e "  Disabling power saving..." |
| `os/apps/gaming-mode.sh` | 64 | echo -e "  Optimizing kernel parameters..." |
| `os/apps/gaming-mode.sh` | 70 | echo -e "  Disabling notifications..." |
| `os/apps/gaming-mode.sh` | 79 | echo -e "${GREEN} Gaming Mode Enabled!${NC}" |
| `os/apps/gaming-mode.sh` | 80 | echo -e "${YELLOW}System optimized for gaming performance.${ |
| `os/apps/gaming-mode.sh` | 92 | echo -e "${YELLOW}Disabling Gaming Mode...${NC}" |
| `os/apps/gaming-mode.sh` | 96 | echo -e "  Restoring CPU governor..." |
| `os/apps/gaming-mode.sh` | 109 | echo -e "  Resetting NVIDIA power mode..." |
| `os/apps/gaming-mode.sh` | 116 | echo -e "  Resetting AMD power state..." |
| `os/apps/gaming-mode.sh` | 121 | echo -e "  Re-enabling power saving..." |
| `os/apps/gaming-mode.sh` | 132 | echo -e "  Re-enabling notifications..." |
| `os/apps/gaming-mode.sh` | 141 | echo -e "${GREEN} Gaming Mode Disabled!${NC}" |
| `os/apps/gaming-mode.sh` | 142 | echo -e "${YELLOW}System restored to normal mode.${NC}" |
| `os/apps/gaming-mode.sh` | 147 | echo -e "${YELLOW}Gaming Mode Status:${NC}" |
| `os/apps/gaming-mode.sh` | 151 | echo -e "  ${GREEN}${NC} Gaming Mode: ${GREEN}ENABLED${NC}" |
| `os/apps/gaming-mode.sh` | 153 | echo -e "  ${RED}${NC} Gaming Mode: ${RED}DISABLED${NC}" |
| `os/apps/gaming-mode.sh` | 159 | echo -e "  CPU Governor: $governor" |
| `os/apps/gaming-mode.sh` | 164 | echo -e "  GPU: NVIDIA (detected)" |
| `os/apps/gaming-mode.sh` | 166 | echo -e "  GPU: Mesa (detected)" |
| `os/apps/gaming-mode.sh` | 174 | echo -e "${YELLOW}Optimizing for: $game${NC}" |
| `os/apps/gaming-mode.sh` | 195 | echo -e "${GREEN} Game optimizations applied!${NC}" |
| `os/apps/gaming-mode.sh` | 229 | echo -e "${RED}Please specify a game to optimize for${NC}" |
| `os/apps/gaming-mode.sh` | 241 | echo -e "${YELLOW}KorrinOS Gaming Mode${NC}" |
| `os/apps/ocr-everywhere.sh` | 20 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/apps/ocr-everywhere.sh` | 21 | echo -e "${BLUE}║                KORRINOS OCR EVERYWHERE     |
| `os/apps/ocr-everywhere.sh` | 22 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/apps/ocr-everywhere.sh` | 39 | echo -e "${YELLOW}Installing OCR dependencies...${NC}" |
| `os/apps/ocr-everywhere.sh` | 52 | echo -e "${YELLOW}Select region to OCR (click and drag)...${ |
| `os/apps/ocr-everywhere.sh` | 63 | echo -e "${GREEN}Extracted text:${NC}" |
| `os/apps/ocr-everywhere.sh` | 70 | echo -e "${GREEN} Copied to clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 75 | echo -e "${RED}No text found${NC}" |
| `os/apps/ocr-everywhere.sh` | 84 | echo -e "${YELLOW}Capturing full screen...${NC}" |
| `os/apps/ocr-everywhere.sh` | 95 | echo -e "${GREEN}Extracted text:${NC}" |
| `os/apps/ocr-everywhere.sh` | 102 | echo -e "${GREEN} Copied to clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 107 | echo -e "${RED}No text found${NC}" |
| `os/apps/ocr-everywhere.sh` | 119 | echo -e "${RED}File not found: $file${NC}" |
| `os/apps/ocr-everywhere.sh` | 123 | echo -e "${YELLOW}Processing: $file${NC}" |
| `os/apps/ocr-everywhere.sh` | 144 | echo -e "${RED}Unsupported file type${NC}" |
| `os/apps/ocr-everywhere.sh` | 150 | echo -e "${GREEN}Extracted text:${NC}" |
| `os/apps/ocr-everywhere.sh` | 157 | echo -e "${GREEN} Copied to clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 162 | echo -e "${RED}No text found${NC}" |
| `os/apps/ocr-everywhere.sh` | 168 | echo -e "${YELLOW}Processing image from clipboard...${NC}" |
| `os/apps/ocr-everywhere.sh` | 180 | echo -e "${GREEN}Extracted text:${NC}" |
| `os/apps/ocr-everywhere.sh` | 187 | echo -e "${GREEN} Copied to clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 192 | echo -e "${RED}No text found${NC}" |
| `os/apps/ocr-everywhere.sh` | 195 | echo -e "${RED}No image in clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 208 | echo -e "${RED}Video not found: $video${NC}" |
| `os/apps/ocr-everywhere.sh` | 212 | echo -e "${YELLOW}Extracting frame from video...${NC}" |
| `os/apps/ocr-everywhere.sh` | 224 | echo -e "${GREEN}Extracted text:${NC}" |
| `os/apps/ocr-everywhere.sh` | 231 | echo -e "${GREEN} Copied to clipboard${NC}" |
| `os/apps/ocr-everywhere.sh` | 236 | echo -e "${RED}No text found${NC}" |
| `os/apps/ocr-everywhere.sh` | 246 | echo -e "${YELLOW}Live OCR Mode (Press Ctrl+C to exit)${NC}" |
| `os/apps/ocr-everywhere.sh` | 263 | echo -e "${YELLOW}OCR History:${NC}" |
| `os/apps/ocr-everywhere.sh` | 287 | echo -e "${YELLOW}Setting OCR language: $lang${NC}" |
| `os/apps/ocr-everywhere.sh` | 291 | echo -e "${YELLOW}Installing language pack...${NC}" |
| `os/apps/ocr-everywhere.sh` | 302 | echo -e "${GREEN} Language set to: $lang${NC}" |
| `os/apps/ocr-everywhere.sh` | 307 | echo -e "${YELLOW}Available OCR Languages:${NC}" |
| `os/apps/ocr-everywhere.sh` | 391 | echo -e "${YELLOW}KorrinOS OCR Everywhere${NC}" |
| `os/apps/voice-commands.sh` | 20 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/apps/voice-commands.sh` | 21 | echo -e "${BLUE}║              KORRINOS VOICE COMMANDS       |
| `os/apps/voice-commands.sh` | 22 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/apps/voice-commands.sh` | 39 | echo -e "${YELLOW}Installing voice dependencies...${NC}" |
| `os/apps/voice-commands.sh` | 104 | echo -e "${RED}No speech recognition available${NC}" |
| `os/apps/voice-commands.sh` | 137 | echo -e "${YELLOW}Listening for wake word...${NC}" |
| `os/apps/voice-commands.sh` | 150 | echo -e "${GREEN}Wake word detected!${NC}" |
| `os/apps/voice-commands.sh` | 164 | echo -e "${YELLOW}Listening for command...${NC}" |
| `os/apps/voice-commands.sh` | 173 | echo -e "  Heard: ${CYAN}$text${NC}" |
| `os/apps/voice-commands.sh` | 306 | echo -e "${YELLOW}Voice Commands:${NC}" |
| `os/apps/voice-commands.sh` | 340 | echo -e "${YELLOW}Voice Command Mode${NC}" |
| `os/apps/voice-commands.sh` | 350 | echo -e "${GREEN}Recording... (release spacebar to stop)${NC |
| `os/apps/voice-commands.sh` | 367 | echo -e "  Heard: ${CYAN}$text${NC}" |
| `os/apps/voice-commands.sh` | 416 | echo -e "${YELLOW}Testing voice recognition...${NC}" |
| `os/apps/voice-commands.sh` | 423 | echo -e "${GREEN}Recognized: $text${NC}" |
| `os/apps/voice-commands.sh` | 425 | echo -e "${RED}No speech detected${NC}" |
| `os/apps/voice-commands.sh` | 432 | echo -e "${YELLOW}Microphone Calibration${NC}" |
| `os/apps/voice-commands.sh` | 440 | echo -e "${GREEN} Calibration complete!${NC}" |
| `os/apps/voice-commands.sh` | 447 | echo -e "${YELLOW}KorrinOS Voice Commands${NC}" |
| `os/apps/customization/gtk-theme.sh` | 18 | echo -e "[Settings]\ngtk-theme-name=$theme" > ~/.config/gtk- |
| `os/apps/system/cognitive-load.sh` | 46 | echo -e "${BLUE}── KorrinOS Cognitive Load Manager ──${NC}" |
| `os/apps/system/cognitive-load.sh` | 51 | if focused; then echo -e "${GREEN}Focus active — low-urgency |
| `os/apps/system/drag-to-install.sh` | 12 | [ -f "$file" ] \|\| { echo -e "${YELLOW}File not found: $file$ |
| `os/apps/system/drag-to-install.sh` | 30 | echo -e "${YELLOW}Unsupported type .$ext ${NC}"; return 1;; |
| `os/apps/system/drag-to-install.sh` | 32 | echo -e "${GREEN}Done.${NC}" |
| `os/apps/system/drag-to-install.sh` | 35 | echo -e "${BLUE}── KorrinOS One-Step Drag-to-Install ──${NC} |
| `os/apps/system/intent-launcher.sh` | 15 | echo -e "${BLUE}── Intent: $intent ──${NC}" |
| `os/apps/system/intent-launcher.sh` | 35 | echo -e "${YELLOW}No prebuilt canvas for '$intent'${NC}" |
| `os/apps/system/intent-launcher.sh` | 40 | echo -e "${BLUE}── KorrinOS Intent Launcher ──${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 15 | echo -e "${RED}Permission denied${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 19 | echo -e "${RED}Command not found${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 23 | echo -e "${RED}No space left on device${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 26 | echo -e "${RED}Segmentation fault${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 30 | echo -e "${RED}Missing shared library${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 33 | echo -e "${YELLOW}Generic 'not found'${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 36 | echo -e "${RED}Connection refused${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 39 | echo -e "${RED}Process killed (OOM likely)${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 42 | echo -e "${YELLOW}No built-in explanation for: $err${NC}" |
| `os/apps/system/terminal-error-explainer.sh` | 47 | echo -e "${BLUE}── KorrinOS Terminal Error Explainer ──${NC} |
| `os/hyperdrive/hyperdrive-cli.sh` | 14 | echo -e "${C}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 19 | echo -e "${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 29 | echo -e "  Status: ${G}Running${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 31 | echo -e "  Status: ${R}Stopped${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 36 | echo -e "  Module: ${G}Loaded${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 38 | echo -e "  Module: ${Y}Not loaded${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 47 | echo -e "  ${Y}Kernel module not loaded — showing user-space |
| `os/hyperdrive/hyperdrive-cli.sh` | 51 | echo -e "  CPU Usage: ${W}${CPU_USAGE}%${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 60 | echo -e "  zRAM VRAM: ${W}${ZRAM_USED} MB${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 68 | echo -e "  Config: ${W}$HD_CONF${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 70 | echo -e "  Config: ${Y}Not installed${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 79 | echo -e "${G}Starting HyperDrive...${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 84 | echo -e "  ${G}Kernel module loaded${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 89 | echo -e "  ${G}Daemon started${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 94 | echo -e "${R}Stopping HyperDrive...${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 97 | echo -e "  ${R}Daemon stopped${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 107 | echo -e "${R}Usage: hyperdrive boost <pid>${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 116 | echo -e "${G}Boosting render thread PID $pid...${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 121 | echo -e "  ${G}Thread boosted via kernel module${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 125 | echo -e "  ${Y}Thread boosted via nice (kernel module not lo |
| `os/hyperdrive/hyperdrive-cli.sh` | 136 | echo -e "${Y}Configuration not found. Run: sudo hyperdrive i |
| `os/hyperdrive/hyperdrive-cli.sh` | 140 | echo -e "${W}Configuration:${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 150 | echo -e "${M}Applying performance tweaks...${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 154 | echo -e "  ${G}CPU governor: performance${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 158 | echo -e "  ${G}Huge pages: enabled${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 162 | echo -e "  ${G}Memory compaction: triggered${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 167 | echo -e "  ${G}Caches dropped${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 171 | echo -e "  ${G}Transparent huge pages: disabled${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 174 | echo -e "${G}All tweaks applied!${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 182 | echo -e "${M}Running HyperDrive benchmark...${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 186 | echo -e "${B}CPU Performance:${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 191 | echo -e "${B}Memory Bandwidth:${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 198 | echo -e "${B}zRAM Compression:${NC}" |
| `os/hyperdrive/hyperdrive-cli.sh` | 203 | echo -e "${G}Benchmark complete!${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 29 | echo -e "${C}" |
| `os/hyperdrive/hyperdrive.sh` | 34 | echo -e "${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 41 | echo -e "${Y}Detecting hardware...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 47 | echo -e "  CPU: ${W}${CPU_MODEL}${NC} (${CPU_CORES} cores @  |
| `os/hyperdrive/hyperdrive.sh` | 52 | echo -e "  RAM: ${W}${TOTAL_RAM}${NC} total, ${AVAIL_RAM} av |
| `os/hyperdrive/hyperdrive.sh` | 57 | echo -e "  GPU: ${G}${GPU_INFO}${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 60 | echo -e "  GPU: ${R}None detected — HyperDrive will emulate$ |
| `os/hyperdrive/hyperdrive.sh` | 67 | echo -e "  Disk: ${W}${DISK_SPEED}${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 71 | echo -e "  zRAM: ${G}Available${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 74 | echo -e "  zRAM: ${Y}Not available${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 87 | 1) TIER_NAME="Low-end"; echo -e "  Tier: ${R}${TIER_NAME}${N |
| `os/hyperdrive/hyperdrive.sh` | 88 | 2) TIER_NAME="Mid-range"; echo -e "  Tier: ${Y}${TIER_NAME}$ |
| `os/hyperdrive/hyperdrive.sh` | 89 | 3) TIER_NAME="High-end"; echo -e "  Tier: ${G}${TIER_NAME}${ |
| `os/hyperdrive/hyperdrive.sh` | 97 | echo -e "${B}Setting up Adaptive Resolution Scaling...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 137 | echo -e "  ${G}Adaptive Resolution: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 144 | echo -e "${B}Setting up Frame Prediction & Interpolation...$ |
| `os/hyperdrive/hyperdrive.sh` | 173 | echo -e "  ${G}Frame Prediction: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 180 | echo -e "${B}Setting up zRAM VRAM Emulation...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 217 | echo -e "  ${G}zRAM VRAM: configured (${VRAM_SIZE_MB}MB comp |
| `os/hyperdrive/hyperdrive.sh` | 219 | echo -e "  ${Y}zRAM not available — VRAM emulation disabled$ |
| `os/hyperdrive/hyperdrive.sh` | 227 | echo -e "${B}Setting up CPU Micro-Optimization...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 272 | echo -e "  ${G}CPU Optimizer: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 279 | echo -e "${B}Setting up LOD Management...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 321 | echo -e "  ${G}LOD Manager: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 328 | echo -e "${B}Setting up Predictive Resource Caching...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 367 | echo -e "  ${G}Predictive Cache: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 374 | echo -e "${B}Setting up Process Scheduler Optimization...${N |
| `os/hyperdrive/hyperdrive.sh` | 411 | echo -e "  ${G}Scheduler Optimization: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 418 | echo -e "${B}Setting up Shader Pre-compilation...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 454 | echo -e "  ${G}Shader Pre-compilation: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 461 | echo -e "${B}Setting up Memory Defragmentation...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 499 | echo -e "  ${G}Memory Defragmentation: configured${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 506 | echo -e "${G}Starting HyperDrive daemon...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 532 | echo -e "  ${G}HyperDrive daemon started${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 540 | echo -e "${M}Installing HyperDrive v1.0...${NC}" |
| `os/hyperdrive/hyperdrive.sh` | 560 | echo -e "${G}╔══════════════════════════════════════════════ |
| `os/hyperdrive/hyperdrive.sh` | 561 | echo -e "${G}║  HyperDrive v1.0 installed successfully!      |
| `os/hyperdrive/hyperdrive.sh` | 562 | echo -e "${G}║                                               |
| `os/hyperdrive/hyperdrive.sh` | 563 | echo -e "${G}║  Status:  systemctl status hyperdrive         |
| `os/hyperdrive/hyperdrive.sh` | 564 | echo -e "${G}║  Logs:    journalctl -u hyperdrive -f         |
| `os/hyperdrive/hyperdrive.sh` | 565 | echo -e "${G}║  Config:  /etc/hyperdrive/hyperdrive.conf     |
| `os/hyperdrive/hyperdrive.sh` | 566 | echo -e "${G}╚══════════════════════════════════════════════ |
| `os/parc-ai/install-parcos.sh` | 12 | log()  { echo -e "${BLUE}[korrinos]${NC} $1"; } |
| `os/parc-ai/install-parcos.sh` | 13 | ok()   { echo -e "${GREEN}[]${NC} $1"; } |
| `os/parc-ai/install-parcos.sh` | 14 | fail() { echo -e "${RED}[]${NC} $1"; exit 1; } |
| `os/parc-ai/install-parcos.sh` | 96 | echo -e "${GREEN}  VOKK v4 installed successfully!${NC}" |
| `os/parc-ai/parcai-model-pipeline.sh` | 25 | log() { echo -e "${BLUE}[$(date +%H:%M:%S)]${NC} $1"; } |
| `os/parc-ai/parcai-model-pipeline.sh` | 26 | success() { echo -e "${GREEN}[]${NC} $1"; } |
| `os/parc-ai/parcai-model-pipeline.sh` | 27 | warn() { echo -e "${YELLOW}[!]${NC} $1"; } |
| `os/parc-ai/parcai-model-pipeline.sh` | 28 | fail() { echo -e "${RED}[]${NC} $1"; exit 1; } |
| `os/parc-ai/parcos-ai-installer.sh` | 16 | log()  { echo -e "${BLUE}[korrinos-ai]${NC} $1"; } |
| `os/parc-ai/parcos-ai-installer.sh` | 17 | ok()   { echo -e "${GREEN}[]${NC} $1"; } |
| `os/parc-ai/parcos-ai-installer.sh` | 18 | fail() { echo -e "${RED}[]${NC} $1"; exit 1; } |
| `os/parc-ai/parcos-ai-installer.sh` | 119 | echo -e "${GREEN}Tinkeria installed successfully!${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 14 | echo -e "${CYAN}=== Top Processes (CPU) ===${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 17 | echo -e "${CYAN}=== Top Processes (RAM) ===${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 28 | echo -e "${CYAN}Watching: $target${NC} (Ctrl+C to stop)" |
| `os/parc-ai/parcos-procmon.sh` | 31 | echo -e "${CYAN}=== Process Monitor: $target ===${NC} $(date |
| `os/parc-ai/parcos-procmon.sh` | 35 | echo -e "${CYAN}=== Resource Usage ===${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 61 | echo -e "${CYAN}=== Open Files for PID $pid ===${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 64 | echo -e "${CYAN}=== Memory Maps ===${NC}" |
| `os/parc-ai/parcos-procmon.sh` | 69 | echo -e "${CYAN}=== System Resources ===${NC}" |
| `os/parc-ai/parcos-tools.sh` | 13 | header() { echo -e "\n${CYAN}=== $1 ===${NC}"; } |
| `os/parc-ai/modules/debugging.sh` | 168 | log()  { echo -e "${GREEN}[INFO]${NC} $*"; } |
| `os/parc-ai/modules/debugging.sh` | 169 | warn() { echo -e "${YELLOW}[WARN]${NC} $*"; } |
| `os/parc-ai/modules/debugging.sh` | 170 | err()  { echo -e "${RED}[ERROR]${NC} $*" >&2; } |
| `os/security/filevault.sh` | 12 | echo -e "${YELLOW}Setting up FileVault (Full Disk Encryption |
| `os/security/filevault.sh` | 26 | echo -e "  ${CYAN}Your recovery key: $recovery_key${NC}" |
| `os/security/filevault.sh` | 27 | echo -e "  ${YELLOW}SAVE THIS KEY! You need it if you forget |
| `os/security/filevault.sh` | 50 | echo -e "${GREEN} FileVault enabled!${NC}" |
| `os/security/filevault.sh` | 51 | echo -e "  Encrypted backup: $backup_dir" |
| `os/security/filevault.sh` | 57 | echo -e "${YELLOW}Disabling FileVault...${NC}" |
| `os/security/filevault.sh` | 65 | echo -e "${GREEN} FileVault disabled${NC}" |
| `os/security/filevault.sh` | 74 | echo -e "${RED}Encrypted file not found${NC}" |
| `os/security/filevault.sh` | 87 | echo -e "${GREEN} File decrypted${NC}" |
| `os/security/filevault.sh` | 89 | echo -e "${RED} Decryption failed${NC}" |
| `os/security/filevault.sh` | 99 | echo -e "${RED}File not found${NC}" |
| `os/security/filevault.sh` | 109 | echo -e "${RED}Keys don't match${NC}" |
| `os/security/filevault.sh` | 118 | echo -e "${GREEN} File encrypted: $file.enc${NC}" |
| `os/security/gatekeeper.sh` | 16 | echo -e "${YELLOW}Verifying: $app_path${NC}" |
| `os/security/gatekeeper.sh` | 33 | echo -e "${YELLOW}Unknown app type${NC}" |
| `os/security/gatekeeper.sh` | 42 | echo -e "${GREEN} Package signature valid${NC}" |
| `os/security/gatekeeper.sh` | 48 | echo -e "${GREEN} From trusted repository: $repo${NC}" |
| `os/security/gatekeeper.sh` | 52 | echo -e "${RED} Package signature invalid or untrusted${NC}" |
| `os/security/gatekeeper.sh` | 60 | echo -e "${YELLOW} AppImage signature cannot be verified${NC |
| `os/security/gatekeeper.sh` | 71 | echo -e "${RED} Suspicious content detected!${NC}" |
| `os/security/gatekeeper.sh` | 77 | echo -e "${GREEN} From installed package: $source${NC}" |
| `os/security/gatekeeper.sh` | 81 | echo -e "${YELLOW} Cannot verify binary source${NC}" |
| `os/security/gatekeeper.sh` | 91 | echo -e "${YELLOW}╔═════════════════════════════════════════ |
| `os/security/gatekeeper.sh` | 92 | echo -e "${YELLOW}║                  GATEKEEPER ALERT        |
| `os/security/gatekeeper.sh` | 93 | echo -e "${YELLOW}╚═════════════════════════════════════════ |
| `os/security/gatekeeper.sh` | 95 | echo -e "  App: ${CYAN}$app_name${NC}" |
| `os/security/gatekeeper.sh` | 96 | echo -e "  Publisher: ${CYAN}$publisher${NC}" |
| `os/security/gatekeeper.sh` | 108 | 1) echo -e "${GREEN} App allowed${NC}"; log_gatekeeper "allo |
| `os/security/gatekeeper.sh` | 110 | 3) rm -f "$app_name"; echo -e "${GREEN} App deleted${NC}"; r |
| `os/security/gatekeeper.sh` | 111 | *) echo -e "${YELLOW}Cancelled${NC}"; return 1 ;; |
| `os/security/gatekeeper.sh` | 118 | echo -e "${CYAN}App Details:${NC}" |
| `os/system/bluetooth-manager.sh` | 20 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/system/bluetooth-manager.sh` | 21 | echo -e "${BLUE}║           KORRINOS BLUETOOTH AUTO-CONNECT  |
| `os/system/bluetooth-manager.sh` | 22 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/system/bluetooth-manager.sh` | 68 | echo -e "${RED}Bluetooth tools not found${NC}" |
| `os/system/bluetooth-manager.sh` | 74 | echo -e "${YELLOW}Bluetooth service not running. Starting... |
| `os/system/bluetooth-manager.sh` | 83 | echo -e "${YELLOW}Scanning for Bluetooth devices...${NC}" |
| `os/system/bluetooth-manager.sh` | 109 | echo -e "${YELLOW}Pairing device: $mac${NC}" |
| `os/system/bluetooth-manager.sh` | 115 | echo -e "${GREEN} Device paired and connected!${NC}" |
| `os/system/bluetooth-manager.sh` | 122 | echo -e "${YELLOW}Connecting to: $mac${NC}" |
| `os/system/bluetooth-manager.sh` | 127 | echo -e "${GREEN} Connected!${NC}" |
| `os/system/bluetooth-manager.sh` | 130 | echo -e "${RED} Connection failed${NC}" |
| `os/system/bluetooth-manager.sh` | 139 | echo -e "${YELLOW}Disconnecting: $mac${NC}" |
| `os/system/bluetooth-manager.sh` | 143 | echo -e "${GREEN} Disconnected${NC}" |
| `os/system/bluetooth-manager.sh` | 149 | echo -e "${YELLOW}Paired Devices:${NC}" |
| `os/system/bluetooth-manager.sh` | 167 | echo -e "  $mac  $name$status" |
| `os/system/bluetooth-manager.sh` | 174 | echo -e "${YELLOW}Starting Bluetooth auto-connect daemon...$ |
| `os/system/bluetooth-manager.sh` | 213 | echo -e "${YELLOW}Starting proximity detection...${NC}" |
| `os/system/bluetooth-manager.sh` | 253 | echo -e "${YELLOW}Switching audio output to Bluetooth device |
| `os/system/bluetooth-manager.sh` | 260 | echo -e "${GREEN} Audio output switched to Bluetooth${NC}" |
| `os/system/bluetooth-manager.sh` | 277 | echo -e "${YELLOW}Bluetooth Status:${NC}" |
| `os/system/bluetooth-manager.sh` | 282 | echo -e "Adapter: ${GREEN}Powered ON${NC}" |
| `os/system/bluetooth-manager.sh` | 284 | echo -e "Adapter: ${RED}Powered OFF${NC}" |
| `os/system/bluetooth-manager.sh` | 289 | echo -e "Paired devices: $device_count" |
| `os/system/bluetooth-manager.sh` | 296 | echo -e "Connected: $connected_count" |
| `os/system/bluetooth-manager.sh` | 302 | echo -e "${YELLOW}Bluetooth Options:${NC}" |
| `os/system/bluetooth-manager.sh` | 411 | echo -e "${YELLOW}KorrinOS Bluetooth Manager${NC}" |
| `os/system/default-apps.sh` | 15 | echo -e "${YELLOW}Refusing: this bundles apps for the OS IMA |
| `os/system/default-apps.sh` | 77 | echo -e "${BLUE}── KorrinOS Default Apps Bundler ──${NC}" |
| `os/system/default-apps.sh` | 84 | echo -e "${GREEN}Default apps bundled.${NC}" |
| `os/system/korrinos-apt-hook.sh` | 111 | echo -e "${color}[KorrinOS]${NC} ${message}" >&2 |
| `os/system/korrinos-errors.sh` | 164 | echo -e "\033[1;36m VOKK v4 says:\033[0m" |
| `os/system/korrinos-errors.sh` | 174 | echo -e "\033[1;36m VOKK v4 (Ollama) says:\033[0m" |
| `os/system/korrinos-errors.sh` | 184 | echo -e "\033[1;36m VOKK v4 says:\033[0m" |
| `os/system/korrinos-errors.sh` | 207 | echo -e "\033[1;31m╔════════════════════════════════════════ |
| `os/system/korrinos-errors.sh` | 208 | echo -e "\033[1;31m║    Error Detected (Exit Code: $exit_cod |
| `os/system/korrinos-errors.sh` | 209 | echo -e "\033[1;31m╚════════════════════════════════════════ |
| `os/system/korrinos-errors.sh` | 212 | echo -e "\033[1;33m  What happened:\033[0m $friendly" |
| `os/system/korrinos-errors.sh` | 216 | echo -e "\033[0;37m  Why it happened:\033[0m $explanation" |
| `os/system/korrinos-errors.sh` | 220 | echo -e "\033[1;32m  How to fix it:\033[0m $fix" |
| `os/system/korrinos-errors.sh` | 279 | echo -e "\033[1;31mScript error at line $line (exit code $co |
| `os/system/korrinos-errors.sh` | 314 | echo -e "\033[1;31m[kernel] $line\033[0m" |
| `os/system/korrinos-greetings.sh` | 274 | echo -e "\033[1;36m═════════════════════════════════════════ |
| `os/system/korrinos-greetings.sh` | 275 | echo -e "\033[1;36m  ${greeting}\033[0m" |
| `os/system/korrinos-greetings.sh` | 276 | echo -e "\033[1;36m═════════════════════════════════════════ |
| `os/system/password-manager.sh` | 27 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/system/password-manager.sh` | 28 | echo -e "${BLUE}║         KORRINOS SECURE PASSWORD MANAGER   |
| `os/system/password-manager.sh` | 29 | echo -e "${BLUE}║         Your data. Your device. Your rules |
| `os/system/password-manager.sh` | 30 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/system/password-manager.sh` | 32 | echo -e "  ${CYAN} No cloud storage${NC}" |
| `os/system/password-manager.sh` | 33 | echo -e "  ${CYAN} No data collection${NC}" |
| `os/system/password-manager.sh` | 34 | echo -e "  ${CYAN} 100% local encryption${NC}" |
| `os/system/password-manager.sh` | 43 | echo -e "${YELLOW}Setting up your secure password vault...${ |
| `os/system/password-manager.sh` | 60 | echo -e "  ${RED}Password must be at least 8 characters${NC} |
| `os/system/password-manager.sh` | 63 | echo -e "  ${RED}Passwords don't match${NC}" |
| `os/system/password-manager.sh` | 71 | echo -e "  ${GREEN} Vault created successfully!${NC}" |
| `os/system/password-manager.sh` | 77 | echo -e "  ${GREEN} Ready to use!${NC}" |
| `os/system/password-manager.sh` | 143 | echo -e "${RED}Invalid master password!${NC}" |
| `os/system/password-manager.sh` | 166 | echo -e "${GREEN} Vault locked${NC}" |
| `os/system/password-manager.sh` | 262 | echo -e "${YELLOW}Unlocking vault...${NC}" |
| `os/system/password-manager.sh` | 292 | echo -e "${GREEN} Password saved securely!${NC}" |
| `os/system/password-manager.sh` | 331 | echo -e "${YELLOW}Stored Passwords:${NC}" |
| `os/system/password-manager.sh` | 349 | echo -e "${YELLOW}Monitoring for password fields...${NC}" |
| `os/system/password-manager.sh` | 397 | echo -e "${GREEN}Filling credentials for: $site${NC}" |
| `os/system/password-manager.sh` | 398 | echo -e "  Username: $username" |
| `os/system/password-manager.sh` | 408 | echo -e "${GREEN} Credentials filled!${NC}" |
| `os/system/password-manager.sh` | 410 | echo -e "${YELLOW}No credentials found for: $site${NC}" |
| `os/system/password-manager.sh` | 427 | echo -e "${YELLOW}Password Health Report:${NC}" |
| `os/system/password-manager.sh` | 441 | echo -e "  ${RED}${NC} $site - Weak password" |
| `os/system/password-manager.sh` | 448 | echo -e "  ${YELLOW}!${NC} $site - Password reused" |
| `os/system/password-manager.sh` | 454 | echo -e "Summary:" |
| `os/system/password-manager.sh` | 455 | echo -e "  Total passwords: $total" |
| `os/system/password-manager.sh` | 456 | echo -e "  Weak: $weak" |
| `os/system/password-manager.sh` | 457 | echo -e "  Reused: $reused" |
| `os/system/password-manager.sh` | 535 | echo -e "${YELLOW}Generating secure password...${NC}" |
| `os/system/password-manager.sh` | 537 | echo -e "Generated: ${CYAN}$password${NC}" |
| `os/system/password-manager.sh` | 557 | echo -e "${GREEN}Credentials for: $2${NC}" |
| `os/system/password-manager.sh` | 558 | echo -e "  Username: ${CYAN}$username${NC}" |
| `os/system/password-manager.sh` | 559 | echo -e "  Password: ${CYAN}$password${NC}" |
| `os/system/password-manager.sh` | 564 | echo -e "${GREEN} Password copied to clipboard${NC}" |
| `os/system/password-manager.sh` | 566 | echo -e "${YELLOW}No credentials found for: $2${NC}" |
| `os/system/password-manager.sh` | 575 | echo -e "${YELLOW}Password Generator:${NC}" |
| `os/system/password-manager.sh` | 581 | echo -e "  Generated password: ${CYAN}$password${NC}" |
| `os/system/password-manager.sh` | 583 | echo -e "  ${GREEN} Copied to clipboard${NC}" |
| `os/system/password-manager.sh` | 588 | echo -e "${YELLOW}Passphrase Generator:${NC}" |
| `os/system/password-manager.sh` | 594 | echo -e "  Generated passphrase: ${CYAN}$passphrase${NC}" |
| `os/system/password-manager.sh` | 596 | echo -e "  ${GREEN} Copied to clipboard${NC}" |
| `os/system/password-manager.sh` | 605 | echo -e "${YELLOW}Unlocking vault...${NC}" |
| `os/system/password-manager.sh` | 608 | echo -e "${GREEN} Vault unlocked!${NC}" |
| `os/system/password-manager.sh` | 651 | echo -e "${YELLOW}KorrinOS Secure Password Manager${NC}" |
| `os/system/password-manager.sh` | 656 | echo -e "  ${CYAN}Quick start:${NC}" |
| `os/system/password-monitor.sh` | 22 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/system/password-monitor.sh` | 23 | echo -e "${BLUE}║            KORRINOS PASSWORD MONITOR       |
| `os/system/password-monitor.sh` | 24 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/system/password-monitor.sh` | 116 | echo -e "${CYAN}╔═══════════════════════════════════════════ |
| `os/system/password-monitor.sh` | 117 | echo -e "${CYAN}║               PASSWORD MANAGER ALERT       |
| `os/system/password-monitor.sh` | 118 | echo -e "${CYAN}╚═══════════════════════════════════════════ |
| `os/system/password-monitor.sh` | 120 | echo -e "  Password field detected on: ${GREEN}$domain${NC}" |
| `os/system/password-monitor.sh` | 141 | echo -e "${YELLOW}Generating password for: $domain${NC}" |
| `os/system/password-monitor.sh` | 147 | echo -e "  Generated password: ${CYAN}$password${NC}" |
| `os/system/password-monitor.sh` | 156 | echo -e "${GREEN} Password saved securely!${NC}" |
| `os/system/password-monitor.sh` | 157 | echo -e "  Site: $domain" |
| `os/system/password-monitor.sh` | 158 | echo -e "  Username: $username" |
| `os/system/password-monitor.sh` | 159 | echo -e "  Password: $password" |
| `os/system/password-monitor.sh` | 164 | echo -e "${GREEN} Password copied to clipboard${NC}" |
| `os/system/password-monitor.sh` | 183 | echo -e "${GREEN}Found credentials for: $domain${NC}" |
| `os/system/password-monitor.sh` | 184 | echo -e "  Username: $username" |
| `os/system/password-monitor.sh` | 185 | echo -e "  Password: ****" |
| `os/system/password-monitor.sh` | 200 | echo -e "${GREEN} Credentials filled!${NC}" |
| `os/system/password-monitor.sh` | 205 | echo -e "${GREEN} Password copied to clipboard${NC}" |
| `os/system/password-monitor.sh` | 207 | echo -e "${YELLOW}No saved password for: $domain${NC}" |
| `os/system/password-monitor.sh` | 219 | echo -e "${YELLOW}Password Monitor Active${NC}" |
| `os/system/password-monitor.sh` | 265 | echo -e "${YELLOW}Installing Browser Extension${NC}" |
| `os/system/password-monitor.sh` | 492 | echo -e "${GREEN} Browser extension created!${NC}" |
| `os/system/password-monitor.sh` | 524 | echo -e "${GREEN} Monitor stopped${NC}" |
| `os/system/password-monitor.sh` | 539 | echo -e "${YELLOW}KorrinOS Password Monitor${NC}" |
| `os/system/power-manager.sh` | 19 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/system/power-manager.sh` | 20 | echo -e "${BLUE}║              KORRINOS POWER MANAGER        |
| `os/system/power-manager.sh` | 21 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/system/power-manager.sh` | 81 | echo -e "Battery: ${GREEN}${capacity}%${NC} ($status)" |
| `os/system/power-manager.sh` | 90 | echo -e "Time remaining: ~${remaining} minutes" |
| `os/system/power-manager.sh` | 94 | echo -e "Battery: ${YELLOW}Not detected${NC}" |
| `os/system/power-manager.sh` | 113 | echo -e "${YELLOW}Setting power profile: $profile${NC}" |
| `os/system/power-manager.sh` | 164 | echo -e "${GREEN} Performance mode enabled${NC}" |
| `os/system/power-manager.sh` | 193 | echo -e "${GREEN} Balanced mode enabled${NC}" |
| `os/system/power-manager.sh` | 230 | echo -e "${GREEN} Power saver mode enabled${NC}" |
| `os/system/power-manager.sh` | 255 | echo -e "${RED}Battery critical! Switching to power saver... |
| `os/system/power-manager.sh` | 258 | echo -e "${YELLOW}Battery low! Switching to balanced...${NC} |
| `os/system/power-manager.sh` | 264 | echo -e "${GREEN}Battery charging. Switching to balanced...$ |
| `os/system/power-manager.sh` | 272 | echo -e "${YELLOW}Optimizing battery health...${NC}" |
| `os/system/power-manager.sh` | 290 | echo -e "${GREEN} Battery charge thresholds set: $charge_sta |
| `os/system/power-manager.sh` | 292 | echo -e "${YELLOW}Charge threshold control not available for |
| `os/system/power-manager.sh` | 299 | echo -e "${YELLOW}Power Usage:${NC}" |
| `os/system/power-manager.sh` | 304 | echo -e "CPU: ${cpu_usage}%" |
| `os/system/power-manager.sh` | 308 | echo -e "Memory: ${mem_usage}%" |
| `os/system/power-manager.sh` | 312 | echo -e "Disk: ${disk_usage}" |
| `os/system/power-manager.sh` | 317 | echo -e "GPU: ${gpu_usage}%" |
| `os/system/power-manager.sh` | 347 | echo -e "${YELLOW}Power Status:${NC}" |
| `os/system/power-manager.sh` | 351 | echo -e "Current profile: ${GREEN}$(get_current_profile)${NC |
| `os/system/power-manager.sh` | 366 | echo -e "${YELLOW}Available Power Profiles:${NC}" |
| `os/system/power-manager.sh` | 372 | echo -e "Current: ${GREEN}$(get_current_profile)${NC}" |
| `os/system/power-manager.sh` | 391 | echo -e "${YELLOW}KorrinOS Power Manager${NC}" |
| `os/system/power-manager.sh` | 397 | echo -e "Current profile: ${GREEN}$(get_current_profile)${NC |
| `os/system/update-system.sh` | 21 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/system/update-system.sh` | 22 | echo -e "${BLUE}║                  KORRINOS UPDATE SYSTEM    |
| `os/system/update-system.sh` | 23 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/system/update-system.sh` | 28 | echo -e "${YELLOW}Checking for updates...${NC}" |
| `os/system/update-system.sh` | 32 | echo -e "  Checking system updates..." |
| `os/system/update-system.sh` | 36 | echo -e "  ${GREEN}${NC} System updates available: $updates" |
| `os/system/update-system.sh` | 39 | echo -e "  ${GREEN}${NC} System updates available: $updates" |
| `os/system/update-system.sh` | 43 | echo -e "  ${GREEN}${NC} System updates available: $updates" |
| `os/system/update-system.sh` | 47 | echo -e "  Checking KorrinOS updates..." |
| `os/system/update-system.sh` | 49 | echo -e "  Current version: $current_version" |
| `os/system/update-system.sh` | 52 | echo -e "  Checking driver updates..." |
| `os/system/update-system.sh` | 53 | echo -e "  ${GREEN}${NC} Driver updates checked" |
| `os/system/update-system.sh` | 56 | echo -e "  Checking app updates..." |
| `os/system/update-system.sh` | 57 | echo -e "  ${GREEN}${NC} App updates checked" |
| `os/system/update-system.sh` | 60 | echo -e "${GREEN}Update check complete!${NC}" |
| `os/system/update-system.sh` | 65 | echo -e "${YELLOW}Applying updates...${NC}" |
| `os/system/update-system.sh` | 69 | echo -e "  Creating backup..." |
| `os/system/update-system.sh` | 73 | echo -e "  Applying system updates..." |
| `os/system/update-system.sh` | 83 | echo -e "  Updating KorrinOS components..." |
| `os/system/update-system.sh` | 87 | echo -e "  Updating drivers..." |
| `os/system/update-system.sh` | 91 | echo -e "  Updating apps..." |
| `os/system/update-system.sh` | 95 | echo -e "  Cleaning up..." |
| `os/system/update-system.sh` | 99 | echo -e "${GREEN} All updates applied!${NC}" |
| `os/system/update-system.sh` | 100 | echo -e "${YELLOW}Some changes may require a reboot.${NC}" |
| `os/system/update-system.sh` | 105 | echo -e "  Updating KorrinOS core..." |
| `os/system/update-system.sh` | 109 | echo -e "    ${GREEN}${NC} Smart Input System up to date" |
| `os/system/update-system.sh` | 113 | echo -e "    ${GREEN}${NC} Desktop components up to date" |
| `os/system/update-system.sh` | 116 | echo -e "    ${GREEN}${NC} System utilities up to date" |
| `os/system/update-system.sh` | 120 | echo -e "  Updating drivers..." |
| `os/system/update-system.sh` | 131 | echo -e "    ${GREEN}${NC} Drivers updated" |
| `os/system/update-system.sh` | 135 | echo -e "  Updating apps..." |
| `os/system/update-system.sh` | 147 | echo -e "    ${GREEN}${NC} Apps updated" |
| `os/system/update-system.sh` | 158 | echo -e "    ${GREEN}${NC} Backup created at $backup_dir" |
| `os/system/update-system.sh` | 176 | echo -e "    ${GREEN}${NC} Cleanup complete" |
| `os/system/update-system.sh` | 180 | echo -e "${YELLOW}Update History:${NC}" |
| `os/system/update-system.sh` | 192 | echo -e "${YELLOW}Rolling back updates...${NC}" |
| `os/system/update-system.sh` | 199 | echo -e "${RED}No backup found for rollback.${NC}" |
| `os/system/update-system.sh` | 203 | echo -e "  Restoring from: $backup_dir" |
| `os/system/update-system.sh` | 208 | echo -e "${GREEN} Rollback complete!${NC}" |
| `os/system/update-system.sh` | 209 | echo -e "${YELLOW}Please reboot to apply changes.${NC}" |
| `os/system/update-system.sh` | 251 | echo -e "${YELLOW}KorrinOS Update System${NC}" |
| `os/territories/vibe-address/core/ingest.sh` | 138 | FILE) echo -e "file\ndoc\ntext\nartifact" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 139 | NET)  echo -e "web\npage\ntab\nonline\nnet" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 140 | APP)  echo -e "app\nprogram\nsoftware\ntool" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 141 | INPUT)echo -e "search\nquery\ntyped\nwrote" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 142 | MEDIA)echo -e "media\ngame\nplay\nvideo\naudio" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 143 | SYSTEM)echo -e "system\nmount\ndisk\nstate" ;; |
| `os/territories/vibe-address/core/ingest.sh` | 144 | USER) echo -e "note\nclip\nsave\nattach" ;; |
| `os/drivers/driver-manager.sh` | 16 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/drivers/driver-manager.sh` | 17 | echo -e "${BLUE}║                  KORRINOS DRIVER MANAGER   |
| `os/drivers/driver-manager.sh` | 18 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/drivers/driver-manager.sh` | 23 | echo -e "${YELLOW}Detecting GPU...${NC}" |
| `os/drivers/driver-manager.sh` | 28 | echo -e "  ${GREEN}${NC} NVIDIA GPU detected: $(echo $gpu_in |
| `os/drivers/driver-manager.sh` | 31 | echo -e "  ${GREEN}${NC} AMD GPU detected: $(echo $gpu_info  |
| `os/drivers/driver-manager.sh` | 34 | echo -e "  ${GREEN}${NC} Intel GPU detected: $(echo $gpu_inf |
| `os/drivers/driver-manager.sh` | 37 | echo -e "  ${YELLOW}?${NC} Unknown GPU: $gpu_info" |
| `os/drivers/driver-manager.sh` | 43 | echo -e "${YELLOW}Detecting WiFi...${NC}" |
| `os/drivers/driver-manager.sh` | 48 | echo -e "  ${GREEN}${NC} Intel WiFi detected" |
| `os/drivers/driver-manager.sh` | 51 | echo -e "  ${GREEN}${NC} Realtek WiFi detected" |
| `os/drivers/driver-manager.sh` | 54 | echo -e "  ${GREEN}${NC} MediaTek WiFi detected" |
| `os/drivers/driver-manager.sh` | 57 | echo -e "  ${YELLOW}?${NC} WiFi: $wifi_info" |
| `os/drivers/driver-manager.sh` | 63 | echo -e "${YELLOW}Detecting audio devices...${NC}" |
| `os/drivers/driver-manager.sh` | 68 | echo -e "  ${GREEN}${NC} Audio device: $(echo $audio_info \|  |
| `os/drivers/driver-manager.sh` | 71 | echo -e "  ${YELLOW}?${NC} No audio device detected" |
| `os/drivers/driver-manager.sh` | 77 | echo -e "${YELLOW}Detecting Bluetooth...${NC}" |
| `os/drivers/driver-manager.sh` | 80 | echo -e "  ${GREEN}${NC} Bluetooth adapter detected" |
| `os/drivers/driver-manager.sh` | 83 | echo -e "  ${GREEN}${NC} Bluetooth adapter detected" |
| `os/drivers/driver-manager.sh` | 86 | echo -e "  ${YELLOW}?${NC} No Bluetooth adapter detected" |
| `os/drivers/driver-manager.sh` | 92 | echo -e "${YELLOW}Installing NVIDIA drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 102 | echo -e "${GREEN} NVIDIA drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 106 | echo -e "${YELLOW}Installing AMD drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 116 | echo -e "${GREEN} AMD drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 120 | echo -e "${YELLOW}Installing Intel drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 130 | echo -e "${GREEN} Intel drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 134 | echo -e "${YELLOW}Installing WiFi drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 144 | echo -e "${GREEN} WiFi drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 148 | echo -e "${YELLOW}Installing Bluetooth drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 158 | echo -e "${GREEN} Bluetooth drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 162 | echo -e "${YELLOW}Installing all detected drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 193 | echo -e "${GREEN} All drivers installed!${NC}" |
| `os/drivers/driver-manager.sh` | 194 | echo -e "${YELLOW}Please reboot to apply changes.${NC}" |
| `os/drivers/driver-manager.sh` | 198 | echo -e "${YELLOW}Driver Status:${NC}" |
| `os/drivers/driver-manager.sh` | 202 | echo -e "GPU:" |
| `os/drivers/driver-manager.sh` | 204 | echo -e "  ${GREEN}${NC} NVIDIA drivers installed" |
| `os/drivers/driver-manager.sh` | 206 | echo -e "  ${GREEN}${NC} Mesa drivers installed" |
| `os/drivers/driver-manager.sh` | 208 | echo -e "  ${RED}${NC} No GPU drivers installed" |
| `os/drivers/driver-manager.sh` | 212 | echo -e "WiFi:" |
| `os/drivers/driver-manager.sh` | 214 | echo -e "  ${GREEN}${NC} WiFi interface available" |
| `os/drivers/driver-manager.sh` | 216 | echo -e "  ${RED}${NC} No WiFi interface" |
| `os/drivers/driver-manager.sh` | 220 | echo -e "Bluetooth:" |
| `os/drivers/driver-manager.sh` | 222 | echo -e "  ${GREEN}${NC} Bluetooth available" |
| `os/drivers/driver-manager.sh` | 224 | echo -e "  ${RED}${NC} Bluetooth not available" |
| `os/drivers/driver-manager.sh` | 245 | echo -e "${YELLOW}Detecting hardware...${NC}" |
| `os/drivers/driver-manager.sh` | 265 | echo -e "${YELLOW}Updating drivers...${NC}" |
| `os/drivers/driver-manager.sh` | 273 | echo -e "${GREEN} Drivers updated!${NC}" |
| `os/drivers/driver-manager.sh` | 280 | echo -e "${YELLOW}KorrinOS Driver Manager${NC}" |
| `os/desktop/gestures.sh` | 19 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/desktop/gestures.sh` | 20 | echo -e "${BLUE}║              KORRINOS TOUCHPAD GESTURES    |
| `os/desktop/gestures.sh` | 21 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/desktop/gestures.sh` | 255 | echo -e "${YELLOW}Available Gestures:${NC}" |
| `os/desktop/gestures.sh` | 298 | echo -e "${GREEN}Enabling gesture support...${NC}" |
| `os/desktop/gestures.sh` | 302 | echo -e "${GREEN} Gesture support enabled!${NC}" |
| `os/desktop/gestures.sh` | 305 | echo -e "${YELLOW}Disabling gesture support...${NC}" |
| `os/desktop/gestures.sh` | 310 | echo -e "${GREEN} Gesture support disabled!${NC}" |
| `os/desktop/gestures.sh` | 322 | echo -e "${YELLOW}Touchpad Calibration${NC}" |
| `os/desktop/gestures.sh` | 332 | echo -e "${YELLOW}KorrinOS Touchpad Gestures${NC}" |
| `os/desktop/shortcuts.sh` | 20 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/desktop/shortcuts.sh` | 21 | echo -e "${BLUE}║              KORRINOS KEYBOARD SHORTCUTS   |
| `os/desktop/shortcuts.sh` | 22 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/desktop/shortcuts.sh` | 465 | echo -e "${YELLOW}Current Keyboard Shortcuts:${NC}" |
| `os/desktop/shortcuts.sh` | 491 | echo -e "${GREEN}Shortcuts reset to defaults!${NC}" |
| `os/desktop/shortcuts.sh` | 529 | echo -e "${GREEN}Shortcuts reloaded!${NC}" |
| `os/desktop/shortcuts.sh` | 536 | echo -e "${YELLOW}KorrinOS Keyboard Shortcuts${NC}" |
| `os/desktop/tiling.sh` | 19 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/desktop/tiling.sh` | 20 | echo -e "${BLUE}║            KORRINOS WINDOW TILING MANAGER  |
| `os/desktop/tiling.sh` | 21 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/desktop/tiling.sh` | 264 | echo -e "${YELLOW}Window Tiling Options:${NC}" |
| `os/desktop/tiling.sh` | 355 | echo -e "${YELLOW}KorrinOS Window Tiling Manager${NC}" |
| `os/desktop/virtual-desktops.sh` | 19 | echo -e "${BLUE}╔═══════════════════════════════════════════ |
| `os/desktop/virtual-desktops.sh` | 20 | echo -e "${BLUE}║            KORRINOS VIRTUAL DESKTOP MANAGE |
| `os/desktop/virtual-desktops.sh` | 21 | echo -e "${BLUE}╚═══════════════════════════════════════════ |
| `os/desktop/virtual-desktops.sh` | 169 | echo -e "${GREEN} Created desktop: $name${NC}" |
| `os/desktop/virtual-desktops.sh` | 195 | echo -e "${GREEN} Removed desktop $desktop${NC}" |
| `os/desktop/virtual-desktops.sh` | 200 | echo -e "${YELLOW}Virtual Desktops:${NC}" |
| `os/desktop/virtual-desktops.sh` | 222 | echo -e "  $num: $name$active" |
| `os/desktop/virtual-desktops.sh` | 234 | echo -e "${YELLOW}Windows on Desktop $((desktop + 1)):${NC}" |
| `os/desktop/virtual-desktops.sh` | 278 | echo -e "${GREEN}Switched to desktop $((target + 1))${NC}" |
| `os/desktop/virtual-desktops.sh` | 310 | echo -e "${GREEN}Window moved to desktop $((target + 1))${NC |
| `os/desktop/virtual-desktops.sh` | 356 | echo -e "${GREEN}Switched to desktop $2${NC}" |
| `os/desktop/virtual-desktops.sh` | 371 | echo -e "${GREEN}Window moved to desktop $2${NC}" |
| `os/desktop/virtual-desktops.sh` | 400 | echo -e "${YELLOW}KorrinOS Virtual Desktop Manager${NC}" |

### C14 useless $(echo) — 524

| file | line | detail |
|------|------|--------|
| `os/release.sh` | 25 | CUR_MAJOR=$(echo "$CURRENT" \| cut -d. -f1) |
| `os/release.sh` | 26 | CUR_MINOR=$(echo "$CURRENT" \| cut -d. -f2) |
| `os/release.sh` | 52 | NEW_MAJOR=$(echo "$arg" \| cut -d. -f1) |
| `os/release.sh` | 53 | NEW_MINOR=$(echo "$arg" \| cut -d. -f2) |
| `os/release.sh` | 54 | NEW_PATCH=$(echo "$arg" \| cut -d. -f3) |
| `os/apps/battery-monitor.sh` | 125 | local start_cap=$(echo "$recent" \| head -1 \| cut -d'\|' -f2) |
| `os/apps/battery-monitor.sh` | 126 | local end_cap=$(echo "$recent" \| tail -1 \| cut -d'\|' -f2) |
| `os/apps/battery-monitor.sh` | 127 | local start_ts=$(echo "$recent" \| head -1 \| cut -d'\|' -f1) |
| `os/apps/battery-monitor.sh` | 128 | local end_ts=$(echo "$recent" \| tail -1 \| cut -d'\|' -f1) |
| `os/apps/gaming-support.sh` | 207 | echo "  Detected NVIDIA hardware: $(echo "$nvidia_device" \|  |
| `os/apps/quick-note.sh` | 33 | local id=$(echo "$timestamp" \| md5sum \| head -c 8) |
| `os/apps/screen-recorder.sh` | 37 | local x=$(echo "$geom" \| grep -oP 'Position: \K\d+') |
| `os/apps/screen-recorder.sh` | 38 | local y=$(echo "$geom" \| grep -oP ', \K\d+') |
| `os/apps/screen-recorder.sh` | 39 | local w=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/apps/screen-recorder.sh` | 40 | local h=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/apps/screen-recorder.sh` | 56 | local x=$(echo "$geom" \| grep -oP 'Position: \K\d+') |
| `os/apps/screen-recorder.sh` | 57 | local y=$(echo "$geom" \| grep -oP ', \K\d+') |
| `os/apps/screen-recorder.sh` | 58 | local w=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/apps/screen-recorder.sh` | 59 | local h=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/apps/smart-clipboard.sh` | 64 | local display=$(echo "$text" \| head -c 60) |
| `os/apps/smart-clipboard.sh` | 101 | local display=$(echo "$text" \| head -c 60) |
| `os/apps/software-center.sh` | 83 | local name=$(echo "$cat" \| cut -d: -f2) |
| `os/apps/software-center.sh` | 84 | local desc=$(echo "$cat" \| cut -d: -f3) |
| `os/apps/software-center.sh` | 95 | local category=$(echo "${APP_CATEGORIES[$((choice-1))]}" \| c |
| `os/apps/software-center.sh` | 114 | local app_cat=$(echo "$app" \| cut -d: -f2) |
| `os/apps/software-center.sh` | 116 | local name=$(echo "$app" \| cut -d: -f1) |
| `os/apps/software-center.sh` | 117 | local desc=$(echo "$app" \| cut -d: -f3) |
| `os/apps/software-center.sh` | 133 | local app_name=$(echo "${APP_LIST[$((choice-1))]}" \| cut -d: |
| `os/apps/software-center.sh` | 155 | local name=$(echo "$app" \| cut -d: -f1) |
| `os/apps/software-center.sh` | 156 | local desc=$(echo "$app" \| cut -d: -f3) |
| `os/apps/software-center.sh` | 178 | local name=$(echo "$app" \| cut -d: -f1) |
| `os/apps/voice-commands.sh` | 250 | local query=$(echo "$command" \| sed 's/.*search for //' \| se |
| `os/apps/gaming/audio-mixer.sh` | 169 | local id=$(echo "$line" \| cut -d$'\t' -f1) |
| `os/apps/gaming/audio-mixer.sh` | 170 | local app=$(echo "$line" \| cut -d$'\t' -f3) |
| `os/apps/gaming/hardware-benchmark.sh` | 21 | local events=$(echo "$out" \| grep "total number of events" \| |
| `os/apps/gaming/hardware-benchmark.sh` | 22 | local eps=$(echo "$out" \| grep "events per second" \| awk '{p |
| `os/apps/gaming/hardware-benchmark.sh` | 54 | local ops=$(echo "$out" \| grep "Total operations" \| awk '{pr |
| `os/apps/gaming/hardware-benchmark.sh` | 55 | local transfer=$(echo "$out" \| grep "transferred" \| awk '{pr |
| `os/apps/security/findmydevice.sh` | 117 | local ts=$(echo $line \| cut -d'\|' -f1) |
| `os/apps/security/findmydevice.sh` | 118 | local loc=$(echo $line \| cut -d'\|' -f2) |
| `os/apps/security/gatekeeper.sh` | 43 | for dir in $(echo "$PATH" \| tr ':' ' '); do |
| `os/apps/security/gatekeeper.sh` | 56 | for loc in $(echo "$untrusted" \| tr ',' ' '); do |
| `os/apps/security/password-manager.sh` | 38 | local hash=$(echo -n "$master" \| sha256sum \| awk '{print $1} |
| `os/apps/security/password-manager.sh` | 54 | local given=$(echo -n "$master" \| sha256sum \| awk '{print $1 |
| `os/apps/security/password-manager.sh` | 85 | local encoded=$(echo -n "$password" \| openssl enc -aes-256-c |
| `os/apps/security/password-manager.sh` | 107 | local password=$(echo "$encoded" \| openssl enc -d -aes-256-c |
| `os/apps/hardware/printer-manager.sh` | 52 | echo "Total jobs: $(echo "$jobs" \| grep -c .)" |
| `os/apps/hardware/webcam-manager.sh` | 129 | echo "    Your groups: $(id -nG \| tr ' ' '\n' \| grep -E "vid |
| `os/apps/system/adaptive-power-grid.sh` | 188 | if [ $(echo "$load > $threshold" \| bc 2>/dev/null) -eq 1 ];  |
| `os/apps/system/backup-restore.sh` | 254 | local name=$(echo "$line" \| awk '{print $9}') |
| `os/apps/system/backup-restore.sh` | 255 | local size=$(echo "$line" \| awk '{print $5}') |
| `os/apps/system/backup-restore.sh` | 256 | local date=$(echo "$line" \| awk '{print $6, $7, $8}') |
| `os/apps/system/digital-twin.sh` | 106 | local name=$(basename "$(echo "$line" \| awk '{print $9}')" . |
| `os/apps/system/digital-twin.sh` | 107 | local size=$(echo "$line" \| awk '{print $5}') |
| `os/apps/system/digital-twin.sh` | 108 | local date=$(echo "$line" \| awk '{print $6, $7, $8}') |
| `os/apps/system/file-versioning.sh` | 15 | local id=$(echo "$file" \| md5sum \| head -c 8) |
| `os/apps/system/file-versioning.sh` | 27 | local id=$(echo "$file" \| md5sum \| head -c 8) |
| `os/apps/system/file-versioning.sh` | 45 | local id=$(echo "$file" \| md5sum \| head -c 8) |
| `os/apps/system/file-versioning.sh` | 65 | local id=$(echo "$file" \| md5sum \| head -c 8) |
| `os/apps/system/file-versioning.sh` | 81 | local id=$(echo "$file" \| md5sum \| head -c 8) |
| `os/apps/system/intent-launcher.sh` | 14 | low=$(echo "$intent" \| tr '[:upper:]' '[:lower:]') |
| `os/apps/system/rollback-recovery.sh` | 198 | local id=$(echo "$line" \| awk '{print $1}') |
| `os/apps/system/rollback-recovery.sh` | 209 | local path=$(echo "$line" \| awk '{print $9}') |
| `os/apps/system/system-monitor.sh` | 156 | [ $(echo "$mem" \| cut -d. -f1) -gt ${alert_mem:-90} ] && not |
| `os/apps/system/system-monitor.sh` | 167 | local cpu_val=$(echo $cpu \| cut -d= -f2) |
| `os/apps/system/system-monitor.sh` | 168 | local mem_val=$(echo $mem \| cut -d= -f2) |
| `os/apps/system/system-monitor.sh` | 169 | local disk_val=$(echo $disk \| cut -d= -f2) |
| `os/apps/system/system-monitor.sh` | 170 | local load_val=$(echo $load \| cut -d= -f2) |
| `os/apps/system/terminal-error-explainer.sh` | 12 | key=$(echo "$err" \| tr '[:upper:]' '[:lower:]') |
| `os/apps/network/speed-test.sh` | 20 | printf "%.2f MB/s\n" "$(echo "$out" \| awk '{print $1/1048576 |
| `os/apps/network/speed-test.sh` | 40 | printf "%.2f MB/s\n" "$(echo "$out" \| awk '{print $1/1048576 |
| `os/apps/network/speed-test.sh` | 61 | local loss=$(echo "$result" \| tail -1 \| grep -o "[0-9]*% pac |
| `os/apps/network/speed-test.sh` | 62 | local jitter=$(echo "$result" \| grep "min/avg/max" \| awk -F= |
| `os/apps/network/speed-test.sh` | 97 | local ts=$(echo $line \| cut -d'\|' -f1) |
| `os/apps/network/speed-test.sh` | 98 | local speed=$(echo $line \| cut -d'\|' -f2) |
| `os/apps/network/vpn-manager.sh` | 64 | vpn_iface=$(echo "$vpn_conn" \| cut -d: -f3) |
| `os/hyperdrive/hyperdrive-daemon.sh` | 90 | W=$(echo "$CURRENT" \| cut -d'x' -f1) |
| `os/hyperdrive/hyperdrive-daemon.sh` | 91 | H=$(echo "$CURRENT" \| cut -d'x' -f2) |
| `os/parc-ai/korrinos-backup.sh` | 91 | done <<< "$(echo "$includes" \| tr ' ' '\n')" |
| `os/parc-ai/korrinos-backup.sh` | 261 | done <<< "$(echo "$includes" \| tr ' ' '\n')" |
| `os/parc-ai/korrinos-clipctx.sh` | 55 | echo "{\"id\":\"$id\",\"text\":\"$(echo "$text" \| sed 's/"/\ |
| `os/parc-ai/korrinos-dashboard.sh` | 142 | name=$(echo "$iface" \| cut -d/ -f5) |
| `os/parc-ai/korrinos-dashboard.sh` | 176 | ps_name=$(echo "$ps" \| cut -d/ -f5) |
| `os/parc-ai/korrinos-dashboard.sh` | 205 | local load_int=$(echo "$load" \| cut -d. -f1) |
| `os/parc-ai/korrinos-monitor.sh` | 70 | [ $i -lt $(echo "$cpu_pct" \| cut -d. -f1 \| head -1) ] && pri |
| `os/parc-ai/korrinos-network.sh` | 21 | name=$(echo "$line" \| awk '{print $1}') |
| `os/parc-ai/korrinos-network.sh` | 22 | state=$(echo "$line" \| awk '{print $2}') |
| `os/parc-ai/korrinos-nlctl.sh` | 32 | local query_lower=$(echo "$query" \| tr '[:upper:]' '[:lower: |
| `os/parc-ai/korrinos-nlctl.sh` | 36 | local level=$(echo "$query_lower" \| grep -o '[0-9]*' \| head  |
| `os/parc-ai/korrinos-nlctl.sh` | 37 | [ -z "$level" ] && level=$(echo "$query_lower" \| grep -qi "m |
| `os/parc-ai/korrinos-nlctl.sh` | 59 | local level=$(echo "$query_lower" \| grep -o '[0-9]*' \| head  |
| `os/parc-ai/korrinos-nlctl.sh` | 60 | [ -z "$level" ] && level=$(echo "$query_lower" \| grep -qi "u |
| `os/parc-ai/korrinos-notepad.sh` | 77 | lower=$(echo "$input" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/korrinos-notepad.sh` | 81 | time_part=$(echo "$lower" \| grep -oP '\d{1,2}(:\d{2})?\s*(am |
| `os/parc-ai/korrinos-notepad.sh` | 83 | hr=$(echo "$time_part" \| grep -oP '^\d{1,2}') |
| `os/parc-ai/korrinos-notepad.sh` | 84 | min=$(echo "$time_part" \| grep -oP ':\d{2}' \| tr -d ':' \|\| e |
| `os/parc-ai/korrinos-notepad.sh` | 85 | ampm=$(echo "$time_part" \| grep -oP '(am\|pm)' \|\| echo "") |
| `os/parc-ai/korrinos-notepad.sh` | 94 | time_part=$(echo "$lower" \| grep -oP '\d{1,2}(:\d{2})?\s*(am |
| `os/parc-ai/korrinos-notepad.sh` | 97 | hr=$(echo "$time_part" \| grep -oP '^\d{1,2}') |
| `os/parc-ai/korrinos-notepad.sh` | 98 | min=$(echo "$time_part" \| grep -oP ':\d{2}' \| tr -d ':' \|\| e |
| `os/parc-ai/korrinos-notepad.sh` | 99 | ampm=$(echo "$time_part" \| grep -oP '(am\|pm)' \|\| echo "") |
| `os/parc-ai/korrinos-notepad.sh` | 113 | num=$(echo "$lower" \| grep -oP 'in\s+\K\d+') |
| `os/parc-ai/korrinos-notepad.sh` | 114 | unit=$(echo "$lower" \| grep -oP 'hour\|minute\|min') |
| `os/parc-ai/korrinos-power.sh` | 115 | [ "$voltage" != "?" ] && echo "  Voltage:     $(echo "scale= |
| `os/parc-ai/korrinos-power.sh` | 120 | [ "$temp" != "?" ] && echo "  Temperature: $(echo "scale=1;  |
| `os/parc-ai/korrinos-power.sh` | 189 | local name=$(echo "$fan" \| cut -d/ -f5) |
| `os/parc-ai/korrinos-splitscreen.sh` | 20 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 21 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 35 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 36 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 50 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 51 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 63 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 64 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 76 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 77 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 89 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 90 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 111 | local w=$(echo "$geo" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 112 | local h=$(echo "$geo" \| awk '{print $2}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 126 | local id=$(echo "$line" \| awk '{print $1}') |
| `os/parc-ai/korrinos-splitscreen.sh` | 127 | local title=$(echo "$line" \| cut -d' ' -f4-) |
| `os/parc-ai/korrinos-voice.sh` | 115 | local cmd=$(echo "$text" \| sed 's/.*tinker[: ]*//i') |
| `os/parc-ai/korrinos-voice.sh` | 136 | local level=$(echo "$cmd" \| grep -o '[0-9]*' \| head -1) |
| `os/parc-ai/korrinos-voice.sh` | 141 | local level=$(echo "$cmd" \| grep -o '[0-9]*' \| head -1) |
| `os/parc-ai/korrinos-voice.sh` | 146 | local app=$(echo "$cmd" \| sed 's/.*open\\|.*launch//i' \| xarg |
| `os/parc-ai/parc-ai.sh` | 338 | html_answer=$(echo "$answer" \| python3 -c " |
| `os/parc-ai/parc-ai.sh` | 360 | " 2>/dev/null) \|\| html_answer=$(echo "$answer" \| sed 's/</\& |
| `os/parc-ai/parcos-desktop.sh` | 74 | local id=$(echo "$line" \| awk '{print $1}') |
| `os/parc-ai/parcos-desktop.sh` | 75 | local ws=$(echo "$line" \| awk '{print $2}') |
| `os/parc-ai/parcos-desktop.sh` | 76 | local title=$(echo "$line" \| cut -d' ' -f4-) |
| `os/parc-ai/parcos-searchie.sh` | 134 | local search_words=$(echo "$query" \| sed 's/find me\\|search  |
| `os/parc-ai/parcos-searchie.sh` | 167 | echo "   Text: $(echo "$text" \| head -1)" |
| `os/parc-ai/modules/agent-browser.sh` | 101 | echo "Opened $(echo "$urls" \| head -20 \| wc -l) tabs" |
| `os/parc-ai/modules/agent-browser.sh` | 154 | local wiki_title=$(echo "$query" \| sed 's/ /_/g') |
| `os/parc-ai/modules/agent-vision.sh` | 92 | local x=$(echo "$coords" \| grep -oP '\d+' \| head -1) |
| `os/parc-ai/modules/agent-vision.sh` | 93 | local y=$(echo "$coords" \| grep -oP '\d+' \| tail -1) |
| `os/parc-ai/modules/ai-brain-smart.sh` | 7 | local query_lower=$(echo "$query" \| tr '[:upper:]' '[:lower: |
| `os/parc-ai/modules/ai-brain-smart.sh` | 11 | local has_context=$(echo "$context" \| python3 -c "import sys |
| `os/parc-ai/modules/ai-brain-smart.sh` | 213 | local lang=$(echo "$query" \| grep -oE '(to\|into\|in)\s+\w+' \| |
| `os/parc-ai/modules/ai-brain-smart.sh` | 214 | local text=$(echo "$query" \| sed -E 's/^(translate\|translati |
| `os/parc-ai/modules/ai-brain-smart.sh` | 322 | local lang=$(echo "$entity" \| cut -d'\|' -f1) |
| `os/parc-ai/modules/ai-brain-smart.sh` | 323 | local text=$(echo "$entity" \| cut -d'\|' -f2) |
| `os/parc-ai/modules/ai-brain-smart.sh` | 396 | expr=$(echo "$expr" \| sed -E "s/^(what is\|what's\|calculate\|s |
| `os/parc-ai/modules/ai-engine.sh` | 77 | answer=$(echo "$response" \| python3 -c " |
| `os/parc-ai/modules/ai-engine.sh` | 160 | ai_math_calc "$(echo "$query" \| grep -oP '[0-9+\-*/().^% ]+' |
| `os/parc-ai/modules/ai-master-brain.sh` | 35 | local lower=$(echo "$input" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/ai-master-brain.sh` | 261 | expr=$(echo "$expr" \| sed -E "s/^(what is\|what's\|calculate\|s |
| `os/parc-ai/modules/ai-master-brain.sh` | 293 | if echo "$text" \| grep -qiE "french\|français"; then lang="fr |
| `os/parc-ai/modules/ai-master-brain.sh` | 294 | if echo "$text" \| grep -qiE "spanish\|español"; then lang="es |
| `os/parc-ai/modules/ai-master-brain.sh` | 295 | if echo "$text" \| grep -qiE "german\|deutsch"; then lang="de" |
| `os/parc-ai/modules/ai-master-brain.sh` | 296 | if echo "$text" \| grep -qiE "chinese\|中文"; then lang="zh"; te |
| `os/parc-ai/modules/ai-master-brain.sh` | 297 | if echo "$text" \| grep -qiE "japanese\|日本語"; then lang="ja";  |
| `os/parc-ai/modules/ai-master-brain.sh` | 298 | if echo "$text" \| grep -qiE "korean\|한국어"; then lang="ko"; te |
| `os/parc-ai/modules/ai-master-brain.sh` | 300 | text=$(echo "$text" \| sed 's/^ *//;s/ *$//') |
| `os/parc-ai/modules/ai-master-brain.sh` | 317 | local words=$(echo "$text" \| wc -w) |
| `os/parc-ai/modules/ai-master-brain.sh` | 423 | local folder=$(echo "$action" \| sed -E 's/.*(create\|make)\s+ |
| `os/parc-ai/modules/ai-master-brain.sh` | 426 | local query=$(echo "$action" \| sed -E 's/.*(find\|search)\s*/ |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 475 | local intent=$(echo "$intent_result" \| python3 -c "import sy |
| `os/parc-ai/modules/ai-personality.sh` | 19 | context=$(echo "$context" \| python3 -c " |
| `os/parc-ai/modules/ai-personality.sh` | 48 | local input=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/ai-personality.sh` | 121 | local last_topic=$(echo "$context" \| python3 -c " |
| `os/parc-ai/modules/ai-self-learn.sh` | 101 | local wiki_title=$(echo "$query" \| sed 's/ /_/g') |
| `os/parc-ai/modules/cards-live.sh` | 8 | local b64=$(echo "$html_code" \| base64 -w0 2>/dev/null) |
| `os/parc-ai/modules/cards-visual.sh` | 59 | local caption=$(echo "$img" \| cut -d'\|' -f2) |
| `os/parc-ai/modules/cards-visual.sh` | 60 | local url=$(echo "$img" \| cut -d'\|' -f1) |
| `os/parc-ai/modules/cards-visual.sh` | 78 | local title=$(echo "$meta" \| grep -oP 'property="og:title"\s |
| `os/parc-ai/modules/cards-visual.sh` | 79 | local desc=$(echo "$meta" \| grep -oP 'property="og:descripti |
| `os/parc-ai/modules/cards-visual.sh` | 80 | local img=$(echo "$meta" \| grep -oP 'property="og:image"\s+c |
| `os/parc-ai/modules/cards-visual.sh` | 81 | local site=$(echo "$meta" \| grep -oP 'property="og:site_name |
| `os/parc-ai/modules/cards-visual.sh` | 83 | [ -z "$title" ] && title=$(echo "$meta" \| grep -oP '<title>\ |
| `os/parc-ai/modules/cards-visual.sh` | 86 | [ -z "$site" ] && site=$(echo "$url" \| grep -oP '://\K[^/]*' |
| `os/parc-ai/modules/cards-workspace.sh` | 89 | local encoded=$(echo "$content" \| base64 -w0 2>/dev/null) |
| `os/parc-ai/modules/commerce.sh` | 19 | local pass_hash=$(echo -n "$password" \| sha256sum \| awk '{pr |
| `os/parc-ai/modules/commerce.sh` | 39 | local pass_hash=$(echo -n "$password" \| sha256sum \| awk '{pr |
| `os/parc-ai/modules/conversation.sh` | 17 | echo "{\"ts\":\"$ts\",\"role\":\"$role\",\"text\":\"$(echo " |
| `os/parc-ai/modules/conversation.sh` | 60 | text=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 388 | local input_lower=$(echo "$input" \| tr '[:upper:]' '[:lower: |
| `os/parc-ai/modules/nlp-engine.sh` | 128 | text=$(echo "$text" \| sed 's/^\(.\)/\U\1/') |
| `os/parc-ai/modules/nlp-engine.sh` | 144 | local words=$(echo "$query" \| tr ' ' '\n' \| head -5) |
| `os/parc-ai/modules/nlp-engine.sh` | 145 | NLP_TOPIC=$(echo "$words" \| tail -1) |
| `os/parc-ai/modules/nlp-engine.sh` | 159 | local specials=$(echo "$query" \| grep -oP '[^a-zA-Z0-9\s]' \| |
| `os/parc-ai/modules/nlp-training.sh` | 94 | input_lower=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/nlu.sh` | 9 | text=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/nlu.sh` | 50 | local urls=$(echo "$text" \| grep -oP 'https?://[^\s]+' \| tr  |
| `os/parc-ai/modules/nlu.sh` | 53 | local paths=$(echo "$text" \| grep -oP '/[a-zA-Z0-9_./-]+' \|  |
| `os/parc-ai/modules/nlu.sh` | 56 | local nums=$(echo "$text" \| grep -oP '\b[0-9]+\.?[0-9]*\b' \| |
| `os/parc-ai/modules/nlu.sh` | 59 | local quoted=$(echo "$text" \| grep -oP '"[^"]*"' \| tr '\n' ' |
| `os/parc-ai/modules/nlu.sh` | 63 | entities+="time:$(echo "$text" \| grep -oP '(tomorrow\|today\|n |
| `os/parc-ai/modules/nlu.sh` | 71 | text=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/persona.sh` | 7 | name=$(echo "$1" \| tr '[:upper:]' '[:lower:]') |
| `os/parc-ai/modules/persona.sh` | 65 | local target=$(echo "$text" \| sed -E 's/^(open\|launch\|start\| |
| `os/parc-ai/modules/textgen.sh` | 213 | local words=$(echo "$text" \| wc -w) |
| `os/parc-ai/modules/textgen.sh` | 214 | local chars=$(echo "$text" \| wc -c) |
| `os/parc-ai/modules/textgen.sh` | 215 | local sentences=$(echo "$text" \| grep -oP '[.!?]+' \| wc -l) |
| `os/system/adaptive-power-grid.sh` | 116 | disk_demand=$(echo "scale=0; $disk_demand * 10" \| bc 2>/dev/ |
| `os/system/adaptive-power-grid.sh` | 117 | [ $(echo "$disk_demand > 100" \| bc 2>/dev/null \|\| echo 0) -e |
| `os/system/adaptive-power-grid.sh` | 121 | local net_demand=$(echo "scale=0; $net_rx / 1000000" \| bc 2> |
| `os/system/adaptive-power-grid.sh` | 122 | [ $(echo "$net_demand > 100" \| bc 2>/dev/null \|\| echo 0) -eq |
| `os/system/adaptive-power-grid.sh` | 142 | local cpu=$(echo "$state" \| cut -d'\|' -f2) |
| `os/system/adaptive-power-grid.sh` | 143 | local gpu=$(echo "$state" \| cut -d'\|' -f3) |
| `os/system/adaptive-power-grid.sh` | 144 | local mem=$(echo "$state" \| cut -d'\|' -f4) |
| `os/system/adaptive-power-grid.sh` | 145 | local disk=$(echo "$state" \| cut -d'\|' -f5) |
| `os/system/adaptive-power-grid.sh` | 146 | local net=$(echo "$state" \| cut -d'\|' -f6) |
| `os/system/adaptive-power-grid.sh` | 197 | local cpu_budget=$(echo "$budget" \| cut -d'\|' -f3) |
| `os/system/adaptive-power-grid.sh` | 198 | local gpu_budget=$(echo "$budget" \| cut -d'\|' -f5) |
| `os/system/adaptive-power-grid.sh` | 240 | local net_budget=$(echo "$budget" \| cut -d'\|' -f6) |
| `os/system/adaptive-power-grid.sh` | 393 | echo "  CPU: $(echo $state \| cut -d'\|' -f2)%" |
| `os/system/adaptive-power-grid.sh` | 394 | echo "  GPU: $(echo $state \| cut -d'\|' -f3)%" |
| `os/system/adaptive-power-grid.sh` | 395 | echo "  Memory: $(echo $state \| cut -d'\|' -f4)%" |
| `os/system/adaptive-power-grid.sh` | 396 | echo "  Disk: $(echo $state \| cut -d'\|' -f5)%" |
| `os/system/adaptive-power-grid.sh` | 397 | echo "  Network: $(echo $state \| cut -d'\|' -f6)%" |
| `os/system/adaptive-power-grid.sh` | 405 | echo "  Total: $(echo $budget \| cut -d'\|' -f2)W" |
| `os/system/adaptive-power-grid.sh` | 406 | echo "  CPU: $(echo $budget \| cut -d'\|' -f3)W" |
| `os/system/adaptive-power-grid.sh` | 407 | echo "  Memory: $(echo $budget \| cut -d'\|' -f4)W" |
| `os/system/adaptive-power-grid.sh` | 408 | echo "  GPU: $(echo $budget \| cut -d'\|' -f5)W" |
| `os/system/adaptive-power-grid.sh` | 409 | echo "  Network: $(echo $budget \| cut -d'\|' -f6)W" |
| `os/system/adaptive-power-grid.sh` | 410 | echo "  Disk: $(echo $budget \| cut -d'\|' -f7)W" |
| `os/system/bluetooth-manager.sh` | 92 | local mac=$(echo $line \| awk '{print $2}') |
| `os/system/bluetooth-manager.sh` | 93 | local name=$(echo $line \| cut -d' ' -f3-) |
| `os/system/bluetooth-manager.sh` | 153 | local mac=$(echo $line \| awk '{print $2}') |
| `os/system/bluetooth-manager.sh` | 154 | local name=$(echo $line \| cut -d' ' -f3-) |
| `os/system/bluetooth-manager.sh` | 158 | local connected=$(echo $info \| grep "Connected: yes" \| wc -l |
| `os/system/bluetooth-manager.sh` | 180 | local mac=$(echo $line \| cut -d: -f1-6) |
| `os/system/bluetooth-manager.sh` | 181 | local name=$(echo $line \| cut -d: -f7) |
| `os/system/bluetooth-manager.sh` | 182 | local profile=$(echo $line \| cut -d: -f8) |
| `os/system/bluetooth-manager.sh` | 183 | local auto=$(echo $line \| cut -d: -f9) |
| `os/system/bluetooth-manager.sh` | 221 | local mac=$(echo $line \| awk '{print $2}') |
| `os/system/bluetooth-manager.sh` | 222 | local name=$(echo $line \| cut -d' ' -f3-) |
| `os/system/bluetooth-manager.sh` | 293 | local mac=$(echo $line \| awk '{print $2}') |
| `os/system/context-aware-adaptation.sh` | 133 | key=$(echo "$pref" \| cut -d= -f1) |
| `os/system/context-aware-adaptation.sh` | 134 | value=$(echo "$pref" \| cut -d= -f2) |
| `os/system/digital-twin.sh` | 437 | if [ $(echo "$cpu > $threshold" \| bc 2>/dev/null \|\| echo 0)  |
| `os/system/digital-twin.sh` | 438 | [ $(echo "$mem > $threshold" \| bc 2>/dev/null \|\| echo 0) -eq |
| `os/system/feature-manager.sh` | 123 | name=$(echo "$name" \| tr '_' ' ') |
| `os/system/hardware-detect.sh` | 163 | for layer in $(echo "$layers" \| tr ',' ' '); do |
| `os/system/installer.sh` | 111 | sudo growpart "${partition%?}" "$(echo "$partition" \| grep - |
| `os/system/installer.sh` | 114 | "$(echo "$partition" \| grep -oE '[0-9]+$')" "${new_size:-100 |
| `os/system/korrinos-errors.sh` | 162 | local explanation=$(echo "A command failed with exit code $e |
| `os/system/korrinos-errors.sh` | 172 | local explanation=$(echo "Explain this Linux error in simple |
| `os/system/korrinos-errors.sh` | 182 | local explanation=$(echo "Error: $exit_code — $stderr" \| vok |
| `os/system/korrinos-errors.sh` | 239 | #     eval "original_$(echo $cmd \| tr '-' '_')=$(command -v  |
| `os/system/password-manager.sh` | 140 | local input_hash=$(echo "$master_pass" \| sha256sum \| awk '{p |
| `os/system/password-manager.sh` | 240 | word=$(echo "$word" \| sed 's/./\U&/') |
| `os/system/password-manager.sh` | 307 | local entry=$(echo "$vault" \| jq -r --arg site "$site" '.ent |
| `os/system/password-manager.sh` | 310 | local username=$(echo "$entry" \| jq -r '.username') |
| `os/system/password-manager.sh` | 311 | local password=$(echo "$entry" \| jq -r '.password') |
| `os/system/password-manager.sh` | 394 | local username=$(echo "$credentials" \| cut -d: -f1) |
| `os/system/password-manager.sh` | 395 | local password=$(echo "$credentials" \| cut -d: -f2) |
| `os/system/password-manager.sh` | 430 | local total=$(echo "$vault" \| jq '.entries \| length') |
| `os/system/password-manager.sh` | 446 | local count=$(echo "$vault" \| jq --arg pass "$password" '[.e |
| `os/system/password-manager.sh` | 554 | username=$(echo "$credentials" \| cut -d: -f1) |
| `os/system/password-manager.sh` | 555 | password=$(echo "$credentials" \| cut -d: -f2) |
| `os/system/password-monitor.sh` | 90 | local domain=$(echo "$url" \| sed -E 's\|https?://\|\|') |
| `os/system/password-monitor.sh` | 93 | domain=$(echo "$domain" \| cut -d'/' -f1) |
| `os/system/password-monitor.sh` | 96 | domain=$(echo "$domain" \| sed 's/^www\.//') |
| `os/system/password-monitor.sh` | 180 | local username=$(echo "$credentials" \| cut -d: -f1) |
| `os/system/password-monitor.sh` | 181 | local password=$(echo "$credentials" \| cut -d: -f2) |
| `os/system/predictive-caching.sh` | 101 | predictions="$predictions $(echo $temporal \| cut -d: -f2-)" |
| `os/system/predictive-caching.sh` | 109 | predictions="$predictions $(echo $seq_pattern \| awk -F'' '{p |
| `os/system/predictive-caching.sh` | 117 | predictions="$predictions $(echo $loc_pattern \| cut -d: -f2- |
| `os/system/predictive-caching.sh` | 125 | predictions="$predictions $(echo $app_pattern \| cut -d: -f2- |
| `os/system/self-healing.sh` | 98 | if [ $(echo "$cpu > 90" \| bc 2>/dev/null \|\| echo 0) -eq 1 ]; |
| `os/system/self-healing.sh` | 105 | if [ $(echo "$mem > 90" \| bc 2>/dev/null \|\| echo 0) -eq 1 ]; |
| `os/system/self-healing.sh` | 119 | if [ $(echo "$swap > 80" \| bc 2>/dev/null \|\| echo 0) -eq 1 ] |
| `os/system/self-healing.sh` | 127 | if [ $(echo "$load > $cores * 2" \| bc 2>/dev/null \|\| echo 0) |
| `os/system/self-healing.sh` | 160 | if [ $(echo "$top_mem > 30" \| bc 2>/dev/null \|\| echo 0) -eq  |
| `os/system/self-healing.sh` | 166 | if [ $(echo "$top_cpu > 80" \| bc 2>/dev/null \|\| echo 0) -eq  |
| `os/system/self-healing.sh` | 172 | if [ $(echo "$disk_io > 100" \| bc 2>/dev/null \|\| echo 0) -eq |
| `os/system/self-healing.sh` | 295 | if [ $(echo "$new_cpu > 90" \| bc 2>/dev/null \|\| echo 0) -eq  |
| `os/system/temporal-mapping.sh` | 66 | local avg_cpu=$(echo "$entries" \| awk -F'\|' '{sum+=$5; count |
| `os/system/temporal-mapping.sh` | 67 | local avg_mem=$(echo "$entries" \| awk -F'\|' '{sum+=$6; count |
| `os/system/temporal-mapping.sh` | 68 | local avg_disk=$(echo "$entries" \| awk -F'\|' '{sum+=$7; coun |
| `os/system/temporal-mapping.sh` | 76 | local avg_cpu=$(echo "$entries" \| awk -F'\|' '{sum+=$5; count |
| `os/system/temporal-mapping.sh` | 77 | local avg_mem=$(echo "$entries" \| awk -F'\|' '{sum+=$6; count |
| `os/system/temporal-mapping.sh` | 105 | local forecast_cpu=$(echo "$pattern" \| cut -d: -f2) |
| `os/system/temporal-mapping.sh` | 106 | local forecast_mem=$(echo "$pattern" \| cut -d: -f3) |
| `os/system/temporal-mapping.sh` | 107 | local forecast_disk=$(echo "$pattern" \| cut -d: -f4) |
| `os/system/temporal-mapping.sh` | 121 | local forecast_cpu=$(echo "$forecast_data" \| cut -d'\|' -f2) |
| `os/system/temporal-mapping.sh` | 122 | local forecast_mem=$(echo "$forecast_data" \| cut -d'\|' -f3) |
| `os/system/temporal-mapping.sh` | 127 | if [ $(echo "$forecast_cpu > 80" \| bc 2>/dev/null \|\| echo 0) |
| `os/system/temporal-mapping.sh` | 140 | if [ $(echo "$forecast_mem > 6000" \| bc 2>/dev/null \|\| echo  |
| `os/system/driver-manager/korrinos-drivers.sh` | 89 | device=$(echo "$line" \| sed 's/.*: //') |
| `os/system/driver-manager/korrinos-drivers.sh` | 132 | echo "    $(echo "$line" \| sed 's/.*: //')" |
| `os/system/driver-manager/korrinos-drivers.sh` | 139 | echo "    $(echo "$line" \| sed 's/.*: //')" |
| `os/system/backup/korrinos-backup.sh` | 177 | dest_name=$(echo "$src" \| tr '/' '_' \| sed 's/^_//') |
| `os/system/package-manager/korrinos-pkg.sh` | 686 | echo "Removing $(echo "$to_remove" \| wc -l) packages..." |
| `os/system/package-manager/korrinos-pkg.sh` | 893 | printf "  %-22s %-15s %-25s %s\n" "$(echo $ts \| tr -d ' ')"  |
| `os/system/appstore/korrinos-appstore.sh` | 355 | download_url=$(echo "$app_info" \| python3 -c "import json,sy |
| `os/system/appstore/korrinos-appstore.sh` | 359 | pkg_type=$(echo "$app_info" \| python3 -c "import json,sys; p |
| `os/system/appstore/korrinos-appstore.sh` | 363 | version=$(echo "$app_info" \| python3 -c "import json,sys; pr |
| `os/system/appstore/korrinos-appstore.sh` | 370 | deps=$(echo "$app_info" \| python3 -c "import json,sys; deps= |
| `os/system/security/korrinos-bugfix.sh` | 180 | if [ "$(echo "$available < $min_mb" \| bc 2>/dev/null \|\| echo |
| `os/system/security/korrinos-health.sh` | 63 | load_pct=$(echo "$load_1 $cores" \| awk '{printf "%.0f", ($1/ |
| `os/system/security/korrinos-health.sh` | 215 | temp=$(echo "$gpu_info" \| awk -F, '{print $4}' \| xargs) |
| `os/system/security/korrinos-health.sh` | 268 | svc=$(echo "$svc" \| xargs) |
| `os/system/cloud-sync/korrinos-cloud.sh` | 448 | total=$(echo "$providers" \| wc -l) |
| `os/system/hardware-cert/korrinos-cert.sh` | 107 | "flags_summary": "$(echo "$cpu_flags" \| head -c 200)" |
| `os/system/hardware-cert/korrinos-cert.sh` | 113 | "storage": "$(echo "$storage_info" \| tr '\n' '; ' \| sed 's/" |
| `os/system/hardware-cert/korrinos-cert.sh` | 114 | "gpu": "$(echo "$gpu_info" \| tr '\n' '; ' \| sed 's/"/\\"/g') |
| `os/system/hardware-cert/korrinos-cert.sh` | 115 | "wifi": "$(echo "$wifi_info" \| tr '\n' '; ' \| sed 's/"/\\"/g |
| `os/system/hardware-cert/korrinos-cert.sh` | 118 | "audio": "$(echo "$audio_info" \| tr '\n' '; ' \| sed 's/"/\\" |
| `os/system/hardware-cert/korrinos-cert.sh` | 119 | "network": "$(echo "$net_info" \| tr '\n' '; ' \| sed 's/"/\\" |
| `os/system/hardware-cert/korrinos-cert.sh` | 132 | echo "GPU: $(echo "$gpu_info" \| head -1 \| sed 's/.*: //')" |
| `os/system/hardware-cert/korrinos-cert.sh` | 133 | echo "WiFi: $(echo "$wifi_info" \| head -1 \| sed 's/.*: //')" |
| `os/system/hardware-cert/korrinos-cert.sh` | 134 | echo "Audio: $(echo "$audio_info" \| head -1)" |
| `os/system/hardware-cert/korrinos-cert.sh` | 588 | total_power=$(echo "scale=1; $current * $voltage / 100000000 |
| `os/system/desktop-env/korrinos-desktop.sh` | 663 | keyval=$(echo "$key" \| sed 's/+/<Primary>/g; s/super/<Super> |
| `os/system/desktop-env/korrinos-desktop.sh` | 1361 | count=$(echo "$monitors" \| wc -l) |
| `os/system/desktop-env/korrinos-desktop.sh` | 1373 | primary=$(echo "$monitors" \| head -1) |
| `os/system/desktop-env/korrinos-desktop.sh` | 1375 | secondary=$(echo "$monitors" \| tail -1) |
| `os/system/update-system/korrinos-update.sh` | 212 | echo "Removing $(echo "$to_remove" \| wc -l) packages..." |
| `os/system/update-system/korrinos-update.sh` | 220 | echo "Installing $(echo "$to_install" \| wc -l) packages..." |
| `os/system/update-system/korrinos-update.sh` | 751 | printf "%-24s \| %-9s \| %-8s \| %s\n" "$(echo $ts \| tr -d ' ') |
| `os/system/installer/korrinos-installer.sh` | 113 | name=$(echo "$line" \| awk '{print $1}') |
| `os/system/installer/korrinos-installer.sh` | 114 | size=$(echo "$line" \| awk '{print $2}') |
| `os/system/installer/korrinos-installer.sh` | 115 | model=$(echo "$line" \| awk '{print $5, $6}') |
| `os/system/mobile-companion/korrinos-mobile.sh` | 163 | subnet=$(echo "$my_ip" \| cut -d. -f1-3) |
| `os/system/mobile-companion/korrinos-mobile.sh` | 321 | adb shell input text "$(echo "$clip" \| sed 's/ /%s/g')" 2>/d |
| `os/system/mobile-companion/korrinos-mobile.sh` | 781 | serial=$(echo "$line" \| awk '{print $1}') |
| `os/system/network/korrinos-network.sh` | 118 | name=$(echo "$line" \| awk -F: '{print $2}' \| xargs) |
| `os/system/network/korrinos-network.sh` | 119 | state=$(echo "$line" \| awk '{print $NF}') |
| `os/system/network/korrinos-network.sh` | 121 | mac=$(echo "$line" \| grep -oP 'link/ether \K[^ ]+' \|\| echo " |
| `os/system/network/korrinos-network.sh` | 123 | mtu=$(echo "$line" \| grep -oP 'mtu \K[0-9]+' \|\| echo "?") |
| `os/system/network/korrinos-network.sh` | 326 | conn_name=$(echo "$line" \| awk '{print $1}') |
| `os/system/enterprise/korrinos-enterprise.sh` | 199 | krb5_realm = $(echo "$domain" \| tr '[:lower:]' '[:upper:]') |
| `os/system/enterprise/korrinos-enterprise.sh` | 441 | .$(echo "$realm" \| tr '[:upper:]' '[:lower:]') = $realm |
| `os/system/enterprise/korrinos-enterprise.sh` | 442 | $(echo "$realm" \| tr '[:upper:]' '[:lower:]') = $realm |
| `os/system/enterprise/korrinos-enterprise.sh` | 534 | http_code=$(echo "$response" \| tail -1) |
| `os/system/enterprise/korrinos-enterprise.sh` | 536 | body=$(echo "$response" \| head -n -1) |
| `os/territories/secure/privacy-ledger.sh` | 19 | local h; h=$(echo -n "$row" \| sha256sum \| awk '{print $1}') |
| `os/territories/secure/privacy-ledger.sh` | 55 | prev=$(echo -n "$prev" \| tr -d ' ') |
| `os/territories/vibe-address/core/action.sh` | 44 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/action.sh` | 48 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/action.sh` | 51 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/action.sh` | 55 | local count; count=$(echo "$fetched" \| sed '/^$/d' \| wc -l \| |
| `os/territories/vibe-address/core/action.sh` | 63 | scored=$(echo "$fetched" \| while IFS= read -r cand; do |
| `os/territories/vibe-address/core/action.sh` | 64 | local fp=$(echo "$cand" \| cut -d'\|' -f5) |
| `os/territories/vibe-address/core/action.sh` | 69 | ranked=$(echo "$scored" \| ve_rank_run 8) |
| `os/territories/vibe-address/core/action.sh` | 80 | rfp=$(echo "$line" \| cut -d'\|' -f3) |
| `os/territories/vibe-address/core/action.sh` | 83 | rpath=$(echo "$line" \| cut -d'\|' -f7) |
| `os/territories/vibe-address/core/action.sh` | 84 | rcat=$(echo "$line"  \| cut -d'\|' -f9) |
| `os/territories/vibe-address/core/action.sh` | 91 | done <<< "$(echo "$ranked")" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/action.sh` | 159 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/action.sh` | 162 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/adapt.sh` | 88 | local with_cat; with_cat=$(echo "$window" \| grep -c '\|yes\|'  |
| `os/territories/vibe-address/core/adapt.sh` | 89 | local with_time; with_time=$(echo "$window" \| grep -c '\|yes\| |
| `os/territories/vibe-address/core/adapt.sh` | 90 | local avg_cand; avg_cand=$(echo "$window" \| awk -F'\|' '{s+=$ |
| `os/territories/vibe-address/core/audit.sh` | 38 | echo "    name    : $(echo "$nametoks" \| tr '\n' ' ' \| sed ' |
| `os/territories/vibe-address/core/audit.sh` | 78 | local pk; pk=$(echo "$path" \| tr ':/' ' ' \| tr -cd 'a-z0-9 ' |
| `os/territories/vibe-address/core/audit.sh` | 95 | pos_raw=$(echo "$ops" \| grep '^POS=' \| cut -d= -f2- \|\| true) |
| `os/territories/vibe-address/core/audit.sh` | 96 | qneg_raw=$(echo "$ops" \| grep '^NEG=' \| cut -d= -f2- \|\| true |
| `os/territories/vibe-address/core/audit.sh` | 103 | intent_time=$(echo "$intent" \| grep -oP 'TIME=\K\S+' \|\| true |
| `os/territories/vibe-address/core/audit.sh` | 104 | intent_cat=$(echo "$intent" \| grep -oP 'CAT=\K\S+' \|\| true) |
| `os/territories/vibe-address/core/audit.sh` | 105 | intent_src=$(echo "$intent" \| grep -oP 'SOURCE=\K\S+' \|\| tru |
| `os/territories/vibe-address/core/audit.sh` | 141 | n=$(echo "$cand" \| sed '/^$/d' \| wc -l \| tr -d ' ') |
| `os/territories/vibe-address/core/audit.sh` | 143 | [ "$lv" -ge 1 ] && [ -z "$(echo "$cand" \| sed '/^$/d')" ] && |
| `os/territories/vibe-address/core/index.sh` | 103 | done <<< "$(echo "$tokens" \| tr ' ' '\n' \| sed '/^$/d')" > " |
| `os/territories/vibe-address/core/index.sh` | 106 | awk -v n=$(echo "$tokens" \| wc -w) \ |
| `os/territories/vibe-address/core/index.sh` | 141 | local fp=$(echo "$line" \| cut -d'\|' -f5) |
| `os/territories/vibe-address/core/index.sh` | 145 | local catpath=$(echo "$line" \| cut -d'\|' -f6) |
| `os/territories/vibe-address/core/index.sh` | 146 | local path=$(echo "$line" \| cut -d'\|' -f4) |
| `os/territories/vibe-address/core/index.sh` | 147 | local epoch=$(echo "$line" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/index.sh` | 148 | local vtype=$(echo "$line" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/index.sh` | 149 | local meta=$(echo "$line" \| cut -d'\|' -f7) |
| `os/territories/vibe-address/core/index.sh` | 150 | local source=$(echo "$line" \| cut -d'\|' -f3) |
| `os/territories/vibe-address/core/ingest.sh` | 208 | local mid; mid=$(echo "$host" \| awk -F'.' '{if(NF>=2) print  |
| `os/territories/vibe-address/core/ingest.sh` | 258 | n=$(echo "$typed" \| wc -w \| tr -d ' ') |
| `os/territories/vibe-address/core/ingest.sh` | 273 | result=$(echo "$conf_lines" \| { |
| `os/territories/vibe-address/core/ingest.sh` | 307 | echo "$result" \| head -20 \| zenity --info --title="Searchie" |
| `os/territories/vibe-address/core/ingest.sh` | 313 | app=$(echo "$app" \| tr '[:upper:]' '[:lower:]' \| sed 's/\.de |
| `os/territories/vibe-address/core/ir.sh` | 158 | local qtok_count; qtok_count=$(echo "$qtokens" \| wc -w \| tr  |
| `os/territories/vibe-address/core/lexin.sh` | 245 | n=$(echo "$s" \| grep -oE '\b([0-9]+\|a\|an\|one\|two\|three\|four\| |
| `os/territories/vibe-address/core/lexin.sh` | 248 | local rest; rest=$(echo "$s" \| sed "s/$n//") |
| `os/territories/vibe-address/core/lexin.sh` | 249 | unit=$(echo "$rest" \| grep -oE '\b(seconds?\|mins?\|minutes?\|h |
| `os/territories/vibe-address/core/lexin.sh` | 257 | forty) num=40;; fifty) num=50;; [0-9]*) num=$(echo "$n" \| tr |
| `os/territories/vibe-address/core/lsh.sh` | 77 | bucket=$(echo "$sig" \| cut -d' ' -f$lo-$hi \| tr ' ' ',' \| ck |
| `os/territories/vibe-address/core/lsh.sh` | 94 | local bucket; bucket=$(echo "$mysig" \| cut -d' ' -f$lo-$hi \| |
| `os/territories/vibe-address/core/lsh.sh` | 105 | a=$(echo "$mysig" \| cut -d' ' -f$i) |
| `os/territories/vibe-address/core/lsh.sh` | 106 | b=$(echo "$osig" \| cut -d' ' -f$i) |
| `os/territories/vibe-address/core/lsh.sh` | 162 | nama=$(ve_ingest_name_tokens "$(echo "$ea" \| cut -d'\|' -f4)" |
| `os/territories/vibe-address/core/lsh.sh` | 163 | namb=$(ve_ingest_name_tokens "$(echo "$eb" \| cut -d'\|' -f4)" |
| `os/territories/vibe-address/core/markov.sh` | 52 | done <<< "$(echo "$tokens" \| tr ' ' '\n' \| sed '/^$/d')" |
| `os/territories/vibe-address/core/markov.sh` | 84 | }" "$(echo "$delta" \| sed 's/^ //')")" |
| `os/territories/vibe-address/core/markov.sh` | 96 | done <<< "$(echo "$ctx" \| tr ' ' '\n' \| sed '/^$/d')" |
| `os/territories/vibe-address/core/markov.sh` | 127 | cur=$(echo "$cur" \| awk -v w="$next" '{ if (NF>=2) sub(/^[^  |
| `os/territories/vibe-address/core/markov.sh` | 142 | next=$(echo "$pred" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/markov.sh` | 143 | local pct=$(echo "$pred" \| cut -d'\|' -f3) |
| `os/territories/vibe-address/core/markov.sh` | 147 | cur=$(echo "$cur" \| awk -v w="$next" '{ if (NF>=2) sub(/^[^  |
| `os/territories/vibe-address/core/match.sh` | 77 | qcount=$(echo "$qtok" \| sed '/^$/d' \| wc -l \| tr -d ' ') |
| `os/territories/vibe-address/core/match.sh` | 78 | ecount=$(echo "$etok" \| sed '/^$/d' \| wc -l \| tr -d ' ') |
| `os/territories/vibe-address/core/match.sh` | 207 | local eepoch=$(echo "$envelope" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/match.sh` | 208 | local epath=$(echo "$envelope" \| cut -d'\|' -f4) |
| `os/territories/vibe-address/core/match.sh` | 209 | local ecat=$(echo "$envelope" \| cut -d'\|' -f6) |
| `os/territories/vibe-address/core/match.sh` | 210 | local etype=$(echo "$envelope" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/match.sh` | 214 | local edepth; edepth=$(echo "$ecat" \| tr ':' '\n' \| wc -l \|  |
| `os/territories/vibe-address/core/phoneme.sh` | 176 | local tmp; tmp=$(echo "$w" \| tr -cd 'aeiou') |
| `os/territories/vibe-address/core/phoneme.sh` | 194 | local v2; v2=$(echo "$stripped" \| tr -cd 'aeiou') |
| `os/territories/vibe-address/core/prf.sh` | 26 | local epath=$(echo "$envelope" \| cut -d'\|' -f4) |
| `os/territories/vibe-address/core/prf.sh` | 27 | local etype=$(echo "$envelope" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/prf.sh` | 28 | local ecat=$(echo "$envelope" \| cut -d'\|' -f6) |
| `os/territories/vibe-address/core/prf.sh` | 92 | local env; env=$(echo "${scored[$i]}" \| cut -d'\|' -f11-) |
| `os/territories/vibe-address/core/prf.sh` | 108 | expansion=$(echo "$fbset" \| sed '/^$/d' \| awk -F: '{cnt[$1]+ |
| `os/territories/vibe-address/core/prf.sh` | 112 | local qexp; qexp=$(printf '%s\n%s\n' "$qvec" "$(echo "$expan |
| `os/territories/vibe-address/core/query.sh` | 126 | local pos;  pos=$(echo "$ops" \| grep '^POS=' \| cut -d= -f2-) |
| `os/territories/vibe-address/core/query.sh` | 127 | local neg;  neg=$(echo "$ops" \| grep '^NEG=' \| cut -d= -f2-) |
| `os/territories/vibe-address/core/query.sh` | 163 | pos=$(echo "$pos" \| sed 's/^ *//;s/ *$//') |
| `os/territories/vibe-address/core/query.sh` | 164 | neg=$(echo "$neg" \| sed 's/^ *//;s/ *$//') |
| `os/territories/vibe-address/core/query.sh` | 185 | if echo "$badfps" \| grep -qx "$(echo "$env" \| cut -d'\|' -f5) |
| `os/territories/vibe-address/core/query.sh` | 201 | local pos_raw; pos_raw=$(echo "$ops" \| grep '^POS=' \| cut -d |
| `os/territories/vibe-address/core/query.sh` | 202 | local qneg_raw; qneg_raw=$(echo "$ops" \| grep '^NEG=' \| cut  |
| `os/territories/vibe-address/core/query.sh` | 207 | qneg=$(echo "$qneg" \| sed 's/ *$//;s/^ *//') |
| `os/territories/vibe-address/core/query.sh` | 216 | local intent_time; intent_time=$(echo "$intent" \| grep -oP ' |
| `os/territories/vibe-address/core/query.sh` | 217 | local intent_cat;  intent_cat=$(echo "$intent" \| grep -oP 'C |
| `os/territories/vibe-address/core/query.sh` | 218 | local intent_src;  intent_src=$(echo "$intent" \| grep -oP 'S |
| `os/territories/vibe-address/core/query.sh` | 256 | strict_count=$(echo "$candidates" \| sed '/^$/d' \| wc -l \| tr |
| `os/territories/vibe-address/core/query.sh` | 272 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/query.sh` | 275 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/query.sh` | 278 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/query.sh` | 281 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/query.sh` | 284 | if [ "$(echo "$fetched" \| sed '/^$/d' \| wc -l \| tr -d ' ')"  |
| `os/territories/vibe-address/core/query.sh` | 295 | fetched=$(echo "$fetched" \| sed '/^$/d' \| sort -u) |
| `os/territories/vibe-address/core/query.sh` | 300 | fetched=$(echo "$fetched" \| sed '/^$/d' \| sort -u) |
| `os/territories/vibe-address/core/query.sh` | 303 | local cand_count; cand_count=$(echo "$fetched" \| sed '/^$/d' |
| `os/territories/vibe-address/core/query.sh` | 322 | scored=$(echo "$fetched" \| while IFS= read -r cand; do |
| `os/territories/vibe-address/core/query.sh` | 323 | local fp=$(echo "$cand" \| cut -d'\|' -f5) |
| `os/territories/vibe-address/core/query.sh` | 331 | if [ "$(echo "$scored" \| sed '/^$/d' \| wc -l \| tr -d ' ')" - |
| `os/territories/vibe-address/core/query.sh` | 333 | scored=$(echo "$scored" \| ve_prf_rerank "$tokens" 2>/dev/nul |
| `os/territories/vibe-address/core/query.sh` | 339 | ranked=$(echo "$scored" \| ve_rank_run "$k") |
| `os/territories/vibe-address/core/query.sh` | 416 | local eepoch; eepoch=$(echo "$envelope" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/query.sh` | 479 | done <<< "$(echo "$tokens" \| tr ' ' '\n' \| sed '/^$/d')" |
| `os/territories/vibe-address/core/query.sh` | 481 | local fps; fps=$(ve_index_tokens_to_fps "$(echo "$corrected" |
| `os/territories/vibe-address/core/query.sh` | 486 | local e; e=$(echo "$envelope" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/query.sh` | 502 | done <<< "$(echo "$tokens" \| tr ' ' '\n' \| sed '/^$/d')" |
| `os/territories/vibe-address/core/query.sh` | 508 | local e; e=$(echo "$envelope" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/query.sh` | 543 | candidates=$(echo "$catfps" \| while IFS= read -r fp; do |
| `os/territories/vibe-address/core/query.sh` | 572 | local eepoch; eepoch=$(echo "$env" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/rank.sh` | 101 | score=$(echo "$line" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/rank.sh` | 102 | cat1=$(echo "$line" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/rank.sh` | 120 | local score=$(echo "$line" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/rank.sh` | 121 | local fp=$(echo "$line" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/rank.sh` | 153 | local fp=$(echo "$line" \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/rank.sh` | 154 | local m1=$(echo "$line" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/rank.sh` | 155 | local m2=$(echo "$line" \| cut -d'\|' -f3) |
| `os/territories/vibe-address/core/rank.sh` | 156 | local m3=$(echo "$line" \| cut -d'\|' -f4) |
| `os/territories/vibe-address/core/rank.sh` | 157 | local m4=$(echo "$line" \| cut -d'\|' -f5) |
| `os/territories/vibe-address/core/rank.sh` | 158 | local m5=$(echo "$line" \| cut -d'\|' -f6) |
| `os/territories/vibe-address/core/rank.sh` | 159 | local m6=$(echo "$line" \| cut -d'\|' -f7) |
| `os/territories/vibe-address/core/rank.sh` | 160 | local m7=$(echo "$line" \| cut -d'\|' -f8) |
| `os/territories/vibe-address/core/rank.sh` | 161 | local m8=$(echo "$line" \| cut -d'\|' -f9) |
| `os/territories/vibe-address/core/rank.sh` | 162 | local m9=$(echo "$line" \| cut -d'\|' -f10) |
| `os/territories/vibe-address/core/rank.sh` | 163 | local env=$(echo "$line" \| cut -d'\|' -f11-) |
| `os/territories/vibe-address/core/rank.sh` | 165 | local cat1=$(echo "$env" \| cut -d'\|' -f4 \| cut -d: -f1) |
| `os/territories/vibe-address/core/sarray.sh` | 29 | local total; total=$(echo "$fplist" \| sed '/^$/d' \| wc -l \|  |
| `os/territories/vibe-address/core/sarray.sh` | 31 | for fp in $(echo "$fplist" \| tail -"$keep"); do |
| `os/territories/vibe-address/core/sarray.sh` | 36 | path=$(echo "$env" \| cut -d'\|' -f4) |
| `os/territories/vibe-address/core/sarray.sh` | 37 | vtype=$(echo "$env" \| cut -d'\|' -f2) |
| `os/territories/vibe-address/core/sarray.sh` | 38 | cat=$(echo "$env" \| cut -d'\|' -f6) |
| `os/territories/vibe-address/core/selftest.sh` | 62 | check "lexin ring reaches picture" "1" "$(echo "$ring" \| gre |
| `os/territories/vibe-address/core/selftest.sh` | 66 | check "time window lo<hi" "1" "$( [ "$(echo "$w" \| awk '{pri |
| `os/territories/vibe-address/core/selftest.sh` | 96 | check "prf query vector tf" "1" "$(echo "$qv" \| awk -F'[: ]' |
| `os/territories/vibe-address/core/selftest.sh` | 196 | while IFS= read -r rw; do [ -n "$rw" ] && rnd_arr+=("$rw");  |
| `os/territories/vibe-address/core/selftest.sh` | 207 | if [ "$fuzzalign" -ge 1 ] 2>/dev/null && [ "$(echo "$fuzzq"  |
| `os/territories/vibe-address/core/selftest.sh` | 214 | local tlrows; tlrows=$(echo "$tl_out" \| grep -c '^  [0-9]\{4 |
| `os/territories/vibe-address/core/selftest.sh` | 223 | local burned_row; burned_row=$(echo "$burnout" \| grep -c "du |
| `os/territories/vibe-address/core/store.sh` | 129 | local c; c=$(echo "$line" \| awk '{print $2}') |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 88 | rel=$(echo "$dir" \| sed "s\|$VIBE_SCAN_ROOT/\|\|;s\|$VIBE_SCAN_R |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 106 | top=$(echo "$rel" \| cut -d/ -f1) |
| `os/territories/game/perf-tune.sh` | 17 | pi=$(echo "scale=6; 4*a(1)" \| bc -l 2>/dev/null) \|\| pi=$(awk |
| `os/territories/hack/sdr-isolation.sh` | 44 | sudo usbip bind --$(echo "$usb" \| tr ':' ' ') 2>/dev/null \|\| |
| `os/data/user-profiles.sh` | 63 | local name=$(echo "$line" \| cut -d: -f1) |
| `os/data/user-profiles.sh` | 64 | local desc=$(echo "$line" \| cut -d: -f2) |
| `os/data/user-profiles.sh` | 65 | local packages=$(echo "$line" \| cut -d: -f3) |
| `os/ai/voice-assistant.sh` | 97 | input=$(echo "$input" \| tr '[:upper:]' '[:lower:]') |
| `os/ai/voice-engine.sh` | 108 | input=$(echo "$input" \| tr '[:upper:]' '[:lower:]') |
| `os/ai/voice-engine.sh` | 141 | local weight=$(echo "$pattern_entry" \| cut -d: -f1) |
| `os/ai/voice-engine.sh` | 142 | local pattern=$(echo "$pattern_entry" \| cut -d: -f2-) |
| `os/drivers/driver-manager.sh` | 28 | echo -e "  ${GREEN}${NC} NVIDIA GPU detected: $(echo $gpu_in |
| `os/drivers/driver-manager.sh` | 31 | echo -e "  ${GREEN}${NC} AMD GPU detected: $(echo $gpu_info  |
| `os/drivers/driver-manager.sh` | 34 | echo -e "  ${GREEN}${NC} Intel GPU detected: $(echo $gpu_inf |
| `os/drivers/driver-manager.sh` | 68 | echo -e "  ${GREEN}${NC} Audio device: $(echo $audio_info \|  |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 23 | protection(){ echo "=== Speaker Protection ==="; cat /sys/cl |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 16 | echo "  QR: $(echo -n "tinker://$PORT?key=$KEY" \| qrencode - |
| `os/hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 378 | mc="$(echo "$c_out" \| grep -oP 'max_core=\K[0-9]+')" |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 46 | xrandr --output $(xrandr \| grep " connected" \| head -1 \| awk |
| `os/hardware-tech/hardware-tuning/thermal-tuning.sh` | 18 | [ -n "$temp" ] && echo "  $t: $(echo "scale=1; $temp/1000" \| |
| `os/desktop/clipboard-manager.sh` | 19 | log_clip "$(echo "$1" \| head -c 100)" |
| `os/desktop/clipboard-manager.sh` | 75 | log_clip "$(echo "$current" \| head -c 100)" |
| `os/desktop/gestures.sh` | 97 | local fingers=$(echo $event \| grep -oP '\d fingers' \| grep - |
| `os/desktop/gestures.sh` | 127 | local fingers=$(echo $event \| grep -oP '\d fingers' \| grep - |
| `os/desktop/screen-tools.sh` | 65 | local x=$(echo "$geom" \| grep -oP 'Position: \K\d+') |
| `os/desktop/screen-tools.sh` | 66 | local y=$(echo "$geom" \| grep -oP ', \K\d+') |
| `os/desktop/screen-tools.sh` | 67 | local w=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/desktop/screen-tools.sh` | 68 | local h=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/desktop/screen-tools.sh` | 87 | local x=$(echo "$geom" \| grep -oP 'Position: \K\d+') |
| `os/desktop/screen-tools.sh` | 88 | local y=$(echo "$geom" \| grep -oP ', \K\d+') |
| `os/desktop/screen-tools.sh` | 89 | local w=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/desktop/screen-tools.sh` | 90 | local h=$(echo "$geom" \| grep -oP 'Geometry: \K\d+x\d+' \| cu |
| `os/desktop/shortcuts.sh` | 108 | key=$(echo $key \| xargs) |
| `os/desktop/shortcuts.sh` | 109 | action=$(echo $action \| xargs) |
| `os/desktop/shortcuts.sh` | 137 | combo=$(echo "$combo" \| xargs); action=$(echo "$action" \| xa |
| `os/desktop/shortcuts.sh` | 140 | local xk=$(echo "$combo" \| sed 's/Super+/Mod4+/g; s/ /+/g') |
| `os/desktop/shortcuts.sh` | 287 | W=$(echo "$res" \| grep -oE '[0-9]+x[0-9]+' \| head -1 \| cut - |
| `os/desktop/shortcuts.sh` | 288 | H=$(echo "$res" \| grep -oE '[0-9]+x[0-9]+' \| head -1 \| cut - |
| `os/desktop/shortcuts.sh` | 473 | key=$(echo $key \| xargs) |
| `os/desktop/shortcuts.sh` | 474 | action=$(echo $action \| xargs) |
| `os/desktop/text-expander.sh` | 58 | expansion=$(echo "$expansion" \| sed 's/\\n/\n/g') |
| `os/desktop/text-expander.sh` | 124 | out=$(echo "$line" \| quick_expand) |
| `os/desktop/text-expander.sh` | 137 | result=$(echo "$result" \| sed "s\|$abbrev\|$expansion\|g") |
| `os/desktop/theme-manager.sh` | 52 | SECONDARY=$(echo $accent \| sed 's/^[0-9a-f]\{6\}/&/' \| sed ' |
| `os/desktop/theme-manager.sh` | 96 | SECONDARY=$(echo $accent) |
| `os/desktop/tiling.sh` | 149 | local pos=$(echo $state \| grep -oP 'Position: \K[^,]+') |
| `os/desktop/tiling.sh` | 150 | local size=$(echo $state \| grep -oP 'Geometry: \K[^ ]+') |
| `os/desktop/tiling.sh` | 164 | local count=$(echo $windows \| wc -w) |
| `os/desktop/tiling.sh` | 197 | local cols=$(echo "sqrt($count)" \| bc) |
| `os/desktop/tiling.sh` | 215 | local slave_count=$(echo $slaves \| wc -w) |
| `os/desktop/virtual-desktops.sh` | 206 | local num=$(echo $line \| awk '{print $1}') |
| `os/desktop/virtual-desktops.sh` | 239 | local win_id=$(echo $line \| awk '{print $1}') |
| `os/desktop/virtual-desktops.sh` | 240 | local win_desktop=$(echo $line \| awk '{print $2}') |
| `os/desktop/virtual-desktops.sh` | 241 | local win_title=$(echo $line \| cut -d' ' -f4-) |
| `os/desktop/window-manager.sh` | 13 | local id=$(echo $line \| awk '{print $1}') |
| `os/desktop/window-manager.sh` | 14 | local desktop=$(echo $line \| awk '{print $2}') |
| `os/desktop/window-manager.sh` | 15 | local title=$(echo $line \| cut -d' ' -f4-) |
| `os/desktop/window-manager.sh` | 115 | local num=$(echo $line \| awk '{print $1}') |

### P01 line >100 cols — 319

| file | line | detail |
|------|------|--------|
| `os/docs/_gap_analysis.py` | 46 | 103 cols |
| `os/docs/_gap_analysis.py` | 408 | 101 cols |
| `os/docs/_gap_analysis.py` | 757 | 108 cols |
| `os/docs/_gap_analysis.py` | 1154 | 107 cols |
| `os/docs/_professionalism_audit.py` | 89 | 104 cols |
| `os/docs/_professionalism_audit.py` | 159 | 104 cols |
| `os/docs/_professionalism_audit.py` | 265 | 112 cols |
| `os/docs/_professionalism_audit.py` | 275 | 102 cols |
| `os/software-gpu/compiler/korlangc.py` | 21 | 101 cols |
| `os/software-gpu/compiler/korlangc.py` | 54 | 118 cols |
| `os/software-gpu/compiler/korlangc.py` | 62 | 138 cols |
| `os/software-gpu/compiler/korlangc.py` | 64 | 108 cols |
| `os/parc-ai/parc-ai-gui.py` | 31 | 108 cols |
| `os/parc-ai/model/parc_trainer.py` | 187 | 155 cols |
| `os/parc-ai/model/parc_trainer.py` | 275 | 102 cols |
| `os/parc-ai/model/train_tinkerai_5b.py` | 144 | 110 cols |
| `os/parc-ai/model/train_tinkerai_5b.py` | 158 | 121 cols |
| `os/parc-ai/model/train_tinkerai_5b.py` | 178 | 141 cols |
| `os/parc-ai/model/train_tinkerai_5b.py` | 194 | 101 cols |
| `os/parc-ai/model/train_tinkeria_3b.py` | 43 | 102 cols |
| `os/parc-ai/model/train_tinkeria_3b.py` | 199 | 126 cols |
| `os/parc-ai/model/train_tinkeria_3b_cpu.py` | 140 | 133 cols |
| `os/parc-ai/model/train_tinkeria_3b_cpu.py` | 187 | 118 cols |
| `os/parc-ai/model/train_tinkeria_galore.py` | 129 | 110 cols |
| `os/parc-ai/model/train_tinkeria_galore.py` | 142 | 121 cols |
| `os/parc-ai/model/train_tinkeria_galore.py` | 161 | 141 cols |
| `os/parc-ai/model/train_tinkeria_galore.py` | 178 | 103 cols |
| `os/parc-ai/model/train_tinkeria_galore.py` | 191 | 104 cols |
| `os/parc-ai/model/train_tinkeria_lora.py` | 144 | 110 cols |
| `os/parc-ai/model/train_tinkeria_lora.py` | 158 | 121 cols |
| `os/parc-ai/model/train_tinkeria_lora.py` | 178 | 141 cols |
| `os/parc-ai/model/train_tinkeria_lora.py` | 195 | 101 cols |
| `os/tinker-cowork/tinker-cowork.py` | 90 | 126 cols |
| `os/tinker-cowork/tinker-cowork.py` | 91 | 160 cols |
| `os/tinker-cowork/tinker-cowork.py` | 92 | 120 cols |
| `os/tinker-cowork/tinker-cowork.py` | 162 | 107 cols |
| `os/tinker-cowork/tinker-cowork.py` | 164 | 101 cols |
| `os/tinker-cowork/tinker-cowork.py` | 221 | 160 cols |
| `os/tinker-cowork/tinker-cowork.py` | 240 | 165 cols |
| `os/tinker-cowork/tinker-cowork.py` | 271 | 101 cols |
| `os/tinker-cowork/tinker-cowork.py` | 282 | 123 cols |
| `os/tinker-cowork/tinker-cowork.py` | 290 | 115 cols |
| `os/tinker-cowork/tinker-cowork.py` | 291 | 138 cols |
| `os/tinker-cowork/tinker-cowork.py` | 300 | 104 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 98 | 102 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 103 | 104 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 118 | 108 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 133 | 107 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 134 | 116 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 175 | 101 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 272 | 113 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 277 | 107 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 280 | 107 cols |
| `os/iso-builder-pro/parc-iso-pro.py` | 306 | 122 cols |
| `os/onboarding/parc-onboard.py` | 361 | 109 cols |
| `os/onboarding/parc-onboard.py` | 386 | 101 cols |
| `os/vokk/data_generator.py` | 145 | 102 cols |
| `os/vokk/data_generator.py` | 148 | 123 cols |
| `os/vokk/data_generator.py` | 285 | 148 cols |
| `os/vokk/image_module.py` | 51 | 109 cols |
| `os/vokk/small_rnn_model.py` | 118 | 105 cols |
| `os/vokk/small_rnn_model.py` | 447 | 102 cols |
| `os/vokk/small_rnn_model.py` | 497 | 119 cols |
| `os/vokk/small_rnn_model.py` | 596 | 126 cols |
| `os/vokk/vokk-v4-engine.py` | 56 | 139 cols |
| `os/vokk/vokk-v4-engine.py` | 64 | 131 cols |
| `os/vokk/vokk-v4-engine.py` | 72 | 106 cols |
| `os/vokk/vokk-v4-engine.py` | 237 | 101 cols |
| `os/vokk/vokk-v4-engine.py` | 322 | 101 cols |
| `os/vokk/vokk-v4-engine.py` | 334 | 113 cols |
| `os/vokk/vokk-v4-engine.py` | 410 | 107 cols |
| `os/vokk/vokk-v4-engine.py` | 448 | 116 cols |
| `os/vokk/vokk-v4-engine.py` | 518 | 122 cols |
| `os/vokk/vokk.py` | 71 | 121 cols |
| `os/vokk/vokk.py` | 72 | 121 cols |
| `os/vokk/vokk.py` | 73 | 121 cols |
| `os/vokk/vokk.py` | 136 | 134 cols |
| `os/vokk/vokk.py` | 137 | 137 cols |
| `os/vokk/vokk.py` | 138 | 146 cols |
| `os/vokk/vokk.py` | 139 | 128 cols |
| `os/vokk/vokk.py` | 140 | 127 cols |
| `os/vokk/vokk.py` | 141 | 145 cols |
| `os/vokk/vokk.py` | 142 | 151 cols |
| `os/vokk/vokk.py` | 145 | 179 cols |
| `os/vokk/vokk.py` | 146 | 176 cols |
| `os/vokk/vokk.py` | 147 | 134 cols |
| `os/vokk/vokk.py` | 148 | 211 cols |
| `os/vokk/vokk.py` | 151 | 152 cols |
| `os/vokk/vokk.py` | 152 | 155 cols |
| `os/vokk/vokk.py` | 153 | 165 cols |
| `os/vokk/vokk.py` | 156 | 205 cols |
| `os/vokk/vokk.py` | 157 | 134 cols |
| `os/vokk/vokk.py` | 158 | 114 cols |
| `os/vokk/vokk.py` | 160 | 131 cols |
| `os/vokk/vokk.py` | 163 | 155 cols |
| `os/vokk/vokk.py` | 164 | 140 cols |
| `os/vokk/vokk.py` | 167 | 315 cols |
| `os/vokk/vokk.py` | 168 | 191 cols |
| `os/vokk/vokk.py` | 169 | 133 cols |
| `os/vokk/vokk.py` | 170 | 145 cols |
| `os/vokk/vokk.py` | 171 | 140 cols |
| `os/vokk/vokk.py` | 172 | 168 cols |
| `os/vokk/vokk.py` | 173 | 154 cols |
| `os/vokk/vokk.py` | 174 | 179 cols |
| `os/vokk/vokk.py` | 175 | 221 cols |
| `os/vokk/vokk.py` | 176 | 150 cols |
| `os/vokk/vokk.py` | 177 | 171 cols |
| `os/vokk/vokk.py` | 178 | 187 cols |
| `os/vokk/vokk.py` | 179 | 140 cols |
| `os/vokk/vokk.py` | 180 | 183 cols |
| `os/vokk/vokk.py` | 181 | 176 cols |
| `os/vokk/vokk.py` | 182 | 137 cols |
| `os/vokk/vokk.py` | 185 | 159 cols |
| `os/vokk/vokk.py` | 186 | 165 cols |
| `os/vokk/vokk.py` | 187 | 143 cols |
| `os/vokk/vokk.py` | 189 | 114 cols |
| `os/vokk/vokk.py` | 190 | 118 cols |
| `os/vokk/vokk.py` | 191 | 102 cols |
| `os/vokk/vokk.py` | 192 | 103 cols |
| `os/vokk/vokk.py` | 213 | 129 cols |
| `os/vokk/vokk.py` | 214 | 110 cols |
| `os/vokk/vokk.py` | 218 | 202 cols |
| `os/vokk/vokk.py` | 231 | 141 cols |
| `os/vokk/vokk.py` | 232 | 120 cols |
| `os/vokk/vokk.py` | 236 | 145 cols |
| `os/vokk/vokk.py` | 243 | 125 cols |
| `os/vokk/vokk.py` | 251 | 133 cols |
| `os/vokk/vokk.py` | 254 | 132 cols |
| `os/vokk/vokk.py` | 410 | 184 cols |
| `os/vokk/vokk.py` | 424 | 179 cols |
| `os/vokk/vokk.py` | 444 | 119 cols |
| `os/vokk/vokk.py` | 449 | 132 cols |
| `os/vokk/vokk.py` | 464 | 147 cols |
| `os/vokk/vokk.py` | 465 | 123 cols |
| `os/vokk/vokk.py` | 499 | 125 cols |
| `os/vokk/vokk.py` | 503 | 129 cols |
| `os/vokk/vokk.py` | 506 | 154 cols |
| `os/vokk/vokk.py` | 535 | 129 cols |
| `os/vokk/vokk.py` | 540 | 109 cols |
| `os/vokk/vokk_controller.py` | 36 | 187 cols |
| `os/vokk/vokk_controller.py` | 37 | 181 cols |
| `os/vokk/vokk_controller.py` | 38 | 186 cols |
| `os/vokk/vokk_controller.py` | 39 | 162 cols |
| `os/vokk/vokk_controller.py` | 40 | 154 cols |
| `os/vokk/vokk_controller.py` | 41 | 168 cols |
| `os/vokk/vokk_controller.py` | 42 | 168 cols |
| `os/vokk/vokk_controller.py` | 43 | 130 cols |
| `os/vokk/vokk_controller.py` | 44 | 136 cols |
| `os/vokk/vokk_controller.py` | 55 | 111 cols |
| `os/vokk/vokk_controller.py` | 58 | 151 cols |
| `os/vokk/vokk_controller.py` | 170 | 130 cols |
| `os/vokk/vokk_controller.py` | 172 | 148 cols |
| `os/vokk/vokk_controller.py` | 176 | 164 cols |
| `os/vokk/vokk_controller.py` | 186 | 103 cols |
| `os/vokk/vokk_controller.py` | 187 | 104 cols |
| `os/vokk/vokk_controller.py` | 188 | 159 cols |
| `os/vokk/vokk_controller.py` | 190 | 128 cols |
| `os/vokk/vokk_controller.py` | 191 | 175 cols |
| `os/vokk/vokk_controller.py` | 193 | 171 cols |
| `os/vokk/vokk_controller.py` | 198 | 116 cols |
| `os/vokk/vokk_controller.py` | 199 | 117 cols |
| `os/vokk/vokk_controller.py` | 200 | 214 cols |
| `os/vokk/vokk_controller.py` | 203 | 196 cols |
| `os/vokk/vokk_controller.py` | 210 | 201 cols |
| `os/vokk/vokk_controller.py` | 212 | 177 cols |
| `os/vokk/vokk_controller.py` | 214 | 119 cols |
| `os/vokk/vokk_controller.py` | 235 | 131 cols |
| `os/vokk/vokk_controller.py` | 247 | 102 cols |
| `os/vokk/vokk_controller.py` | 253 | 101 cols |
| `os/vokk/vokk_controller.py` | 255 | 105 cols |
| `os/vokk/vokk_controller.py` | 260 | 108 cols |
| `os/vokk/vokk_controller.py` | 262 | 103 cols |
| `os/vokk/vokk_controller.py` | 264 | 103 cols |
| `os/vokk/vokk_controller.py` | 269 | 101 cols |
| `os/vokk/vokk_controller.py` | 280 | 109 cols |
| `os/vokk/vokk_controller.py` | 281 | 115 cols |
| `os/vokk/vokk_controller.py` | 293 | 161 cols |
| `os/vokk/vokk_controller.py` | 297 | 153 cols |
| `os/vokk/vokk_controller.py` | 392 | 102 cols |
| `os/vokk/vokk_controller.py` | 414 | 103 cols |
| `os/vokk/vokk_controller.py` | 421 | 132 cols |
| `os/vokk/vokk_search.py` | 27 | 156 cols |
| `os/vokk/vokk_search.py` | 29 | 138 cols |
| `os/vokk/vokk_search.py` | 30 | 122 cols |
| `os/vokk/vokk_search.py` | 31 | 120 cols |
| `os/vokk/vokk_search.py` | 33 | 104 cols |
| `os/vokk/vokk_search.py` | 34 | 109 cols |
| `os/vokk/vokk_search.py` | 35 | 107 cols |
| `os/vokk/vokk_search.py` | 36 | 163 cols |
| `os/vokk/vokk_search.py` | 243 | 101 cols |
| `os/vokk/system-controller/controller.py` | 29 | 113 cols |
| `os/vokk/system-controller/controller.py` | 30 | 107 cols |
| `os/vokk/system-controller/controller.py` | 31 | 104 cols |
| `os/vokk/system-controller/controller.py` | 35 | 113 cols |
| `os/vokk/system-controller/controller.py` | 46 | 107 cols |
| `os/vokk/system-controller/controller.py` | 57 | 107 cols |
| `os/vokk/system-controller/controller.py` | 58 | 101 cols |
| `os/vokk/system-controller/controller.py` | 60 | 115 cols |
| `os/vokk/system-controller/controller.py` | 89 | 107 cols |
| `os/vokk/system-controller/controller.py` | 109 | 105 cols |
| `os/vokk/system-controller/controller.py` | 112 | 107 cols |
| `os/vokk/system-controller/controller.py` | 122 | 101 cols |
| `os/vokk/system-controller/controller.py` | 123 | 106 cols |
| `os/vokk/system-controller/controller.py` | 255 | 112 cols |
| `os/vokk/system-controller/controller.py` | 258 | 105 cols |
| `os/vokk/system-controller/controller.py` | 259 | 112 cols |
| `os/vokk/system-controller/controller.py` | 260 | 113 cols |
| `os/vokk/system-controller/controller.py` | 262 | 209 cols |
| `os/vokk/system-controller/controller.py` | 268 | 125 cols |
| `os/vokk/system-controller/controller.py` | 271 | 171 cols |
| `os/vokk/system-controller/controller.py` | 274 | 106 cols |
| `os/vokk/system-controller/controller.py` | 350 | 104 cols |
| `os/vokk/system-controller/controller.py` | 371 | 105 cols |
| `os/vokk/system-controller/controller.py` | 380 | 101 cols |
| `os/vokk/system-controller/controller.py` | 397 | 106 cols |
| `os/vokk/system-controller/controller.py` | 398 | 125 cols |
| `os/vokk/system-controller/controller.py` | 400 | 102 cols |
| `os/vokk/system-controller/controller.py` | 401 | 106 cols |
| `os/vokk/system-controller/controller.py` | 409 | 133 cols |
| `os/vokk/system-controller/controller.py` | 415 | 122 cols |
| `os/vokk/system-controller/controller.py` | 417 | 114 cols |
| `os/territories/vibe-address/searchie-keyd.py` | 31 | 107 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 96 | 104 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 122 | 108 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 141 | 105 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 149 | 106 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 158 | 120 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 163 | 122 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 168 | 121 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 176 | 112 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 177 | 115 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 184 | 103 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 191 | 120 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 194 | 109 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 198 | 139 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 204 | 101 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 217 | 128 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 255 | 137 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 256 | 110 cols |
| `os/hardware-cert/parc-hardware-cert.py` | 282 | 109 cols |
| `os/marketplace/parc-market.py` | 149 | 109 cols |
| `os/marketplace/parc-market.py` | 161 | 101 cols |
| `os/marketplace/parc-market.py` | 188 | 109 cols |
| `os/marketplace/parc-market.py` | 251 | 103 cols |
| `os/marketplace/parc-market.py` | 324 | 114 cols |
| `os/marketplace/parc-market.py` | 333 | 116 cols |
| `os/marketplace/parc-market.py` | 337 | 107 cols |
| `os/marketplace/seed-first-party.py` | 13 | 112 cols |
| `os/marketplace/seed-first-party.py` | 25 | 126 cols |
| `os/marketplace/seed-first-party.py` | 33 | 121 cols |
| `os/terminal-history/parc-term-history.py` | 72 | 102 cols |
| `os/terminal-history/parc-term-history.py` | 80 | 119 cols |
| `os/terminal-history/parc-term-history.py` | 113 | 125 cols |
| `os/terminal-history/parc-term-history.py` | 134 | 122 cols |
| `os/terminal-history/parc-term-history.py` | 137 | 105 cols |
| `os/terminal-history/parc-term-history.py` | 141 | 101 cols |
| `os/terminal-history/parc-term-history.py` | 145 | 103 cols |
| `os/terminal-history/parc-term-history.py` | 151 | 139 cols |
| `os/terminal-history/parc-term-history.py` | 154 | 129 cols |
| `os/terminal-history/parc-term-history.py` | 188 | 115 cols |
| `os/ai/generate-dataset.py` | 288 | 107 cols |
| `os/ai/generate-dataset.py` | 329 | 191 cols |
| `os/ai/generate-dataset.py` | 429 | 101 cols |
| `os/ai/generate-dataset.py` | 431 | 111 cols |
| `os/ai/generate-dataset.py` | 432 | 120 cols |
| `os/ai/generate-dataset.py` | 434 | 116 cols |
| `os/ai/generate-dataset.py` | 435 | 115 cols |
| `os/ai/inference.py` | 79 | 119 cols |
| `os/ai/inference.py` | 82 | 174 cols |
| `os/game-console/parc-game-console.py` | 255 | 104 cols |
| `os/game-console/parc-game-console.py` | 280 | 112 cols |
| `os/game-console/parc-game-console.py` | 293 | 104 cols |
| `os/game-console/parc-game-console.py` | 298 | 101 cols |
| `os/account/parc-account.py` | 121 | 117 cols |
| `os/account/parc-account.py` | 124 | 107 cols |
| `os/account/parc-account.py` | 148 | 105 cols |
| `os/account/parc-account.py` | 169 | 101 cols |
| `os/account/parc-account.py` | 229 | 128 cols |
| `os/account/parc-account.py` | 232 | 107 cols |
| `os/account/parc-account.py` | 237 | 116 cols |
| `os/mobile-companion/parc-mobile.py` | 153 | 105 cols |
| `os/mobile-companion/parc-mobile.py` | 156 | 124 cols |
| `os/mobile-companion/parc-mobile.py` | 198 | 101 cols |
| `os/mobile-companion/parc-mobile.py` | 249 | 101 cols |
| `os/mobile-companion/parc-mobile.py` | 253 | 138 cols |
| `os/brand/parc-brand.py` | 143 | 136 cols |
| `os/brand/parc-brand.py` | 144 | 146 cols |
| `os/brand/parc-brand.py` | 146 | 157 cols |
| `os/brand/parc-brand.py` | 148 | 105 cols |
| `os/brand/parc-brand.py` | 170 | 130 cols |
| `os/brand/parc-brand.py` | 171 | 138 cols |
| `os/brand/parc-brand.py` | 172 | 126 cols |
| `os/brand/parc-brand.py` | 177 | 101 cols |
| `os/brand/parc-brand.py` | 187 | 113 cols |
| `os/brand/parc-brand.py` | 188 | 196 cols |
| `os/brand/parc-brand.py` | 200 | 102 cols |
| `os/brand/parc-brand.py` | 202 | 105 cols |
| `os/enterprise/parc-enterprise.py` | 68 | 101 cols |
| `os/enterprise/parc-enterprise.py` | 87 | 103 cols |
| `os/enterprise/parc-enterprise.py` | 93 | 112 cols |
| `os/enterprise/parc-enterprise.py` | 99 | 106 cols |
| `os/enterprise/parc-enterprise.py` | 160 | 102 cols |
| `os/languages/korlang/korlang.py` | 391 | 139 cols |
| `os/languages/korlang/korlang.py` | 561 | 118 cols |
| `os/languages/korlang/korlang.py` | 627 | 118 cols |
| `os/languages/korlang/korlang.py` | 653 | 103 cols |
| `os/languages/korlang/korlang.py` | 654 | 106 cols |
| `os/languages/korlang/korlang.py` | 886 | 108 cols |
| `os/languages/korlang/korlang.py` | 976 | 108 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 230 | 114 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 261 | 120 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 344 | 108 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 405 | 104 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 568 | 103 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 652 | 121 cols |
| `os/languages/korrinuilang/korrinuilang.py` | 658 | 103 cols |
| `os/desktop/nibra-style/server.py` | 58 | 121 cols |
| `os/desktop/nibra-style/server.py` | 78 | 110 cols |
| `os/desktop/nibra-style/server.py` | 79 | 104 cols |

### P07 debug print in library — 290

| file | line | detail |
|------|------|--------|
| `os/docs/_gap_analysis.py` | 27 | print(f"corpus: {len(corpus)} files, {len(BLOB)} c |
| `os/docs/_gap_analysis.py` | 1190 | print(f"checked={len(rows)} present={present_n} AB |
| `os/docs/_gap_analysis.py` | 1191 | print("wrote docs/GAP-ANALYSIS.md + docs/_gap_anal |
| `os/docs/_professionalism_audit.py` | 56 | print(f"shell files: {len(sh_files)}", file=sys.st |
| `os/docs/_professionalism_audit.py` | 143 | print(f"python files: {len(py_files)}", file=sys.s |
| `os/docs/_professionalism_audit.py` | 192 | print(f"c files: {len(c_files)}", file=sys.stderr) |
| `os/docs/_professionalism_audit.py` | 224 | print(f"web files: {len(web_files)}", file=sys.std |
| `os/docs/_professionalism_audit.py` | 319 | print(f"TOTAL FINDINGS: {total}") |
| `os/docs/_professionalism_audit.py` | 321 | print(f"  {len(findings[cat]):5d}  {cat}") |
| `os/docs/_professionalism_audit.py` | 322 | print("wrote docs/PROFESSIONALISM.md + docs/_profe |
| `os/software-gpu/compiler/korlangc.py` | 76 | print("Usage: korlangc <input.kor> [output.c]"); s |
| `os/software-gpu/compiler/korlangc.py` | 83 | print(f"Written to {out}") |
| `os/software-gpu/compiler/korlangc.py` | 97 | print(ret.stderr.strip()) |
| `os/software-gpu/compiler/korlangc.py` | 98 | print(f"gcc exit: {ret.returncode}") |
| `os/software-gpu/compiler/korlangc.py` | 102 | print(run.stdout.strip()) |
| `os/software-gpu/compiler/korlangc.py` | 104 | print(run.stderr.strip()) |
| `os/software-gpu/compiler/korlangc.py` | 105 | print(f"run exit: {run.returncode}") |
| `os/parc-ai/parc-ai-gui.py` | 438 | print("  parc-ai-gui: syntax check pass") |
| `os/parc-ai/overlay/narrator.py` | 161 | print("Usage: narrator.py <message> [duration_ms]" |
| `os/parc-ai/model/parc_inference.py` | 38 | print(f"Neural model loaded: {self.model.count_par |
| `os/parc-ai/model/parc_inference.py` | 40 | print("No trained model found. Using Ollama fallba |
| `os/parc-ai/model/parc_inference.py` | 120 | print("No model loaded for benchmarking.") |
| `os/parc-ai/model/parc_inference.py` | 134 | print(f"\n{'='*60}") |
| `os/parc-ai/model/parc_inference.py` | 135 | print(f"  VOKK v4 Benchmark") |
| `os/parc-ai/model/parc_inference.py` | 136 | print(f"  Model: {self.model.count_parameters():,} |
| `os/parc-ai/model/parc_inference.py` | 137 | print(f"  Device: {self.device}") |
| `os/parc-ai/model/parc_inference.py` | 138 | print(f"{'='*60}\n") |
| `os/parc-ai/model/parc_inference.py` | 147 | print(f"Q: {q}") |
| `os/parc-ai/model/parc_inference.py` | 148 | print(f"A: {result['answer'][:100]}...") |
| `os/parc-ai/model/parc_inference.py` | 149 | print(f"Time: {elapsed:.2f}s") |
| `os/parc-ai/model/parc_inference.py` | 150 | print() |
| `os/parc-ai/model/parc_inference.py` | 152 | print(f"Average time: {total_time/len(test_questio |
| `os/parc-ai/model/parc_inference.py` | 161 | print("VOKK v4 Neural Chat (type 'quit' to exit)") |
| `os/parc-ai/model/parc_inference.py` | 167 | print(f"VOKK v4: {result['answer']}") |
| `os/parc-ai/model/parc_inference.py` | 172 | print(json.dumps(result, indent=2)) |
| `os/parc-ai/model/parc_model.py` | 232 | print(f"Model created: {model.count_parameters():, |
| `os/parc-ai/model/parc_model.py` | 237 | print(f"Forward pass: logits shape={logits.shape}, |
| `os/parc-ai/model/parc_model.py` | 242 | print(f"Generated: {generated.shape}") |
| `os/parc-ai/model/parc_tokenizer.py` | 45 | print(f"Tokenizer trained: {len(self.token_to_id)} |
| `os/parc-ai/model/parc_trainer.py` | 101 | print(f"Loaded {len(qa_pairs)} Q&A pairs and {len( |
| `os/parc-ai/model/parc_trainer.py` | 107 | print(f"\n{'='*60}") |
| `os/parc-ai/model/parc_trainer.py` | 108 | print(f"  VOKK v4 Training Pipeline") |
| `os/parc-ai/model/parc_trainer.py` | 109 | print(f"  Device: {self.device}") |
| `os/parc-ai/model/parc_trainer.py` | 110 | print(f"  Q&A pairs: {len(qa_pairs)}") |
| `os/parc-ai/model/parc_trainer.py` | 111 | print(f"  Epochs: {epochs}") |
| `os/parc-ai/model/parc_trainer.py` | 112 | print(f"{'='*60}\n") |
| `os/parc-ai/model/parc_trainer.py` | 115 | print("[1/4] Training tokenizer...") |
| `os/parc-ai/model/parc_trainer.py` | 122 | print("[2/4] Creating model...") |
| `os/parc-ai/model/parc_trainer.py` | 134 | print(f"  Model: {param_count:,} parameters") |
| `os/parc-ai/model/parc_trainer.py` | 137 | print("[3/4] Preparing data...") |
| `os/parc-ai/model/parc_trainer.py` | 142 | print("[4/4] Training...") |
| `os/parc-ai/model/parc_trainer.py` | 187 | print(f"  Epoch {epoch+1}/{epochs} \| Loss: {avg_lo |
| `os/parc-ai/model/parc_trainer.py` | 193 | print(f"\nTraining complete! Best loss: {best_loss |
| `os/parc-ai/model/parc_trainer.py` | 194 | print(f"Model saved to: {self.model_dir}") |
| `os/parc-ai/model/parc_trainer.py` | 210 | print(f"Model loaded from {path}") |
| `os/parc-ai/model/parc_trainer.py` | 211 | print(f"Parameters: {self.model.count_parameters() |
| `os/parc-ai/model/parc_trainer.py` | 240 | print("\nVOKK v4 Chat (type 'quit' to exit)") |
| `os/parc-ai/model/parc_trainer.py` | 241 | print("=" * 40) |
| `os/parc-ai/model/parc_trainer.py` | 246 | print("Goodbye!") |
| `os/parc-ai/model/parc_trainer.py` | 253 | print(f"\nVOKK v4: {answer}") |
| `os/parc-ai/model/parc_trainer.py` | 277 | print("No training data found. Add Q&A pairs to th |
| `os/tinker-cowork/tinker-cowork.py` | 101 | print(f"Downloading {model_id}...") |
| `os/tinker-cowork/tinker-cowork.py` | 105 | print(f"Download failed: {e}") |
| `os/tinker-cowork/tinker-cowork.py` | 338 | print(""" |
| `os/tinker-cowork/tinker-cowork.py` | 352 | print("Goodbye!") |
| `os/tinker-cowork/tinker-cowork.py` | 359 | print(response) |
| `os/tinker-cowork/tinker-cowork.py` | 362 | print("\nInterrupted") |
| `os/tinker-cowork/tinker-cowork.py` | 368 | print(""" |
| `os/iso-builder-pro/parc-iso-pro.py` | 166 | print(f"Building KorrinOS {profile.name} ISO...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 167 | print(f"  Base: {profile.base} {profile.base_versi |
| `os/iso-builder-pro/parc-iso-pro.py` | 168 | print(f"  Desktop: {profile.desktop}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 169 | print(f"  Kernel: {profile.kernel}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 170 | print(f"  Packages: {len(profile.packages)}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 171 | print(f"  Flatpaks: {len(profile.flatpaks)}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 172 | print(f"  Output: {output_file}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 200 | print(f"\n ISO built successfully!") |
| `os/iso-builder-pro/parc-iso-pro.py` | 201 | print(f"  File: {output_file}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 202 | print(f"  Size: {size:.2f} GB") |
| `os/iso-builder-pro/parc-iso-pro.py` | 203 | print(f"  SHA256: {sha256}") |
| `os/iso-builder-pro/parc-iso-pro.py` | 213 | print("  Preparing rootfs...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 226 | print("  Installing packages...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 252 | print("  Configuring system...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 301 | print("  Installing Flatpaks...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 313 | print("  Setting up bootloader...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 352 | print("  Creating squashfs...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 368 | print("  Creating ISO...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 393 | print("  Signing ISO...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 398 | print("  Testing ISO in QEMU...") |
| `os/iso-builder-pro/parc-iso-pro.py` | 427 | print("Available profiles:") |
| `os/iso-builder-pro/parc-iso-pro.py` | 430 | print(f"  {name}: {profile.description}") |
| `os/onboarding/parc-onboard.py` | 419 | print(""" |
| `os/onboarding/parc-onboard.py` | 428 | print("Step 1: Your Account") |
| `os/onboarding/parc-onboard.py` | 433 | print("\nStep 2: Appearance") |
| `os/onboarding/parc-onboard.py` | 437 | print("\nStep 3: Keyboard Layout") |
| `os/onboarding/parc-onboard.py` | 441 | print("\nStep 4: Timezone") |
| `os/onboarding/parc-onboard.py` | 445 | print("\nStep 5: Privacy Level") |
| `os/onboarding/parc-onboard.py` | 446 | print("  1) Minimal - No telemetry") |
| `os/onboarding/parc-onboard.py` | 447 | print("  2) Balanced - Anonymous stats (recommende |
| `os/onboarding/parc-onboard.py` | 448 | print("  3) Full - Help improve KorrinOS") |
| `os/onboarding/parc-onboard.py` | 452 | print("\nStep 6: Essential Apps (y/n)") |
| `os/onboarding/parc-onboard.py` | 458 | print("\nStep 7: Advanced Features (y/n)") |
| `os/onboarding/parc-onboard.py` | 465 | print("\n Setup complete! Applying settings...") |
| `os/onboarding/parc-onboard.py` | 468 | print(f""" |
| `os/vokk/computer_use.py` | 283 | print(json.dumps(cu.describe(), indent=2)) |
| `os/vokk/computer_use.py` | 286 | print("\nTesting screen capture...") |
| `os/vokk/computer_use.py` | 288 | print(f"  Saved to {shot}") |
| `os/vokk/computer_use.py` | 291 | print(f"  OCR text ({len(text)} chars): {text[:200 |
| `os/vokk/data_generator.py` | 386 | print(f"Total QA pairs: {len(data)}") |
| `os/vokk/data_generator.py` | 387 | print(f"Total chars: {total_chars}") |
| `os/vokk/data_generator.py` | 388 | print(f"Avg chars per pair: {total_chars // max(le |
| `os/vokk/data_generator.py` | 389 | print("\nSample:") |
| `os/vokk/data_generator.py` | 391 | print(f"  Q: {q}") |
| `os/vokk/data_generator.py` | 392 | print(f"  A: {a}") |
| `os/vokk/image_module.py` | 266 | print("VOKK v4 Image Module v1.0") |
| `os/vokk/image_module.py` | 267 | print("Testing link extraction...") |
| `os/vokk/image_module.py` | 270 | print(f"Found {len(urls)} URLs: {urls}") |
| `os/vokk/image_module.py` | 272 | print("\nTesting graph generation...") |
| `os/vokk/image_module.py` | 274 | print(result) |
| `os/vokk/small_rnn_model.py` | 227 | print(f"Model saved to {path} ({os.path.getsize(pa |
| `os/vokk/small_rnn_model.py` | 240 | print(f"Model loaded from {path}") |
| `os/vokk/small_rnn_model.py` | 403 | print(f"[{tag or optimizer}] training examples: {n |
| `os/vokk/small_rnn_model.py` | 456 | print(f"  epoch {epoch}: loss = {avg_loss:.4f}", f |
| `os/vokk/small_rnn_model.py` | 461 | print(f"  [checkpoint] saved at epoch {epoch + 1}  |
| `os/vokk/small_rnn_model.py` | 475 | print(f"[merge] no previous model at {prev_path} - |
| `os/vokk/small_rnn_model.py` | 480 | print(f"[merge] could not read previous model: {e} |
| `os/vokk/small_rnn_model.py` | 497 | print(f"[merge] architecture mismatch ({key}): pre |
| `os/vokk/small_rnn_model.py` | 501 | print(f"[merge] embed shape mismatch - skipping") |
| `os/vokk/small_rnn_model.py` | 520 | print(f"[merge] merged with previous model (alpha= |
| `os/vokk/small_rnn_model.py` | 552 | print(f"Backed up previous model to {prev_path}") |
| `os/vokk/small_rnn_model.py` | 554 | print(f"Previous backup already exists: {prev_path |
| `os/vokk/small_rnn_model.py` | 556 | print("=" * 56) |
| `os/vokk/small_rnn_model.py` | 557 | print("VOKK v4 RNN Trainer (Adam base + GaLore fin |
| `os/vokk/small_rnn_model.py` | 558 | print("=" * 56) |
| `os/vokk/small_rnn_model.py` | 561 | print("FATAL: data_generator produced no data.") |
| `os/vokk/small_rnn_model.py` | 572 | print(f"Tokenizer: vocab={tokenizer.vocab_size}") |
| `os/vokk/small_rnn_model.py` | 579 | print(f"Resumed weights from {model_path}") |
| `os/vokk/small_rnn_model.py` | 583 | print(f"\n--- Stage 1: base training (Adam, {args. |
| `os/vokk/small_rnn_model.py` | 592 | print(f"Base model saved: {model_path}") |
| `os/vokk/small_rnn_model.py` | 596 | print(f"\n--- Stage 2: GaLore fine-tune ({args.fin |
| `os/vokk/small_rnn_model.py` | 605 | print(f"Final model saved: {model_path}") |
| `os/vokk/small_rnn_model.py` | 609 | print("\n--- Stage 3: merge with previous model -- |
| `os/vokk/small_rnn_model.py` | 613 | print("\n=== Inference samples ===") |
| `os/vokk/small_rnn_model.py` | 617 | print(f"  Q: {q}") |
| `os/vokk/small_rnn_model.py` | 618 | print(f"  A(train): {a}") |
| `os/vokk/small_rnn_model.py` | 619 | print(f"  A(gen):   {tokenizer.decode(gen)}") |
| `os/vokk/small_rnn_model.py` | 621 | print("\n" + "=" * 56) |
| `os/vokk/small_rnn_model.py` | 622 | print("Done. VOKK v4 model ready.") |
| `os/vokk/small_rnn_model.py` | 623 | print("=" * 56) |
| `os/vokk/taskbar_ai.py` | 128 | print(result) |
| `os/vokk/vokk-ui.py` | 221 | print("vokk CLI") |
| `os/vokk/vokk-ui.py` | 231 | print(f"GUI unavailable ({e}); falling back to CLI |
| `os/vokk/vokk-v4-engine.py` | 119 | print(f"Download failed: {e}") |
| `os/vokk/vokk-v4-engine.py` | 485 | print(f"VOKK v4 API server running on http://{self |
| `os/vokk/vokk-v4-engine.py` | 488 | print(f"Server error: {e}") |
| `os/vokk/vokk-v4-engine.py` | 495 | print("llama.cpp found, attempting to load TinyLla |
| `os/vokk/vokk-v4-engine.py` | 498 | print("Model loaded!") |
| `os/vokk/vokk-v4-engine.py` | 500 | print("Could not download model, using rule-based  |
| `os/vokk/vokk-v4-engine.py` | 502 | print("llama.cpp not found. Install with: pip inst |
| `os/vokk/vokk-v4-engine.py` | 503 | print("Running in rule-based mode") |
| `os/vokk/vokk-v4-engine.py` | 505 | print("\n VOKK v4 Ready! Type 'help' for commands, |
| `os/vokk/vokk-v4-engine.py` | 513 | print("Goodbye!") |
| `os/vokk/vokk-v4-engine.py` | 517 | print(f"\n{response.text}\n") |
| `os/vokk/vokk-v4-engine.py` | 518 | print(f"[Intent: {response.intent} \| Confidence: { |
| `os/vokk/vokk-v4-engine.py` | 520 | print("\nGoodbye!") |
| `os/vokk/vokk_controller.py` | 477 | print(f"VOKK v4 v2.0 — System Controller + Image G |
| `os/vokk/vokk_controller.py` | 478 | print(f"http://localhost:{PORT}") |
| `os/vokk/vokk_controller.py` | 483 | print("\nStopped.") |
| `os/vokk/vokk_search.py` | 186 | print(f"[search] trained on {len(self.docs)} docs  |
| `os/vokk/vokk_search.py` | 246 | print(f"[search] index saved: {INDEX_PATH}", flush |
| `os/vokk/vokk_search.py` | 265 | print(f"[search] index loaded: {len(self.docs)} do |
| `os/vokk/vokk_search.py` | 275 | print("[search] need trained rnn model+tokenizer f |
| `os/vokk/vokk_search.py` | 286 | print(f"  Q: {q}\n  A: {ans}  (score {sc:.3f})", f |
| `os/vokk/system-controller/controller.py` | 436 | print(f"VOKK v4 System Controller v1.0") |
| `os/vokk/system-controller/controller.py` | 437 | print(f"http://localhost:{PORT}") |
| `os/vokk/system-controller/controller.py` | 442 | print("\nStopped.") |
| `os/territories/vibe-address/searchie-gui.py` | 527 | print("  ASK     ", hits[0][:80]) |
| `os/territories/vibe-address/searchie-gui.py` | 537 | print("  DELETE   staged", len(items), "file(s), p |
| `os/territories/vibe-address/searchie-gui.py` | 542 | print("  CONFIRM  file removed, memory traces purg |
| `os/territories/vibe-address/searchie-gui.py` | 543 | print("  SELFTEST PASS") |
| `os/territories/vibe-address/searchie-keyd.py` | 76 | print("searchie-keyd: Wayland session — no Tab+F7  |
| `os/territories/vibe-address/searchie-keyd.py` | 80 | print("searchie-keyd: no display — nothing to do." |
| `os/territories/vibe-address/searchie-keyd.py` | 85 | print("searchie-keyd: cannot resolve Tab/F7 keycod |
| `os/territories/vibe-address/searchie-keyd.py` | 91 | print("searchie-keyd: no virtual keyboard (xinput? |
| `os/territories/vibe-address/searchie-keyd.py` | 109 | print(f"searchie-keyd: cannot start xinput: {e}",  |
| `os/territories/vibe-address/searchie-keyd.py` | 116 | print(f"searchie-keyd: Tab+F7 chord live (tab={tab |
| `os/territories/vibe-address/searchie-keyd.py` | 127 | print(f"[dbg] event={evtype}", file=sys.stderr, fl |
| `os/territories/vibe-address/searchie-keyd.py` | 134 | print(f"[dbg] detail={kc} evtype={evtype}", file=s |
| `os/hardware-cert/parc-hardware-cert.py` | 256 | print(f"  {status} {hw.vendor} {hw.model} ({hw.com |
| `os/hardware-cert/parc-hardware-cert.py` | 267 | print(f"Submitted: {hw.id}") |
| `os/hardware-cert/parc-hardware-cert.py` | 272 | print(f"  {hw['type'].upper()}: {hw.get('model', h |
| `os/hardware-cert/parc-hardware-cert.py` | 276 | print(json.dumps(results, indent=2)) |
| `os/hardware-cert/parc-hardware-cert.py` | 280 | if len(sys.argv) < 2: print("Usage: tinker-hw-cert |
| `os/hardware-cert/parc-hardware-cert.py` | 285 | else: print("Unknown command") |
| `os/marketplace/parc-market-gui.py` | 247 | print("PyQt6 required for GUI. Run: pip install Py |
| `os/marketplace/parc-market.py` | 353 | print(f"  {app.name} v{app.version} - {app.descrip |
| `os/marketplace/parc-market.py` | 354 | print(f"    {app.category} \| ${app.price} \| {app.r |
| `os/marketplace/parc-market.py` | 358 | print(f"Installed {app_id}") |
| `os/marketplace/parc-market.py` | 360 | print("Install failed") |
| `os/marketplace/parc-market.py` | 370 | print(f"Submitted {name} for review") |
| `os/marketplace/parc-market.py` | 377 | print("Usage: tinker-market [search\|install\|submit |
| `os/marketplace/parc-market.py` | 387 | print("Unknown command") |
| `os/marketplace/seed-first-party.py` | 46 | print(f"  vet sandbox: {src.name}") |
| `os/marketplace/seed-first-party.py` | 48 | print("  firejail not present — vet skipped (build |
| `os/marketplace/seed-first-party.py` | 56 | print(f"  vet error: {e}") |
| `os/marketplace/seed-first-party.py` | 81 | print(f"  SKIP {spec['name']} — vet failed") |
| `os/marketplace/seed-first-party.py` | 84 | print(f"  preinstalled: {spec['name']} ({app.id})" |
| `os/marketplace/seed-first-party.py` | 85 | print("done") |
| `os/terminal-history/parc-term-history.py` | 115 | print(f"Restoring session: {session['name']}") |
| `os/terminal-history/parc-term-history.py` | 116 | print(f"  Started: {session['started_at']}") |
| `os/terminal-history/parc-term-history.py` | 117 | print(f"  Commands: {len(commands)}") |
| `os/terminal-history/parc-term-history.py` | 118 | print() |
| `os/terminal-history/parc-term-history.py` | 121 | print(f"$ {cmd['command']}") |
| `os/terminal-history/parc-term-history.py` | 123 | print(cmd['output'][:200]) |
| `os/terminal-history/parc-term-history.py` | 124 | print() |
| `os/terminal-history/parc-term-history.py` | 163 | print(f"Started session: {session.id}") |
| `os/terminal-history/parc-term-history.py` | 167 | print("Session ended") |
| `os/terminal-history/parc-term-history.py` | 172 | print(f"  {status} {s['id']} - {s['name']} ({s['st |
| `os/terminal-history/parc-term-history.py` | 180 | print(f"  [{cmd['timestamp'][:19]}] {cmd['command' |
| `os/terminal-history/parc-term-history.py` | 184 | print("Bookmarked") |
| `os/terminal-history/parc-term-history.py` | 188 | if len(sys.argv) < 2: print("Usage: tinker-term-hi |
| `os/terminal-history/parc-term-history.py` | 195 | else: print("Unknown command") |
| `os/ai/generate-dataset.py` | 485 | print(f"Dataset generated:") |
| `os/ai/generate-dataset.py` | 486 | print(f"  Total samples: {len(dataset)}") |
| `os/ai/generate-dataset.py` | 487 | print(f"  Unique actions: {len(actions)}") |
| `os/ai/generate-dataset.py` | 488 | print(f"  Output: {output_dir}") |
| `os/ai/inference.py` | 42 | print(f"Error loading model: {e}") |
| `os/ai/inference.py` | 98 | print(f"Error executing command: {e}") |
| `os/ai/inference.py` | 158 | print("KorrinOS Voice Assistant (type 'quit' to ex |
| `os/ai/inference.py` | 159 | print("Type commands or speak naturally...") |
| `os/ai/inference.py` | 160 | print() |
| `os/ai/inference.py` | 167 | print("Goodbye!") |
| `os/ai/inference.py` | 175 | print(f"Assistant: {response}") |
| `os/ai/inference.py` | 178 | print("\nGoodbye!") |
| `os/ai/inference.py` | 189 | print("Model not found. Please train first:") |
| `os/ai/inference.py` | 190 | print("  python3 train.py") |
| `os/ai/inference.py` | 199 | print(response) |
| `os/ai/train.py` | 231 | print("Loading dataset...") |
| `os/ai/train.py` | 233 | print(f"Loaded {len(texts)} samples, {len(action_m |
| `os/ai/train.py` | 236 | print("Creating tokenizer...") |
| `os/ai/train.py` | 241 | print("Encoding texts...") |
| `os/ai/train.py` | 245 | print("Creating model...") |
| `os/ai/train.py` | 253 | print(f"Training for {epochs} epochs...") |
| `os/ai/train.py` | 280 | print(f"Epoch {epoch+1}/{epochs} - Accuracy: {accu |
| `os/ai/train.py` | 283 | print("Saving model...") |
| `os/ai/train.py` | 292 | print(f"Model saved to {output_dir}") |
| `os/ai/train.py` | 293 | print("Training complete!") |
| `os/game-console/parc-game-console.py` | 309 | print("Scanning for games...") |
| `os/game-console/parc-game-console.py` | 313 | print(f"  Found: {game.name} ({game.platform})") |
| `os/game-console/parc-game-console.py` | 314 | print(f"\nTotal: {len(games)} games found") |
| `os/game-console/parc-game-console.py` | 321 | print(f"  {row['name']} ({row['platform']}) - Play |
| `os/game-console/parc-game-console.py` | 325 | print(f"Launched: {game_id}") |
| `os/game-console/parc-game-console.py` | 327 | print("Launch failed") |
| `os/game-console/parc-game-console.py` | 331 | print(f"Screenshot saved: {path}") |
| `os/game-console/parc-game-console.py` | 335 | print(f"Recording started: {path}") |
| `os/game-console/parc-game-console.py` | 343 | print("Usage: tinker-game-console [scan\|list\|launc |
| `os/game-console/parc-game-console.py` | 357 | print("Unknown command") |
| `os/account/parc-account.py` | 254 | print(f"Syncing to {target_device_ip}...") |
| `os/account/parc-account.py` | 275 | print(f"Created account: {me.id} ({me.username})") |
| `os/account/parc-account.py` | 279 | print(f"Registered device: {device.id} ({device.na |
| `os/account/parc-account.py` | 283 | print("Synced theme preference") |
| `os/account/parc-account.py` | 287 | print("Exported to /tmp/my-account.tinker") |
| `os/account/parc-account.py` | 292 | print(f"  Device: {d.name} ({d.id})") |
| `os/mobile-companion/parc-mobile.py` | 270 | print(f"Pairing code: {code}") |
| `os/mobile-companion/parc-mobile.py` | 271 | print("Enter this code in the KorrinOS mobile app" |
| `os/mobile-companion/parc-mobile.py` | 272 | print("Or scan the QR code:") |
| `os/mobile-companion/parc-mobile.py` | 274 | print(f"QR Code (base64): {qr_b64[:50]}...") |
| `os/mobile-companion/parc-mobile.py` | 279 | print(f"  {d.name} ({d.type.value}) - {d.os} - Las |
| `os/mobile-companion/parc-mobile.py` | 290 | print("Notification sent") |
| `os/mobile-companion/parc-mobile.py` | 299 | print("Usage: tinker-mobile [pair\|list\|notify\|stat |
| `os/mobile-companion/parc-mobile.py` | 310 | print(json.dumps(status, indent=2)) |
| `os/mobile-companion/parc-mobile.py` | 312 | print("Unknown command") |
| `os/mobile-companion/scripts/mobile-companion-server.py` | 207 | print(f"KorrinOS Mobile Companion WS listening on  |
| `os/brand/parc-brand.py` | 203 | print(f"Brand assets saved to {output_dir}") |
| `os/brand/parc-brand.py` | 208 | print("KorrinOS Brand Identity System generated!") |
| `os/brand/parc-brand.py` | 209 | print(f"Name: {brand.name}, Tagline: {brand.taglin |
| `os/brand/parc-brand.py` | 210 | print(f"Primary: {brand.colors.primary}, Mascot: { |
| `os/enterprise/parc-enterprise.py` | 162 | print(f"Enrolled: {device.id}") |
| `os/enterprise/parc-enterprise.py` | 166 | print(f"  {d.hostname} ({d.ip}) - {d.status.value} |
| `os/enterprise/parc-enterprise.py` | 170 | print(f"Compliance: {'PASS' if report.passed else  |
| `os/enterprise/parc-enterprise.py` | 172 | print(f"  {check}: {'OK' if result else 'FAIL'}") |
| `os/enterprise/parc-enterprise.py` | 176 | if len(sys.argv) < 2: print("Usage: tinker-enterpr |
| `os/enterprise/parc-enterprise.py` | 180 | else: print("Unknown command") |
| `os/languages/korlang/korlang.py` | 972 | print(f"Korlang: {self.fn}  {out}") |
| `os/languages/korlang/korlang.py` | 978 | print(f"GCC error:\n{r.stderr}"); return False |
| `os/languages/korlang/korlang.py` | 979 | print(f"Korlang: {c_file}  {binary}"); return True |
| `os/languages/korlang/korlang.py` | 983 | print("Usage: korlang <file.kor> [-o output] [--ru |
| `os/languages/korlang/korlang.py` | 998 | print(f"Korlang: {fn}  {out}") |
| `os/languages/korlang/korlang.py` | 1002 | print(f"\n--- Running {out} ---\n") |
| `os/languages/korrinuilang/korrinuilang.py` | 779 | print(f"KorrinUILang: {self.filename} -> {output}" |
| `os/languages/korrinuilang/korrinuilang.py` | 784 | print("Usage: korrinuilang <file.kui> [-o output.k |

### C109 mixed indentation — 261

| file | line | detail |
|------|------|--------|
| `kernel/tinker/adaptive_display.c` | 37 | seq_printf(m, "mode:          %s\n", modes[ad_mode |
| `kernel/tinker/adaptive_display.c` | 38 | seq_printf(m, "refresh_hz:    %u\n", ad_refresh_hz |
| `kernel/tinker/adaptive_display.c` | 39 | seq_printf(m, "frames:        %llu\n", ad_frames); |
| `kernel/tinker/adaptive_display.c` | 40 | seq_puts(m, "policy:        adaptive refresh/brigh |
| `kernel/tinker/app_store.c` | 51 | seq_printf(m, "Total removed:    %d\n", apps_st->t |
| `kernel/tinker/app_store.c` | 52 | seq_printf(m, "Total updates:    %d\n", apps_st->t |
| `kernel/tinker/app_store.c` | 53 | seq_printf(m, "Total reviews:    %d\n", apps_st->t |
| `kernel/tinker/app_store.c` | 54 | seq_printf(m, "Repos synced:     %d\n", apps_st->r |
| `kernel/tinker/app_store.c` | 140 | .proc_open    = apps_open, |
| `kernel/tinker/app_store.c` | 142 | .proc_read    = seq_read, |
| `kernel/tinker/app_store.c` | 156 | &apps_proc_ops); |
| `kernel/tinker/battery_life.c` | 58 | seq_printf(m, "mode:       %s\n", lifespan_mode ?  |
| `kernel/tinker/battery_life.c` | 61 | seq_printf(m, "policy:     trickle below min, top- |
| `kernel/tinker/battery_life.c` | 67 | size_t len, loff_t *ppos) |
| `kernel/tinker/battery_life.c` | 132 | &battery_fops); |
| `kernel/tinker/cache_tiering.c` | 51 | : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx) |
| `kernel/tinker/cache_tiering.c` | 52 | : "a"(4), "c"(subleaf)); |
| `kernel/tinker/cache_tiering.c` | 59 | : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx) |
| `kernel/tinker/cache_tiering.c` | 60 | : "a"(4), "c"(subleaf)); |
| `kernel/tinker/cache_tiering.c` | 79 | seq_printf(m, "enabled:            %u\n", cache_en |
| `kernel/tinker/cache_tiering.c` | 80 | seq_printf(m, "l3_total_ways:      %u\n", cache_ma |
| `kernel/tinker/cache_tiering.c` | 83 | seq_printf(m, "policy:             priority-weight |
| `kernel/tinker/cache_tiering.c` | 84 | seq_puts(m, "backend:            CAT/MPAM (way-par |
| `kernel/tinker/cloud_sync.c` | 77 | case CLOUD_DISABLED:     return "disabled"; |
| `kernel/tinker/cloud_sync.c` | 78 | case CLOUD_IDLE:         return "idle"; |
| `kernel/tinker/cloud_sync.c` | 79 | case CLOUD_SYNCING:      return "syncing"; |
| `kernel/tinker/cloud_sync.c` | 80 | case CLOUD_ERROR:        return "error"; |
| `kernel/tinker/cloud_sync.c` | 82 | default:                 return "unknown"; |
| `kernel/tinker/cloud_sync.c` | 94 | seq_printf(m, "Auto-sync:       %s\n", cloud_st->a |
| `kernel/tinker/cloud_sync.c` | 95 | seq_printf(m, "Total syncs:     %d\n", cloud_st->t |
| `kernel/tinker/cloud_sync.c` | 97 | seq_printf(m, "Total errors:    %d\n", cloud_st->t |
| `kernel/tinker/cloud_sync.c` | 98 | seq_printf(m, "Providers:       %d\n\n", cloud_st- |
| `kernel/tinker/cloud_sync.c` | 109 | seq_printf(m, "    last error: %s\n", p->last_erro |
| `kernel/tinker/cloud_sync.c` | 195 | .proc_open    = cloud_open, |
| `kernel/tinker/cloud_sync.c` | 197 | .proc_read    = seq_read, |
| `kernel/tinker/cloud_sync.c` | 212 | &cloud_proc_ops); |
| `kernel/tinker/coil_whine.c` | 39 | seq_printf(m, "enabled:          %u\n", cw_enabled |
| `kernel/tinker/coil_whine.c` | 40 | seq_printf(m, "freq_khz:         %u\n", cw_freq_kh |
| `kernel/tinker/coil_whine.c` | 41 | seq_printf(m, "base_freq_khz:    %u\n", cw_base_fr |
| `kernel/tinker/coil_whine.c` | 43 | seq_printf(m, "spread_khz:       %u\n", cw_spread_ |
| `kernel/tinker/coil_whine.c` | 44 | seq_printf(m, "popup_shown:      %u\n", cw_popup_s |
| `kernel/tinker/coil_whine.c` | 45 | seq_puts(m, "policy:           shift VRM PWM out o |
| `kernel/tinker/coil_whine.c` | 46 | seq_puts(m, "consent:          explicit toggle req |
| `kernel/tinker/cxl_memory.c` | 39 | seq_printf(m, "enabled:     %u\n", cxl_enabled); |
| `kernel/tinker/cxl_memory.c` | 40 | seq_printf(m, "dram_mb:     %llu\n", cxl_pool_dram |
| `kernel/tinker/cxl_memory.c` | 41 | seq_printf(m, "vram_mb:     %llu\n", cxl_pool_vram |
| `kernel/tinker/cxl_memory.c` | 42 | seq_printf(m, "cxl_mb:      %llu\n", cxl_pool_cxl_ |
| `kernel/tinker/cxl_memory.c` | 43 | seq_printf(m, "net_mb:      %llu\n", cxl_pool_net_ |
| `kernel/tinker/cxl_memory.c` | 44 | seq_puts(m, "policy:      opt-in only; nothing ena |
| `kernel/tinker/data_shredder.c` | 36 | seq_printf(m, "swap_on_shutdown:    %u\n", shred_s |
| `kernel/tinker/data_shredder.c` | 37 | seq_printf(m, "scramble_hwid:       %u\n", shred_s |
| `kernel/tinker/data_shredder.c` | 38 | seq_printf(m, "scrub_pages:         %u\n", shred_s |
| `kernel/tinker/data_shredder.c` | 39 | seq_printf(m, "shred_ops:           %llu\n", |
| `kernel/tinker/data_shredder.c` | 41 | seq_puts(m, "policy:              never touches ac |
| `kernel/tinker/desktop_state.c` | 104 | case COMP_NONE:    return "none"; |
| `kernel/tinker/desktop_state.c` | 108 | case COMP_KWIN:    return "kwin"; |
| `kernel/tinker/desktop_state.c` | 109 | default:           return "unknown"; |
| `kernel/tinker/desktop_state.c` | 126 | seq_printf(m, "Panel:     %s height=%d autohide=%s |
| `kernel/tinker/desktop_state.c` | 131 | seq_printf(m, "Dock:      %s items=%d size=%d auto |
| `kernel/tinker/desktop_state.c` | 137 | seq_printf(m, "  Name:     %s\n", desk_st->theme_n |
| `kernel/tinker/desktop_state.c` | 138 | seq_printf(m, "  Icons:    %s\n", desk_st->icon_th |
| `kernel/tinker/desktop_state.c` | 140 | seq_printf(m, "  Dark:     %s\n", desk_st->dark_mo |
| `kernel/tinker/desktop_state.c` | 147 | seq_printf(m, "  Blur:         %s\n", desk_st->blu |
| `kernel/tinker/desktop_state.c` | 149 | seq_printf(m, "  Shadows:      %s\n", desk_st->sha |
| `kernel/tinker/desktop_state.c` | 150 | seq_printf(m, "  VSync:        %s\n", desk_st->vsy |
| `kernel/tinker/desktop_state.c` | 174 | seq_printf(m, "  Large text:     %s\n", desk_st->l |
| `kernel/tinker/desktop_state.c` | 176 | seq_printf(m, "  Sticky keys:    %s\n", desk_st->s |
| `kernel/tinker/desktop_state.c` | 192 | size_t count, loff_t *ppos) |
| `kernel/tinker/desktop_state.c` | 291 | .proc_open    = desktop_open, |
| `kernel/tinker/desktop_state.c` | 293 | .proc_read    = seq_read, |
| `kernel/tinker/driver_monitor.c` | 57 | case DRIVER_GPU:         return "gpu"; |
| `kernel/tinker/driver_monitor.c` | 58 | case DRIVER_WIFI:        return "wifi"; |
| `kernel/tinker/driver_monitor.c` | 61 | case DRIVER_AUDIO:       return "audio"; |
| `kernel/tinker/driver_monitor.c` | 62 | case DRIVER_PRINTER:     return "printer"; |
| `kernel/tinker/driver_monitor.c` | 63 | default:                 return "other"; |
| `kernel/tinker/driver_monitor.c` | 86 | seq_printf(m, "GPU mode:         %s\n", gpu_mode_s |
| `kernel/tinker/driver_monitor.c` | 87 | seq_printf(m, "DKMS modules:     %d\n", drv_st->dk |
| `kernel/tinker/driver_monitor.c` | 102 | seq_printf(m, "    UPDATE: %s\n", d->update_versio |
| `kernel/tinker/driver_monitor.c` | 114 | size_t count, loff_t *ppos) |
| `kernel/tinker/driver_monitor.c` | 169 | drv_st->drivers[i].update_version); |
| `kernel/tinker/driver_monitor.c` | 184 | .proc_open    = drivers_open, |
| `kernel/tinker/driver_monitor.c` | 186 | .proc_read    = seq_read, |
| `kernel/tinker/driver_monitor.c` | 201 | &drv_proc_ops); |
| `kernel/tinker/dust_dislodger.c` | 120 | seq_printf(m, "enabled:          %u\n", dust_enabl |
| `kernel/tinker/dust_dislodger.c` | 121 | seq_printf(m, "pulse_hz:         %u\n", dust_pulse |
| `kernel/tinker/dust_dislodger.c` | 122 | seq_printf(m, "amplitude_pct:    %u\n", dust_ampli |
| `kernel/tinker/dust_dislodger.c` | 123 | seq_printf(m, "duration_s:       %u\n", dust_durat |
| `kernel/tinker/dust_dislodger.c` | 125 | seq_printf(m, "runs:             %llu\n", dust_run |
| `kernel/tinker/dust_dislodger.c` | 126 | seq_puts(m, "spec_state:       "); |
| `kernel/tinker/dust_dislodger.c` | 131 | seq_printf(m, "spec_max_rpm:     %u\n", dust_decl_ |
| `kernel/tinker/dust_dislodger.c` | 132 | seq_printf(m, "spec_measured:    %u rpm @ %u%% dut |
| `kernel/tinker/dust_dislodger.c` | 134 | seq_printf(m, "spec_slope:       %u rpm/%% pwm\n", |
| `kernel/tinker/dust_dislodger.c` | 135 | seq_printf(m, "spec_safe_min:    %u%% duty\n", dus |
| `kernel/tinker/dust_dislodger.c` | 137 | seq_printf(m, "spec_amp_cap:     %u%%\n", dust_saf |
| `kernel/tinker/dust_dislodger.c` | 138 | seq_puts(m, "policy:           idle-gated fan osci |
| `kernel/tinker/dust_dislodger.c` | 139 | seq_puts(m, "consent:          explicit toggle req |
| `kernel/tinker/dust_dislodger.c` | 149 | "write 'calibrate <max_rpm> [stall_guess]' first\n |
| `kernel/tinker/dust_dislodger.c` | 257 | &dust_fops); |
| `kernel/tinker/dvfs_shaver.c` | 45 | seq_printf(m, "state:            %s\n", states[dvf |
| `kernel/tinker/dvfs_shaver.c` | 46 | seq_printf(m, "peak_mhz:         %u\n", dvfs_peak_ |
| `kernel/tinker/dvfs_shaver.c` | 47 | seq_printf(m, "min_mhz:          %u\n", dvfs_min_m |
| `kernel/tinker/dvfs_shaver.c` | 48 | seq_printf(m, "transitions:      %llu\n", dvfs_tra |
| `kernel/tinker/dvfs_shaver.c` | 51 | seq_puts(m, "policy:           software-driven mic |
| `kernel/tinker/energy_sched.c` | 117 | size_t len, loff_t *ppos) |
| `kernel/tinker/enterprise_state.c` | 71 | seq_printf(m, "Domain:          %s %s\n", |
| `kernel/tinker/enterprise_state.c` | 74 | seq_printf(m, "SSO:             %s %s\n", |
| `kernel/tinker/enterprise_state.c` | 77 | seq_printf(m, "MFA:             %s %s\n", |
| `kernel/tinker/enterprise_state.c` | 80 | seq_printf(m, "GPO:             %s (%d policies)\n |
| `kernel/tinker/enterprise_state.c` | 82 | seq_printf(m, "SCIM:            %s\n", |
| `kernel/tinker/enterprise_state.c` | 84 | seq_printf(m, "VPN:             %s %s\n", |
| `kernel/tinker/enterprise_state.c` | 87 | seq_printf(m, "Audit:           %s\n", |
| `kernel/tinker/enterprise_state.c` | 90 | seq_printf(m, "  Total logins:       %d\n", ent_st |
| `kernel/tinker/enterprise_state.c` | 91 | seq_printf(m, "  Auth failures:      %d\n", ent_st |
| `kernel/tinker/enterprise_state.c` | 92 | seq_printf(m, "  MFA challenges:     %d\n", ent_st |
| `kernel/tinker/enterprise_state.c` | 183 | .proc_open    = enterprise_open, |
| `kernel/tinker/enterprise_state.c` | 185 | .proc_read    = seq_read, |
| `kernel/tinker/enterprise_state.c` | 200 | &enterprise_proc_ops); |
| `kernel/tinker/finance_audit.c` | 35 | seq_printf(m, "apps:              %u\n", fa_apps); |
| `kernel/tinker/fpga_scaler.c` | 32 | seq_printf(m, "enabled:          %u\n", fpga_enabl |
| `kernel/tinker/fpga_scaler.c` | 34 | seq_puts(m, "backend:          FPGA fabric (absent |
| `kernel/tinker/gamemode.c` | 97 | (task_tgid_nr(p) == gamemode_tgid); |
| `kernel/tinker/gamemode.c` | 163 | seq_printf(m, "tgid:    %d\n", gamemode_tgid); |
| `kernel/tinker/gamemode.c` | 170 | size_t len, loff_t *ppos) |
| `kernel/tinker/gamemode.c` | 235 | &gamemode_fops); |
| `kernel/tinker/hardware_dna.c` | 59 | seq_printf(m, "cpus:         %u\n", dna_cpus); |
| `kernel/tinker/hardware_tuning.c` | 30 | seq_printf(m, "governor:        %u\n", tune_govern |
| `kernel/tinker/hardware_tuning.c` | 32 | seq_printf(m, "fan_curve:       %u\n", tune_fan_cu |
| `kernel/tinker/hardware_tuning.c` | 33 | seq_puts(m, "policy:          low-level tuning far |
| `kernel/tinker/hardware_tuning.c` | 34 | seq_puts(m, "consent:         risky items explicit |
| `kernel/tinker/hw_cert.c` | 62 | seq_printf(m, "Certified:        %d\n", cert_st->t |
| `kernel/tinker/hw_cert.c` | 63 | seq_printf(m, "Tested:           %d\n", cert_st->t |
| `kernel/tinker/hw_cert.c` | 64 | seq_printf(m, "Thermal zones:    %d\n\n", cert_st- |
| `kernel/tinker/hw_cert.c` | 162 | .proc_open    = cert_open, |
| `kernel/tinker/hw_cert.c` | 164 | .proc_read    = seq_read, |
| `kernel/tinker/hw_cert.c` | 178 | &cert_proc_ops); |
| `kernel/tinker/installer_state.c` | 68 | case INSTALLER_PHASE_INIT:      return "init"; |
| `kernel/tinker/installer_state.c` | 69 | case INSTALLER_PHASE_DETECT:    return "detect"; |
| `kernel/tinker/installer_state.c` | 71 | case INSTALLER_PHASE_COPY:      return "copy"; |
| `kernel/tinker/installer_state.c` | 72 | case INSTALLER_PHASE_BOOT:      return "bootloader |
| `kernel/tinker/installer_state.c` | 73 | case INSTALLER_PHASE_USER:      return "user"; |
| `kernel/tinker/installer_state.c` | 74 | case INSTALLER_PHASE_CONFIG:    return "config"; |
| `kernel/tinker/installer_state.c` | 76 | case INSTALLER_PHASE_ERROR:     return "error"; |
| `kernel/tinker/installer_state.c` | 77 | default:                        return "unknown"; |
| `kernel/tinker/installer_state.c` | 91 | inst_st->install_start); |
| `kernel/tinker/installer_state.c` | 94 | inst_st->install_start); |
| `kernel/tinker/installer_state.c` | 98 | seq_printf(m, "Phase:            %s\n", phase_str( |
| `kernel/tinker/installer_state.c` | 99 | seq_printf(m, "Progress:         %d%%\n", inst_st- |
| `kernel/tinker/installer_state.c` | 100 | seq_printf(m, "Elapsed:          %lu.%lus\n", |
| `kernel/tinker/installer_state.c` | 103 | seq_printf(m, "  Target disk:    %s\n", inst_st->t |
| `kernel/tinker/installer_state.c` | 105 | seq_printf(m, "  Partition:      %s\n", inst_st->p |
| `kernel/tinker/installer_state.c` | 106 | seq_printf(m, "  Filesystem:     %s\n", inst_st->f |
| `kernel/tinker/installer_state.c` | 107 | seq_printf(m, "  Username:       %s\n", inst_st->u |
| `kernel/tinker/installer_state.c` | 108 | seq_printf(m, "  Hostname:       %s\n", inst_st->h |
| `kernel/tinker/installer_state.c` | 109 | seq_printf(m, "  Timezone:       %s\n", inst_st->t |
| `kernel/tinker/installer_state.c` | 110 | seq_printf(m, "  Locale:         %s\n", inst_st->l |
| `kernel/tinker/installer_state.c` | 111 | seq_printf(m, "  Keyboard:       %s\n", inst_st->k |
| `kernel/tinker/installer_state.c` | 112 | seq_printf(m, "  Encryption:     %s\n", inst_st->e |
| `kernel/tinker/installer_state.c` | 113 | seq_printf(m, "  LVM:            %s\n", inst_st->l |
| `kernel/tinker/installer_state.c` | 115 | seq_printf(m, "  Bootloader:     %s\n", inst_st->b |
| `kernel/tinker/installer_state.c` | 117 | seq_printf(m, "  Services:       %s\n", inst_st->s |
| `kernel/tinker/installer_state.c` | 118 | seq_printf(m, "  Validation:     %s\n", inst_st->v |
| `kernel/tinker/installer_state.c` | 142 | size_t count, loff_t *ppos) |
| `kernel/tinker/installer_state.c` | 218 | .proc_open    = installer_open, |
| `kernel/tinker/installer_state.c` | 220 | .proc_read    = seq_read, |
| `kernel/tinker/installer_state.c` | 235 | &installer_proc_ops); |
| `kernel/tinker/mobile_companion.c` | 63 | case MOBILE_CONN_NONE:         return "none"; |
| `kernel/tinker/mobile_companion.c` | 64 | case MOBILE_CONN_USB:          return "usb-adb"; |
| `kernel/tinker/mobile_companion.c` | 65 | case MOBILE_CONN_WIFI_ADB:     return "wifi-adb"; |
| `kernel/tinker/mobile_companion.c` | 67 | case MOBILE_CONN_SSH:          return "ssh"; |
| `kernel/tinker/mobile_companion.c` | 68 | default:                       return "unknown"; |
| `kernel/tinker/mobile_companion.c` | 80 | seq_printf(m, "Auto-connect:       %s\n", mob_st-> |
| `kernel/tinker/mobile_companion.c` | 82 | seq_printf(m, "Notifications:      %d\n", mob_st-> |
| `kernel/tinker/mobile_companion.c` | 85 | seq_printf(m, "Devices:            %d\n\n", mob_st |
| `kernel/tinker/mobile_companion.c` | 96 | seq_printf(m, "    Model:    %s\n", d->model); |
| `kernel/tinker/mobile_companion.c` | 97 | seq_printf(m, "    Android:  %s\n", d->android_ver |
| `kernel/tinker/mobile_companion.c` | 98 | seq_printf(m, "    Battery:  %d%% (%s)\n", |
| `kernel/tinker/mobile_companion.c` | 100 | seq_printf(m, "    Mirror:   %s\n", |
| `kernel/tinker/mobile_companion.c` | 102 | seq_printf(m, "    Notify:   %s (%d forwarded)\n", |
| `kernel/tinker/mobile_companion.c` | 117 | size_t count, loff_t *ppos) |
| `kernel/tinker/mobile_companion.c` | 210 | .proc_open    = mobile_open, |
| `kernel/tinker/mobile_companion.c` | 212 | .proc_read    = seq_read, |
| `kernel/tinker/neural_audio.c` | 33 | seq_printf(m, "enabled:          %u\n", na_enabled |
| `kernel/tinker/neural_audio.c` | 34 | seq_printf(m, "gain_pct:         %u\n", na_gain_pc |
| `kernel/tinker/neural_audio.c` | 35 | seq_printf(m, "noise_floor:      %u\n", na_noise_f |
| `kernel/tinker/neural_audio.c` | 36 | seq_printf(m, "frames:           %llu\n", na_frame |
| `kernel/tinker/neural_audio.c` | 37 | seq_puts(m, "engine:           enhancement chain ( |
| `kernel/tinker/neural_super_res.c` | 31 | seq_printf(m, "enabled:          %u\n", nsr_enable |
| `kernel/tinker/neural_super_res.c` | 32 | seq_printf(m, "scale:            %ux\n", nsr_scale |
| `kernel/tinker/neural_super_res.c` | 33 | seq_printf(m, "sharpness:        %u\n", nsr_sharpn |
| `kernel/tinker/neural_super_res.c` | 34 | seq_printf(m, "frames:           %llu\n", nsr_fram |
| `kernel/tinker/neural_super_res.c` | 35 | seq_puts(m, "engine:           neural upscale fron |
| `kernel/tinker/oled_wear.c` | 61 | seq_printf(m, "dim_pct:       %u%%\n", oled_dim_pc |
| `kernel/tinker/oled_wear.c` | 64 | seq_printf(m, "policy:        proportional burn-in |
| `kernel/tinker/pkg_tracker.c` | 49 | int action, int success) |
| `kernel/tinker/pkg_tracker.c` | 96 | seq_printf(m, "Total installs:    %d\n", pkg_state |
| `kernel/tinker/pkg_tracker.c` | 97 | seq_printf(m, "Total updates:     %d\n", pkg_state |
| `kernel/tinker/pkg_tracker.c` | 98 | seq_printf(m, "Total removes:     %d\n", pkg_state |
| `kernel/tinker/pkg_tracker.c` | 99 | seq_printf(m, "Verified:          %d\n", pkg_state |
| `kernel/tinker/pkg_tracker.c` | 115 | default:          action_str = "UNKNOWN"; break; |
| `kernel/tinker/pkg_tracker.c` | 134 | .proc_open    = pkg_open, |
| `kernel/tinker/pkg_tracker.c` | 135 | .proc_read    = seq_read, |
| `kernel/tinker/pkg_tracker.c` | 156 | action, name, version, &success); |
| `kernel/tinker/pkg_tracker.c` | 180 | .proc_open    = pkg_open, |
| `kernel/tinker/pkg_tracker.c` | 182 | .proc_read    = seq_read, |
| `kernel/tinker/pkg_tracker.c` | 197 | &pkg_proc_write_ops); |
| `kernel/tinker/predictive_prewarm.c` | 33 | seq_printf(m, "enabled:     %u\n", pw_enabled); |
| `kernel/tinker/predictive_prewarm.c` | 35 | seq_printf(m, "hits:        %llu\n", pw_hits); |
| `kernel/tinker/predictive_prewarm.c` | 36 | seq_printf(m, "misses:      %llu\n", pw_misses); |
| `kernel/tinker/predictive_prewarm.c` | 37 | seq_printf(m, "accuracy:    %u %%\n", |
| `kernel/tinker/predictive_prewarm.c` | 41 | seq_puts(m, "policy:      predictive prewarm of li |
| `kernel/tinker/predictive_render.c` | 32 | seq_printf(m, "enabled:         %u\n", pr_enabled) |
| `kernel/tinker/predictive_render.c` | 34 | seq_printf(m, "target_fps:      %u\n", pr_target_f |
| `kernel/tinker/predictive_render.c` | 35 | seq_printf(m, "hits:            %llu\n", pr_hits); |
| `kernel/tinker/predictive_render.c` | 36 | seq_printf(m, "misses:          %llu\n", pr_misses |
| `kernel/tinker/predictive_render.c` | 37 | seq_puts(m, "policy:          predictive frame pre |
| `kernel/tinker/ray_traced_audio.c` | 72 | rta.ceiling_reflect_pct) / 3; |
| `kernel/tinker/ray_traced_audio.c` | 85 | seq_printf(m, "enabled:           %u\n", rta.enabl |
| `kernel/tinker/ray_traced_audio.c` | 88 | seq_printf(m, "room (cm):         %ux%ux%u\n", rta |
| `kernel/tinker/ray_traced_audio.c` | 90 | seq_printf(m, "reflectivity:      W%u F%u C%u %%\n |
| `kernel/tinker/ray_traced_audio.c` | 94 | seq_printf(m, "occlusion_pct:     %u\n", rta.occlu |
| `kernel/tinker/ray_traced_audio.c` | 96 | seq_printf(m, "frames:            %llu\n", rta.fra |
| `kernel/tinker/ray_traced_audio.c` | 97 | seq_puts(m, "engine:            geometric ray-cast |
| `kernel/tinker/remote_hardware_api.c` | 34 | seq_printf(m, "armed:           %u\n", rha_armed); |
| `kernel/tinker/remote_hardware_api.c` | 35 | seq_printf(m, "allow_power:     %u\n", rha_allow_p |
| `kernel/tinker/remote_hardware_api.c` | 36 | seq_printf(m, "allow_fan:       %u\n", rha_allow_f |
| `kernel/tinker/remote_hardware_api.c` | 37 | seq_printf(m, "calls:           %llu\n", rha_calls |
| `kernel/tinker/remote_hardware_api.c` | 38 | seq_puts(m, "policy:          opt-in, armed-token  |
| `kernel/tinker/sdgpu.c` | 32 | seq_printf(m, "enabled:          %u\n", sdgpu_enab |
| `kernel/tinker/sdgpu.c` | 34 | seq_printf(m, "latency_ms:       %u\n", sdgpu_late |
| `kernel/tinker/sdgpu.c` | 35 | seq_printf(m, "power_cap_pct:    %u\n", sdgpu_powe |
| `kernel/tinker/sdgpu.c` | 36 | seq_puts(m, "backend:          DRM/compute hint pl |
| `kernel/tinker/smart_power_grid.c` | 38 | [PG_PERF]       = "performance", |
| `kernel/tinker/smart_power_grid.c` | 44 | seq_printf(m, "profile:       %s\n", profiles[pg_p |
| `kernel/tinker/smart_power_grid.c` | 48 | seq_puts(m, "policy:        value-aware energy sch |
| `kernel/tinker/thermal_sched.c` | 128 | msecs_to_jiffies(5000)); |
| `kernel/tinker/thermal_sched.c` | 139 | seq_printf(m, "enabled:        %u\n", enabled); |
| `kernel/tinker/thermal_sched.c` | 140 | seq_printf(m, "zones:          %u\n", heat.zones); |
| `kernel/tinker/thermal_sched.c` | 146 | seq_printf(m, "  cpu%d:       %llu mc\n", cpu, |
| `kernel/tinker/tinker.c` | 41 | seq_puts(m, "oled:     available (TINKER_OLED_WEAR |
| `kernel/tinker/unified_memory.c` | 31 | seq_printf(m, "enabled:          %u\n", um_enabled |
| `kernel/tinker/unified_memory.c` | 32 | seq_printf(m, "pools:            %u\n", um_pools); |
| `kernel/tinker/unified_memory.c` | 33 | seq_printf(m, "local_dram_mb:    %llu\n", um_local |
| `kernel/tinker/unified_memory.c` | 34 | seq_printf(m, "remote_mb:        %llu\n", um_remot |
| `kernel/tinker/unified_memory.c` | 35 | seq_puts(m, "policy:           software-defined ti |
| `kernel/tinker/unified_memory.c` | 36 | seq_puts(m, "consent:          opt-in only\n"); |
| `kernel/tinker/update_monitor.c` | 72 | case UPDATE_STATE_IDLE:      state_str = "idle"; b |
| `kernel/tinker/update_monitor.c` | 75 | case UPDATE_STATE_FAILED:    state_str = "failed"; |
| `kernel/tinker/update_monitor.c` | 77 | default:                     state_str = "unknown" |
| `kernel/tinker/update_monitor.c` | 81 | seq_printf(m, "State:            %s\n", state_str) |
| `kernel/tinker/update_monitor.c` | 82 | seq_printf(m, "Auto-update:      %s\n", upd_state- |
| `kernel/tinker/update_monitor.c` | 83 | seq_printf(m, "Total updates:    %d\n", upd_state- |
| `kernel/tinker/update_monitor.c` | 84 | seq_printf(m, "Rollbacks:        %d\n", upd_state- |
| `kernel/tinker/update_monitor.c` | 85 | seq_printf(m, "Failures:         %d\n", upd_state- |
| `kernel/tinker/update_monitor.c` | 91 | upd_state->kernels[i].active ? "[active]" : "      |
| `kernel/tinker/update_monitor.c` | 107 | size_t count, loff_t *ppos) |
| `kernel/tinker/update_monitor.c` | 155 | .proc_open    = update_open, |
| `kernel/tinker/update_monitor.c` | 157 | .proc_read    = seq_read, |
| `kernel/tinker/zero_latency_input.c` | 32 | seq_printf(m, "enabled:          %u\n", zl_enabled |
| `kernel/tinker/zero_latency_input.c` | 34 | seq_printf(m, "events:           %llu\n", zl_event |
| `kernel/tinker/zero_latency_input.c` | 35 | seq_puts(m, "policy:           low-latency input s |
| `kernel/tinker/zero_latency_input.c` | 78 | MAX_RT_PRIO - 1); |

### W01 line >120 cols — 228

| file | line | detail |
|------|------|--------|
| `os/apps/apps/nibra-betterlife/drizzle.config.ts` | 1 | 129 cols |
| `os/apps/apps/nibra-betterlife/db/schema.ts` | 2 | 271 cols |
| `os/apps/apps/nibra-betterlife/db/schema.ts` | 3 | 207 cols |
| `os/apps/apps/nibra-betterlife/scripts/build-service.mjs` | 1 | 151 cols |
| `os/apps/apps/nibra-betterlife/tests/account-service.test.mjs` | 1 | 189 cols |
| `os/apps/apps/nibra-betterlife/tests/account-service.test.mjs` | 2 | 411 cols |
| `os/apps/apps/nibra-betterlife/tests/account-service.test.mjs` | 3 | 222 cols |
| `os/apps/apps/nibra-betterlife/tests/account-service.test.mjs` | 4 | 1615 cols |
| `os/apps/apps/nibra-betterlife/tests/agent.test.mjs` | 1 | 189 cols |
| `os/apps/apps/nibra-betterlife/tests/agent.test.mjs` | 2 | 402 cols |
| `os/apps/apps/nibra-betterlife/tests/agent.test.mjs` | 3 | 511 cols |
| `os/apps/apps/nibra-betterlife/tests/agent.test.mjs` | 4 | 275 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 11 | 214 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 18 | 418 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 19 | 414 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 20 | 152 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 21 | 190 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 22 | 191 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 23 | 125 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 24 | 182 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 26 | 137 cols |
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 28 | 188 cols |
| `os/apps/apps/nibra-betterlife/tests/provider-boundary.mjs` | 1 | 199 cols |
| `os/apps/apps/nibra-betterlife/tests/provider-boundary.mjs` | 2 | 494 cols |
| `os/apps/apps/nibra-betterlife/tests/provider-boundary.mjs` | 3 | 420 cols |
| `os/apps/apps/nibra-betterlife/tests/provider-boundary.mjs` | 4 | 1694 cols |
| `os/apps/apps/nibra-betterlife/tests/v11-desktop.mjs` | 1 | 199 cols |
| `os/apps/apps/nibra-betterlife/tests/v11-desktop.mjs` | 2 | 3037 cols |
| `os/apps/apps/nibra-betterlife/src/AgentReview.tsx` | 1 | 165 cols |
| `os/apps/apps/nibra-betterlife/src/AgentReview.tsx` | 2 | 1981 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 2 | 415 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 3 | 121 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 8 | 279 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 11 | 307 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 12 | 318 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 13 | 464 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 15 | 800 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 17 | 134 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 19 | 200 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 20 | 153 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 23 | 122 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 24 | 131 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 25 | 277 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 27 | 220 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 28 | 271 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 30 | 561 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 31 | 184 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 34 | 340 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 35 | 529 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 36 | 488 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 37 | 1921 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 38 | 868 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 39 | 150 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 41 | 2369 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 42 | 629 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 44 | 6794 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 46 | 933 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 47 | 1641 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 48 | 1470 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 49 | 2047 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 50 | 1750 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 51 | 1183 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 53 | 1324 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 54 | 2369 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 55 | 3557 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 56 | 355 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 58 | 2306 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 59 | 1524 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 61 | 813 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 62 | 865 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 63 | 3016 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 64 | 1961 cols |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 65 | 186 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 1 | 196 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 2 | 171 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 3 | 139 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 4 | 167 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 5 | 378 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 6 | 212 cols |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 7 | 1680 cols |
| `os/apps/apps/nibra-betterlife/src/ChatPanel.tsx` | 1 | 237 cols |
| `os/apps/apps/nibra-betterlife/src/ChatPanel.tsx` | 2 | 427 cols |
| `os/apps/apps/nibra-betterlife/src/ChatPanel.tsx` | 3 | 1493 cols |
| `os/apps/apps/nibra-betterlife/src/ChatPanel.tsx` | 4 | 3885 cols |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 2 | 295 cols |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 3 | 588 cols |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 4 | 2344 cols |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 5 | 537 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 12 | 551 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 13 | 343 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 15 | 134 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 17 | 121 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 18 | 775 cols |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 20 | 355 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 1 | 147 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 3 | 122 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 4 | 168 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 5 | 125 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 6 | 125 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 9 | 527 cols |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 10 | 445 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 2 | 153 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 3 | 142 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 4 | 228 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 5 | 142 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 6 | 127 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 7 | 129 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 8 | 141 cols |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 9 | 144 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 1 | 295 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 2 | 241 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 3 | 1628 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 4 | 398 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 6 | 225 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 9 | 855 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 10 | 133 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 11 | 608 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 13 | 123 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 14 | 788 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 15 | 818 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 16 | 704 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 17 | 988 cols |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 18 | 576 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 41 | 156 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 55 | 152 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 512 | 143 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 519 | 155 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 524 | 149 cols |
| `os/apps/apps/aether-workspace/src/App.tsx` | 530 | 125 cols |
| `os/apps/apps/aether-workspace/src/utils/automations.ts` | 179 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 88 | 164 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 121 | 145 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 133 | 145 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 142 | 122 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 156 | 122 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 177 | 139 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 210 | 162 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 248 | 182 cols |
| `os/apps/apps/aether-workspace/src/components/AutomationsHub.tsx` | 259 | 180 cols |
| `os/apps/apps/aether-workspace/src/components/CommandPalette.tsx` | 181 | 154 cols |
| `os/apps/apps/aether-workspace/src/components/CommandPalette.tsx` | 196 | 123 cols |
| `os/apps/apps/aether-workspace/src/components/CommandPalette.tsx` | 215 | 128 cols |
| `os/apps/apps/aether-workspace/src/components/CommandPalette.tsx` | 236 | 134 cols |
| `os/apps/apps/aether-workspace/src/components/CommandPalette.tsx` | 248 | 144 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 106 | 175 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 130 | 148 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 138 | 135 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 142 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 150 | 131 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 172 | 137 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 181 | 131 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 193 | 131 cols |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 205 | 192 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 45 | 147 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 50 | 166 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 51 | 161 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 52 | 168 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 92 | 206 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 97 | 123 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 105 | 182 cols |
| `os/apps/apps/aether-workspace/src/components/Header.tsx` | 115 | 187 cols |
| `os/apps/apps/aether-workspace/src/components/LockScreen.tsx` | 41 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/LockScreen.tsx` | 66 | 157 cols |
| `os/apps/apps/aether-workspace/src/components/LockScreen.tsx` | 82 | 150 cols |
| `os/apps/apps/aether-workspace/src/components/LockScreen.tsx` | 95 | 124 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 60 | 189 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 179 | 145 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 182 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 185 | 122 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 195 | 128 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 222 | 128 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 242 | 130 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 257 | 187 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 302 | 198 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 311 | 155 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 322 | 143 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 344 | 132 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 353 | 155 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 370 | 140 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 446 | 156 cols |
| `os/apps/apps/aether-workspace/src/components/NoteEditor.tsx` | 463 | 160 cols |
| `os/apps/apps/aether-workspace/src/components/NotesList.tsx` | 85 | 209 cols |
| `os/apps/apps/aether-workspace/src/components/NotesList.tsx` | 113 | 129 cols |
| `os/apps/apps/aether-workspace/src/components/NotesList.tsx` | 130 | 154 cols |
| `os/apps/apps/aether-workspace/src/components/NotesList.tsx` | 158 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 153 | 143 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 238 | 142 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 254 | 187 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 273 | 161 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 290 | 211 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 302 | 147 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 317 | 141 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 331 | 180 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 362 | 140 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 367 | 147 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 375 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 385 | 157 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 390 | 135 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 396 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 399 | 139 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 402 | 180 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 405 | 163 cols |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 415 | 145 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 63 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 68 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 85 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 102 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 112 | 124 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 119 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 133 | 121 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 175 | 154 cols |
| `os/apps/apps/aether-workspace/src/components/Sidebar.tsx` | 191 | 124 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 89 | 156 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 91 | 161 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 93 | 155 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 95 | 140 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 119 | 199 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 130 | 124 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 142 | 176 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 149 | 128 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 174 | 130 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 258 | 143 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 267 | 171 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 276 | 195 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 285 | 127 cols |
| `os/apps/apps/aether-workspace/src/components/TasksManager.tsx` | 294 | 121 cols |
| `os/browser-extension/chrome/content.js` | 198 | 137 cols |
| `os/brand/output/tailwind.config.js` | 26 | 247 cols |

### C08 legacy ~/.tinker state path — 197

| file | line | detail |
|------|------|--------|
| `os/iso-builder.sh` | 7 | BUILD_DIR="$HOME/.tinker/iso-build" |
| `os/apps/battery-monitor.sh` | 7 | BAT_DIR="$HOME/.tinker/battery" |
| `os/apps/file-manager.sh` | 6 | FM_DIR="$HOME/.tinker/filemanager" |
| `os/apps/package-manager.sh` | 7 | PKG_DIR="$HOME/.tinker/packages" |
| `os/apps/quick-note.sh` | 7 | NOTE_DIR="$HOME/.tinker/notes" |
| `os/apps/smart-clipboard.sh` | 7 | CLIP_DIR="$HOME/.tinker/clipboard" |
| `os/apps/system-cleaner.sh` | 7 | CLEAN_DIR="$HOME/.tinker/cleaner" |
| `os/apps/gaming/anticheat-helper.sh` | 6 | AC_DIR="$HOME/.tinker/anticheat" |
| `os/apps/gaming/audio-mixer.sh` | 6 | AM_DIR="$HOME/.tinker/audio" |
| `os/apps/gaming/controller-mapper.sh` | 6 | MAPPER_DIR="$HOME/.tinker/controller" |
| `os/apps/gaming/discord-presence.sh` | 6 | DISCORD_DIR="$HOME/.tinker/discord" |
| `os/apps/gaming/emulator-manager.sh` | 6 | EMU_DIR="$HOME/.tinker/emulators" |
| `os/apps/gaming/fps-monitor.sh` | 6 | FPS_DIR="$HOME/.tinker/fps" |
| `os/apps/gaming/game-launcher.sh` | 6 | LAUNCHER_DIR="$HOME/.tinker/game-launcher" |
| `os/apps/gaming/game-replay.sh` | 6 | REPLAY_DIR="$HOME/.tinker/replay" |
| `os/apps/gaming/game-saves-sync.sh` | 6 | SYNC_DIR="$HOME/.tinker/game-sync" |
| `os/apps/gaming/gif-recorder.sh` | 6 | GIF_DIR="$HOME/.tinker/gifs" |
| `os/apps/gaming/hardware-benchmark.sh` | 6 | HB_DIR="$HOME/.tinker/benchmark" |
| `os/apps/gaming/performance-graph.sh` | 6 | GRAPH_DIR="$HOME/.tinker/perf-graph" |
| `os/apps/gaming/streaming-manager.sh` | 6 | STREAM_DIR="$HOME/.tinker/streaming" |
| `os/apps/gaming/wine-manager.sh` | 6 | WINE_DIR="$HOME/.tinker/wine" |
| `os/apps/customization/conky-stats.sh` | 4 | CONKY_DIR="$HOME/.tinker/conky" |
| `os/apps/customization/cursor-themes.sh` | 6 | CURSOR_DIR="$HOME/.tinker/cursor" |
| `os/apps/customization/desktop-effects.sh` | 6 | EFFECTS_DIR="$HOME/.tinker/desktop-effects" |
| `os/apps/customization/font-manager.sh` | 4 | FONT_DIR="$HOME/.tinker/fonts" |
| `os/apps/customization/grub-theme.sh` | 6 | GRUB_DIR="$HOME/.tinker/grub-theme" |
| `os/apps/customization/gtk-theme.sh` | 4 | GTK_DIR="$HOME/.tinker/gtk-theme" |
| `os/apps/customization/icon-packs.sh` | 6 | ICON_DIR="$HOME/.tinker/icons" |
| `os/apps/customization/login-theme.sh` | 6 | LOGIN_DIR="$HOME/.tinker/login-theme" |
| `os/apps/customization/qt-theme.sh` | 4 | QT_DIR="$HOME/.tinker/qt-theme" |
| `os/apps/customization/shell-theme.sh` | 4 | SHELL_DIR="$HOME/.tinker/shell-theme" |
| `os/apps/customization/wallpaper-manager.sh` | 4 | WP_DIR="$HOME/.tinker/wallpaper" |
| `os/apps/customization/window-animations.sh` | 6 | ANIM_DIR="$HOME/.tinker/animations" |
| `os/apps/security/filevault.sh` | 6 | VAULT_DIR="$HOME/.tinker/vaults" |
| `os/apps/security/findmydevice.sh` | 6 | FMD_DIR="$HOME/.tinker/findmy" |
| `os/apps/security/firewall.sh` | 12 | FW_DIR="$HOME/.tinker/firewall" |
| `os/apps/security/gatekeeper.sh` | 6 | GK_DIR="$HOME/.tinker/gatekeeper" |
| `os/apps/security/password-manager.sh` | 13 | PM_DIR="$HOME/.tinker/passwords" |
| `os/apps/security/privacy.sh` | 7 | PV_DIR="$HOME/.tinker/privacy" |
| `os/apps/security/security-suite.sh` | 6 | SS_DIR="$HOME/.tinker/security" |
| `os/apps/hardware/display-calibration.sh` | 6 | CAL_DIR="$HOME/.tinker/display-cal" |
| `os/apps/hardware/docking-station.sh` | 6 | DOCK_DIR="$HOME/.tinker/docking" |
| `os/apps/hardware/fingerprint-manager.sh` | 6 | FP_DIR="$HOME/.tinker/fingerprint" |
| `os/apps/hardware/gpio-manager.sh` | 6 | GPIO_DIR="$HOME/.tinker/gpio" |
| `os/apps/hardware/hdr-manager.sh` | 6 | HDR_DIR="$HOME/.tinker/hdr" |
| `os/apps/hardware/kvm-switch.sh` | 6 | KVM_DIR="$HOME/.tinker/kvm" |
| `os/apps/hardware/nfc-manager.sh` | 6 | NFC_DIR="$HOME/.tinker/nfc" |
| `os/apps/hardware/pen-stylus.sh` | 6 | PEN_DIR="$HOME/.tinker/pen" |
| `os/apps/hardware/printer-manager.sh` | 6 | PM_DIR="$HOME/.tinker/printers" |
| `os/apps/hardware/scanner-manager.sh` | 6 | SCANNER_DIR="$HOME/.tinker/scanner" |
| `os/apps/hardware/serial-uart.sh` | 6 | SERIAL_DIR="$HOME/.tinker/serial" |
| `os/apps/hardware/thunderbolt-manager.sh` | 6 | TB_DIR="$HOME/.tinker/thunderbolt" |
| `os/apps/hardware/touchscreen-manager.sh` | 6 | TOUCH_DIR="$HOME/.tinker/touchscreen" |
| `os/apps/hardware/usb-manager.sh` | 9 | USB_DIR="$HOME/.tinker/usb" |
| `os/apps/hardware/webcam-manager.sh` | 6 | WC_DIR="$HOME/.tinker/webcam" |
| `os/apps/system/adaptive-power-grid.sh` | 6 | GRID_DIR="$HOME/.tinker/power-grid" |
| `os/apps/system/auto-updates.sh` | 6 | UPDATE_DIR="$HOME/.tinker/auto-updates" |
| `os/apps/system/backup-restore.sh` | 8 | BACKUP_DIR="$HOME/.tinker/backups" |
| `os/apps/system/cognitive-load.sh` | 10 | CL_CONFIG="$HOME/.tinker/cognitive-load.conf" |
| `os/apps/system/cognitive-load.sh` | 11 | CL_STATE="$HOME/.tinker/cognitive-load.state" |
| `os/apps/system/cognitive-load.sh` | 38 | echo "[$(date +%T)] suppressed non-urgent notification: $*" >> "$HOME/ |
| `os/apps/system/command-palette.sh` | 6 | PALETTE_DIR="$HOME/.tinker/palette" |
| `os/apps/system/context-aware.sh` | 6 | CA_DIR="$HOME/.tinker/context-aware" |
| `os/apps/system/digital-twin.sh` | 6 | TWIN_DIR="$HOME/.tinker/digital-twin" |
| `os/apps/system/disk-visualizer.sh` | 6 | DISK_DIR="$HOME/.tinker/disk-viz" |
| `os/apps/system/duplicate-finder.sh` | 6 | DUP_DIR="$HOME/.tinker/duplicates" |
| `os/apps/system/fast-boot.sh` | 6 | FASTBOOT_DIR="$HOME/.tinker/fast-boot" |
| `os/apps/system/file-versioning.sh` | 6 | VCS_DIR="$HOME/.tinker/vcs" |
| `os/apps/system/focus-mode.sh` | 6 | FOCUS_DIR="$HOME/.tinker/focus" |
| `os/apps/system/global-search.sh` | 6 | SEARCH_DIR="$HOME/.tinker/search" |
| `os/apps/system/markdown-editor.sh` | 6 | MD_DIR="$HOME/.tinker/markdown" |
| `os/apps/system/parental-controls.sh` | 6 | PARENT_DIR="$HOME/.tinker/parental" |
| `os/apps/system/pomodoro-timer.sh` | 6 | POMO_DIR="$HOME/.tinker/pomodoro" |
| `os/apps/system/power-manager.sh` | 6 | POWER_DIR="$HOME/.tinker/power-manager" |
| `os/apps/system/predictive-caching.sh` | 6 | PFA_DIR="$HOME/.tinker/pfa" |
| `os/apps/system/predictive-intelligence.sh` | 6 | PSI_DIR="$HOME/.tinker/psi" |
| `os/apps/system/quick-actions.sh` | 6 | ACTIONS_DIR="$HOME/.tinker/quick-actions" |
| `os/apps/system/rollback-recovery.sh` | 6 | ROLLBACK_DIR="$HOME/.tinker/rollback" |
| `os/apps/system/screen-time.sh` | 6 | SCREEN_DIR="$HOME/.tinker/screen-time" |
| `os/apps/system/self-healing.sh` | 6 | HEAL_DIR="$HOME/.tinker/self-healing" |
| `os/apps/system/system-monitor.sh` | 6 | MONITOR_DIR="$HOME/.tinker/system-monitor" |
| `os/apps/system/temporal-mapping.sh` | 6 | TRM_DIR="$HOME/.tinker/trm" |
| `os/apps/system/time-tracker.sh` | 6 | TRACK_DIR="$HOME/.tinker/time-tracker" |
| `os/apps/network/bandwidth-limiter.sh` | 6 | BW_DIR="$HOME/.tinker/bandwidth" |
| `os/apps/network/dns-manager.sh` | 6 | DNS_DIR="$HOME/.tinker/dns" |
| `os/apps/network/firewall-gui.sh` | 6 | FW_DIR="$HOME/.tinker/firewall" |
| `os/apps/network/hotspot-manager.sh` | 6 | HOTSPOT_DIR="$HOME/.tinker/hotspot" |
| `os/apps/network/mesh-network.sh` | 6 | MESH_DIR="$HOME/.tinker/mesh" |
| `os/apps/network/network-monitor.sh` | 6 | NM_DIR="$HOME/.tinker/net-monitor" |
| `os/apps/network/proxy-manager.sh` | 6 | PROXY_DIR="$HOME/.tinker/proxy" |
| `os/apps/network/speed-test.sh` | 6 | ST_DIR="$HOME/.tinker/speedtest" |
| `os/apps/network/vpn-manager.sh` | 6 | VPN_DIR="$HOME/.tinker/vpn" |
| `os/apps/network/wifi-analyzer.sh` | 6 | WA_DIR="$HOME/.tinker/wifi" |
| `os/security/biometric.sh` | 6 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/filevault.sh` | 7 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/findmydevice.sh` | 6 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/firewall.sh` | 6 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/gatekeeper.sh` | 7 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/privacy.sh` | 6 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/security/sip.sh` | 6 | SECURITY_DIR="$HOME/.tinker/security" |
| `os/vokk/agent/system-agent.sh` | 4 | AGENT_DIR="$HOME/.tinker/agent"; AGENT_CONFIG="$AGENT_DIR/config.json" |
| `os/vokk/agent/system-agent.sh` | 327 | AGENT_DIR="$HOME/.tinker/agent" |
| `os/vokk/agent/system-agent.sh` | 405 | AGENT_DIR="$HOME/.tinker/agent" |
| `os/system/adaptive-power-grid.sh` | 44 | GRID_DIR="$HOME/.tinker/power-grid" |
| `os/system/audio-clarity.sh` | 10 | AUDIO_DIR="$HOME/.tinker/audio" |
| `os/system/auto-updates.sh` | 6 | UPDATE_DIR="$HOME/.tinker/updates" |
| `os/system/backup-restore.sh` | 6 | BACKUP_DIR="$HOME/.tinker/backups" |
| `os/system/backup-restore.sh` | 54 | --exclude="$HOME/.tinker/backups" \ |
| `os/system/context-aware-adaptation.sh` | 5 | CSS_CONTEXT="$HOME/.tinker/css_context.json" |
| `os/system/context-aware-adaptation.sh` | 6 | CSS_PREFERENCES="$HOME/.tinker/css_preferences.json" |
| `os/system/context-aware-adaptation.sh` | 7 | CSS_CONFIG="$HOME/.tinker/css_config.json" |
| `os/system/context-aware-adaptation.sh` | 9 | mkdir -p "$HOME/.tinker" |
| `os/system/context-aware-adaptation.sh` | 16 | "log_file": "$HOME/.tinker/css_context.log", |
| `os/system/context-aware.sh` | 35 | CSS_DIR="$HOME/.tinker/context" |
| `os/system/digital-twin.sh` | 39 | TWIN_DIR="$HOME/.tinker/twin" |
| `os/system/fast-boot.sh` | 6 | BOOT_DIR="$HOME/.tinker/boot" |
| `os/system/feature-manager.sh` | 5 | CONFIG_FILE="$HOME/.tinker/optional-features.conf" |
| `os/system/gaming-meta.sh` | 12 | META_MARKER="$HOME/.tinker/gaming-meta.installed" |
| `os/system/optimization-toggles.sh` | 6 | OPT_DIR="$HOME/.tinker/optimizations" |
| `os/system/password-manager.sh` | 17 | TINKER_HOME="$HOME/.tinker" |
| `os/system/password-monitor.sh` | 274 | echo "  4. Select: $HOME/.tinker/browser-extension/firefox/" |
| `os/system/password-monitor.sh` | 282 | echo "  5. Select: $HOME/.tinker/browser-extension/chrome/" |
| `os/system/password-monitor.sh` | 293 | local ext_dir="$HOME/.tinker/browser-extension" |
| `os/system/predictive-caching.sh` | 33 | PFA_DIR="$HOME/.tinker/caching" |
| `os/system/predictive-intelligence.sh` | 5 | PSI_HISTORY="$HOME/.tinker/psi_history.json" |
| `os/system/predictive-intelligence.sh` | 6 | PSI_PREDICTIONS="$HOME/.tinker/psi_predictions.json" |
| `os/system/predictive-intelligence.sh` | 7 | PSI_CONFIG="$HOME/.tinker/psi_config.json" |
| `os/system/predictive-intelligence.sh` | 9 | mkdir -p "$HOME/.tinker" |
| `os/system/predictive-intelligence.sh` | 16 | "log_file": "$HOME/.tinker/psi_actions.log", |
| `os/system/predictive-pre-caching.sh` | 5 | PFA_HISTORY="$HOME/.tinker/pfa_history.json" |
| `os/system/predictive-pre-caching.sh` | 6 | PFA_CACHE="$HOME/.tinker/pfa_cache.json" |
| `os/system/predictive-pre-caching.sh` | 7 | PFA_CONFIG="$HOME/.tinker/pfa_config.json" |
| `os/system/predictive-pre-caching.sh` | 8 | PFA_LOG="$HOME/.tinker/pfa_access.log" |
| `os/system/predictive-pre-caching.sh` | 10 | mkdir -p "$HOME/.tinker" |
| `os/system/rollback-recovery.sh` | 6 | RECOVERY_DIR="$HOME/.tinker/recovery" |
| `os/system/self-healing.sh` | 40 | HEAL_DIR="$HOME/.tinker/self-healing" |
| `os/system/temporal-mapping.sh` | 28 | TRM_DIR="$HOME/.tinker/temporal" |
| `os/system/temporal-resource-mapping.sh` | 5 | TRM_HISTORY="$HOME/.tinker/trm_history.json" |
| `os/system/temporal-resource-mapping.sh` | 6 | TRM_FORECASTS="$HOME/.tinker/trm_forecastrates.json" |
| `os/system/temporal-resource-mapping.sh` | 7 | TRM_CONFIG="$HOME/.tinker/trm_config.json" |
| `os/system/temporal-resource-mapping.sh` | 9 | mkdir -p "$HOME/.tinker" |
| `os/system/temporal-resource-mapping.sh` | 16 | "log_file": "$HOME/.tinker/trm_resources.log", |
| `os/data/hardware-db.sh` | 5 | HARDWARE_DB="$HOME/.tinker/hardware.db" |
| `os/data/user-profiles.sh` | 4 | PROFILES_DB="$HOME/.tinker/profiles.db" |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 7 | DAEMON_WS_DIR="$HOME/.tinker/mobile-companion" |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 8 | DAEMON_LOG="$HOME/.tinker/mobile-companion.log" |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 9 | SERVER="$HOME/.tinker/mobile-companion/mobile-companion-server.py" |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 4 | NAE_CONFIG="$HOME/.tinker/neural-audio.json"; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 4 | GPT_CONFIG="$HOME/.tinker/gpu-toggle.json"; GPT_KEY="$HOME/.tinker/gpu |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 5 | PORT=8768; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 3 | RAPI_CONFIG="$HOME/.tinker/remote-api.json"; RAPI_KEY="$HOME/.tinker/a |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 4 | PORT=8767; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/smart-power-grid/smart-power-grid.sh` | 3 | SPG_CONFIG="$HOME/.tinker/smart-power.json"; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 5 | CW_DIR="$HOME/.tinker/coil-whine-killer"; CW_CONFIG="$CW_DIR/config.js |
| `os/hardware-tech/cross-app-automation/cross-app-automation.sh` | 5 | CAA_DIR="$HOME/.tinker/automations"; CAA_LOG="$CAA_DIR/audit.log" |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 5 | RTA_DIR="$HOME/.tinker/ray-traced-audio"; RTA_CONFIG="$RTA_DIR/config. |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 5 | SA_DIR="$HOME/.tinker/finance"; SA_DB="$SA_DIR/finance.db" |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 5 | OLED_DIR="$HOME/.tinker/oled-shield"; OLED_CONFIG="$OLED_DIR/config.js |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 4 | ZLI_CONFIG="$HOME/.tinker/zero-latency.json"; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 5 | CT_DIR="$HOME/.tinker/cache-tiering"; CT_CONFIG="$CT_DIR/config.json" |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 5 | DS_DIR="$HOME/.tinker/data-shredder"; DS_LOG="$DS_DIR/shred.log" |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 3 | SDGPU_CONFIG="$HOME/.tinker/sdgpu.json"; mkdir -p "$HOME/.tinker" /tmp |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 4 | DNA_DIR="$HOME/.tinker/hardware-dna"; mkdir -p "$DNA_DIR" |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 5 | LIFE_DIR="$HOME/.tinker/lifespan-doubler"; LIFE_CONFIG="$LIFE_DIR/conf |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 5 | CXL_DIR="$HOME/.tinker/cxl-memory"; CXL_CONFIG="$CXL_DIR/config.json" |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 5 | UCM_DIR="$HOME/.tinker/ucm"; UCM_DB="$UCM_DIR/memory.db" |
| `os/hardware-tech/predictive-prewarm/predictive-prewarm.sh` | 5 | PW_DIR="$HOME/.tinker/predictive-prewarm"; PW_CONFIG="$PW_DIR/config.j |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 5 | FPGA_DIR="$HOME/.tinker/fpga-scaler"; FPGA_CONFIG="$FPGA_DIR/config.js |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 3 | UCP_CONFIG="$HOME/.tinker/ucp.json"; mkdir -p "$HOME/.tinker" |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 5 | ES_DIR="$HOME/.tinker/energy-scheduler"; ES_CONFIG="$ES_DIR/config.jso |
| `os/hardware-tech/lib/hardware-consent.sh` | 7 | CONSENT_DIR="$HOME/.tinker/consent" |
| `os/hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 4 | TSS_DIR="$HOME/.tinker/thermal-scheduler"; TSS_CONFIG="$TSS_DIR/config |
| `os/hardware-tech/predictive-render/predictive-render.sh` | 3 | PR_CONFIG="$HOME/.tinker/predictive-render.json"; mkdir -p "$HOME/.tin |
| `os/hardware-tech/neural-super-res/neural-super-res.sh` | 3 | NSR_CONFIG="$HOME/.tinker/neural-super-res.json"; mkdir -p "$HOME/.tin |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 5 | AD_CONFIG="$HOME/.tinker/adaptive-display.json"; mkdir -p "$HOME/.tink |
| `os/hardware-tech/dvfs-shaver/dvfs-shaver.sh` | 5 | DVFS_DIR="$HOME/.tinker/dvfs-shaver"; DVFS_CONFIG="$DVFS_DIR/config.js |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 5 | DUST_DIR="$HOME/.tinker/dust-dislodger"; DUST_CONFIG="$DUST_DIR/config |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 180 | f_hz="$(python3 -c "import json;print(json.load(open('$HOME/.tinker/du |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 181 | f_dur="$(python3 -c "import json;print(json.load(open('$HOME/.tinker/d |
| `os/desktop/app-launcher.sh` | 7 | FAVORITES_FILE="$HOME/.tinker/favorites" |
| `os/desktop/clipboard-manager.sh` | 6 | CLIP_DIR="$HOME/.tinker/clipboard" |
| `os/desktop/desktop-integration.sh` | 7 | DESKTOP_DIR="$HOME/.tinker/desktop" |
| `os/desktop/desktop-integration.sh` | 10 | PLUGINS_DIR="$HOME/.tinker/plugins" |
| `os/desktop/dock.sh` | 6 | DOCK_DIR="$HOME/.tinker/dock" |
| `os/desktop/file-search.sh` | 6 | SEARCH_INDEX="$HOME/.tinker/search-index" |
| `os/desktop/help-system.sh` | 6 | HELP_DIR="$HOME/.tinker/help" |
| `os/desktop/multi-monitor.sh` | 140 | local config_file="$HOME/.tinker/monitor-config.conf" |
| `os/desktop/multi-monitor.sh` | 148 | local config_file="$HOME/.tinker/monitor-config.conf" |
| `os/desktop/notification-center.sh` | 6 | NOTIF_DIR="$HOME/.tinker/notifications" |
| `os/desktop/settings-gui.sh` | 6 | CONFIG_DIR="$HOME/.tinker" |
| `os/desktop/setup-wizard.sh` | 7 | CONFIG_DIR="$HOME/.tinker" |
| `os/desktop/shortcuts.sh` | 133 | local rc="$HOME/.tinker/.xbindkeysrc" |
| `os/desktop/shortcuts.sh` | 134 | mkdir -p "$HOME/.tinker" "$SHORTCUTS_DIR" |
| `os/desktop/text-expander.sh` | 6 | EXPANDER_DIR="$HOME/.tinker/text-expander" |
| `os/desktop/theme-manager.sh` | 6 | THEME_DIR="$HOME/.tinker/themes" |
| `os/desktop/widgets.sh` | 6 | WIDGET_DIR="$HOME/.tinker/widgets" |
| `os/account/parc-id` | 4 | ACCOUNT_DIR="$HOME/.tinker/account" |

### C07 hardcoded /tmp path — 169

| file | line | detail |
|------|------|--------|
| `os/install-parcai.sh` | 95 | if [ -p /tmp/korrinos_ai_pipe ]; then |
| `os/install-parcai.sh` | 96 | read -r cmd < /tmp/korrinos_ai_pipe \|\| true |
| `os/install-parcai.sh` | 97 | [ -n "$cmd" ] && bash "$AI_DIR/vokk.sh" $cmd > /tmp/korrinos_ai_output |
| `os/install-parcai.sh` | 105 | mkfifo /tmp/korrinos_ai_pipe 2>/dev/null \|\| true |
| `os/install-parcai.sh` | 106 | chmod 666 /tmp/korrinos_ai_pipe 2>/dev/null \|\| true |
| `os/iso-artifacts.sh` | 22 | for f in /tmp/iso-final.log /tmp/iso-relog.log /tmp/iso-build.log; do |
| `os/publish-iso.sh` | 37 | if ! ssh -i "$KEY" -o BatchMode=yes -o ConnectTimeout=15 "${USER}@${HO |
| `os/publish-iso.sh` | 38 | echo "  SSH handshake failed:"; sed 's/^/    /' /tmp/sf-auth.check |
| `os/publish-iso.sh` | 43 | cat /tmp/sf-auth.check |
| `os/apps/app-store.sh` | 167 | wget -O /tmp/discord.deb "https://discordapp.com/api/download?platform |
| `os/apps/app-store.sh` | 168 | sudo dpkg -i /tmp/discord.deb |
| `os/apps/app-store.sh` | 175 | wget -O /tmp/zoom.deb "https://zoom.us/client/latest/zoom_amd64.deb" |
| `os/apps/app-store.sh` | 176 | sudo dpkg -i /tmp/zoom.deb |
| `os/apps/gaming-mode.sh` | 14 | GAMING_MODE_FILE="/tmp/tinker-gaming-mode" |
| `os/apps/gaming-mode.sh` | 15 | CPU_GOVERNOR_FILE="/tmp/tinker-cpu-governor" |
| `os/apps/gaming-mode.sh` | 71 | if [ -f /tmp/tinker-notifications ]; then |
| `os/apps/gaming-mode.sh` | 72 | kill -STOP $(cat /tmp/tinker-notifications) 2>/dev/null \|\| true |
| `os/apps/gaming-mode.sh` | 133 | if [ -f /tmp/tinker-notifications ]; then |
| `os/apps/gaming-mode.sh` | 134 | kill -CONT $(cat /tmp/tinker-notifications) 2>/dev/null \|\| true |
| `os/apps/gaming-support.sh` | 109 | local archive="/tmp/ge-proton.tar.gz" |
| `os/apps/ocr-everywhere.sh` | 54 | local screenshot="/tmp/ocr-region-$(date +%s).png" |
| `os/apps/ocr-everywhere.sh` | 86 | local screenshot="/tmp/ocr-full-$(date +%s).png" |
| `os/apps/ocr-everywhere.sh` | 170 | local clipboard_img="/tmp/ocr-clipboard-$(date +%s).png" |
| `os/apps/ocr-everywhere.sh` | 214 | local frame="/tmp/ocr-frame-$(date +%s).png" |
| `os/apps/voice-commands.sh` | 123 | local output="/tmp/voice-record-$(date +%s).wav" |
| `os/apps/voice-commands.sh` | 353 | local audio="/tmp/voice-interactive-$(date +%s).wav" |
| `os/apps/voice-commands.sh` | 438 | arecord -f S16_LE -r 16000 -c 1 -d 10 /tmp/calibration.wav 2>/dev/null |
| `os/apps/system/backup-restore.sh` | 190 | #   'x.tar; touch /tmp/pwn; #' |
| `os/apps/system/markdown-editor.sh` | 52 | pandoc "$file" -o /tmp/preview.html && xdg-open /tmp/preview.html |
| `os/apps/system/power-manager.sh` | 89 | # like ../../../../../tmp/evil traversed out of the profile directory  |
| `os/apps/system/power-manager.sh` | 91 | # execution: `power-manager.sh apply ../../../../tmp/x` ran `x` with t |
| `os/hyperdrive/hyperdrive-cli.sh` | 192 | dd if=/dev/zero of=/tmp/bench bs=1M count=1024 2>&1 \| grep -E "copied\| |
| `os/hyperdrive/hyperdrive-cli.sh` | 193 | rm -f /tmp/bench |
| `os/hyperdrive/hyperdrive-daemon.sh` | 24 | if [ -f /tmp/mangohud_stats ]; then |
| `os/hyperdrive/hyperdrive-daemon.sh` | 25 | CURRENT_FPS=$(grep -o 'fps:[0-9]*' /tmp/mangohud_stats \| cut -d: -f2 \| |
| `os/hyperdrive/hyperdrive.sh` | 65 | DISK_SPEED=$(dd if=/dev/zero of=/tmp/hd_test bs=1M count=100 oflag=dsy |
| `os/hyperdrive/hyperdrive.sh` | 66 | rm -f /tmp/hd_test |
| `os/parc-ai/korrinos-focus.sh` | 75 | sox -n /tmp/focus_ambient.wav synth 1800 brownnoise vol 0.1 fade 0 180 |
| `os/parc-ai/korrinos-focus.sh` | 76 | mpv --loop --no-video /tmp/focus_ambient.wav &>/dev/null & |
| `os/parc-ai/korrinos-voice.sh` | 60 | pico2wave -w /tmp/tts_out.wav -l "$voice" "$text" 2>/dev/null && \ |
| `os/parc-ai/korrinos-voice.sh` | 61 | paplay /tmp/tts_out.wav 2>/dev/null |
| `os/parc-ai/korrinos-voice.sh` | 105 | arecord -d "$duration" -f S16_LE -r 16000 /tmp/voice_cmd.wav 2>/dev/nu |
| `os/parc-ai/korrinos-voice.sh` | 109 | whisper /tmp/voice_cmd.wav --language en --output_format txt --output_ |
| `os/parc-ai/korrinos-voice.sh` | 110 | local text=$(cat /tmp/voice_cmd.txt 2>/dev/null) |
| `os/parc-ai/parcos-media.sh` | 156 | local output="${1:-/tmp/recording_$(date +%Y%m%d_%H%M%S).mp4}" |
| `os/parc-ai/parcos-tools.sh` | 92 | local out="${1:-/tmp/screenshot_$(date +%Y%m%d_%H%M%S).png}" |
| `os/parc-ai/parcos-usb.sh` | 114 | local mnt="/tmp/usb_mount" |
| `os/parc-ai/modules/agent-browser.sh` | 40 | cp "$db_path" /tmp/browser_history.db 2>/dev/null |
| `os/parc-ai/modules/agent-browser.sh` | 43 | db = sqlite3.connect('/tmp/browser_history.db') |
| `os/parc-ai/modules/ai-image-gen.sh` | 8 | local output="${5:-/tmp/vokk_card_$(date +%s).png}" |
| `os/parc-ai/modules/ai-image-gen.sh` | 51 | local output="${3:-/tmp/vokk_code_$(date +%s).png}" |
| `os/parc-ai/modules/ai-image-gen.sh` | 117 | local output="${3:-/tmp/vokk_flow_$(date +%s).png}" |
| `os/parc-ai/modules/ai-image-gen.sh` | 162 | local output="${4:-/tmp/vokk_chart_$(date +%s).png}" |
| `os/parc-ai/modules/ai-image-gen.sh` | 208 | local output="${3:-/tmp/vokk_alert_$(date +%s).png}" |
| `os/parc-ai/modules/ai-image-gen.sh` | 249 | local output="${3:-/tmp/vokk_progress_$(date +%s).png}" |
| `os/parc-ai/modules/ai-master-brain.sh` | 6 | TINKERAI_MEMORY="/tmp/vokk_memory.json" |
| `os/parc-ai/modules/ai-narrative.sh` | 371 | AI_CONVERSATION_MEMORY_FILE="/tmp/vokk_memory.json" |
| `os/parc-ai/modules/ai-personality.sh` | 6 | TINKERAI_CONTEXT_FILE="/tmp/vokk_context.json" |
| `os/parc-ai/modules/ai-voice.sh` | 35 | echo "$text" \| piper --output_file /tmp/vokk_speech.wav && paplay /tmp |
| `os/parc-ai/modules/ai-voice.sh` | 46 | local output="/tmp/vokk_voice_$(date +%s).wav" |
| `os/parc-ai/modules/device.sh` | 169 | local output="${1:-/tmp/screenshot-$(date +%s).png}" |
| `os/parc-ai/modules/multimodal.sh` | 70 | local prompt="$1" output="${2:-/tmp/tinker-generated.png}" |
| `os/parc-ai/modules/multimodal.sh` | 148 | local text="$1" output="${2:-/tmp/tinker-tts.wav}" |
| `os/parc-ai/modules/parcos-features.sh` | 304 | tk_clipboard_history="/tmp/korrinos_clipboard.txt" |
| `os/vokk/agent/system-agent.sh` | 50 | cat > /tmp/tinker_hotkey.py << 'PYKEY' |
| `os/vokk/agent/system-agent.sh` | 91 | echo "  Daemon script: /tmp/tinker_hotkey.py" |
| `os/vokk/agent/system-agent.sh` | 92 | echo "  Run: python3 /tmp/tinker_hotkey.py" |
| `os/system/bluetooth-manager.sh` | 15 | BT_STATE="/tmp/tinker-bt-state" |
| `os/system/content-filter.sh` | 53 | { cat "$backup"; grep -v -f <(cut -d' ' -f2 "$backup" \| sed '/./s/.*/\| |
| `os/system/content-filter.sh` | 54 | sudo cp /tmp/tinkos-hosts /etc/hosts |
| `os/system/content-filter.sh` | 64 | sudo rm -f "$CF_HOSTS_MARK" /tmp/tinkos-hosts 2>/dev/null \|\| true |
| `os/system/default-apps.sh` | 30 | (curl -fsSL https://go.microsoft.com/fwlink/?LinkID=760868 -o /tmp/vsc |
| `os/system/default-apps.sh` | 31 | && dpkg -i /tmp/vscode.deb \|\| true) |
| `os/system/default-apps.sh` | 38 | -o /tmp/chrome.deb && dpkg -i /tmp/chrome.deb \|\| \ |
| `os/system/default-apps.sh` | 40 | curl -fsSLo /tmp/brave.deb https://github.com/brave/brave-browser/rele |
| `os/system/default-apps.sh` | 41 | && dpkg -i /tmp/brave.deb \|\| true) |
| `os/system/fast-boot.sh` | 59 | cat > /tmp/parallel-boot.conf << 'EOF' |
| `os/system/fast-boot.sh` | 64 | sudo mv /tmp/parallel-boot.conf /etc/systemd/system.conf.d/ |
| `os/system/gamemode-setup.sh` | 17 | GM_LOG="/tmp/tinker-gamemode-setup.log" |
| `os/system/hardware-detect.sh` | 14 | HD_LOG="/tmp/tinker-hardware-detect.log" |
| `os/system/hardware-detect.sh` | 15 | INSTALL_MARKER="/tmp/tinker-hw-detected" |
| `os/system/hardware-detect.sh` | 36 | [ -n "$nvidia" ] && echo "nvidia" >> /tmp/tinker-gpu-raw |
| `os/system/hardware-detect.sh` | 37 | [ -n "$amd" ] && echo "amd" >> /tmp/tinker-gpu-raw |
| `os/system/hardware-detect.sh` | 38 | [ -n "$intel" ] && echo "intel" >> /tmp/tinker-gpu-raw |
| `os/system/hardware-detect.sh` | 51 | done < /tmp/tinker-gpu-raw 2>/dev/null |
| `os/system/hardware-detect.sh` | 76 | echo "$profile" > /tmp/tinker-hw-profile |
| `os/system/hardware-detect.sh` | 77 | echo "$layers" > /tmp/tinker-hw-layers |
| `os/system/hardware-detect.sh` | 162 | local layers=$(cat /tmp/tinker-hw-layers 2>/dev/null) |
| `os/system/hardware-detect.sh` | 179 | rm -f /tmp/tinker-gpu-raw /tmp/tinker-hw-profile /tmp/tinker-hw-layers |
| `os/system/hardware-detect.sh` | 190 | echo "  Driver Layers: $(cat /tmp/tinker-hw-layers)" |
| `os/system/hardware-detect.sh` | 198 | log "Driver installation complete. Summary written to /tmp/tinker-hw-s |
| `os/system/hardware-detect.sh` | 203 | echo "Layers: $(cat /tmp/tinker-hw-layers)" |
| `os/system/hardware-detect.sh` | 205 | } > /tmp/tinker-hw-summary |
| `os/system/korrinos-battery-bar.sh` | 7 | BAT_BAR_HTML="/tmp/korrinos-battery-bar.html" |
| `os/system/parc-gamemode-hook.sh` | 18 | LOG="/tmp/tinker-gamemode-hook.log" |
| `os/system/password-manager.sh` | 168 | rm -f /tmp/tinker-vault-session |
| `os/system/password-manager.sh` | 178 | if [ -f /tmp/tinker-vault-session ]; then |
| `os/system/password-manager.sh` | 179 | cat /tmp/tinker-vault-session |
| `os/system/password-manager.sh` | 183 | echo "$master_pass" > /tmp/tinker-vault-session |
| `os/system/password-manager.sh` | 184 | chmod 600 /tmp/tinker-vault-session |
| `os/system/password-manager.sh` | 289 | local master=$(cat /tmp/tinker-vault-session) |
| `os/system/password-manager.sh` | 487 | LOCK_FILE="/tmp/tinker-vault-session" |
| `os/system/power-manager.sh` | 15 | POWER_STATE="/tmp/tinker-power-state" |
| `os/system/hardware-cert/korrinos-cert.sh` | 202 | dd if=/dev/urandom of=/tmp/memtest bs=1M count="$test_size" 2>/dev/nul |
| `os/system/hardware-cert/korrinos-cert.sh` | 204 | read_back=$(md5sum /tmp/memtest 2>/dev/null \| awk '{print $1}') |
| `os/system/hardware-cert/korrinos-cert.sh` | 205 | rm -f /tmp/memtest |
| `os/system/hardware-cert/korrinos-cert.sh` | 220 | disk_write=$(dd if=/dev/zero of=/tmp/disktest bs=1M count=100 oflag=di |
| `os/system/hardware-cert/korrinos-cert.sh` | 221 | disk_read=$(dd if=/tmp/disktest of=/dev/null bs=1M iflag=direct 2>&1 \| |
| `os/system/hardware-cert/korrinos-cert.sh` | 222 | rm -f /tmp/disktest |
| `os/system/installer/korrinos-installer.sh` | 224 | dd if=/dev/zero of=/tmp/korrinos-swap bs=1M count=$((swap_size * 1024) |
| `os/system/installer/korrinos-installer.sh` | 225 | chmod 600 /tmp/korrinos-swap |
| `os/system/installer/korrinos-installer.sh` | 226 | mkswap /tmp/korrinos-swap 2>/dev/null |
| `os/system/installer/korrinos-installer.sh` | 267 | swapon /tmp/korrinos-swap 2>/dev/null \|\| true |
| `os/system/installer/korrinos-installer.sh` | 301 | /tmp/korrinos-swap none          swap    sw              0       0 |
| `os/system/mobile-companion/korrinos-mobile.sh` | 537 | subprocess.run(['scrot', '/tmp/screenshot.png'], capture_output=True) |
| `os/system/mobile-companion/korrinos-mobile.sh` | 540 | with open('/tmp/screenshot.png', 'rb') as f: |
| `os/territories/cue-watchdog.sh` | 35 | if [ -x /tmp/opencode/tiocsti ] && [ -c "$tty" ]; then |
| `os/territories/cue-watchdog.sh` | 36 | /tmp/opencode/tiocsti "$tty" "cue on" >/dev/null 2>&1 \|\| true |
| `os/territories/secure/canary-monitor.sh` | 31 | : > /tmp/canary-baseline.tmp |
| `os/territories/secure/canary-monitor.sh` | 33 | [ -e "$f" ] && { echo "$f $(sha256sum "$f" \| awk '{print $1}')"; } >>  |
| `os/territories/secure/canary-monitor.sh` | 35 | cp /tmp/canary-baseline.tmp "$BASELINE" |
| `os/territories/vibe-address/core/bulk.sh` | 46 | { fp=$5; f="/tmp/vibe-bulk-fp." fp; print $0 >> f } |
| `os/territories/vibe-address/core/bulk.sh` | 48 | for ff in /tmp/vibe-bulk-fp.*; do |
| `os/territories/vibe-address/core/bulk.sh` | 50 | fp=${ff#/tmp/vibe-bulk-fp.} |
| `os/territories/vibe-address/core/bulk.sh` | 53 | rm -f /tmp/vibe-bulk-fp.* |
| `os/territories/vibe-address/core/bulk.sh` | 130 | print bm "\t" fp >> "/tmp/vibe-tm-m" |
| `os/territories/vibe-address/core/bulk.sh` | 131 | print bh "\t" fp >> "/tmp/vibe-tm-h" |
| `os/territories/vibe-address/core/bulk.sh` | 132 | print bd "\t" fp >> "/tmp/vibe-tm-d" |
| `os/territories/vibe-address/core/bulk.sh` | 133 | print bw "\t" fp >> "/tmp/vibe-tm-w" |
| `os/territories/vibe-address/core/bulk.sh` | 134 | print bmo "\t" fp >> "/tmp/vibe-tm-mo" |
| `os/territories/vibe-address/core/bulk.sh` | 135 | print bq "\t" fp >> "/tmp/vibe-tm-q" |
| `os/territories/vibe-address/core/bulk.sh` | 136 | print by "\t" fp >> "/tmp/vibe-tm-y" |
| `os/territories/vibe-address/core/bulk.sh` | 139 | ff="/tmp/vibe-tm-$level" |
| `os/territories/vibe-address/core/ingest.sh` | 231 | "echo F7 > /tmp/vibe-f7.pressed & $vah ask-complete" |
| `os/territories/vibe-address/core/prf.sh` | 97 | done \| sort -t'\|' -k2nr > /tmp/PRF_ranked_$$ |
| `os/territories/vibe-address/core/prf.sh` | 104 | done < <(head -"$VE_PRF_N" /tmp/PRF_ranked_$$ \| cut -d'\|' -f1) |
| `os/territories/vibe-address/core/prf.sh` | 113 | rm -f /tmp/PRF_ranked_$$ |
| `os/territories/game/perf-tune.sh` | 23 | has dd && echo "Disk write (1G): $(dd if=/dev/zero of=/tmp/perf.t bs=1 |
| `os/territories/hack/amnesia-firewall.sh` | 22 | LOG="/tmp/tinker-amnesia.log" |
| `os/territories/hack/masked-proc.sh` | 50 | local mountpoint="${1:-/tmp/tinker-decoy-proc}" |
| `os/territories/hack/panic-wipe.sh` | 51 | > /tmp/tinker-amnesia.log 2>/dev/null \|\| true |
| `os/territories/hack/threat-monitor.sh` | 71 | if ! bash -n "$f" 2>/tmp/bug-err; then |
| `os/territories/hack/threat-monitor.sh` | 72 | debug "SYNTAX ERROR in $f:"; sed 's/^/    /' /tmp/bug-err |
| `os/territories/hack/threat-monitor.sh` | 82 | python3 -m py_compile "$f" 2>/tmp/py-err && echo "  py_compile OK: $f" |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 301 | ["arecord", "-d", "1", "-f", "S16_LE", "-r", "44100", "-c", "1", "/tmp |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 306 | with open("/tmp/cw_scan.wav", "rb") as f: |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 151 | ["import", "-window", "root", "-resize", "256x256", "/tmp/oled_sample. |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 156 | img = Image.open("/tmp/oled_sample.png") |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 344 | img.save("/tmp/oled_compensation.png") |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 346 | print(f"  Mask saved: /tmp/oled_compensation.png") |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 100 | cat > /tmp/telemetry_noise.py << 'PYNOISE' |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 153 | python3 /tmp/telemetry_noise.py |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 3 | SDGPU_CONFIG="$HOME/.tinker/sdgpu.json"; mkdir -p "$HOME/.tinker" /tmp |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 34 | open('/tmp/tinker-dna-opt.json','w').write(json.dumps(opt)) |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 40 | gov=$(python3 -c "import json;print(json.load(open('/tmp/tinker-dna-op |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 41 | iosched=$(python3 -c "import json;print(json.load(open('/tmp/tinker-dn |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 42 | swappy=$(python3 -c "import json;print(json.load(open('/tmp/tinker-dna |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 43 | dirty=$(python3 -c "import json;print(json.load(open('/tmp/tinker-dna- |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 44 | turbo=$(python3 -c "import json;print(json.load(open('/tmp/tinker-dna- |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 61 | rm -f /tmp/tinker-dna-opt.json |
| `os/desktop/gestures.sh` | 301 | echo $! > /tmp/tinker-gestures.pid |
| `os/desktop/gestures.sh` | 306 | if [ -f /tmp/tinker-gestures.pid ]; then |
| `os/desktop/gestures.sh` | 307 | kill $(cat /tmp/tinker-gestures.pid) 2>/dev/null |
| `os/desktop/gestures.sh` | 308 | rm -f /tmp/tinker-gestures.pid |
| `os/desktop/night-mode.sh` | 6 | NIGHT_MODE_FILE="/tmp/tinker-night-mode" |
| `os/desktop/shortcuts.sh` | 421 | if [ -f /tmp/tinker-gaming-mode ]; then |
| `os/desktop/shortcuts.sh` | 429 | if [ -f /tmp/tinker-focus-mode ]; then |
| `os/desktop/tiling.sh` | 15 | TILING_STATE="/tmp/tinker-tiling-state" |
| `os/desktop/virtual-desktops.sh` | 15 | DESKTOPS_STATE="/tmp/tinker-desktops-state" |

### C11 error text on stdout — 135

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 723 | [ -z "$kernel_found" ] && echo "   ERROR: no kernel found!" |
| `os/build-distro.sh` | 833 | echo "ERROR: no squashfs at $IMAGE/casper/filesystem.squashfs — run 'b |
| `os/publish-iso.sh` | 38 | echo "  SSH handshake failed:"; sed 's/^/    /' /tmp/sf-auth.check |
| `os/release.sh` | 24 | [ -z "$CURRENT" ] && { echo "ERROR: cannot read current version from R |
| `os/release.sh` | 70 | echo "ABORT: selftest failed" |
| `os/release.sh` | 104 | [ -f "$ISO_PATH" ] \|\| { echo "ERROR: ISO not found after build"; exit  |
| `os/apps/app-store.sh` | 207 | echo -e "${RED} Failed to install $app${NC}" |
| `os/apps/gaming-support.sh` | 118 | echo "  Download failed." |
| `os/apps/gaming/screenshot-tool.sh` | 43 | [ -f "$path" ] && echo "  Saved: $path" \|\| echo "  Capture failed" |
| `os/apps/gaming/screenshot-tool.sh` | 63 | [ -f "$path" ] && echo "  Saved: $path" \|\| echo "  Capture cancelled/f |
| `os/apps/gaming/screenshot-tool.sh` | 82 | [ -f "$path" ] && echo "  Saved: $path" \|\| echo "  Capture failed" |
| `os/apps/security/biometric.sh` | 80 | [ $rc -eq 0 ] && echo "   Fingerprint verified" \|\| echo "   Verificati |
| `os/apps/security/security-suite.sh` | 96 | echo "Recent failed login attempts:" |
| `os/apps/hardware/fingerprint-manager.sh` | 70 | fprintd-verify "$USER" 2>/dev/null \|\| echo "Verification failed" |
| `os/apps/hardware/gpio-manager.sh` | 75 | local value=$(cat /sys/class/gpio/gpio$pin/value 2>/dev/null \|\| echo " |
| `os/apps/hardware/webcam-manager.sh` | 74 | [ -f "$WC_DIR/test-001.jpg" ] && echo "     Captured test frame: $WC_D |
| `os/apps/system/auto-updates.sh` | 154 | echo "Update failed (exit code: $result)" |
| `os/apps/system/auto-updates.sh` | 155 | echo "$(date +%s)\|failed\|$type" >> "$HISTORY_FILE" |
| `os/apps/system/backup-restore.sh` | 126 | echo "Could not build the backup pipeline (see error above)." |
| `os/apps/system/backup-restore.sh` | 162 | echo "Verification: FAILED" |
| `os/apps/system/backup-restore.sh` | 176 | echo "Backup failed!" |
| `os/apps/system/backup-restore.sh` | 177 | echo "$(date +%s)\|create\|$name\|0\|0\|failed" >> "$LOG_FILE" |
| `os/apps/system/backup-restore.sh` | 243 | echo "Restore failed!" |
| `os/apps/system/backup-restore.sh` | 244 | echo "$(date +%s)\|restore\|$name\|failed" >> "$LOG_FILE" |
| `os/apps/system/backup-restore.sh` | 345 | verify) tar -tf "$BACKUP_DIR/$2.tar" >/dev/null 2>&1 && echo "OK" \|\| e |
| `os/apps/system/rollback-recovery.sh` | 105 | echo "Snapshot failed" |
| `os/apps/system/rollback-recovery.sh` | 106 | echo "$(date +%s)\|create\|$name\|$tool\|failed" >> "$LOG_FILE" |
| `os/apps/system/rollback-recovery.sh` | 153 | echo "Rollback failed" |
| `os/apps/system/rollback-recovery.sh` | 154 | echo "$(date +%s)\|rollback\|$name\|$tool\|failed" >> "$LOG_FILE" |
| `os/apps/system/self-healing.sh` | 114 | echo "  [WARN] $failed failed service(s)" |
| `os/apps/system/terminal-error-explainer.sh` | 47 | echo -e "${BLUE}── KorrinOS Terminal Error Explainer ──${NC}" |
| `os/parc-ai/korrinos-annotate.sh` | 27 | [ -f "$output" ] && echo "$output" \|\| echo "Screenshot failed" |
| `os/parc-ai/korrinos-liquid-glass.sh` | 205 | echo "Error: picom failed to start. Check $GLASS_DIR/picom.log" |
| `os/parc-ai/korrinos-liquid-glass.sh` | 209 | echo "Error: picom not installed. Install with: sudo apt install picom |
| `os/parc-ai/korrinos-monitor.sh` | 226 | kill -"$signal" "$pid" 2>/dev/null && echo "   Sent ${signal} to ${pid |
| `os/parc-ai/korrinos-network.sh` | 168 | nslookup "$test_domain" 2>/dev/null \| grep -q "Address:" && echo "     |
| `os/parc-ai/korrinos-shell-ai.sh` | 61 | echo "  error [text]        Explain last error" |
| `os/parc-ai/korrinos-smoothui.sh` | 233 | echo " Failed to start picom" |
| `os/parc-ai/korrinos-tinkeria.sh` | 163 | echo "Help me debug this error. What could be wrong?" |
| `os/parc-ai/korrinos-tinkeria.sh` | 182 | echo "Explain this error message: $*" |
| `os/parc-ai/korrinos-tinkeria.sh` | 195 | echo "  debug        — Help debug an error" |
| `os/parc-ai/korrinos-tinkeria.sh` | 201 | echo "  explain-error <msg> — Explain an error" |
| `os/parc-ai/korrinos-voice.sh` | 201 | echo "Notification types: disk_full, battery_low, update_complete, err |
| `os/parc-ai/korrinos-widgets-panel.sh` | 182 | echo "Error: conky not installed. Install with: sudo apt install conky |
| `os/parc-ai/parcos-recovery.sh` | 108 | sudo grub-mkconfig -o /boot/grub/grub.cfg 2>/dev/null \|\| echo "GRUB up |
| `os/parc-ai/parcos-recovery.sh` | 116 | sudo update-initramfs -u 2>/dev/null \|\| echo "initramfs update failed" |
| `os/parc-ai/parcos-terminal.sh` | 65 | echo "Analyzing error..." |
| `os/parc-ai/parcos-terminal.sh` | 71 | echo "Tinkeria not available for error analysis" |
| `os/parc-ai/parcos-terminal.sh` | 147 | echo "  tinker fix '...'  Fix error with AI" |
| `os/parc-ai/parcos-terminal.sh` | 169 | echo "  fix <error>           Fix error with AI" |
| `os/parc-ai/modules/agent-system.sh` | 38 | echo "$cmd" \| sudo -S bash 2>/dev/null \|\| echo "Sudo failed or not ava |
| `os/parc-ai/modules/agent-system.sh` | 207 | xdotool type --clearmodifiers "$text" 2>/dev/null \|\| echo "Type failed |
| `os/parc-ai/modules/agent-system.sh` | 228 | xdotool key --clearmodifiers "$key" 2>/dev/null \|\| echo "Key failed" |
| `os/parc-ai/modules/agent-vision.sh` | 16 | echo "Screenshot failed"; return 1 |
| `os/parc-ai/modules/ai-master-brain.sh` | 98 | echo "$lower" \| grep -qE "(frustrated\|annoying\|stupid\|dumb\|broken\|does |
| `os/parc-ai/modules/ai-personality.sh` | 57 | if echo "$input" \| grep -qiE "frustrated\|annoying\|stupid\|dumb\|broken\|d |
| `os/parc-ai/modules/ai-personality.sh` | 166 | if echo "$error" \| grep -qi "not found\\|command not found"; then |
| `os/parc-ai/modules/ai-personality.sh` | 168 | elif echo "$error" \| grep -qi "permission denied\\|access denied"; then |
| `os/parc-ai/modules/ai-personality.sh` | 170 | elif echo "$error" \| grep -qi "timeout\\|timed out"; then |
| `os/parc-ai/modules/ai-personality.sh` | 172 | elif echo "$error" \| grep -qi "connection\\|network\\|offline"; then |
| `os/parc-ai/modules/ai-personality.sh` | 175 | echo "Something went wrong: $error. Would you like me to try again?" |
| `os/parc-ai/modules/cards-interactive.sh` | 38 | " 2>/dev/null \|\| echo "<div class='ai-card'>Scenario error</div>" |
| `os/parc-ai/modules/cards-interactive.sh` | 238 | " 2>/dev/null \|\| echo "<div class='ai-card'>Map error</div>" |
| `os/parc-ai/modules/cards-live.sh` | 140 | " 2>/dev/null \|\| echo "<div class='ai-card'>Data visualization error</ |
| `os/parc-ai/modules/cards-structural.sh` | 47 | " 2>/dev/null \|\| echo "<div class='ai-card'>Flowchart error</div>" |
| `os/parc-ai/modules/cards-structural.sh` | 90 | " 2>/dev/null \|\| echo "<div class='ai-card'>State machine error</div>" |
| `os/parc-ai/modules/cards-structural.sh` | 222 | " 2>/dev/null \|\| echo "<div class='ai-card'>Pipeline error</div>" |
| `os/parc-ai/modules/cards-workspace.sh` | 44 | " 2>/dev/null \|\| echo "<div class='ai-card'>File explorer error</div>" |
| `os/parc-ai/modules/cards-workspace.sh` | 82 | " 2>/dev/null \|\| echo "<div class='ai-card'>Diff viewer error</div>" |
| `os/parc-ai/modules/codegen.sh` | 149 | " 2>/dev/null \|\| echo "Could not analyze error. Please share the full  |
| `os/parc-ai/modules/codegen.sh` | 162 | echo "/* - Error handling patterns */" |
| `os/parc-ai/modules/math.sh` | 47 | " 2>/dev/null \|\| echo "Conversion failed" |
| `os/parc-ai/modules/web.sh` | 41 | " 2>/dev/null \|\| echo "Failed to fetch URL" |
| `os/security/biometric.sh` | 19 | echo "Fingerprint enrollment failed" |
| `os/security/biometric.sh` | 49 | echo "Face enrollment failed" |
| `os/security/filevault.sh` | 89 | echo -e "${RED} Decryption failed${NC}" |
| `os/system/bluetooth-manager.sh` | 130 | echo -e "${RED} Connection failed${NC}" |
| `os/system/default-apps.sh` | 39 | (echo "Chrome failed; installing Brave" && |
| `os/system/digital-twin.sh` | 440 | echo "Health check FAILED - initiating auto-rollback" |
| `os/system/korrinos-errors.sh` | 162 | local explanation=$(echo "A command failed with exit code $exit_code.  |
| `os/system/korrinos-errors.sh` | 172 | local explanation=$(echo "Explain this Linux error in simple terms and |
| `os/system/korrinos-errors.sh` | 182 | local explanation=$(echo "Error: $exit_code — $stderr" \| vokk 2>/dev/n |
| `os/system/korrinos-errors.sh` | 208 | echo -e "\033[1;31m║    Error Detected (Exit Code: $exit_code)         |
| `os/system/korrinos-errors.sh` | 279 | echo -e "\033[1;31mScript error at line $line (exit code $code)\033[0m |
| `os/system/korrinos-errors.sh` | 302 | echo "KorrinOS Error System installed!" |
| `os/system/korrinos-errors.sh` | 310 | echo "KorrinOS Error Monitor running..." |
| `os/system/korrinos-errors.sh` | 313 | if echo "$line" \| grep -qi "error\\|fault\\|panic\\|oops"; then |
| `os/system/korrinos-errors.sh` | 322 | echo "Testing error system..." |
| `os/system/korrinos-errors.sh` | 329 | echo "KorrinOS Human Error System v1.0" |
| `os/system/korrinos-errors.sh` | 341 | echo "  • Automatic error trapping in scripts" |
| `os/system/backup/korrinos-backup.sh` | 88 | echo "ERROR: No sources configured for backup." |
| `os/system/package-manager/korrinos-pkg.sh` | 34 | echo "ERROR: Package manager lock timeout. Remove $lockfile manually." |
| `os/system/package-manager/korrinos-pkg.sh` | 242 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 344 | echo "FAILED: Could not install $pkg from any source." |
| `os/system/package-manager/korrinos-pkg.sh` | 358 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 381 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 407 | [ -z "$query" ] && { echo "ERROR: search query required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 452 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 781 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/package-manager/korrinos-pkg.sh` | 833 | [ -z "$repo_url" ] && { echo "ERROR: repository URL required"; return  |
| `os/system/package-manager/korrinos-pkg.sh` | 865 | [ -z "$pkg" ] && { echo "ERROR: package name required"; return 1; } |
| `os/system/appstore/korrinos-appstore.sh` | 136 | echo "  WARNING: Signature verification failed" |
| `os/system/appstore/korrinos-appstore.sh` | 143 | echo "  Failed to download metadata" |
| `os/system/appstore/korrinos-appstore.sh` | 442 | echo "Download failed: $download_url" |
| `os/system/security/korrinos-bugfix.sh` | 74 | echo "ERROR: Path traversal detected: $path"\ |
| `os/system/security/korrinos-bugfix.sh` | 181 | echo "ERROR: Insufficient disk space ($available MB available, $min_mb |
| `os/system/security/korrinos-bugfix.sh` | 195 | echo "12/30: JSON error handling..." |
| `os/system/security/korrinos-bugfix.sh` | 201 | echo "   JSON error handling verified." |
| `os/system/security/korrinos-bugfix.sh` | 217 | echo "14/30: Error message consistency..." |
| `os/system/security/korrinos-bugfix.sh` | 221 | sed -i 's/echo "ERROR:/echo "ERROR:/g' "$script" 2>/dev/null \|\| true |
| `os/system/security/korrinos-bugfix.sh` | 223 | echo "   Error messages standardized." |
| `os/system/security/korrinos-health.sh` | 170 | echo "  Connectivity: FAILED" |
| `os/system/security/korrinos-health.sh` | 178 | echo "  DNS: FAILED" |
| `os/system/security/korrinos-health.sh` | 315 | echo "  Failed SSH attempts (24h): $failed" |
| `os/system/cloud-sync/korrinos-cloud.sh` | 272 | curl -s -u "$user:$pass" -T "$f" "$base_url$rel" 2>/dev/null && echo " |
| `os/system/cloud-sync/korrinos-cloud.sh` | 282 | curl -s -u "$user:$pass" "$base_url$f" -o "$sync_dir/$filename" 2>/dev |
| `os/system/cloud-sync/korrinos-cloud.sh` | 412 | curl -s -u "$user:$pass" -T "$f" "$url/$rel" 2>/dev/null && echo "  $r |
| `os/system/update-system/korrinos-update.sh` | 33 | echo "ERROR: Update lock timeout. Remove $UPDATE_LOCK manually." |
| `os/system/update-system/korrinos-update.sh` | 406 | echo "Update failed. Retry $retry_count/$max_retries..." |
| `os/system/update-system/korrinos-update.sh` | 799 | echo "Last update failed. Auto-rollback available." |
| `os/system/mobile-companion/korrinos-mobile.sh` | 395 | echo "File transfer failed." |
| `os/system/mobile-companion/korrinos-mobile.sh` | 417 | echo "Receive failed." |
| `os/system/enterprise/korrinos-enterprise.sh` | 165 | echo "ERROR: Domain not found. Check DNS and network." |
| `os/system/enterprise/korrinos-enterprise.sh` | 553 | echo "SCIM connection failed (HTTP $http_code)" |
| `os/system/enterprise/korrinos-enterprise.sh` | 748 | echo "User creation failed." |
| `os/territories/vibe-address/core/action.sh` | 120 | rm -rf "$rpath2" && echo "  DELETED  $rpath2" \|\| echo "  FAILED   $rpa |
| `os/territories/vibe-address/core/selftest.sh` | 253 | echo "SELFTEST FAIL  ($fails/$total failed)" |
| `os/territories/hack/ephemeral-ram.sh` | 37 | echo "bind failed; symlinking world state instead" |
| `os/territories/hack/threat-monitor.sh` | 44 | echo "=== Recent failed-auth (brute-force?) ===" |
| `os/territories/hack/threat-monitor.sh` | 46 | echo "  total failed passwords logged: ${county:-0}" |
| `os/territories/hack/threat-monitor.sh` | 82 | python3 -m py_compile "$f" 2>/tmp/py-err && echo "  py_compile OK: $f" |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 47 | echo "ERROR: python3 not installed — required for the WebSocket server |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 46 | xrandr --output $(xrandr \| grep " connected" \| head -1 \| awk '{print $ |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 389 | echo "   Safety check failed. Aborting." |
| `os/languages/install.sh` | 17 | echo "Error: Python 3 required" |

### C02 no strict mode — 131

| file | line | detail |
|------|------|--------|
| `os/apps/apps/aether-workspace.sh` | 1 | no 'set -euo pipefail' |
| `os/apps/apps/nibra-betterlife.sh` | 1 | no 'set -euo pipefail' |
| `os/apps/apps/parc-ai.sh` | 1 | no 'set -euo pipefail' |
| `os/scripts/integrate-gpu-features.sh` | 1 | no 'set -euo pipefail' |
| `os/hyperdrive/hyperdrive-cli.sh` | 1 | no 'set -euo pipefail' |
| `os/hyperdrive/hyperdrive-daemon.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/parcos-uninstall-blocker.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/overlay/agent-narrator.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/overlay/narrator.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/agent-automation.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/agent-browser.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/agent-system.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/agent-vision.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-brain-smart.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-engine.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-image-gen.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-master-brain.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-narrative.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-personality.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-self-learn.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/ai-voice.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/cards-interactive.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/cards-live.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/cards-structural.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/cards-visual.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/cards-workspace.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/commerce.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/contacts.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/conversation.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/creative.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/device.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/entertainment.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/execution.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/finance.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/knowledge-parcos.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/language.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/math.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/multimodal.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/nlp-engine.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/nlp-training.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/nlu.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/persona.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/productivity.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/textgen.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/travel.sh` | 1 | no 'set -euo pipefail' |
| `os/parc-ai/modules/web.sh` | 1 | no 'set -euo pipefail' |
| `os/vokk/vokk.sh` | 1 | no 'set -euo pipefail' |
| `os/vokk/agent/system-agent.sh` | 1 | no 'set -euo pipefail' |
| `os/system/context-aware-adaptation.sh` | 1 | no 'set -euo pipefail' |
| `os/system/feature-manager.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-apt-hook.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-battery-bar.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-errors.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-firstboot.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-greetings.sh` | 1 | no 'set -euo pipefail' |
| `os/system/korrinos-mascot.sh` | 1 | no 'set -euo pipefail' |
| `os/system/parc-gamemode-hook.sh` | 1 | no 'set -euo pipefail' |
| `os/system/predictive-intelligence.sh` | 1 | no 'set -euo pipefail' |
| `os/system/predictive-pre-caching.sh` | 1 | no 'set -euo pipefail' |
| `os/system/temporal-resource-mapping.sh` | 1 | no 'set -euo pipefail' |
| `os/control-center/launch_feature.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/vibe-address/core/audit.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/game/three-worlds-expand.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/vokk/vokk-lock.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/vokk/vokk-scheduler.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/vokk/vokk.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/lib/common.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/daily-driver/enhanced.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/ai-scheduler/native-scheduler.sh` | 1 | no 'set -euo pipefail' |
| `os/territories/ai-scheduler/prioritize.sh` | 1 | no 'set -euo pipefail' |
| `os/data/hardware-db.sh` | 1 | no 'set -euo pipefail' |
| `os/data/user-profiles.sh` | 1 | no 'set -euo pipefail' |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/neural-audio/neural-audio-engine.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/remote-hardware-api/gpu-phone-toggle.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/smart-power-grid/smart-power-grid.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/cross-app-automation/cross-app-automation.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/cache-tiering/cache-tiering.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/sdgpu/software-gpu.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-dna/hardware-dna.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/unified-memory/unified-contextual-memory.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/predictive-prewarm/predictive-prewarm.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/unified-control-plane/unified-control-plane.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/lib/backend-helper.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/lib/hardware-consent.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/predictive-render/predictive-render.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/neural-super-res/neural-super-res.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/adaptive-display/adaptive-display.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/14-categories.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/audio-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/camera-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/display-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/input-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/led-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/memory-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/network-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/power-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/security-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/storage-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/thermal-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/hardware-tuning/usb-tuning.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/dvfs-shaver/dvfs-shaver.sh` | 1 | no 'set -euo pipefail' |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 1 | no 'set -euo pipefail' |
| `os/desktop/system-tray.sh` | 1 | no 'set -euo pipefail' |
| `os/desktop/nibra-style/ui-enhancements.sh` | 1 | no 'set -euo pipefail' |
| `os/desktop/nibra-style/vookk-integration.sh` | 1 | no 'set -euo pipefail' |
| `os/software-gpu/runner/software-gpu-run` | 1 | no 'set -euo pipefail' |
| `os/tinker-cowork/tinker-cowork` | 1 | no 'set -euo pipefail' |
| `os/iso-builder-pro/parc-iso-pro` | 1 | no 'set -euo pipefail' |
| `os/hardware-cert/parc-hw-cert` | 1 | no 'set -euo pipefail' |
| `os/terminal-history/parc-term-history` | 1 | no 'set -euo pipefail' |
| `os/game-console/parc-game-console` | 1 | no 'set -euo pipefail' |
| `os/account/parc-id` | 1 | no 'set -euo pipefail' |
| `os/enterprise/parc-enterprise` | 1 | no 'set -euo pipefail' |

### N03 filename embeds a brand — 124

| file | line | detail |
|------|------|--------|
| `os/install-parcai.sh` | 0 | install-parcai.sh |
| `os/parcos-agent.sh` | 0 | parcos-agent.sh |
| `os/apps/apps/parc-ai.sh` | 0 | parc-ai.sh |
| `os/apps/apps/nibra-betterlife/KORRINOS.md` | 0 | tinkeros.md |
| `os/apps/apps/nibra-betterlife/KORRINOS.md` | 0 | tinkeros.md |
| `os/apps/apps/aether-workspace/KORRINOS.md` | 0 | tinkeros.md |
| `os/apps/apps/aether-workspace/KORRINOS.md` | 0 | tinkeros.md |
| `os/apps/customization/korrinos-typeface.sh` | 0 | korrinos-typeface.sh |
| `os/parc-ai/install-parcos.sh` | 0 | install-parcos.sh |
| `os/parc-ai/korrinos-annotate.sh` | 0 | korrinos-annotate.sh |
| `os/parc-ai/korrinos-ascii.sh` | 0 | korrinos-ascii.sh |
| `os/parc-ai/korrinos-backup.sh` | 0 | korrinos-backup.sh |
| `os/parc-ai/korrinos-cleanup.sh` | 0 | korrinos-cleanup.sh |
| `os/parc-ai/korrinos-clipctx.sh` | 0 | korrinos-clipctx.sh |
| `os/parc-ai/korrinos-dashboard.sh` | 0 | korrinos-dashboard.sh |
| `os/parc-ai/korrinos-devsuite.sh` | 0 | korrinos-devsuite.sh |
| `os/parc-ai/korrinos-dock.sh` | 0 | korrinos-dock.sh |
| `os/parc-ai/korrinos-focus.sh` | 0 | korrinos-focus.sh |
| `os/parc-ai/korrinos-hotkeys.sh` | 0 | korrinos-hotkeys.sh |
| `os/parc-ai/korrinos-liquid-glass.sh` | 0 | korrinos-liquid-glass.sh |
| `os/parc-ai/korrinos-monitor.sh` | 0 | korrinos-monitor.sh |
| `os/parc-ai/korrinos-network.sh` | 0 | korrinos-network.sh |
| `os/parc-ai/korrinos-nlctl.sh` | 0 | korrinos-nlctl.sh |
| `os/parc-ai/korrinos-notepad.sh` | 0 | korrinos-notepad.sh |
| `os/parc-ai/korrinos-power.sh` | 0 | korrinos-power.sh |
| `os/parc-ai/korrinos-schedule.sh` | 0 | korrinos-schedule.sh |
| `os/parc-ai/korrinos-security-apps.sh` | 0 | korrinos-security-apps.sh |
| `os/parc-ai/korrinos-shell-ai.sh` | 0 | korrinos-shell-ai.sh |
| `os/parc-ai/korrinos-shortcuts.sh` | 0 | korrinos-shortcuts.sh |
| `os/parc-ai/korrinos-smoothui.sh` | 0 | korrinos-smoothui.sh |
| `os/parc-ai/korrinos-sounds.sh` | 0 | korrinos-sounds.sh |
| `os/parc-ai/korrinos-splitscreen.sh` | 0 | korrinos-splitscreen.sh |
| `os/parc-ai/korrinos-tinkeria.sh` | 0 | korrinos-tinkeria.sh |
| `os/parc-ai/korrinos-tinkeria.sh` | 0 | korrinos-tinkeria.sh |
| `os/parc-ai/korrinos-toggles.sh` | 0 | korrinos-toggles.sh |
| `os/parc-ai/korrinos-voice.sh` | 0 | korrinos-voice.sh |
| `os/parc-ai/korrinos-widgets-panel.sh` | 0 | korrinos-widgets-panel.sh |
| `os/parc-ai/korrinos-widgets.sh` | 0 | korrinos-widgets.sh |
| `os/parc-ai/parc-ai-gui.py` | 0 | parc-ai-gui.py |
| `os/parc-ai/parc-ai.sh` | 0 | parc-ai.sh |
| `os/parc-ai/parcai-model-pipeline.sh` | 0 | parcai-model-pipeline.sh |
| `os/parc-ai/parcos-ai-installer.sh` | 0 | parcos-ai-installer.sh |
| `os/parc-ai/parcos-backup.sh` | 0 | parcos-backup.sh |
| `os/parc-ai/parcos-clipboard.sh` | 0 | parcos-clipboard.sh |
| `os/parc-ai/parcos-desktop.sh` | 0 | parcos-desktop.sh |
| `os/parc-ai/parcos-devtool.sh` | 0 | parcos-devtool.sh |
| `os/parc-ai/parcos-media.sh` | 0 | parcos-media.sh |
| `os/parc-ai/parcos-pm.sh` | 0 | parcos-pm.sh |
| `os/parc-ai/parcos-procmon.sh` | 0 | parcos-procmon.sh |
| `os/parc-ai/parcos-recovery.sh` | 0 | parcos-recovery.sh |
| `os/parc-ai/parcos-searchie.sh` | 0 | parcos-searchie.sh |
| `os/parc-ai/parcos-security.sh` | 0 | parcos-security.sh |
| `os/parc-ai/parcos-settings.sh` | 0 | parcos-settings.sh |
| `os/parc-ai/parcos-terminal.sh` | 0 | parcos-terminal.sh |
| `os/parc-ai/parcos-tools.sh` | 0 | parcos-tools.sh |
| `os/parc-ai/parcos-uninstall-blocker.sh` | 0 | parcos-uninstall-blocker.sh |
| `os/parc-ai/parcos-usb.sh` | 0 | parcos-usb.sh |
| `os/parc-ai/modules/knowledge-parcos.sh` | 0 | knowledge-parcos.sh |
| `os/parc-ai/modules/parcos-features.sh` | 0 | parcos-features.sh |
| `os/parc-ai/model/parc_inference.py` | 0 | parc_inference.py |
| `os/parc-ai/model/parc_model.py` | 0 | parc_model.py |
| `os/parc-ai/model/parc_tokenizer.py` | 0 | parc_tokenizer.py |
| `os/parc-ai/model/parc_trainer.py` | 0 | parc_trainer.py |
| `os/parc-ai/model/train_tinkerai_5b.py` | 0 | train_tinkerai_5b.py |
| `os/parc-ai/model/train_tinkeria_3b.py` | 0 | train_tinkeria_3b.py |
| `os/parc-ai/model/train_tinkeria_3b_cpu.py` | 0 | train_tinkeria_3b_cpu.py |
| `os/parc-ai/model/train_tinkeria_galore.py` | 0 | train_tinkeria_galore.py |
| `os/parc-ai/model/train_tinkeria_lora.py` | 0 | train_tinkeria_lora.py |
| `os/tinker-cowork/tinker-cowork` | 0 | tinker-cowork |
| `os/tinker-cowork/tinker-cowork.py` | 0 | tinker-cowork.py |
| `os/iso-builder-pro/parc-iso-pro` | 0 | parc-iso-pro |
| `os/iso-builder-pro/parc-iso-pro.py` | 0 | parc-iso-pro.py |
| `os/onboarding/parc-onboard.py` | 0 | parc-onboard.py |
| `os/vokk/vokk-ui.py` | 0 | vokk-ui.py |
| `os/vokk/vokk-v4-engine.py` | 0 | vokk-v4-engine.py |
| `os/vokk/vokk.py` | 0 | vokk.py |
| `os/vokk/vokk.sh` | 0 | vokk.sh |
| `os/vokk/vokk_controller.py` | 0 | vokk_controller.py |
| `os/vokk/vokk_search.py` | 0 | vokk_search.py |
| `os/system/korrinos-apt-hook.sh` | 0 | korrinos-apt-hook.sh |
| `os/system/korrinos-battery-bar.sh` | 0 | korrinos-battery-bar.sh |
| `os/system/korrinos-errors.sh` | 0 | korrinos-errors.sh |
| `os/system/korrinos-firstboot.sh` | 0 | korrinos-firstboot.sh |
| `os/system/korrinos-greetings.sh` | 0 | korrinos-greetings.sh |
| `os/system/korrinos-mascot.sh` | 0 | korrinos-mascot.sh |
| `os/system/parc-dust.sh` | 0 | parc-dust.sh |
| `os/system/parc-gamemode-hook.sh` | 0 | parc-gamemode-hook.sh |
| `os/system/parc-gamemode-launch.sh` | 0 | parc-gamemode-launch.sh |
| `os/system/driver-manager/korrinos-drivers.sh` | 0 | korrinos-drivers.sh |
| `os/system/backup/korrinos-backup.sh` | 0 | korrinos-backup.sh |
| `os/system/package-manager/korrinos-pkg.sh` | 0 | korrinos-pkg.sh |
| `os/system/appstore/korrinos-appstore.sh` | 0 | korrinos-appstore.sh |
| `os/system/monitor/korrinos-process.sh` | 0 | korrinos-process.sh |
| `os/system/security/korrinos-bugfix.sh` | 0 | korrinos-bugfix.sh |
| `os/system/security/korrinos-health.sh` | 0 | korrinos-health.sh |
| `os/system/cloud-sync/korrinos-cloud.sh` | 0 | korrinos-cloud.sh |
| `os/system/firewall/korrinos-firewall.sh` | 0 | korrinos-firewall.sh |
| `os/system/hardware-cert/korrinos-cert.sh` | 0 | korrinos-cert.sh |
| `os/system/desktop-env/korrinos-desktop.sh` | 0 | korrinos-desktop.sh |
| `os/system/desktop-env/korrinos-notify.sh` | 0 | korrinos-notify.sh |
| `os/system/update-system/korrinos-update.sh` | 0 | korrinos-update.sh |
| `os/system/installer/korrinos-installer.sh` | 0 | korrinos-installer.sh |
| `os/system/mobile-companion/korrinos-mobile.sh` | 0 | korrinos-mobile.sh |
| `os/system/network/korrinos-network.sh` | 0 | korrinos-network.sh |
| `os/system/enterprise/korrinos-enterprise.sh` | 0 | korrinos-enterprise.sh |
| `os/control-center/parc-control-center.py` | 0 | parc-control-center.py |
| `os/territories/vokk/vokk-lock.sh` | 0 | vokk-lock.sh |
| `os/territories/vokk/vokk-scheduler.sh` | 0 | vokk-scheduler.sh |
| `os/territories/vokk/vokk.sh` | 0 | vokk.sh |
| `os/hardware-cert/parc-hardware-cert.py` | 0 | parc-hardware-cert.py |
| `os/hardware-cert/parc-hw-cert` | 0 | parc-hw-cert |
| `os/marketplace/parc-market-gui.py` | 0 | parc-market-gui.py |
| `os/marketplace/parc-market.py` | 0 | parc-market.py |
| `os/terminal-history/parc-term-history` | 0 | parc-term-history |
| `os/terminal-history/parc-term-history.py` | 0 | parc-term-history.py |
| `os/game-console/parc-game-console` | 0 | parc-game-console |
| `os/game-console/parc-game-console.py` | 0 | parc-game-console.py |
| `os/account/parc-account.py` | 0 | parc-account.py |
| `os/account/parc-id` | 0 | parc-id |
| `os/mobile-companion/parc-mobile.py` | 0 | parc-mobile.py |
| `os/brand/parc-brand.py` | 0 | parc-brand.py |
| `os/enterprise/parc-enterprise` | 0 | parc-enterprise |
| `os/enterprise/parc-enterprise.py` | 0 | parc-enterprise.py |
| `os/bin/korrinos-launch` | 0 | korrinos-launch |

### C33 no usage/help text — 111

| file | line | detail |
|------|------|--------|
| `os/install-parcai.sh` | 1 | no --help/usage path |
| `os/apps/software-center.sh` | 1 | no --help/usage path |
| `os/apps/system/drag-to-install.sh` | 1 | no --help/usage path |
| `os/apps/system/intent-launcher.sh` | 1 | no --help/usage path |
| `os/apps/system/terminal-error-explainer.sh` | 1 | no --help/usage path |
| `os/scripts/integrate-gpu-features.sh` | 1 | no --help/usage path |
| `os/scripts/setup-gpu-stacks.sh` | 1 | no --help/usage path |
| `os/parc-ai/parcos-uninstall-blocker.sh` | 1 | no --help/usage path |
| `os/parc-ai/overlay/agent-narrator.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/agent-automation.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/agent-system.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/agent-vision.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-brain-smart.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-engine.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-image-gen.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-master-brain.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-narrative.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-nlu-crf.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-personality.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/ai-voice.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/cards-live.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/cards-structural.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/cards-visual.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/cards-workspace.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/codegen.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/commerce.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/contacts.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/conversation.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/entertainment.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/execution.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/finance.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/language.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/multimodal.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/nlp-670-patterns.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/nlp-training.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/nlu.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/persona.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/productivity.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/textgen.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/travel.sh` | 1 | no --help/usage path |
| `os/parc-ai/modules/web.sh` | 1 | no --help/usage path |
| `os/systemd/install-services.sh` | 1 | no --help/usage path |
| `os/security/biometric.sh` | 1 | no --help/usage path |
| `os/security/filevault.sh` | 1 | no --help/usage path |
| `os/security/findmydevice.sh` | 1 | no --help/usage path |
| `os/security/firewall.sh` | 1 | no --help/usage path |
| `os/security/gatekeeper.sh` | 1 | no --help/usage path |
| `os/security/privacy.sh` | 1 | no --help/usage path |
| `os/security/security.sh` | 1 | no --help/usage path |
| `os/security/sip.sh` | 1 | no --help/usage path |
| `os/vokk/vokk.sh` | 1 | no --help/usage path |
| `os/system/default-apps.sh` | 1 | no --help/usage path |
| `os/system/init.sh` | 1 | no --help/usage path |
| `os/system/install-greetings.sh` | 1 | no --help/usage path |
| `os/system/korrinos-firstboot.sh` | 1 | no --help/usage path |
| `os/system/security-suite.sh` | 1 | no --help/usage path |
| `os/control-center/launch_feature.sh` | 1 | no --help/usage path |
| `os/territories/world-optimizer.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/adapt.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/align.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/audit.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/auto.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/bloom.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/bulk.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/cms.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/dista.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/index.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/ir.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/lexin.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/lsh.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/markov.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/match.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/phoneme.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/prf.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/query.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/rank.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/sarray.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/selftest.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/store.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/core/time.sh` | 1 | no --help/usage path |
| `os/territories/vibe-address/connectors/bootstrap.sh` | 1 | no --help/usage path |
| `os/territories/game/gpu-lock.sh` | 1 | no --help/usage path |
| `os/territories/game/gpu-pipeline.sh` | 1 | no --help/usage path |
| `os/territories/game/three-worlds-expand.sh` | 1 | no --help/usage path |
| `os/territories/vokk/vokk-lock.sh` | 1 | no --help/usage path |
| `os/territories/vokk/vokk-scheduler.sh` | 1 | no --help/usage path |
| `os/territories/vokk/vokk.sh` | 1 | no --help/usage path |
| `os/territories/lib/common.sh` | 1 | no --help/usage path |
| `os/territories/daily-driver/enhanced.sh` | 1 | no --help/usage path |
| `os/territories/ai-scheduler/native-scheduler.sh` | 1 | no --help/usage path |
| `os/territories/ai-scheduler/prioritize.sh` | 1 | no --help/usage path |
| `os/hardware-tech/lib/hardware-consent.sh` | 1 | no --help/usage path |
| `os/hardware-tech/hardware-tuning/14-categories.sh` | 1 | no --help/usage path |
| `os/hardware-tech/hardware-tuning/gpu-tuning.sh` | 1 | no --help/usage path |
| `os/desktop/help-system.sh` | 1 | no --help/usage path |
| `os/desktop/settings-gui.sh` | 1 | no --help/usage path |
| `os/desktop/setup-wizard.sh` | 1 | no --help/usage path |
| `os/desktop/start-desktop.sh` | 1 | no --help/usage path |
| `os/desktop/system-tray.sh` | 1 | no --help/usage path |
| `os/desktop/nibra-style/ui-enhancements.sh` | 1 | no --help/usage path |
| `os/desktop/nibra-style/vookk-integration.sh` | 1 | no --help/usage path |
| `os/software-gpu/runner/software-gpu-run` | 1 | no --help/usage path |
| `os/tinker-cowork/tinker-cowork` | 1 | no --help/usage path |
| `os/iso-builder-pro/parc-iso-pro` | 1 | no --help/usage path |
| `os/hardware-cert/parc-hw-cert` | 1 | no --help/usage path |
| `os/terminal-history/parc-term-history` | 1 | no --help/usage path |
| `os/game-console/parc-game-console` | 1 | no --help/usage path |
| `os/enterprise/parc-enterprise` | 1 | no --help/usage path |
| `os/bin/korrinos-launch` | 1 | no --help/usage path |

### C06 TODO/FIXME/HACK left in code — 86

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 5 | HACK/GAME/NORMAL) baked in. |
| `os/parc-ai/korrinos-devsuite.sh` | 270 | TODO\\|FIXME\\|HACK\\|XXX" "$dir" 2>/dev/null \| |
| `os/parc-ai/parcos-devtool.sh` | 287 | TODO\\|FIXME\\|HACK\\|XXX" "$file" 2>/dev/null  |
| `os/parc-ai/modules/cards-live.sh` | 210 | TODO</div> |
| `os/parc-ai/modules/cards-structural.sh` | 177 | TODO (3)</div> |
| `os/parc-ai/modules/debugging.sh` | 197 | TODO: implement |
| `os/parc-ai/modules/execution.sh` | 237 | XXX \| |
| `os/parc-ai/modules/execution.sh` | 238 | XXX \| |
| `os/parc-ai/modules/execution.sh` | 239 | XXX \| |
| `os/parc-ai/modules/execution.sh` | 240 | XXX** \| |
| `os/parc-ai/modules/knowledge-parcos.sh` | 18 | HACK, NORMAL, GAME — switch with Space+Shift |
| `os/parc-ai/modules/knowledge-parcos.sh` | 21 | HACK + 13 GAME + 14 SECURE) |
| `os/parc-ai/modules/knowledge-parcos.sh` | 34 | HACK (Space+Shift+1 / Ctrl+Left): |
| `os/parc-ai/modules/knowledge-parcos.sh` | 95 | HACK (18): amnesia-firewall, app-guard, brows |
| `os/parc-ai/modules/knowledge-parcos.sh` | 132 | HACK world |
| `os/parc-ai/modules/knowledge-parcos.sh` | 137 | HACK world |
| `os/parc-ai/modules/knowledge-parcos.sh` | 201 | HACK \| NORMAL \| GAME |
| `os/parc-ai/modules/knowledge-parcos.sh` | 309 | HACK for security, NORMAL for everyday use, G |
| `os/parc-ai/modules/knowledge-parcos.sh` | 321 | HACK world only allows Tor Browser and safe a |
| `os/parc-ai/modules/productivity.sh` | 8 | TODO LIST --- |
| `os/parc-ai/modules/productivity.sh` | 21 | TODO List ===" |
| `os/territories/arsenal.sh` | 4 | HACK   -> offensive (black-hat) + defensive (wh |
| `os/territories/arsenal.sh` | 27 | HACK WORLD: offensive + defensive toolset --- |
| `os/territories/arsenal.sh` | 158 | HACK tools are for authorized testing only."  |
| `os/territories/install-territories.sh` | 31 | HACK\|GAME\|NORMAL" |
| `os/territories/modes.sh` | 21 | HACK ;; |
| `os/territories/modes.sh` | 33 | HACK)   "$TERR/hack/hack-mode.sh" enter ;; |
| `os/territories/modes.sh` | 107 | HACK / NORMAL(secure) / GAME terrains." ;; |
| `os/territories/world-engine.sh` | 3 | HACK, GAME. |
| `os/territories/world-engine.sh` | 8 | HACK   (Space+Shift+1,  Ctrl+Arrow-Left) |
| `os/territories/world-engine.sh` | 31 | HACK GAME) |
| `os/territories/world-engine.sh` | 87 | HACK\|GAME) : ;; |
| `os/territories/world-engine.sh` | 88 | HACK\|GAME)"; return 1 ;; |
| `os/territories/world-engine.sh` | 150 | HACK (left), 2 => NORMAL (center), 3 => GAME  |
| `os/territories/world-engine.sh` | 153 | HACK   ;; |
| `os/territories/world-engine.sh` | 197 | HACK\|GAME>   switch worlds |
| `os/territories/world-engine.sh` | 212 | HACK) echo korr-hack;; GAME) echo korr-game; |
| `os/territories/world-engine.sh` | 213 | HACK) echo 10.20.0.3;; GAME) echo 10.20.0.4; |
| `os/territories/world-engine.sh` | 226 | HACK; isolate_world GAME |
| `os/territories/world-optimizer.sh` | 6 | HACK          : defense-first — hardened with low-heat |
| `os/territories/world-optimizer.sh` | 35 | HACK: defense-first tuning (low footprint, har |
| `os/territories/world-optimizer.sh` | 61 | HACK\|hack) opt_hack ;; |
| `os/territories/game/three-worlds-expand.sh` | 3 | HACK) with additional specialized worlds and |
| `os/territories/hack/app-guard.sh` | 2 | HACK world rule: ONLY safe apps YOU approve r |
| `os/territories/hack/app-guard.sh` | 4 | HACK. |
| `os/territories/hack/app-guard.sh` | 14 | HACK/approved-apps" |
| `os/territories/hack/app-guard.sh` | 15 | HACK/guard-bin" |
| `os/territories/hack/app-guard.sh` | 22 | HACK world." |
| `os/territories/hack/app-guard.sh` | 54 | HACK world: \x27%s\x27 is NOT an approved saf |
| `os/territories/hack/app-guard.sh` | 59 | HACK world ($covered binaries shimmed)." |
| `os/territories/hack/app-guard.sh` | 63 | HACK world..." |
| `os/territories/hack/app-guard.sh` | 70 | HACK world:" |
| `os/territories/hack/app-guard.sh` | 81 | HACK world rule: only safe apps you approve m |
| `os/territories/hack/browser-gate.sh` | 2 | HACK world rule: ONLY Tor Browser works here. |
| `os/territories/hack/browser-gate.sh` | 4 | HACK world, leaving tor-browser / -torbrowser |
| `os/territories/hack/browser-gate.sh` | 8 | HACK world and is |
| `os/territories/hack/browser-gate.sh` | 14 | HACK/guard-bin" |
| `os/territories/hack/browser-gate.sh` | 26 | HACK world..." |
| `os/territories/hack/browser-gate.sh` | 32 | HACK world: only TOR BROWSER is permitted. (% |
| `os/territories/hack/browser-gate.sh` | 41 | HACK world)..." |
| `os/territories/hack/browser-gate.sh` | 56 | HACK world rule: only Tor Browser is permitte |
| `os/territories/hack/canary-honeypot.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/ephemeral-ram.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/gpu-pipeline.sh` | 2 | HACK territory) |
| `os/territories/hack/hack-defense.sh` | 3 | HACK world. This is the defensive side of |
| `os/territories/hack/hack-gate.sh` | 2 | HACK territory) |
| `os/territories/hack/hack-mode.sh` | 3 | HACK world. Wires together: |
| `os/territories/hack/hack-mode.sh` | 10 | HACK (left)     Ctrl+Arrow-Left |
| `os/territories/hack/hack-mode.sh` | 36 | HACK MODE — entering the offensive workspace" |
| `os/territories/hack/hack-mode.sh` | 41 | HACK |
| `os/territories/hack/hack-mode.sh` | 53 | HACK world rule: the ONLY browser that works  |
| `os/territories/hack/hack-mode.sh` | 57 | HACK world rule: ONLY safe apps you explicitl |
| `os/territories/hack/hack-mode.sh` | 65 | HACK WORLD READY." |
| `os/territories/hack/hack-mode.sh` | 71 | HACK world -> back to NORMAL." |
| `os/territories/hack/hack-ways.sh` | 2 | HACK territory) |
| `os/territories/hack/hack-ways.sh` | 74 | HACK WAYS library ($((${#WAYS[@]})) technique |
| `os/territories/hack/intent-hardware.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/kill-switch.sh` | 2 | HACK territory) |
| `os/territories/hack/masked-proc.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/panic-wipe.sh` | 2 | HACK territory) |
| `os/territories/hack/reverse-proxy.sh` | 2 | HACK territory) |
| `os/territories/hack/sdr-isolation.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/sdr-isolation.sh` | 10 | HACK: bridge a Software-Defined Radio (RTL-SDR |
| `os/territories/hack/supply-chain.sh` | 2 | HACK + SECURE territory) |
| `os/territories/hack/threat-monitor.sh` | 2 | HACK territory) |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 22 | XXX/power/control"; } |

### C102 mixed tabs/spaces — 64

| file | line | detail |
|------|------|--------|
| `kernel/tinker/adaptive_display.c` | 21 | #define AD_BUFSZ	64 |
| `kernel/tinker/adaptive_display.c` | 24 | static unsigned int ad_mode;	/* 0 auto, 1 stationa |
| `kernel/tinker/battery_life.c` | 22 | #define BATTERY_BUFSZ	64 |
| `kernel/tinker/battery_life.c` | 23 | #define CHARGE_MIN_DEFAULT	20	/* % */ |
| `kernel/tinker/battery_life.c` | 24 | #define CHARGE_MAX_DEFAULT	80	/* % */ |
| `kernel/tinker/cache_tiering.c` | 25 | #define CACHE_BUFSZ	64 |
| `kernel/tinker/coil_whine.c` | 25 | #define CW_BUFSZ	64 |
| `kernel/tinker/cxl_memory.c` | 22 | #define CXL_BUFSZ	64 |
| `kernel/tinker/cxl_memory.c` | 24 | #define POOL_DRAM	0 |
| `kernel/tinker/cxl_memory.c` | 25 | #define POOL_VRAM	1 |
| `kernel/tinker/cxl_memory.c` | 26 | #define POOL_CXL	2 |
| `kernel/tinker/cxl_memory.c` | 27 | #define POOL_NET	3 |
| `kernel/tinker/data_shredder.c` | 25 | #define SHRED_BUFSZ	64 |
| `kernel/tinker/dust_dislodger.c` | 31 | #define DUST_BUFSZ	64 |
| `kernel/tinker/dust_dislodger.c` | 32 | #define DUST_STALL_MARGIN	10	/* pct added above th |
| `kernel/tinker/dust_dislodger.c` | 33 | #define DUST_MIN_PWM		20	/* calibration duty floor |
| `kernel/tinker/dust_dislodger.c` | 34 | #define DUST_RESONANCE_DIV	3	/* pulse ceiling = (m |
| `kernel/tinker/dust_dislodger.c` | 35 | #define DUST_SANE_MAX_RPM	30000	/* impossible-abov |
| `kernel/tinker/dust_dislodger.c` | 36 | #define DUST_SANE_MIN_RPM	100	/* impossible-below  |
| `kernel/tinker/dust_dislodger.c` | 50 | static unsigned int dust_decl_max_rpm;		/* rated/m |
| `kernel/tinker/dust_dislodger.c` | 51 | static unsigned int dust_measured_rpm;		/* last me |
| `kernel/tinker/dust_dislodger.c` | 52 | static unsigned int dust_measured_duty;		/* duty o |
| `kernel/tinker/dust_dislodger.c` | 53 | static unsigned int dust_safe_min_duty;		/* stall  |
| `kernel/tinker/dust_dislodger.c` | 54 | static unsigned int dust_rpm_per_pct;		/* calculat |
| `kernel/tinker/dust_dislodger.c` | 55 | static unsigned int dust_safe_max_hz;		/* resonanc |
| `kernel/tinker/dust_dislodger.c` | 56 | static unsigned int dust_safe_amp_max_pct;	/* trou |
| `kernel/tinker/dvfs_shaver.c` | 24 | #define DVFS_BUFSZ	64 |
| `kernel/tinker/dvfs_shaver.c` | 25 | #define DVFS_STATE_AUTO	0 |
| `kernel/tinker/dvfs_shaver.c` | 26 | #define DVFS_STATE_PEAK	1 |
| `kernel/tinker/dvfs_shaver.c` | 27 | #define DVFS_STATE_MIN	2 |
| `kernel/tinker/energy_sched.c` | 24 | #define ENERGY_BUFSZ		64 |
| `kernel/tinker/energy_sched.c` | 25 | #define ENERGY_MODE_AUTO	0 |
| `kernel/tinker/energy_sched.c` | 26 | #define ENERGY_MODE_PEAK	1 |
| `kernel/tinker/energy_sched.c` | 27 | #define ENERGY_MODE_SAVER	2 |
| `kernel/tinker/finance_audit.c` | 22 | #define FA_BUFSZ	64 |
| `kernel/tinker/finance_audit.c` | 23 | #define FA_MAX_APPS	32 |
| `kernel/tinker/fpga_scaler.c` | 23 | #define FPGA_BUFSZ	64 |
| `kernel/tinker/fpga_scaler.c` | 26 | static unsigned int fpga_precision_bits = 32;	/* d |
| `kernel/tinker/gamemode.c` | 25 | #define GAMEMODE_MAX_PRIO		10 |
| `kernel/tinker/gamemode.c` | 26 | #define GAMEMODE_BUFSZ			64 |
| `kernel/tinker/hardware_tuning.c` | 20 | #define TUNE_BUFSZ	64 |
| `kernel/tinker/hardware_tuning.c` | 23 | static unsigned int tune_governor;	/* 0 auto, 1 pe |
| `kernel/tinker/hardware_tuning.c` | 25 | static unsigned int tune_fan_curve;	/* 0-100 aggre |
| `kernel/tinker/neural_audio.c` | 21 | #define NA_BUFSZ	64 |
| `kernel/tinker/neural_super_res.c` | 20 | #define NSR_BUFSZ	64 |
| `kernel/tinker/neural_super_res.c` | 24 | static unsigned int nsr_scale = 2;	/* 1x, 2x, 4x * |
| `kernel/tinker/oled_wear.c` | 25 | #define OLED_BUFSZ	64 |
| `kernel/tinker/oled_wear.c` | 28 | static unsigned int oled_dim_pct = 100;	/* 100 = n |
| `kernel/tinker/predictive_prewarm.c` | 22 | #define PREWARM_BUFSZ	64 |
| `kernel/tinker/predictive_render.c` | 20 | #define PR_BUFSZ	64 |
| `kernel/tinker/ray_traced_audio.c` | 25 | #define RTA_BUFSZ	128 |
| `kernel/tinker/remote_hardware_api.c` | 21 | #define RHA_BUFSZ	64 |
| `kernel/tinker/sdgpu.c` | 21 | #define SDGPU_BUFSZ	64 |
| `kernel/tinker/sdgpu.c` | 25 | static unsigned int sdgpu_compute_share = 100;	/*  |
| `kernel/tinker/smart_power_grid.c` | 22 | #define PG_BUFSZ	64 |
| `kernel/tinker/smart_power_grid.c` | 23 | #define PG_BALANCED	0 |
| `kernel/tinker/smart_power_grid.c` | 24 | #define PG_PERF		1 |
| `kernel/tinker/smart_power_grid.c` | 25 | #define PG_SUSTAINED	2 |
| `kernel/tinker/smart_power_grid.c` | 26 | #define PG_LONGEVITY	3 |
| `kernel/tinker/thermal_sched.c` | 30 | #define THERMAL_MAX_ZONES	8 |
| `kernel/tinker/thermal_sched.c` | 31 | #define THERMAL_BUF_SZ		32 |
| `kernel/tinker/thermal_sched.c` | 44 | static DEFINE_MUTEX(thermal_lock);	/* guards seq_f |
| `kernel/tinker/unified_memory.c` | 20 | #define UM_BUFSZ	64 |
| `kernel/tinker/zero_latency_input.c` | 22 | #define ZL_BUFSZ	64 |

### C111 no MODULE_LICENSE — 49

| file | line | detail |
|------|------|--------|
| `os/hardware-tech/lib/device-weights.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/audio_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/battery_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/cat_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/display_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/fan_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/fpga_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/msr_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/thermal_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/hardware-tech/backend/src/usb_control.c` | 1 | kernel module lacks MODULE_LICENSE |
| `os/languages/demo/hello.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/adaptive_display.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/app_store.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/battery_life.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/cache_tiering.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/cloud_sync.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/coil_whine.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/cxl_memory.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/data_shredder.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/desktop_state.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/driver_monitor.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/dust_dislodger.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/dvfs_shaver.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/energy_sched.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/enterprise_state.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/finance_audit.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/fpga_scaler.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/gamemode.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/hardware_dna.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/hardware_tuning.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/hw_cert.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/hyperdrive.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/installer_state.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/mobile_companion.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/neural_audio.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/neural_super_res.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/oled_wear.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/pkg_tracker.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/predictive_prewarm.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/predictive_render.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/ray_traced_audio.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/remote_hardware_api.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/sdgpu.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/smart_power_grid.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/thermal_sched.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/tinker.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/unified_memory.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/update_monitor.c` | 1 | kernel module lacks MODULE_LICENSE |
| `kernel/tinker/zero_latency_input.c` | 1 | kernel module lacks MODULE_LICENSE |

### C04 trailing whitespace — 40

| file | line | detail |
|------|------|--------|
| `os/apps/system/parental-controls.sh` | 95 | 'limit\|time) ' |
| `os/apps/system/predictive-caching.sh` | 124 | ' == hour) & ' |
| `os/apps/system/predictive-intelligence.sh` | 75 | 'eader=None, ' |
| `os/apps/system/screen-time.sh` | 83 | '    limit) ' |
| `os/apps/system/system-monitor.sh` | 49 | 'ble: %s\\n", ' |
| `os/apps/system/temporal-mapping.sh` | 209 | 'me="Today"; ' |
| `os/apps/network/speed-test.sh` | 21 | "{print $1}' " |
| `os/parc-ai/modules/nlu.sh` | 16 | 'd patterns  ' |
| `os/vokk/agent/system-agent.sh` | 167 | ' "status"], ' |
| `os/system/password-monitor.sh` | 349 | 'SelectorAll ' |
| `os/system/password-monitor.sh` | 386 | "etected' \|\| " |
| `os/system/temporal-resource-mapping.sh` | 50 | 'ory = $mem  ' |
| `os/territories/self-watchdog.sh` | 46 | 'AD THIS NOW ' |
| `os/territories/secure/key-wallet.sh` | 23 | 'or-stdin:-> ' |
| `os/territories/lib/common.sh` | 41 | 'ev/null ) & ' |
| `os/territories/hack/gpu-pipeline.sh` | 20 | 'via vendor" ' |
| `os/territories/hack/gpu-pipeline.sh` | 61 | 'nel -o out" ' |
| `os/hardware-tech/remote-hardware-api/remote-api.sh` | 5 | 'init(){ ' |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 107 | 'title_font, ' |
| `os/hardware-tech/coil-whine-killer/coil-whine-killer.sh` | 268 | '"i2c-",""), ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 190 | 'bscriptions ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 198 | 'ransactions ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 252 | 'bscriptions ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 292 | 'l_sessions) ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 349 | ', severity) ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 359 | 'criptions s ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 367 | ', severity) ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 373 | 'criptions s ' |
| `os/hardware-tech/finance-audit/subscription-audit.sh` | 386 | ', severity) ' |
| `os/hardware-tech/oled-shield/oled-shield.sh` | 201 | 'ndowname"], ' |
| `os/hardware-tech/zero-latency-input/zero-latency-input.sh` | 17 | 'entirely)"; ' |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 125 | 'it.com/r/", ' |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 110 | 'free,name", ' |
| `os/hardware-tech/cxl-memory/cxl-memory.sh` | 111 | ',nounits"], ' |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 152 | ' "getpid"], ' |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 297 | 'TOP", pid], ' |
| `os/hardware-tech/thermal-scheduler/thermal-scheduler.sh` | 454 | 'ask), pid], ' |
| `os/hardware-tech/dvfs-shaver/dvfs-shaver.sh` | 355 | '"min_mv"] + ' |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 289 | ' pwm_file], ' |
| `os/hardware-tech/dust-dislodger/dust-dislodger.sh` | 290 | ').encode(), ' |

### C16 'which' instead of 'command -v' — 39

| file | line | detail |
|------|------|--------|
| `os/apps/gaming/audio-mixer.sh` | 118 | local which=${1:-output} |
| `os/apps/gaming/audio-mixer.sh` | 123 | case $which in |
| `os/apps/gaming/audio-mixer.sh` | 129 | case $which in |
| `os/apps/gaming/audio-mixer.sh` | 142 | echo "Toggled $which mute" |
| `os/apps/customization/korrinos-typeface.sh` | 137 | Italic face, which is far too strong for body text. |
| `os/apps/system/global-search.sh` | 38 | which *$query* 2>/dev/null \| head -10 |
| `os/parc-ai/korrinos-schedule.sh` | 121 | # which raised TypeError on None, and because stderr was dis |
| `os/parc-ai/korrinos-toggles.sh` | 70 | # power draw in watts, which was computed above but never us |
| `os/parc-ai/modules/ai-brain-smart.sh` | 327 | echo "What do you want me to translate, and to which languag |
| `os/parc-ai/modules/ai-brain-smart.sh` | 330 | echo "What do you want me to translate, and to which languag |
| `os/parc-ai/modules/ai-knowledge-broad.sh` | 38 | {"q": "what is photosynthesis", "a": "Photosynthesis is the  |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 37 | {"q":"what is the higgs boson","a":"The Higgs boson is a par |
| `os/parc-ai/modules/ai-knowledge-mega.sh` | 140 | {"q":"what is a compiler","a":"A compiler translates entire  |
| `os/parc-ai/modules/ai-master-brain.sh` | 196 | echo "What do you want me to translate, and to which languag |
| `os/parc-ai/modules/nlu.sh` | 15 | if [[ "$text" =~ ^(what\|who\|where\|when\|why\|how\|which\|can you |
| `os/parc-ai/modules/textgen.sh` | 453 | f'Nor loses the grip by which it is brimm\'d.', |
| `os/security/gatekeeper.sh` | 119 | echo "  Path: $(which "$app" 2>/dev/null \|\| echo "$app")" |
| `os/system/audio-clarity.sh` | 91 | # This is done by boosting odd harmonics which naturally occ |
| `os/system/korrinos-errors.sh` | 240 | # which EXECUTED the resolved path as shell code. A PATH ent |
| `os/system/security/korrinos-bugfix.sh` | 90 | sed -i 's/\$(which \([^)]*\))/$(which \1 2>/dev/null)/g' "$s |
| `os/system/security/korrinos-bugfix.sh` | 233 | # Already using python3 json.dump which is atomic |
| `os/territories/vibe-address/vibe-address.sh` | 15 | #    core/adapt.sh   feedback loop: learns which signals mat |
| `os/territories/vibe-address/core/action.sh` | 168 | echo "ALTERNATIVES\|review below — choose which to delete, or |
| `os/territories/vibe-address/core/adapt.sh` | 6 | #  retrieval parameters based on *which signals actually suc |
| `os/territories/vibe-address/core/bloom.sh` | 9 | #  which makes thousands of updates cheap in bash while keep |
| `os/territories/vibe-address/core/ir.sh` | 17 | #  times earns far less than five distinct words — which is  |
| `os/territories/vibe-address/core/lexin.sh` | 35 | VE_LEX_STOPWORDS="the a an that this those these of in on at |
| `os/territories/vibe-address/core/lexin.sh` | 93 | echo "$input" \| tr ' ' '\n' \| grep -vE "^(the\|a\|an\|that\|this |
| `os/territories/vibe-address/core/query.sh` | 9 | #    2. INTENT CLASSIFIER determine which matchers to activa |
| `os/territories/vibe-address/core/query.sh` | 16 | #    9. ADAPT            record which signals succeeded |
| `os/territories/vibe-address/core/query.sh` | 26 | # this helper instead, which normalises space+newline runs t |
| `os/territories/vibe-address/core/query.sh` | 367 | # ---- tier explanation: which relax mechanism produced the  |
| `os/territories/vibe-address/core/query.sh` | 428 | # find which day buckets overlap the window |
| `os/territories/vibe-address/core/rank.sh` | 10 | #    adaptive weights shift based on which signals succeeded |
| `os/territories/vibe-address/core/tree.sh` | 5 | #  The category index is an actual on-disk trie (directories |
| `os/territories/game/anticheat-consent.sh` | 4 | # access (kernel drivers, memory reads) which are privacy/in |
| `os/hardware-tech/energy-scheduler/energy-scheduler.sh` | 313 | echo "║  Calculating: which tasks deserve your battery       |
| `os/languages/install.sh` | 38 | echo "  Korlang installed: $(which korlang)" |
| `os/languages/install.sh` | 52 | echo "  KorrinUILang installed: $(which korrinuilang)" |

### C107 C++ style // comment — 37

| file | line | detail |
|------|------|--------|
| `kernel/tinker/adaptive_display.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/app_store.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/battery_life.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/cache_tiering.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/cloud_sync.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/coil_whine.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/cxl_memory.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/data_shredder.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/desktop_state.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/driver_monitor.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/dust_dislodger.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/dvfs_shaver.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/energy_sched.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/enterprise_state.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/finance_audit.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/fpga_scaler.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/gamemode.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/hardware_dna.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/hardware_tuning.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/hw_cert.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/installer_state.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/mobile_companion.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/neural_audio.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/neural_super_res.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/oled_wear.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/pkg_tracker.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/predictive_prewarm.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/predictive_render.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/ray_traced_audio.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/remote_hardware_api.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/sdgpu.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/smart_power_grid.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/thermal_sched.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/tinker.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/unified_memory.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/update_monitor.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |
| `kernel/tinker/zero_latency_input.c` | 1 | // SPDX-License-Identifier: GPL-2.0 |

### C09 rm -rf on a variable — 36

| file | line | detail |
|------|------|--------|
| `os/build-distro.sh` | 42 | "$SUDO" rm -rf "$ROOTFS" |
| `os/build-distro.sh` | 361 | "$SUDO" rm -rf "$ROOTFS/opt/korrinos" |
| `os/build-distro.sh` | 637 | "$SUDO" rm -rf "$IMAGE" |
| `os/iso-builder.sh` | 76 | rm -rf "$BUILD_DIR" |
| `os/iso-builder.sh` | 198 | rm -rf "$mdir" |
| `os/iso-builder.sh` | 281 | rm -rf "$BUILD_DIR" |
| `os/apps/file-manager.sh` | 157 | [ "$confirm" = "y" ] && rm -rf "$file" && echo "Deleted" |
| `os/apps/ocr-everywhere.sh` | 141 | rm -rf $tmpdir |
| `os/apps/customization/korrinos-typeface.sh` | 263 | if [ -w "$d" ]; then rm -rf "$d"; else sudo -n rm -rf "$d" 2>/dev/null |
| `os/apps/system/predictive-caching.sh` | 204 | rm -rf "$CACHE_DIR"/* |
| `os/apps/system/self-healing.sh` | 208 | rm -rf "$HOME/.local/share/Trash"/* 2>/dev/null |
| `os/parc-ai/korrinos-backup.sh` | 100 | rm -rf "$backup_path" |
| `os/parc-ai/korrinos-backup.sh` | 129 | rm -rf "$backup_base/$old" 2>/dev/null |
| `os/parc-ai/korrinos-backup.sh` | 273 | rm -rf "$backup_base/$backup_name" "$backup_base/${backup_name}.tar.gz |
| `os/parc-ai/parc-ai.sh` | 820 | rm -rf "$dir" |
| `os/system/backup-restore.sh` | 180 | rm -rf "$BACKUP_DIR/$name" |
| `os/system/digital-twin.sh` | 172 | rm -rf "$replica_dir" |
| `os/system/rollback-recovery.sh` | 110 | rm -rf "$SNAPSHOTS_DIR/$name" |
| `os/system/driver-manager/korrinos-drivers.sh` | 467 | cd / && rm -rf "$tmpdir" |
| `os/system/driver-manager/korrinos-drivers.sh` | 477 | cd / && rm -rf "$tmpdir" |
| `os/system/appstore/korrinos-appstore.sh` | 146 | rm -rf "$tmpdir" |
| `os/system/appstore/korrinos-appstore.sh` | 440 | rm -rf "$tmpdir" |
| `os/system/appstore/korrinos-appstore.sh` | 443 | rm -rf "$tmpdir" |
| `os/system/appstore/korrinos-appstore.sh` | 496 | rm -rf "$appdir" |
| `os/system/appstore/korrinos-appstore.sh` | 761 | rm -rf "$tmpdir" |
| `os/territories/secure/app-allowlist.sh` | 52 | rm -rf "$DENYDIR/bin"; mkdir -p "$DENYDIR/bin" |
| `os/territories/vibe-address/vibe-address.sh` | 324 | rm -rf "$sdir" |
| `os/territories/vibe-address/core/action.sh` | 120 | rm -rf "$rpath2" && echo "  DELETED  $rpath2" \|\| echo "  FAILED   $rpa |
| `os/territories/vibe-address/core/align.sh` | 175 | rm -rf "$VIBE_STATE/markov" "$(ve_lsh_dir)" 2>/dev/null \|\| true |
| `os/territories/vibe-address/core/bulk.sh` | 172 | rm -rf "$tmp" |
| `os/territories/vibe-address/core/index.sh` | 138 | rm -rf "$inv" "$fpd" "$VIBE_INDEX/time"; mkdir -p "$inv" "$fpd" |
| `os/territories/vibe-address/core/selftest.sh` | 106 | rm -rf "$(ve_markov_dir)" && ve_markov_dir >/dev/null 2>&1 |
| `os/territories/vibe-address/core/selftest.sh` | 118 | rm -rf "$VIBE_STATE/lsh"; mkdir -p "$VIBE_STATE/lsh" |
| `os/territories/hack/supply-chain.sh` | 36 | rm -rf "$BUILDROOT/out"; mkdir -p "$BUILDROOT/out" |
| `os/brand/install-boot-intro.sh` | 47 | rm -rf "$THEME_DIR" |
| `os/brand/install-grub-theme.sh` | 40 | rm -rf "$THEME_DST" |

### C18 for-loop over $(...) (word split) — 33

| file | line | detail |
|------|------|--------|
| `os/apps/security/gatekeeper.sh` | 43 | for dir in $(echo "$PATH" \| tr ':' ' '); do |
| `os/apps/security/gatekeeper.sh` | 56 | for loc in $(echo "$untrusted" \| tr ',' ' '); do |
| `os/apps/system/power-manager.sh` | 145 | for iface in $(iw dev \| grep Interface \| awk '{print $2}');  |
| `os/apps/system/screen-time.sh` | 33 | for i in $(seq 6 -1 0); do |
| `os/apps/system/time-tracker.sh` | 53 | for i in $(seq 6 -1 0); do |
| `os/parc-ai/modules/parcos-features.sh` | 202 | for i in $(seq 1 "$duration"); do |
| `os/parc-ai/modules/productivity.sh` | 88 | for i in $(seq 0 6); do |
| `os/system/hardware-detect.sh` | 163 | for layer in $(echo "$layers" \| tr ',' ' '); do |
| `os/system/password-manager.sh` | 237 | for i in $(seq 1 $word_count); do |
| `os/system/predictive-caching.sh` | 65 | for hour in $(seq 0 23); do |
| `os/system/temporal-mapping.sh` | 64 | for hour in $(seq 0 23); do |
| `os/system/temporal-mapping.sh` | 74 | for day in $(seq 1 7); do |
| `os/system/hardware-cert/korrinos-cert.sh` | 180 | for i in $(seq 1 1000000); do echo $i > /dev/null; done 2>/d |
| `os/system/hardware-cert/korrinos-cert.sh` | 519 | for i in $(seq 1 30); do |
| `os/system/desktop-env/korrinos-desktop.sh` | 1215 | for i in $(seq 0 $((count - 1))); do |
| `os/system/mobile-companion/korrinos-mobile.sh` | 166 | for i in $(seq 1 254); do |
| `os/territories/vibe-address/core/bloom.sh` | 87 | for pos in $(ve_bloom_positions "$key"); do |
| `os/territories/vibe-address/core/bloom.sh` | 106 | for pos in $(ve_bloom_positions "$key"); do |
| `os/territories/vibe-address/core/cms.sh` | 28 | for i in $(seq 1 $((VIBE_CMS_D * 4))); do |
| `os/territories/vibe-address/core/cms.sh` | 46 | for i in $(seq 1 "$VIBE_CMS_D"); do echo "$rowzero"; done >  |
| `os/territories/vibe-address/core/cms.sh` | 57 | for i in $(seq 1 "$VIBE_CMS_D"); do |
| `os/territories/vibe-address/core/lsh.sh` | 57 | for j in $(seq 1 "$VE_LSH_ROWS"); do |
| `os/territories/vibe-address/core/lsh.sh` | 62 | for j in $(seq 1 "$VE_LSH_ROWS"); do |
| `os/territories/vibe-address/core/lsh.sh` | 74 | for j in $(seq 1 "$VE_LSH_BANDS"); do |
| `os/territories/vibe-address/core/lsh.sh` | 91 | for j in $(seq 1 "$VE_LSH_BANDS"); do |
| `os/territories/vibe-address/core/lsh.sh` | 103 | for i in $(seq 1 "$VE_LSH_ROWS"); do |
| `os/territories/vibe-address/core/sarray.sh` | 31 | for fp in $(echo "$fplist" \| tail -"$keep"); do |
| `os/territories/vibe-address/core/tree.sh` | 24 | for seg in $(tr ':/:.' '   ' <<<"${raw,,}"); do |
| `os/territories/game/audio-focus.sh` | 40 | for s in $(pactl list short sources 2>/dev/null \| awk '/moni |
| `os/territories/hack/intent-hardware.sh` | 59 | for s in $(find_audio_in); do pactl set-source-mute "$s" 1 2 |
| `os/territories/hack/kill-switch.sh` | 75 | for iface in $(list_ifaces); do |
| `os/hardware-tech/data-shredder/data-shredder.sh` | 61 | for iface in $(ip -o link show \| awk -F': ' '{print $2}' \| g |
| `os/hardware-tech/hardware-tuning/cpu-tuning.sh` | 14 | cores) for i in $(seq ${2:-3} ${3:-7}); do echo 0 \| sudo tee |

### P06 except: pass (silent swallow) — 33

| file | line | detail |
|------|------|--------|
| `os/docs/_gap_analysis.py` | 24 | except OSError: |
| `os/docs/_professionalism_audit.py` | 37 | except OSError: |
| `os/docs/_professionalism_audit.py` | 53 | except OSError: |
| `os/tinker-cowork/tinker-cowork.py` | 254 | except: |
| `os/vokk/computer_use.py` | 40 | except ImportError: |
| `os/vokk/taskbar_ai.py` | 45 | except Exception: |
| `os/vokk/taskbar_ai.py` | 71 | except Exception: |
| `os/vokk/taskbar_ai.py` | 81 | except Exception: |
| `os/vokk/taskbar_ai.py` | 91 | except Exception: |
| `os/vokk/taskbar_ai.py` | 98 | except Exception: |
| `os/vokk/vokk-ui.py` | 208 | except Exception: |
| `os/vokk/vokk-ui.py` | 212 | except Exception: |
| `os/vokk/vokk-v4-engine.py` | 394 | except ValueError: |
| `os/vokk/vokk.py` | 15 | except: |
| `os/vokk/vokk.py` | 86 | except: |
| `os/vokk/vokk.py` | 208 | except: |
| `os/vokk/vokk.py` | 240 | except: |
| `os/vokk/vokk.py` | 246 | except: |
| `os/vokk/vokk.py` | 266 | except: |
| `os/vokk/vokk.py` | 282 | except: |
| `os/vokk/vokk.py` | 414 | except: |
| `os/vokk/vokk.py` | 427 | except: |
| `os/vokk/vokk.py` | 558 | except: |
| `os/territories/vibe-address/searchie-gui.py` | 129 | except Exception: |
| `os/territories/vibe-address/searchie-gui.py` | 140 | except Exception: |
| `os/game-console/parc-game-console.py` | 152 | except Exception: |
| `os/game-console/parc-game-console.py` | 186 | except Exception: |
| `os/game-console/parc-game-console.py` | 202 | except Exception: |
| `os/game-console/parc-game-console.py` | 234 | except Exception: |
| `os/mobile-companion/scripts/mobile-companion-server.py` | 188 | except WebSocketError: |
| `os/mobile-companion/scripts/mobile-companion-server.py` | 193 | except Exception: |
| `os/mobile-companion/scripts/mobile-companion-server.py` | 198 | except Exception: |
| `os/desktop/nibra-style/server.py` | 228 | except OSError: |

### P13 shell=True — 29

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 174 | add("P13 shell=True", rel, i, s[:50]) |
| `os/tinker-cowork/tinker-cowork.py` | 290 | result = subprocess.run(action.content, shell=True |
| `os/onboarding/parc-onboard.py` | 371 | subprocess.run(f"{gsettings} gtk-theme 'Adwaita-da |
| `os/onboarding/parc-onboard.py` | 372 | subprocess.run(f"{gsettings} color-scheme 'prefer- |
| `os/onboarding/parc-onboard.py` | 374 | subprocess.run(f"{gsettings} gtk-theme 'Adwaita'", |
| `os/onboarding/parc-onboard.py` | 375 | subprocess.run(f"{gsettings} color-scheme 'prefer- |
| `os/onboarding/parc-onboard.py` | 378 | subprocess.run(f"setxkbmap {self.state.keyboard_la |
| `os/onboarding/parc-onboard.py` | 382 | subprocess.run(f"timedatectl set-timezone {self.st |
| `os/onboarding/parc-onboard.py` | 386 | subprocess.run(f"flatpak install -y flathub {app.l |
| `os/vokk/computer_use.py` | 223 | subprocess.Popen(command, shell=True, stdout=subpr |
| `os/vokk/vokk.py` | 213 | out = subprocess.run("top -bn1 \| grep Cpu(s) \| awk |
| `os/vokk/vokk.py` | 217 | out = subprocess.run("free -m", shell=True, captur |
| `os/vokk/vokk.py` | 221 | out = subprocess.run("df -h /", shell=True, captur |
| `os/vokk/vokk.py` | 231 | out = subprocess.run("cat /sys/class/power_supply/ |
| `os/vokk/vokk.py` | 236 | out = subprocess.run("cat /sys/class/thermal/therm |
| `os/vokk/vokk.py` | 243 | out = subprocess.run("vcgencmd measure_temp 2>/dev |
| `os/vokk/vokk.py` | 251 | out = subprocess.run("ip addr show wlan0 2>/dev/nu |
| `os/vokk/vokk.py` | 254 | out = subprocess.run("ip addr show eth0 2>/dev/nul |
| `os/vokk/vokk.py` | 410 | out = subprocess.run("ls /usr/share/applications/* |
| `os/vokk/vokk.py` | 424 | out = subprocess.run("find " + str(Path.home()) +  |
| `os/vokk/vokk.py` | 444 | out = subprocess.run("uptime -p 2>/dev/null \|\| upt |
| `os/vokk/vokk.py` | 449 | out = subprocess.run("ps -eo comm,%cpu,%mem --sort |
| `os/vokk/vokk.py` | 464 | out = subprocess.run("amixer get Master 2>/dev/nul |
| `os/vokk/system-controller/controller.py` | 89 | gpu = subprocess.check_output(['lspci', '\|', 'grep |
| `os/hardware-cert/parc-hardware-cert.py` | 176 | result = subprocess.run(test["cmd"], shell=True, c |
| `os/ai/inference.py` | 95 | subprocess.run(cmd, shell=True, check=False) |
| `os/game-console/parc-game-console.py` | 267 | subprocess.Popen(game[3], shell=True)  # executabl |
| `os/languages/korlang/korlang.py` | 976 | r=subprocess.run(f"gcc -o {binary} {c_file} -lm -l |
| `os/desktop/nibra-style/server.py` | 28 | return subprocess.run(cmd, shell=True, capture_out |

### C15 regex in [[ ]] unquoted RHS risk — 28

| file | line | detail |
|------|------|--------|
| `os/apps/system/backup-restore.sh` | 110 | if [[ ! "$name" =~ ^[A-Za-z0-9._-]+$ ]]; then |
| `os/apps/system/backup-restore.sh` | 193 | if [[ ! "$name" =~ ^[A-Za-z0-9._-]+$ ]]; then |
| `os/apps/system/power-manager.sh` | 93 | if [[ ! "$profile" =~ ^[A-Za-z0-9._-]+$ ]] \|\| [[ "$profile"  |
| `os/parc-ai/modules/agent-browser.sh` | 199 | if [[ ! "$url" =~ ^https?:// ]]; then |
| `os/parc-ai/modules/nlu.sh` | 11 | if [[ "$text" =~ ^(hi\|hello\|hey\|good\s*(morning\|afternoon\|ev |
| `os/parc-ai/modules/nlu.sh` | 13 | if [[ "$text" =~ ^(bye\|goodbye\|see\s*ya\|later\|exit\|quit) ]]; |
| `os/parc-ai/modules/nlu.sh` | 15 | if [[ "$text" =~ ^(what\|who\|where\|when\|why\|how\|which\|can you |
| `os/parc-ai/modules/nlu.sh` | 17 | if [[ "$text" =~ ^(open\|launch\|start\|run\|execute\|play\|show\|h |
| `os/parc-ai/modules/nlu.sh` | 19 | if [[ "$text" =~ ^(create\|make\|write\|draft\|generate\|build\|co |
| `os/parc-ai/modules/nlu.sh` | 21 | if [[ "$text" =~ ^(search\|find\|look\|google\|lookup\|query\|fetc |
| `os/parc-ai/modules/nlu.sh` | 23 | if [[ "$text" =~ ^(remind\|schedule\|set\|add\|create) ]] && [[  |
| `os/parc-ai/modules/nlu.sh` | 25 | if [[ "$text" =~ ^(code\|program\|function\|script\|debug\|fix\|co |
| `os/parc-ai/modules/nlu.sh` | 27 | if [[ "$text" =~ ^(image\|photo\|picture\|draw\|illustration\|aud |
| `os/parc-ai/modules/nlu.sh` | 29 | if [[ "$text" =~ ^(summarize\|summary\|tldr\|condense\|shorten\|b |
| `os/parc-ai/modules/nlu.sh` | 31 | if [[ "$text" =~ ^(translate\|translation\|convert.*language)  |
| `os/parc-ai/modules/nlu.sh` | 33 | if [[ "$text" =~ ^(calculate\|compute\|solve\|math\|what\s+is\s+ |
| `os/parc-ai/modules/nlu.sh` | 35 | if [[ "$text" =~ ^(set\|toggle\|adjust\|change\|configure\|enable |
| `os/parc-ai/modules/nlu.sh` | 37 | if [[ "$text" =~ ^(buy\|order\|purchase\|book\|reserve\|shop\|pric |
| `os/parc-ai/modules/nlu.sh` | 39 | if [[ "$text" =~ ^(help\|assist\|support\|what can you) ]]; the |
| `os/parc-ai/modules/nlu.sh` | 41 | if [[ "$text" =~ ^(thanks\|thank you\|thx\|appreciate) ]]; then |
| `os/parc-ai/modules/nlu.sh` | 62 | if [[ "$text" =~ (tomorrow\|today\|next\s+(week\|month\|monday\|t |
| `os/territories/vibe-address/core/time.sh` | 84 | if [[ "$phrase" =~ (first\|1st)[[:space:]]+(week\|wk) ]]; then |
| `os/territories/vibe-address/core/time.sh` | 85 | elif [[ "$phrase" =~ (second\|2nd)[[:space:]]+(week\|wk) ]]; t |
| `os/territories/vibe-address/core/time.sh` | 86 | elif [[ "$phrase" =~ (third\|3rd)[[:space:]]+(week\|wk) ]]; th |
| `os/territories/vibe-address/core/time.sh` | 87 | elif [[ "$phrase" =~ (fourth\|4th)[[:space:]]+(week\|wk) ]]; t |
| `os/territories/vibe-address/core/time.sh` | 97 | if [[ "$phrase" =~ (19\|20)[0-9]{2} ]]; then year="${BASH_REM |
| `os/territories/vibe-address/core/time.sh` | 138 | if [[ "$phrase" =~ ([0-9]{1,2})[[:space:]]*(am\|pm\|oclock) ]] |
| `os/territories/vibe-address/core/time.sh` | 162 | if [[ "$phrase" =~ between[[:space:]]+(.+)[[:space:]]+and[[: |

### C24 exporting a local-ish var — 27

| file | line | detail |
|------|------|--------|
| `os/install-parcai.sh` | 80 | export HOME="${HOME:-/home/tinkerspace}" |
| `os/apps/customization/qt-theme.sh` | 14 | export QT_STYLE_OVERRIDE="$theme" |
| `os/apps/customization/qt-theme.sh` | 15 | echo "export QT_STYLE_OVERRIDE=$theme" >> ~/.bashrc |
| `os/apps/network/proxy-manager.sh` | 54 | export http_proxy="$http_proxy" |
| `os/apps/network/proxy-manager.sh` | 55 | export https_proxy="$https_proxy" |
| `os/parc-ai/korrinos-schedule.sh` | 43 | export ID="${id:-}" NAME="${name:-}" COMMAND="${command:-}"  |
| `os/parc-ai/korrinos-schedule.sh` | 145 | export TASK_ID="${task_id:-}" NOW_ISO="$(date -Iseconds)" |
| `os/parc-ai/korrinos-schedule.sh` | 202 | export TASK_ID="${task_id:-}" |
| `os/parc-ai/korrinos-schedule.sh` | 221 | export TASK_ID="${task_id:-}" |
| `os/parc-ai/korrinos-schedule.sh` | 247 | export TASK_ID="${id:-}" MESSAGE="${message:-}" TIME="${time |
| `os/territories/world-engine.sh` | 70 | export TINKER_WORLD=$w |
| `os/territories/world-engine.sh` | 139 | export TINKER_WORLD="$w" |
| `os/territories/vibe-address/core/selftest.sh` | 21 | export HOME="$tmp" |
| `os/territories/vibe-address/core/selftest.sh` | 22 | export VIBE_HOME="$tmp/vibe" |
| `os/territories/vibe-address/core/selftest.sh` | 25 | export VIBE_EVENTS="$VIBE_HOME/events" |
| `os/territories/vibe-address/core/selftest.sh` | 26 | export VIBE_TREE="$VIBE_HOME/tree" |
| `os/territories/vibe-address/core/selftest.sh` | 27 | export VIBE_INDEX="$VIBE_HOME/index" |
| `os/territories/vibe-address/core/selftest.sh` | 28 | export VIBE_STATE="$VIBE_HOME/state" |
| `os/territories/vibe-address/core/selftest.sh` | 29 | export VIBE_CACHE="$VIBE_HOME/cache" |
| `os/territories/vibe-address/core/selftest.sh` | 245 | export HOME="$HOME_OLD" |
| `os/territories/vibe-address/core/selftest.sh` | 246 | export VIBE_HOME="$VIBE_HOME_OLD" |
| `os/hardware-tech/ray-traced-audio/ray-traced-audio.sh` | 11 | export TINKER_AUDIO_BACKEND="$(cd "$(dirname "${BASH_SOURCE[ |
| `os/hardware-tech/lifespan-doubler/lifespan-doubler.sh` | 120 | export TINKER_BATTERY_BACKEND="$BAT_BIN" |
| `os/hardware-tech/fpga-scaler/fpga-scaler.sh` | 10 | export TINKER_FPGA_BACKEND="$(cd "$(dirname "${BASH_SOURCE[0 |
| `os/hardware-tech/dvfs-shaver/dvfs-shaver.sh` | 70 | export TINKER_MSR_BACKEND="$MSR_BIN" |
| `os/territories/vibe-address/searchie` | 15 | export SEARCHIE_ENGINE="${SEARCHIE_ENGINE:-$SEARCHIE_ROOT}" |
| `os/territories/vibe-address/searchie` | 16 | export VIBE_HOME="${VIBE_HOME:-$HOME/.local/share/tinkeros/v |

### W09 'any' type escape hatch — 22

| file | line | detail |
|------|------|--------|
| `os/apps/apps/nibra-betterlife/src/AgentReview.tsx` | 2 | export default function AgentReview({action,s,setS |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 14 | export default function App({account=null,onSignou |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 15 | const [s,setS]=useState<State>(workspace.load),[pa |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 17 | const [attachments,setAttachments]=useState<{id:st |
| `os/apps/apps/nibra-betterlife/src/App.tsx` | 33 | const newItem=(type:string,item:any=null)=>{setEdi |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 2 | export default function AuthGate(){const [stage,se |
| `os/apps/apps/nibra-betterlife/src/AuthGate.tsx` | 3 | const enter=async(u:any)=>{try{await initializeWor |
| `os/apps/apps/nibra-betterlife/src/ChatPanel.tsx` | 2 | export default function ChatPanel({s,setS,notify,g |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 3 | export function parseAgentResponse(text:string):{r |
| `os/apps/apps/nibra-betterlife/src/agent.ts` | 4 | export function applyAction(state:State,a:AgentAct |
| `os/apps/apps/nibra-betterlife/src/core.ts` | 18 | accountStatus:()=>Promise<{user:any;local?:boolean |
| `os/apps/apps/nibra-betterlife/src/data.ts` | 4 | export type AgentAction={id:string;type:string;tit |
| `os/apps/apps/nibra-betterlife/src/providers.ts` | 4 | 'Z.ai':{endpoint:'https://api.z.ai/api/paas/v4',mo |
| `os/apps/apps/nibra-betterlife/server/worker.mjs` | 14 | if(u.pathname==='/api/device/start'&&req.method=== |
| `os/apps/apps/aether-workspace/src/App.tsx` | 40 | // Exclude any mock/sample notes to ensure only re |
| `os/apps/apps/aether-workspace/src/App.tsx` | 54 | // Exclude any mock/sample tasks |
| `os/apps/apps/aether-workspace/src/App.tsx` | 68 | // Exclude any mock/sample journal entries |
| `os/apps/apps/aether-workspace/src/App.tsx` | 194 | // Extract any new tasks from checklists |
| `os/apps/apps/aether-workspace/src/types.ts` | 66 | config?: Record<string, any>; |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 180 | onChange={(e) => setCategory(e.target.value as any |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 192 | onChange={(e) => setMood(e.target.value as any)} |
| `os/apps/apps/aether-workspace/src/components/SecuritySyncModal.tsx` | 128 | } catch (err: any) { |

### P05 bare except — 21

| file | line | detail |
|------|------|--------|
| `os/parc-ai/model/parc_inference.py` | 95 | except: |
| `os/tinker-cowork/tinker-cowork.py` | 254 | except: |
| `os/vokk/vokk.py` | 15 | except: |
| `os/vokk/vokk.py` | 86 | except: |
| `os/vokk/vokk.py` | 94 | except: |
| `os/vokk/vokk.py` | 129 | except: |
| `os/vokk/vokk.py` | 208 | except: |
| `os/vokk/vokk.py` | 240 | except: |
| `os/vokk/vokk.py` | 246 | except: |
| `os/vokk/vokk.py` | 266 | except: |
| `os/vokk/vokk.py` | 273 | except: |
| `os/vokk/vokk.py` | 282 | except: |
| `os/vokk/vokk.py` | 414 | except: |
| `os/vokk/vokk.py` | 427 | except: |
| `os/vokk/vokk.py` | 439 | except: |
| `os/vokk/vokk.py` | 542 | except: |
| `os/vokk/vokk.py` | 552 | except: |
| `os/vokk/vokk.py` | 558 | except: |
| `os/vokk/vokk.py` | 562 | except: |
| `os/vokk/vokk_controller.py` | 20 | except: |
| `os/vokk/vokk_controller.py` | 27 | except: |

### C10 eval present — 20

| file | line | detail |
|------|------|--------|
| `os/apps/system/quick-actions.sh` | 56 | eval "$cmd" |
| `os/parc-ai/korrinos-schedule.sh` | 178 | eval "$command" 2>&1 \| tee -a "$SCHEDULE_LOG" |
| `os/parc-ai/parcai-model-pipeline.sh` | 381 | model.eval() |
| `os/parc-ai/overlay/agent-narrator.sh` | 25 | output=$(eval "$cmd" 2>&1) |
| `os/parc-ai/modules/ai-brain-smart.sh` | 402 | result = eval('$expr') |
| `os/parc-ai/modules/ai-master-brain.sh` | 264 | result = eval('$expr') |
| `os/parc-ai/modules/cards-interactive.sh` | 95 | var r=eval(input.replace(/AND/g,'&&').replace(/OR/g,'\|\|').replace(/==/ |
| `os/parc-ai/modules/math.sh` | 15 | result = eval(expr, {'__builtins__': {}}, ns) |
| `os/system/backup/korrinos-backup.sh` | 58 | dest=$(eval echo "$dest") |
| `os/system/backup/korrinos-backup.sh` | 155 | dest=$(eval echo "$dest") |
| `os/system/backup/korrinos-backup.sh` | 252 | dest=$(eval echo "$dest") |
| `os/system/desktop-env/korrinos-desktop.sh` | 1170 | eval "$(xdotool getmouselocation 2>/dev/null \| sed 's/x:\([0-9]*\) y:\ |
| `os/territories/hack/hack-ways.sh` | 25 | "xss-check\|eval payload in browser (manual)\|medium\|[CONSENT]\|cross-sit |
| `os/territories/hack/threat-monitor.sh` | 76 | && debug "risky pattern in $f (eval / destructive)" \|\| true |
| `os/territories/hack/threat-monitor.sh` | 83 | grep -nE 'exec\(\|eval\(\|os\.system\(\|subprocess.*shell=True' "$f" 2>/d |
| `os/ai/voice-assistant.sh` | 152 | eval "${COMMANDS[$action]}" |
| `os/ai/voice-engine.sh` | 182 | eval "${ACTIONS[$best_action]}" |
| `os/desktop/app-launcher.sh` | 62 | eval "$exec &" |
| `os/desktop/shortcuts.sh` | 289 | [ -z "$W" ] && eval "$(xdotool getdisplaygeometry \| awk '{print "W="$1 |
| `os/desktop/text-expander.sh` | 62 | expansion=$(eval "$expansion" 2>/dev/null) |

### P17 os.system — 20

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 184 | add("P17 os.system", rel, i, s[:50]) |
| `os/software-gpu/compiler/korlangc.py` | 92 | # os.system(f"./{exe}"), which produced ".//tmp/x" |
| `os/mobile-companion/parc-mobile.py` | 210 | os.system("playerctl play") |
| `os/mobile-companion/parc-mobile.py` | 212 | os.system("playerctl pause") |
| `os/mobile-companion/parc-mobile.py` | 214 | os.system("playerctl next") |
| `os/mobile-companion/parc-mobile.py` | 216 | os.system("playerctl previous") |
| `os/mobile-companion/parc-mobile.py` | 218 | os.system("pactl set-sink-volume @DEFAULT_SINK@ +5 |
| `os/mobile-companion/parc-mobile.py` | 220 | os.system("pactl set-sink-volume @DEFAULT_SINK@ -5 |
| `os/mobile-companion/parc-mobile.py` | 222 | os.system("pactl set-sink-mute @DEFAULT_SINK@ togg |
| `os/mobile-companion/parc-mobile.py` | 224 | os.system("xdg-screensaver lock") |
| `os/mobile-companion/parc-mobile.py` | 226 | os.system("systemctl suspend") |
| `os/mobile-companion/parc-mobile.py` | 228 | os.system("systemctl poweroff") |
| `os/mobile-companion/parc-mobile.py` | 230 | os.system("systemctl reboot") |
| `os/mobile-companion/parc-mobile.py` | 232 | os.system(params.get("command", "")) |
| `os/mobile-companion/parc-mobile.py` | 234 | os.system(f"xdotool type '{params.get('text', '')} |
| `os/mobile-companion/parc-mobile.py` | 236 | os.system(f"xdotool key {params.get('key', '')}") |
| `os/mobile-companion/parc-mobile.py` | 238 | os.system(f"xdotool mousemove {params.get('x', 0)} |
| `os/mobile-companion/parc-mobile.py` | 240 | os.system(f"xdotool click {params.get('button', 1) |
| `os/mobile-companion/parc-mobile.py` | 242 | os.system("gnome-screenshot -a -f /tmp/mobile_scre |
| `os/mobile-companion/scripts/mobile-companion-server.py` | 71 | rc = os.system(cmd)  # noqa: S605 - user-invoked r |

### C101 line >100 cols — 20

| file | line | detail |
|------|------|--------|
| `os/hardware-tech/lib/device-weights.c` | 78 | 116 cols |
| `os/hardware-tech/backend/src/battery_control.c` | 82 | 108 cols |
| `os/hardware-tech/backend/src/battery_control.c` | 143 | 116 cols |
| `os/hardware-tech/backend/src/battery_control.c` | 151 | 113 cols |
| `os/hardware-tech/backend/src/cat_control.c` | 160 | 107 cols |
| `os/hardware-tech/backend/src/cat_control.c` | 192 | 117 cols |
| `os/hardware-tech/backend/src/cat_control.c` | 212 | 134 cols |
| `os/hardware-tech/backend/src/cat_control.c` | 214 | 128 cols |
| `os/hardware-tech/backend/src/fan_control.c` | 147 | 144 cols |
| `os/hardware-tech/backend/src/fpga_control.c` | 73 | 105 cols |
| `os/hardware-tech/backend/src/msr_control.c` | 276 | 105 cols |
| `os/hardware-tech/backend/src/usb_control.c` | 84 | 114 cols |
| `os/languages/demo/hello.c` | 31 | 145 cols |
| `os/languages/demo/hello.c` | 32 | 101 cols |
| `os/languages/demo/hello.c` | 38 | 111 cols |
| `os/languages/demo/hello.c` | 40 | 101 cols |
| `os/languages/demo/hello.c` | 43 | 103 cols |
| `os/languages/demo/hello.c` | 47 | 121 cols |
| `os/languages/demo/hello.c` | 51 | 157 cols |
| `os/languages/demo/hello.c` | 53 | 111 cols |

### P02 trailing whitespace — 16

| file | line | detail |
|------|------|--------|
| `os/parc-ai/model/parc_model.py` | 85 | 'n_layers=6, ' |
| `os/iso-builder-pro/parc-iso-pro.py` | 158 | 'str = None, ' |
| `os/iso-builder-pro/parc-iso-pro.py` | 260 | 'ln", "-sf", ' |
| `os/iso-builder-pro/parc-iso-pro.py` | 274 | 'ash", "-c", ' |
| `os/onboarding/parc-onboard.py` | 459 | 'f-Healing", ' |
| `os/vokk/vokk-v4-engine.py` | 84 | 'llama-cli", ' |
| `os/vokk/vokk-v4-engine.py` | 137 | ' int = 512, ' |
| `os/vokk/vokk-v4-engine.py` | 226 | 'interface", ' |
| `os/marketplace/parc-market.py` | 212 | ': str = "", ' |
| `os/ai/train.py` | 70 | 'embed_dim)] ' |
| `os/ai/train.py` | 74 | 'idden_dim)] ' |
| `os/ai/train.py` | 78 | 'm_classes)] ' |
| `os/game-console/parc-game-console.py` | 172 | 'ion,name"], ' |
| `os/account/parc-account.py` | 262 | 'ker._tcp"], ' |
| `os/mobile-companion/parc-mobile.py` | 252 | 'capacity"], ' |
| `os/enterprise/parc-enterprise.py` | 117 | 'HERE id=?", ' |

### N02 non-canonical brand in use — 15

| file | line | detail |
|------|------|--------|
| `<tree-wide>` | 0 | Korrin: 1298 occurrences |
| `<tree-wide>` | 0 | korrin: 2584 occurrences |
| `<tree-wide>` | 0 | KorrinOS: 24 occurrences |
| `<tree-wide>` | 0 | Tinkeros: 2 occurrences |
| `<tree-wide>` | 0 | tinkeros: 9 occurrences |
| `<tree-wide>` | 0 | Tinker AI: 4 occurrences |
| `<tree-wide>` | 0 | TinkerAI: 4 occurrences |
| `<tree-wide>` | 0 | tinker-ai: 4 occurrences |
| `<tree-wide>` | 0 | Parc: 10 occurrences |
| `<tree-wide>` | 0 | parc-ai: 1755 occurrences |
| `<tree-wide>` | 0 | VOKK: 215 occurrences |
| `<tree-wide>` | 0 | vokk: 796 occurrences |
| `<tree-wide>` | 0 | Zegrate: 4 occurrences |
| `<tree-wide>` | 0 | HyperDrive: 52 occurrences |
| `<tree-wide>` | 0 | hyperdrive: 357 occurrences |

### C22 bare sleep — 13

| file | line | detail |
|------|------|--------|
| `os/apps/gaming/screenshot-tool.sh` | 93 | sleep 1 |
| `os/apps/network/wifi-analyzer.sh` | 90 | sleep 1 |
| `os/parc-ai/korrinos-smoothui.sh` | 228 | sleep 1 |
| `os/parc-ai/modules/parcos-features.sh` | 218 | sleep 1 |
| `os/system/install-greetings.sh` | 107 | sleep 2 |
| `os/system/desktop-env/korrinos-notify.sh` | 173 | sleep 1 |
| `os/mobile-companion/scripts/mobile-companion-daemon.sh` | 43 | sleep 1 |
| `os/desktop/setup-wizard.sh` | 62 | sleep 1 |
| `os/desktop/setup-wizard.sh` | 106 | sleep 1 |
| `os/desktop/setup-wizard.sh` | 172 | sleep 1 |
| `os/desktop/setup-wizard.sh` | 210 | sleep 1 |
| `os/desktop/start-desktop.sh` | 15 | sleep 2 |
| `os/desktop/nibra-style/nibra-shell.sh` | 89 | sleep 1 |

### C110 missing SPDX license header — 12

| file | line | detail |
|------|------|--------|
| `os/hardware-tech/lib/device-weights.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/audio_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/battery_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/cat_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/display_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/fan_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/fpga_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/msr_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/thermal_control.c` | 1 | no SPDX in first 12 lines |
| `os/hardware-tech/backend/src/usb_control.c` | 1 | no SPDX in first 12 lines |
| `os/languages/demo/hello.c` | 1 | no SPDX in first 12 lines |
| `kernel/tinker/hyperdrive.c` | 1 | no SPDX in first 12 lines |

### P11 eval() — 8

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 170 | add("P11 eval()", rel, i, s[:50]) |
| `os/docs/_professionalism_audit.py` | 240 | add("W06 eval()", rel, i, s[:50]) |
| `os/parc-ai/model/parc_inference.py` | 36 | self.model.eval() |
| `os/parc-ai/model/parc_model.py` | 156 | self.eval() |
| `os/parc-ai/model/parc_trainer.py` | 218 | self.model.eval() |
| `os/parc-ai/model/train_tinkerai_5b.py` | 116 | model.eval() |
| `os/parc-ai/model/train_tinkeria_lora.py` | 114 | model.eval() |
| `os/vokk/vokk.py` | 437 | val = eval(expr, {"__builtins__": None}, {}) |

### P18 assert used outside tests — 8

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 186 | add("P18 assert used outside tests", rel, i, s[:50 |
| `os/territories/vibe-address/searchie-gui.py` | 523 | assert r.returncode == 0, f"record failed: {r.stde |
| `os/territories/vibe-address/searchie-gui.py` | 526 | assert hits, f"ask terse got nothing:\n{a.stdout}{ |
| `os/territories/vibe-address/searchie-gui.py` | 530 | assert "REVIEW\|" in d.stdout, f"delete no review:\ |
| `os/territories/vibe-address/searchie-gui.py` | 534 | assert items, "delete staged zero real files" |
| `os/territories/vibe-address/searchie-gui.py` | 535 | assert all(os.path.exists(p) for |
| `os/territories/vibe-address/searchie-gui.py` | 540 | assert c.returncode == 0, f"confirm failed: {c.std |
| `os/territories/vibe-address/searchie-gui.py` | 541 | assert not os.path.exists(probe), "confirm did not |

### C25 pipe to head (SIGPIPE risk) — 7

| file | line | detail |
|------|------|--------|
| `os/parc-ai/korrinos-devsuite.sh` | 324 | command -v gcc &>/dev/null && echo "  GCC: $(gcc --version 2 |
| `os/parc-ai/parcos-devtool.sh` | 137 | java -version 2>&1 \| head -1 \|\| echo "  Not installed" |
| `os/parc-ai/parcos-devtool.sh` | 145 | php --version 2>&1 \| head -1 \|\| echo "  Not installed" |
| `os/territories/hack/ephemeral-ram.sh` | 71 | HOME="$RAM_MNT/home" TMPDIR="$RAM_MNT/tmp" sudo -E "$@" 2>&1 |
| `os/territories/hack/gpu-pipeline.sh` | 70 | hashcat -b --benchmark-all 2>&1 \| head -25 \|\| true |
| `os/territories/hack/sdr-isolation.sh` | 63 | sudo hcitool lescan --passive --duplicates 2>&1 \| head -20 \| |
| `os/territories/hack/supply-chain.sh` | 71 | bash -c "hostname tinker-sim; date; $payload" 2>&1 \| head -4 |

### P12 exec() — 7

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 172 | add("P12 exec()", rel, i, s[:50]) |
| `os/parc-ai/parc-ai-gui.py` | 447 | app.exec() |
| `os/parc-ai/overlay/narrator.py` | 176 | app.exec() |
| `os/onboarding/parc-onboard.py` | 487 | sys.exit(app.exec()) |
| `os/control-center/parc-control-center.py` | 467 | sys.exit(app.exec()) |
| `os/territories/vibe-address/searchie-gui.py` | 557 | app.exec() |
| `os/marketplace/parc-market-gui.py` | 245 | sys.exit(app.exec()) |

### P04 TODO/FIXME left — 6

| file | line | detail |
|------|------|--------|
| `os/docs/_professionalism_audit.py` | 79 | m = re.search(r"\b(TODO\|FIXME\|XXX\|HACK)\b:?[ ]*(.{0,40})", l |
| `os/docs/_professionalism_audit.py` | 80 | add("C06 TODO/FIXME/HACK left in code", rel, i, (m.group(0)[ |
| `os/docs/_professionalism_audit.py` | 156 | add("P04 TODO/FIXME left", rel, i, s[:60]) |
| `os/docs/_professionalism_audit.py` | 214 | add("C108 TODO/FIXME left", rel, i, s[:50]) |
| `os/docs/_professionalism_audit.py` | 254 | add("W13 TODO/FIXME left", rel, i, s[:50]) |
| `os/tinker-cowork/tinker-cowork.py` | 379 | "Search for TODO comments in the codebase" |

### C105 memcpy (size must be checked) — 6

| file | line | detail |
|------|------|--------|
| `os/hardware-tech/backend/src/cat_control.c` | 45 | memcpy(out + 0, &ebx, 4); |
| `os/hardware-tech/backend/src/cat_control.c` | 46 | memcpy(out + 4, &edx, 4); |
| `os/hardware-tech/backend/src/cat_control.c` | 47 | memcpy(out + 8, &ecx, 4); |
| `os/hardware-tech/backend/src/msr_control.c` | 86 | memcpy(out + 0, &ebx, 4); |
| `os/hardware-tech/backend/src/msr_control.c` | 87 | memcpy(out + 4, &edx, 4); |
| `os/hardware-tech/backend/src/msr_control.c` | 88 | memcpy(out + 8, &ecx, 4); |

### W02 console.log left in — 6

| file | line | detail |
|------|------|--------|
| `os/apps/apps/nibra-betterlife/tests/desktop-smoke.mjs` | 28 | console.log('PASS: desktop launch, empty workspace |
| `os/apps/apps/nibra-betterlife/tests/package-smoke.mjs` | 60 | console.log( |
| `os/apps/apps/nibra-betterlife/tests/package-smoke.mjs` | 87 | console.log(`PASS: packaged ${process.platform} ap |
| `os/apps/apps/nibra-betterlife/tests/provider-boundary.mjs` | 4 | await p.evaluate(endpoint=>{const v=JSON.parse(loc |
| `os/apps/apps/nibra-betterlife/tests/require-node-sqlite.mjs` | 34 | console.log(`# SKIP ${label}: ${skipReason()}`); |
| `os/apps/apps/nibra-betterlife/tests/v11-desktop.mjs` | 2 | const bodies=[];const server=createServer((req,res |

### C21 curl piped to shell — 5

| file | line | detail |
|------|------|--------|
| `os/apps/network/mesh-network.sh` | 31 | echo "Install: curl -fsSL https://tailscale.com/install.sh \| |
| `os/apps/network/mesh-network.sh` | 55 | echo "Install: curl -s https://install.zerotier.com \| sudo b |
| `os/parc-ai/korrinos-tinkeria.sh` | 110 | echo "  (Ollama not available — install with: curl -fsSL htt |
| `os/parc-ai/parcos-devtool.sh` | 164 | curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \|  |
| `os/system/cloud-sync/korrinos-cloud.sh` | 98 | curl -s https://rclone.org/install.sh \| sudo bash 2>&1 \| tai |

### C26 complex inline command substitution — 4

| file | line | detail |
|------|------|--------|
| `os/apps/gaming/audio-mixer.sh` | 171 | local vol=$(pactl list sink-inputs 2>/dev/null \| awk -v id=" |
| `os/apps/system/system-monitor.sh` | 28 | local prev_total=$(awk '/^cpu / {for(i=2;i<=NF;i++) sum+=$i; |
| `os/apps/system/system-monitor.sh` | 31 | local total=$(awk '/^cpu / {for(i=2;i<=NF;i++) sum+=$i; prin |
| `os/parc-ai/modules/ai-self-learn.sh` | 81 | local result=$(curl -s -L "https://api.duckduckgo.com/?q=$en |

### W07 innerHTML assignment — 3

| file | line | detail |
|------|------|--------|
| `os/browser-extension/chrome/content.js` | 63 | notification.innerHTML = ` |
| `os/browser-extension/chrome/popup.js` | 44 | savedList.innerHTML = '<div class="saved-item">No  |
| `os/browser-extension/chrome/popup.js` | 46 | savedList.innerHTML = entries.map(([site, data]) = |

### C23 killall — 2

| file | line | detail |
|------|------|--------|
| `os/apps/gaming-support.sh` | 263 | killall gamemoded 2>/dev/null \|\| true |
| `os/system/desktop-env/korrinos-notify.sh` | 172 | killall dunst 2>/dev/null \|\| true |

### C103 unsafe string function — 2

| file | line | detail |
|------|------|--------|
| `os/hardware-tech/backend/src/audio_control.c` | 49 | strcpy(a.sun_path, "/run/user/0/pipewire-0"); |
| `os/hardware-tech/backend/src/audio_control.c` | 51 | if (!ok) { a.sun_path[0]=0; strcpy(a.sun_path+1,"p |

### W13 TODO/FIXME left — 2

| file | line | detail |
|------|------|--------|
| `os/apps/apps/aether-workspace/src/utils/automations.ts` | 31 | * Extract checkbox or TODO lines from note content |
| `os/apps/apps/aether-workspace/src/utils/automations.ts` | 38 | const todoRegex = /^[ \t]*(?:TODO\|FIXME\|ACTION):\s |

### W12 'as any' cast — 2

| file | line | detail |
|------|------|--------|
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 180 | onChange={(e) => setCategory(e.target.value as any |
| `os/apps/apps/aether-workspace/src/components/DailyJournal.tsx` | 192 | onChange={(e) => setMood(e.target.value as any)} |

### P15 str.format instead of f-string — 1

| file | line | detail |
|------|------|--------|
| `os/vokk/data_generator.py` | 41 | q = tpl.format(q=stem).strip().lower() |

### C106 sprintf (use snprintf) — 1

| file | line | detail |
|------|------|--------|
| `os/languages/demo/hello.c` | 15 | #define kl_fmt(buf, ...) sprintf(buf, __VA_ARGS__) |

### N01 brand-name split brain — 1

| file | line | detail |
|------|------|--------|
| `<tree-wide>` | 0 | korrin=2584, korrinos=2505, parc-ai=1755, Korrin=1298, KorrinOS=1281, vokk=796,  |
