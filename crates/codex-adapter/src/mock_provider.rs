use token_core::models::{Confidence, QuotaSnapshot, QuotaSource};

use crate::provider::{ProviderError, QuotaProvider};

pub struct MockProvider;

impl QuotaProvider for MockProvider {
    fn snapshot(&self) -> Result<QuotaSnapshot, ProviderError> {
        Ok(mock_snapshot())
    }
}

pub fn mock_snapshot() -> QuotaSnapshot {
    QuotaSnapshot {
        five_hour_remaining: Some(120_000),
        five_hour_limit: Some(150_000),
        total_remaining: Some(1_200_000),
        total_limit: Some(1_500_000),
        reset_at: None,
        source: QuotaSource::Mock,
        confidence: Confidence::Medium,
        parsed_at: "2026-07-05T00:00:00Z".to_string(),
        raw_input_sha256: None,
        parser_warnings: Vec::new(),
    }
}
