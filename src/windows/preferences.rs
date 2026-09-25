use adw::prelude::*;

pub struct PreferencesWindow;

impl PreferencesWindow {
    pub fn build(app: &crate::application::JotletApplication) -> adw::PreferencesDialog {
        let dialog = adw::PreferencesDialog::builder()
            .title("Preferences")
            .build();

        Self::setup_ui(&dialog, app);
        dialog
    }

    fn setup_ui(dialog: &adw::PreferencesDialog, app: &crate::application::JotletApplication) {
        // Appearance page
        let appearance_page = adw::PreferencesPage::builder()
            .title("Appearance")
            .icon_name("applications-graphics-symbolic")
            .build();

        let appearance_group = adw::PreferencesGroup::builder()
            .title("Theme")
            .build();

        let theme_row = adw::ComboRow::builder()
            .title("Color Scheme")
            .subtitle("Choose the application appearance")
            .build();

        let theme_list = gtk::StringList::new(&["System", "Light", "Dark"]);
        theme_row.set_model(Some(&theme_list));

        let style_manager = adw::StyleManager::default();
        let current = match style_manager.color_scheme() {
            adw::ColorScheme::PreferDark | adw::ColorScheme::ForceDark => 2u32,
            adw::ColorScheme::PreferLight | adw::ColorScheme::ForceLight => 1u32,
            _ => 0u32,
        };
        theme_row.set_selected(current);

        theme_row.connect_selected_notify(move |row| {
            let manager = adw::StyleManager::default();
            let scheme = match row.selected() {
                1 => adw::ColorScheme::ForceLight,
                2 => adw::ColorScheme::ForceDark,
                _ => adw::ColorScheme::Default,
            };
            manager.set_color_scheme(scheme);
        });

        appearance_group.add(&theme_row);
        appearance_page.add(&appearance_group);

        // Typography group
        let typo_group = adw::PreferencesGroup::builder()
            .title("Typography")
            .description("Configure the default font for new sticky notes")
            .build();

        let initial_font_family = app
            .database()
            .map(|db| {
                db.with_connection(|conn| {
                    Ok(crate::database::models::get_string_setting(
                        conn,
                        "default_font_family",
                        "Default",
                    ))
                })
                .unwrap_or_else(|_| "Default".to_string())
            })
            .unwrap_or_else(|| "Default".to_string());

        let font_dialog = gtk::FontDialog::new();
        let font_btn = gtk::FontDialogButton::new(Some(font_dialog));
        font_btn.set_use_font(true);
        font_btn.set_use_size(false);
        font_btn.set_valign(gtk::Align::Center);
        if !initial_font_family.is_empty() && initial_font_family != "Default" {
            let desc = gtk::pango::FontDescription::from_string(&initial_font_family);
            font_btn.set_font_desc(&desc);
        }

        let font_row = adw::ActionRow::builder()
            .title("Default Font Family")
            .subtitle("Choose the typeface for notes (Adwaita, JetBrains Mono, DejaVu, etc.)")
            .activatable_widget(&font_btn)
            .build();
        font_row.add_suffix(&font_btn);

        let app_font = app.clone();
        font_btn.connect_font_desc_notify(move |btn| {
            if let Some(desc) = btn.font_desc() {
                if let Some(fam) = desc.family() {
                    let fam_str = fam.to_string();
                    if let Some(db) = app_font.database() {
                        let _ = db.with_connection(|conn| {
                            crate::database::models::set_string_setting(
                                conn,
                                "default_font_family",
                                &fam_str,
                            )
                        });
                    }
                }
            }
        });

        let initial_font_size = app
            .database()
            .map(|db| {
                db.with_connection(|conn| {
                    Ok(crate::database::models::get_string_setting(
                        conn,
                        "default_font_size",
                        "26",
                    ))
                })
                .unwrap_or_else(|_| "26".to_string())
            })
            .unwrap_or_else(|| "26".to_string());

        let size_row = adw::ComboRow::builder()
            .title("Default Font Size")
            .subtitle("Select default size for new notes")
            .build();

        let size_options = &[
            "14", "16", "18", "20", "22", "24", "26", "28", "32", "36", "48",
        ];
        let size_list = gtk::StringList::new(size_options);
        size_row.set_model(Some(&size_list));

        let size_idx = size_options
            .iter()
            .position(|&s| s == initial_font_size)
            .unwrap_or(6);
        size_row.set_selected(size_idx as u32);

        let app_size = app.clone();
        size_row.connect_selected_notify(move |row| {
            let sizes = [
                "14", "16", "18", "20", "22", "24", "26", "28", "32", "36", "48",
            ];
            let idx = row.selected() as usize;
            if let Some(&size_val) = sizes.get(idx) {
                if let Some(db) = app_size.database() {
                    let _ = db.with_connection(|conn| {
                        crate::database::models::set_string_setting(
                            conn,
                            "default_font_size",
                            size_val,
                        )
                    });
                }
            }
        });

        typo_group.add(&font_row);
        typo_group.add(&size_row);
        appearance_page.add(&typo_group);

        // General page
        let general_page = adw::PreferencesPage::builder()
            .title("General")
            .icon_name("preferences-other-symbolic")
            .build();

        let behavior_group = adw::PreferencesGroup::builder()
            .title("Behavior")
            .build();

        let start_on_login_row = adw::SwitchRow::builder()
            .title("Start Sticky Notes on login")
            .subtitle("Automatically start Jotlet in the background on desktop login")
            .active(crate::services::autostart::is_autostart_enabled())
            .build();

        start_on_login_row.connect_active_notify(|row| {
            if row.is_active() {
                let _ = crate::services::autostart::enable_autostart();
            } else {
                let _ = crate::services::autostart::disable_autostart();
            }
        });

        let initial_confirm_delete = app.database().map(|db| {
            db.with_connection(|conn| {
                Ok(crate::database::models::get_bool_setting(conn, "confirm_delete", true))
            }).unwrap_or(true)
        }).unwrap_or(true);

        let confirm_delete_row = adw::SwitchRow::builder()
            .title("Confirm before deleting")
            .subtitle("Show a confirmation dialog when deleting notes")
            .active(initial_confirm_delete)
            .build();

        let app_clone = app.clone();
        confirm_delete_row.connect_active_notify(move |row| {
            let active = row.is_active();
            if let Some(db) = app_clone.database() {
                let _ = db.with_connection(|conn| {
                    crate::database::models::set_bool_setting(conn, "confirm_delete", active)
                });
            }
        });

        behavior_group.add(&start_on_login_row);
        behavior_group.add(&confirm_delete_row);
        general_page.add(&behavior_group);

        dialog.add(&appearance_page);
        dialog.add(&general_page);
    }
}
