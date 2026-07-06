import type { Mood } from "../store/quotaStore";

type CompanionProps = {
  mood: Mood;
  isActive?: boolean;
  onInteract?: () => void;
};

export function Companion({ mood, isActive = false, onInteract }: CompanionProps) {
  return (
    <button
      className={`companion companion-${mood}${isActive ? " companion-active" : ""}`}
      type="button"
      aria-label={`Companion mood: ${mood}`}
      onClick={onInteract}
    >
      <div className="companion-face">
        <span className="companion-eye" />
        <span className="companion-eye" />
        <span className="companion-mouth" />
      </div>
    </button>
  );
}
