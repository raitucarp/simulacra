//! Categorical tabs for the vertical settings navigation rail.

use crate::ui::icons::LucideIcon;
use rust_i18n::t;

/// Categorical tabs for the vertical settings navigation rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    AiIntelligence,
    Appearance,
    KnowledgeVault,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 4] = [
        SettingsTab::General,
        SettingsTab::AiIntelligence,
        SettingsTab::Appearance,
        SettingsTab::KnowledgeVault,
    ];

    /// i18n key of [`Self::label`].
    pub fn label_key(self) -> &'static str {
        match self {
            Self::General => "settings.tab_general",
            Self::AiIntelligence => "settings.tab_ai",
            Self::Appearance => "settings.tab_appearance",
            Self::KnowledgeVault => "settings.tab_vault",
        }
    }

    pub fn icon(self) -> LucideIcon {
        match self {
            Self::General => LucideIcon::Sliders,
            Self::AiIntelligence => LucideIcon::Bot,
            Self::Appearance => LucideIcon::Palette,
            Self::KnowledgeVault => LucideIcon::Brain,
        }
    }

    /// i18n key of [`Self::title`].
    pub fn title_key(self) -> &'static str {
        match self {
            Self::General => "settings.title_general",
            Self::AiIntelligence => "settings.title_ai",
            Self::Appearance => "settings.title_appearance",
            Self::KnowledgeVault => "settings.tab_vault",
        }
    }

    /// i18n key of [`Self::description`].
    pub fn description_key(self) -> &'static str {
        match self {
            Self::General => "settings.desc_general",
            Self::AiIntelligence => "settings.desc_ai",
            Self::Appearance => "settings.desc_appearance",
            Self::KnowledgeVault => "settings.desc_vault",
        }
    }

    /// Translated rail label.
    pub fn label(self) -> String {
        t!(self.label_key()).to_string()
    }

    /// Translated content-pane heading.
    pub fn title(self) -> String {
        t!(self.title_key()).to_string()
    }

    /// Translated content-pane subtitle.
    pub fn description(self) -> String {
        t!(self.description_key()).to_string()
    }
}
