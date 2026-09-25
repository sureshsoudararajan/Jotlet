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
    flow_box: gtk::FlowBox,
    stack: gtk::Stack,
    grid_stack: gtk::Stack,
    search_entry: gtk::SearchEntry,
    notes: Rc<RefCell<Vec<Note>>>,
    app: JotletApplication,
}

impl NotesOverview {
    pub fn new(app: &JotletApplication) -> Self {
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Notes")
            .default_width(540)
            .default_height(580)
            .build();

        let list_box = gtk::ListBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .margin_start(16)
            .margin_end(16)
            .margin_top(8)
            .margin_bottom(16)
            .build();
        list_box.add_css_class("note-overview-list");

        let flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .max_children_per_line(10)
            .min_children_per_line(2)
            .selection_mode(gtk::SelectionMode::None)
            .row_spacing(12)
            .column_spacing(12)
            .margin_start(16)
            .margin_end(16)
            .margin_top(12)
            .margin_bottom(16)
            .homogeneous(true)
            .build();

        let stack = gtk::Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::Crossfade);

        let grid_stack = gtk::Stack::new();
        grid_stack.set_transition_type(gtk::StackTransitionType::Crossfade);

        let search_entry = gtk::SearchEntry::builder()
            .placeholder_text("Search notes…")
            .hexpand(true)
            .build();

        let notes = Rc::new(RefCell::new(Vec::new()));

        let overview = Self {
            window,
            list_box,
            flow_box,
            stack,
            grid_stack,
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
            .tooltip_text("New Note (Ctrl+N)")
            .build();

        let app_clone = app.clone();
        new_btn.connect_clicked(move |_| {
            app_clone.activate_action("new-note", None);
        });
        header.pack_start(&new_btn);

        // View switcher (List / Grid)
        let view_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        view_box.add_css_class("linked");

        let list_btn = gtk::ToggleButton::builder()
            .icon_name("view-list-bullet-symbolic")
            .tooltip_text("List View")
            .active(true)
            .build();

        let grid_btn = gtk::ToggleButton::builder()
            .icon_name("view-grid-symbolic")
            .tooltip_text("Grid View")
            .group(&list_btn)
            .build();

        view_box.append(&list_btn);
        view_box.append(&grid_btn);
        header.pack_start(&view_box);

        let stack_l = self.stack.clone();
        let app_l = app.clone();
        list_btn.connect_toggled(move |btn| {
            if btn.is_active() {
                stack_l.set_visible_child_name("list");
                if let Some(db) = app_l.database() {
                    let _ = db.with_connection(|conn| {
                        models::set_string_setting(conn, "overview_view", "list")
                    });
                }
            }
        });

        let stack_g = self.stack.clone();
        let app_g = app.clone();
        grid_btn.connect_toggled(move |btn| {
            if btn.is_active() {
                stack_g.set_visible_child_name("grid");
                if let Some(db) = app_g.database() {
                    let _ = db.with_connection(|conn| {
                        models::set_string_setting(conn, "overview_view", "grid")
                    });
                }
            }
        });

        let search_bar = gtk::SearchBar::builder()
            .search_mode_enabled(false)
            .child(&self.search_entry)
            .build();

        let search_btn = gtk::ToggleButton::builder()
            .icon_name("system-search-symbolic")
            .tooltip_text("Search (Ctrl+F)")
            .build();
        search_btn
            .bind_property("active", &search_bar, "search-mode-enabled")
            .bidirectional()
            .build();

        let style_manager = adw::StyleManager::default();
        let is_dark = style_manager.is_dark();
        let theme_btn = gtk::Button::builder()
            .icon_name(if is_dark {
                "weather-clear-symbolic"
            } else {
                "weather-clear-night-symbolic"
            })
            .tooltip_text(if is_dark {
                "Switch to Light Mode"
            } else {
                "Switch to Dark Mode"
            })
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
        let this_dark = self.clone();
        adw::StyleManager::default().connect_dark_notify(move |sm| {
            let dark = sm.is_dark();
            tb_clone.set_icon_name(if dark {
                "weather-clear-symbolic"
            } else {
                "weather-clear-night-symbolic"
            });
            tb_clone.set_tooltip_text(Some(if dark {
                "Switch to Light Mode"
            } else {
                "Switch to Dark Mode"
            }));
            this_dark.sync_dark_mode(dark);
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

        let list_placeholder = adw::StatusPage::builder()
            .icon_name("document-new-symbolic")
            .title("No Notes")
            .description("Create your first sticky note")
            .build();
        self.list_box.set_placeholder(Some(&list_placeholder));

        let grid_placeholder = adw::StatusPage::builder()
            .icon_name("document-new-symbolic")
            .title("No Notes")
            .description("Create your first sticky note")
            .build();

        let list_scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .propagate_natural_width(false)
            .propagate_natural_height(false)
            .child(&self.list_box)
            .build();

        let grid_scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .propagate_natural_width(false)
            .propagate_natural_height(false)
            .child(&self.flow_box)
            .build();

        self.grid_stack.add_named(&grid_scroll, Some("notes"));
        self.grid_stack.add_named(&grid_placeholder, Some("empty"));

        self.stack.add_named(&list_scroll, Some("list"));
        self.stack.add_named(&self.grid_stack, Some("grid"));

        // Restore saved view preference (defaulting to grid on first launch)
        let initial_view = app
            .database()
            .map(|db| {
                db.with_connection(|conn| {
                    Ok(models::get_string_setting(conn, "overview_view", "grid"))
                })
                .unwrap_or_else(|_| "grid".to_string())
            })
            .unwrap_or_else(|| "grid".to_string());

        if initial_view == "list" {
            list_btn.set_active(true);
            self.stack.set_visible_child_name("list");
        } else {
            grid_btn.set_active(true);
            self.stack.set_visible_child_name("grid");
        }

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&header);
        content.append(&search_bar);
        content.append(&self.stack);

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

    fn sync_dark_mode(&self, is_dark: bool) {
        if is_dark {
            self.window.add_css_class("dark");
        } else {
            self.window.remove_css_class("dark");
        }
        let notes = self.notes.borrow().clone();
        self.populate_views(&notes);
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

        let this3 = self.clone();
        let action_close = gio::ActionEntry::builder("close")
            .activate(move |_, _, _| {
                this3.window.set_visible(false);
            })
            .build();

        self.window.add_action_entries([action_import, action_backup, action_close]);
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
                self.populate_views(&notes);
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
                self.populate_views(&notes);
                self.notes.replace(notes);
            }
            Err(e) => error!("Failed to search notes: {}", e),
        }
    }

