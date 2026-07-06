import { create } from "zustand";

export type Mood =
  | "happy"
  | "relaxed"
  | "concerned"
  | "panicking"
  | "exhausted"
  | "celebrating"
  | "unknown";

export type QuotaSnapshot = {
  fiveHourRemaining: number | null;
  fiveHourLimit: number | null;
  totalRemaining: number | null;
  totalLimit: number | null;
  resetAt: string | null;
  source: "codex-cli" | "codex-local" | "codex-app-server" | "manual" | "mock";
  confidence: "high" | "medium" | "low";
  parsedAt: string;
  rawInputSha256: string | null;
  parserWarnings: string[];
};

type QuotaState = {
  snapshot: QuotaSnapshot;
  setSnapshot: (snapshot: QuotaSnapshot) => void;
};

export const useQuotaStore = create<QuotaState>((set) => ({
  snapshot: {
    fiveHourRemaining: 120000,
    fiveHourLimit: 150000,
    totalRemaining: 1200000,
    totalLimit: 1500000,
    resetAt: null,
    source: "mock",
    confidence: "medium",
    parsedAt: new Date().toISOString(),
    rawInputSha256: null,
    parserWarnings: []
  },
  setSnapshot: (snapshot) => set({ snapshot })
}));
