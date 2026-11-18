//! The views of the Insights page, their icons, locale keys and palette commands.

use crate::ui::features::command_palette::CommandId;
use crate::ui::icons::LucideIcon;
use serde::{Deserialize, Serialize};

/// One view of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightsView {
    #[default]
    Overview,
    Network,
    Sets,
    Symmetry,
    Categories,
    Embedding,
    Activity,
}

impl InsightsView {
    pub const ALL: [Self; 7] = [Self::Overview, Self::Network, Self::Sets, Self::Symmetry, Self::Categories, Self::Embedding, Self::Activity];

    /// Stable key (locale keys `insights.view.<key>`, persisted state).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Network => "network",
            Self::Sets => "sets",
            Self::Symmetry => "symmetry",
            Self::Categories => "categories",
            Self::Embedding => "embedding",
            Self::Activity => "activity",
        }
    }

    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::Overview => LucideIcon::Gauge,
            Self::Network => LucideIcon::Network,
            Self::Sets => LucideIcon::Layers,
            Self::Symmetry => LucideIcon::GitBranch,
            Self::Categories => LucideIcon::GitCompareArrows,
            Self::Embedding => LucideIcon::Compass,
            Self::Activity => LucideIcon::Clock,
        }
    }

    /// The palette command opening this view.
    pub const fn command(self) -> CommandId {
        match self {
            Self::Overview => CommandId::InsightsOverview,
            Self::Network => CommandId::InsightsNetwork,
            Self::Sets => CommandId::InsightsSets,
            Self::Symmetry => CommandId::InsightsSymmetry,
            Self::Categories => CommandId::InsightsCategories,
            Self::Embedding => CommandId::InsightsEmbedding,
            Self::Activity => CommandId::InsightsActivity,
        }
    }

    /// The view a command opens (`go_insights` keeps the current view: `None`).
    pub fn of_command(id: CommandId) -> Option<Self> {
        Self::ALL.into_iter().find(|view| view.command() == id)
    }
}
