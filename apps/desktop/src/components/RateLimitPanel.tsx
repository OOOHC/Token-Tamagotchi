import type { LocalCodexUsageSnapshot } from "./AutoUsagePanel";

export type CodexRateLimitSnapshot = {
  available: boolean;
  limitId: string | null;
  planType: string | null;
  fiveHourUsedPercent: number | null;
  fiveHourRemainingPercent: number | null;
  fiveHourResetAt: string | null;
  weeklyUsedPercent: number | null;
  weeklyRemainingPercent: number | null;
  weeklyResetAt: string | null;
  rateLimitReachedType: string | null;
  observedAt: string;
  warnings: string[];
};

type RateLimitPanelProps = {
  rateLimits: CodexRateLimitSnapshot | null;
  usage: LocalCodexUsageSnapshot | null;
  refreshStatus: "checking" | "synced" | "offline";
  refreshError: string | null;
};

export function RateLimitPanel({ rateLimits, usage, refreshStatus, refreshError }: RateLimitPanelProps) {
  const fiveHourTotal = estimateTotalTokens(usage?.fiveHourUsed ?? null, rateLimits?.fiveHourUsedPercent ?? null);
  const weeklyTotal = estimateTotalTokens(usage?.weeklyUsed ?? null, rateLimits?.weeklyUsedPercent ?? null);

  return (
    <section className="rate-limit-panel" aria-label="Codex official rate limits">
      <div className="auto-usage-header">
        <span>[Auto]: Codex rate limits</span>
        <span>{refreshStatus}</span>
      </div>
      {rateLimits ? (
        <div className="auto-usage-readout">
          <div>
            <span>5h remain</span>
            <strong>{formatPercent(rateLimits.fiveHourRemainingPercent)}</strong>
          </div>
          <div>
            <span>5h used</span>
            <strong>{formatUsage(usage?.fiveHourUsed ?? null, rateLimits.fiveHourUsedPercent)}</strong>
          </div>
          <div>
            <span>5h total</span>
            <strong>{formatEstimate(fiveHourTotal)}</strong>
          </div>
          <div>
            <span>7d remain</span>
            <strong>{formatPercent(rateLimits.weeklyRemainingPercent)}</strong>
          </div>
          <div>
            <span>7d used</span>
            <strong>{formatUsage(usage?.weeklyUsed ?? null, rateLimits.weeklyUsedPercent)}</strong>
          </div>
          <div>
            <span>7d total</span>
            <strong>{formatEstimate(weeklyTotal)}</strong>
          </div>
          <div>
            <span>5h reset</span>
            <strong>{rateLimits.fiveHourResetAt ? formatDate(rateLimits.fiveHourResetAt) : "unknown"}</strong>
          </div>
          <div>
            <span>7d reset</span>
            <strong>{rateLimits.weeklyResetAt ? formatDate(rateLimits.weeklyResetAt) : "unknown"}</strong>
          </div>
          <div>
            <span>updated</span>
            <strong>{formatDate(rateLimits.observedAt)}</strong>
          </div>
        </div>
      ) : null}
      {refreshError ? <p className="auto-usage-error">{refreshError}</p> : null}
      {rateLimits?.warnings.length ? (
        <ul className="auto-usage-warnings">
          {rateLimits.warnings.slice(0, 2).map((warning) => (
            <li key={warning}>{warning}</li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function formatPercent(value: number | null) {
  return value === null ? "unknown" : `${value}%`;
}

function formatUsage(tokens: number | null, percent: number | null) {
  const tokenText = tokens === null ? "unknown" : formatNumber(tokens);
  const percentText = percent === null ? "" : ` (${percent}%)`;

  return `${tokenText}${percentText}`;
}

function estimateTotalTokens(usedTokens: number | null, usedPercent: number | null) {
  if (usedTokens === null || usedPercent === null || usedPercent <= 0) {
    return null;
  }

  return Math.round((usedTokens / usedPercent) * 100);
}

function formatEstimate(value: number | null) {
  return value === null ? "unknown" : `~${formatNumber(value)}`;
}

function formatNumber(value: number) {
  return new Intl.NumberFormat("en-US").format(value);
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  const now = new Date();
  const isToday =
    date.getFullYear() === now.getFullYear() &&
    date.getMonth() === now.getMonth() &&
    date.getDate() === now.getDate();

  if (isToday) {
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  }

  return date.toLocaleString([], {
    month: "short",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit"
  });
}
