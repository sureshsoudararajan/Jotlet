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

- **Floating Independent Windows**: Create as many sticky notes as needed, each with its own size, position, font, and color.
- **Native GNOME & Libadwaita UI**: Built with GTK4 and Libadwaita, adapting automatically to system dark and light modes with manual toggle support.
- **Clutter-Free Sticky Windows**: Distraction-free floating sticky notes with window controls removed from notes (`Ctrl+W` to close).
- **Typography & System Font Support**:
  - Full system font discovery via Fontconfig and Pango.
  - Native GTK4 font picker dialog (`gtk::FontDialogButton`) to browse, preview, and select from all installed system fonts.
  - Quick-pick toolbar font dropdown (`Adwaita Sans`, `Adwaita Mono`, `JetBrainsMono Nerd Font`, `DejaVu Sans`, `DejaVu Serif`, `Liberation Sans`, `Noto Sans`, etc.).
  - Configurable font size (14px–48px, default 26px).
  - Global Default Font Family and Size in Preferences.
  - Per-note font and size persistence in SQLite.
- **Smart Formatting & Lists**:
  - Bold (`Ctrl+B`), Italic (`Ctrl+I`), Underline (`Ctrl+U`), Strikethrough, Heading.
  - Bulleted lists (`•`) and auto-incrementing numbered lists (`1.`, `2.`, ...).
  - **Interactive Checklists**: Click checkboxes (`☐` ↔ `☑`) to toggle completed status with clean task strikethrough.
  - **Smart Enter Continuation**: Automatically continues checklists and lists upon Enter, cleanly exiting on empty lines.
  - Clickable URLs with automatic link detection and browser launching.
- **10 Pastel & Wallpaper Color Themes**:
  - 9 tuned pastel colors (Default, Yellow, Cream, Blue, Green, Purple, Pink, Orange, Gray) tuned for light and dark modes.
  - **Wallpaper (Auto)**: Dynamically samples your GNOME desktop wallpaper and derives matching light and dark pastel accents.
- **Notes Overview (Grid & List Views)**:
  - Default **Grid View** with responsive, pastel note cards.
  - Modern **Card-based List View** with color pills, bold titles, pin badges, clamped 2-line previews, and timestamps.
  - Real-time search across note titles and content.
- **Fast Debounced Auto-Save**: Changes persist automatically after a 500ms debounce interval without locking the UI.
- **Crash-Resilient SQLite Storage**: Uses SQLite with Write-Ahead Logging (WAL) and migration tracking.
- **Background Daemon Architecture**: Closing note windows hides them while keeping the application running in the background. Pinned notes automatically restore on system boot (`--background`).
- **XDG Autostart**: Optional login startup via `~/.config/autostart/`.
- **Import & Export**:
  - Export single notes to Markdown (`.md`), Plain Text (`.txt`), HTML (`.html`), or JSON.
  - Full backup of all notes to a single JSON archive.
  - Import notes from Markdown, Plain Text, or JSON backups.
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
| `Ctrl + W` | Close active note window (persists & hides) |
| `Ctrl + H` | Open the Notes Overview dashboard |
| `Ctrl + F` | Search notes in Overview |
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

| Name | Light Background | Dark Background | Description |
| --- | --- | --- | --- |
| **Default** | `#fafafa` | `#2d2d2d` | Neutral Adwaita background |
| **Yellow** | `#fef9e7` | `#3a3520` | Classic post-it pastel yellow |
| **Cream** | `#faf6f0` | `#3a342a` | Warm parchment |
| **Blue** | `#eef4fc` | `#232f3e` | Calming sky blue |
| **Green** | `#eafcef` | `#1e3326` | Fresh sage green |
| **Purple** | `#f4ecf9` | `#352440` | Subtle lavender |
| **Pink** | `#fceef3` | `#3e2230` | Soft blush pink |
| **Orange** | `#fff3e0` | `#3a2a18` | Warm peach |
| **Gray** | `#f0f0f0` | `#383838` | Sleek slate |
| **Wallpaper (Auto)** | *Adaptive Pastel* | *Adaptive Deep Pastel* | Dynamically samples GNOME desktop wallpaper |

