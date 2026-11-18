//! The tabs of the Journal page.

use crate::ui::icons::LucideIcon;

/// One tab of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum JournalTab {
    #[default]
    Today,
    Journal,
    Mood,
    Focus,
    Capsules,
}

impl JournalTab {
    pub const ALL: [Self; 5] = [Self::Today, Self::Journal, Self::Mood, Self::Focus, Self::Capsules];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Today => "today",
            Self::Journal => "journal",
            Self::Mood => "mood",
            Self::Focus => "focus",
            Self::Capsules => "capsules",
        }
    }

    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::Today => LucideIcon::Sun,
            Self::Journal => LucideIcon::NotebookPen,
            Self::Mood => LucideIcon::HeartPulse,
            Self::Focus => LucideIcon::Timer,
            Self::Capsules => LucideIcon::Mail,
        }
    }

    /// Locale key of its label.
    pub fn label_key(self) -> String {
        format!("journal.tab.{}", self.key())
    }
}
