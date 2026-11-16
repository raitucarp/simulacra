//! The wizard steps of the Agent Forge and which of them each kind of profile walks through.

use crate::domain::trait_option::{TRAIT_ILLUSTRATION_EXT, TRAIT_ILLUSTRATION_ROOT};
use crate::domain::Profile;
use serde::{Deserialize, Serialize};

/// Root namespace of every Agent Forge i18n key.
pub const FORGE_I18N_ROOT: &str = "forge";
/// Directory (under the illustration root) holding one banner per step.
pub const STEP_BANNER_DIR: &str = "steps";
const STEP_GROUP: &str = "step";
const KIND_GROUP: &str = "kind";
const LABEL_SUFFIX: &str = "label";
const TAGLINE_SUFFIX: &str = "tagline";
const TIP_SUFFIX: &str = "tip";

/// What the forge is creating: an autonomous network agent or a user-controlled persona.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForgeKind {
    Agent,
    Persona,
}

impl ForgeKind {
    pub const ALL: [Self; 2] = [Self::Agent, Self::Persona];

    /// The kind matching an existing profile's role.
    pub fn of(profile: &Profile) -> Self {
        if profile.is_agent() {
            Self::Agent
        } else {
            Self::Persona
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Persona => "persona",
        }
    }

    /// i18n key of the kind's display name: `forge.kind.{key}`.
    pub fn label_key(self) -> String {
        format!("{FORGE_I18N_ROOT}.{KIND_GROUP}.{}", self.key())
    }
}

/// One page of the character-creator wizard, in walking order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ForgeStep {
    Origin,
    Appearance,
    Birthsign,
    Attributes,
    Disposition,
    Mind,
    Voice,
    DailyLife,
    Relationships,
    Grimoire,
    Review,
}

use ForgeStep as S;

/// Every step an agent walks through.
const AGENT_STEPS: [ForgeStep; 11] = [
    S::Origin, S::Appearance, S::Birthsign, S::Attributes, S::Disposition, S::Mind,
    S::Voice, S::DailyLife, S::Relationships, S::Grimoire, S::Review,
];
/// Personas only describe who they are, how they look and how they write.
const PERSONA_STEPS: [ForgeStep; 4] = [S::Origin, S::Appearance, S::Voice, S::Review];

/// The steps `kind` walks through, in order.
pub fn steps_for(kind: ForgeKind) -> &'static [ForgeStep] {
    match kind {
        ForgeKind::Agent => &AGENT_STEPS,
        ForgeKind::Persona => &PERSONA_STEPS,
    }
}

impl ForgeStep {
    pub const ALL: [Self; 11] = AGENT_STEPS;

    /// Stable snake_case id; also the banner file stem under `illustrations/steps/`.
    pub const fn key(self) -> &'static str {
        match self {
            S::Origin => "origin",
            S::Appearance => "appearance",
            S::Birthsign => "birthsign",
            S::Attributes => "attributes",
            S::Disposition => "disposition",
            S::Mind => "mind",
            S::Voice => "voice",
            S::DailyLife => "daily_life",
            S::Relationships => "relationships",
            S::Grimoire => "grimoire",
            S::Review => "review",
        }
    }

    /// Position in [`ForgeStep::ALL`].
    pub fn index(self) -> usize {
        Self::ALL.iter().position(|step| *step == self).unwrap_or_default()
    }

    /// The step with key `key`, if any.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|step| step.key() == key)
    }

    fn i18n(self, suffix: &str) -> String {
        format!("{FORGE_I18N_ROOT}.{STEP_GROUP}.{}.{suffix}", self.key())
    }

    /// i18n key of the step title.
    pub fn label_key(self) -> String {
        self.i18n(LABEL_SUFFIX)
    }

    /// i18n key of the one-line tagline under the title.
    pub fn tagline_key(self) -> String {
        self.i18n(TAGLINE_SUFFIX)
    }

    /// i18n key of Mnemo the owl's plain-language tip for this step.
    pub fn guide_tip_key(self) -> String {
        self.i18n(TIP_SUFFIX)
    }

    /// Asset path of the step banner: `illustrations/steps/{key}.svg`.
    pub fn banner(self) -> String {
        format!("{TRAIT_ILLUSTRATION_ROOT}/{STEP_BANNER_DIR}/{}.{TRAIT_ILLUSTRATION_EXT}", self.key())
    }

    /// The step after this one for `kind`, or `None` at the end.
    pub fn next(self, kind: ForgeKind) -> Option<Self> {
        let steps = steps_for(kind);
        let at = steps.iter().position(|step| *step == self)?;
        steps.get(at + 1).copied()
    }

    /// The step before this one for `kind`, or `None` at the start.
    pub fn previous(self, kind: ForgeKind) -> Option<Self> {
        let steps = steps_for(kind);
        let at = steps.iter().position(|step| *step == self)?;
        at.checked_sub(1).and_then(|prev| steps.get(prev).copied())
    }
}
