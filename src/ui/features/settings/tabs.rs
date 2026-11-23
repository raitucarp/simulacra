//! The categories of the settings window, in rail order, with their icon, header art and
//! translated texts.

use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{ILLUSTRATION_EXT, ILLUSTRATION_ROOT};
use super::text::tr;

/// Folder of the category header illustrations.
const ART_DIR: &str = "settings";
/// Prefix of every category text key.
const KEY_PREFIX: &str = "prefs.section";

/// A category of the settings window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsTab {
    General,
    Appearance,
    ReadingWriting,
    AiIntelligence,
    Embeddings,
    Images,
    Agents,
    Learning,
    Import,
    PrivacyData,
    Notifications,
    Shortcuts,
    /// Running code blocks (epic P).
    Code,
    Symbols,
    Licences,
    Advanced,
}

impl SettingsTab {
    /// Every category, in rail order.
    pub const ALL: [SettingsTab; 16] = [
        Self::General, Self::Appearance, Self::ReadingWriting, Self::AiIntelligence, Self::Embeddings,
        Self::Images, Self::Agents, Self::Learning, Self::Import, Self::PrivacyData, Self::Notifications,
        Self::Shortcuts, Self::Code, Self::Symbols, Self::Licences, Self::Advanced,
    ];

    /// Stable id (element ids, i18n keys, art file and the command palette).
    pub const fn id(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::Appearance => "appearance",
            Self::ReadingWriting => "reading_writing",
            Self::AiIntelligence => "ai_models",
            Self::Embeddings => "embeddings",
            Self::Images => "images",
            Self::Agents => "agents",
            Self::Learning => "learning",
            Self::Import => "import",
            Self::PrivacyData => "privacy_data",
            Self::Notifications => "notifications",
            Self::Shortcuts => "shortcuts",
            Self::Code => "code",
            Self::Symbols => "symbols",
            Self::Licences => "licences",
            Self::Advanced => "advanced",
        }
    }

    /// The category with `id`.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|tab| tab.id() == id)
    }

    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::General => LucideIcon::Sliders,
            Self::Appearance => LucideIcon::Palette,
            Self::ReadingWriting => LucideIcon::Edit,
            Self::AiIntelligence => LucideIcon::Bot,
            Self::Embeddings => LucideIcon::Brain,
            Self::Images => LucideIcon::Image,
            Self::Agents => LucideIcon::Users,
            Self::Learning => LucideIcon::BookOpen,
            Self::Import => LucideIcon::Archive,
            Self::PrivacyData => LucideIcon::Lock,
            Self::Notifications => LucideIcon::Volume2,
            Self::Shortcuts => LucideIcon::Keyboard,
            Self::Code => LucideIcon::Code,
            Self::Symbols => LucideIcon::Sparkles,
            Self::Licences => LucideIcon::Scale,
            Self::Advanced => LucideIcon::Terminal,
        }
    }

    /// The neighbour `offset` places away in rail order, wrapping at both ends.
    pub fn step(self, offset: isize) -> Self {
        let count = Self::ALL.len() as isize;
        let index = Self::ALL.iter().position(|tab| *tab == self).unwrap_or_default() as isize;
        Self::ALL[(index + offset).rem_euclid(count) as usize]
    }

    /// Header illustration: `illustrations/settings/<id>.svg`.
    pub fn illustration(self) -> String {
        format!("{ILLUSTRATION_ROOT}/{ART_DIR}/{}.{ILLUSTRATION_EXT}", self.id())
    }

    /// i18n key of one of this category's texts (`title`, `description`, `keywords`).
    pub fn text_key(self, part: &str) -> String {
        format!("{KEY_PREFIX}.{}.{part}", self.id())
    }

    /// Translated rail label and heading.
    pub fn title(self) -> String {
        tr(&self.text_key(TITLE))
    }

    /// Translated heading subtitle.
    pub fn description(self) -> String {
        tr(&self.text_key(DESCRIPTION))
    }

    /// Translated search keywords (comma separated).
    pub fn keywords(self) -> String {
        tr(&self.text_key(KEYWORDS))
    }
}

/// Text part names shared by categories, groups and rows.
pub(super) const TITLE: &str = "title";
pub(super) const DESCRIPTION: &str = "description";
pub(super) const KEYWORDS: &str = "keywords";
