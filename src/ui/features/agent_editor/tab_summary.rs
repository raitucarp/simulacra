//! Live summaries and horizontal sub-tab definitions for each [`AgentConfigTab`].

use super::config_tab::AgentConfigTab;
use crate::domain::Profile;
use crate::ui::icons::LucideIcon;
use rust_i18n::t;

const STATUS_ACTIVE: &str = "Active";
const STATUS_PAUSED: &str = "Paused";

impl AgentConfigTab {
    /// Returns a live 1-line summary of the profile's settings for this vertical category.
    pub fn live_summary(self, profile: &Profile) -> String {
        let cfg = &profile.config;
        match self {
            Self::IdentityAndBio => format!("{} · {}y", cfg.gender.short_label(), cfg.age),
            Self::PsychologyAndMbti => format!(
                "{} · {} · {}",
                cfg.mbti_full_code(),
                cfg.enneagram.code(),
                cfg.instinctual_variant.code()
            ),
            Self::IntelligenceAndMind => {
                format!("Gf {}% · EQ {}%", cfg.fluid_intelligence, cfg.emotional_intelligence)
            }
            Self::VoiceDictionGrammar => {
                format!("{} · {}", cfg.primary_language, cfg.diction_register.short_label())
            }
            Self::ScheduleAndMentions => {
                let status = if cfg.is_active { STATUS_ACTIVE } else { STATUS_PAUSED };
                format!("{} · {:02}:00–{:02}:00", status, cfg.active_hours_start, cfg.active_hours_end)
            }
            Self::PromptAndAssembly => format!("{} Sections", cfg.injected_section_count()),
        }
    }

    /// Returns the horizontal sub-tab labels and icons for this vertical category.
    pub fn sub_tabs(self) -> Vec<(String, LucideIcon)> {
        match self {
            Self::IdentityAndBio => vec![
                (t!("lab.sub_profile_vocation").to_string(), LucideIcon::User),
                (t!("lab.sub_demographics_mood").to_string(), LucideIcon::Sliders),
                (t!("lab.sub_bio_lifestyle").to_string(), LucideIcon::BookOpen),
            ],
            Self::PsychologyAndMbti => vec![
                (t!("lab.sub_jungian_mbti").to_string(), LucideIcon::Compass),
                (t!("lab.sub_big_five").to_string(), LucideIcon::Activity),
                (t!("lab.sub_enneagram").to_string(), LucideIcon::Sparkles),
                (t!("lab.sub_attachment_ethics").to_string(), LucideIcon::Heart),
            ],
            Self::IntelligenceAndMind => vec![
                (t!("lab.sub_cognitive_style").to_string(), LucideIcon::Brain),
                (t!("lab.sub_intelligence_quotients").to_string(), LucideIcon::Cpu),
                (t!("lab.sub_tempo_humor").to_string(), LucideIcon::Zap),
            ],
            Self::VoiceDictionGrammar => vec![
                (t!("lab.sub_language").to_string(), LucideIcon::Globe),
                (t!("lab.sub_grammar_diction").to_string(), LucideIcon::FileText),
                (t!("lab.sub_idiolect_quirks").to_string(), LucideIcon::MessageSquare),
            ],
            Self::ScheduleAndMentions => vec![
                (t!("lab.sub_posting_rhythm").to_string(), LucideIcon::Calendar),
                (t!("lab.sub_mention_conflict").to_string(), LucideIcon::Share2),
            ],
            Self::PromptAndAssembly => vec![
                (t!("lab.sub_prompt_assembly").to_string(), LucideIcon::Sliders),
                (t!("lab.sub_directive_template").to_string(), LucideIcon::Terminal),
                (t!("lab.sub_live_preview").to_string(), LucideIcon::Sparkles),
            ],
        }
    }
}
