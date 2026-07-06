import type { Mood, QuotaSnapshot } from "../store/quotaStore";

export function useMood(snapshot: QuotaSnapshot): Mood {
  if (
    snapshot.fiveHourRemaining === null ||
    snapshot.fiveHourLimit === null ||
    snapshot.fiveHourLimit === 0
  ) {
    return "unknown";
  }

  const percent = (snapshot.fiveHourRemaining / snapshot.fiveHourLimit) * 100;

  if (percent > 80) {
    return "happy";
  }

  if (percent >= 50) {
    return "relaxed";
  }

  if (percent >= 20) {
    return "concerned";
  }

  if (percent >= 5) {
    return "panicking";
  }

  return "exhausted";
}
