import { useQuotaStore } from "../store/quotaStore";

export function useQuota() {
  return useQuotaStore();
}

