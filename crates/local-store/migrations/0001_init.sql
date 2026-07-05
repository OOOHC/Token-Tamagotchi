CREATE TABLE IF NOT EXISTS quota_snapshots (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  five_hour_remaining INTEGER,
  five_hour_limit INTEGER,
  total_remaining INTEGER,
  total_limit INTEGER,
  reset_at TEXT,
  source TEXT NOT NULL,
  confidence TEXT NOT NULL,
  raw_input_sha256 TEXT,
  parser_warnings_json TEXT NOT NULL DEFAULT '[]',
  parsed_at TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_quota_snapshots_raw_input_sha256
  ON quota_snapshots(raw_input_sha256)
  WHERE raw_input_sha256 IS NOT NULL;

CREATE TABLE IF NOT EXISTS settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS raw_imports_optional (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  raw_input_sha256 TEXT UNIQUE NOT NULL,
  raw_text TEXT NOT NULL,
  source TEXT NOT NULL,
  imported_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
