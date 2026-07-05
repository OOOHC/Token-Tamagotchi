# Token Tamagotchi

**A playful desktop companion for monitoring Codex quota, 5-hour limits, and usage pressure.**

Token Tamagotchi lives on your desktop and reacts to your Codex usage state before you run out mid-build.

Instead of opening another dashboard, you get a tiny companion that quietly shows whether your Codex quota is healthy, getting low, or exhausted.

## Vision

Developers do not need another dashboard.

Token Tamagotchi turns Codex quota pressure into an ambient desktop experience:

- Plenty of quota: your pet is happy.
- Quota getting low: your pet becomes concerned.
- Near limit: your pet starts panicking.
- Exhausted: your pet sleeps.
- Restored: your pet celebrates.

The goal is simple:

> Help developers notice Codex usage pressure before it interrupts their workflow.

## Core Philosophy

Your quota data stays on your machine. Token Tamagotchi does not scrape browser sessions, capture credentials, or automatically execute Codex commands in v0.1. You provide the input; the app provides the insight.

No network access is required for quota monitoring.

## Features

- **Privacy-first:** Manual, user-initiated quota input.
- **Local-only:** Parsed quota snapshots are stored locally in SQLite.
- **Opt-in raw history:** Raw CLI text is never stored unless explicitly enabled.
- **Mood engine:** The pet reacts to 5-hour quota, total quota, parser confidence, and warnings.
- **Provider-ready:** Codex is first, with future adapters possible through the same `QuotaSnapshot` contract.

## Project Status

Token Tamagotchi is in early v0.1 development.

The current MVP loop is:

```text
paste -> parse -> persist -> mood -> display
```

### Included in v0.1

- Floating Windows-first Tauri pet window.
- Manual paste/import of user-provided Codex CLI `/status` text.
- Parser output normalized into `QuotaSnapshot`.
- SQLite storage for parsed quota snapshots and `raw_input_sha256`.
- Pet mood changes based on quota thresholds.
- Mock/manual provider fallback.
- Local-only quota monitoring with no telemetry, scraping, or automatic Codex command execution.

### Not Required for v0.1

- System tray/menu bar status icon.
- Transparent window.
- Low-quota desktop notifications.
- `/usage` import beyond fixtures and future parser coverage.
- Claude, Cursor, OpenRouter, Gemini, or other provider adapters.
- Cloud sync, team analytics, or dashboard scraping.
- Skin marketplace or advanced animation tooling.

## How It Works

```text
Run /status in Codex
        |
Paste the output into Token Tamagotchi
        |
The app parses the result
        |
A quota snapshot is stored locally
        |
The pet mood updates
```

Token Tamagotchi does not collect OpenAI credentials, scrape private dashboards, or upload quota data.

## Pet Mood States

| Quota State | Pet Mood |
| --- | --- |
| Unknown or missing data | Sleeping |
| Healthy | Happy |
| Moderate pressure | Focused |
| Low quota | Tired |
| Stale or uncertain data | Sleeping / warning copy |

The first implementation keeps mood states intentionally small. More expressive transitions belong in later versions.

## Data Confidence

Every parsed snapshot includes parser confidence:

| Confidence | Meaning |
| --- | --- |
| `high` | Main quota fields parsed and no parser warnings. |
| `medium` | Partial but useful quota data parsed. |
| `low` | Unknown input, unsupported format, or too many missing fields. |

Parser warnings are surfaced so users can tell when the app understood only part of the pasted output.

## Development

Run commands from the repository root.

```bash
npm install
cargo build
cargo test --workspace
npm run build
```

For the desktop development flow:

```bash
npm run dev
```

## Requirements

- Node.js and npm.
- Rust and Cargo.
- Windows: Visual Studio Build Tools with the MSVC/C++ linker for Tauri/Rust builds.

## Project Layout

```text
token-tamagotchi/
|-- apps/desktop/              # Tauri desktop app, React + Vite frontend
|-- crates/token-core/         # Domain models, quota engine, mood engine
|-- crates/codex-adapter/      # Codex providers and parser logic
|-- crates/local-store/        # Local persistence
|-- fixtures/codex/            # Parser samples
|-- skins/                     # Pet skins and sprite metadata
`-- docs/                      # Product, architecture, parser, roadmap docs
```

## Privacy

By default, `quota_snapshots` store parsed values and `raw_input_sha256`, not raw CLI text.

Raw text belongs only in the opt-in `raw_imports_optional` table. If raw history/debugging is not enabled, pasted CLI output should be parsed and discarded after snapshot creation.

## Documentation

- [Product Blueprint](docs/product-blueprint.md)
- [PRD](docs/prd.md)
- [Architecture](docs/architecture.md)
- [Data Sources](docs/data-sources.md)
- [Parser Contract](docs/parser-contract.md)
- [Database Schema](docs/database.md)
- [MVP Acceptance Checklist](docs/mvp-acceptance-checklist.md)
- [Roadmap](docs/roadmap.md)
- [Development Guide](docs/development-guide.md)

## Disclaimer

Token Tamagotchi is an independent open-source project. It is not an official OpenAI product.

Codex usage data shown by Token Tamagotchi should be treated as an interpreted local snapshot. The official Codex usage dashboard remains the source of truth.
