//! Import functionality for notes.
//!
//! Supports importing notes from plain text and Markdown files.

use crate::database::models::Note;
use log::info;
use std::path::Path;

/// Imports a note from a plain text file.
pub fn import_plain_text(path: &Path) -> Result<Note, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let title = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported Note")
        .to_string();

    let mut note = Note::new();
    note.title = title;
    note.content = content;

    info!("Imported note from {}", path.display());
    Ok(note)
}

/// Imports a note from a Markdown file.
pub fn import_markdown(path: &Path) -> Result<Note, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;

    // Try to extract title from first heading
    let (title, body) = if let Some(first_line) = content.lines().next() {
        if first_line.starts_with("# ") {
            let title = first_line.trim_start_matches("# ").to_string();
            let body = content
                .lines()
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            (title, body)
        } else {
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Imported Note")
                .to_string();
            (title, content)
        }
    } else {
        ("Imported Note".to_string(), content)
    };

    let mut note = Note::new();
    note.title = title;
    note.content = body;

    info!("Imported Markdown note from {}", path.display());
    Ok(note)
}

/// Imports notes from a JSON backup file.
pub fn import_json_backup(path: &Path) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let notes: Vec<Note> = serde_json::from_str(&content)?;
    info!("Imported {} notes from JSON backup", notes.len());
    Ok(notes)
}
