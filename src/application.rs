use crate::config;
use crate::database::models::Note;
use crate::database::Database;
use crate::windows::note_window::NoteWindow;
use crate::windows::notes_overview::NotesOverview;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::gio;
use log::{error, info};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

// ============================================
// Subclass implementation
// ============================================

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct JotletApplication {
        pub database: RefCell<Option<Arc<Database>>>,
        pub note_windows: RefCell<HashMap<String, NoteWindow>>,
        pub overview_window: RefCell<Option<NotesOverview>>,
        pub hold_guard: RefCell<Option<gio::ApplicationHoldGuard>>,
        pub is_background: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for JotletApplication {
        const NAME: &'static str = "JotletApplication";
        type Type = super::JotletApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for JotletApplication {}

    impl ApplicationImpl for JotletApplication {
        fn activate(&self) {
            let app = self.obj();
            app.on_activate();
        }

        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            app.on_startup();
        }
    }

    impl GtkApplicationImpl for JotletApplication {}
    impl AdwApplicationImpl for JotletApplication {}
}

// ============================================
// Public API
// ============================================

glib::wrapper! {
    pub struct JotletApplication(ObjectSubclass<imp::JotletApplication>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl JotletApplication {
    /// Creates and runs the application, returning the exit code.
    pub fn run(is_background: bool) -> i32 {
        let app = glib::Object::builder::<Self>()
            .property("application-id", config::APP_ID)
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            .build();

        app.imp().is_background.set(is_background);
        app.run().into()
    }

    /// Called once during application startup.
    fn on_startup(&self) {
        info!("Application startup");
        self.load_resources();
        self.load_css();
        self.init_database();
        self.setup_actions();
        self.setup_accels();

        // Keep the application running in the background
        let guard = self.hold();
        self.imp().hold_guard.replace(Some(guard));
    }

    /// Called when the application is activated.
    fn on_activate(&self) {
        info!("Application activated");

        let is_bg = self.imp().is_background.get();
        self.imp().is_background.set(false);

        let imp = self.imp();
        let windows = imp.note_windows.borrow();
        if !windows.is_empty() {
            if !is_bg {
                self.show_overview();
                if let Some(win) = windows.values().next() {
                    win.present();
                }
            }
            return;
        }
        drop(windows);

        self.restore_notes();

        if !is_bg {
            self.show_overview();
            let windows = imp.note_windows.borrow();
            if windows.is_empty() {
                drop(windows);
                self.action_new_note();
            }
        }
    }

    fn load_resources(&self) {
        let resources =
            gio::Resource::load(concat!(env!("OUT_DIR"), "/jotlet.gresource"))
                .expect("Failed to load GResources");
        gio::resources_register(&resources);
    }

    fn load_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource(&format!("{}/style.css", config::RESOURCE_PATH));

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().expect("Could not get default display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn init_database(&self) {
        match Database::open() {
            Ok(db) => {
                self.imp().database.replace(Some(Arc::new(db)));
                info!("Database initialized successfully");
            }
            Err(e) => {
                error!("Failed to initialize database: {}", e);
            }
        }
    }

    fn setup_actions(&self) {
        let action_new = gio::ActionEntry::builder("new-note")
            .activate(|app: &Self, _, _| app.action_new_note())
            .build();

        let action_show = gio::ActionEntry::builder("show-notes")
            .activate(|app: &Self, _, _| app.action_show_notes())
            .build();

        let action_preferences = gio::ActionEntry::builder("preferences")
            .activate(|app: &Self, _, _| app.action_preferences())
            .build();

        let action_search = gio::ActionEntry::builder("search")
            .activate(|app: &Self, _, _| app.action_show_notes())
            .build();

        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| app.action_about())
            .build();

        let action_quit = gio::ActionEntry::builder("quit")
            .activate(|app: &Self, _, _| app.action_quit())
            .build();

        self.add_action_entries([
            action_new,
            action_show,
            action_preferences,
            action_search,
            action_about,
            action_quit,
        ]);
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.new-note", &["<Control>n"]);
        self.set_accels_for_action("app.show-notes", &["<Control><Shift>n"]);
        self.set_accels_for_action("app.preferences", &["<Control>comma"]);
        self.set_accels_for_action("app.search", &["<Control>f"]);
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    pub fn database(&self) -> Option<Arc<Database>> {
        self.imp().database.borrow().clone()
    }

    fn restore_notes(&self) {
        let Some(db) = self.database() else {
            error!("No database available for restoring notes");
            return;
        };

        match db.with_connection(|conn| {
            let notes = crate::database::models::get_all_notes(conn)?;
            Ok(notes)
        }) {
            Ok(notes) => {
                info!("Restoring {} notes", notes.len());
                for note in notes {
                    if note.is_visible {
                        self.open_note_window(note);
                    }
                }
            }
            Err(e) => error!("Failed to restore notes: {}", e),
        }
    }

    pub fn open_note_window(&self, note: Note) {
        let imp = self.imp();
        let mut windows = imp.note_windows.borrow_mut();

        if let Some(existing) = windows.get(&note.id) {
            existing.present();
            return;
        }

        let note_id = note.id.clone();
        let window = NoteWindow::new(self, note);
        window.present();
        windows.insert(note_id, window);
    }

    pub fn on_note_window_closed(&self, note_id: &str) {
        self.imp().note_windows.borrow_mut().remove(note_id);
    }

    fn action_new_note(&self) {
        let note = Note::new();
        if let Some(db) = self.database() {
            if let Err(e) = db.with_connection(|conn| {
                crate::database::models::insert_note(conn, &note)?;
                Ok(())
            }) {
                error!("Failed to save new note: {}", e);
            }
        }
        self.open_note_window(note);
    }

    pub fn show_overview(&self) {
        let imp = self.imp();
        let mut ov = imp.overview_window.borrow_mut();
        if let Some(ref overview) = *ov {
            overview.present();
        } else {
            let overview = NotesOverview::new(self);
            overview.present();
            *ov = Some(overview);
        }
    }

    pub fn refresh_overview(&self) {
        if let Some(ref ov) = *self.imp().overview_window.borrow() {
            ov.load_notes();
        }
    }

    fn action_show_notes(&self) {
        self.show_overview();
    }

    fn action_preferences(&self) {
        let dialog = crate::windows::preferences::PreferencesWindow::build();
        let win = self.active_window();
        dialog.present(win.as_ref());
    }

    fn action_about(&self) {
        let about = adw::AboutDialog::builder()
            .application_name(config::APP_NAME)
            .version(config::APP_VERSION)
            .developer_name("Jotlet Contributors")
            .license_type(gtk::License::Gpl30)
            .website(config::APP_WEBSITE)
            .application_icon(config::APP_ID)
            .comments(config::APP_DESCRIPTION)
            .build();

        let win = self.active_window();
        about.present(win.as_ref());
    }

    fn action_quit(&self) {
        info!("Quitting application");
        let imp = self.imp();
        let windows = imp.note_windows.borrow();
        for window in windows.values() {
            window.save_now();
        }
        drop(windows);
        self.imp().hold_guard.borrow_mut().take();
        self.quit();
    }
}
