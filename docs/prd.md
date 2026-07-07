# Product Requirement Document (PRD)

## Target Audience

Codex users who want a playful, non-intrusive desktop companion for monitoring 5-hour limits, 7-day usage pressure, and quota health while coding.

## Current Product Requirements

1. **Automatic Codex Quota Reading**
   Read local Codex rate-limit state through `codex app-server --stdio` and `account/rateLimits/read` without asking users to enter quota totals manually.

2. **5-Hour And 7-Day Food Meters**
   Show remaining 5-hour quota as token food and 7-day quota as reserve. Both meters use the same mood color thresholds as the companion.

3. **Auto Refresh**
   Refresh quota state on an interval and provide a manual refresh action inside Details.

4. **Desktop Companion Default**
   The default surface should show Bit as a desktop companion with a compact 5H status HUD. Clicking the status HUD expands quota meters.

5. **Mood Feedback**
   Convert quota thresholds into companion mood, expression, color, idle animation, and short status copy.

6. **Token-Eating Feedback**
   When 5-hour remaining quota decreases, Bit should briefly eat token food and show a short feeding message such as `nom nom...`.

7. **Details Panel**
   Details should expose used tokens, estimated totals, reset timing, source state, sync state, and manual refresh.

8. **Developer Debug Tools**
   Mood tests, manual import, source checks, parser diagnostics, and token-feed test controls are developer tools. They should be hidden by default and shown only when explicitly enabled for local development.

9. **Local Persistence**
   Store parsed quota snapshots in SQLite. Raw command output is local-only and opt-in.

10. **Window Behavior**
   Support transparent/borderless, always-on-top, draggable desktop companion behavior. Expanded panels should remain reachable near screen edges.

11. **Platform Target**
   Primary development target is Windows. The architecture should remain compatible with macOS and Linux through Tauri.

## Non-Goals For Current Release

- Auto-scraping browser dashboards.
- Asking users to manually provide true quota totals as the main path.
- API key management.
- Cloud sync or telemetry.
- AI prediction or task recommendations.
- Multi-provider UI beyond the internal provider contract.
- Full skin marketplace or advanced animation tooling.
- True 3D rendering. The current companion is CSS/DOM-based with an industrial voxel visual style.

## Acceptance Criteria

- App reads Codex local app-server quota state automatically.
- App presents 5-hour and 7-day remaining quota as food meters.
- Companion mood changes when 5-hour quota thresholds change.
- Clicking the companion shows status copy and the quota-meter toggle.
- Details exposes usage, estimated totals, reset timing, sync state, and manual refresh.
- Debug tools are hidden by default and available only when explicitly enabled for local development.
- Bit displays token-eating feedback when quota decreases.
- App remains local-first and does not upload quota or usage data.
