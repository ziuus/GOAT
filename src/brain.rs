use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use walkdir::{DirEntry, WalkDir};

const MAX_INDEXED_FILE_BYTES: u64 = 512 * 1024;

pub struct IndexSummary {
    pub scanned_files: usize,
    pub indexed_files: usize,
    pub skipped_files: usize,
    pub failed_files: usize,
}

pub struct Brain {
    conn: Connection,
}

impl Brain {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_blocks (
                id INTEGER PRIMARY KEY,
                label TEXT NOT NULL,
                description TEXT,
                value TEXT NOT NULL,
                scope TEXT NOT NULL DEFAULT 'global',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS interactions (
                id INTEGER PRIMARY KEY,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS indexed_files (
                path TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                sha256 TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                modified_unix INTEGER NOT NULL,
                indexed_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        Ok(Brain { conn })
    }

    pub fn insert_memory(&self, label: &str, description: &str, value: &str, scope: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO memory_blocks (label, description, value, scope) VALUES (?1, ?2, ?3, ?4)",
            (label, description, value, scope),
        )?;
        Ok(())
    }

    pub fn get_all_memories(&self) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT label, value FROM memory_blocks")?;
        let memories = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?
        .filter_map(Result::ok)
        .collect();
        
        Ok(memories)
    }

    pub fn log_interaction(&self, role: &str, content: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO interactions (role, content) VALUES (?1, ?2)",
            (role, content),
        )?;
        Ok(())
    }

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
                    Ok(entry) => entry,
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
            Ok(metadata) => metadata,
            Err(_) => return Ok(false),
        };

        if metadata.len() > MAX_INDEXED_FILE_BYTES || !is_allowed_extension(path) {
            return Ok(false);
        }

        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(_) => return Ok(false),
        };

        if bytes.contains(&0) {
            return Ok(false);
        }

        let content = match String::from_utf8(bytes.clone()) {
            Ok(content) => content,
            Err(_) => return Ok(false),
        };

        let digest = Sha256::digest(&bytes);
        let hash = digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let modified_unix = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or_default();

        self.conn.execute(
            "INSERT INTO indexed_files (path, content, sha256, size_bytes, modified_unix, indexed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP)
             ON CONFLICT(path) DO UPDATE SET
                content = excluded.content,
                sha256 = excluded.sha256,
                size_bytes = excluded.size_bytes,
                modified_unix = excluded.modified_unix,
                indexed_at = CURRENT_TIMESTAMP",
            params![
                path.to_string_lossy().to_string(),
                content,
                hash,
                metadata.len() as i64,
                modified_unix,
            ],
        )?;

        Ok(true)
    }
}

fn is_ignored_entry(entry: &DirEntry) -> bool {
    let ignored_names = [
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
        .map(|name| ignored_names.contains(&name))
        .unwrap_or(false)
}

fn is_allowed_extension(path: &Path) -> bool {
    let allowed = [
        "rs", "toml", "md", "txt", "json", "yaml", "yml", "js", "jsx", "ts", "tsx", "py",
        "go", "java", "kt", "c", "h", "cpp", "hpp", "css", "html", "sh", "zsh", "fish", "sql",
        "env.example", "gitignore",
    ];

    let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    if allowed.contains(&file_name) {
        return true;
    }

    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| allowed.contains(&extension))
        .unwrap_or(false)
}