    fn confirm_delete_note(&self, note_id: &str, note_title: &str) {
        let confirm = self
            .app
            .database()
            .map(|db| {
                db.with_connection(|conn| {
                    Ok(models::get_bool_setting(conn, "confirm_delete", true))
                })
                .unwrap_or(true)
            })
            .unwrap_or(true);

        let nid = note_id.to_string();
        if !confirm {
            if let Some(db) = self.app.database() {
                let _ = db.with_connection(|conn| models::delete_note(conn, &nid));
            }
            self.app.on_note_window_closed(&nid);
            self.load_notes();
            return;
        }

        let this = self.clone();
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
                if let Some(db) = this.app.database() {
                    let _ = db.with_connection(|conn| models::delete_note(conn, &nid));
                }
                this.app.on_note_window_closed(&nid);
                this.load_notes();
            }
        });

        dialog.present(Some(&self.window));
    }

    fn populate_views(&self, notes: &[Note]) {
        // 1. Populate List View
        while let Some(child) = self.list_box.first_child() {
            self.list_box.remove(&child);
        }

        let is_dark = adw::StyleManager::default().is_dark();

        for note in notes {
            let row_box = gtk::Box::new(gtk::Orientation::Horizontal, 12);
            row_box.add_css_class("note-list-card");
            if is_dark {
                row_box.add_css_class("dark");
            }

            // Left: Color pill / badge
            let color_pill = gtk::Box::new(gtk::Orientation::Vertical, 0);
            color_pill.add_css_class("note-list-pill");
            color_pill.add_css_class(&format!("swatch-color-{}", note.color));
            color_pill.set_valign(gtk::Align::Fill);
            row_box.append(&color_pill);

            // Middle: Title row + Preview text
            let middle_box = gtk::Box::new(gtk::Orientation::Vertical, 3);
            middle_box.set_hexpand(true);
            middle_box.set_valign(gtk::Align::Center);

            let title_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
            if note.pinned {
                let pin_icon = gtk::Image::from_icon_name("view-pin-symbolic");
                pin_icon.set_pixel_size(13);
                pin_icon.add_css_class("dim-label");
                title_row.append(&pin_icon);
            }

            let title_label = gtk::Label::builder()
                .label(&note.title)
                .hexpand(true)
                .xalign(0.0)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .build();
            title_label.add_css_class("note-list-title");
            title_row.append(&title_label);

            let preview_text = note.plain_text_preview();
            let preview_label = gtk::Label::builder()
                .label(if preview_text.is_empty() {
                    "Empty note"
                } else {
                    &preview_text
                })
                .xalign(0.0)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .lines(2)
                .wrap(true)
                .wrap_mode(gtk::pango::WrapMode::WordChar)
                .build();
            preview_label.add_css_class("note-list-preview");
            if preview_text.is_empty() {
                preview_label.add_css_class("dim-label");
            }

            middle_box.append(&title_row);
            middle_box.append(&preview_label);
            row_box.append(&middle_box);

            // Right: Date + Delete button
            let right_box = gtk::Box::new(gtk::Orientation::Horizontal, 8);
            right_box.set_valign(gtk::Align::Center);

            let date_str = note.updated_at.split('T').next().unwrap_or("");
            let date_label = gtk::Label::builder()
                .label(date_str)
                .valign(gtk::Align::Center)
                .build();
            date_label.add_css_class("note-list-date");
            date_label.add_css_class("dim-label");
            right_box.append(&date_label);

            let del_btn = gtk::Button::builder()
                .icon_name("user-trash-symbolic")
                .valign(gtk::Align::Center)
                .tooltip_text("Delete Note")
                .has_frame(false)
                .build();
            del_btn.add_css_class("flat");
            del_btn.add_css_class("circular");

            let this = self.clone();
            let note_id = note.id.clone();
            let note_title = note.title.clone();
            del_btn.connect_clicked(move |_| {
                this.confirm_delete_note(&note_id, &note_title);
            });
            right_box.append(&del_btn);

            row_box.append(&right_box);

            let list_row = gtk::ListBoxRow::builder()
                .child(&row_box)
                .activatable(true)
                .build();
            list_row.add_css_class("note-list-row-item");

            self.list_box.append(&list_row);
        }

        // 2. Populate Grid View
        while let Some(child) = self.flow_box.first_child() {
            self.flow_box.remove(&child);
        }

        if notes.is_empty() {
            self.grid_stack.set_visible_child_name("empty");
        } else {
            self.grid_stack.set_visible_child_name("notes");
        }

        for note in notes {
            let card = gtk::Box::new(gtk::Orientation::Vertical, 6);
            card.add_css_class("note-grid-card");
            card.add_css_class(note.color.css_class());
            if is_dark {
                card.add_css_class("dark");
            }
            card.set_width_request(180);
            card.set_height_request(140);

            // Card Header: Pin indicator + Title + Delete button
            let card_header = gtk::Box::new(gtk::Orientation::Horizontal, 6);

            if note.pinned {
                let pin_icon = gtk::Image::from_icon_name("view-pin-symbolic");
                pin_icon.set_pixel_size(13);
                pin_icon.add_css_class("dim-label");
                card_header.append(&pin_icon);
            }

            let title_label = gtk::Label::builder()
                .label(&note.title)
                .hexpand(true)
                .xalign(0.0)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .build();
            title_label.add_css_class("note-grid-title");
            card_header.append(&title_label);

            let del_btn = gtk::Button::builder()
                .icon_name("user-trash-symbolic")
                .tooltip_text("Delete Note")
                .has_frame(false)
                .valign(gtk::Align::Center)
                .build();
            del_btn.add_css_class("flat");
            del_btn.add_css_class("circular");

            let this_del = self.clone();
            let note_id = note.id.clone();
            let note_title = note.title.clone();
            del_btn.connect_clicked(move |_| {
                this_del.confirm_delete_note(&note_id, &note_title);
            });
            card_header.append(&del_btn);

            // Card Body: Plain text preview (bounded to 3 lines)
            let preview_text = note.plain_text_preview();
            let preview_label = gtk::Label::builder()
                .label(if preview_text.is_empty() {
                    "Empty note"
                } else {
                    &preview_text
                })
                .wrap(true)
                .wrap_mode(gtk::pango::WrapMode::WordChar)
                .lines(3)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .xalign(0.0)
                .valign(gtk::Align::Start)
                .vexpand(true)
                .build();
            preview_label.add_css_class("note-grid-preview");
            if preview_text.is_empty() {
                preview_label.add_css_class("dim-label");
            }

            // Card Footer: Updated timestamp
            let date_str = note.updated_at.split('T').next().unwrap_or("");
            let footer_label = gtk::Label::builder()
                .label(date_str)
                .xalign(0.0)
                .build();
            footer_label.add_css_class("note-grid-footer");
            footer_label.add_css_class("dim-label");

            card.append(&card_header);
            card.append(&preview_label);
            card.append(&footer_label);

            let flow_child = gtk::FlowBoxChild::new();
            flow_child.set_child(Some(&card));
            flow_child.set_focusable(true);

            let app_open = self.app.clone();
            let note_clone = note.clone();
            let gesture = gtk::GestureClick::new();
            gesture.connect_released(move |_, _, _, _| {
                app_open.open_note_window(note_clone.clone());
            });
            card.add_controller(gesture);

            self.flow_box.append(&flow_child);
        }
    }
}
