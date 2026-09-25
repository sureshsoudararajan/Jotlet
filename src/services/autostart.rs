//! Autostart management for XDG-compliant desktop sessions.
//!
//! Creates/removes a .desktop file in ~/.config/autostart/ to
//! start the application automatically on login.

use crate::config;
use log::info;
use std::path::PathBuf;

/// Returns the path to the autostart desktop file.
fn autostart_path() -> PathBuf {
    let config_dir = glib::user_config_dir().join("autostart");
    config_dir.join(format!("{}.desktop", config::APP_ID))
}

/// Enables autostart by creating the desktop file.
pub fn enable_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let path = autostart_path();

    // Ensure directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name={}\n\
         Exec=jotlet --background\n\
         Icon={}\n\
         Comment={}\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n\
         Hidden=false\n",
        config::APP_NAME,
        config::APP_ID,
        config::APP_DESCRIPTION
    );

    std::fs::write(&path, content)?;
    info!("Autostart enabled: {}", path.display());
    Ok(())
}

/// Disables autostart by removing the desktop file.
pub fn disable_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let path = autostart_path();
    if path.exists() {
        std::fs::remove_file(&path)?;
        info!("Autostart disabled: {}", path.display());
    }
    Ok(())
}

/// Checks if autostart is currently enabled.
pub fn is_autostart_enabled() -> bool {
    autostart_path().exists()
}
