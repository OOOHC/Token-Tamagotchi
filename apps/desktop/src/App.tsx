import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { PointerEvent } from "react";
import { Companion } from "./components/Pet";
import { Progress } from "./components/Progress";
import { Bubble } from "./components/Bubble";
import { StatusImport } from "./components/StatusImport";
import { ParseDiagnostics } from "./components/ParseDiagnostics";
import { SourceStatus, type CodexSourceStatus } from "./components/SourceStatus";
import { AutoUsagePanel, type LocalCodexUsageSnapshot } from "./components/AutoUsagePanel";
import { RateLimitPanel, type CodexRateLimitSnapshot } from "./components/RateLimitPanel";
import { WindowControls } from "./components/WindowControls";
import { useQuota } from "./hooks/useQuota";
import { useMood } from "./hooks/useMood";
import type { QuotaSnapshot } from "./store/quotaStore";

type WindowLayoutState = {
  panelAbove: boolean;
};

export function App() {
  const { snapshot, setSnapshot } = useQuota();
  const mood = useMood(snapshot);
  const [isDetailsOpen, setIsDetailsOpen] = useState(false);
  const [isCompanionActive, setIsCompanionActive] = useState(false);
  const [isFoodToggleVisible, setIsFoodToggleVisible] = useState(false);
  const [isQuickStatsVisible, setIsQuickStatsVisible] = useState(false);
  const [bubbleNonce, setBubbleNonce] = useState(0);
  const [isBubbleVisible, setIsBubbleVisible] = useState(false);
  const [sourceStatus, setSourceStatus] = useState<CodexSourceStatus | null>(null);
  const [localUsage, setLocalUsage] = useState<LocalCodexUsageSnapshot | null>(null);
  const [rateLimits, setRateLimits] = useState<CodexRateLimitSnapshot | null>(null);
  const [rateLimitStatus, setRateLimitStatus] = useState<"checking" | "synced" | "offline">("checking");
  const [rateLimitError, setRateLimitError] = useState<string | null>(null);
  const [isPanelAbove, setIsPanelAbove] = useState(false);
  const dragStartRef = useRef<{ x: number; y: number } | null>(null);
  const dragClampTimersRef = useRef<number[]>([]);
  const windowLayout = isDetailsOpen ? "details" : isQuickStatsVisible ? "food" : isBubbleVisible || isFoodToggleVisible ? "peek" : "compact";
  const previousWindowLayoutRef = useRef(windowLayout);
  const previousPanelAboveRef = useRef(isPanelAbove);

  const refreshRateLimits = useCallback(() => {
    setRateLimitStatus("checking");
    setRateLimitError(null);

    invoke<CodexRateLimitSnapshot>("get_codex_rate_limits")
      .then((limits) => {
        setRateLimits(limits);

        if (limits.available) {
          setRateLimitStatus("synced");
          setSnapshot(snapshotFromRateLimits(limits));
        } else {
          setRateLimitStatus("offline");
          setRateLimitError(limits.warnings[0] ?? "Codex rate limits are unavailable.");
        }
      })
      .catch((error) => {
        setRateLimitStatus("offline");
        setRateLimitError(String(error));
      });
  }, [setSnapshot]);

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

    invoke<{ dataSource: CodexSourceStatus }>("get_diagnostics")
      .then((diagnostics) => {
        if (isMounted) {
          setSourceStatus(diagnostics.dataSource);
        }
      })
      .catch(() => {
        if (isMounted) {
          setSourceStatus({
            cliDetected: false,
            version: null,
            statusCommandAvailable: false,
            usageCommandAvailable: false,
            mode: "cli-missing",
            message: "Codex CLI source check failed. Manual import remains available."
          });
        }
      });

    return () => {
      isMounted = false;
    };
  }, [setSnapshot]);

  useEffect(() => {
    let isMounted = true;

    refreshRateLimits();
    const interval = window.setInterval(() => {
      if (isMounted) {
        refreshRateLimits();
      }
    }, 30_000);

    return () => {
      isMounted = false;
      window.clearInterval(interval);
    };
  }, [refreshRateLimits]);

  useEffect(() => {
    let isMounted = true;

    function refreshLocalUsage() {
      invoke<LocalCodexUsageSnapshot>("get_local_codex_usage")
        .then((usage) => {
          if (!isMounted) {
            return;
          }

          setLocalUsage(usage);
        })
        .catch(() => {
          if (isMounted) {
            setLocalUsage(null);
          }
        });
    }

    refreshLocalUsage();
    const interval = window.setInterval(refreshLocalUsage, 30_000);

    return () => {
      isMounted = false;
      window.clearInterval(interval);
    };
  }, []);

  useEffect(() => {
    if (!isBubbleVisible) {
      return;
    }

    const timeout = window.setTimeout(() => {
      setIsBubbleVisible(false);
      if (!isQuickStatsVisible) {
        setIsFoodToggleVisible(false);
      }
    }, 3_200);

    return () => window.clearTimeout(timeout);
  }, [bubbleNonce, isBubbleVisible, isQuickStatsVisible]);

  useEffect(() => {
    const previousLayout = previousWindowLayoutRef.current;
    const previousPanelAbove = previousPanelAboveRef.current;

    void invoke<WindowLayoutState>("set_window_layout", {
      layout: windowLayout,
      previousLayout,
      previousPanelAbove
    })
      .then((state) => {
        setIsPanelAbove(state.panelAbove);
        previousWindowLayoutRef.current = windowLayout;
        previousPanelAboveRef.current = state.panelAbove;
      })
      .catch((error) => {
        console.error("Failed to set window layout", error);
      });
  }, [windowLayout]);

  useEffect(() => {
    return () => {
      clearDragClampTimers();
    };
  }, []);

  function interactWithCompanion() {
    setIsCompanionActive(true);
    setIsFoodToggleVisible(true);
    setIsBubbleVisible(true);
    setBubbleNonce((value) => value + 1);
    window.setTimeout(() => setIsCompanionActive(false), 420);
  }

  function toggleFoodMeters() {
    setIsQuickStatsVisible((value) => {
      if (value) {
        setIsDetailsOpen(false);
        setIsFoodToggleVisible(false);
      }

      return !value;
    });
  }

  function startPossibleWindowDrag(event: PointerEvent<HTMLElement>) {
    if (event.button !== 0) {
      return;
    }

    dragStartRef.current = { x: event.clientX, y: event.clientY };
  }

  function continuePossibleWindowDrag(event: PointerEvent<HTMLElement>) {
    const start = dragStartRef.current;

    if (!start) {
      return;
    }

    const deltaX = Math.abs(event.clientX - start.x);
    const deltaY = Math.abs(event.clientY - start.y);

    if (deltaX < 8 && deltaY < 8) {
      return;
    }

    dragStartRef.current = null;
    void invoke("start_window_drag").catch((error) => {
      console.error("Failed to start window drag", error);
    });
    scheduleWindowConstraint([120, 360, 900, 1_600, 2_800, 4_200]);
  }

  function stopPossibleWindowDrag() {
    dragStartRef.current = null;
    scheduleWindowConstraint([0, 160]);
  }

  function scheduleWindowConstraint(delays: number[]) {
    for (const delay of delays) {
      const timer = window.setTimeout(() => {
        void invoke<WindowLayoutState>("constrain_window_to_screen")
          .then((state) => {
            setIsPanelAbove(state.panelAbove);
            previousPanelAboveRef.current = state.panelAbove;
          })
          .catch((error) => {
            console.error("Failed to constrain window", error);
          });
      }, delay);

      dragClampTimersRef.current.push(timer);
    }
  }

  function clearDragClampTimers() {
    for (const timer of dragClampTimersRef.current) {
      window.clearTimeout(timer);
    }

    dragClampTimersRef.current = [];
  }

  const remainingPercent = rateLimits?.fiveHourRemainingPercent ?? snapshot.fiveHourRemaining;
  const alert = quotaAlertFor(remainingPercent);
  const companion = (
    <section
      className="companion-stage"
      aria-label="Desktop companion"
      onPointerDown={startPossibleWindowDrag}
      onPointerMove={continuePossibleWindowDrag}
      onPointerUp={stopPossibleWindowDrag}
      onPointerCancel={stopPossibleWindowDrag}
    >
      <div className="companion-idle-cluster">
        <Companion mood={mood} isActive={isCompanionActive} onInteract={interactWithCompanion} />
        {isFoodToggleVisible ? (
          <button
            className="food-toggle-button"
            type="button"
            aria-label={isQuickStatsVisible ? "Hide quota food meters" : "Show quota food meters"}
            aria-expanded={isQuickStatsVisible}
            onPointerDown={(event) => event.stopPropagation()}
            onClick={toggleFoodMeters}
          >
            {isQuickStatsVisible ? "-" : "+"}
          </button>
        ) : null}
      </div>
    </section>
  );
  const companionInfo = isBubbleVisible || isQuickStatsVisible ? (
    <section className="companion-panel" aria-label="Companion status panel">
      {isQuickStatsVisible ? (
        <div className="companion-panel-header">
          <span>[Quota]</span>
          <WindowControls variant="mini" />
        </div>
      ) : null}
      {isBubbleVisible ? <Bubble key={bubbleNonce} mood={mood} /> : null}
      {isQuickStatsVisible && alert ? (
        <section className={`quota-alert quota-alert-${alert.level}`} aria-label="Quota alert">
          <span>{alert.status}</span>
          <strong>{alert.message}</strong>
        </section>
      ) : null}
      {isQuickStatsVisible ? (
        <>
          <section className="quota-panel" aria-label="Quota status">
            <Progress label="5h food" value={snapshot.fiveHourRemaining} max={snapshot.fiveHourLimit} prominence="primary" />
            <Progress label="7d food" value={snapshot.totalRemaining} max={snapshot.totalLimit} />
          </section>
          <section className="utility-panel" aria-label="Companion controls">
            <button
              className="feed-toggle"
              type="button"
              aria-expanded={isDetailsOpen}
              onClick={() => setIsDetailsOpen((value) => !value)}
            >
              {isDetailsOpen ? "Hide Details" : "Details"}
            </button>
            {isDetailsOpen ? (
              <div className="utility-panel-body">
                <button
                  className="refresh-button"
                  type="button"
                  onClick={refreshRateLimits}
                  disabled={rateLimitStatus === "checking"}
                >
                  {rateLimitStatus === "checking" ? "Refreshing" : "Refresh Now"}
                </button>
                <RateLimitPanel
                  rateLimits={rateLimits}
                  usage={localUsage}
                  refreshStatus={rateLimitStatus}
                  refreshError={rateLimitError}
                />
                <details className="debug-section">
                  <summary>[Debug]</summary>
                  <div className="debug-section-body">
                    <SourceStatus status={sourceStatus} />
                    <AutoUsagePanel usage={localUsage} />
                    <StatusImport onParsed={setSnapshot} />
                    <ParseDiagnostics snapshot={snapshot} />
                  </div>
                </details>
              </div>
            ) : null}
          </section>
        </>
      ) : null}
    </section>
  ) : null;

  return (
    <main className={`app-shell app-shell-${windowLayout} ${isDetailsOpen ? "app-shell-expanded" : "app-shell-compact"} ${isPanelAbove ? "app-shell-panel-above" : "app-shell-panel-below"}`}>
      {isPanelAbove ? companionInfo : null}
      {companion}
      {isPanelAbove ? null : companionInfo}
    </main>
  );
}

function quotaAlertFor(remainingPercent: number | null) {
  if (remainingPercent === null) {
    return null;
  }

  if (remainingPercent <= 5) {
    return {
      level: "critical",
      status: "[Alert]: Exhausted",
      message: "Please avoid huge prompts until reset."
    };
  }

  if (remainingPercent <= 20) {
    return {
      level: "warning",
      status: "[Alert]: Low 5h food",
      message: "Suggestion: refactor or split the next prompt."
    };
  }

  return null;
}

function snapshotFromRateLimits(rateLimits: CodexRateLimitSnapshot): QuotaSnapshot {
  return {
    fiveHourRemaining: rateLimits.fiveHourRemainingPercent,
    fiveHourLimit: rateLimits.fiveHourRemainingPercent === null ? null : 100,
    totalRemaining: rateLimits.weeklyRemainingPercent,
    totalLimit: rateLimits.weeklyRemainingPercent === null ? null : 100,
    resetAt: rateLimits.fiveHourResetAt,
    source: "codex-app-server",
    confidence: rateLimits.available ? "high" : "low",
    parsedAt: rateLimits.observedAt,
    rawInputSha256: null,
    parserWarnings: rateLimits.warnings
  };
}
