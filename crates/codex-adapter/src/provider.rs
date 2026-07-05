use token_core::models::QuotaSnapshot;

pub trait QuotaProvider {
    fn snapshot(&self) -> Result<QuotaSnapshot, ProviderError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderError {
    pub message: String,
}

impl ProviderError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

