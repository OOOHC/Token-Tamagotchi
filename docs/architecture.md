# Architecture

## High-Level Design

- **Frontend:** React + TypeScript (Vite) + Tauri WebView.
- **Desktop Shell:** Tauri windowing, commands, app state, and platform integration.
- **Backend:** Rust command bridge exposed through Tauri commands.
- **Core Logic:** Decoupled into `crates/` for reuse, testing, and future providers.

## Rules of Engagement

- **Commands layer (`apps/desktop/src-tauri/src/commands/`):** API boundary only. No parser, quota, mood, or SQL business logic here.
- **Core crates (`crates/`):** Domain models, parsing, provider contracts, quota calculations, mood logic, and persistence.
- **Frontend (`apps/desktop/src/`):** UI rendering and interaction state. It consumes normalized data, not raw Codex output.
- **Storage:** Stores parsed quota snapshots by default. Raw input storage is local-only and opt-in.
- **Pure functions preferred:** Parser, quota pressure, and mood logic should be testable without Tauri or React.

## Data Flow

```text
Codex app-server rate limits
  -> Codex Provider/Adapter
  -> QuotaSnapshot
  -> Storage
  -> Tauri Command
  -> UI State
  -> Companion Feedback
```

## Dependency Direction

```text
apps/desktop/src
  -> apps/desktop/src-tauri/src/commands
  -> crates/*
```

UI must not know parser internals. Commands must not implement business rules. Core crates must not depend on React or Tauri.

Manual import and parser fixtures follow the same adapter shape, but they are debug/fallback paths rather than the default product experience.

## Rendering Evolution

The current app uses React DOM UI inside a compact Tauri window. Desktop-presence work should remain a presentation-layer concern:

- **v0.2:** Transparent/borderless Tauri windows, always-on-top behavior, draggable companion surface, edge-aware panels, and lightweight 2D animation.
- **v0.3:** Canvas, Three.js, or React Three Fiber for low-poly or voxel-style real-time rendering, richer idle motion, and stronger desktop-entity behavior.

3D rendering must not move quota parsing, mood calculation, or persistence out of `crates/`. The living companion is a richer view of the same normalized `QuotaSnapshot` state.

## Privacy Implementation

Token Tamagotchi is local-first by design.

- The app does not collect OpenAI credentials.
- The app does not scrape private dashboards.
- The app does not upload quota data.
- The app reads local Codex rate-limit state through a user-local Codex process.
- Parsed quota snapshots are stored locally.
- Raw CLI text is never stored unless explicitly enabled for local history/debugging.

By default, `quota_snapshots` store parsed values and `raw_input_sha256`, not raw CLI text. Raw text belongs only in the opt-in `raw_imports_optional` table.

## Project Layout

```text
token-tamagotchi/
|-- apps/desktop/              # Tauri desktop app, React + Vite frontend
|-- crates/token-core/         # Domain models, quota engine, mood engine
|-- crates/codex-adapter/      # Codex providers and parser logic
|-- crates/local-store/        # Local persistence
|-- fixtures/codex/            # Parser samples
|-- skins/                     # Companion skins and sprite metadata
`-- docs/                      # Product, architecture, parser, roadmap docs
```
