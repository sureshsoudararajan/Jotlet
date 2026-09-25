use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the available note colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NoteColor {
    #[default]
    Default,
    Yellow,
    Cream,
    Blue,
    Green,
    Purple,
    Pink,
    Orange,
    Gray,
}

impl NoteColor {
    /// Returns all available colors for the color picker.
    pub fn all() -> &'static [NoteColor] {
        &[
            NoteColor::Default,
            NoteColor::Yellow,
            NoteColor::Cream,
            NoteColor::Blue,
            NoteColor::Green,
            NoteColor::Purple,
            NoteColor::Pink,
            NoteColor::Orange,
            NoteColor::Gray,
        ]
    }

    /// Returns the CSS class name for this color.
    pub fn css_class(&self) -> &'static str {
        match self {
            NoteColor::Default => "note-color-default",
            NoteColor::Yellow => "note-color-yellow",
            NoteColor::Cream => "note-color-cream",
            NoteColor::Blue => "note-color-blue",
            NoteColor::Green => "note-color-green",
            NoteColor::Purple => "note-color-purple",
            NoteColor::Pink => "note-color-pink",
            NoteColor::Orange => "note-color-orange",
            NoteColor::Gray => "note-color-gray",
        }
    }

    /// Returns the display name for this color.
    pub fn display_name(&self) -> &'static str {
        match self {
            NoteColor::Default => "Default",
            NoteColor::Yellow => "Yellow",
            NoteColor::Cream => "Cream",
            NoteColor::Blue => "Blue",
            NoteColor::Green => "Green",
            NoteColor::Purple => "Purple",
            NoteColor::Pink => "Pink",
            NoteColor::Orange => "Orange",
            NoteColor::Gray => "Gray",
        }
    }

    /// Returns the swatch color for the color picker (light mode).
    pub fn swatch_color(&self) -> &'static str {
        match self {
            NoteColor::Default => "#f6f5f4",
            NoteColor::Yellow => "#f5e642",
            NoteColor::Cream => "#e8dcc8",
            NoteColor::Blue => "#8bb8ea",
            NoteColor::Green => "#7ee096",
            NoteColor::Purple => "#b98fd0",
            NoteColor::Pink => "#e882a0",
            NoteColor::Orange => "#f0a840",
            NoteColor::Gray => "#b8b7b4",
        }
    }

    /// Convert from a string stored in the database.
    pub fn from_str_lossy(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "yellow" => NoteColor::Yellow,
            "cream" => NoteColor::Cream,
            "blue" => NoteColor::Blue,
            "green" => NoteColor::Green,
            "purple" => NoteColor::Purple,
            "pink" => NoteColor::Pink,
            "orange" => NoteColor::Orange,
            "gray" | "grey" => NoteColor::Gray,
            _ => NoteColor::Default,
        }
    }
}

impl fmt::Display for NoteColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            NoteColor::Default => "default",
            NoteColor::Yellow => "yellow",
            NoteColor::Cream => "cream",
            NoteColor::Blue => "blue",
            NoteColor::Green => "green",
            NoteColor::Purple => "purple",
            NoteColor::Pink => "pink",
            NoteColor::Orange => "orange",
            NoteColor::Gray => "gray",
        })
    }
}
