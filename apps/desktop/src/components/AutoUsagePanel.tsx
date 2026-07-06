export type LocalCodexUsageSnapshot = {
  available: boolean;
  fiveHourUsed: number;
  weeklyUsed: number;
  totalUsed: number | null;
  latestUsageTotalTokens: number | null;
  primaryResetAt: string | null;
  secondaryResetAt: string | null;
  limitReached: boolean | null;
  observedAt: string;
  warnings: string[];
};

type AutoUsagePanelProps = {
  usage: LocalCodexUsageSnapshot | null;
};

export function AutoUsagePanel({ usage }: AutoUsagePanelProps) {
  return (
    <section className="auto-usage-panel" aria-label="Automatic local usage">
      <div className="auto-usage-header">
        <span>[Auto]: Local Codex logs</span>
        <span>{usage?.available ? "online" : "checking"}</span>
      </div>
      {usage ? (
        <div className="auto-usage-readout">
          <div>
            <span>5h used</span>
            <strong>{formatNumber(usage.fiveHourUsed)}</strong>
          </div>
          <div>
            <span>latest</span>
            <strong>{usage.latestUsageTotalTokens === null ? "unknown" : formatNumber(usage.latestUsageTotalTokens)}</strong>
          </div>
          <div>
            <span>7d used</span>
            <strong>{formatNumber(usage.weeklyUsed)}</strong>
          </div>
          <div>
            <span>lifetime</span>
            <strong>{usage.totalUsed === null ? "unknown" : formatNumber(usage.totalUsed)}</strong>
          </div>
          <div>
            <span>reset</span>
            <strong>{usage.primaryResetAt ? formatDate(usage.primaryResetAt) : "unknown"}</strong>
          </div>
        </div>
      ) : null}
      {usage?.warnings.length ? (
        <ul className="auto-usage-warnings">
          {usage.warnings.slice(0, 2).map((warning) => (
            <li key={warning}>{warning}</li>
          ))}
        </ul>
      ) : null}
    </section>
  );
}

function formatNumber(value: number) {
  return new Intl.NumberFormat("en-US").format(value);
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}
