# Parser Contract

The parser converts user-provided raw CLI text into the internal `QuotaSnapshot` model. It is a privacy boundary as much as a parsing boundary: raw input is sensitive, while normalized quota state is safe for UI, mood logic, and default storage.

The parser must never panic on unknown formats. Missing fields must be represented as `None`.

## Expected Output Structure

```rust
pub struct QuotaSnapshot {
    pub five_hour_remaining: Option<u64>,
    pub five_hour_limit: Option<u64>,
    pub total_remaining: Option<u64>,
    pub total_limit: Option<u64>,
    pub reset_at: Option<String>,
    pub source: QuotaSource,
    pub confidence: Confidence,
    pub parsed_at: String,
    pub raw_input_sha256: Option<String>,
    pub parser_warnings: Vec<String>,
}
```

## Field Rules

- `five_hour_remaining`: Remaining quota in the current 5-hour window.
- `five_hour_limit`: Total quota available in the current 5-hour window.
- `total_remaining`: Remaining quota for the larger plan/account period.
- `total_limit`: Total quota for the larger plan/account period.
- `reset_at`: Reset timestamp if the source exposes one.
- `source`: Source adapter that produced the snapshot.
- `confidence`: Parser certainty after evaluating the input.
- `parsed_at`: Timestamp when Token Tamagotchi parsed the input.
- `raw_input_sha256`: SHA-256 hash of the raw input, used for idempotent local writes and duplicate detection.
- `parser_warnings`: Human-readable warnings for partial parses, missing fields, or suspicious formats.

## Raw Input Handling

Raw input must not be stored inside `QuotaSnapshot`.

Raw text persistence is local-only and opt-in. If enabled by the user, storage should keep raw text separately from parsed snapshots, such as in a dedicated debug/import table. The raw input hash can be stored by default because it supports deduplication without retaining the original text.

## Confidence Determination Rules

Confidence must be deterministic and covered by tests. The primary fields for confidence are `five_hour_remaining` and `total_remaining`.

| Confidence | Required Conditions | Typical Scenario |
| --- | --- | --- |
| `High` | `five_hour_remaining` and `total_remaining` are parsed, and `parser_warnings` is empty. | Known Codex CLI output with no missing critical data. |
| `Medium` | At least one of `five_hour_remaining` or `total_remaining` is parsed, and `parser_warnings.len() <= 2`. | Partial parse, missing optional fields, or minor format drift. |
| `Low` | Neither `five_hour_remaining` nor `total_remaining` is parsed, or `parser_warnings.len() > 2`. | Unknown input, unsupported format, or parser failure. |

The implementation should expose a pure `calculate_confidence` function so confidence behavior can be tested without Tauri, storage, or UI:

```rust
pub fn calculate_confidence(
    five_hour_remaining: Option<u64>,
    total_remaining: Option<u64>,
    parser_warnings: &[String],
) -> Confidence
```

`parser_warnings` should explain confidence reductions in user-facing language when possible. `High` confidence must not include warnings in v0.1.

## Required Behavior

- Unknown formats return a low-confidence snapshot, not an error or crash.
- Missing fields become `null`/`None`.
- Numeric fields are parsed as integer token counts.
- The parser output includes `source`, `confidence`, `parsed_at`, `raw_input_sha256`, and `parser_warnings`.
- Parser warnings should be specific enough for UI copy, such as "Reset time was not found."
- Fixtures under `fixtures/codex/` define expected parser scenarios.
