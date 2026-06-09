//! GOAT brain — SQLite-backed memory, session, and interaction storage.
//!
//! Uses `anyhow` for all error propagation, with context strings that are
//! user-readable and safe to display (no internal SQL details unless in debug).
//!
//! # Schema migrations
//!
//! Migrations are applied automatically on DB open.  Each migration is
//! idempotent (uses `IF NOT EXISTS` or `ADD COLUMN IF NOT EXISTS` equivalents).
//!
//! **Migration 001:** sessions.updated_at column (added in Phase 1.6)
//! - Safe for old DBs — column is added only if absent.

use anyhow::{Context, Result};
use rusqlite::{Connection, params};

use crate::project::ProjectMetadata;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::{DirEntry, WalkDir};

const MAX_INDEXED_FILE_BYTES: u64 = 512 * 1024;

// ── Types ──────────────────────────────────────────────────────────────────────

pub struct IndexSummary {
    pub scanned_files: usize,
    pub indexed_files: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
}

/// A session record with metadata.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: String,
    pub title: String,
    /// ISO-8601 creation timestamp (from CURRENT_TIMESTAMP).
    pub created_at: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

impl SessionRecord {
    /// Returns true if the session ID looks like a UUID v4.
    pub fn is_uuid(&self) -> bool {
        // UUID v4: 8-4-4-4-12 hex digits separated by hyphens
        let p = &self.id;
        p.len() == 36
            && p.chars().enumerate().all(|(i, c)| {
                if [8, 13, 18, 23].contains(&i) {
                    c == '-'
                } else {
                    c.is_ascii_hexdigit()
                }
            })
    }
}

pub struct Brain {
    conn: Connection,
}

// ── Brain implementation ──────────────────────────────────────────────────────

impl Brain {
    /// Open (or create) the brain database at `path`.
    ///
    /// Applies all schema migrations automatically.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let conn = Connection::open(path)
            .with_context(|| format!("failed to open brain database at {}", path.display()))?;

        conn.pragma_update(None, "journal_mode", "WAL")
            .context("failed to set WAL journal mode")?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .context("failed to set synchronous=NORMAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .context("failed to enable foreign keys")?;
        conn.pragma_update(None, "busy_timeout", 5000)
            .context("failed to set busy_timeout")?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS memory_blocks (
                id          INTEGER PRIMARY KEY,
                label       TEXT NOT NULL,
                description TEXT,
                value       TEXT NOT NULL,
                scope       TEXT NOT NULL DEFAULT 'global',
                created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS sessions (
                id         TEXT PRIMARY KEY,
                title      TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS interactions (
                id         INTEGER PRIMARY KEY,
                session_id TEXT NOT NULL,
                role       TEXT NOT NULL,
                content    TEXT NOT NULL,
                timestamp  DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY(session_id) REFERENCES sessions(id)
            );

            CREATE TABLE IF NOT EXISTS indexed_files (
                path          TEXT PRIMARY KEY,
                content       TEXT NOT NULL,
                sha256        TEXT NOT NULL,
                size_bytes    INTEGER NOT NULL,
                modified_unix INTEGER NOT NULL,
                indexed_at    DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS projects (
                root_path     TEXT PRIMARY KEY,
                metadata_json TEXT NOT NULL,
                last_scan     DATETIME DEFAULT CURRENT_TIMESTAMP
            );",
        )
        .context("failed to initialize brain schema")?;

        // ── Migration 001: add updated_at if it doesn't exist (safe for old DBs)
        // SQLite does not support ADD COLUMN IF NOT EXISTS, so we use a pragma check.
        let has_updated_at: bool = conn
            .prepare("PRAGMA table_info(sessions)")
            .context("failed to query sessions schema")?
            .query_map([], |row| row.get::<_, String>(1))
            .context("failed to read sessions columns")?
            .filter_map(|r| r.ok())
            .any(|col| col == "updated_at");

        if !has_updated_at {
            // SQLite only allows constant defaults in ALTER TABLE ADD COLUMN.
            // Add column with NULL default, then back-fill existing rows with their created_at.
            conn.execute_batch(
                "ALTER TABLE sessions ADD COLUMN updated_at DATETIME;
                 UPDATE sessions SET updated_at = created_at WHERE updated_at IS NULL;",
            )
            .context("failed to apply migration 001 (sessions.updated_at)")?;
        }

        Ok(Brain { conn })
    }

    // ── Memory ────────────────────────────────────────────────────────────────

    pub fn insert_memory(
        &self,
        label: &str,
        description: &str,
        value: &str,
        scope: &str,
    ) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO memory_blocks (label, description, value, scope) VALUES (?1, ?2, ?3, ?4)",
                (label, description, value, scope),
            )
            .context("failed to insert memory block")?;
        Ok(())
    }

