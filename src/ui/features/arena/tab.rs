//! The tabs of the Arena page.

use crate::ui::icons::LucideIcon;

/// One tab of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ArenaTab {
    #[default]
    Debate,
    Lab,
}

impl ArenaTab {
    pub const ALL: [Self; 2] = [Self::Debate, Self::Lab];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Debate => "debate",
            Self::Lab => "lab",
        }
    }

    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::Debate => LucideIcon::Swords,
            Self::Lab => LucideIcon::FlaskConical,
        }
    }

    /// Locale key of its label.
    pub fn label_key(self) -> String {
        format!("arena.tab.{}", self.key())
    }
}
