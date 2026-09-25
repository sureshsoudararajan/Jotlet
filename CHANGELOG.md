# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-25

### Added
- **Core Architecture**: Native GTK4 + Libadwaita application written in Rust with SQLite storage.
- **Multiple Floating Notes**: Independent note windows with individual titles, colors, sizes, and states.
- **SQLite Persistence**: Automatic debounced saving (500ms), crash safety with WAL journal mode, and schema migrations.
- **Window State Memory**: Restores window dimensions, visibility, pin state, and colors across restarts.
- **Rich Text Editing**:
  - Bold (`Ctrl+B`), Italic (`Ctrl+I`), Underline (`Ctrl+U`), Strikethrough
  - Headings
  - Bulleted lists (`Ctrl+Shift+8`)
  - Numbered lists with auto-incrementing (`Ctrl+Shift+7`)
  - Interactive checklists (`Ctrl+Shift+9`) with click-to-toggle checkboxes (`☐` ↔ `☑`) and strikethrough formatting
  - Markdown table insertion
  - Clickable URLs with automatic link detection and default browser integration
- **Color Palette**: 9 pastel colors (Default, Yellow, Cream, Blue, Green, Purple, Pink, Orange, Gray) optimized for both Light and Dark modes.
- **Theme Support**: System (automatic GNOME desktop tracking), Light, and Dark modes via Libadwaita StyleManager.
- **Notes Overview Window**: Search notes in real-time by title and content, preview cards with color indicators, and direct note management.
- **Background Operation**: Application holds background execution via `gio::ApplicationHoldGuard` when note windows are closed; distinct Close Note vs Quit Application semantics.
- **Autostart Support**: XDG login autostart integration with `--background` mode toggleable in Preferences.
- **Import & Export**:
  - Export notes to Markdown (`.md`), Plain Text (`.txt`), HTML (`.html`), and JSON
  - Export all notes to JSON backup
  - Import notes from Markdown, Plain Text, and JSON backup
- **Arch Linux Packaging**: `PKGBUILD` for compiling and installing with `makepkg -si`.
- **Keyboard Shortcuts**: Quick shortcuts for formatting, searching, note creation, and preferences.
