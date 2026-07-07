# Security

Token Tamagotchi should not collect or transmit Codex usage data without explicit user action.

## Privacy Boundaries

- Do not add telemetry, analytics, cloud sync, browser scraping, HTTPS proxying, or credential capture by default.
- Codex quota should be read from local sources only, such as `codex app-server --stdio` or read-only local Codex SQLite diagnostics.
- Local Codex logs may contain sensitive context. Only extract quota counters, reset timestamps, and usage totals.
- Do not persist prompts, responses, project paths, thread content, raw Codex log bodies, or raw pasted CLI text unless the user explicitly enables a local debugging/history feature.
- Debug tools must remain hidden in production builds unless explicitly enabled by a developer override.

Please avoid committing local quota exports, logs, API keys, database files, or screenshots that expose private projects. Report security concerns privately to the project maintainer.
