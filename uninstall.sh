#!/usr/bin/env bash
set -e

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
fi

echo "==> Uninstalling Jotlet from $PREFIX..."
rm -f "$BIN_DIR/jotlet"
rm -f "$APP_DIR/com.example.Jotlet.desktop"
rm -f "$ICON_DIR/com.example.Jotlet.svg"
rm -f "$SCHEMA_DIR/com.example.Jotlet.gschema.xml"

if command -v glib-compile-schemas &>/dev/null; then
    glib-compile-schemas "$SCHEMA_DIR"
fi

if command -v update-desktop-database &>/dev/null; then
    update-desktop-database "$APP_DIR" 2>/dev/null || true
fi

echo "==> Jotlet uninstalled successfully."
