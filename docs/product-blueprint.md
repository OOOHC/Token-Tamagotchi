# Product Blueprint: Token Tamagotchi

## Core Concept

Token Tamagotchi is a privacy-first, local-only desktop companion for monitoring AI coding quota. It turns raw quota information into an at-a-glance companion state so users can understand whether they can keep working, slow down, or wait for a reset.

## The Tamagotchi Metaphor

- **Input:** User-provided Codex CLI/status output, with mock and manual providers used during MVP development.
- **Processing:** Quota pressure analysis based on 5-hour remaining quota, total remaining quota, reset timing, and parser confidence.
- **Feedback:** A living desktop companion that changes mood, expression, and message based on quota health.

## Technical Philosophy

- **Local-first:** No cloud telemetry. The app must not upload quota, usage logs, prompts, project names, or local Codex output.
- **Provider-based:** Codex is the first provider, but the architecture should allow future providers without changing the UI model.
- **MVP-focused:** Mock/manual quota and user-provided Codex CLI output parsing come first. Other providers are future scope.
- **Polished:** Pixel-art aesthetic, small-window ergonomics, and developer-friendly feedback.

## MVP

- Show mock/manual quota values in a desktop window.
- Accept user-provided Codex CLI output as raw text input.
- Normalize every provider into a `QuotaSnapshot`.
- Convert quota health into pet mood.
- Keep provider, parser, storage, and UI concerns separate.
