//! The information architecture: every category's grouped cards and the rows in each.

use super::commands::SettingsCommand as Cmd;
use super::rows::{DataFolder, EmbeddedPane as Pane, SettingRow, SettingRow::*, ThemeScale};
use crate::domain::preferences::{
    AdvancedField as Adv, AgentField as Agt, AiField as Ai, AppearanceField as App, EmbeddingField as Emb,
    GeneralField as Gen, ImageField as Img, ImportField as Imp, LearningField as Lrn, NotificationField as Ntf,
    PrivacyField, SettingField as F, WritingField as Wrt,
};
use crate::ui::features::settings::tabs::SettingsTab;

/// Prefix of every group title key.
const GROUP_KEY_PREFIX: &str = "prefs.group";

/// One card of a category: a titled list of rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsGroup {
    /// Stable key inside the category.
    pub key: &'static str,
    pub rows: &'static [SettingRow],
}

impl SettingsGroup {
    /// i18n key of the card title.
    pub fn title_key(&self, tab: SettingsTab) -> String {
        format!("{GROUP_KEY_PREFIX}.{}.{}", tab.id(), self.key)
    }
}

const fn group(key: &'static str, rows: &'static [SettingRow]) -> SettingsGroup {
    SettingsGroup { key, rows }
}

const GENERAL: &[SettingsGroup] = &[
    group("language", &[Field(F::General(Gen::Language)), Field(F::General(Gen::DateFormat)), Field(F::General(Gen::TimeFormat)), Field(F::General(Gen::NumberFormat)), Field(F::General(Gen::FirstWeekday))]),
    group("startup", &[Field(F::General(Gen::DefaultPersona)), Field(F::General(Gen::StartupScreen)), Action(Cmd::ReplayWelcome)]),
];
const APPEARANCE: &[SettingsGroup] = &[
    group("reading", &[Theme(ThemeScale::Density), Theme(ThemeScale::FontSize), Field(F::Appearance(App::ReadingWidth)), Field(F::Appearance(App::ReducedMotion))]),
    group("themes", &[Embedded(Pane::Themes)]),
];
const WRITING: &[SettingsGroup] = &[
    group("posts", &[Field(F::Writing(Wrt::PostLengthLimit)), Field(F::Writing(Wrt::AutoSplitThreads)), Field(F::Writing(Wrt::DefaultIdentity)), Field(F::Writing(Wrt::AutoArchive)), Field(F::Writing(Wrt::SymbolShorthand))]),
    group("drafts", &[Field(F::Writing(Wrt::AutosaveSeconds)), Field(F::Writing(Wrt::Spellcheck)), Field(F::Writing(Wrt::AtomicHints)), Field(F::Writing(Wrt::ReadabilityMeter))]),
    group("links", &[Field(F::Writing(Wrt::WikilinkClick)), Field(F::Writing(Wrt::HashtagClick))]),
];
const AI: &[SettingsGroup] = &[
    group("provider", &[Embedded(Pane::AiProvider)]),
    group("credentials", &[Embedded(Pane::Credentials)]),
    group("requests", &[Field(F::Ai(Ai::RequestTimeout)), Field(F::Ai(Ai::MaxRetries))]),
    group("budget", &[Field(F::Ai(Ai::DailyRequestBudget)), Field(F::Ai(Ai::DailyTokenCap)), Field(F::Ai(Ai::AskBeforeSpending))]),
];
const EMBEDDINGS: &[SettingsGroup] = &[
    group("model", &[Field(F::Embeddings(Emb::Provider)), Field(F::Embeddings(Emb::Model)), Field(F::Embeddings(Emb::Endpoint)), Field(F::Embeddings(Emb::Dimensions))]),
    group("chunking", &[Field(F::Embeddings(Emb::ChunkSize)), Field(F::Embeddings(Emb::ChunkOverlap))]),
    group("index", &[Field(F::Embeddings(Emb::SemanticSearch)), Action(Cmd::ReindexEmbeddings), Action(Cmd::CancelReindex)]),
];
const IMAGES: &[SettingsGroup] = &[
    group("engine", &[Field(F::Images(Img::DefaultEngine)), Field(F::Images(Img::Size)), Field(F::Images(Img::StylePrefix))]),
    group("avatars", &[Field(F::Images(Img::AutoAvatars))]),
];
const AGENTS: &[SettingsGroup] = &[
    group("autonomy", &[Field(F::Agents(Agt::AutonomousPosting)), Embedded(Pane::SchedulerStatus), Field(F::Agents(Agt::PostsPerHour)), Field(F::Agents(Agt::MaxConcurrent))]),
    group("quiet", &[Field(F::Agents(Agt::QuietHours)), Field(F::Agents(Agt::QuietStart)), Field(F::Agents(Agt::QuietEnd))]),
    group("replies", &[Field(F::Agents(Agt::ReplyDepthLimit))]),
];
const LEARNING: &[SettingsGroup] = &[
    group("reviews", &[Field(F::Learning(Lrn::DailyReviewLimit)), Field(F::Learning(Lrn::NewCardsPerDay)), Field(F::Learning(Lrn::DesiredRetention))]),
    group("reminder", &[Field(F::Learning(Lrn::ReviewReminder)), Field(F::Learning(Lrn::ReminderHour))]),
    group("weekly", &[Field(F::Learning(Lrn::WeeklyReminder)), Field(F::Learning(Lrn::WeeklyDay)), Field(F::Learning(Lrn::WeeklyHour))]), // sb E2
];
const IMPORT: &[SettingsGroup] = &[
    group("splitting", &[Field(F::Import(Imp::Segmentation)), Field(F::Import(Imp::Identity))]),
    group("files", &[Field(F::Import(Imp::MaxFileMb)), Field(F::Import(Imp::KeepOriginal))]),
];
const PRIVACY: &[SettingsGroup] = &[
    group("storage", &[Path(DataFolder::Database), Path(DataFolder::Media)]),
    group("maintenance", &[Action(Cmd::Backup), Action(Cmd::Export), Action(Cmd::EncryptedSnapshot), Action(Cmd::RestoreSnapshot), Action(Cmd::ClearCaches)]), // sb F2: export, snapshots
    group("privacy", &[Field(F::Privacy(PrivacyField::LinkPreviews)), Field(F::Privacy(PrivacyField::JournalInSearch)), Field(F::Privacy(PrivacyField::Telemetry))]),
];
const NOTIFICATIONS: &[SettingsGroup] = &[group(
    "toasts",
    &[Field(F::Notifications(Ntf::Toasts)), Field(F::Notifications(Ntf::Confirmations)), Field(F::Notifications(Ntf::AiFailures)), Field(F::Notifications(Ntf::AgentActivity))],
)];
const SHORTCUTS: &[SettingsGroup] = &[group("keymap", &[Action(Cmd::OpenKeymap)])];
const ADVANCED: &[SettingsGroup] = &[
    group("diagnostics", &[Field(F::Advanced(Adv::LogLevel)), Field(F::Advanced(Adv::DeveloperOptions))]),
    group("reset", &[Action(Cmd::ResetAll)]),
];

/// The grouped cards of `tab`, in display order.
pub fn groups(tab: SettingsTab) -> &'static [SettingsGroup] {
    match tab {
        SettingsTab::General => GENERAL,
        SettingsTab::Appearance => APPEARANCE,
        SettingsTab::ReadingWriting => WRITING,
        SettingsTab::AiIntelligence => AI,
        SettingsTab::Embeddings => EMBEDDINGS,
        SettingsTab::Images => IMAGES,
        SettingsTab::Agents => AGENTS,
        SettingsTab::Learning => LEARNING,
        SettingsTab::Import => IMPORT,
        SettingsTab::PrivacyData => PRIVACY,
        SettingsTab::Notifications => NOTIFICATIONS,
        SettingsTab::Shortcuts => SHORTCUTS,
        SettingsTab::Advanced => ADVANCED,
    }
}

/// Every `(category, row)` pair, in window order.
pub fn all_rows() -> Vec<(SettingsTab, SettingRow)> {
    SettingsTab::ALL
        .into_iter()
        .flat_map(|tab| groups(tab).iter().flat_map(move |group| group.rows.iter().map(move |row| (tab, *row))))
        .collect()
}
