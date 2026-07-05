#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub path: String,
}

impl DatabaseConfig {
    pub fn local(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

