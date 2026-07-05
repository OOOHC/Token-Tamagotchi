import { Pet } from "./components/Pet";
import { Progress } from "./components/Progress";
import { Bubble } from "./components/Bubble";
import { useQuota } from "./hooks/useQuota";
import { useMood } from "./hooks/useMood";

export function App() {
  const quota = useQuota();
  const mood = useMood(quota.snapshot);

  return (
    <main className="app-shell">
      <Pet mood={mood} />
      <Bubble mood={mood} />
      <section className="quota-panel" aria-label="Quota status">
        <Progress label="5h" value={quota.snapshot.fiveHourRemaining} max={quota.snapshot.fiveHourLimit} />
        <Progress label="Total" value={quota.snapshot.totalRemaining} max={quota.snapshot.totalLimit} />
      </section>
    </main>
  );
}

