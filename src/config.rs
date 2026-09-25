//! Centralized application configuration.
//! Change these constants to rebrand the application.

/// The application ID (reverse domain notation).
pub const APP_ID: &str = "com.example.Jotlet";

/// The human-readable application name.
pub const APP_NAME: &str = "Jotlet";

/// The application version.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The application description.
pub const APP_DESCRIPTION: &str = "A lightweight, native GNOME Sticky Notes application";

/// The GResource path prefix.
pub const RESOURCE_PATH: &str = "/com/example/Jotlet";

/// The application website.
pub const APP_WEBSITE: &str = "https://github.com/example/jotlet";

/// The application license (SPDX identifier).
pub const APP_LICENSE_SPDX: &str = "GPL-3.0-or-later";

/// XDG directory name for data storage.
pub const XDG_DIR_NAME: &str = "jotlet";

/// Database filename.
pub const DB_FILENAME: &str = "notes.db";

/// Default note width in pixels.
pub const DEFAULT_NOTE_WIDTH: i32 = 320;

/// Default note height in pixels.
pub const DEFAULT_NOTE_HEIGHT: i32 = 300;

/// Minimum note width in pixels.
pub const MIN_NOTE_WIDTH: i32 = 220;

/// Minimum note height in pixels.
pub const MIN_NOTE_HEIGHT: i32 = 180;

/// Auto-save debounce delay in milliseconds.
pub const AUTOSAVE_DELAY_MS: u32 = 500;
