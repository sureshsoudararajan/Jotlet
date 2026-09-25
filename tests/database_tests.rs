use jotlet::database::migrations::run_migrations;
use jotlet::database::models::{
    delete_note, get_all_notes, get_all_notes_including_archived, get_note, insert_note,
    search_notes, update_note, Note,
};
use jotlet::models::note_color::NoteColor;
use rusqlite::Connection;

fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    run_migrations(&conn).expect("Failed to run migrations");
    conn
}

#[test]
fn test_database_creation_and_migration() {
    let conn = setup_test_db();
    let version: u32 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("Failed to query user_version");
    assert_eq!(version, 3);

    // Test settings table
    assert!(jotlet::database::models::get_bool_setting(&conn, "confirm_delete", true));
    jotlet::database::models::set_bool_setting(&conn, "confirm_delete", false)
        .expect("Failed to set bool setting");
    assert!(!jotlet::database::models::get_bool_setting(&conn, "confirm_delete", true));

    assert_eq!(
        jotlet::database::models::get_string_setting(&conn, "overview_view", "list"),
        "list"
    );
    jotlet::database::models::set_string_setting(&conn, "overview_view", "grid")
        .expect("Failed to set string setting");
    assert_eq!(
        jotlet::database::models::get_string_setting(&conn, "overview_view", "list"),
        "grid"
    );
}

#[test]
fn test_note_insert_and_get() {
    let conn = setup_test_db();
    let mut note = Note::new();
    note.title = "Test Note".to_string();
    note.content = "Some content".to_string();
    note.color = NoteColor::Blue;
    note.font_family = "JetBrainsMono Nerd Font".to_string();
    note.font_size = 22;

    insert_note(&conn, &note).expect("Failed to insert note");

    let retrieved = get_note(&conn, &note.id)
        .expect("Failed to get note")
        .expect("Note not found");

    assert_eq!(retrieved.id, note.id);
    assert_eq!(retrieved.title, "Test Note");
    assert_eq!(retrieved.content, "Some content");
    assert_eq!(retrieved.color, NoteColor::Blue);
    assert_eq!(retrieved.font_family, "JetBrainsMono Nerd Font");
    assert_eq!(retrieved.font_size, 22);
}

#[test]
fn test_note_update() {
    let conn = setup_test_db();
    let mut note = Note::new();
    note.title = "Original Title".to_string();
    note.content = "Original Content".to_string();

    insert_note(&conn, &note).expect("Failed to insert note");

    note.title = "Updated Title".to_string();
    note.content = "Updated Content".to_string();
    note.color = NoteColor::Green;
    note.pinned = true;
    note.width = 450;
    note.height = 400;
    note.font_family = "DejaVu Serif".to_string();
    note.font_size = 32;

    update_note(&conn, &note).expect("Failed to update note");

    let retrieved = get_note(&conn, &note.id)
        .expect("Failed to get note")
        .expect("Note not found");

    assert_eq!(retrieved.title, "Updated Title");
    assert_eq!(retrieved.content, "Updated Content");
    assert_eq!(retrieved.color, NoteColor::Green);
    assert!(retrieved.pinned);
    assert_eq!(retrieved.width, 450);
    assert_eq!(retrieved.height, 400);
    assert_eq!(retrieved.font_family, "DejaVu Serif");
    assert_eq!(retrieved.font_size, 32);
}

#[test]
fn test_note_delete() {
    let conn = setup_test_db();
    let note = Note::new();
    insert_note(&conn, &note).expect("Failed to insert note");

    delete_note(&conn, &note.id).expect("Failed to delete note");

    let retrieved = get_note(&conn, &note.id).expect("Failed to get note");
    assert!(retrieved.is_none());
}

#[test]
fn test_note_search() {
    let conn = setup_test_db();

    let mut note1 = Note::new();
    note1.title = "Linux Kernel Notes".to_string();
    note1.content = "Arch Linux setup and pacman tips".to_string();
    insert_note(&conn, &note1).expect("Failed to insert");

    let mut note2 = Note::new();
    note2.title = "Docker Containers".to_string();
    note2.content = "DevOps learning and Dockerfiles".to_string();
    insert_note(&conn, &note2).expect("Failed to insert");

    let results = search_notes(&conn, "pacman").expect("Failed to search");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, note1.id);

    let results2 = search_notes(&conn, "Docker").expect("Failed to search");
    assert_eq!(results2.len(), 1);
    assert_eq!(results2[0].id, note2.id);

    let results3 = search_notes(&conn, "Nonexistent").expect("Failed to search");
    assert_eq!(results3.len(), 0);
}

#[test]
fn test_get_all_notes_and_archived() {
    let conn = setup_test_db();

    let mut note1 = Note::new();
    note1.title = "Active Note".to_string();
    insert_note(&conn, &note1).expect("Failed to insert");

    let mut note2 = Note::new();
    note2.title = "Archived Note".to_string();
    note2.is_archived = true;
    insert_note(&conn, &note2).expect("Failed to insert");

    let active = get_all_notes(&conn).expect("Failed to get active");
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].title, "Active Note");

    let all = get_all_notes_including_archived(&conn).expect("Failed to get all");
    assert_eq!(all.len(), 2);
}
