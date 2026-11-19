//! The Learn tab's own state: which tool shows, and what each tool has open.

use crate::domain::learning::{GoalId, PalaceId, RoomKind, SessionId, Walk, WalkScope};
use crate::ui::icons::LucideIcon;

/// The four tools of the Learn tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LearnTool {
    #[default]
    Tutor,
    Goals,
    Gaps,
    Palace,
}

impl LearnTool {
    pub const ALL: [Self; 4] = [Self::Tutor, Self::Goals, Self::Gaps, Self::Palace];

    /// Stable key (element ids, locale keys).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Tutor => "tutor",
            Self::Goals => "goals",
            Self::Gaps => "gaps",
            Self::Palace => "palace",
        }
    }

    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::Tutor => LucideIcon::HelpCircle,
            Self::Goals => LucideIcon::Target,
            Self::Gaps => LucideIcon::Gauge,
            Self::Palace => LucideIcon::MapPin,
        }
    }
}

/// What the Goals tool shows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GoalsScreen {
    #[default]
    List,
    Goal(GoalId),
    NewGoal,
    /// The weekly review ritual.
    Weekly,
}

/// What the Palace tool shows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PalaceScreen {
    #[default]
    List,
    Edit(PalaceId),
    Walk(Walk),
}

/// Horizons offered for a new goal's target date, in days.
pub const GOAL_HORIZONS: [u32; 4] = [14, 30, 90, 180];
/// The horizon chosen by default.
pub const DEFAULT_HORIZON: u32 = 30;

/// The Learn tab's state.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LearningUi {
    pub tool: LearnTool,
    /// The tutor session shown (none: the start screen and the list).
    pub tutor_open: Option<SessionId>,
    /// Every exchange of the open dialogue shows (otherwise only the latest).
    pub tutor_all_exchanges: bool,
    /// Summary questions ticked for review items (by index); none chosen yet means all.
    pub tutor_unticked: Vec<usize>,
    pub goals: GoalsScreen,
    pub new_goal_days: u32,
    /// Weekly review actions left unticked (by index).
    pub weekly_unticked: Vec<usize>,
    /// The goal the weekly review's actions go to.
    pub weekly_goal: Option<GoalId>,
    /// The goal whose radar shows (none: every concept).
    pub gaps_goal: Option<GoalId>,
    /// The concept highlighted on the radar.
    pub gaps_selected: Option<String>,
    pub palace: PalaceScreen,
    /// The room and locus selected in the palace editor.
    pub palace_room: Option<String>,
    pub palace_slot: Option<usize>,
    pub new_room_kind: RoomKind,
    pub walk_scope: WalkScope,
    /// The locus a post picked in the picker goes to.
    pub pending_place: Option<(PalaceId, String, usize)>,
}

impl LearningUi {
    pub fn new() -> Self {
        Self { new_goal_days: DEFAULT_HORIZON, new_room_kind: RoomKind::Hall, ..Self::default() }
    }

    /// The walk in progress, if the palace tool shows one.
    pub fn walk(&self) -> Option<&Walk> {
        match &self.palace {
            PalaceScreen::Walk(walk) => Some(walk),
            _ => None,
        }
    }
}
