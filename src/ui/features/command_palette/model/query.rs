//! The palette query: a leading `>` narrows the results to commands.

/// Prefix that switches the palette to commands only.
pub const COMMAND_PREFIX: char = '>';

/// What the palette searches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteMode {
    /// Screens, commands, people, posts, documents and settings.
    Everything,
    /// Commands only.
    Commands,
}

/// A parsed query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteQuery {
    pub mode: PaletteMode,
    /// The search text, trimmed, without the mode prefix.
    pub text: String,
}

impl PaletteQuery {
    /// Whether there is nothing to search for (the palette shows recent items instead).
    pub fn is_blank(&self) -> bool {
        self.text.is_empty()
    }
}

/// Parses the raw text of the palette's input.
pub fn parse_query(raw: &str) -> PaletteQuery {
    let trimmed = raw.trim_start();
    match trimmed.strip_prefix(COMMAND_PREFIX) {
        Some(rest) => PaletteQuery { mode: PaletteMode::Commands, text: rest.trim().to_string() },
        None => PaletteQuery { mode: PaletteMode::Everything, text: trimmed.trim().to_string() },
    }
}
