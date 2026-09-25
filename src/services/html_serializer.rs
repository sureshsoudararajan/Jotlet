//! HTML serialization/deserialization for rich text storage.
//!
//! Converts between GtkTextBuffer content (with tags) and HTML stored in SQLite.
//! For v1.0, content is stored as plain text. HTML serialization will be
//! implemented when rich text persistence is added.

use gtk::prelude::*;

/// Serializes a GtkTextBuffer to HTML string.
pub fn serialize_buffer(buffer: &gtk::TextBuffer) -> String {
    let start = buffer.start_iter();
    let end = buffer.end_iter();
    // For now, return plain text. Rich text serialization in Phase 6.
    buffer.text(&start, &end, true).to_string()
}

/// Deserializes an HTML string into a GtkTextBuffer.
pub fn deserialize_to_buffer(html: &str, buffer: &gtk::TextBuffer) {
    // For now, set as plain text. Rich text deserialization in Phase 6.
    buffer.set_text(html);
}

/// Strips HTML tags for plain text preview.
pub fn strip_html(html: &str) -> String {
    regex::Regex::new(r"<[^>]+>")
        .map(|re| re.replace_all(html, "").to_string())
        .unwrap_or_else(|_| html.to_string())
}
