# Token Tamagotchi

**A playful desktop companion for monitoring your Codex quota, 5-hour limits, and total usage.**

Token Tamagotchi lives on your desktop and keeps an eye on your 5-hour limits, total remaining usage, and token pressure so your workflow does not get interrupted mid-build.

Long term, Token Tamagotchi is designed to evolve from a quota monitor into a lightweight desktop companion that feels present on your desktop.

## Why

Monitor your Codex quota without opening another dashboard.

Codex quota pressure is easiest to miss when you are already deep in a refactor, PR review, or long coding task. Token Tamagotchi turns that pressure into an ambient desktop signal you can understand at a glance.

## MVP Features

- Tiny desktop companion.
- 5-hour quota display.
- Total remaining usage display.
- Auto refresh for displayed quota state.
- Low quota alerts.
- Mood changes based on remaining quota.

## Quick Start

```bash
npm install
cargo build
```

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
