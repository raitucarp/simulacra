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
    /// sb E1: how well stated credences matched what happened.
    Calibration,
    /// sb E1: posts and passages that may contradict each other.
    Tensions,
    /// sb F3: concepts over time.
    Timeline,
    /// sb F3: how varied the voices, stances and meanings read are.
    Diversity,
    /// sb F3: how easily the posts read.
    Readability,
}

impl InsightsView {
    pub const ALL: [Self; 12] = [Self::Overview, Self::Network, Self::Sets, Self::Symmetry, Self::Categories, Self::Embedding, Self::Activity, Self::Calibration, Self::Tensions, Self::Timeline, Self::Diversity, Self::Readability];

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
            Self::Calibration => "calibration",
            Self::Tensions => "tensions",
            Self::Timeline => "timeline",
            Self::Diversity => "diversity",
            Self::Readability => "readability",
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
            Self::Calibration => LucideIcon::ChartScatter,
            Self::Tensions => LucideIcon::Zap,
            Self::Timeline => LucideIcon::Activity,
            Self::Diversity => LucideIcon::Shuffle,
            Self::Readability => LucideIcon::BookOpen,
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
            Self::Calibration => CommandId::InsightsCalibration,
            Self::Tensions => CommandId::InsightsTensions,
            Self::Timeline => CommandId::InsightsTimeline,
            Self::Diversity => CommandId::InsightsDiversity,
            Self::Readability => CommandId::InsightsReadability,
        }
    }

    /// sb E1: the views fed by the thinking tools rather than the report (no filter bar).
    pub const fn is_thinking(self) -> bool {
        matches!(self, Self::Calibration | Self::Tensions)
    }

    /// sb F3: the lens views, fed by their own background report (with the filter bar).
    pub const fn is_lens(self) -> bool {
        matches!(self, Self::Timeline | Self::Diversity | Self::Readability)
    }

    /// The view a command opens (`go_insights` keeps the current view: `None`).
    pub fn of_command(id: CommandId) -> Option<Self> {
        Self::ALL.into_iter().find(|view| view.command() == id)
    }
}