---

## 📂 File & Data Storage Locations

In strict accordance with the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html):

| Data | Path |
| --- | --- |
| **SQLite Database** | `~/.local/share/jotlet/notes.db` |
| **Autostart Entry** | `~/.config/autostart/com.example.Jotlet.desktop` |
| **GSettings Schema** | `com.example.Jotlet` |

---

## 🛠️ Installation & Setup (Arch Linux)

### 1. Prerequisites

Ensure system build tools, GTK4, and Libadwaita are installed:

```bash
sudo pacman -S --needed rustup gtk4 libadwaita sqlite pkgconf git
rustup default stable
```

---

### 2. Installation Options

Clone the repository:

```bash
git clone https://github.com/sureshsoudararajan/Jotlet.git
cd Jotlet
```

#### Option A: Quick Automated Install (Recommended)

Run the included install script:

```bash
# System-wide installation (available to all users & desktop app grid):
sudo ./install.sh

# Or install for your current user only (no root / no password needed):
./install.sh
```

**What the installer does automatically:**
- Compiles the stripped, optimized release binary (`target/release/jotlet`).
- Installs binary to `/usr/bin/jotlet` (or `~/.local/bin/jotlet`).
- Registers GNOME desktop entry (`com.example.Jotlet.desktop`).
- Installs application vector icon (`com.example.Jotlet.svg`).
- Compiles GSettings preference schema (`com.example.Jotlet.gschema.xml`).
- Refreshes desktop and icon caches so **Jotlet immediately appears in your GNOME App Grid**.

#### Option B: Native Arch Package (`makepkg`)

Build and install a package managed directly by `pacman`:

```bash
makepkg -si
```

#### Option C: Run Without Installing (Development)

Run directly with Cargo:

```bash
cargo run
```

---

### 3. Launching Jotlet

- **From GNOME App Grid**: Press the Super key, search for **Jotlet**, and launch.
- **From Terminal**:
  ```bash
  jotlet
  ```
- **Silent Background Autostart**:
  ```bash
  jotlet --background
  ```

---

### 4. Uninstallation

If you ever need to remove Jotlet:

```bash
# If installed via sudo ./install.sh:
sudo ./uninstall.sh

# If installed via ./install.sh:
./uninstall.sh

# If installed via makepkg:
sudo pacman -R jotlet
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
- [x] Rich text formatting (Bold, Italic, Underline, Strikethrough, Heading)
- [x] Bullet and numbered lists with smart Enter continuation
- [x] Interactive checklists (`☐` / `☑`) with strikeout completion
- [x] 10 custom pastel and wallpaper-adaptive color schemes
- [x] System font typography selector and native GTK4 font dialog
- [x] Real-time notes search, Grid View, and Card-based List View
- [x] Export (Markdown, HTML, Plain Text, JSON) and Backup Import
- [x] Login autostart integration
- [x] Arch Linux PKGBUILD
- [ ] Inline reminder alarms
- [ ] Note grouping / tag categories
- [ ] Export note as PNG / image card

---

## 👤 Author

Developed by **Suresh Soundararajan**  
- **GitHub**: [@sureshsoudararajan](https://github.com/sureshsoudararajan)  
- **Repository**: [https://github.com/sureshsoudararajan/Jotlet](https://github.com/sureshsoudararajan/Jotlet)  
- **Issues & Feedback**: [https://github.com/sureshsoudararajan/Jotlet/issues](https://github.com/sureshsoudararajan/Jotlet/issues)  

---

## 📜 License

Jotlet is licensed under the **GNU General Public License v3.0 or later** ([GPL-3.0-or-later](LICENSE)).
