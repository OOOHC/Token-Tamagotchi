import type { Mood } from "../store/quotaStore";

type PetProps = {
  mood: Mood;
};

export function Pet({ mood }: PetProps) {
  return (
    <div className={`pet pet-${mood}`} aria-label={`Pet mood: ${mood}`}>
      <div className="pet-face">
        <span className="pet-eye" />
        <span className="pet-eye" />
      </div>
    </div>
  );
}

