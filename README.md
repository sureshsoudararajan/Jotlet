# Jotlet 📝

A lightweight, fast, beautiful, native GNOME Sticky Notes application written in Rust using GTK4 and Libadwaita.

![Jotlet Icon](data/icons/hicolor/scalable/apps/com.example.Jotlet.svg)

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL_3.0_or_later-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK-4.16%2B-red.svg)](https://gtk.org/)
[![Libadwaita](https://img.shields.io/badge/Libadwaita-1.6%2B-purple.svg)](https://gnome.pages.gitlab.gnome.org/libadwaita/)

---

## Overview

**Jotlet** is a desktop Sticky Notes application designed specifically for the GNOME desktop environment and Arch Linux / Wayland. It prioritizes fast startup, low memory footprint, distraction-free interaction, and native GNOME look and feel.

Whether capturing daily logs, command-line snippets, DevOps notes, reminders, or checklists, Jotlet acts as a floating desktop workspace that stays running quietly in the background.

---

## ✨ Features

- **Floating Independent Windows**: Create as many sticky notes as needed, each with its own size, position, and color.
- **Native GNOME & Libadwaita UI**: Built with GTK4 and Libadwaita widgets, adapting automatically to system dark and light modes.
- **Rich Text Formatting**:
  - Bold (`Ctrl+B`), Italic (`Ctrl+I`), Underline (`Ctrl+U`), Strikethrough
  - Headings
  - Bulleted lists (`Ctrl+Shift+8`)
  - Auto-incrementing numbered lists (`Ctrl+Shift+7`)
  - **Interactive Checklists** (`Ctrl+Shift+9`): Click checkboxes (`☐` ↔ `☑`) to toggle completed status with instant strikethrough styling.
  - Basic Markdown table templates
  - Clickable URLs with automatic link detection and browser launching
- **Subtle Pastel Palette**: 9 custom-crafted colors (Default, Yellow, Cream, Blue, Green, Purple, Pink, Orange, Gray) tuned for both light and dark backgrounds.
- **Fast Debounced Auto-Save**: Changes persist automatically after a 500ms debounce interval without locking the UI.
- **Crash-Resilient SQLite Storage**: Uses SQLite with Write-Ahead Logging (WAL) for transactional, non-destructive persistence.
- **Background Daemon Architecture**: Closing note windows hides them while keeping the application running in the background. Reopen hidden notes anytime from the Notes Overview.
- **XDG Autostart**: Optional login startup via `~/.config/autostart/` with `--background` flag support.
- **Notes Overview & Search**: Dedicated management window with live real-time search across note titles and content.
- **Import & Export**:
  - Export single notes to Markdown (`.md`), Plain Text (`.txt`), HTML (`.html`), or JSON
  - Full backup of all notes to a single JSON archive
  - Import notes from Markdown, Plain Text, or JSON backups
- **Zero Telemetry**: 100% offline, privacy-first personal data handling. No network calls, accounts, or telemetry.

---

## 🏛️ Architectural Decisions

### Why Rust?
Rust provides memory safety, predictable zero-cost abstractions, fearless concurrency, and compiled native performance without a garbage collector. The resulting binary is self-contained, launches in milliseconds, and consumes minimal RAM (~25-35MB typical resident memory compared to ~250MB+ for Electron-based note apps).

### Why GTK4 & Libadwaita?
Rather than simulating a Linux desktop app using web views or non-native toolkits, Jotlet directly targets the modern GNOME platform:
- Native Wayland gesture and input handling
- First-class support for Libadwaita's style manager (seamless light/dark transitions)
- GNOME accessibility standards, tooltips, and keyboard navigation
- High-DPI crisp rendering and smooth animations

### Rich Text Editor: Native GtkTextView vs WebKitGTK
During architectural evaluation, two approaches were considered for rich text editing:
1. **WebKitGTK (contentEditable HTML)**: Trivial table and rich text support, but introduces a massive ~55MB binary overhead, high memory usage, security attack surfaces, and a non-native feel.
2. **Native GtkTextView with GtkTextTag (Chosen)**: Lightweight, ultra-fast, native keyboard behavior, zero external browser runtime dependencies, and a stripped binary size of only **4.4 MB**.

Jotlet uses `GtkTextView` paired with `GtkTextTag` for typography styles, interactive `GestureClick` controllers for clickable checkboxes and links, and standard markdown structures for tables.

### Wayland Window Geometry Persistence
On Wayland, compositors intentionally prevent client applications from positioning themselves at absolute desktop coordinates for security and sandboxing reasons. Jotlet saves and restores window dimensions (`width` and `height`) and remembers note visibility and pin states, allowing the Wayland compositor to manage window placement naturally.

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| --- | --- |
| `Ctrl + N` | Create a new sticky note |
| `Ctrl + Shift + N` | Open the Notes Overview window |
| `Ctrl + F` | Search notes |
| `Ctrl + ,` | Open Preferences |
| `Ctrl + B` | Toggle **Bold** |
| `Ctrl + I` | Toggle *Italic* |
| `Ctrl + U` | Toggle <u>Underline</u> |
| `Ctrl + Shift + 7` | Insert / toggle numbered list |
| `Ctrl + Shift + 8` | Insert / toggle bullet list |
| `Ctrl + Shift + 9` | Insert / toggle interactive checklist |
| `Ctrl + Z` | Undo typing |
| `Ctrl + Shift + Z` | Redo typing |
| `Ctrl + Q` | Quit Jotlet completely (closes all notes and daemon) |
| `Escape` | Dismiss popovers / close overview |

---

## 🎨 Note Color Palette

| Name | Light Background | Dark Background |
| --- | --- | --- |
| **Default** | `#fafafa` | `#2d2d2d` |
| **Yellow** | `#fef9e7` | `#3a3520` |
| **Cream** | `#faf6f0` | `#3a342a` |
| **Blue** | `#eef4fc` | `#232f3e` |
| **Green** | `#eafcef` | `#1e3326` |
| **Purple** | `#f4ecf9` | `#352440` |
| **Pink** | `#fceef3` | `#3e2230` |
| **Orange** | `#fff3e0` | `#3a2a18` |
| **Gray** | `#f0f0f0` | `#383838` |

---

## 📂 File & Data Storage Locations

In strict accordance with the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html):

| Data | Path |
| --- | --- |
| **SQLite Database** | `~/.local/share/jotlet/notes.db` |
| **Autostart Entry** | `~/.config/autostart/com.example.Jotlet.desktop` |
| **GSettings Schema** | `com.example.Jotlet` |

---

## 🛠️ Installation & Building

### Prerequisites on Arch Linux

```bash
sudo pacman -S --needed rustup gtk4 libadwaita sqlite pkgconf
rustup default stable
```

### Build from Source

```bash
git clone https://github.com/example/jotlet.git
cd jotlet

# Compile debug build
cargo build

# Run directly
cargo run

# Build optimized release binary
cargo build --release
```

### Install with PKGBUILD (Arch Linux)

```bash
cd jotlet
makepkg -si
```

---

## 🧪 Testing

Jotlet comes with an automated integration test suite covering database operations, schema migrations, and export/import round-trips:

```bash
# Run all automated tests
cargo test

# Run strict clippy linting
cargo clippy -- -D warnings
```

---

## 🔒 Privacy & Offline Guarantee

Jotlet is completely offline software:
- **No analytics or telemetry**
- **No remote API or cloud dependencies**
- **No background network threads**
- **Your notes stay on your machine, always.**

---

## 🗺️ Roadmap

- [x] Multiple floating sticky notes
- [x] SQLite persistence with auto-save
- [x] Rich text formatting (Bold, Italic, Underline, Strikethrough)
- [x] Bullet and numbered lists
- [x] Interactive checklists (`☐` / `☑`)
- [x] 9 custom pastel color schemes with light/dark adaptations
- [x] Real-time notes search and overview window
- [x] Export (Markdown, HTML, Plain Text, JSON) and Import
- [x] Login autostart integration
- [x] Arch Linux PKGBUILD
- [ ] Inline reminder alarms
- [ ] Note grouping / tag categories
- [ ] Export note as PNG / image card

---

## 📜 License

Jotlet is licensed under the **GNU General Public License v3.0 or later** ([GPL-3.0-or-later](LICENSE)).
