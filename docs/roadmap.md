# Roadmap

Token Tamagotchi should evolve naturally:

```text
v0.1 = Monitor
v0.2 = Desktop Companion
v0.3 = Living Entity
v0.4 = Predict / Assist
```

## v0.1: Monitor

Goal: monitor Codex quota pressure through a playful desktop companion.

- Manual paste/import of user-provided Codex CLI `/status` text.
- Parser outputs normalized `QuotaSnapshot`.
- SQLite stores parsed snapshots and `raw_input_sha256`.
- Floating Windows-first Tauri companion window.
- 5-hour quota display.
- Total remaining usage display.
- Token food meter.
- Mood changes from quota thresholds.
- Auto refresh for displayed state and stale-data indicators.
- Basic mock/manual fallback.
- No network access, telemetry, scraping, or automatic Codex command execution.

## v0.2: Desktop Companion

Goal: make Token Tamagotchi feel like a small desktop presence rather than a normal utility window.

- Improved parser coverage from fixtures.
- Better mood transitions and stale-data states.
- Notifications or gentle local reminders.
- Always-on-top/window behavior settings.
- History/trend summaries from local snapshots.
- Better warning copy from `parser_warnings`.
- Transparent or borderless companion window exploration.
- Basic click/drag interactions and lightweight idle animation.

## v0.3: Living Entity

Goal: evolve the companion into an animated desktop entity while keeping quota logic local and testable.

- Skin manifest support and first alternate skin.
- Provider contract stabilization.
- Additional CLI adapters, such as Claude or OpenRouter, if they fit the same local-first model.
- Optional user-exported JSON import.
- Low-poly, voxel, Canvas, Three.js, or React Three Fiber rendering exploration.
- Mouse-aware reactions and quota-reactive motion.

## v0.4: Predict / Assist

Goal: offer lightweight guidance without compromising privacy.

- Estimates for remaining PR reviews, coding time, or task capacity.
- Answers to questions like "Can I still run this task?" or "Can I keep coding for another 2 hours?"
- Prompt splitting and model-choice suggestions.
- Cross-platform packaging validation for macOS and Linux.

AI-assisted guidance belongs after the monitor and desktop companion foundations, not v0.1. The first version should stay focused on being a clear quota monitor.
