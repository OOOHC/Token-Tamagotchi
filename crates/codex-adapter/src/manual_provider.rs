use token_core::models::QuotaSnapshot;

use crate::provider::{ProviderError, QuotaProvider};

pub struct ManualProvider {
    snapshot: QuotaSnapshot,
}

impl ManualProvider {
    pub fn new(snapshot: QuotaSnapshot) -> Self {
        Self { snapshot }
    }
}

impl QuotaProvider for ManualProvider {
    fn snapshot(&self) -> Result<QuotaSnapshot, ProviderError> {
        Ok(self.snapshot.clone())
    }
}

