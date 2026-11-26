pub mod repo;

use crate::error::{AppError, AppResult};
use rusqlite::Connection;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

const MIGRATIONS: &[(&str, &str)] = &[("001_init", include_str!("../../migrations/001_init.sql"))];

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Opens (or creates) the SQLite database on disk and applies the migrations.
    pub fn open<P: AsRef<Path>>(db_path: P) -> AppResult<Self> {
        let path = db_path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.configure_and_migrate()?;
        Ok(db)
    }

    /// Opens an in-memory SQLite database (useful for unit tests).
    #[cfg(test)]
    pub fn open_in_memory() -> AppResult<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.configure_and_migrate()?;
        Ok(db)
    }

    fn configure_and_migrate(&self) -> AppResult<()> {
        let mut conn = self
            .conn
            .lock()
            .map_err(|_| AppError::Internal("Corrupted SQLite lock".into()))?;

        // Enable foreign keys and WAL mode on every connection
        conn.pragma_update(None, "foreign_keys", "ON")?;
        let _ = conn.pragma_update(None, "journal_mode", "WAL");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
                name TEXT PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;

        for (name, sql) in MIGRATIONS {
            let already_applied: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM _migrations WHERE name = ?1)",
                [name],
                |row| row.get(0),
            )?;

            if !already_applied {
                let tx = conn.transaction()?;
                tx.execute_batch(sql)?;
                tx.execute("INSERT INTO _migrations (name) VALUES (?1)", [name])?;
                tx.commit()?;
            }
        }

        Ok(())
    }

    /// Performs an operation with exclusive access to the SQLite connection.
    pub fn with_conn<F, T>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&mut Connection) -> AppResult<T>,
    {
        let mut conn = self
            .conn
            .lock()
            .map_err(|_| AppError::Internal("Corrupted SQLite lock".into()))?;
        f(&mut conn)
    }
}