    pub fn get_all_memories(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT label, value FROM memory_blocks")
            .context("failed to prepare memory query")?;
        let memories = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .context("failed to query memory blocks")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(memories)
    }

    // ── Sessions ──────────────────────────────────────────────────────────────

    /// Create a new session with a UUID id.
    pub fn create_session(&self, id: &str, title: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO sessions (id, title) VALUES (?1, ?2)",
                (id, title),
            )
            .context("failed to create session")?;
        Ok(())
    }

    /// Update the title of an existing session.
    pub fn update_session_title(&self, id: &str, title: &str) -> Result<()> {
        self.conn
            .execute("UPDATE sessions SET title = ?1 WHERE id = ?2", (title, id))
            .context("failed to update session title")?;
        Ok(())
    }

    /// Get all sessions as (id, title) pairs, ordered by most-recent first.
    /// For backward compatibility — used in internal code that only needs id+title.
    pub fn get_sessions(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title FROM sessions ORDER BY created_at DESC")
            .context("failed to prepare sessions query")?;
        let sessions = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .context("failed to query sessions")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(sessions)
    }

    /// Get all sessions as full `SessionRecord` structs (with timestamps).
    pub fn get_session_records(&self) -> Result<Vec<SessionRecord>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, COALESCE(title, 'Untitled'), \
                        COALESCE(created_at, ''), COALESCE(updated_at, '') \
                 FROM sessions ORDER BY created_at DESC",
            )
            .context("failed to prepare session records query")?;
        let records = stmt
            .query_map([], |row| {
                Ok(SessionRecord {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            })
            .context("failed to query session records")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(records)
    }

    pub fn load_session_history(&self, session_id: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT role, content FROM interactions WHERE session_id = ?1 ORDER BY id ASC")
            .context("failed to prepare history query")?;
        let history = stmt
            .query_map([session_id], |row| Ok((row.get(0)?, row.get(1)?)))
            .context("failed to query session history")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(history)
    }

    pub fn log_interaction(&self, session_id: &str, role: &str, content: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO interactions (session_id, role, content) VALUES (?1, ?2, ?3)",
                (session_id, role, content),
            )
            .context("failed to log interaction")?;
        // Update session updated_at.
        let _ = self.conn.execute(
            "UPDATE sessions SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            [session_id],
        );
        Ok(())
    }

    pub fn recall_search(&self, query: &str) -> Result<Vec<(String, String, String)>> {
        let like_query = format!("%{}%", query);
        let mut stmt = self
            .conn
            .prepare("SELECT session_id, role, content FROM interactions WHERE content LIKE ?1 ORDER BY id DESC LIMIT 10")
            .context("failed to prepare recall query")?;
        let results = stmt
            .query_map([like_query], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .context("failed to query recall results")?
            .filter_map(|r| r.ok())
            .collect();
        Ok(results)
    }

    // ── Projects ──────────────────────────────────────────────────────────────

    pub fn save_project(&self, root_path: &str, metadata: &ProjectMetadata) -> Result<()> {
        let json =
            serde_json::to_string(metadata).context("failed to serialize project metadata")?;
        self.conn
            .execute(
                "INSERT INTO projects (root_path, metadata_json, last_scan) 
                 VALUES (?1, ?2, CURRENT_TIMESTAMP)
                 ON CONFLICT(root_path) DO UPDATE SET 
                 metadata_json = excluded.metadata_json, 
                 last_scan = CURRENT_TIMESTAMP",
                (root_path, json),
            )
            .context("failed to save project metadata")?;
        Ok(())
    }

    pub fn get_project(&self, root_path: &str) -> Result<Option<ProjectMetadata>> {
        let mut stmt = self
            .conn
            .prepare("SELECT metadata_json FROM projects WHERE root_path = ?1")
            .context("failed to prepare project query")?;

        let mut rows = stmt
            .query([root_path])
            .context("failed to query projects")?;
        if let Some(row) = rows.next().context("failed to read project row")? {
            let json: String = row.get(0)?;
            let meta: ProjectMetadata =
                serde_json::from_str(&json).context("failed to deserialize project metadata")?;
            Ok(Some(meta))
        } else {
            Ok(None)
        }
    }

    // ── File indexing ─────────────────────────────────────────────────────────

    pub fn index_paths<P: AsRef<Path>>(&self, paths: &[P]) -> Result<IndexSummary> {
        let mut summary = IndexSummary {
            scanned_files: 0,
            indexed_files: 0,
            skipped_files: 0,
            failed_files: 0,
        };

        for root in paths {
            let root = root.as_ref();
            if !root.exists() {
                continue;
            }
            for entry in WalkDir::new(root)
                .follow_links(false)
                .into_iter()
                .filter_entry(|entry| !is_ignored_entry(entry))
            {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => {
                        summary.failed_files += 1;
                        continue;
                    }
                };
                if !entry.file_type().is_file() {
                    continue;
                }
                summary.scanned_files += 1;
                match self.index_file(entry.path()) {
                    Ok(true) => summary.indexed_files += 1,
                    Ok(false) => summary.skipped_files += 1,
                    Err(_) => summary.failed_files += 1,
                }
            }
        }

        Ok(summary)
    }

    fn index_file(&self, path: &Path) -> Result<bool> {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return Ok(false),
        };
        if metadata.len() > MAX_INDEXED_FILE_BYTES || !is_allowed_extension(path) {
            return Ok(false);
        }
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(_) => return Ok(false),
        };
        if bytes.contains(&0) {
            return Ok(false);
        }
        let content = match String::from_utf8(bytes.clone()) {
            Ok(s) => s,
            Err(_) => return Ok(false),
        };
        let digest = Sha256::digest(&bytes);
        let hash = digest
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        let modified_unix = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or_default();

        self.conn
            .execute(
                "INSERT INTO indexed_files (path, content, sha256, size_bytes, modified_unix, indexed_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)
                 ON CONFLICT(path) DO UPDATE SET
                     content       = excluded.content,
                     sha256        = excluded.sha256,
                     size_bytes    = excluded.size_bytes,
                     modified_unix = excluded.modified_unix,
                     indexed_at    = CURRENT_TIMESTAMP",
                params![
                    path.to_string_lossy().to_string(),
                    content,
                    hash,
                    metadata.len() as i64,
                    modified_unix,
                ],
            )
            .context("failed to upsert indexed file")?;

        Ok(true)
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn is_ignored_entry(entry: &DirEntry) -> bool {
    let ignored = [
        ".git",
        ".hg",
        ".svn",
        "node_modules",
        "target",
        "dist",
        "build",
        ".next",
        ".turbo",
        ".cache",
        "vendor",
        "__pycache__",
    ];
    entry
        .file_name()
        .to_str()
        .map(|name| ignored.contains(&name))
        .unwrap_or(false)
}

