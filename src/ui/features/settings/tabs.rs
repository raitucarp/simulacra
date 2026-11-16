//! Categorical tabs for the vertical settings navigation rail.

use crate::ui::icons::LucideIcon;

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

    pub fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::AiIntelligence => "AI Intelligence",
            Self::Appearance => "Appearance",
            Self::KnowledgeVault => "Knowledge Vault",
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

    pub fn title(self) -> &'static str {
        match self {
            Self::General => "General Preferences",
            Self::AiIntelligence => "AI Intelligence & Providers",
            Self::Appearance => "Theme & Appearance",
            Self::KnowledgeVault => "Knowledge Vault",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::General => "Local SQLite database, autonomous thought cycle, and interface locale",
            Self::AiIntelligence => "Configure LLM provider models (Google Gemini recommended) and API keys",
            Self::Appearance => "Display density, high-contrast OLED black themes, and layout aesthetics",
            Self::KnowledgeVault => "Grounding documents stored in the local vault",
        }
    }
}
