# Product Requirement Document (PRD)

## Target Audience

Codex users who want a playful, non-intrusive desktop companion for monitoring 5-hour limits, 7-day usage pressure, and quota health while coding.

## v0.2 Product Requirements

1. **Automatic Codex Quota Reading**
   Read local Codex rate-limit state through `codex app-server --stdio` and `account/rateLimits/read` without asking users to enter quota totals manually.

2. **5-Hour And 7-Day Food Meters**
   Show remaining 5-hour quota and 7-day quota as compact token food meters. The 5-hour meter drives the companion mood.

3. **Auto Refresh**
   Refresh quota state on an interval and provide a manual refresh action inside Details.

4. **Desktop Companion Default**
   The default surface should show only the companion. Clicking the companion shows a temporary status message and the control to expand quota meters.

5. **Mood Feedback**
   Convert quota thresholds into companion mood, expression, color, idle animation, and short status copy.

6. **Details And Debug**
   Details should expose used tokens, estimated totals, reset timing, source state, and parser/manual diagnostics without making manual input the primary flow.

7. **Local Persistence**
   Store parsed quota snapshots in SQLite. Raw command output is local-only and opt-in.

8. **Window Behavior**
   Support transparent/borderless, always-on-top, draggable desktop companion behavior. Expanded panels should remain reachable near screen edges.

9. **Platform Target**
   Primary development target is Windows. The architecture should remain compatible with macOS and Linux through Tauri.

## Non-Goals For v0.2

- Auto-scraping browser dashboards.
- Asking users to manually provide true quota totals as the main path.
- API key management.
- Cloud sync or telemetry.
- AI prediction or task recommendations.
- Multi-provider UI beyond the internal provider contract.
- Full skin marketplace or advanced animation tooling.
- True 3D/voxel living companion behavior.

## Acceptance Criteria

- App reads Codex local app-server quota state automatically.
- App presents 5-hour and 7-day remaining quota as food meters.
- Companion mood changes when 5-hour quota thresholds change.
- Clicking the companion shows status copy and the quota-meter toggle.
- Details exposes usage, estimated totals, reset timing, and debug information.
- Manual import remains available for testing and parser diagnostics.
- App remains local-first and does not upload quota or usage data.
