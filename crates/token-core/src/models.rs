use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum QuotaSource {
    CodexCli,
    Manual,
    Mock,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Mood {
    Happy,
    Focused,
    Tired,
    Sleeping,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QuotaSnapshot {
    pub five_hour_remaining: Option<u64>,
    pub five_hour_limit: Option<u64>,
    pub total_remaining: Option<u64>,
    pub total_limit: Option<u64>,
    pub reset_at: Option<String>,
    pub source: QuotaSource,
    pub confidence: Confidence,
    pub parsed_at: String,
    pub raw_input_sha256: Option<String>,
    pub parser_warnings: Vec<String>,
}
