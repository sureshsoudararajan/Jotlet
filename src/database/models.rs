use crate::models::note_color::NoteColor;
use chrono::{DateTime, Local, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a sticky note in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub color: NoteColor,
    pub x: f64,
    pub y: f64,
    pub width: i32,
    pub height: i32,
    pub pinned: bool,
    pub always_on_top: bool,
    pub is_visible: bool,
    pub is_archived: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Note {
    /// Creates a new note with default values.
    pub fn new() -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            title: Self::default_title(),
            content: String::new(),
            color: NoteColor::Default,
            x: 0.0,
            y: 0.0,
            width: crate::config::DEFAULT_NOTE_WIDTH,
            height: crate::config::DEFAULT_NOTE_HEIGHT,
            pinned: false,
            always_on_top: false,
            is_visible: true,
            is_archived: false,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Generates a default title based on the current date.
    fn default_title() -> String {
        let now: DateTime<Local> = Local::now();
        format!("Note – {}", now.format("%d %b"))
    }

    /// Creates a Note from a database row.
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            title: row.get("title")?,
            content: row.get("content")?,
            color: NoteColor::from_str_lossy(&row.get::<_, String>("color")?),
            x: row.get("x")?,
            y: row.get("y")?,
            width: row.get("width")?,
            height: row.get("height")?,
            pinned: row.get::<_, i32>("pinned")? != 0,
            always_on_top: row.get::<_, i32>("always_on_top")? != 0,
            is_visible: row.get::<_, i32>("is_visible")? != 0,
            is_archived: row.get::<_, i32>("is_archived")? != 0,
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }

    /// Returns the plain text preview of the note content (strips HTML).
    pub fn plain_text_preview(&self) -> String {
        // Simple HTML tag stripping for preview
        let text = regex::Regex::new(r"<[^>]+>")
            .map(|re| re.replace_all(&self.content, "").to_string())
            .unwrap_or_else(|_| self.content.clone());
        let text = text.trim().to_string();
        if text.len() > 100 {
            format!("{}…", &text[..100])
        } else {
            text
        }
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================
// Database operations
// ============================================

/// Insert a new note into the database.
pub fn insert_note(conn: &Connection, note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO notes (id, title, content, color, x, y, width, height, pinned,
         always_on_top, is_visible, is_archived, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            note.id,
            note.title,
            note.content,
            note.color.to_string(),
            note.x,
            note.y,
            note.width,
            note.height,
            note.pinned as i32,
            note.always_on_top as i32,
            note.is_visible as i32,
            note.is_archived as i32,
            note.created_at,
            note.updated_at,
        ],
    )?;
    Ok(())
}

/// Update an existing note in the database.
pub fn update_note(conn: &Connection, note: &Note) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "UPDATE notes SET title=?2, content=?3, color=?4, x=?5, y=?6, width=?7, height=?8,
         pinned=?9, always_on_top=?10, is_visible=?11, is_archived=?12, updated_at=?13
         WHERE id=?1",
        params![
            note.id,
            note.title,
            note.content,
            note.color.to_string(),
            note.x,
            note.y,
            note.width,
            note.height,
            note.pinned as i32,
            note.always_on_top as i32,
            note.is_visible as i32,
            note.is_archived as i32,
            note.updated_at,
        ],
    )?;
    Ok(())
}

/// Delete a note from the database.
pub fn delete_note(conn: &Connection, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

/// Get all non-archived notes, ordered by last modified.
pub fn get_all_notes(conn: &Connection) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let mut stmt =
        conn.prepare("SELECT * FROM notes WHERE is_archived = 0 ORDER BY updated_at DESC")?;
    let notes = stmt
        .query_map([], Note::from_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(notes)
}

/// Get all notes including archived, ordered by last modified.
pub fn get_all_notes_including_archived(
    conn: &Connection,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM notes ORDER BY updated_at DESC")?;
    let notes = stmt
        .query_map([], Note::from_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(notes)
}

/// Get a single note by ID.
pub fn get_note(conn: &Connection, id: &str) -> Result<Option<Note>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM notes WHERE id = ?1")?;
    let mut notes = stmt
        .query_map(params![id], Note::from_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(notes.pop())
}

/// Search notes by title and content.
pub fn search_notes(
    conn: &Connection,
    query: &str,
) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let pattern = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT * FROM notes WHERE is_archived = 0
         AND (title LIKE ?1 OR content LIKE ?1)
         ORDER BY updated_at DESC",
    )?;
    let notes = stmt
        .query_map(params![pattern], Note::from_row)?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(notes)
}
