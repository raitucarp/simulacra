//! Pure conversion of a gpui chord (`secondary-shift-p`) into the key caps shown to the user.

use rust_i18n::t;

/// Separator between the keys of a gpui chord.
const CHORD_SEPARATOR: char = '-';
/// Glyphs of keys that read better as symbols than words.
const UP_GLYPH: &str = "↑";
const DOWN_GLYPH: &str = "↓";
const LEFT_GLYPH: &str = "←";
const RIGHT_GLYPH: &str = "→";
const COMMAND_GLYPH: &str = "⌘";
const OPTION_GLYPH: &str = "⌥";

/// One key cap of a chord.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCap {
    Ctrl,
    Command,
    Shift,
    Alt,
    Enter,
    Escape,
    Tab,
    Space,
    Up,
    Down,
    Left,
    Right,
    /// A printable key, upper-cased (`P`, `/`, `1`).
    Char(String),
}

/// Whether the app runs with macOS key conventions.
pub const fn is_mac() -> bool {
    cfg!(target_os = "macos")
}

/// The key caps of `chord`, resolving `secondary` to Ctrl (or Cmd on macOS).
pub fn key_caps(chord: &str, mac: bool) -> Vec<KeyCap> {
    chord.split(CHORD_SEPARATOR).filter(|part| !part.is_empty()).map(|part| key_cap(part, mac)).collect()
}

fn key_cap(part: &str, mac: bool) -> KeyCap {
    match part {
        "secondary" if mac => KeyCap::Command,
        "secondary" | "ctrl" => KeyCap::Ctrl,
        "cmd" | "platform" => KeyCap::Command,
        "shift" => KeyCap::Shift,
        "alt" => KeyCap::Alt,
        "enter" => KeyCap::Enter,
        "escape" => KeyCap::Escape,
        "tab" => KeyCap::Tab,
        "space" => KeyCap::Space,
        "up" => KeyCap::Up,
        "down" => KeyCap::Down,
        "left" => KeyCap::Left,
        "right" => KeyCap::Right,
        other => KeyCap::Char(other.to_uppercase()),
    }
}

impl KeyCap {
    /// The localized text on the cap.
    pub fn label(&self, mac: bool) -> String {
        match self {
            Self::Ctrl => t!("keymap.key.ctrl").to_string(),
            Self::Command => COMMAND_GLYPH.to_string(),
            Self::Shift => t!("keymap.key.shift").to_string(),
            Self::Alt if mac => OPTION_GLYPH.to_string(),
            Self::Alt => t!("keymap.key.alt").to_string(),
            Self::Enter => t!("keymap.key.enter").to_string(),
            Self::Escape => t!("keymap.key.escape").to_string(),
            Self::Tab => t!("keymap.key.tab").to_string(),
            Self::Space => t!("keymap.key.space").to_string(),
            Self::Up => UP_GLYPH.to_string(),
            Self::Down => DOWN_GLYPH.to_string(),
            Self::Left => LEFT_GLYPH.to_string(),
            Self::Right => RIGHT_GLYPH.to_string(),
            Self::Char(text) => text.clone(),
        }
    }
}

/// Localized cap labels of `chord` on this platform.
pub fn chord_labels(chord: &str) -> Vec<String> {
    key_caps(chord, is_mac()).iter().map(|cap| cap.label(is_mac())).collect()
}
