use crate::application::JotletApplication;
use crate::database::models::{self, Note};

use adw::prelude::*;
use gtk::gio;
use log::error;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NotesOverview {
    window: adw::ApplicationWindow,
    list_box: gtk::ListBox,
    search_entry: gtk::SearchEntry,
    notes: Rc<RefCell<Vec<Note>>>,
    app: JotletApplication,
}

impl NotesOverview {
    pub fn new(app: &JotletApplication) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Notes")
            .default_width(450)
            .default_height(560)
            .build();

        let list_box = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .margin_start(12)
            .margin_end(12)
            .margin_top(6)
            .margin_bottom(12)
            .build();
        list_box.add_css_class("boxed-list");

        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Search notes…")
            .hexpand(true)
            .build();

        let notes = Rc::new(RefCell::new(Vec::new()));

        let overview = Self {
            window,
            list_box,
            search_entry,
            notes,
            app: app.clone(),
        };

        overview.setup_ui(app);
        overview.load_notes();

        overview
    }

    pub fn present(&self) {
        self.load_notes();
        self.window.set_visible(true);
        self.window.present();
    }

    fn setup_ui(&self, app: &JotletApplication) {
        let header = adw::HeaderBar::new();

        let new_btn = gtk::Button::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("New Note")
            .build();

        let app_clone = app.clone();
        new_btn.connect_clicked(move |_| {
            app_clone.activate_action("new-note", None);
        });
        header.pack_start(&new_btn);

        let search_bar = gtk::SearchBar::builder()
            .search_mode_enabled(false)
            .child(&self.search_entry)
            .build();

        let search_btn = gtk::ToggleButton::builder()
            .icon_name("system-search-symbolic")
            .tooltip_text("Search")
            .build();
        search_btn
            .bind_property("active", &search_bar, "search-mode-enabled")
            .bidirectional()
            .build();
        let style_manager = adw::StyleManager::default();
        let is_dark = style_manager.is_dark();
        let theme_btn = gtk::Button::builder()
            .icon_name(if is_dark { "weather-clear-symbolic" } else { "weather-clear-night-symbolic" })
            .tooltip_text(if is_dark { "Switch to Light Mode" } else { "Switch to Dark Mode" })
            .build();

        theme_btn.connect_clicked(move |_| {
            let sm = adw::StyleManager::default();
            if sm.is_dark() {
                sm.set_color_scheme(adw::ColorScheme::ForceLight);
            } else {
                sm.set_color_scheme(adw::ColorScheme::ForceDark);
            }
        });

        let tb_clone = theme_btn.clone();
        adw::StyleManager::default().connect_dark_notify(move |sm| {
            let dark = sm.is_dark();
            tb_clone.set_icon_name(if dark { "weather-clear-symbolic" } else { "weather-clear-night-symbolic" });
            tb_clone.set_tooltip_text(Some(if dark { "Switch to Light Mode" } else { "Switch to Dark Mode" }));
        });

        // Menu for extra actions (Import, Export All, Preferences, About)
        let menu = gio::Menu::new();
        menu.append(Some("Import Note…"), Some("win.import-note"));
        menu.append(Some("Backup All Notes…"), Some("win.backup-all"));
        menu.append(Some("Preferences"), Some("app.preferences"));
        menu.append(Some("About Jotlet"), Some("app.about"));

        let menu_btn = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Menu")
            .menu_model(&menu)
            .build();
        header.pack_end(&menu_btn);
        header.pack_end(&theme_btn);
        header.pack_end(&search_btn);

        let placeholder = adw::StatusPage::builder()
            .icon_name("document-new-symbolic")
            .title("No Notes")
            .description("Create your first sticky note")
            .build();
        self.list_box.set_placeholder(Some(&placeholder));

        let scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(&self.list_box)
            .build();

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&header);
        content.append(&search_bar);
        content.append(&scroll);

        self.window.set_content(Some(&content));

        let this = self.clone();
        self.search_entry.connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            this.filter_notes(&query);
        });

        let app_clone3 = app.clone();
        let this2 = self.clone();
        self.list_box.connect_row_activated(move |_, row| {
            let idx = row.index() as usize;
            let notes = this2.notes.borrow();
            if let Some(note) = notes.get(idx) {
                app_clone3.open_note_window(note.clone());
            }
        });

        self.window.connect_close_request(|win| {
            win.set_visible(false);
            glib::Propagation::Stop
        });

        self.setup_window_actions();
    }

    fn setup_window_actions(&self) {
        let this = self.clone();
        let action_import = gio::ActionEntry::builder("import-note")
            .activate(move |_, _, _| {
                this.action_import();
            })
            .build();

        let this2 = self.clone();
        let action_backup = gio::ActionEntry::builder("backup-all")
            .activate(move |_, _, _| {
                this2.action_backup_all();
            })
            .build();

        let action_group = gio::SimpleActionGroup::new();
        action_group.add_action_entries([action_import, action_backup]);
        self.window.insert_action_group("win", Some(&action_group));
    }

    fn action_import(&self) {
        let dialog = gtk::FileDialog::builder()
            .title("Import Note")
            .build();

        let this = self.clone();
        dialog.open(
            Some(&self.window),
            None::<&gio::Cancellable>,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        let res = match path.extension().and_then(|s| s.to_str()) {
                            Some("md") | Some("markdown") => {
                                crate::services::import::import_markdown(&path)
                            }
                            Some("json") => {
                                if let Ok(notes) =
                                    crate::services::import::import_json_backup(&path)
                                {
                                    if let Some(db) = this.app.database() {
                                        for n in notes {
                                            let _ = db.with_connection(|conn| {
                                                models::insert_note(conn, &n)
                                            });
                                        }
                                    }
                                    this.load_notes();
                                    return;
                                }
                                return;
                            }
                            _ => crate::services::import::import_plain_text(&path),
                        };

                        if let Ok(note) = res {
                            if let Some(db) = this.app.database() {
                                let _ = db
                                    .with_connection(|conn| models::insert_note(conn, &note));
                            }
                            this.app.open_note_window(note);
                            this.load_notes();
                        }
                    }
                }
            },
        );
    }

    fn action_backup_all(&self) {
        let dialog = gtk::FileDialog::builder()
            .title("Backup All Notes")
            .initial_name("jotlet_backup.json")
            .build();

        let this = self.clone();
        dialog.save(
            Some(&self.window),
            None::<&gio::Cancellable>,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        let notes = this.notes.borrow().clone();
                        if let Err(e) = crate::services::export::export_all_notes(&notes, &path) {
                            error!("Failed to backup notes: {}", e);
                        }
                    }
                }
            },
        );
    }

    pub fn load_notes(&self) {
        let Some(db) = self.app.database() else { return };
        match db.with_connection(models::get_all_notes) {
            Ok(notes) => {
                self.populate_list(&notes);
                self.notes.replace(notes);
            }
            Err(e) => error!("Failed to load notes: {}", e),
        }
    }

    fn filter_notes(&self, query: &str) {
        let Some(db) = self.app.database() else { return };
        if query.is_empty() {
            self.load_notes();
            return;
        }
        match db.with_connection(|conn| models::search_notes(conn, query)) {
            Ok(notes) => {
                self.populate_list(&notes);
                self.notes.replace(notes);
            }
            Err(e) => error!("Failed to search notes: {}", e),
        }
    }

    fn populate_list(&self, notes: &[Note]) {
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }

        for note in notes {
            let row = adw::ActionRow::builder()
                .title(&note.title)
                .subtitle(note.plain_text_preview())
                .activatable(true)
                .build();

            let color_dot = gtk::DrawingArea::builder()
                .width_request(12)
                .height_request(12)
                .valign(gtk::Align::Center)
                .build();

            let swatch = note.color.swatch_color().to_string();
            color_dot.set_draw_func(move |_, cr, width, height| {
                if let Ok(rgba) = gtk::gdk::RGBA::parse(&swatch) {
                    cr.set_source_rgba(
                        rgba.red() as f64,
                        rgba.green() as f64,
                        rgba.blue() as f64,
                        rgba.alpha() as f64,
                    );
                } else {
                    cr.set_source_rgba(0.9, 0.9, 0.9, 1.0);
                }
                cr.arc(
                    width as f64 / 2.0,
                    height as f64 / 2.0,
                    5.0,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                let _ = cr.fill();
            });

            // Delete button suffix
            let del_btn = gtk::Button::builder()
                .icon_name("user-trash-symbolic")
                .valign(gtk::Align::Center)
                .tooltip_text("Delete Note")
                .has_frame(false)
                .build();
            del_btn.add_css_class("flat");

            let this = self.clone();
            let note_id = note.id.clone();
            let note_title = note.title.clone();
            del_btn.connect_clicked(move |_| {
                let this_inner = this.clone();
                let nid = note_id.clone();
                let dialog = adw::AlertDialog::builder()
                    .heading("Delete Note?")
                    .body(format!("\"{}\" will be permanently deleted.", note_title))
                    .default_response("cancel")
                    .close_response("cancel")
                    .build();

                dialog.add_response("cancel", "Cancel");
                dialog.add_response("delete", "Delete");
                dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

                dialog.connect_response(None, move |_, response| {
                    if response == "delete" {
                        if let Some(db) = this_inner.app.database() {
                            let _ = db.with_connection(|conn| models::delete_note(conn, &nid));
                        }
                        this_inner.app.on_note_window_closed(&nid);
                        this_inner.load_notes();
                    }
                });

                dialog.present(Some(&this.window));
            });

            row.add_prefix(&color_dot);
            row.add_suffix(&del_btn);
            self.list_box.append(&row);
        }
    }
}
