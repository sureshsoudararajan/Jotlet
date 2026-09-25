use crate::application::JotletApplication;
use crate::config;
use crate::database::models::{self, Note};
use crate::models::note_color::NoteColor;
use crate::widgets::color_picker::ColorPicker;
use crate::widgets::note_editor::NoteEditor;
use crate::widgets::note_toolbar::NoteToolbar;

use adw::prelude::*;
use chrono::Utc;
use gtk::glib;
use gtk::gio;
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NoteWindow {
    window: adw::ApplicationWindow,
    note: Rc<RefCell<Note>>,
    title_entry: gtk::Entry,
    editor: NoteEditor,
    toolbar: NoteToolbar,
    color_picker: ColorPicker,
    pin_button: gtk::ToggleButton,
    content_box: gtk::Box,
    save_timeout_id: Rc<RefCell<Option<glib::SourceId>>>,
    app: JotletApplication,
}

impl NoteWindow {
    pub fn new(app: &JotletApplication, note: Note) -> Self {
        let initial_width = note.width.max(config::MIN_NOTE_WIDTH);
        let initial_height = note.height.max(config::MIN_NOTE_HEIGHT);

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(initial_width)
            .default_height(initial_height)
            .title(&note.title)
            .resizable(true)
            .build();

        window.set_size_request(config::MIN_NOTE_WIDTH, config::MIN_NOTE_HEIGHT);

        let note_rc = Rc::new(RefCell::new(note));
        let title_entry = gtk::Entry::builder()
            .placeholder_text("Note title…")
            .hexpand(true)
            .width_chars(6)
            .has_frame(false)
            .build();
        title_entry.add_css_class("note-title");
        title_entry.add_css_class("flat");

        let editor = NoteEditor::new();
        let toolbar = NoteToolbar::new();
        let color_picker = ColorPicker::new();

        let pin_button = gtk::ToggleButton::builder()
            .icon_name("view-pin-symbolic")
            .tooltip_text("Keep on top")
            .has_frame(false)
            .build();

        let content_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let save_timeout_id = Rc::new(RefCell::new(None));

        let note_window = Self {
            window,
            note: note_rc,
            title_entry,
            editor,
            toolbar,
            color_picker,
            pin_button,
            content_box,
            save_timeout_id,
            app: app.clone(),
        };

        note_window.setup_ui();
        note_window.load_note();
        note_window.connect_signals();

        note_window
    }

    pub fn present(&self) {
        self.window.present();
    }

    pub fn window(&self) -> &adw::ApplicationWindow {
        &self.window
    }

    pub fn note_id(&self) -> String {
        self.note.borrow().id.clone()
    }

    fn setup_ui(&self) {
        let header_bar = adw::HeaderBar::new();
        header_bar.set_show_start_title_buttons(false);
        header_bar.set_show_end_title_buttons(false);
        header_bar.set_decoration_layout(Some(""));
        header_bar.add_css_class("flat");
        header_bar.add_css_class("note-header");

        let color_menu_btn = gtk::MenuButton::builder()
            .icon_name("color-select-symbolic")
            .tooltip_text("Change color")
            .popover(self.color_picker.popover())
            .has_frame(false)
            .build();

        let style_manager = adw::StyleManager::default();
        let is_dark = style_manager.is_dark();
        let theme_btn = gtk::Button::builder()
            .icon_name(if is_dark { "weather-clear-symbolic" } else { "weather-clear-night-symbolic" })
            .tooltip_text(if is_dark { "Switch to Light Mode" } else { "Switch to Dark Mode" })
            .has_frame(false)
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

        let all_notes_btn = gtk::Button::builder()
            .icon_name("view-list-bullet-symbolic")
            .tooltip_text("All Notes (Ctrl+H)")
            .has_frame(false)
            .build();
        let app_show = self.app.clone();
        all_notes_btn.connect_clicked(move |_| {
            app_show.show_overview();
        });

        let new_note_btn = gtk::Button::builder()
            .icon_name("list-add-symbolic")
            .tooltip_text("New Note (Ctrl+N)")
            .has_frame(false)
            .build();
        let app_new = self.app.clone();
        new_note_btn.connect_clicked(move |_| {
            app_new.activate_action("new-note", None);
        });

        let menu = gio::Menu::new();
        menu.append(Some("All Notes (Ctrl+H)"), Some("app.show-notes"));
        menu.append(Some("New Note (Ctrl+N)"), Some("app.new-note"));
        menu.append(Some("Export…"), Some("win.export"));
        menu.append(Some("Preferences"), Some("app.preferences"));
        menu.append(Some("About Jotlet"), Some("app.about"));
        menu.append(Some("Close Note (Ctrl+W)"), Some("win.close"));
        menu.append(Some("Delete Note"), Some("win.delete"));

        let header_menu_btn = gtk::MenuButton::builder()
            .icon_name("view-more-symbolic")
            .tooltip_text("Note options")
            .menu_model(&menu)
            .has_frame(false)
            .build();

        header_bar.pack_start(&all_notes_btn);
        header_bar.pack_start(&new_note_btn);
        header_bar.set_title_widget(Some(&self.title_entry));
        header_bar.pack_end(&header_menu_btn);
        header_bar.pack_end(&theme_btn);
        header_bar.pack_end(&self.pin_button);
        header_bar.pack_end(&color_menu_btn);

        let scroll = gtk::ScrolledWindow::builder()
            .vexpand(true)
            .hexpand(true)
            .child(self.editor.text_view())
            .build();

        self.content_box.append(&header_bar);
        self.content_box.append(&scroll);
        self.content_box.append(self.toolbar.widget());

        self.window.set_content(Some(&self.content_box));

        let current_color = self.note.borrow().color;
        self.apply_note_color(&current_color);

        self.setup_window_actions();
    }

