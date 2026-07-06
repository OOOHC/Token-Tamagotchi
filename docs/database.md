# Database Schema (SQLite)

## Design Principle: Privacy Isolation

To honor the privacy commitment, raw CLI inputs are decoupled from historical analytics. Sensitive raw data is stored in a separate table that is only populated if the user explicitly opts into history/debugging features.

## Tables

### quota_snapshots

Default persistence for normalized quota state. This table is safe for general trend analysis.

| Column | Type | Notes |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | Local row id |
| five_hour_remaining | INTEGER NULL | Parsed token count |
| five_hour_limit | INTEGER NULL | Parsed token count |
| total_remaining | INTEGER NULL | Parsed token count |
| total_limit | INTEGER NULL | Parsed token count |
| reset_at | TEXT NULL | Source-provided reset timestamp |
| source | TEXT NOT NULL | `codex-app-server`, `codex-local`, `codex-cli`, `manual`, `mock` |
| confidence | TEXT NOT NULL | `high`, `medium`, `low` |
| raw_input_sha256 | TEXT NULL | Deduplication key; may link to `raw_imports_optional` when raw history is enabled |
| parser_warnings_json | TEXT NOT NULL | JSON array of parser warnings |
| parsed_at | TEXT NOT NULL | RFC3339 parser timestamp |
| created_at | TEXT NOT NULL | System insert timestamp |

### settings

Stores local app preferences.

| Column | Type | Notes |
| --- | --- | --- |
| key | TEXT PRIMARY KEY | Setting name |
| value_json | TEXT NOT NULL | JSON-encoded setting value |
| updated_at | TEXT NOT NULL | Last update timestamp |

### raw_imports_optional

Opt-in storage for sensitive raw text. This table must not be populated unless the `raw_history_enabled` setting is true.

| Column | Type | Notes |
| --- | --- | --- |
| id | INTEGER PRIMARY KEY | Local row id |
| raw_input_sha256 | TEXT UNIQUE NOT NULL | SHA-256 hash of raw input |
| raw_text | TEXT NOT NULL | Full CLI raw output |
| source | TEXT NOT NULL | Import source |
| imported_at | TEXT NOT NULL | Import timestamp |

## Idempotency Policy

- Compute `raw_input_sha256` before parsing or storing an import.
- `quota_snapshots.raw_input_sha256` has a unique partial index when present.
- If the same raw input is imported twice, the app ignores the duplicate by default.
- Duplicate override for high-frequency polling is future scope.

## Raw Text Policy

- Raw command output is never stored in `quota_snapshots`.
- Raw text storage is local-only and opt-in.
- Raw text must be stored separately in `raw_imports_optional`.
- Disabling raw history should prevent future writes to `raw_imports_optional`; deletion behavior can be defined when the settings UI exists.
