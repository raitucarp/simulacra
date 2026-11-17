//! One-shot actions offered by the settings window, each routed by a stable command id (the
//! command palette can run the same ids).

/// An action button in the settings window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettingsCommand {
    /// Shows the first-run welcome again.
    ReplayWelcome,
    /// Re-embeds every post and document with the current model.
    ReindexEmbeddings,
    /// Stops the indexing that is running (sb B3).
    CancelReindex,
    OpenDatabaseFolder,
    OpenMediaFolder,
    /// Writes a copy of the vault next to it.
    Backup,
    /// Exports the vault (a stub until the export wave lands).
    Export,
    /// Forgets the cached illustration and avatar rasters.
    ClearCaches,
    /// Opens the keyboard shortcut sheet.
    OpenKeymap,
    /// Puts every preference back to its default.
    ResetAll,
}

impl SettingsCommand {
    /// Every command.
    pub const ALL: [SettingsCommand; 10] = [
        Self::ReplayWelcome, Self::ReindexEmbeddings, Self::CancelReindex, Self::OpenDatabaseFolder, Self::OpenMediaFolder,
        Self::Backup, Self::Export, Self::ClearCaches, Self::OpenKeymap, Self::ResetAll,
    ];

    /// Stable command id.
    pub const fn id(self) -> &'static str {
        match self {
            Self::ReplayWelcome => "onboarding.replay",
            Self::ReindexEmbeddings => "embeddings.reindex",
            Self::CancelReindex => "embeddings.cancel",
            Self::OpenDatabaseFolder => "data.open_database_folder",
            Self::OpenMediaFolder => "data.open_media_folder",
            Self::Backup => "data.backup",
            Self::Export => "data.export",
            Self::ClearCaches => "data.clear_caches",
            Self::OpenKeymap => "keymap.open",
            Self::ResetAll => "settings.reset_all",
        }
    }

    /// The command with `id`.
    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.id() == id)
    }

    /// Whether the button is drawn as destructive (and asks for a second click).
    pub const fn is_destructive(self) -> bool {
        matches!(self, Self::ResetAll)
    }
}
