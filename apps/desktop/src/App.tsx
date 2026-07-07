import { useCallback, useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { PointerEvent } from "react";
import { Companion } from "./components/Pet";
import { Progress } from "./components/Progress";
import { StatusImport } from "./components/StatusImport";
import { ParseDiagnostics } from "./components/ParseDiagnostics";
import { SourceStatus, type CodexSourceStatus } from "./components/SourceStatus";
import { AutoUsagePanel, type LocalCodexUsageSnapshot } from "./components/AutoUsagePanel";
import { RateLimitPanel, type CodexRateLimitSnapshot } from "./components/RateLimitPanel";
import { WindowControls } from "./components/WindowControls";
import { useQuota } from "./hooks/useQuota";
import { useMood } from "./hooks/useMood";
import type { Mood, QuotaSnapshot } from "./store/quotaStore";

type WindowLayoutState = {
  panelAbove: boolean;
  dockEdge: "left" | "right" | null;
};

type BitActivity = "idle" | "scan" | "type" | "snack";
type ViteImportMeta = ImportMeta & {
  env?: {
    VITE_SHOW_DEBUG_TOOLS?: string;
  };
};

const LAST_CELEBRATED_RESET_KEY = "token-tamagotchi:last-celebrated-five-hour-reset";
const AUTO_FOLD_BODY_MODE = false;
const VITE_ENV = (import.meta as ViteImportMeta).env;
const SHOW_DEBUG_TOOLS = VITE_ENV?.VITE_SHOW_DEBUG_TOOLS === "true";
const MOOD_TEST_CASES = [
  { label: "Happy", percent: 95 },
  { label: "Relaxed", percent: 65 },
  { label: "Concerned", percent: 30 },
  { label: "Panicking", percent: 12 },
  { label: "Exhausted", percent: 3 },
  { label: "Unknown", percent: null }
] as const;

export function App() {
  const { snapshot, setSnapshot } = useQuota();
  const baseMood = useMood(snapshot);
  const [celebrationMood, setCelebrationMood] = useState<Mood | null>(null);
  const mood = celebrationMood ?? baseMood;
  const [isDetailsOpen, setIsDetailsOpen] = useState(false);
  const [isCompanionActive, setIsCompanionActive] = useState(false);
  const [isQuickStatsVisible, setIsQuickStatsVisible] = useState(false);
  const [isBodyMode, setIsBodyMode] = useState(false);
  const [bubbleNonce, setBubbleNonce] = useState(0);
  const [isBubbleVisible, setIsBubbleVisible] = useState(false);
  const [isFeeding, setIsFeeding] = useState(false);
  const [feedAnimationKey, setFeedAnimationKey] = useState(0);
  const [tokenMealPercent, setTokenMealPercent] = useState<number | null>(null);
  const [sourceStatus, setSourceStatus] = useState<CodexSourceStatus | null>(null);
  const [localUsage, setLocalUsage] = useState<LocalCodexUsageSnapshot | null>(null);
  const [rateLimits, setRateLimits] = useState<CodexRateLimitSnapshot | null>(null);
  const [rateLimitStatus, setRateLimitStatus] = useState<"checking" | "synced" | "offline">("checking");
  const [rateLimitError, setRateLimitError] = useState<string | null>(null);
  const [isPanelAbove, setIsPanelAbove] = useState(false);
  const [dockEdge, setDockEdge] = useState<"left" | "right" | null>(null);
  const dragStartRef = useRef<{ x: number; y: number } | null>(null);
  const dragClampTimersRef = useRef<number[]>([]);
  const nativeDragInFlightRef = useRef(false);
  const ignoreNextClickRef = useRef(false);
  const celebrationTimeoutRef = useRef<number | null>(null);
  const lastBodyToggleAtRef = useRef(0);
  const windowLayout = isDetailsOpen ? "details" : isQuickStatsVisible ? "food" : isBodyMode || isBubbleVisible ? "body" : "compact";
  const previousWindowLayoutRef = useRef(windowLayout);
  const previousPanelAboveRef = useRef(isPanelAbove);
  const previousMoodRef = useRef(mood);
  const previousBaseMoodRef = useRef(baseMood);
  const previousFiveHourPercentRef = useRef<number | null>(null);
  const previousFiveHourUsedTokensRef = useRef<number | null>(null);
  const feedingTimeoutRef = useRef<number | null>(null);

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
    }, 3_200);

    return () => window.clearTimeout(timeout);
  }, [bubbleNonce, isBubbleVisible]);

  useEffect(() => {
    if (previousMoodRef.current === mood) {
      return;
    }

    previousMoodRef.current = mood;
    setIsBubbleVisible(true);
    setBubbleNonce((value) => value + 1);
  }, [mood]);

  useEffect(() => {
    const previousMood = previousBaseMoodRef.current;
    previousBaseMoodRef.current = baseMood;

    const recoveredFromPressure =
      (previousMood === "panicking" || previousMood === "exhausted") &&
      (baseMood === "relaxed" || baseMood === "happy");

    if (!recoveredFromPressure) {
      return;
    }

    triggerCelebration();
  }, [baseMood]);

  useEffect(() => {
    if (!rateLimits?.available || rateLimits.fiveHourRemainingPercent === null || rateLimits.fiveHourRemainingPercent < 100) {
      return;
    }

    const resetKey = rateLimits.fiveHourResetAt ?? `observed:${rateLimits.observedAt}`;

    try {
      if (window.localStorage.getItem(LAST_CELEBRATED_RESET_KEY) === resetKey) {
        return;
      }

      window.localStorage.setItem(LAST_CELEBRATED_RESET_KEY, resetKey);
    } catch (error) {
      console.warn("Failed to persist reset celebration marker", error);
    }

    triggerCelebration();
  }, [rateLimits]);

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
        setDockEdge(state.dockEdge);
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
      clearCelebrationTimer();
      clearFeedingTimer();
    };
  }, []);

  useEffect(() => {
    if (!AUTO_FOLD_BODY_MODE || !isBodyMode || isQuickStatsVisible) {
      return;
    }

    const timeout = window.setTimeout(() => {
      setIsBodyMode(false);
    }, 90_000);

    return () => window.clearTimeout(timeout);
  }, [isBodyMode, isQuickStatsVisible]);

  function triggerCelebration() {
    clearCelebrationTimer();
    setCelebrationMood("celebrating");
    setIsBubbleVisible(true);
    setBubbleNonce((value) => value + 1);

    celebrationTimeoutRef.current = window.setTimeout(() => {
      setCelebrationMood(null);
      celebrationTimeoutRef.current = null;
    }, 2_800);
  }

  function triggerTokenMeal(deltaPercent: number) {
    clearFeedingTimer();
    setIsFeeding(false);
    setFeedAnimationKey((value) => value + 1);
    setTokenMealPercent(deltaPercent);
    setIsBubbleVisible(true);
    setBubbleNonce((value) => value + 1);

    window.requestAnimationFrame(() => {
      setIsFeeding(true);
    });

    feedingTimeoutRef.current = window.setTimeout(() => {
      setIsFeeding(false);
      setTokenMealPercent(null);
      feedingTimeoutRef.current = null;
    }, 2_200);
  }

  function clearFeedingTimer() {
    if (feedingTimeoutRef.current === null) {
      return;
    }

    window.clearTimeout(feedingTimeoutRef.current);
    feedingTimeoutRef.current = null;
  }

  function clearCelebrationTimer() {
    if (celebrationTimeoutRef.current === null) {
      return;
    }

    window.clearTimeout(celebrationTimeoutRef.current);
    celebrationTimeoutRef.current = null;
  }

  function interactWithCompanion() {
    if (ignoreNextClickRef.current) {
      ignoreNextClickRef.current = false;
      return;
    }

    const now = window.performance.now();
    const shouldToggleBody = !isQuickStatsVisible && now - lastBodyToggleAtRef.current > 300;

    if (shouldToggleBody) {
      lastBodyToggleAtRef.current = now;
      setIsBodyMode((value) => !value);
    }

    if (dockEdge !== null) {
      setDockEdge(null);
    }

    setIsCompanionActive(true);
    setIsBubbleVisible(true);
    setBubbleNonce((value) => value + 1);
    window.setTimeout(() => setIsCompanionActive(false), 420);
  }

  function toggleFoodMeters() {
    if (ignoreNextClickRef.current) {
      ignoreNextClickRef.current = false;
      return;
    }

    setDockEdge(null);
    setIsQuickStatsVisible((value) => {
      if (value) {
        setIsDetailsOpen(false);
        setIsBodyMode(true);
        setIsBubbleVisible(true);
        setBubbleNonce((current) => current + 1);
      } else {
        setIsBodyMode(false);
      }

      return !value;
    });
  }

  function applyMoodTest(percent: number | null) {
    setSnapshot(createMoodTestSnapshot(percent));
    setIsBodyMode(true);
    setIsQuickStatsVisible(true);
    setIsDetailsOpen(true);
  }

  function startPossibleWindowDrag(event: PointerEvent<HTMLElement>) {
    if (event.button !== 0) {
      return;
    }

    dragStartRef.current = { x: event.clientX, y: event.clientY };
  }

  function continuePossibleWindowDrag(event: PointerEvent<HTMLElement>) {
    const start = dragStartRef.current;

    if (!start || nativeDragInFlightRef.current) {
      return;
    }

    const deltaX = Math.abs(event.clientX - start.x);
    const deltaY = Math.abs(event.clientY - start.y);

    if (deltaX < 8 && deltaY < 8) {
      return;
    }

    dragStartRef.current = null;
    ignoreNextClickRef.current = true;
    window.setTimeout(() => {
      ignoreNextClickRef.current = false;
    }, 450);
    clearDragClampTimers();
    nativeDragInFlightRef.current = true;
    void invoke("start_window_drag")
      .catch((error) => {
        console.error("Failed to start window drag", error);
      })
      .finally(() => {
        nativeDragInFlightRef.current = false;
        scheduleWindowConstraint([0, 180]);
      });
  }

  function stopPossibleWindowDrag() {
    dragStartRef.current = null;

    if (nativeDragInFlightRef.current) {
      return;
    }

    scheduleWindowConstraint([120]);
  }

  function cancelPossibleWindowDrag() {
    dragStartRef.current = null;

    if (!nativeDragInFlightRef.current) {
      scheduleWindowConstraint([120]);
    }
  }

  function scheduleWindowConstraint(delays: number[]) {
    clearDragClampTimers();

    for (const delay of delays) {
      const timer = window.setTimeout(() => {
        void invoke<WindowLayoutState>("constrain_window_to_screen", { layout: windowLayout })
          .then((state) => {
            setIsPanelAbove(state.panelAbove);
            setDockEdge(state.dockEdge);
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

  const fiveHourRemainingPercent =
    rateLimits?.fiveHourRemainingPercent ?? percentageFromSnapshot(snapshot.fiveHourRemaining, snapshot.fiveHourLimit);

  useEffect(() => {
    const previousPercent = previousFiveHourPercentRef.current;
    previousFiveHourPercentRef.current = fiveHourRemainingPercent;

    if (previousPercent === null || fiveHourRemainingPercent === null) {
      return;
    }

    const consumedPercent = Math.round(previousPercent - fiveHourRemainingPercent);

    if (consumedPercent <= 0) {
      return;
    }

    triggerTokenMeal(consumedPercent);
  }, [fiveHourRemainingPercent]);

  useEffect(() => {
    if (!localUsage?.available) {
      return;
    }

    const currentUsedTokens = localUsage.fiveHourUsed;
    const previousUsedTokens = previousFiveHourUsedTokensRef.current;
    previousFiveHourUsedTokensRef.current = currentUsedTokens;

    if (previousUsedTokens === null) {
      return;
    }

    const consumedTokens = currentUsedTokens - previousUsedTokens;

    if (consumedTokens <= 0) {
      return;
    }

    triggerTokenMeal(1);
  }, [localUsage]);

  const alert = quotaAlertFor(fiveHourRemainingPercent);
  const isBodyVisible = windowLayout !== "compact";
  const bitActivity: BitActivity = isFeeding ? "snack" : "idle";
  const companion = (
    <section
      className="companion-stage"
      aria-label="Desktop companion"
      onPointerDown={startPossibleWindowDrag}
      onPointerMove={continuePossibleWindowDrag}
      onPointerUp={stopPossibleWindowDrag}
      onPointerCancel={cancelPossibleWindowDrag}
    >
      <div className="companion-idle-cluster">
        <Companion
          mood={mood}
          isActive={isCompanionActive}
          activity={mood === "panicking" || mood === "exhausted" ? "idle" : bitActivity}
          isFeeding={isFeeding}
          feedAnimationKey={feedAnimationKey}
          isStatusVisible={isBubbleVisible}
          isStatusExpanded={isQuickStatsVisible}
          statusMessage={statusMessageFor(fiveHourRemainingPercent, tokenMealPercent)}
          onInteract={interactWithCompanion}
          onStatusClick={toggleFoodMeters}
        />
      </div>
    </section>
  );
  const companionInfo = isQuickStatsVisible ? (
    <section className="companion-panel" aria-label="Companion status panel">
      {isQuickStatsVisible ? (
        <div className="companion-panel-header">
          <span>[Quota]</span>
          <WindowControls variant="mini" />
        </div>
      ) : null}
      {isQuickStatsVisible && alert ? (
        <section className={`quota-alert quota-alert-${alert.level}`} aria-label="Quota alert">
          <span>{alert.status}</span>
          <strong>{alert.message}</strong>
        </section>
      ) : null}
      {isQuickStatsVisible ? (
        <>
          <section className="quota-panel" aria-label="Quota status">
            <Progress label="5h token food" value={snapshot.fiveHourRemaining} max={snapshot.fiveHourLimit} prominence="primary" />
            <Progress label="7d reserve" value={snapshot.totalRemaining} max={snapshot.totalLimit} />
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
                {SHOW_DEBUG_TOOLS ? (
                  <details className="debug-section">
                    <summary>[Debug]</summary>
                    <div className="debug-section-body">
                      <section className="mood-test-panel" aria-label="Mood test controls">
                        <div className="mood-test-header">
                          <span>[Mood Test]</span>
                          <span>dev only</span>
                        </div>
                        <div className="mood-test-grid">
                          {MOOD_TEST_CASES.map((testCase) => (
                            <button
                              key={testCase.label}
                              className="mood-test-button"
                              type="button"
                              onClick={() => applyMoodTest(testCase.percent)}
                            >
                              {testCase.label}
                            </button>
                          ))}
                          <button
                            className="mood-test-button mood-test-button-celebrate"
                            type="button"
                            onClick={() => {
                              applyMoodTest(100);
                              triggerCelebration();
                            }}
                          >
                            Celebrating
                          </button>
                          <button
                            className="mood-test-button mood-test-button-feed"
                            type="button"
                            onClick={() => {
                              setIsBodyMode(true);
                              setIsQuickStatsVisible(true);
                              triggerTokenMeal(3);
                            }}
                          >
                            Feed Token
                          </button>
                        </div>
                      </section>
                      <SourceStatus status={sourceStatus} />
                      <AutoUsagePanel usage={localUsage} />
                      <StatusImport onParsed={setSnapshot} />
                      <ParseDiagnostics snapshot={snapshot} />
                    </div>
                  </details>
                ) : null}
              </div>
            ) : null}
          </section>
        </>
      ) : null}
    </section>
  ) : null;

  return (
    <main className={`app-shell app-shell-${windowLayout} ${isBodyVisible ? "app-shell-pose-body" : "app-shell-pose-head"} ${isDetailsOpen ? "app-shell-expanded" : "app-shell-compact"} ${dockEdge ? `app-shell-docked-${dockEdge}` : ""} ${isPanelAbove ? "app-shell-panel-above" : "app-shell-panel-below"}`}>
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

function percentageFromSnapshot(remaining: number | null, limit: number | null) {
  if (remaining === null || limit === null || limit <= 0) {
    return null;
  }

  return Math.max(0, Math.min(100, (remaining / limit) * 100));
}

function statusMessageFor(remainingPercent: number | null, tokenMealPercent: number | null) {
  if (tokenMealPercent !== null) {
    return "nom nom...";
  }

  if (remainingPercent === null) {
    return "5H --%";
  }

  return `5H ${Math.round(remainingPercent)}%`;
}

function createMoodTestSnapshot(percent: number | null): QuotaSnapshot {
  const limit = percent === null ? null : 100;

  return {
    fiveHourRemaining: percent,
    fiveHourLimit: limit,
    totalRemaining: percent,
    totalLimit: limit,
    resetAt: percent === null ? null : new Date(Date.now() + 60 * 60 * 1000).toISOString(),
    source: "mock",
    confidence: percent === null ? "low" : "high",
    parsedAt: new Date().toISOString(),
    rawInputSha256: null,
    parserWarnings: percent === null ? ["Mood test: missing quota fields."] : [`Mood test: forced ${percent}% 5h quota.`]
  };
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
