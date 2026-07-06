use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension};
use token_core::models::{Confidence, QuotaSnapshot, QuotaSource};

const INIT_SQL: &str = include_str!("../migrations/0001_init.sql");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub path: String,
}

impl DatabaseConfig {
    pub fn local(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

pub struct LocalStore {
    connection: Connection,
}

impl LocalStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let connection = Connection::open(path)?;
        let store = Self { connection };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn in_memory() -> Result<Self, StoreError> {
        let connection = Connection::open_in_memory()?;
        let store = Self { connection };
        store.run_migrations()?;
        Ok(store)
    }

    pub fn save_quota_snapshot(&self, snapshot: &QuotaSnapshot) -> Result<SaveOutcome, StoreError> {
        let inserted = self.connection.execute(
            r#"
            INSERT OR IGNORE INTO quota_snapshots (
                five_hour_remaining,
                five_hour_limit,
                total_remaining,
                total_limit,
                reset_at,
                source,
                confidence,
                raw_input_sha256,
                parser_warnings_json,
                parsed_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                optional_u64_to_i64(snapshot.five_hour_remaining),
                optional_u64_to_i64(snapshot.five_hour_limit),
                optional_u64_to_i64(snapshot.total_remaining),
                optional_u64_to_i64(snapshot.total_limit),
                snapshot.reset_at.as_deref(),
                source_to_str(&snapshot.source),
                confidence_to_str(&snapshot.confidence),
                snapshot.raw_input_sha256.as_deref(),
                serde_json::to_string(&snapshot.parser_warnings)?,
                snapshot.parsed_at,
            ],
        )?;

        Ok(if inserted == 0 {
            SaveOutcome::Duplicate
        } else {
            SaveOutcome::Inserted
        })
    }

    pub fn latest_quota_snapshot(&self) -> Result<Option<QuotaSnapshot>, StoreError> {
        self.connection
            .query_row(
                r#"
                SELECT
                    five_hour_remaining,
                    five_hour_limit,
                    total_remaining,
                    total_limit,
                    reset_at,
                    source,
                    confidence,
                    raw_input_sha256,
                    parser_warnings_json,
                    parsed_at
                FROM quota_snapshots
                ORDER BY id DESC
                LIMIT 1
                "#,
                [],
                |row| {
                    let warnings_json: String = row.get(8)?;

                    Ok(QuotaSnapshot {
                        five_hour_remaining: optional_i64_to_u64(row.get(0)?),
                        five_hour_limit: optional_i64_to_u64(row.get(1)?),
                        total_remaining: optional_i64_to_u64(row.get(2)?),
                        total_limit: optional_i64_to_u64(row.get(3)?),
                        reset_at: row.get(4)?,
                        source: source_from_str(row.get::<_, String>(5)?.as_str()),
                        confidence: confidence_from_str(row.get::<_, String>(6)?.as_str()),
                        raw_input_sha256: row.get(7)?,
                        parser_warnings: serde_json::from_str(&warnings_json).unwrap_or_default(),
                        parsed_at: row.get(9)?,
                    })
                },
            )
            .optional()
            .map_err(StoreError::from)
    }

    fn run_migrations(&self) -> Result<(), StoreError> {
        self.connection.execute_batch(INIT_SQL)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SaveOutcome {
    Inserted,
    Duplicate,
}

#[derive(Debug)]
pub enum StoreError {
    Sqlite(rusqlite::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Sqlite(error) => write!(formatter, "sqlite error: {error}"),
            StoreError::Json(error) => write!(formatter, "json error: {error}"),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<rusqlite::Error> for StoreError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

fn optional_u64_to_i64(value: Option<u64>) -> Option<i64> {
    value.and_then(|value| i64::try_from(value).ok())
}

fn optional_i64_to_u64(value: Option<i64>) -> Option<u64> {
    value.and_then(|value| u64::try_from(value).ok())
}

fn source_to_str(source: &QuotaSource) -> &'static str {
    match source {
        QuotaSource::CodexCli => "codex-cli",
        QuotaSource::Manual => "manual",
        QuotaSource::Mock => "mock",
    }
}

fn source_from_str(source: &str) -> QuotaSource {
    match source {
        "codex-cli" => QuotaSource::CodexCli,
        "manual" => QuotaSource::Manual,
        "mock" => QuotaSource::Mock,
        _ => QuotaSource::Manual,
    }
}

fn confidence_to_str(confidence: &Confidence) -> &'static str {
    match confidence {
        Confidence::High => "high",
        Confidence::Medium => "medium",
        Confidence::Low => "low",
    }
}

fn confidence_from_str(confidence: &str) -> Confidence {
    match confidence {
        "high" => Confidence::High,
        "medium" => Confidence::Medium,
        "low" => Confidence::Low,
        _ => Confidence::Low,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(hash: Option<&str>) -> QuotaSnapshot {
        QuotaSnapshot {
            five_hour_remaining: Some(12_000),
            five_hour_limit: Some(150_000),
            total_remaining: Some(820_000),
            total_limit: Some(1_500_000),
            reset_at: Some("2026-07-05T23:00:00Z".to_string()),
            source: QuotaSource::CodexCli,
            confidence: Confidence::High,
            parsed_at: "2026-07-05T00:00:00Z".to_string(),
            raw_input_sha256: hash.map(str::to_string),
            parser_warnings: vec!["example warning".to_string()],
        }
    }

    #[test]
    fn saves_and_loads_latest_snapshot() {
        let store = LocalStore::in_memory().expect("store opens");
        let snapshot = snapshot(Some("abc"));

        assert_eq!(
            store.save_quota_snapshot(&snapshot).expect("save succeeds"),
            SaveOutcome::Inserted
        );

        let latest = store
            .latest_quota_snapshot()
            .expect("latest succeeds")
            .expect("snapshot exists");

        assert_eq!(latest.five_hour_remaining, Some(12_000));
        assert_eq!(latest.confidence, Confidence::High);
        assert_eq!(latest.parser_warnings, vec!["example warning"]);
    }

    #[test]
    fn detects_duplicate_raw_input_hash() {
        let store = LocalStore::in_memory().expect("store opens");
        let snapshot = snapshot(Some("duplicate"));

        assert_eq!(
            store
                .save_quota_snapshot(&snapshot)
                .expect("first save succeeds"),
            SaveOutcome::Inserted
        );
        assert_eq!(
            store
                .save_quota_snapshot(&snapshot)
                .expect("second save succeeds"),
            SaveOutcome::Duplicate
        );
    }
}
