import type { Mood, QuotaSnapshot } from "../store/quotaStore";

export function useMood(snapshot: QuotaSnapshot): Mood {
  if (snapshot.fiveHourRemaining === null || snapshot.fiveHourLimit === null) {
    return "sleeping";
  }

  const ratio = snapshot.fiveHourRemaining / snapshot.fiveHourLimit;

  if (ratio <= 0.15) {
    return "tired";
  }

  if (ratio <= 0.5) {
    return "focused";
  }

  return "happy";
}

