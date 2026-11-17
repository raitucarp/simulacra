//! Typed keys of the SQLite `settings` table (no stringly-typed keys outside this file).

/// Generates [`SettingKey`] from one `Variant => "stored key"` table.
macro_rules! setting_keys {
    ($($(#[$doc:meta])* $variant:ident => $key:literal),* $(,)?) => {
        /// Every row key the app reads or writes in the `settings` table.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SettingKey {
            $($(#[$doc])* $variant,)*
        }

        impl SettingKey {
            /// Every key, in declaration order.
            pub const ALL: &'static [SettingKey] = &[$(SettingKey::$variant,)*];

            /// The key as stored in the `settings` table.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(SettingKey::$variant => $key,)*
                }
            }

            /// The typed key for a stored key string.
            pub fn from_key(key: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|candidate| candidate.as_str() == key)
            }
        }
    };
}

setting_keys! {
    /// Active network selection.
    ActiveNetwork => "active_network_id",
    /// Active authoring profile.
    ActiveProfile => "active_profile_id",
    /// Legacy global autonomous-scheduler switch; migrated into
    /// `preferences.agents.autonomous_posting` on first load, then removed.
    SchedulerEnabled => "scheduler_enabled",
    /// LLM provider identifier, credentials, model and sampling settings.
    LlmProvider => "llm_provider",
    LlmApiKey => "llm_api_key",
    LlmModel => "llm_model",
    LlmEndpoint => "llm_endpoint",
    LlmTemperature => "llm_temperature",
    LlmMaxTokens => "llm_max_tokens",
    /// JSON array of stored credentials (provider/service keys referenced by agents).
    Credentials => "llm.credentials",
    /// Active palette id (bundled or custom).
    ThemePresetId => "theme.preset_id",
    /// Partial palette JSON applied over the active palette.
    ThemeOverrides => "theme.overrides",
    /// JSON array of the user's custom palettes.
    ThemeCustomPalettes => "theme.custom_palettes",
    /// Spacing density, type scale, radius scale and interface font family.
    ThemeDensity => "theme.density",
    ThemeTypeScale => "theme.type_scale",
    ThemeRadiusScale => "theme.radius_scale",
    ThemeFontFamily => "theme.font_family",
    /// In-progress Agent Forge draft (JSON from `AgentDraft::to_autosave`).
    ForgeAutosave => "forge.autosave",
    /// Whether Mnemo the guide owl is hidden in the Agent Forge (`true` / `false`).
    ForgeHideGuide => "forge.hide_guide",
    /// Whether the first-run welcome was completed or skipped (`true` / `false`).
    OnboardingDone => "onboarding.done",
    /// JSON map of resizable layout id to its saved panel sizes (logical pixels).
    PanelLayouts => "ui.panel_layouts",
    /// JSON array of the seed profile ids the user deleted (never re-seeded).
    DismissedSeeds => "seeds.dismissed",
    /// Typed preferences, one JSON row per settings category (`domain::preferences`).
    PrefsGeneral => "preferences.general",
    PrefsAppearance => "preferences.appearance",
    PrefsWriting => "preferences.writing",
    PrefsAi => "preferences.ai",
    PrefsEmbeddings => "preferences.embeddings",
    PrefsImages => "preferences.images",
    PrefsAgents => "preferences.agents",
    PrefsLearning => "preferences.learning",
    PrefsImport => "preferences.import",
    PrefsPrivacy => "preferences.privacy",
    PrefsNotifications => "preferences.notifications",
    PrefsAdvanced => "preferences.advanced",
    /// Pre-palette serialized `ThemeTokens`; migrated on load, then removed.
    LegacyThemeTokens => "theme_tokens",
}
