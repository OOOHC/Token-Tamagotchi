import { useEffect, useState } from "react";
import type { QuotaSnapshot } from "../store/quotaStore";

type ParseDiagnosticsProps = {
  snapshot: QuotaSnapshot;
};

const confidenceCopy: Record<QuotaSnapshot["confidence"], string> = {
  high: "[Parse]: High confidence.",
  medium: "[Parse]: Partial confidence.",
  low: "[Parse]: Low confidence."
};

export function ParseDiagnostics({ snapshot }: ParseDiagnosticsProps) {
  const [, refreshAge] = useState(0);
  const warnings = snapshot.parserWarnings.slice(0, 3);
  const dataAge = ageInMinutes(snapshot.parsedAt);
  const isStale = dataAge !== null && dataAge >= 30;

  useEffect(() => {
    const interval = window.setInterval(() => {
      refreshAge((value) => value + 1);
    }, 60_000);

    return () => window.clearInterval(interval);
  }, []);

  return (
    <section
      className={`diagnostics diagnostics-${snapshot.confidence}${isStale ? " diagnostics-stale" : ""}`}
      aria-label="Parse diagnostics"
    >
      <div className="diagnostics-header">
        <span>{confidenceCopy[snapshot.confidence]}</span>
        <span>{isStale ? "[Data]: stale" : snapshot.source}</span>
      </div>
      <div className="diagnostics-meta">
        <span>{formatAge(dataAge, snapshot.parsedAt)}</span>
        {snapshot.resetAt ? <span>reset {snapshot.resetAt}</span> : null}
      </div>
      {warnings.length > 0 ? (
        <ul className="diagnostics-warnings">
          {warnings.map((warning) => (
            <li key={warning}>{warning}</li>
          ))}
        </ul>
      ) : (
        <p className="diagnostics-clean">[Warnings]: none.</p>
      )}
    </section>
  );
}

function ageInMinutes(value: string) {
  const parsed = new Date(value);

  if (Number.isNaN(parsed.getTime())) {
    return null;
  }

  return Math.max(0, Math.floor((Date.now() - parsed.getTime()) / 60_000));
}

function formatAge(age: number | null, fallback: string) {
  if (age === null) {
    return `parsed ${fallback}`;
  }

  if (age < 1) {
    return "parsed just now";
  }

  return `parsed ${age}m ago`;
}
