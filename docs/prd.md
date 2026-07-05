# Product Requirement Document (PRD)

## Target Audience

Codex users who want a playful, non-intrusive desktop companion for monitoring 5-hour limits, total remaining usage, and quota pressure while coding.

## MVP Features

1. **Status Import**
   Manual paste/import of user-provided Codex CLI `/status` output.

2. **Quota Parsing**
   Parse 5-hour remaining quota, total remaining quota, reset timing, source, timestamp, and parser confidence into `QuotaSnapshot`.

3. **Quota Persistence**
   Store parsed quota snapshots in SQLite. Raw command output is local-only and opt-in.

4. **Visual Feedback**
   Show a floating desktop companion representing quota health through mood, color/expression, a token food meter, and short status copy.

5. **Manual/Mock Fallback**
   Support mock and manual quota providers so the app remains useful before Codex parsing is fully stable.

6. **Auto Refresh**
   Refresh displayed quota state and stale-data indicators on an interval. Automatic Codex command execution remains out of scope for v0.1 unless explicitly user-configured later.

7. **Platform Target**
   Primary development target is Windows. The architecture should remain compatible with macOS and Linux through Tauri.

## Non-Goals (v0.1)

- Auto-scraping browser dashboards.
- Automatically executing Codex commands.
- API key management.
- Cloud sync or telemetry.
- AI prediction or task recommendations.
- Multi-provider support beyond the internal provider contract.
- Full skin marketplace or advanced animation tooling.

## MVP Acceptance Criteria

- User can paste a Codex `/status` sample.
- App produces a normalized `QuotaSnapshot`.
- Snapshot is persisted locally.
- Companion mood changes when quota thresholds change.
- UI presents quota as a token food meter.
- Unknown or unsupported input fails gracefully without crashing.
- No network access is required for quota monitoring.