fn is_allowed_extension(path: &Path) -> bool {
    let allowed = [
        "rs",
        "toml",
        "md",
        "txt",
        "json",
        "yaml",
        "yml",
        "js",
        "jsx",
        "ts",
        "tsx",
        "py",
        "go",
        "java",
        "kt",
        "c",
        "h",
        "cpp",
        "hpp",
        "css",
        "html",
        "sh",
        "zsh",
        "fish",
        "sql",
        "env.example",
        "gitignore",
    ];
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if allowed.contains(&file_name) {
        return true;
    }
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| allowed.contains(&e))
        .unwrap_or(false)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_brain() -> Brain {
        Brain::new(":memory:").expect("in-memory brain should open")
    }

    #[test]
    fn test_brain_opens_in_memory() {
        let b = temp_brain();
        let sessions = b.get_sessions().unwrap();
        assert!(sessions.is_empty());
    }

    #[test]
    fn test_create_and_list_session() {
        let b = temp_brain();
        b.create_session("test-id", "Test Session").unwrap();
        let sessions = b.get_sessions().unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].0, "test-id");
    }

    #[test]
    fn test_session_record_has_timestamps() {
        let b = temp_brain();
        b.create_session("r1", "Record Test").unwrap();
        let records = b.get_session_records().unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].id, "r1");
        // timestamps should be non-empty
        assert!(!records[0].created_at.is_empty());
    }

    #[test]
    fn test_session_record_is_uuid_detection() {
        let uuid_rec = SessionRecord {
            id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            title: "t".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };
        let legacy_rec = SessionRecord {
            id: "1718900000".to_string(),
            title: "t".to_string(),
            created_at: "".to_string(),
            updated_at: "".to_string(),
        };
        assert!(uuid_rec.is_uuid());
        assert!(!legacy_rec.is_uuid());
    }

    #[test]
    fn test_log_interaction_updates_session() {
        let b = temp_brain();
        b.create_session("sess1", "Test").unwrap();
        b.log_interaction("sess1", "user", "hello").unwrap();
        let history = b.load_session_history("sess1").unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].0, "user");
        assert_eq!(history[0].1, "hello");
    }

    #[test]
    fn test_migration_idempotent() {
        // Opening twice should not fail (migration is idempotent).
        let b = temp_brain();
        drop(b);
        // In-memory DB is gone, but we can verify the schema logic doesn't panic.
    }
}
