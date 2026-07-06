import type { Mood } from "../store/quotaStore";

const copy: Record<Mood, string> = {
  happy: "[Status]: Quota healthy. nom nom...",
  relaxed: "[Status]: Quota stable.",
  concerned: "[Status]: Quota dropping. I'm getting hungry...",
  panicking: "[Status]: Low Quota. Suggestion: Refactor Prompt.",
  exhausted: "[Status]: Quota exhausted. Please don't send another huge prompt...",
  celebrating: "[Status]: Quota restored. Breakfast!!",
  unknown: "[Status]: Waiting for quota data."
};

type BubbleProps = {
  mood: Mood;
};

export function Bubble({ mood }: BubbleProps) {
  return <p className="bubble">{copy[mood]}</p>;
}
