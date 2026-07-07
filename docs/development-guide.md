# Development Guide

## Prerequisites

- **Rust & Cargo:** Required for backend logic and core crates.
- **Node.js & npm:** Required for the Tauri/React desktop app.
- **Tauri OS Dependencies:** Follow the official Tauri setup docs for Windows, macOS, or Linux.

## Setup

Run all commands from the repository root.

```bash
npm install
```

This installs the desktop workspace dependencies.

## Development

```bash
npm run tauri -- dev
```

This launches the Tauri desktop app with the local React/Vite frontend.

For a production-style build check:

```bash
npm run build
cargo build
```

## Debug Tools

The Details panel contains developer-only controls for mood tests, token-feed animation tests, manual import, source checks, and parser diagnostics.

These controls are hidden by default in both development and production builds. Enable developer mode explicitly when you need to test moods, token eating, parser behavior, or source diagnostics:

```bash
VITE_SHOW_DEBUG_TOOLS=true
```

PowerShell example:

```powershell
$env:VITE_SHOW_DEBUG_TOOLS = "true"
npm run tauri -- dev
```

Use a normal `npm run build` without this environment variable for the production-ready version.

## Testing

```bash
cargo test --workspace
```

Parser and domain tests should use samples from `fixtures/`.

## Fixtures

Use `fixtures/codex/status/` and `fixtures/codex/usage/` for parser coverage. Add or update a fixture before changing parser behavior for a new Codex output format.

## Development Workflow

1. Define or update the contract in `docs/`.
2. Add or update fixtures under `fixtures/`.
3. Implement parser/domain logic in `crates/`.
4. Expose behavior through thin Tauri commands.
5. Consume normalized state from the UI.

## Development Rules

- Keep Tauri commands thin.
- Keep parser, quota, mood, and storage logic in `crates/`.
- Do not require network access for quota monitoring.
- Do not store raw CLI text unless the user explicitly opts in.
