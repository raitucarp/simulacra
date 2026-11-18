//! [`CommandId`]: every command the shell can run, with the stable wire key that names it in
//! the palette history and in the locale files.

/// Expands a `Variant => "wire_key"` table into the command enum, `ALL` and `wire_key()`.
macro_rules! command_ids {
    ($($(#[$doc:meta])* $variant:ident => $key:literal),* $(,)?) => {
        /// A command the shell dispatches (from the palette, a shortcut, a menu or a button).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum CommandId {
            $($(#[$doc])* $variant,)*
        }

        impl CommandId {
            /// Every command, in declaration order.
            pub const ALL: &'static [CommandId] = &[$(CommandId::$variant,)*];

            /// Stable identifier: persisted in the palette history and used in locale keys.
            /// Never rename one; add a new command instead.
            pub const fn wire_key(self) -> &'static str {
                match self {
                    $(CommandId::$variant => $key,)*
                }
            }

            /// The command whose [`Self::wire_key`] is `key`.
            pub fn from_wire_key(key: &str) -> Option<Self> {
                Self::ALL.iter().copied().find(|id| id.wire_key() == key)
            }
        }
    };
}

command_ids! {
    /// Screens of the primary navigation.
    GoTimeline => "go_timeline",
    GoKnowledge => "go_knowledge",
    GoAgents => "go_agents",
    GoPersonas => "go_personas",
    GoMindGraph => "go_mind_graph",
    /// sb C1: the Review page (learning commands below, sb C1b).
    GoReview => "go_review",
    /// Creation ("New" menu).
    NewThread => "new_thread",
    /// Quick capture of a note into the Inbox.
    NewNote => "new_note",
    /// Imports any file through the import sheet.
    ImportFile => "import_file",
    /// The timeline filtered to captured posts.
    OpenInbox => "open_inbox",
    NewAgent => "new_agent",
    CreateNetwork => "create_network",
    /// Identity and network.
    SwitchNetwork => "switch_network",
    SwitchPersona => "switch_persona",
    /// Pins or unpins the thread open in the inspector.
    PinThread => "pin_thread",
    /// Preferences.
    OpenSettings => "open_settings",
    OpenAppearance => "open_appearance",
    /// Window chrome.
    ToggleSidebar => "toggle_sidebar",
    ToggleRightSidebar => "toggle_right_sidebar",
    ShowShortcuts => "show_shortcuts",
    OpenPalette => "open_palette",
    OpenCommands => "open_commands",
    ReplayWelcome => "replay_welcome",
    /// sb B3: the palette in semantic mode, and "find similar" for the open thought.
    SemanticSearch => "semantic_search",
    FindSimilar => "find_similar",
    /// sb C1b: learn the thought open in the inspector; a new connection map.
    LearnPost => "learn_post",
    NewConnectionMap => "new_connection_map",
}
