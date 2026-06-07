use rusqlite::{Connection, Result};
use std::path::Path;

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
}
