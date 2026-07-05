# MVP Acceptance Checklist

## Must Have

- [ ] App launches on Windows as a Tauri desktop window.
- [ ] UI accepts manual paste/import of Codex CLI `/status` text.
- [ ] Parser returns a normalized `QuotaSnapshot`.
- [ ] Parser handles unknown or partial input without crashing.
- [ ] Parser records `confidence`, `parser_warnings`, and `raw_input_sha256`.
- [ ] SQLite persists parsed quota snapshots across restarts.
- [ ] Duplicate raw input can be detected through `raw_input_sha256`.
- [ ] Companion mood changes based on predefined quota thresholds.
- [ ] UI shows quota as a token food meter.
- [ ] UI shows 5-hour quota, total quota, parser confidence, and key warning text.
- [ ] UI refreshes displayed state and stale-data indicators on an interval.
- [ ] App requires no network access for quota monitoring.

## Should Have

- [ ] Window can stay always-on-top.
- [ ] Window uses a compact floating companion layout.
- [ ] Basic terminal/pixel-art visual style is implemented.
- [ ] Manual/mock provider fallback is available from the UI.

## Not Required For v0.1

- [ ] Transparent window.
- [ ] System tray/menu bar status icon.
- [ ] Advanced animation system.
- [ ] AI prediction or prompt/model recommendations.
- [ ] Skin marketplace or skin editor.
- [ ] Cross-platform packaging validation for macOS and Linux.
