//! The command registry: pure data describing each [`CommandId`] (icon, kind, availability)
//! plus its localized title and search keywords. Shortcuts come from the central keymap.
//!
//! To add a command: add a variant to `ids.rs`, a row to [`COMMANDS`], the locale keys
//! `palette.cmd.<wire_key>` and `palette.cmd.<wire_key>_keywords`, and an arm in the shell's
//! command dispatcher.

use super::ids::CommandId;
use crate::ui::icons::LucideIcon;
use rust_i18n::t;

/// Locale key prefix of command titles (`palette.cmd.<wire_key>`).
const TITLE_KEY_PREFIX: &str = "palette.cmd.";
/// Locale key suffix of the comma-separated search keywords.
const KEYWORDS_KEY_SUFFIX: &str = "_keywords";

/// Where a command is listed in the palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    /// Opens one of the primary screens ("Screens" group).
    Screen,
    /// Does something ("Commands" group).
    Action,
}

/// Whether a command can run yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    /// Listed and dispatched.
    Ready,
    /// Reserved for a later wave: hidden from the palette and menus until it ships.
    Planned,
    /// Works through its shortcut but is not offered in the palette (opening the palette itself).
    ShortcutOnly,
}

/// One registry row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandSpec {
    pub id: CommandId,
    pub icon: LucideIcon,
    pub kind: CommandKind,
    pub availability: Availability,
}

const fn screen(id: CommandId, icon: LucideIcon) -> CommandSpec {
    CommandSpec { id, icon, kind: CommandKind::Screen, availability: Availability::Ready }
}

const fn action(id: CommandId, icon: LucideIcon, availability: Availability) -> CommandSpec {
    CommandSpec { id, icon, kind: CommandKind::Action, availability }
}

use Availability::{Ready, ShortcutOnly};
use CommandId as C;
use LucideIcon as I;

/// Every command, in the order the palette lists them.
pub const COMMANDS: &[CommandSpec] = &[
    screen(C::GoTimeline, I::Activity),
    screen(C::GoKnowledge, I::BookOpen),
    screen(C::GoAgents, I::Users),
    screen(C::GoPersonas, I::UserCheck),
    screen(C::GoMindGraph, I::Share2),
    screen(C::GoReview, I::GraduationCap),
    screen(C::GoInsights, I::ChartColumn),
    screen(C::GoJournal, I::NotebookPen),
    action(C::NewThread, I::MessageSquarePlus, Ready),
    action(C::NewNote, I::StickyNote, Ready),
    action(C::ImportFile, I::FileUp, Ready),
    action(C::OpenInbox, I::Inbox, Ready),
    action(C::NewAgent, I::UserPlus, Ready),
    action(C::CreateNetwork, I::Boxes, Ready),
    action(C::SwitchNetwork, I::ChevronsUpDown, Ready),
    action(C::SwitchPersona, I::Repeat, Ready),
    action(C::PinThread, I::Pin, Ready),
    action(C::OpenSettings, I::Settings, Ready),
    action(C::OpenAppearance, I::Palette, Ready),
    action(C::ToggleSidebar, I::PanelLeft, Ready),
    action(C::ToggleRightSidebar, I::PanelRight, Ready),
    action(C::ShowShortcuts, I::Keyboard, Ready),
    action(C::OpenPalette, I::Search, ShortcutOnly),
    action(C::OpenCommands, I::Command, ShortcutOnly),
    action(C::ReplayWelcome, I::Sparkles, Ready),
    action(C::SemanticSearch, I::Sparkles, Ready),
    action(C::FindSimilar, I::Compass, Ready),
    action(C::LearnPost, I::GraduationCap, Ready),
    action(C::NewConnectionMap, I::Share2, Ready),
    action(C::PostActions, I::WandSparkles, Ready),
    action(C::ToggleOutline, I::ListTree, Ready),
    action(C::InsightsOverview, I::Gauge, Ready), action(C::InsightsNetwork, I::Network, Ready), action(C::InsightsSets, I::Layers, Ready),
    action(C::InsightsSymmetry, I::GitBranch, Ready), action(C::InsightsCategories, I::GitCompareArrows, Ready), action(C::InsightsEmbedding, I::Compass, Ready), action(C::InsightsActivity, I::Clock, Ready),
    action(C::ImageIdeas, I::ImagePlus, Ready),
    action(C::NewJournalEntry, I::Feather, Ready), action(C::LogMood, I::HeartPulse, Ready), action(C::StartFocus, I::Timer, Ready), action(C::NewTimeCapsule, I::Mail, Ready),
    action(C::InsightsCalibration, I::ChartScatter, Ready), action(C::InsightsTensions, I::Zap, Ready), action(C::ResolveDuePredictions, I::CalendarClock, Ready), action(C::ShowArgumentMap, I::Waypoints, Ready), // sb E1
];

/// The registry row of `id`.
pub fn spec(id: CommandId) -> &'static CommandSpec {
    COMMANDS.iter().find(|spec| spec.id == id).expect("every CommandId has a registry row")
}

impl CommandSpec {
    /// Whether the palette offers this command.
    pub fn is_listed(&self) -> bool {
        self.availability == Availability::Ready
    }

    /// Whether the command does something when dispatched.
    pub fn is_runnable(&self) -> bool {
        self.availability != Availability::Planned
    }

    /// Locale key of the title.
    pub fn title_key(&self) -> String {
        format!("{TITLE_KEY_PREFIX}{}", self.id.wire_key())
    }

    /// Localized title.
    pub fn title(&self) -> String {
        let key = self.title_key();
        t!(key.as_str()).to_string()
    }

    /// Localized, comma-separated search keywords.
    pub fn keywords(&self) -> String {
        let key = format!("{}{KEYWORDS_KEY_SUFFIX}", self.title_key());
        t!(key.as_str()).to_string()
    }
}
