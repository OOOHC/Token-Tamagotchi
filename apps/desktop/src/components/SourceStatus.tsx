export type DataSourceMode = "manual-import" | "auto-capable" | "cli-missing";

export type CodexSourceStatus = {
  cliDetected: boolean;
  version: string | null;
  statusCommandAvailable: boolean;
  usageCommandAvailable: boolean;
  mode: DataSourceMode;
  message: string;
};

type SourceStatusProps = {
  status: CodexSourceStatus | null;
};

const modeCopy: Record<DataSourceMode, string> = {
  "manual-import": "[Source]: Manual import",
  "auto-capable": "[Source]: Auto capable",
  "cli-missing": "[Source]: CLI missing"
};

export function SourceStatus({ status }: SourceStatusProps) {
  if (!status) {
    return (
      <section className="source-status source-status-loading" aria-label="Data source status">
        <span>[Source]: checking...</span>
      </section>
    );
  }

  return (
    <section className={`source-status source-status-${status.mode}`} aria-label="Data source status">
      <div className="source-status-header">
        <span>{modeCopy[status.mode]}</span>
        <span>{status.version ?? "codex unknown"}</span>
      </div>
      <p>{status.message}</p>
      <div className="source-status-flags">
        <span>status: {status.statusCommandAvailable ? "yes" : "no"}</span>
        <span>usage: {status.usageCommandAvailable ? "yes" : "no"}</span>
      </div>
    </section>
  );
}
