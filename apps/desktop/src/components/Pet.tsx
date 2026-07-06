import type { Mood } from "../store/quotaStore";

type CompanionProps = {
  mood: Mood;
};

export function Companion({ mood }: CompanionProps) {
  return (
    <div className={`companion companion-${mood}`} aria-label={`Companion mood: ${mood}`}>
      <div className="companion-face">
        <span className="companion-eye" />
        <span className="companion-eye" />
      </div>
    </div>
  );
}
