# Product Blueprint: Token Tamagotchi

## Core Concept

Token Tamagotchi is a playful desktop companion for monitoring Codex quota, 5-hour limits, and total usage. It turns quota pressure into an ambient desktop signal so developers can notice limits before they interrupt a coding session.

## The Tamagotchi Metaphor

- **Input:** User-provided Codex CLI/status output, with mock and manual providers used during MVP development.
- **Processing:** Quota pressure analysis based on 5-hour remaining quota, total remaining quota, reset timing, and parser confidence.
- **Feedback:** A desktop companion that changes mood, expression, and message based on quota health.
- **Signature Detail:** The companion does not just watch tokens, it eats tokens. The quota bar is a token food meter that empties as usage pressure rises.

## Companion States

| Remaining | Mood | Companion Copy |
| --- | --- | --- |
| `> 80%` | Happy | `(•ᴗ•) nom nom...` |
| `50-80%` | Relaxed | `[Status]: Quota healthy.` |
| `20-50%` | Concerned | `(•﹏•) I'm getting hungry...` |
| `5-20%` | Panicking | `[Status]: Low quota. Suggestion: Refactor prompt.` |
| `0-5%` | Exhausted | `(╥﹏╥) Please don't send another huge prompt...` |
| Restored | Celebrating | `＼(＾▽＾)／ Breakfast!!` |

The companion's quota bar should feel like a food meter:

```text
██████████░░░░░░ 63%

(•ᴗ•) nom nom...
```

This gives the monitor a memorable personality without weakening its role as a developer tool.

## Technical Philosophy

- **Local-first:** No cloud telemetry. The app must not upload quota, usage logs, prompts, project names, or local Codex output.
- **Provider-based:** Codex is the first provider, but the architecture should allow future providers without changing the UI model.
- **MVP-focused:** v1 is a monitor. Prediction and AI-assisted recommendations are future scope.
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
- Treat the progress bar as a token food meter.
- Let expressions and short status text carry the companion's mood.

The companion should feel like it came from the code world, not like a decorative desktop widget.

## UI Metaphor

The quota progress bar is a semi-transparent token food meter. As quota is consumed, the food meter empties and the companion becomes more concerned.

This metaphor should guide the first UI implementation:

- The food meter must be visible at a glance.
- The companion state must be derived from remaining quota thresholds.
- The UI should pair every emotional state with concrete quota information.
- Low quota states should reduce anxiety by being expressive, but never hide the actual numbers.

## Long-Term Companion Vision

The final form is not a dashboard. Token Tamagotchi should evolve into a desktop-level interactive companion: a small, always-present entity that appears to live on the user's desktop.

The product should evolve through three visual layers:

1. **Monitor:** readable quota state, parser confidence, warnings, and token food meter.
2. **Companion:** compact always-on-top desktop presence with lightweight motion and click/drag feedback.
3. **Living Entity:** animated 2D/3D presence with idle behavior, mouse awareness, and quota-reactive motion.

Catdeskpet-style desktop presence is a useful interaction reference, but product-facing language should remain **desktop companion**. The goal is a developer tool with presence, not an entertainment pet.

## MVP

- Show mock/manual quota values in a desktop window.
- Accept user-provided Codex CLI output as raw text input.
- Normalize every provider into a `QuotaSnapshot`.
- Convert quota health into companion mood.
- Display quota as a token food meter.
- Keep provider, parser, storage, and UI concerns separate.
