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

- Official Codex app-server rate-limit source through local `codex app-server --stdio`.
- Auto refresh for 5-hour and 7-day remaining percentages.
- Token usage diagnostics for 5-hour and 7-day windows.
- Estimated total token capacity derived from local usage and official used percentages.
- 5-hour and 7-day reset timing in the details panel.
- Manual refresh, sync status, offline/error state, and low-quota local alert.
- Transparent, borderless, always-on-top companion window exploration.
- Default mode shows only the live mood companion.
- Companion mood updates from the configured quota thresholds.
- Click companion to show a temporary quota mood/status message bubble.
- Food-meter toggle appears after interacting with the companion.
- Side toggle reveals mini controls, 5-hour and 7-day food meters, and Details.
- Dragging the companion surface repositions the floating window.
- Window size adapts to companion-only, status bubble, food-meter, and details modes.
- Window position is constrained to the current screen so the companion and expanded panels remain reachable.
- Expanded panels can appear above or below the companion depending on screen position.
- Details content scrolls inside the details panel without adding a scrollbar to the companion surface.
- Basic click interaction, mood-specific facial expression, and lightweight idle/mood animation.
- Debug layer for local logs, manual import, source checks, and parser diagnostics.

Deferred from v0.2:

- System tray/menu bar status.
- Persistent history/trend charting.
- Full OS-level notification integration.
- Window behavior settings UI.
- True no-window desktop entity behavior.

## v0.3: Living Entity

Goal: evolve the companion into an animated desktop entity while keeping quota logic local and testable.

- True desktop-entity presentation with no obvious rectangular window surface.
- Better drag/positioning behavior for a companion that lives on the desktop.
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
