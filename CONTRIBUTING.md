# Contributing

Token Tamagotchi is early-stage. Keep changes small, tested, and aligned with the provider/parser/core layering described in `docs/architecture.md`.

## Development Rules

- Commands in `apps/desktop/src-tauri/src/commands/` should only bridge frontend calls into core crates.
- Parser behavior must be covered by fixtures under `fixtures/codex/`.
- UI state should consume normalized quota snapshots, not raw Codex output.

