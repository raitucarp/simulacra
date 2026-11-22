//! What one settings row shows: a typed preference, a theme scale, an action, a data folder
//! or a pane hosted inside the row list. Pure ids and text keys only.

use super::commands::SettingsCommand;
use crate::domain::preferences::SettingField;

/// Prefix of every row text key.
const ROW_KEY_PREFIX: &str = "prefs.row";

/// A theme scale owned by the Appearance pane (search finds it here).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeScale {
    Density,
    FontSize,
}

/// A folder of the local vault.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataFolder {
    Database,
    Media,
}

impl DataFolder {
    /// The command that opens this folder.
    pub const fn open_command(self) -> SettingsCommand {
        match self {
            Self::Database => SettingsCommand::OpenDatabaseFolder,
            Self::Media => SettingsCommand::OpenMediaFolder,
        }
    }
}

/// A larger control hosted as a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmbeddedPane {
    /// Provider grid, API key, endpoint and default model.
    AiProvider,
    /// Stored provider and service keys.
    Credentials,
    /// Queue status and the last model failure.
    SchedulerStatus,
    /// The theme gallery, palette editor and layout scales.
    Themes,
    /// The icon sets of Settings › Symbols (S13).
    SymbolSets,
    /// Who made the downloaded icon sets (decision #19: the only place naming them).
    Licences,
    /// The runtimes code blocks run in, and Refresh (P1).
    Runtimes,
}

/// One row of the settings window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingRow {
    Field(SettingField),
    Theme(ThemeScale),
    Action(SettingsCommand),
    Path(DataFolder),
    Embedded(EmbeddedPane),
}

impl SettingRow {
    /// Stable id, unique across the window (element ids and text keys).
    pub fn id(self) -> String {
        match self {
            Self::Field(field) => field.id(),
            Self::Theme(ThemeScale::Density) => "theme.density".into(),
            Self::Theme(ThemeScale::FontSize) => "theme.font_size".into(),
            Self::Action(command) => format!("command.{}", command.id()),
            Self::Path(DataFolder::Database) => "path.database".into(),
            Self::Path(DataFolder::Media) => "path.media".into(),
            Self::Embedded(pane) => format!("pane.{}", pane_key(pane)),
        }
    }

    /// i18n key of one of this row's texts (`title`, `description`, `keywords`).
    pub fn text_key(self, part: &str) -> String {
        format!("{ROW_KEY_PREFIX}.{}.{part}", self.id())
    }

    /// Whether the row is listed in its category (theme scales are edited in the hosted
    /// Appearance pane, so they only show up as search results).
    pub const fn listed_in_category(self) -> bool {
        !matches!(self, Self::Theme(_))
    }
}

const fn pane_key(pane: EmbeddedPane) -> &'static str {
    match pane {
        EmbeddedPane::AiProvider => "ai_provider",
        EmbeddedPane::Credentials => "credentials",
        EmbeddedPane::SchedulerStatus => "scheduler_status",
        EmbeddedPane::Themes => "themes",
        EmbeddedPane::SymbolSets => "symbol_sets",
        EmbeddedPane::Licences => "licences",
        EmbeddedPane::Runtimes => "runtimes",
    }
}
