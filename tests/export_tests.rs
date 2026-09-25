use jotlet::database::models::Note;
use jotlet::models::note_color::NoteColor;
use jotlet::services::export::{export_all_notes, export_note, ExportFormat};
use jotlet::services::import::{import_json_backup, import_markdown, import_plain_text};

#[test]
fn test_export_and_import_plain_text() {
    let dir = tempfile_dir();
    let file_path = dir.join("test_note.txt");

    let mut note = Note::new();
    note.title = "Shopping List".to_string();
    note.content = "Apples\nBananas\nMilk".to_string();

    export_note(&note, &file_path, ExportFormat::PlainText).expect("Export failed");
    assert!(file_path.exists());

    let imported = import_plain_text(&file_path).expect("Import failed");
    assert_eq!(imported.title, "test_note");
    assert!(imported.content.contains("Shopping List"));
    assert!(imported.content.contains("Apples"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_export_and_import_markdown() {
    let dir = tempfile_dir();
    let file_path = dir.join("devops_notes.md");

    let mut note = Note::new();
    note.title = "DevOps Learning".to_string();
    note.content = "## Kubernetes\n- Pods\n- Services".to_string();

    export_note(&note, &file_path, ExportFormat::Markdown).expect("Export failed");
    assert!(file_path.exists());

    let imported = import_markdown(&file_path).expect("Import failed");
    assert_eq!(imported.title, "DevOps Learning");
    assert!(imported.content.contains("Kubernetes"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_export_html() {
    let dir = tempfile_dir();
    let file_path = dir.join("note.html");

    let mut note = Note::new();
    note.title = "HTML Note".to_string();
    note.content = "<b>Bold text</b> and <i>italic text</i>".to_string();

    export_note(&note, &file_path, ExportFormat::Html).expect("Export failed");
    assert!(file_path.exists());

    let html_content = std::fs::read_to_string(&file_path).expect("Read failed");
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("HTML Note"));
    assert!(html_content.contains("<b>Bold text</b>"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_export_and_import_json_backup() {
    let dir = tempfile_dir();
    let file_path = dir.join("backup.json");

    let mut note1 = Note::new();
    note1.title = "Note 1".to_string();
    note1.content = "Content 1".to_string();
    note1.color = NoteColor::Yellow;

    let mut note2 = Note::new();
    note2.title = "Note 2".to_string();
    note2.content = "Content 2".to_string();
    note2.color = NoteColor::Purple;

    let notes = vec![note1, note2];
    export_all_notes(&notes, &file_path).expect("Export all failed");
    assert!(file_path.exists());

    let restored = import_json_backup(&file_path).expect("Import backup failed");
    assert_eq!(restored.len(), 2);
    assert_eq!(restored[0].title, "Note 1");
    assert_eq!(restored[0].color, NoteColor::Yellow);
    assert_eq!(restored[1].title, "Note 2");
    assert_eq!(restored[1].color, NoteColor::Purple);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_export_and_import_checklist() {
    let dir = tempfile_dir();
    let file_path = dir.join("checklist.md");

    let mut note = Note::new();
    note.title = "Tasks".to_string();
    note.content = "☐ Todo item\n☑ Done item".to_string();

    export_note(&note, &file_path, ExportFormat::Markdown).expect("Export failed");
    assert!(file_path.exists());

    let imported = import_markdown(&file_path).expect("Import failed");
    assert_eq!(imported.title, "Tasks");
    assert!(imported.content.contains("Todo item"));
    assert!(imported.content.contains("Done item"));

    let _ = std::fs::remove_dir_all(&dir);
}

fn tempfile_dir() -> std::path::PathBuf {
    let id = uuid::Uuid::new_v4().to_string();
    let dir = std::env::temp_dir().join(format!("jotlet_test_{}", id));
    std::fs::create_dir_all(&dir).expect("Failed to create temp dir");
    dir
}
