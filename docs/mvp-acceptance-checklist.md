# MVP Acceptance Checklist

## Must Have

- [x] App launches on Windows as a Tauri desktop window.
- [x] UI accepts manual paste/import of Codex CLI `/status` text.
- [x] Parser returns a normalized `QuotaSnapshot`.
- [x] Parser handles unknown or partial input without crashing.
- [x] Parser records `confidence`, `parser_warnings`, and `raw_input_sha256`.
- [x] SQLite persists parsed quota snapshots across restarts.
- [x] Duplicate raw input can be detected through `raw_input_sha256`.
- [x] Companion mood changes based on predefined quota thresholds.
- [x] UI shows quota as a token food meter.
- [x] UI shows 5-hour quota, total quota, parser confidence, and key warning text.
- [x] UI refreshes displayed state and stale-data indicators on an interval.
- [x] App requires no network access for quota monitoring.

## Should Have

- [x] Window can stay always-on-top.
- [x] Window uses a compact floating companion layout.
- [x] Basic terminal/pixel-art visual style is implemented.
- [x] Manual/mock provider fallback is available at startup.

## Not Required For v0.1

- [ ] Transparent window.
- [ ] System tray/menu bar status icon.
- [ ] Advanced animation system.
- [ ] 3D/voxel/low-poly companion rendering.
- [ ] Mouse tracking or idle animation.
- [ ] AI prediction or prompt/model recommendations.
- [ ] Skin marketplace or skin editor.
- [ ] Cross-platform packaging validation for macOS and Linux.
