#!/usr/bin/env bash
set -e

# Ensure release binary exists or build it
if [ ! -f "target/release/jotlet" ]; then
    echo "==> Building Jotlet (release profile)..."
    if [ -n "$SUDO_USER" ] && [ "$EUID" -eq 0 ]; then
        sudo -u "$SUDO_USER" cargo build --release
    else
        cargo build --release
    fi
else
    echo "==> Using release binary: target/release/jotlet"
    # Attempt rebuild if cargo is available to get freshest code, but don't fail if root lacks toolchain
    if [ -n "$SUDO_USER" ] && [ "$EUID" -eq 0 ]; then
        sudo -u "$SUDO_USER" cargo build --release 2>/dev/null || true
    fi
fi

# Determine target prefix
if [ "$EUID" -eq 0 ]; then
    PREFIX="/usr"
    SCHEMA_DIR="/usr/share/glib-2.0/schemas"
    APP_DIR="/usr/share/applications"
    ICON_DIR="/usr/share/icons/hicolor/scalable/apps"
    BIN_DIR="/usr/bin"
else
    PREFIX="$HOME/.local"
    SCHEMA_DIR="$HOME/.local/share/glib-2.0/schemas"
    APP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"
    BIN_DIR="$HOME/.local/bin"
    mkdir -p "$BIN_DIR" "$SCHEMA_DIR" "$APP_DIR" "$ICON_DIR"
fi

echo "==> Installing Jotlet to $PREFIX..."
install -Dm755 target/release/jotlet "$BIN_DIR/jotlet"
install -Dm644 data/com.example.Jotlet.desktop "$APP_DIR/com.example.Jotlet.desktop"
install -Dm644 data/icons/hicolor/scalable/apps/com.example.Jotlet.svg "$ICON_DIR/com.example.Jotlet.svg"
install -Dm644 data/com.example.Jotlet.gschema.xml "$SCHEMA_DIR/com.example.Jotlet.gschema.xml"

echo "==> Updating desktop & icon caches..."
if command -v glib-compile-schemas &>/dev/null; then
    glib-compile-schemas "$SCHEMA_DIR"
fi

if command -v update-desktop-database &>/dev/null; then
    update-desktop-database "$APP_DIR" 2>/dev/null || true
fi

if command -v gtk-update-icon-cache &>/dev/null; then
    gtk-update-icon-cache -qtf "${PREFIX}/share/icons/hicolor" 2>/dev/null || true
fi

echo ""
echo "✨ Jotlet installed successfully!"
echo "   • Binary:  $BIN_DIR/jotlet"
echo "   • Desktop: $APP_DIR/com.example.Jotlet.desktop"
echo "   • You can launch it from your GNOME App Grid or run 'jotlet' in terminal."
