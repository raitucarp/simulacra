//! Typed keys of the non-trait illustration families, one variant per asset file, plus the
//! canvas size of every illustration kind (see `assets/illustrations/STYLE.md`).

/// Root of every illustration asset path.
pub const ILLUSTRATION_ROOT: &str = "illustrations";
/// File extension of illustration templates.
pub const ILLUSTRATION_EXT: &str = "svg";

/// Illustration family, which fixes the template canvas (viewBox) size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IllustrationKind {
    TraitCard,
    Step,
    Class,
    Guide,
    Empty,
    Portrait,
}

impl IllustrationKind {
    /// Template canvas (viewBox width, height) in SVG units.
    pub const fn canvas(self) -> (f32, f32) {
        match self {
            Self::TraitCard => (160.0, 120.0),
            Self::Step => (320.0, 120.0),
            Self::Class => (200.0, 240.0),
            Self::Guide => (128.0, 128.0),
            Self::Empty => (240.0, 160.0),
            Self::Portrait => (256.0, 256.0),
        }
    }
}

/// Generates a key enum with `ALL`, `KIND`, `key()` and `path()` for one asset folder.
macro_rules! art_keys {
    ($(#[$meta:meta])* $name:ident in $dir:literal as $kind:ident { $($variant:ident => $key:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            /// Every key, one per asset file.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];
            /// Illustration family (canvas size) of these assets.
            pub const KIND: IllustrationKind = IllustrationKind::$kind;

            /// Asset file stem.
            pub const fn key(self) -> &'static str {
                match self {
                    $(Self::$variant => $key,)+
                }
            }

            /// Asset path: `illustrations/<dir>/<key>.svg`.
            pub fn path(self) -> String {
                format!("{ILLUSTRATION_ROOT}/{}/{}.{ILLUSTRATION_EXT}", $dir, self.key())
            }
        }
    };
}

art_keys! {
    /// Banner of one Agent Forge step.
    StepKey in "steps" as Step {
        Origin => "origin", Appearance => "appearance", Attributes => "attributes",
        Birthsign => "birthsign", Mind => "mind", Disposition => "disposition", Voice => "voice",
        Relationships => "relationships", DailyLife => "daily_life", Grimoire => "grimoire",
        Review => "review",
        Embodiment => "embodiment", Rhetoric => "rhetoric", Engine => "engine",
    }
}

art_keys! {
    /// Full-height art of an archetype class.
    ClassKey in "classes" as Class {
        SocraticGadfly => "socratic_gadfly", StoicSage => "stoic_sage",
        EmpiricalScientist => "empirical_scientist", ExistentialPoet => "existential_poet",
        RationalistArchitect => "rationalist_architect", MysticVisionary => "mystic_visionary",
        CompassionateMentor => "compassionate_mentor", ContrarianTrickster => "contrarian_trickster",
    }
}

art_keys! {
    /// Pose of the guide owl.
    GuidePose in "guide" as Guide {
        Idle => "idle", Point => "point", Think => "think", Read => "read", Cheer => "cheer",
    }
}

art_keys! {
    /// Empty-state scene.
    EmptyKey in "empty" as Empty {
        NoPosts => "no_posts", NoAgents => "no_agents", NoDocuments => "no_documents",
        NoNetwork => "no_network", NoResults => "no_results", GraphEmpty => "graph_empty",
        VaultEmpty => "vault_empty", VaultNoMatches => "vault_no_matches",
        ReviewEmpty => "review_empty", ReviewDone => "review_done",
    }
}
