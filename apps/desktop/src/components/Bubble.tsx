import type { Mood } from "../store/quotaStore";

const copy: Record<Mood, string> = {
  happy: "Plenty of tokens. Let's build.",
  focused: "Healthy quota. Keep an eye on the window.",
  tired: "Quota is getting low.",
  sleeping: "Waiting for clearer quota data."
};

type BubbleProps = {
  mood: Mood;
};

export function Bubble({ mood }: BubbleProps) {
  return <p className="bubble">{copy[mood]}</p>;
}

