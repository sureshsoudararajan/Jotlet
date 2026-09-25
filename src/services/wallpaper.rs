//! Wallpaper color extraction and adaptive palette generation for GNOME desktops.

use gtk::gio;
use gtk::prelude::*;
use log::info;

/// Represents an RGB color with values in 0..=255.
#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    /// Convert RGB (0..=255) to HSL: (H: 0..360, S: 0..1, L: 0..1)
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        let r = self.0 as f32 / 255.0;
        let g = self.1 as f32 / 255.0;
        let b = self.2 as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;
        if delta == 0.0 {
            return (0.0, 0.0, l);
        }

        let s = if l > 0.5 {
            delta / (2.0 - max - min)
        } else {
            delta / (max + min)
        };

        let mut h = if max == r {
            (g - b) / delta + (if g < b { 6.0 } else { 0.0 })
        } else if max == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        h *= 60.0;

        (h, s, l)
    }

    /// Convert HSL (H: 0..360, S: 0..1, L: 0..1) back to RGB (0..=255)
    pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
        if s == 0.0 {
            let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
            return Rgb(v, v, v);
        }

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let h_prime = h / 60.0;
        let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r1, g1, b1) = if (0.0..1.0).contains(&h_prime) {
            (c, x, 0.0)
        } else if (1.0..2.0).contains(&h_prime) {
            (x, c, 0.0)
        } else if (2.0..3.0).contains(&h_prime) {
            (0.0, c, x)
        } else if (3.0..4.0).contains(&h_prime) {
            (0.0, x, c)
        } else if (4.0..5.0).contains(&h_prime) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;

        Rgb(r, g, b)
    }

    /// Parse hex color string e.g. "#26a269" or "26a269"
    pub fn from_hex(s: &str) -> Option<Self> {
        let clean = s.trim().trim_start_matches('#');
        if clean.len() == 6 {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            Some(Rgb(r, g, b))
        } else {
            None
        }
    }
}

/// Detects the wallpaper base color from GNOME desktop background settings.
pub fn get_wallpaper_base_color() -> Rgb {
    // Check if schema org.gnome.desktop.background is installed
    if let Some(source) = gio::SettingsSchemaSource::default() {
        if source.lookup("org.gnome.desktop.background", true).is_some() {
            let settings = gio::Settings::new("org.gnome.desktop.background");
            let pcol = settings.string("primary-color");
            if let Some(rgb) = Rgb::from_hex(&pcol) {
                info!("Detected wallpaper primary color: {}", pcol);
                return rgb;
            }
        }
    }

    // Default GNOME fallback (Adwaita green #26a269)
    Rgb(38, 162, 105)
}

/// Generates custom dynamic CSS rules for the `.note-color-wallpaper` theme.
pub fn generate_wallpaper_css() -> String {
    let base = get_wallpaper_base_color();
    let (h, _, _) = base.to_hsl();

    // Light mode pastel palette:
    // Header & toolbar: soft tint, L ~ 86%, S ~ 32%
    let light_header = Rgb::from_hsl(h, 0.32, 0.86).to_hex();
    // Body: very light tint, L ~ 96%, S ~ 22%
    let light_body = Rgb::from_hsl(h, 0.22, 0.96).to_hex();

    // Dark mode pastel palette:
    // Header & toolbar: deep tone, L ~ 20%, S ~ 28%
    let dark_header = Rgb::from_hsl(h, 0.28, 0.20).to_hex();
    // Body: darker tone, L ~ 12%, S ~ 24%
    let dark_body = Rgb::from_hsl(h, 0.24, 0.12).to_hex();

    format!(
        "/* Dynamic Wallpaper Adaptive Note Colors */\n\
         .note-color-wallpaper .note-header {{ background-color: {light_header}; color: #2e3436; }}\n\
         .note-color-wallpaper .note-body {{ background-color: {light_body}; color: #2e3436; }}\n\
         .note-color-wallpaper .formatting-toolbar {{ background-color: {light_header}; color: #2e3436; }}\n\
         .note-color-wallpaper.note-grid-card {{ background-color: {light_body}; color: #2e3436; }}\n\
         \n\
         .dark.note-color-wallpaper .note-header, .dark .note-color-wallpaper .note-header {{ background-color: {dark_header}; color: #f6f5f4; }}\n\
         .dark.note-color-wallpaper .note-body, .dark .note-color-wallpaper .note-body {{ background-color: {dark_body}; color: #f6f5f4; }}\n\
         .dark.note-color-wallpaper .formatting-toolbar, .dark .note-color-wallpaper .formatting-toolbar {{ background-color: {dark_header}; color: #f6f5f4; }}\n\
         .dark.note-color-wallpaper.note-grid-card, .dark .note-color-wallpaper.note-grid-card {{ background-color: {dark_body}; color: #f6f5f4; }}\n"
    )
}
