# Roadmap

## v0.1: The Minimalist Pet

Goal: prove the paste -> parse -> persist -> mood -> display loop.

- Manual paste/import of user-provided Codex CLI `/status` text.
- Parser outputs normalized `QuotaSnapshot`.
- SQLite stores parsed snapshots and `raw_input_sha256`.
- Floating Windows-first Tauri pet window.
- Mood changes from quota thresholds.
- Basic mock/manual fallback.
- No network access, telemetry, scraping, or automatic Codex command execution.

## v0.2: The Feedback Loop

Goal: make the companion feel useful across a real work session.

- Improved parser coverage from fixtures.
- Better mood transitions and stale-data states.
- Notifications or gentle local reminders.
- Always-on-top/window behavior settings.
- History/trend summaries from local snapshots.
- Better warning copy from `parser_warnings`.

## v0.3: Ecosystem

Goal: make the architecture extensible without compromising privacy.

- Skin manifest support and first alternate skin.
- Provider contract stabilization.
- Additional CLI adapters, such as Claude or OpenRouter, if they fit the same local-first model.
- Optional user-exported JSON import.
- Cross-platform packaging validation for macOS and Linux.
