use adw::prelude::*;

pub struct PreferencesWindow;

impl PreferencesWindow {
    pub fn build() -> adw::PreferencesDialog {
        let dialog = adw::PreferencesDialog::builder()
            .title("Preferences")
            .build();

        Self::setup_ui(&dialog);
        dialog
    }

    fn setup_ui(dialog: &adw::PreferencesDialog) {
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

        let confirm_delete_row = adw::SwitchRow::builder()
            .title("Confirm before deleting")
            .subtitle("Show a confirmation dialog when deleting notes")
            .active(true)
            .build();

        behavior_group.add(&start_on_login_row);
        behavior_group.add(&confirm_delete_row);
        general_page.add(&behavior_group);

        dialog.add(&appearance_page);
        dialog.add(&general_page);
    }
}
