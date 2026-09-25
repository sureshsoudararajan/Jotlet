use log::info;
use rusqlite::Connection;

/// Current schema version.
const SCHEMA_VERSION: u32 = 3;

/// Run all pending migrations.
pub fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let current_version: u32 = conn.pragma_query_value(None, "user_version", |row| row.get(0))?;

    info!(
        "Database schema version: {} (target: {})",
        current_version, SCHEMA_VERSION
    );

    if current_version < 1 {
        migrate_v1(conn)?;
    }

    if current_version < 2 {
        migrate_v2(conn)?;
    }

    if current_version < 3 {
        migrate_v3(conn)?;
    }

    // Set the current schema version
    conn.pragma_update(None, "user_version", SCHEMA_VERSION)?;

    Ok(())
}

/// Migration v1: Create the initial notes table.
fn migrate_v1(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running migration v1: creating notes table");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS notes (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL DEFAULT 'New Note',
            content     TEXT NOT NULL DEFAULT '',
            color       TEXT NOT NULL DEFAULT 'default',
            x           REAL NOT NULL DEFAULT 0.0,
            y           REAL NOT NULL DEFAULT 0.0,
            width       INTEGER NOT NULL DEFAULT 320,
            height      INTEGER NOT NULL DEFAULT 300,
            pinned      INTEGER NOT NULL DEFAULT 0,
            always_on_top INTEGER NOT NULL DEFAULT 0,
            is_visible  INTEGER NOT NULL DEFAULT 1,
            is_archived INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );",
    )?;

    Ok(())
}

/// Migration v2: Create settings table for persistent preferences.
fn migrate_v2(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running migration v2: creating settings table");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS settings (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );",
    )?;

    Ok(())
}

/// Migration v3: Add font_family and font_size columns to notes table.
fn migrate_v3(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running migration v3: adding font_family and font_size columns to notes table");

    conn.execute_batch(
        "ALTER TABLE notes ADD COLUMN font_family TEXT NOT NULL DEFAULT '';
         ALTER TABLE notes ADD COLUMN font_size INTEGER NOT NULL DEFAULT 26;",
    )?;

    Ok(())
}
