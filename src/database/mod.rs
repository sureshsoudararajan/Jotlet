pub mod migrations;
pub mod models;

use crate::config;
use log::{error, info};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

/// Thread-safe database wrapper.
pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// Opens the database, creating it and running migrations if needed.
    pub fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::db_path()?;

        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        info!("Opening database at: {}", db_path.display());
        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent access and crash safety
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;

        let db = Self {
            conn: Mutex::new(conn),
        };

        // Run migrations
        db.migrate()?;

        Ok(db)
    }

    /// Returns the path to the database file.
    fn db_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let data_dir = glib::user_data_dir().join(config::XDG_DIR_NAME);
        Ok(data_dir.join(config::DB_FILENAME))
    }

    /// Executes a closure with access to the database connection.
    pub fn with_connection<F, T>(&self, f: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: FnOnce(&Connection) -> Result<T, Box<dyn std::error::Error>>,
    {
        let conn = self.conn.lock().map_err(|e| {
            error!("Failed to lock database connection: {}", e);
            format!("Database lock error: {}", e)
        })?;
        f(&conn)
    }

    /// Runs database migrations.
    fn migrate(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.with_connection(|conn| {
            migrations::run_migrations(conn)?;
            Ok(())
        })
    }
}
