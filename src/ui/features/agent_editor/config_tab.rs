//! Roster mode and primary vertical category tabs of the editor.

use crate::ui::icons::LucideIcon;
use rust_i18n::t;

/// Distinguishes between autonomous Network Agents (members of the active network)
/// and controllable Human Personas (Persona Lab for switching active user identity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabMode {
    #[default]
    NetworkAgents,
    PersonaLab,
}

/// Primary vertical category tabs in the configuration workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentConfigTab {
    #[default]
    IdentityAndBio,
    PsychologyAndMbti,
    IntelligenceAndMind,
    VoiceDictionGrammar,
    ScheduleAndMentions,
    PromptAndAssembly,
}

const LABEL_IDENTITY: &str = "Identity & Bio";
const LABEL_PSYCHOLOGY: &str = "Psychology & MBTI";
const LABEL_INTELLIGENCE: &str = "Intelligence & Mind";
const LABEL_VOICE: &str = "Voice & Style";
const LABEL_SCHEDULE: &str = "Schedule & Social";
const LABEL_PROMPT: &str = "Prompt Engine";

impl AgentConfigTab {
    pub const ALL: [Self; 6] = [
        Self::IdentityAndBio,
        Self::PsychologyAndMbti,
        Self::IntelligenceAndMind,
        Self::VoiceDictionGrammar,
        Self::ScheduleAndMentions,
        Self::PromptAndAssembly,
    ];

    #[inline]
    pub fn localized_label(self) -> String {
        match self {
            Self::IdentityAndBio => t!("lab.cat_identity").to_string(),
            Self::PsychologyAndMbti => t!("lab.cat_psychology").to_string(),
            Self::IntelligenceAndMind => t!("lab.cat_intelligence").to_string(),
            Self::VoiceDictionGrammar => t!("lab.cat_voice").to_string(),
            Self::ScheduleAndMentions => t!("lab.cat_schedule").to_string(),
            Self::PromptAndAssembly => t!("lab.cat_prompt").to_string(),
        }
    }

    #[inline]
    pub const fn label(self) -> &'static str {
        match self {
            Self::IdentityAndBio => LABEL_IDENTITY,
            Self::PsychologyAndMbti => LABEL_PSYCHOLOGY,
            Self::IntelligenceAndMind => LABEL_INTELLIGENCE,
            Self::VoiceDictionGrammar => LABEL_VOICE,
            Self::ScheduleAndMentions => LABEL_SCHEDULE,
            Self::PromptAndAssembly => LABEL_PROMPT,
        }
    }

    #[inline]
    pub const fn icon(self) -> LucideIcon {
        match self {
            Self::IdentityAndBio => LucideIcon::User,
            Self::PsychologyAndMbti => LucideIcon::Brain,
            Self::IntelligenceAndMind => LucideIcon::Sparkles,
            Self::VoiceDictionGrammar => LucideIcon::MessageSquare,
            Self::ScheduleAndMentions => LucideIcon::Calendar,
            Self::PromptAndAssembly => LucideIcon::Terminal,
        }
    }
}
