import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { QuotaSnapshot } from "../store/quotaStore";

type ImportSnapshotResponse = {
  snapshot: QuotaSnapshot;
  saveOutcome: "inserted" | "duplicate";
};

type StatusImportProps = {
  onParsed: (snapshot: QuotaSnapshot) => void;
};

export function StatusImport({ onParsed }: StatusImportProps) {
  const [rawText, setRawText] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);
  const [isParsing, setIsParsing] = useState(false);

  async function parseStatusText() {
    if (!rawText.trim()) {
      setError("[Status]: Paste Codex quota text first.");
      return;
    }

    setIsParsing(true);
    setError(null);
    setStatus(null);

    try {
      const response = await invoke<ImportSnapshotResponse>("parse_status_text", { rawText });
      onParsed(response.snapshot);
      setStatus(
        response.saveOutcome === "duplicate"
          ? "[Status]: Duplicate snapshot ignored."
          : "[Status]: Snapshot saved."
      );
      setRawText("");
    } catch (error) {
      setError(error instanceof Error ? error.message : String(error));
    } finally {
      setIsParsing(false);
    }
  }

  return (
    <section className="import-panel" aria-label="Import Codex status">
      <textarea
        className="status-input"
        value={rawText}
        onChange={(event) => setRawText(event.target.value)}
        placeholder="5h remaining: 120,000&#10;5h limit: 150,000&#10;total remaining: 1,200,000&#10;total limit: 1,500,000"
        spellCheck={false}
      />
      <div className="import-actions">
        <button className="import-button" type="button" onClick={parseStatusText} disabled={isParsing}>
          {isParsing ? "Parsing" : "Feed"}
        </button>
        {status ? <span className="import-status">{status}</span> : null}
        {error ? <span className="import-error">{error}</span> : null}
      </div>
    </section>
  );
}
