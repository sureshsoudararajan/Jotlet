//! Export functionality for notes.
//!
//! Supports exporting individual notes and all notes
//! in various formats: plain text, Markdown, HTML, and JSON backup.

use crate::database::models::Note;
use log::info;
use std::path::Path;

/// Export format options.
pub enum ExportFormat {
    PlainText,
    Markdown,
    Html,
    Json,
}

/// Exports a single note to a file.
pub fn export_note(
    note: &Note,
    path: &Path,
    format: ExportFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = match format {
        ExportFormat::PlainText => note_to_plain_text(note),
        ExportFormat::Markdown => note_to_markdown(note),
        ExportFormat::Html => note_to_html(note),
        ExportFormat::Json => serde_json::to_string_pretty(note)?,
    };

    std::fs::write(path, content)?;
    info!("Exported note '{}' to {}", note.title, path.display());
    Ok(())
}

/// Exports all notes as a JSON backup.
pub fn export_all_notes(
    notes: &[Note],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(notes)?;
    std::fs::write(path, json)?;
    info!("Exported {} notes to {}", notes.len(), path.display());
    Ok(())
}

fn note_to_plain_text(note: &Note) -> String {
    format!(
        "{}\n{}\n\n{}",
        note.title,
        "=".repeat(note.title.len()),
        crate::services::html_serializer::strip_html(&note.content)
    )
}

fn note_to_markdown(note: &Note) -> String {
    format!(
        "# {}\n\n{}\n\n---\n*Created: {}*\n*Modified: {}*\n",
        note.title,
        crate::services::html_serializer::strip_html(&note.content),
        note.created_at,
        note.updated_at
    )
}

fn note_to_html(note: &Note) -> String {
    format!(
        "<!DOCTYPE html>\n<html>\n<head><title>{}</title>\n\
         <meta charset=\"utf-8\">\n\
         <style>body {{ font-family: sans-serif; max-width: 600px; margin: 2em auto; }}</style>\n\
         </head>\n<body>\n<h1>{}</h1>\n{}\n\
         <footer><small>Created: {} | Modified: {}</small></footer>\n\
         </body>\n</html>",
        note.title, note.title, note.content, note.created_at, note.updated_at
    )
}
