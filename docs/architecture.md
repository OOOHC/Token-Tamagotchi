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
User-provided Codex CLI text
  -> Provider/Adapter
  -> QuotaSnapshot
  -> Storage
  -> Tauri Command
  -> UI State
  -> Pet Feedback
```

## Dependency Direction

```text
apps/desktop/src
  -> apps/desktop/src-tauri/src/commands
  -> crates/*
```

UI must not know parser internals. Commands must not implement business rules. Core crates must not depend on React or Tauri.
