//! What the palette can open: [`PaletteTarget`] and its searchable [`PaletteItem`].

use crate::domain::{PostId, ProfileId};
use crate::ui::features::command_palette::registry::CommandId;
use crate::ui::features::settings::SettingsTab;
use crate::ui::icons::LucideIcon;

/// A result group, in display order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PaletteGroup {
    Recent,
    Frequent,
    Screens,
    Commands,
    People,
    Posts,
    Documents,
    Settings,
}

impl PaletteGroup {
    /// Locale key of the group heading.
    pub const fn title_key(self) -> &'static str {
        match self {
            Self::Recent => "palette.group.recent",
            Self::Frequent => "palette.group.frequent",
            Self::Screens => "palette.group.screens",
            Self::Commands => "palette.group.commands",
            Self::People => "palette.group.people",
            Self::Posts => "palette.group.posts",
            Self::Documents => "palette.group.documents",
            Self::Settings => "palette.group.settings",
        }
    }
}

/// What running a result does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteTarget {
    Command(CommandId),
    Profile(ProfileId),
    Post(PostId),
    Document(String),
    Settings(SettingsTab),
}

impl PaletteTarget {
    /// Stable key of the target in the palette history.
    pub fn history_key(&self) -> String {
        match self {
            Self::Command(id) => format!("cmd:{}", id.wire_key()),
            Self::Profile(id) => format!("profile:{id}"),
            Self::Post(id) => format!("post:{id}"),
            Self::Document(id) => format!("doc:{id}"),
            Self::Settings(tab) => format!("settings:{}", tab.label_key()),
        }
    }

    /// Whether the target is a command (kept in command mode).
    pub fn is_command(&self) -> bool {
        matches!(self, Self::Command(_))
    }
}

/// One searchable result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteItem {
    pub target: PaletteTarget,
    pub group: PaletteGroup,
    pub title: String,
    /// Secondary text on the right of the title (handle, author, chunk count).
    pub detail: String,
    /// Extra searchable text that is not highlighted (keywords, post body).
    pub keywords: String,
    pub icon: LucideIcon,
    /// Chord shown on the right of the row.
    pub shortcut: Option<&'static str>,
}
