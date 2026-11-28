//! [`CommandCategory`] (M1): what a command is about, so the palette can list its commands
//! under headings. Seeded from the shortcut sheet's groups (Navigation, Writing); categories
//! without a command yet (Docs, Developer, Journey) are added with their first command.

use super::ids::CommandId;

/// A heading of the palette's commands, in display order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CommandCategory {
    /// Going somewhere: screens, panels, the palette itself, switching network or persona.
    Navigate,
    /// Making something new: threads, notes, agents, entries, debates, imports.
    Create,
    /// Acting on the thought open in the inspector.
    Post,
    /// Beliefs, predictions, questions and symbols.
    Thinking,
    /// Habits: mood, focus, the monthly review.
    Practice,
    Insights,
    /// Preferences, backups and help with the app.
    Settings,
}

impl CommandCategory {
    pub const ALL: [Self; 7] = [Self::Navigate, Self::Create, Self::Post, Self::Thinking, Self::Practice, Self::Insights, Self::Settings];

    /// Locale key of the heading.
    pub const fn title_key(self) -> &'static str {
        match self {
            Self::Navigate => "palette.category.navigate",
            Self::Create => "palette.category.create",
            Self::Post => "palette.category.post",
            Self::Thinking => "palette.category.thinking",
            Self::Practice => "palette.category.practice",
            Self::Insights => "palette.category.insights",
            Self::Settings => "palette.category.settings",
        }
    }
}

use CommandCategory as K;
use CommandId as C;

impl CommandId {
    /// The heading this command is listed under.
    pub const fn category(self) -> CommandCategory {
        match self {
            C::GoTimeline | C::GoKnowledge | C::GoAgents | C::GoPersonas | C::GoMindGraph | C::GoReview | C::GoInsights | C::GoJournal | C::GoArena | C::GoTasks | C::GoSearch => K::Navigate,
            C::OpenInbox | C::OpenCollection | C::ScheduledPosts | C::SwitchNetwork | C::SwitchPersona | C::ToggleSidebar | C::ToggleRightSidebar | C::OpenPalette | C::OpenCommands | C::SemanticSearch => K::Navigate,
            C::NewThread | C::NewNote | C::NewVoiceNote | C::NewDrawing | C::ImportFile | C::NewAgent | C::CreateNetwork | C::NewJournalEntry | C::NewTimeCapsule => K::Create,
            C::NewDebate | C::NewThoughtExperiment | C::NewConnectionMap | C::ImageIdeas => K::Create,
            C::PinThread | C::BookmarkPost | C::FindSimilar | C::LearnPost | C::PostActions | C::ToggleOutline | C::ShowArgumentMap | C::DebateOpenThought | C::ExperimentOpenThought => K::Post,
            C::ResolveDuePredictions | C::OpenQuestions | C::SymbolKey => K::Thinking,
            C::LogMood | C::StartFocus | C::MigrationReview => K::Practice,
            C::InsightsOverview | C::InsightsNetwork | C::InsightsSets | C::InsightsSymmetry | C::InsightsCategories | C::InsightsEmbedding | C::InsightsActivity => K::Insights,
            C::InsightsCalibration | C::InsightsTensions | C::InsightsTimeline | C::InsightsDiversity | C::InsightsReadability => K::Insights,
            C::OpenSettings | C::OpenAppearance | C::ShowShortcuts | C::ReplayWelcome | C::ExportBundle | C::EncryptedSnapshot | C::RestoreSnapshot => K::Settings,
        }
    }
}
