import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Companion } from "./components/Pet";
import { Progress } from "./components/Progress";
import { Bubble } from "./components/Bubble";
import { StatusImport } from "./components/StatusImport";
import { ParseDiagnostics } from "./components/ParseDiagnostics";
import { useQuota } from "./hooks/useQuota";
import { useMood } from "./hooks/useMood";
import type { QuotaSnapshot } from "./store/quotaStore";

export function App() {
  const { snapshot, setSnapshot } = useQuota();
  const mood = useMood(snapshot);

  useEffect(() => {
    let isMounted = true;

    invoke<QuotaSnapshot>("get_quota_snapshot")
      .then((snapshot) => {
        if (isMounted) {
          setSnapshot(snapshot);
        }
      })
      .catch(() => {
        // Keep the mock snapshot if the local store is unavailable.
      });

    return () => {
      isMounted = false;
    };
  }, [setSnapshot]);

  return (
    <main className="app-shell">
      <Companion mood={mood} />
      <Bubble mood={mood} />
      <section className="quota-panel" aria-label="Quota status">
        <Progress label="5h" value={snapshot.fiveHourRemaining} max={snapshot.fiveHourLimit} />
        <Progress label="Total" value={snapshot.totalRemaining} max={snapshot.totalLimit} />
      </section>
      <ParseDiagnostics snapshot={snapshot} />
      <StatusImport onParsed={setSnapshot} />
    </main>
  );
}
