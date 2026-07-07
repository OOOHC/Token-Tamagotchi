import type { Mood } from "../store/quotaStore";

type CompanionProps = {
  mood: Mood;
  isActive?: boolean;
  activity?: "idle" | "scan" | "type" | "snack";
  isFeeding?: boolean;
  feedAnimationKey?: number;
  isStatusVisible?: boolean;
  isStatusExpanded?: boolean;
  statusMessage?: string;
  onInteract?: () => void;
  onStatusClick?: () => void;
};

export function Companion({
  mood,
  isActive = false,
  activity = "idle",
  isFeeding = false,
  feedAnimationKey = 0,
  isStatusVisible = false,
  isStatusExpanded = false,
  statusMessage = "Quota stable.",
  onInteract,
  onStatusClick
}: CompanionProps) {
  return (
    <div
      className={`companion bit bit-${mood} bit-activity-${activity}${isFeeding ? " bit-feeding" : ""}${isActive ? " bit-active" : ""}${isStatusVisible ? " bit-status-visible" : ""}`}
      role="button"
      tabIndex={0}
      aria-label={`Bit companion mood: ${mood}`}
      onClick={onInteract}
    >
      <div className="bit-shadow" />
      <div className="bit-aura" />
      <button
        className="bit-holo-panel"
        type="button"
        aria-label={isStatusExpanded ? "Hide quota food meters" : "Show quota food meters"}
        aria-expanded={isStatusExpanded}
        onClick={(event) => {
          event.stopPropagation();
          onStatusClick?.();
        }}
      >
        <span>STATUS</span>
        <strong>{statusMessage}</strong>
      </button>
      <div className="bit-bot" aria-hidden="true">
        <div className="bit-head">
          <span className="bit-head-shell bit-head-shell-back" />
          <span className="bit-head-layer bit-head-layer-top" />
          <span className="bit-head-layer bit-head-layer-mid" />
          <span className="bit-ear bit-ear-left" />
          <span className="bit-ear bit-ear-right" />
          <div className="bit-screen">
            <span className="bit-screen-glass" />
            <span className="bit-eye bit-eye-left" />
            <span className="bit-eye bit-eye-right" />
            <span className="bit-mouth" />
            <span key={`mouth-token-${feedAnimationKey}`} className="bit-mouth-token" />
            <span className="bit-sleep-text">Zz...</span>
          </div>
        </div>
        <div className="bit-neck" />
        <div className="bit-body">
          <span className="bit-body-plate" />
          <span className="bit-core" />
          <span className="bit-chest-icon" />
        </div>
        <span className="bit-arm bit-arm-left" />
        <span className="bit-arm bit-arm-right" />
        <span className="bit-leg bit-leg-left" />
        <span className="bit-leg bit-leg-right" />
      </div>
      <div className="bit-token-orbit" aria-hidden="true">
        <span className="bit-token bit-token-one" />
        <span className="bit-token bit-token-two" />
      </div>
      <div key={`token-feed-${feedAnimationKey}`} className="bit-token-feed" aria-hidden="true">
        <span />
        <span />
        <span />
      </div>
      <div className="bit-code-particles" aria-hidden="true">
        <span />
        <span />
        <span />
        <span />
        <span />
        <span />
      </div>
      <div className="bit-confetti" aria-hidden="true">
        <span />
        <span />
        <span />
        <span />
        <span />
        <span />
      </div>
    </div>
  );
}