    fn load_note(&self) {
        let note = self.note.borrow();
        self.title_entry.set_text(&note.title);
        self.editor.set_content(&note.content);
        self.editor.set_font_family(&note.font_family);
        self.editor.set_font_size(note.font_size);
        self.toolbar.set_font_family(&note.font_family);
        self.toolbar.set_font_size(note.font_size);
        self.pin_button.set_active(note.pinned);
    }

    fn connect_signals(&self) {
        let this = self.clone();
        self.title_entry.connect_changed(move |_| {
            this.schedule_save();
        });

        let this2 = self.clone();
        self.editor.connect_changed(move || {
            this2.schedule_save();
        });

        let this3 = self.clone();
        self.pin_button.connect_toggled(move |btn| {
            this3.note.borrow_mut().pinned = btn.is_active();
            this3.schedule_save();
        });

        let this4 = self.clone();
        self.color_picker.connect_color_selected(move |color| {
            this4.set_note_color(color);
        });

        // Listen for system/preferences dark mode changes to update note theme
        let this_theme = self.clone();
        adw::StyleManager::default().connect_dark_notify(move |_| {
            let color = this_theme.note.borrow().color;
            this_theme.apply_note_color(&color);
        });

        let editor = self.editor.clone();
        self.toolbar.connect_format_action(move |action| {
            editor.apply_format(action);
        });

        let editor_font = self.editor.clone();
        let this_font = self.clone();
        self.toolbar.connect_font_family_changed(move |family| {
            editor_font.set_font_family(&family);
            this_font.note.borrow_mut().font_family = family;
            this_font.schedule_save();
        });

        let editor2 = self.editor.clone();
        let this_size = self.clone();
        self.toolbar.connect_font_size_changed(move |size| {
            editor2.set_font_size(size);
            this_size.note.borrow_mut().font_size = size;
            this_size.schedule_save();
        });

        let this7 = self.clone();
        self.window.connect_close_request(move |_| {
            this7.on_close_request();
            glib::Propagation::Proceed
        });
    }

    fn setup_window_actions(&self) {
        let this = self.clone();
        let action_delete = gio::ActionEntry::builder("delete")
            .activate(move |_, _, _| {
                this.action_delete();
            })
            .build();

        let this2 = self.clone();
        let action_export = gio::ActionEntry::builder("export")
            .activate(move |_, _, _| {
                this2.action_export();
            })
            .build();

        let this3 = self.clone();
        let action_close = gio::ActionEntry::builder("close")
            .activate(move |_, _, _| {
                this3.window.close();
            })
            .build();

        self.window
            .add_action_entries([action_delete, action_export, action_close]);
    }

    fn schedule_save(&self) {
        if let Some(id) = self.save_timeout_id.borrow_mut().take() {
            unsafe {
                gtk::glib::ffi::g_source_remove(id.as_raw());
            }
        }

        let this = self.clone();
        let id = glib::timeout_add_local_once(
            std::time::Duration::from_millis(config::AUTOSAVE_DELAY_MS as u64),
            move || {
                this.save_timeout_id.borrow_mut().take();
                this.save_now();
            },
        );
        self.save_timeout_id.replace(Some(id));
    }

