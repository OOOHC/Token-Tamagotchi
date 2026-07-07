# Product Blueprint: Token Tamagotchi

## Core Concept

Token Tamagotchi is a playful desktop companion for monitoring Codex quota, 5-hour limits, and 7-day usage pressure. It turns quota pressure into an ambient desktop signal so developers can notice limits before they interrupt a coding session.

## The Tamagotchi Metaphor

- **Input:** Local Codex app-server rate-limit data by default, with manual/debug providers retained for testing and fallback.
- **Processing:** Quota pressure analysis based on 5-hour remaining quota, 7-day remaining quota, reset timing, source freshness, and parser/provider confidence.
- **Feedback:** Bit, a desktop companion that changes mood, expression, color, and message based on quota health.
- **Signature Detail:** The companion does not just watch tokens, it eats tokens. The quota bar is a token food meter that empties as usage pressure rises.

## Companion States

| Remaining | Mood | Companion Copy |
| --- | --- | --- |
| `> 80%` | Happy | `5H 96%` |
| `50-80%` | Relaxed | `[Status]: Quota healthy.` |
| `20-50%` | Concerned | `(._.) I'm getting hungry...` |
| `5-20%` | Panicking | `[Status]: Low quota. Suggestion: Refactor prompt.` |
| `0-5%` | Exhausted | `(x_x) Please don't send another huge prompt...` |
| Restored | Celebrating | `Breakfast!!` |
| Quota drop | Feeding | `nom nom...` |

The companion's quota bar should feel like a food meter:

```text
[==========------] 63%

(o_o) nom nom...
```

This gives the monitor a memorable personality without weakening its role as a developer tool.

## Technical Philosophy

- **Local-first:** No cloud telemetry. The app must not upload quota, usage logs, prompts, project names, or local Codex output.
- **Provider-based:** Codex is the first provider, but the architecture should allow future providers without changing the UI model.
- **MVP-focused:** v1 is a monitor, v0.2 is the desktop companion, and prediction/AI-assisted recommendations are future scope.
- **Polished:** Pixel-art or terminal-style aesthetic, small-window ergonomics, and developer-friendly feedback.

## Brand Language

Use **desktop companion** in product-facing language. Avoid leading with **desktop pet**, which makes the product sound less like a developer tool.

The product should feel playful, but the language should stay useful. The companion can express emotion, but its copy must also communicate quota state or next action.

Prefer status-style copy that blends feeling with information:

```text
[Status]: Low Quota
[Suggestion]: Refactor Prompt
```

Avoid generic cute-only copy such as "I'm hungry" when the same moment can carry useful context. A stronger version is:

```text
[Status]: Low Quota. Suggestion: Refactor Prompt.
```

## Visual Direction

- Use pixel art or terminal-style UI.
- Favor techy blocks, pixels, terminal cells, and compact sprite shapes.
- Avoid a soft, round, toy-like cartoon aesthetic.
- Keep the window compact and glanceable.
- Treat progress bars as token food meters.
- Let expressions and short status text carry the companion's mood.

The companion should feel like it came from the code world, not like a decorative desktop widget.

## UI Metaphor

The quota progress bar is a semi-transparent token food meter. As quota is consumed, the food meter empties and the companion becomes more concerned.

This metaphor should guide the first UI implementation:

- The food meter must be visible at a glance when expanded.
- The default surface should show the companion first, not a dashboard.
- The companion state must be derived from remaining quota thresholds.
- 5-hour and 7-day food meters should use the same quota mood color system.
- Quota decreases should trigger a short token-eating animation and feeding copy.
- The UI should pair every emotional state with concrete quota information.
- Low quota states should reduce anxiety by being expressive, but never hide the actual quota state.

## Long-Term Companion Vision

The final form is not a dashboard. Token Tamagotchi should evolve into a desktop-level interactive companion: a small, always-present entity that appears to live on the user's desktop.

The product should evolve through three visual layers:

1. **Monitor:** readable quota state, provider confidence, warnings, and token food meter.
2. **Companion:** compact always-on-top desktop presence with automatic Codex quota reading, lightweight motion, and click/drag feedback.
3. **Living Entity:** animated 2D/3D presence with idle behavior, mouse awareness, and quota-reactive motion.

Catdeskpet-style desktop presence is a useful interaction reference, but product-facing language should remain **desktop companion**. The goal is a developer tool with presence, not an entertainment pet.

## MVP

- Read local Codex app-server rate limits without cloud telemetry.
- Show 5-hour and 7-day remaining quota as token food meters.
- Normalize every provider into a `QuotaSnapshot`.
- Convert 5-hour quota health into companion mood.
- Keep manual import/parser diagnostics as debug fallback, not the primary user path.
- Keep provider, parser, storage, and UI concerns separate.