    pub fn save_now(&self) {
        {
            let mut note = self.note.borrow_mut();
            note.title = self.title_entry.text().to_string();
            note.content = self.editor.get_content();
            note.font_family = self.editor.font_family();
            note.font_size = self.editor.font_size();

            // Record actual allocated window dimensions
            let current_w = self.window.width();
            let current_h = self.window.height();
            if current_w >= config::MIN_NOTE_WIDTH && current_h >= config::MIN_NOTE_HEIGHT {
                note.width = current_w;
                note.height = current_h;
            }

            note.updated_at = Utc::now().to_rfc3339();
        }

        let note = self.note.borrow().clone();
        if let Some(db) = self.app.database() {
            if let Err(e) = db.with_connection(|conn| {
                models::update_note(conn, &note)?;
                Ok(())
            }) {
                error!("Failed to save note {}: {}", note.id, e);
            }
        }

        self.app.refresh_overview();
    }

    fn on_close_request(&self) {
        self.save_now();

        {
            let mut note = self.note.borrow_mut();
            note.is_visible = false;
            note.updated_at = Utc::now().to_rfc3339();
        }

        let note = self.note.borrow().clone();
        if let Some(db) = self.app.database() {
            let _ = db.with_connection(|conn| {
                models::update_note(conn, &note)?;
                Ok(())
            });
        }

        self.app.refresh_overview();

        self.app.on_note_window_closed(&note.id);
    }

    fn set_note_color(&self, color: NoteColor) {
        self.note.borrow_mut().color = color;
        self.apply_note_color(&color);
        self.schedule_save();
    }

    fn apply_note_color(&self, color: &NoteColor) {
        for c in NoteColor::all() {
            self.content_box.remove_css_class(c.css_class());
        }
        self.content_box.add_css_class(color.css_class());

        let is_dark = adw::StyleManager::default().is_dark();
        if is_dark {
            self.content_box.add_css_class("dark");
        } else {
            self.content_box.remove_css_class("dark");
        }
    }

    fn action_delete(&self) {
        let confirm = self.app.database().map(|db| {
            db.with_connection(|conn| {
                Ok(models::get_bool_setting(conn, "confirm_delete", true))
            }).unwrap_or(true)
        }).unwrap_or(true);

        if !confirm {
            self.delete_note();
            return;
        }

        let this = self.clone();
        let dialog = adw::AlertDialog::builder()
            .heading("Delete Note?")
            .body("This note will be permanently deleted.")
            .default_response("cancel")
            .close_response("cancel")
            .build();

        dialog.add_response("cancel", "Cancel");
        dialog.add_response("delete", "Delete");
        dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);

        dialog.connect_response(None, move |_, response| {
            if response == "delete" {
                this.delete_note();
            }
        });

        dialog.present(Some(&self.window));
    }

    fn delete_note(&self) {
        let note_id = self.note.borrow().id.clone();
        if let Some(db) = self.app.database() {
            if let Err(e) = db.with_connection(|conn| {
                models::delete_note(conn, &note_id)?;
                Ok(())
            }) {
                error!("Failed to delete note {}: {}", note_id, e);
            }
        }

        self.app.on_note_window_closed(&note_id);
        self.window.close();
    }

    fn action_export(&self) {
        let note = self.note.borrow().clone();
        let clean_title = note.title.replace('/', "-");
        let dialog = gtk::FileDialog::builder()
            .title("Export Note")
            .initial_name(format!("{}.md", clean_title))
            .build();

        let this = self.clone();
        dialog.save(
            Some(&self.window),
            None::<&gio::Cancellable>,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        let format = match path.extension().and_then(|s| s.to_str()) {
                            Some("html") | Some("htm") => crate::services::export::ExportFormat::Html,
                            Some("txt") => crate::services::export::ExportFormat::PlainText,
                            Some("json") => crate::services::export::ExportFormat::Json,
                            _ => crate::services::export::ExportFormat::Markdown,
                        };
                        let note_data = this.note.borrow().clone();
                        if let Err(e) = crate::services::export::export_note(&note_data, &path, format) {
                            error!("Failed to export note: {}", e);
                        } else {
                            info!("Note exported to {}", path.display());
                        }
                    }
                }
            },
        );
    }
}
