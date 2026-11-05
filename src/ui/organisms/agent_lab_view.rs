//! Unified Personas & Network Agents Workspace Organism.
//!
//! Implements a State-of-the-Art (SOTA) Three-Tier Progressive Disclosure Layout grounded in
//! cognitive psychology (Hick's Law, Miller's Law chunking, Gestalt proximity & figure-ground):
//! - **Tier 1 (Left Roster Pane)**: Compact vertical Contact List (`theme.contact_list_pane_width`)
//!   with segmented switcher (`Network Agents` vs `My Personas`), Avatar, Name, `@handle`, and MBTI badge.
//! - **Tier 2 (Vertical Category Rail)**: Primary configuration domains (`Identity & Bio`,
//!   `Psychology & MBTI`, `Intelligence & Mind`, `Voice & Style`, `Schedule & Social`, `Prompt Engine`)
//!   with live summary subtitles and 1-click Quick Archetype Presets.
//! - **Tier 3 (Horizontal Sub-Tabs & Focused Viewport)**: Peer sub-tabs within each vertical category
//!   that organize controls into neat, compact, single-viewport cards without visual overload.
//! - **100% Tokenized**: Every color, spacing, radius, font size, and dimension uses `ThemeTokens`.

use crate::domain::{
    AttachmentStyle, CapitalizationStyle, ClassicTemperament, CodeSwitchingStyle,
    CognitiveIntelligenceStyle, CognitivePace, ConflictHandlingStyle, DictionRegister, EmojiUsage,
    EnneagramType, EpistemicStance, GrammarStyle, HumanChronotype, HumanGender, HumorStyle,
    IdentityVariant, InstinctualVariant, LocusOfControl, MbtiType, MentionAttitude,
    MoralFoundation, PostingCadence, PrePostReasoningMode, Profile, ProfileId,
    PromptAssemblyStrategy, PunctuationHabit, SentenceLengthPreference,
};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

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
            Self::IdentityAndBio => "Identity & Bio",
            Self::PsychologyAndMbti => "Psychology & MBTI",
            Self::IntelligenceAndMind => "Intelligence & Mind",
            Self::VoiceDictionGrammar => "Voice & Style",
            Self::ScheduleAndMentions => "Schedule & Social",
            Self::PromptAndAssembly => "Prompt Engine",
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

    /// Returns a live 1-line summary of the profile's settings for this vertical category.
    pub fn live_summary(self, profile: &Profile) -> String {
        let cfg = &profile.config;
        match self {
            Self::IdentityAndBio => {
                format!("{} · {}y", cfg.gender.short_label(), cfg.age)
            }
            Self::PsychologyAndMbti => {
                format!(
                    "{} · {} · {}",
                    cfg.mbti_full_code(),
                    cfg.enneagram.code(),
                    cfg.instinctual_variant.code()
                )
            }
            Self::IntelligenceAndMind => {
                format!("Gf {}% · EQ {}%", cfg.fluid_intelligence, cfg.emotional_intelligence)
            }
            Self::VoiceDictionGrammar => {
                format!("{} · {}", cfg.primary_language, cfg.diction_register.short_label())
            }
            Self::ScheduleAndMentions => {
                let status = if cfg.is_active { "Active" } else { "Paused" };
                format!("{} · {:02}:00–{:02}:00", status, cfg.active_hours_start, cfg.active_hours_end)
            }
            Self::PromptAndAssembly => {
                format!("{} Sections", cfg.injected_section_count())
            }
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

/// Text input entities bound to the currently selected agent/persona in the right pane.
pub struct AgentEditorInputs<'a> {
    pub name_input: &'a Entity<InputState>,
    pub handle_input: &'a Entity<InputState>,
    pub description_input: &'a Entity<InputState>,
    pub occupation_input: &'a Entity<InputState>,
    pub location_input: &'a Entity<InputState>,
    pub pronouns_input: &'a Entity<InputState>,
    pub avatar_path_input: &'a Entity<InputState>,
    pub bio_input: &'a Entity<TextareaState>,
    pub hobbies_input: &'a Entity<InputState>,
    pub values_input: &'a Entity<InputState>,
    pub routine_input: &'a Entity<InputState>,
    pub primary_lang_input: &'a Entity<InputState>,
    pub secondary_lang_input: &'a Entity<InputState>,
    pub typing_quirks_input: &'a Entity<InputState>,
    pub signature_phrases_input: &'a Entity<InputState>,
    pub avoided_words_input: &'a Entity<InputState>,
    pub preferred_topics_input: &'a Entity<InputState>,
    pub mention_instruction_input: &'a Entity<TextareaState>,
    pub system_prompt_input: &'a Entity<TextareaState>,
    pub prompt_template_input: &'a Entity<TextareaState>,
}

pub type SelectProfileCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type SelectConfigTabCallback = Box<dyn Fn(AgentConfigTab, &mut Window, &mut App) + 'static>;
pub type SelectSubTabCallback = Box<dyn Fn(usize, &mut Window, &mut App) + 'static>;
pub type SwitchLabModeCallback = Box<dyn Fn(LabMode, &mut Window, &mut App) + 'static>;
pub type UpdateProfileCallback = Box<dyn Fn(Profile, &mut Window, &mut App) + 'static>;
pub type SaveTextInputsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type CreateProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type DeleteProfileCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type PickAvatarCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type ActivatePersonaCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type TriggerAgentSimCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;

/// Parameters for rendering the full Agent & Persona Lab Workspace.
pub struct AgentWorkspaceParams<'a> {
    pub state: &'a AppState,
    pub mode: LabMode,
    pub selected_profile_id: Option<&'a ProfileId>,
    pub active_config_tab: AgentConfigTab,
    pub active_sub_tab: usize,
    pub inputs: AgentEditorInputs<'a>,
    pub theme: &'a ThemeTokens,
    pub on_select_profile: SelectProfileCallback,
    pub on_select_tab: SelectConfigTabCallback,
    pub on_select_sub_tab: SelectSubTabCallback,
    pub on_switch_mode: SwitchLabModeCallback,
    pub on_update_profile: UpdateProfileCallback,
    pub on_save_text_inputs: SaveTextInputsCallback,
    pub on_create_profile: CreateProfileCallback,
    pub on_delete_profile: DeleteProfileCallback,
    pub on_pick_avatar: PickAvatarCallback,
    pub on_activate_persona: ActivatePersonaCallback,
    pub on_trigger_agent: TriggerAgentSimCallback,
}

/// Legacy helper signature preserved for compatibility.
pub fn render_agent_lab_view(
    profiles: &[Profile],
    theme: &ThemeTokens,
    on_trigger_simulation: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let agent_profiles: Vec<_> = profiles.iter().filter(|p| p.is_agent()).collect();

    div()
        .id("agent-lab-legacy-container")
        .flex_1()
        .h_full()
        .p(theme.spacing_xl)
        .bg(theme.bg_app)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(
            Button::new("legacy-trigger-sim-btn")
                .primary()
                .icon(Icon::new(LucideIcon::Zap))
                .label(t!("lab.synthesize_post").to_string())
                .on_click(move |_, window, cx| on_trigger_simulation(window, cx)),
        )
        .children(agent_profiles.into_iter().map(|p| {
            div()
                .p(theme.spacing_md)
                .rounded(theme.radius_md)
                .bg(theme.bg_card)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .text_size(theme.font_size_sm)
                .text_color(theme.text_primary)
                .child(format!("{} ({}) · {}", p.name, p.handle, p.config.mbti_full_code()))
        }))
}

/// Renders the complete Three-Tier Workspace:
/// - Left Pane: Contact Roster (`Network Agents` | `My Personas`)
/// - Right Pane: Compact Hero Header + (Vertical Category Rail | Horizontal Sub-Tabs Content Area)
pub fn render_agent_workspace_view(params: AgentWorkspaceParams<'_>) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;
    let mode = params.mode;
    let active_tab = params.active_config_tab;
    let sub_tabs = active_tab.sub_tabs();
    let active_sub_tab = params.active_sub_tab.min(sub_tabs.len().saturating_sub(1));

    let network_name = state
        .current_network()
        .map(|n| n.name.clone())
        .unwrap_or_else(|| t!("workspace.active").to_string());

    let grounding_doc_titles: Vec<String> = state
        .current_documents()
        .map(|d| d.title.clone())
        .collect();

    let agents_count = state.current_network_agents().len();
    let personas_count = state.current_network_personas().len();

    let contact_list: Vec<&Profile> = match mode {
        LabMode::NetworkAgents => state.current_network_agents(),
        LabMode::PersonaLab => state.current_network_personas(),
    };

    let selected_profile: Option<&Profile> = params
        .selected_profile_id
        .and_then(|id| contact_list.iter().find(|p| &p.id == id).copied())
        .or_else(|| contact_list.first().copied());

    let on_select_profile = Rc::new(params.on_select_profile);
    let on_select_tab = Rc::new(params.on_select_tab);
    let on_select_sub_tab = Rc::new(params.on_select_sub_tab);
    let on_switch_mode = Rc::new(params.on_switch_mode);
    let on_update_profile = Rc::new(params.on_update_profile);
    let on_save_text = Rc::new(params.on_save_text_inputs);
    let on_create = Rc::new(params.on_create_profile);
    let on_delete = Rc::new(params.on_delete_profile);
    let on_pick_avatar = Rc::new(params.on_pick_avatar);
    let on_activate_persona = Rc::new(params.on_activate_persona);
    let on_trigger_agent = Rc::new(params.on_trigger_agent);

    let selected_id_opt = selected_profile.map(|p| p.id.clone());

    div()
        .id("agent-workspace-two-pane-root")
        .w_full()
        .h_full()
        .bg(theme.bg_app)
        .flex()
        .flex_row()
        .overflow_hidden()
        // ====================================================================
        // TIER 1: LEFT CONTACT ROSTER PANE (Segmented Mode + Contact Cards)
        // ====================================================================
        .child({
            let cb_mode_agents = on_switch_mode.clone();
            let cb_mode_personas = on_switch_mode.clone();
            let cb_create = on_create.clone();

            div()
                .id("agent-workspace-left-contact-pane")
                .w(theme.contact_list_pane_width)
                .flex_shrink_0()
                .h_full()
                .bg(theme.bg_sidebar)
                .border_r(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .flex()
                .flex_col()
                // 1. Segmented Switcher (Network Agents vs My Personas)
                .child(
                    div()
                        .w_full()
                        .p(theme.spacing_sm)
                        .border_b(theme.border_width_hairline)
                        .border_color(theme.border_subtle)
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(
                            div()
                                .w_full()
                                .p(theme.spacing_2xs)
                                .rounded(theme.radius_md)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .flex()
                                .items_center()
                                .gap(theme.spacing_2xs)
                                .child({
                                    let is_agents = mode == LabMode::NetworkAgents;
                                    div()
                                        .id("mode-pill-network-agents")
                                        .flex_1()
                                        .py(theme.spacing_xs)
                                        .px(theme.spacing_xs)
                                        .rounded(theme.radius_sm)
                                        .cursor_pointer()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(theme.spacing_xs)
                                        .when(is_agents, |s| {
                                            s.bg(theme.accent_brand_subtle)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.accent_brand_border)
                                                .text_color(theme.text_primary)
                                        })
                                        .when(!is_agents, |s| s.text_color(theme.text_muted))
                                        .on_click(move |_, w, cx| cb_mode_agents(LabMode::NetworkAgents, w, cx))
                                        .child(
                                            Icon::new(LucideIcon::Users)
                                                .size(theme.font_size_xs)
                                                .text_color(if is_agents {
                                                    theme.accent_brand
                                                } else {
                                                    theme.text_muted
                                                }),
                                        )
                                        .child(
                                            div()
                                                .text_size(theme.font_size_2xs)
                                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                .truncate()
                                                .child(format!("{} ({})", t!("lab.mode_agents"), agents_count)),
                                        )
                                })
                                .child({
                                    let is_personas = mode == LabMode::PersonaLab;
                                    div()
                                        .id("mode-pill-persona-lab")
                                        .flex_1()
                                        .py(theme.spacing_xs)
                                        .px(theme.spacing_xs)
                                        .rounded(theme.radius_sm)
                                        .cursor_pointer()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .gap(theme.spacing_xs)
                                        .when(is_personas, |s| {
                                            s.bg(theme.accent_brand_subtle)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.accent_brand_border)
                                                .text_color(theme.text_primary)
                                        })
                                        .when(!is_personas, |s| s.text_color(theme.text_muted))
                                        .on_click(move |_, w, cx| cb_mode_personas(LabMode::PersonaLab, w, cx))
                                        .child(
                                            Icon::new(LucideIcon::UserCheck)
                                                .size(theme.font_size_xs)
                                                .text_color(if is_personas {
                                                    theme.accent_user
                                                } else {
                                                    theme.text_muted
                                                }),
                                        )
                                        .child(
                                            div()
                                                .text_size(theme.font_size_2xs)
                                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                .truncate()
                                                .child(format!("{} ({})", t!("lab.mode_personas"), personas_count)),
                                        )
                                }),
                        )
                        // Active Network Scope Subtitle
                        .child(
                            div()
                                .px(theme.spacing_xs)
                                .pt(theme.spacing_2xs)
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(theme.font_size_2xs)
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .text_color(theme.text_muted)
                                        .truncate()
                                        .child(network_name.to_uppercase()),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_mono.clone())
                                        .text_size(theme.font_size_2xs)
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.accent_brand)
                                        .child(format!("{}", contact_list.len())),
                                ),
                        ),
                )
                // 2. Scrollable Contact Roster
                .child(
                    div()
                        .id("agent-contact-list-scroll")
                        .flex_1()
                        .overflow_y_scrollbar()
                        .p(theme.spacing_xs)
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_2xs)
                        .children(contact_list.iter().enumerate().map(|(idx, prof)| {
                            let is_selected = selected_id_opt.as_ref() == Some(&prof.id);
                            let is_active_user_persona = state.active_profile_id == prof.id;
                            let prof_id = prof.id.clone();
                            let cb_sel = on_select_profile.clone();
                            let hover_bg = theme.hover_bg;

                            div()
                                .id(format!("contact-row-{}-{}", idx, prof.id))
                                .w_full()
                                .px(theme.spacing_sm)
                                .py(theme.spacing_sm)
                                .rounded(theme.radius_md)
                                .cursor_pointer()
                                .border(theme.border_width_hairline)
                                .when(is_selected, |s| {
                                    s.bg(theme.bg_elevated)
                                        .border_color(theme.accent_brand)
                                })
                                .when(!is_selected, |s| {
                                    s.bg(theme.bg_transparent)
                                        .border_color(theme.bg_transparent)
                                        .hover(move |h| h.bg(hover_bg))
                                })
                                .on_click(move |_, w, cx| {
                                    cb_sel(prof_id.clone(), w, cx);
                                })
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(theme.spacing_sm)
                                        .min_w_0()
                                        .flex_1()
                                        .child(render_profile_avatar(
                                            prof,
                                            theme.contact_avatar_size,
                                            theme,
                                        ))
                                        .child(
                                            div()
                                                .flex_1()
                                                .min_w_0()
                                                .flex()
                                                .flex_col()
                                                .gap(theme.spacing_3xs)
                                                .child(
                                                    div()
                                                        .text_size(theme.font_size_sm)
                                                        .font_weight(if is_selected {
                                                            gpui_kit::FontWeight::BOLD
                                                        } else {
                                                            gpui_kit::FontWeight::SEMIBOLD
                                                        })
                                                        .text_color(theme.text_primary)
                                                        .truncate()
                                                        .child(prof.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(theme.font_size_2xs)
                                                        .text_color(theme.text_muted)
                                                        .truncate()
                                                        .child(if !prof.config.description.is_empty() {
                                                            format!("{} · {}", prof.display_handle(), prof.config.description)
                                                        } else {
                                                            prof.display_handle()
                                                        }),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_shrink_0()
                                        .flex()
                                        .flex_col()
                                        .items_end()
                                        .gap(theme.spacing_2xs)
                                        .child(
                                            div()
                                                .px(theme.spacing_xs)
                                                .py(theme.spacing_3xs)
                                                .rounded(theme.radius_xs)
                                                .bg(theme.bg_card)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.border_subtle)
                                                .font_family(theme.font_family_mono.clone())
                                                .text_size(theme.font_size_2xs)
                                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                .text_color(if prof.is_agent() {
                                                    theme.accent_agent
                                                } else {
                                                    theme.accent_user
                                                })
                                                .child(prof.config.mbti.code()),
                                        )
                                        .when(prof.is_agent() && prof.config.is_active, |s| {
                                            s.child(
                                                div()
                                                    .w(theme.spacing_xs)
                                                    .h(theme.spacing_xs)
                                                    .rounded(theme.radius_full)
                                                    .bg(theme.accent_success),
                                            )
                                        })
                                        .when(!prof.is_agent() && is_active_user_persona, |s| {
                                            s.child(
                                                div()
                                                    .text_size(theme.font_size_2xs)
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_color(theme.accent_success)
                                                    .child("●"),
                                            )
                                        }),
                                )
                        })),
                )
                // 3. Pinned Bottom Button: Create New Network Agent or New Human Persona
                .child(
                    div()
                        .w_full()
                        .p(theme.spacing_sm)
                        .border_t(theme.border_width_hairline)
                        .border_color(theme.border_subtle)
                        .child(
                            Button::new("create-new-lab-profile-btn")
                                .primary()
                                .w_full()
                                .icon(Icon::new(LucideIcon::Plus))
                                .label(match mode {
                                    LabMode::NetworkAgents => t!("lab.new_agent").to_string(),
                                    LabMode::PersonaLab => t!("lab.new_persona").to_string(),
                                })
                                .on_click(move |_, w, cx| {
                                    cb_create(w, cx);
                                }),
                        ),
                )
        })
        // ====================================================================
        // TIER 2 & 3: RIGHT CONFIGURATION WORKSPACE
        // ====================================================================
        .child(if let Some(profile) = selected_profile {
            let prof_id_for_pick = profile.id.clone();
            let prof_id_for_del = profile.id.clone();
            let prof_id_for_act = profile.id.clone();
            let prof_id_for_trig = profile.id.clone();

            let cb_pick_hero = on_pick_avatar.clone();
            let cb_save_header = on_save_text.clone();
            let cb_del = on_delete.clone();
            let cb_act = on_activate_persona.clone();
            let cb_trig = on_trigger_agent.clone();

            let is_active_user_persona = state.active_profile_id == profile.id;
            let can_delete = contact_list.len() > 1;

            div()
                .id("agent-workspace-right-config-pane")
                .flex_1()
                .min_w_0()
                .h_full()
                .flex()
                .flex_col()
                .bg(theme.bg_app)
                // ------------------------------------------------------------
                // A. COMPACT HERO HEADER (Identity + Quick Actions)
                // ------------------------------------------------------------
                .child(
                    div()
                        .w_full()
                        .px(theme.spacing_lg)
                        .py(theme.spacing_sm)
                        .bg(theme.bg_elevated)
                        .border_b(theme.border_width_hairline)
                        .border_color(theme.border_subtle)
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(theme.spacing_md)
                        // Left: Clickable Avatar + Name + @handle + Concise Badges
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_md)
                                .min_w_0()
                                .flex_1()
                                .child(
                                    div()
                                        .id("hero-avatar-upload-trigger")
                                        .cursor_pointer()
                                        .on_click(move |_, w, cx| {
                                            cb_pick_hero(prof_id_for_pick.clone(), w, cx);
                                        })
                                        .child(render_profile_avatar(
                                            profile,
                                            theme.hero_avatar_size,
                                            theme,
                                        )),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .flex()
                                        .flex_col()
                                        .gap(theme.spacing_2xs)
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(theme.spacing_sm)
                                                .flex_wrap()
                                                .child(
                                                    div()
                                                        .font_family(theme.font_family_display.clone())
                                                        .text_size(theme.font_size_md)
                                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                                        .text_color(theme.text_primary)
                                                        .child(profile.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .font_family(theme.font_family_mono.clone())
                                                        .text_size(theme.font_size_xs)
                                                        .text_color(theme.text_muted)
                                                        .child(profile.display_handle()),
                                                )
                                                .when(!profile.config.occupation.is_empty(), |s| {
                                                    s.child(
                                                        div()
                                                            .text_size(theme.font_size_xs)
                                                            .text_color(theme.text_secondary)
                                                            .child(format!("· {}", profile.config.occupation)),
                                                    )
                                                }),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .flex_wrap()
                                                .gap(theme.spacing_xs)
                                                .child(render_meta_badge(
                                                    &format!(
                                                        "{} · {}",
                                                        profile.config.mbti_full_code(),
                                                        profile.config.mbti.archetype_name()
                                                    ),
                                                    theme.accent_brand,
                                                    theme.accent_brand_subtle,
                                                    theme.accent_brand_border,
                                                    theme,
                                                ))
                                                .child(render_meta_badge(
                                                    &format!(
                                                        "{} ({})",
                                                        profile.config.enneagram.code(),
                                                        profile.config.instinctual_variant.code()
                                                    ),
                                                    theme.accent_dialectic,
                                                    theme.accent_dialectic_subtle,
                                                    theme.accent_dialectic_border,
                                                    theme,
                                                ))
                                                .child(render_meta_badge(
                                                    &profile.config.primary_language,
                                                    theme.accent_user,
                                                    theme.hover_bg,
                                                    theme.border_subtle,
                                                    theme,
                                                )),
                                        ),
                                ),
                        )
                        // Right: Save + Synthesize / Activate + Delete
                        .child(
                            div()
                                .flex_shrink_0()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_xs)
                                .child(
                                    Button::new("save-agent-text-fields-btn")
                                        .ghost()
                                        .compact()
                                        .icon(Icon::new(LucideIcon::Check))
                                        .label(t!("lab.save_changes").to_string())
                                        .on_click(move |_, w, cx| {
                                            cb_save_header(w, cx);
                                        }),
                                )
                                .when(profile.is_agent(), |s| {
                                    s.child(
                                        Button::new("trigger-this-agent-post-btn")
                                            .primary()
                                            .compact()
                                            .icon(Icon::new(LucideIcon::Sparkles))
                                            .label(t!("lab.synthesize_post").to_string())
                                            .on_click(move |_, w, cx| {
                                                cb_trig(prof_id_for_trig.clone(), w, cx);
                                            }),
                                    )
                                })
                                .when(!profile.is_agent(), |s| {
                                    if is_active_user_persona {
                                        s.child(render_meta_badge(
                                            &t!("lab.active_persona_badge"),
                                            theme.accent_success,
                                            theme.accent_success_subtle,
                                            theme.accent_success_border,
                                            theme,
                                        ))
                                    } else {
                                        s.child(
                                            Button::new("activate-human-persona-btn")
                                                .primary()
                                                .compact()
                                                .icon(Icon::new(LucideIcon::UserCheck))
                                                .label(t!("lab.use_persona").to_string())
                                                .on_click(move |_, w, cx| {
                                                    cb_act(prof_id_for_act.clone(), w, cx);
                                                }),
                                        )
                                    }
                                })
                                .when(can_delete, |s| {
                                    s.child(
                                        Button::new("delete-lab-profile-btn")
                                            .ghost()
                                            .compact()
                                            .icon(Icon::new(LucideIcon::Trash))
                                            .tooltip(t!("lab.delete_tooltip").to_string())
                                            .on_click(move |_, w, cx| {
                                                cb_del(prof_id_for_del.clone(), w, cx);
                                            }),
                                    )
                                }),
                        ),
                )
                // ------------------------------------------------------------
                // B. MASTER-DETAIL BODY: VERTICAL CATEGORY RAIL + HORIZONTAL SUB-TABS
                // ------------------------------------------------------------
                .child(
                    div()
                        .w_full()
                        .flex_1()
                        .min_h_0()
                        .flex()
                        .flex_row()
                        .overflow_hidden()
                        // ====================================================
                        // B1. VERTICAL PRIMARY CATEGORY RAIL (Left of Config)
                        // ====================================================
                        .child({
                            let cb_preset_upd = on_update_profile.clone();
                            let prof_for_presets = profile.clone();

                            div()
                                .id("agent-vertical-category-rail")
                                .w(theme.agent_category_rail_width)
                                .flex_shrink_0()
                                .h_full()
                                .bg(theme.bg_sidebar)
                                .border_r(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .p(theme.spacing_sm)
                                .flex()
                                .flex_col()
                                .justify_between()
                                .overflow_y_scrollbar()
                                // Top: 6 Vertical Category Tabs with Live Subtitles
                                .child(
                                    div()
                                        .w_full()
                                        .flex()
                                        .flex_col()
                                        .gap(theme.spacing_xs)
                                        .children(AgentConfigTab::ALL.iter().map(|&tab| {
                                            let is_active = tab == active_tab;
                                            let cb_tab = on_select_tab.clone();
                                            let hover_bg = theme.hover_bg;
                                            let summary = tab.live_summary(profile);

                                            div()
                                                .id(format!("agent-vert-tab-{:?}", tab))
                                                .w_full()
                                                .px(theme.spacing_sm)
                                                .py(theme.spacing_sm)
                                                .rounded(theme.radius_md)
                                                .cursor_pointer()
                                                .border(theme.border_width_hairline)
                                                .when(is_active, |s| {
                                                    s.bg(theme.bg_elevated)
                                                        .border_color(theme.accent_brand_border)
                                                })
                                                .when(!is_active, |s| {
                                                    s.bg(theme.bg_transparent)
                                                        .border_color(theme.bg_transparent)
                                                        .hover(move |h| h.bg(hover_bg))
                                                })
                                                .on_click(move |_, w, cx| {
                                                    cb_tab(tab, w, cx);
                                                })
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .gap(theme.spacing_xs)
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(theme.spacing_sm)
                                                        .min_w_0()
                                                        .flex_1()
                                                        .child(
                                                            div()
                                                                .w(theme.avatar_size_sm)
                                                                .h(theme.avatar_size_sm)
                                                                .rounded(theme.radius_sm)
                                                                .bg(if is_active {
                                                                    theme.accent_brand_subtle
                                                                } else {
                                                                    theme.bg_card
                                                                })
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .child(
                                                                    Icon::new(tab.icon())
                                                                        .size(theme.font_size_xs)
                                                                        .text_color(if is_active {
                                                                            theme.accent_brand
                                                                        } else {
                                                                            theme.text_muted
                                                                        }),
                                                                ),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex_1()
                                                                .min_w_0()
                                                                .flex()
                                                                .flex_col()
                                                                .child(
                                                                    div()
                                                                        .text_size(theme.font_size_xs)
                                                                        .font_weight(if is_active {
                                                                            gpui_kit::FontWeight::BOLD
                                                                        } else {
                                                                            gpui_kit::FontWeight::MEDIUM
                                                                        })
                                                                        .text_color(if is_active {
                                                                            theme.text_primary
                                                                        } else {
                                                                            theme.text_secondary
                                                                        })
                                                                        .truncate()
                                                                        .child(tab.localized_label()),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .font_family(theme.font_family_mono.clone())
                                                                        .text_size(theme.font_size_2xs)
                                                                        .text_color(theme.text_muted)
                                                                        .truncate()
                                                                        .child(summary),
                                                                ),
                                                        ),
                                                )
                                                .when(is_active, |s| {
                                                    s.child(
                                                        div()
                                                            .w(theme.border_width_threadline)
                                                            .h(theme.spacing_lg)
                                                            .rounded(theme.radius_full)
                                                            .bg(theme.accent_brand),
                                                    )
                                                })
                                        })),
                                )
                                // Bottom of Vertical Rail: 1-Click Quick Archetype Presets
                                .child(
                                    div()
                                        .w_full()
                                        .pt(theme.spacing_sm)
                                        .border_t(theme.border_width_hairline)
                                        .border_color(theme.border_subtle)
                                        .flex()
                                        .flex_col()
                                        .gap(theme.spacing_xs)
                                        .child(
                                            div()
                                                .px(theme.spacing_2xs)
                                                .text_size(theme.font_size_2xs)
                                                .font_weight(gpui_kit::FontWeight::BOLD)
                                                .text_color(theme.text_muted)
                                                .child(t!("lab.presets_title").to_string()),
                                        )
                                        .children(
                                            [
                                                ("Socratic Inquirer", "ENTP-A · 5w4", 0usize),
                                                ("Analytic Rationalist", "INTJ-A · 5w6", 1usize),
                                                ("Luminous Humanist", "INFJ-A · 4w5", 2usize),
                                                ("Pragmatic Builder", "ENTJ-A · 8w7", 3usize),
                                            ]
                                            .into_iter()
                                            .map(|(preset_name, preset_code, preset_idx)| {
                                                let cb_p = cb_preset_upd.clone();
                                                let p_base = prof_for_presets.clone();
                                                let hover_bg = theme.hover_bg;

                                                div()
                                                    .id(format!("archetype-preset-btn-{}", preset_idx))
                                                    .w_full()
                                                    .px(theme.spacing_sm)
                                                    .py(theme.spacing_xs)
                                                    .rounded(theme.radius_sm)
                                                    .bg(theme.bg_card)
                                                    .border(theme.border_width_hairline)
                                                    .border_color(theme.border_subtle)
                                                    .cursor_pointer()
                                                    .hover(move |h| h.bg(hover_bg).border_color(theme.accent_brand_border))
                                                    .on_click(move |_, w, cx| {
                                                        let mut updated = p_base.clone();
                                                        apply_archetype_preset(&mut updated, preset_idx);
                                                        cb_p(updated, w, cx);
                                                    })
                                                    .flex()
                                                    .items_center()
                                                    .justify_between()
                                                    .child(
                                                        div()
                                                            .text_size(theme.font_size_2xs)
                                                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                            .text_color(theme.text_secondary)
                                                            .truncate()
                                                            .child(preset_name),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family(theme.font_family_mono.clone())
                                                            .text_size(theme.font_size_2xs)
                                                            .text_color(theme.accent_agent)
                                                            .child(preset_code),
                                                    )
                                            }),
                                        ),
                                )
                        })
                        // ====================================================
                        // B2. RIGHT CONTENT AREA (Horizontal Sub-Tabs + Body)
                        // ====================================================
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .h_full()
                                .flex()
                                .flex_col()
                                .bg(theme.bg_app)
                                // Top Horizontal Sub-Tab Bar
                                .child(
                                    div()
                                        .w_full()
                                        .px(theme.spacing_lg)
                                        .py(theme.spacing_sm)
                                        .bg(theme.bg_sidebar)
                                        .border_b(theme.border_width_hairline)
                                        .border_color(theme.border_subtle)
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .gap(theme.spacing_sm)
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .flex_wrap()
                                                .gap(theme.spacing_xs)
                                                .children(sub_tabs.into_iter().enumerate().map(
                                                    |(idx, (sub_label, sub_icon))| {
                                                        let is_sub_active = idx == active_sub_tab;
                                                        let cb_sub = on_select_sub_tab.clone();
                                                        let hover_bg = theme.hover_bg;

                                                        div()
                                                            .id(format!("agent-horiz-subtab-{:?}-{}", active_tab, idx))
                                                            .px(theme.spacing_md)
                                                            .py(theme.spacing_xs)
                                                            .rounded(theme.radius_md)
                                                            .cursor_pointer()
                                                            .border(theme.border_width_hairline)
                                                            .when(is_sub_active, |s| {
                                                                s.bg(theme.accent_brand_subtle)
                                                                    .border_color(theme.accent_brand)
                                                                    .text_color(theme.text_primary)
                                                            })
                                                            .when(!is_sub_active, |s| {
                                                                s.bg(theme.bg_card)
                                                                    .border_color(theme.border_subtle)
                                                                    .text_color(theme.text_secondary)
                                                                    .hover(move |h| h.bg(hover_bg))
                                                            })
                                                            .on_click(move |_, w, cx| {
                                                                cb_sub(idx, w, cx);
                                                            })
                                                            .flex()
                                                            .items_center()
                                                            .gap(theme.spacing_xs)
                                                            .child(
                                                                Icon::new(sub_icon)
                                                                    .size(theme.font_size_xs)
                                                                    .text_color(if is_sub_active {
                                                                        theme.accent_brand
                                                                    } else {
                                                                        theme.text_muted
                                                                    }),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(theme.font_size_xs)
                                                                    .font_weight(if is_sub_active {
                                                                        gpui_kit::FontWeight::BOLD
                                                                    } else {
                                                                        gpui_kit::FontWeight::MEDIUM
                                                                    })
                                                                    .child(sub_label),
                                                            )
                                                    },
                                                )),
                                        ),
                                )
                                // Scrollable Focused Sub-Tab Content Area
                                .child(
                                    div()
                                        .id("agent-config-subtab-scroll-body")
                                        .flex_1()
                                        .overflow_y_scrollbar()
                                        .p(theme.spacing_lg)
                                        .child(match active_tab {
                                            AgentConfigTab::IdentityAndBio => render_identity_bio_tab(
                                                profile,
                                                active_sub_tab,
                                                &params.inputs,
                                                theme,
                                                on_update_profile.clone(),
                                                on_save_text.clone(),
                                                on_pick_avatar.clone(),
                                            )
                                            .into_any_element(),
                                            AgentConfigTab::PsychologyAndMbti => render_psychology_mbti_tab(
                                                profile,
                                                active_sub_tab,
                                                theme,
                                                on_update_profile.clone(),
                                            )
                                            .into_any_element(),
                                            AgentConfigTab::IntelligenceAndMind => render_intelligence_mind_tab(
                                                profile,
                                                active_sub_tab,
                                                theme,
                                                on_update_profile.clone(),
                                            )
                                            .into_any_element(),
                                            AgentConfigTab::VoiceDictionGrammar => render_voice_grammar_tab(
                                                profile,
                                                active_sub_tab,
                                                &params.inputs,
                                                theme,
                                                on_update_profile.clone(),
                                                on_save_text.clone(),
                                            )
                                            .into_any_element(),
                                            AgentConfigTab::ScheduleAndMentions => render_schedule_mentions_tab(
                                                profile,
                                                active_sub_tab,
                                                &params.inputs,
                                                theme,
                                                on_update_profile.clone(),
                                                on_save_text.clone(),
                                            )
                                            .into_any_element(),
                                            AgentConfigTab::PromptAndAssembly => render_prompt_assembly_tab(
                                                profile,
                                                active_sub_tab,
                                                &network_name,
                                                &grounding_doc_titles,
                                                &params.inputs,
                                                theme,
                                                on_update_profile.clone(),
                                                on_save_text.clone(),
                                            )
                                            .into_any_element(),
                                        }),
                                ),
                        ),
                )
                .into_any_element()
        } else {
            div()
                .flex_1()
                .h_full()
                .flex()
                .items_center()
                .justify_center()
                .text_size(theme.font_size_sm)
                .text_color(theme.text_muted)
                .child("Select or create a profile in the left roster.")
                .into_any_element()
        })
}

/// Applies one of 4 cohesive human psychological & linguistic presets with 1 click.
fn apply_archetype_preset(profile: &mut Profile, preset_idx: usize) {
    let cfg = &mut profile.config;
    match preset_idx {
        0 => {
            // Socratic Inquirer
            cfg.mbti = MbtiType::Entp;
            cfg.identity_variant = IdentityVariant::Assertive;
            cfg.enneagram = EnneagramType::Type5w4;
            cfg.instinctual_variant = InstinctualVariant::SoSx;
            cfg.intelligence_style = CognitiveIntelligenceStyle::FirstPrinciplesDeductive;
            cfg.epistemic_stance = EpistemicStance::FallibilistSkeptic;
            cfg.humor_style = HumorStyle::SocraticIrony;
            cfg.diction_register = DictionRegister::SocraticIronic;
            cfg.mention_attitude = MentionAttitude::SocraticQuestioning;
        }
        1 => {
            // Analytic Rationalist
            cfg.mbti = MbtiType::Intj;
            cfg.identity_variant = IdentityVariant::Assertive;
            cfg.enneagram = EnneagramType::Type5w6;
            cfg.instinctual_variant = InstinctualVariant::SpSo;
            cfg.intelligence_style = CognitiveIntelligenceStyle::SystemsArchitect;
            cfg.epistemic_stance = EpistemicStance::RationalistDeductive;
            cfg.humor_style = HumorStyle::DryDeadpanWit;
            cfg.diction_register = DictionRegister::AcademicPhilosophical;
            cfg.mention_attitude = MentionAttitude::AnalyticalDebater;
        }
        2 => {
            // Luminous Humanist
            cfg.mbti = MbtiType::Infj;
            cfg.identity_variant = IdentityVariant::Assertive;
            cfg.enneagram = EnneagramType::Type4w5;
            cfg.instinctual_variant = InstinctualVariant::SxSo;
            cfg.intelligence_style = CognitiveIntelligenceStyle::PoeticIntuitive;
            cfg.epistemic_stance = EpistemicStance::PhenomenologicalExistential;
            cfg.humor_style = HumorStyle::WarmSelfDeprecating;
            cfg.diction_register = DictionRegister::LiteraryPoetic;
            cfg.mention_attitude = MentionAttitude::EmpatheticListener;
        }
        _ => {
            // Pragmatic Builder
            cfg.mbti = MbtiType::Entj;
            cfg.identity_variant = IdentityVariant::Assertive;
            cfg.enneagram = EnneagramType::Type8w7;
            cfg.instinctual_variant = InstinctualVariant::SoSp;
            cfg.intelligence_style = CognitiveIntelligenceStyle::PragmaticRealist;
            cfg.epistemic_stance = EpistemicStance::PragmaticEmpiricist;
            cfg.humor_style = HumorStyle::PlayfulBanter;
            cfg.diction_register = DictionRegister::EverydayNatural;
            cfg.mention_attitude = MentionAttitude::WarmAndReceptive;
        }
    }
}

// ============================================================================
// VERTICAL CATEGORY 1: IDENTITY & BIO (3 Focused Horizontal Sub-Tabs)
// ============================================================================
fn render_identity_bio_tab(
    profile: &Profile,
    sub_tab: usize,
    inputs: &AgentEditorInputs<'_>,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
    on_save_text: Rc<SaveTextInputsCallback>,
    on_pick_avatar: Rc<PickAvatarCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();
    let prof_id = profile.id.clone();

    match sub_tab {
        // SUB-TAB 0: Profile & Vocation (Avatar + Name + Handle + Role + Vocation)
        0 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "AVATAR & PORTRAIT",
                    "Upload a local image file (PNG, JPG, WebP) or use the monogram avatar.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_md)
                        .child(render_profile_avatar(profile, theme.hero_avatar_size, theme))
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_labeled_input("Avatar File Path", inputs.avatar_path_input, theme)),
                                )
                                .child({
                                    let cb_pick = on_pick_avatar.clone();
                                    let pid = prof_id.clone();
                                    div()
                                        .pt(theme.spacing_md)
                                        .child(
                                            Button::new("upload-avatar-file-btn")
                                                .primary()
                                                .compact()
                                                .icon(Icon::new(LucideIcon::Image))
                                                .label(t!("lab.upload_avatar").to_string())
                                                .on_click(move |_, w, cx| cb_pick(pid.clone(), w, cx)),
                                        )
                                })
                                .when(profile.avatar_path.is_some(), |s| {
                                    let cb_clear = on_update.clone();
                                    let p_clear = prof_clone.clone();
                                    s.child(
                                        div()
                                            .pt(theme.spacing_md)
                                            .child(
                                                Button::new("clear-avatar-file-btn")
                                                    .ghost()
                                                    .compact()
                                                    .icon(Icon::new(LucideIcon::X))
                                                    .on_click(move |_, w, cx| {
                                                        let mut p = p_clear.clone();
                                                        p.avatar_path = None;
                                                        cb_clear(p, w, cx);
                                                    }),
                                            ),
                                    )
                                }),
                        ),
                ),
            )
            .child(
                render_section_card(
                    "CORE IDENTITY & VOCATION",
                    "Human name, @username handle, short role headline, and daily profession.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Full Human Name", inputs.name_input, theme)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Username (@handle)", inputs.handle_input, theme)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Role / Short Headline", inputs.description_input, theme)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Occupation / Vocation", inputs.occupation_input, theme)),
                        ),
                )
                .child(render_save_footer("save-identity-sub0-btn", on_save_text, theme)),
            ),

        // SUB-TAB 1: Demographics & Mood (Location, Pronouns, Gender, Age, Current Mood)
        1 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "DEMOGRAPHICS & CULTURAL ROOTS",
                    "Geographic background, personal pronouns, gender identity, and chronological age.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Cultural & Geographic Roots", inputs.location_input, theme)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input("Pronouns", inputs.pronouns_input, theme)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Gender Identity", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(HumanGender::ALL.iter().map(|&gender| {
                                            let is_sel = profile.config.gender == gender;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("gender-chip-{:?}", gender),
                                                gender.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.gender = gender;
                                                    p.config.pronouns = gender.default_pronouns().to_string();
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .flex()
                                .flex_col()
                                .items_end()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Chronological Age", theme))
                                .child(render_u8_stepper(
                                    "age-stepper",
                                    profile.config.age.clamp(16, 110) as u8,
                                    16,
                                    110,
                                    " yrs",
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |new_age, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.age = u16::from(new_age);
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        ),
                ),
            )
            .child(
                render_section_card(
                    "CURRENT AFFECTIVE MOOD",
                    "Immediate emotional tone coloring this person's thoughts today.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(
                            [
                                "Contemplative & Lucid",
                                "Playfully Ironic & Inquisitive",
                                "Focused & Pragmatic",
                                "Serene & Sub Specie Aeternitatis",
                                "Luminous Attention",
                                "Electric & Associative",
                                "Lyrical & Earthy",
                            ]
                            .into_iter()
                            .map(|mood_preset| {
                                let is_sel = profile.config.current_mood == mood_preset;
                                let cb = on_update.clone();
                                let p_next = prof_clone.clone();

                                render_choice_chip(
                                    format!("mood-preset-{}", mood_preset),
                                    mood_preset,
                                    is_sel,
                                    theme,
                                    move |w, cx| {
                                        let mut p = p_next.clone();
                                        p.config.current_mood = mood_preset.to_string();
                                        cb(p, w, cx);
                                    },
                                )
                            }),
                        ),
                )
                .child(render_save_footer("save-identity-sub1-btn", on_save_text, theme)),
            ),

        // SUB-TAB 2: Biography & Lifestyle (Bio, Hobbies, Core Values, Daily Routine)
        _ => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "BIOGRAPHY, HOBBIES & VALUES",
                    "Lived history, personal passions, moral convictions, and daily rhythm.",
                    theme,
                )
                .child(render_labeled_textarea(
                    "Personal Biography (Lived History)",
                    inputs.bio_input,
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Hobbies, Interests & Passions",
                                    inputs.hobbies_input,
                                    theme,
                                )),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Daily Routine & Habits",
                                    inputs.routine_input,
                                    theme,
                                )),
                        ),
                )
                .child(render_labeled_input(
                    "Core Moral & Intellectual Values",
                    inputs.values_input,
                    theme,
                ))
                .child(render_save_footer("save-identity-sub2-btn", on_save_text, theme)),
            ),
    }
}

// ============================================================================
// VERTICAL CATEGORY 2: PSYCHOLOGY & MBTI (4 Focused Horizontal Sub-Tabs)
// ============================================================================
fn render_psychology_mbti_tab(
    profile: &Profile,
    sub_tab: usize,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();

    match sub_tab {
        // SUB-TAB 0: Jungian & MBTI (Grouped 4x4 Matrix + Identity Variant + Cognitive Stack)
        0 => {
            let groups: [(&str, [MbtiType; 4]); 4] = [
                ("ANALYSTS (NT)", [MbtiType::Intj, MbtiType::Intp, MbtiType::Entj, MbtiType::Entp]),
                ("DIPLOMATS (NF)", [MbtiType::Infj, MbtiType::Infp, MbtiType::Enfj, MbtiType::Enfp]),
                ("SENTINELS (SJ)", [MbtiType::Istj, MbtiType::Isfj, MbtiType::Estj, MbtiType::Esfj]),
                ("EXPLORERS (SP)", [MbtiType::Istp, MbtiType::Isfp, MbtiType::Estp, MbtiType::Esfp]),
            ];

            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                .child(
                    render_section_card(
                        "16-TYPE JUNGIAN & MBTI MATRIX",
                        "Select an archetype grouped by temperament. Automatically updates the 4-function cognitive stack.",
                        theme,
                    )
                    .children(groups.into_iter().map(|(group_title, types)| {
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_2xs)
                            .child(
                                div()
                                    .text_size(theme.font_size_2xs)
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_color(theme.text_muted)
                                    .child(group_title),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .gap(theme.spacing_xs)
                                    .children(types.into_iter().map(|mbti| {
                                        let is_sel = profile.config.mbti == mbti;
                                        let cb = on_update.clone();
                                        let p_next = prof_clone.clone();
                                        let hover_bg = theme.hover_bg;

                                        div()
                                            .id(format!("mbti-card-{}", mbti.code()))
                                            .flex_1()
                                            .p(theme.spacing_sm)
                                            .rounded(theme.radius_md)
                                            .cursor_pointer()
                                            .border(theme.border_width_hairline)
                                            .when(is_sel, |s| {
                                                s.bg(theme.accent_brand_subtle)
                                                    .border_color(theme.accent_brand)
                                            })
                                            .when(!is_sel, |s| {
                                                s.bg(theme.bg_app)
                                                    .border_color(theme.border_subtle)
                                                    .hover(move |h| h.bg(hover_bg))
                                            })
                                            .on_click(move |_, w, cx| {
                                                let mut p = p_next.clone();
                                                p.config.mbti = mbti;
                                                cb(p, w, cx);
                                            })
                                            .flex()
                                            .flex_col()
                                            .gap(theme.spacing_3xs)
                                            .child(
                                                div()
                                                    .font_family(theme.font_family_mono.clone())
                                                    .text_size(theme.font_size_sm)
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_color(if is_sel {
                                                        theme.accent_brand
                                                    } else {
                                                        theme.text_primary
                                                    })
                                                    .child(mbti.code()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(theme.font_size_2xs)
                                                    .text_color(theme.text_muted)
                                                    .truncate()
                                                    .child(mbti.archetype_name()),
                                            )
                                    })),
                            )
                    }))
                    // Identity Variant + Active Cognitive Stack Pipeline
                    .child(
                        div()
                            .w_full()
                            .pt(theme.spacing_xs)
                            .border_t(theme.border_width_hairline)
                            .border_color(theme.border_subtle)
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(theme.spacing_md)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(theme.spacing_xs)
                                    .children(IdentityVariant::ALL.iter().map(|&var| {
                                        let is_sel = profile.config.identity_variant == var;
                                        let cb = on_update.clone();
                                        let p_next = prof_clone.clone();

                                        render_choice_chip(
                                            format!("id-var-{}", var.suffix()),
                                            var.label(),
                                            is_sel,
                                            theme,
                                            move |w, cx| {
                                                let mut p = p_next.clone();
                                                p.config.identity_variant = var;
                                                cb(p, w, cx);
                                            },
                                        )
                                    })),
                            )
                            .child(
                                div()
                                    .px(theme.spacing_md)
                                    .py(theme.spacing_xs)
                                    .rounded(theme.radius_md)
                                    .bg(theme.bg_callout)
                                    .border(theme.border_width_hairline)
                                    .border_color(theme.accent_brand_border)
                                    .flex()
                                    .items_center()
                                    .gap(theme.spacing_sm)
                                    .child(
                                        Icon::new(LucideIcon::Brain)
                                            .size(theme.font_size_xs)
                                            .text_color(theme.accent_brand),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_xs)
                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(profile.config.mbti.cognitive_functions()),
                                    ),
                            ),
                    ),
                )
        }

        // SUB-TAB 1: Big Five (OCEAN) & HEXACO (Clean 2x3 Grid of Compact Trait Cards)
        1 => {
            let traits_row1 = [
                (
                    "Openness",
                    "Curiosity, aesthetics & imagination",
                    profile.config.openness,
                    theme.accent_brand,
                    0usize,
                ),
                (
                    "Conscientiousness",
                    "Discipline, precision & orderliness",
                    profile.config.conscientiousness,
                    theme.accent_user,
                    1usize,
                ),
            ];
            let traits_row2 = [
                (
                    "Extraversion",
                    "Social energy & assertiveness",
                    profile.config.extraversion,
                    theme.accent_dialectic,
                    2usize,
                ),
                (
                    "Agreeableness",
                    "Warmth, empathy & cooperative harmony",
                    profile.config.agreeableness,
                    theme.accent_success,
                    3usize,
                ),
            ];
            let traits_row3 = [
                (
                    "Neuroticism",
                    "Affective sensitivity & vigilance",
                    profile.config.neuroticism,
                    theme.accent_danger,
                    4usize,
                ),
                (
                    "Honesty-Humility (HEXACO)",
                    "Sincerity, modesty & epistemic candor",
                    profile.config.honesty_humility,
                    theme.accent_agent,
                    5usize,
                ),
            ];

            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                .child(
                    render_section_card(
                        "BIG FIVE (OCEAN) & HEXACO DIMENSIONS",
                        "Empirical six-factor personality profile arranged in a balanced 2-column matrix.",
                        theme,
                    )
                    .children([traits_row1, traits_row2, traits_row3].into_iter().map(|row| {
                        div()
                            .w_full()
                            .flex()
                            .gap(theme.spacing_md)
                            .children(row.into_iter().map(|(title, subtitle, val, color, idx)| {
                                let cb = on_update.clone();
                                let p_base = prof_clone.clone();
                                div()
                                    .flex_1()
                                    .child(render_compact_trait_card(
                                        &format!("ocean-card-{}", idx),
                                        title,
                                        subtitle,
                                        val,
                                        color,
                                        theme,
                                        move |next_val, w, cx| {
                                            let mut p = p_base.clone();
                                            match idx {
                                                0 => p.config.openness = next_val,
                                                1 => p.config.conscientiousness = next_val,
                                                2 => p.config.extraversion = next_val,
                                                3 => p.config.agreeableness = next_val,
                                                4 => p.config.neuroticism = next_val,
                                                _ => p.config.honesty_humility = next_val,
                                            }
                                            cb(p, w, cx);
                                        },
                                    ))
                            }))
                    })),
                )
        }

        // SUB-TAB 2: Enneagram, Instinctual Stack & Tritype
        2 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "ENNEAGRAM OF PERSONALITY & WING",
                    "Core motivational dynamics across the 9 Enneagram types and wings.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(EnneagramType::ALL.iter().map(|&ennea| {
                            let is_sel = profile.config.enneagram == ennea;
                            let cb = on_update.clone();
                            let p_next = prof_clone.clone();

                            render_choice_chip(
                                format!("ennea-chip-{}", ennea.code()),
                                ennea.title(),
                                is_sel,
                                theme,
                                move |w, cx| {
                                    let mut p = p_next.clone();
                                    p.config.enneagram = ennea;
                                    cb(p, w, cx);
                                },
                            )
                        })),
                ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .gap(theme.spacing_md)
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "INSTINCTUAL STACKING",
                                    "Dominant / secondary biological drive (sp/so/sx).",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(InstinctualVariant::ALL.iter().map(|&inst| {
                                            let is_sel = profile.config.instinctual_variant == inst;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("instinct-chip-{}", inst.code()),
                                                inst.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.instinctual_variant = inst;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "TRITYPE ARCHETYPE",
                                    "3-center triad (Head · Heart · Gut).",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(
                                            [
                                                ("514", "514 · Philosopher"),
                                                ("583", "583 · Strategist"),
                                                ("549", "549 · Contemplative"),
                                                ("459", "459 · Mystic"),
                                                ("145", "145 · Researcher"),
                                                ("748", "748 · Messenger"),
                                                ("358", "358 · SolutionMinder"),
                                                ("853", "853 · Trailblazer"),
                                                ("954", "954 · Gentle Sage"),
                                            ]
                                            .into_iter()
                                            .map(|(tri_code, tri_label)| {
                                                let is_sel = profile.config.tritype == tri_code;
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();

                                                render_choice_chip(
                                                    format!("tritype-preset-{}", tri_code),
                                                    tri_label,
                                                    is_sel,
                                                    theme,
                                                    move |w, cx| {
                                                        let mut p = p_next.clone();
                                                        p.config.tritype = tri_code.to_string();
                                                        cb(p, w, cx);
                                                    },
                                                )
                                            }),
                                        ),
                                ),
                            ),
                    ),
            ),

        // SUB-TAB 3: Attachment, Moral Foundations, Temperament & Locus of Control (2x2 Grid)
        _ => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                div()
                    .w_full()
                    .flex()
                    .gap(theme.spacing_md)
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "ATTACHMENT STYLE (BOWLBY / AINSWORTH)",
                                    "Relational trust and interpersonal bonding pattern.",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(AttachmentStyle::ALL.iter().map(|&att| {
                                            let is_sel = profile.config.attachment_style == att;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("attach-chip-{:?}", att),
                                                att.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.attachment_style = att;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "MORAL FOUNDATION (JONATHAN HAIDT)",
                                    "Primary ethical lens when evaluating dilemmas.",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(MoralFoundation::ALL.iter().map(|&mf| {
                                            let is_sel = profile.config.moral_foundation == mf;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("moral-chip-{:?}", mf),
                                                mf.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.moral_foundation = mf;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                            ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .gap(theme.spacing_md)
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "CLASSICAL TEMPERAMENT",
                                    "Foundational affective temperament.",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(ClassicTemperament::ALL.iter().map(|&temp| {
                                            let is_sel = profile.config.temperament == temp;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("temp-chip-{:?}", temp),
                                                temp.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.temperament = temp;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(
                                render_section_card(
                                    "LOCUS OF CONTROL (ROTTER)",
                                    "Agency attribution in human affairs.",
                                    theme,
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(LocusOfControl::ALL.iter().map(|&loc| {
                                            let is_sel = profile.config.locus_of_control == loc;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("locus-chip-{:?}", loc),
                                                loc.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.locus_of_control = loc;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                            ),
                    ),
            ),
    }
}

// ============================================================================
// VERTICAL CATEGORY 3: INTELLIGENCE & MIND (3 Focused Horizontal Sub-Tabs)
// ============================================================================
fn render_intelligence_mind_tab(
    profile: &Profile,
    sub_tab: usize,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();

    match sub_tab {
        // SUB-TAB 0: Cognitive Architecture & Epistemic Stance
        0 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "PRIMARY COGNITIVE & REASONING ARCHITECTURE",
                    "How this person processes information, constructs arguments, and synthesizes insights.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(CognitiveIntelligenceStyle::ALL.iter().map(|&cog| {
                            let is_sel = profile.config.intelligence_style == cog;
                            let cb = on_update.clone();
                            let p_next = prof_clone.clone();

                            render_choice_chip(
                                format!("cog-style-{:?}", cog),
                                cog.label(),
                                is_sel,
                                theme,
                                move |w, cx| {
                                    let mut p = p_next.clone();
                                    p.config.intelligence_style = cog;
                                    cb(p, w, cx);
                                },
                            )
                        })),
                ),
            )
            .child(
                render_section_card(
                    "EPISTEMIC STANCE (THEORY OF KNOWLEDGE)",
                    "Criteria for truth, justification, and belief revision.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(EpistemicStance::ALL.iter().map(|&ep| {
                            let is_sel = profile.config.epistemic_stance == ep;
                            let cb = on_update.clone();
                            let p_next = prof_clone.clone();

                            render_choice_chip(
                                format!("epistemic-stance-{:?}", ep),
                                ep.label(),
                                is_sel,
                                theme,
                                move |w, cx| {
                                    let mut p = p_next.clone();
                                    p.config.epistemic_stance = ep;
                                    cb(p, w, cx);
                                },
                            )
                        })),
                ),
            ),

        // SUB-TAB 1: Multiple Intelligences (Cattell-Horn-Carroll Gf/Gc, Gardner, Goleman EQ)
        1 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "MULTIPLE INTELLIGENCE QUOTIENTS (CHC / GARDNER / GOLEMAN)",
                    "Configure fluid reasoning (Gf), crystallized knowledge (Gc), emotional intelligence (EQ), social intuition (SQ), and existential depth.",
                    theme,
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_compact_trait_card(
                                    "iq-fluid",
                                    "Fluid Intelligence (Gf)",
                                    "Novel pattern recognition & abstract reasoning",
                                    profile.config.fluid_intelligence,
                                    theme.accent_brand,
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |val, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.fluid_intelligence = val;
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_compact_trait_card(
                                    "iq-crystallized",
                                    "Crystallized Intelligence (Gc)",
                                    "Historical, cultural & domain mastery",
                                    profile.config.crystallized_intelligence,
                                    theme.accent_user,
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |val, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.crystallized_intelligence = val;
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        ),
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_compact_trait_card(
                                    "iq-emotional",
                                    "Emotional Intelligence (EQ)",
                                    "Affective attunement & empathy",
                                    profile.config.emotional_intelligence,
                                    theme.accent_success,
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |val, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.emotional_intelligence = val;
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_compact_trait_card(
                                    "iq-social",
                                    "Social Intelligence (SQ)",
                                    "Pragmatics, diplomacy & group dynamics",
                                    profile.config.social_intelligence,
                                    theme.accent_dialectic,
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |val, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.social_intelligence = val;
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        ),
                )
                .child(render_compact_trait_card(
                    "iq-existential",
                    "Existential & Philosophical Intelligence",
                    "Capacity for ultimate questions of meaning, ethics, and being",
                    profile.config.existential_intelligence,
                    theme.accent_agent,
                    theme,
                    {
                        let cb = on_update.clone();
                        let p_base = prof_clone.clone();
                        move |val, w, cx| {
                            let mut p = p_base.clone();
                            p.config.existential_intelligence = val;
                            cb(p, w, cx);
                        }
                    },
                )),
            ),

        // SUB-TAB 2: Cognitive Tempo & Humor
        _ => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "COGNITIVE TEMPO (SYSTEM 1 / SYSTEM 2)",
                    "Deliberation speed and associative rhythm.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(CognitivePace::ALL.iter().map(|&pace| {
                            let is_sel = profile.config.cognitive_pace == pace;
                            let cb = on_update.clone();
                            let p_next = prof_clone.clone();

                            render_choice_chip(
                                format!("cog-pace-{:?}", pace),
                                pace.label(),
                                is_sel,
                                theme,
                                move |w, cx| {
                                    let mut p = p_next.clone();
                                    p.config.cognitive_pace = pace;
                                    cb(p, w, cx);
                                },
                            )
                        })),
                ),
            )
            .child(
                render_section_card(
                    "SENSE OF HUMOR & WIT",
                    "How this person uses irony, levity, or deadpan wit in conversation.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(theme.spacing_xs)
                        .children(HumorStyle::ALL.iter().map(|&hum| {
                            let is_sel = profile.config.humor_style == hum;
                            let cb = on_update.clone();
                            let p_next = prof_clone.clone();

                            render_choice_chip(
                                format!("humor-style-{:?}", hum),
                                hum.label(),
                                is_sel,
                                theme,
                                move |w, cx| {
                                    let mut p = p_next.clone();
                                    p.config.humor_style = hum;
                                    cb(p, w, cx);
                                },
                            )
                        })),
                ),
            ),
    }
}

// ============================================================================
// VERTICAL CATEGORY 4: VOICE & STYLE (3 Focused Horizontal Sub-Tabs)
// ============================================================================
fn render_voice_grammar_tab(
    profile: &Profile,
    sub_tab: usize,
    inputs: &AgentEditorInputs<'_>,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
    on_save_text: Rc<SaveTextInputsCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();

    match sub_tab {
        // SUB-TAB 0: Languages & Bilingualism
        0 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "LANGUAGES, FLUENCY & CODE-SWITCHING",
                    "Primary writing language, secondary languages, and bilingual code-switching behavior.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Primary Writing Language",
                                    inputs.primary_lang_input,
                                    theme,
                                )),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Secondary Languages & Classical Roots",
                                    inputs.secondary_lang_input,
                                    theme,
                                )),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Quick Primary Language Presets", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(
                                    [
                                        "Bahasa Indonesia",
                                        "English",
                                        "Bilingual (Indonesian & English)",
                                        "Francais & English",
                                        "Deutsch & English",
                                        "Japanese & English",
                                    ]
                                    .into_iter()
                                    .map(|lang_preset| {
                                        let is_sel = profile.config.primary_language == lang_preset;
                                        let cb = on_update.clone();
                                        let p_next = prof_clone.clone();

                                        render_choice_chip(
                                            format!("lang-preset-{}", lang_preset),
                                            lang_preset,
                                            is_sel,
                                            theme,
                                            move |w, cx| {
                                                let mut p = p_next.clone();
                                                p.config.primary_language = lang_preset.to_string();
                                                cb(p, w, cx);
                                            },
                                        )
                                    }),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Code-Switching & Bilingualism Style", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(CodeSwitchingStyle::ALL.iter().map(|&cs| {
                                    let is_sel = profile.config.code_switching == cs;
                                    let cb = on_update.clone();
                                    let p_next = prof_clone.clone();

                                    render_choice_chip(
                                        format!("code-switch-{:?}", cs),
                                        cs.label(),
                                        is_sel,
                                        theme,
                                        move |w, cx| {
                                            let mut p = p_next.clone();
                                            p.config.code_switching = cs;
                                            cb(p, w, cx);
                                        },
                                    )
                                })),
                        ),
                )
                .child(render_save_footer("save-voice-sub0-btn", on_save_text, theme)),
            ),

        // SUB-TAB 1: Grammar, Diction & Sentence Rhythm
        1 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "GRAMMAR, DICTION REGISTER & LEXICAL COMPLEXITY",
                    "Syntactic structure, vocabulary register, and sentence pacing.",
                    theme,
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Grammar & Syntax Style", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(GrammarStyle::ALL.iter().map(|&gr| {
                                            let is_sel = profile.config.grammar_style == gr;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("grammar-style-{:?}", gr),
                                                gr.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.grammar_style = gr;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Diction & Lexical Register", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(DictionRegister::ALL.iter().map(|&dic| {
                                            let is_sel = profile.config.diction_register == dic;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("diction-reg-{:?}", dic),
                                                dic.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.diction_register = dic;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        ),
                )
                .child(render_compact_trait_card(
                    "vocab-complexity-gauge",
                    "Vocabulary Complexity",
                    "0% = Everyday Plain Speech · 100% = Erudite Polymathic Lexicon",
                    profile.config.vocabulary_complexity,
                    theme.accent_brand,
                    theme,
                    {
                        let cb = on_update.clone();
                        let p_base = prof_clone.clone();
                        move |val, w, cx| {
                            let mut p = p_base.clone();
                            p.config.vocabulary_complexity = val;
                            cb(p, w, cx);
                        }
                    },
                ))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Sentence Length & Rhythm", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(SentenceLengthPreference::ALL.iter().map(|&sl| {
                                    let is_sel = profile.config.sentence_length == sl;
                                    let cb = on_update.clone();
                                    let p_next = prof_clone.clone();

                                    render_choice_chip(
                                        format!("sentence-len-{:?}", sl),
                                        sl.label(),
                                        is_sel,
                                        theme,
                                        move |w, cx| {
                                            let mut p = p_next.clone();
                                            p.config.sentence_length = sl;
                                            cb(p, w, cx);
                                        },
                                    )
                                })),
                        ),
                ),
            ),

        // SUB-TAB 2: Idiolect, Typing Mechanics & Signature Phrases
        _ => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "TYPING MECHANICS, PUNCTUATION & IDIOLECT",
                    "Capitalization, punctuation habits, emoji usage, signature expressions, and avoided clichés.",
                    theme,
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Capitalization Habit", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(CapitalizationStyle::ALL.iter().map(|&cap| {
                                            let is_sel = profile.config.capitalization == cap;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("cap-style-{:?}", cap),
                                                cap.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.capitalization = cap;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Emoji & Emoticon Habit", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(EmojiUsage::ALL.iter().map(|&em| {
                                            let is_sel = profile.config.emoji_usage == em;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("emoji-usage-{:?}", em),
                                                em.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.emoji_usage = em;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Punctuation Habit", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(PunctuationHabit::ALL.iter().map(|&punc| {
                                    let is_sel = profile.config.punctuation_habit == punc;
                                    let cb = on_update.clone();
                                    let p_next = prof_clone.clone();

                                    render_choice_chip(
                                        format!("punc-habit-{:?}", punc),
                                        punc.label(),
                                        is_sel,
                                        theme,
                                        move |w, cx| {
                                            let mut p = p_next.clone();
                                            p.config.punctuation_habit = punc;
                                            cb(p, w, cx);
                                        },
                                    )
                                })),
                        ),
                )
                .child(render_labeled_input(
                    "Specific Typing Quirks (e.g. em-dashes, numbered premises, parentheses)",
                    inputs.typing_quirks_input,
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Signature Phrases & Idioms",
                                    inputs.signature_phrases_input,
                                    theme,
                                )),
                        )
                        .child(
                            div()
                                .flex_1()
                                .child(render_labeled_input(
                                    "Avoided Words & Taboo Clichés",
                                    inputs.avoided_words_input,
                                    theme,
                                )),
                        ),
                )
                .child(render_save_footer("save-voice-sub2-btn", on_save_text, theme)),
            ),
    }
}

// ============================================================================
// VERTICAL CATEGORY 5: SCHEDULE & SOCIAL (2 Focused Horizontal Sub-Tabs)
// ============================================================================
fn render_schedule_mentions_tab(
    profile: &Profile,
    sub_tab: usize,
    inputs: &AgentEditorInputs<'_>,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
    on_save_text: Rc<SaveTextInputsCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();

    match sub_tab {
        // SUB-TAB 0: Posting Schedule, Chronotype & Topics
        0 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "POSTING SCHEDULE, CHRONOTYPE & CIRCADIAN RHYTHM",
                    "Configure when and how often this network member posts thoughts.",
                    theme,
                )
                .child(render_toggle_row(
                    "agent-active-loop-toggle",
                    "Active in Network Autonomous Loop",
                    "When enabled, this member participates in scheduled and reactive network threads.",
                    profile.config.is_active,
                    theme,
                    {
                        let cb = on_update.clone();
                        let p_next = prof_clone.clone();
                        move |w, cx| {
                            let mut p = p_next.clone();
                            p.config.is_active = !p.config.is_active;
                            cb(p, w, cx);
                        }
                    },
                ))
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Posting Frequency & Cadence", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(PostingCadence::ALL.iter().map(|&cad| {
                                            let is_sel = profile.config.posting_cadence == cad;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("post-cadence-{:?}", cad),
                                                cad.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.posting_cadence = cad;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Circadian Chronotype", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(HumanChronotype::ALL.iter().map(|&ch| {
                                            let is_sel = profile.config.chronotype == ch;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("chrono-{:?}", ch),
                                                ch.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.chronotype = ch;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_sm)
                                .child(render_field_label("Active Hours Window (0–23h):", theme))
                                .child(render_u8_stepper(
                                    "start-hour-step",
                                    profile.config.active_hours_start,
                                    0,
                                    23,
                                    ":00",
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |h, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.active_hours_start = h;
                                            cb(p, w, cx);
                                        }
                                    },
                                ))
                                .child(
                                    div()
                                        .text_size(theme.font_size_xs)
                                        .text_color(theme.text_muted)
                                        .child("to"),
                                )
                                .child(render_u8_stepper(
                                    "end-hour-step",
                                    profile.config.active_hours_end,
                                    0,
                                    23,
                                    ":00",
                                    theme,
                                    {
                                        let cb = on_update.clone();
                                        let p_base = prof_clone.clone();
                                        move |h, w, cx| {
                                            let mut p = p_base.clone();
                                            p.config.active_hours_end = h;
                                            cb(p, w, cx);
                                        }
                                    },
                                )),
                        ),
                )
                .child(render_compact_trait_card(
                    "spontaneity-gauge",
                    "Spontaneity & Serendipity Drive",
                    "Likelihood of initiating unprompted threads on a fresh spark of inspiration",
                    profile.config.spontaneity_rate,
                    theme.accent_dialectic,
                    theme,
                    {
                        let cb = on_update.clone();
                        let p_base = prof_clone.clone();
                        move |val, w, cx| {
                            let mut p = p_base.clone();
                            p.config.spontaneity_rate = val;
                            cb(p, w, cx);
                        }
                    },
                ))
                .child(render_labeled_input(
                    "Preferred Hashtags & Discourse Topics",
                    inputs.preferred_topics_input,
                    theme,
                ))
                .child(render_save_footer("save-schedule-sub0-btn", on_save_text, theme)),
            ),

        // SUB-TAB 1: @Mention Attitude & Conflict Handling
        _ => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "ATTITUDE WHEN @MENTIONED & INTELLECTUAL DISAGREEMENT",
                    "How this person reacts when tagged (@handle) in a post or challenged in debate.",
                    theme,
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Attitude When @Mentioned", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(MentionAttitude::ALL.iter().map(|&att| {
                                    let is_sel = profile.config.mention_attitude == att;
                                    let cb = on_update.clone();
                                    let p_next = prof_clone.clone();

                                    render_choice_chip(
                                        format!("mention-att-{:?}", att),
                                        att.label(),
                                        is_sel,
                                        theme,
                                        move |w, cx| {
                                            let mut p = p_next.clone();
                                            p.config.mention_attitude = att;
                                            cb(p, w, cx);
                                        },
                                    )
                                })),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label("Intellectual Disagreement & Conflict Style", theme))
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap(theme.spacing_xs)
                                .children(ConflictHandlingStyle::ALL.iter().map(|&cf| {
                                    let is_sel = profile.config.conflict_handling == cf;
                                    let cb = on_update.clone();
                                    let p_next = prof_clone.clone();

                                    render_choice_chip(
                                        format!("conflict-style-{:?}", cf),
                                        cf.label(),
                                        is_sel,
                                        theme,
                                        move |w, cx| {
                                            let mut p = p_next.clone();
                                            p.config.conflict_handling = cf;
                                            cb(p, w, cx);
                                        },
                                    )
                                })),
                        ),
                )
                .child(render_labeled_textarea(
                    "Custom @Mention Behavioral Instruction",
                    inputs.mention_instruction_input,
                    theme,
                ))
                .child(render_save_footer("save-schedule-sub1-btn", on_save_text, theme)),
            ),
    }
}

// ============================================================================
// VERTICAL CATEGORY 6: PROMPT ENGINE (3 Focused Horizontal Sub-Tabs)
// ============================================================================
#[allow(clippy::too_many_arguments)]
fn render_prompt_assembly_tab(
    profile: &Profile,
    sub_tab: usize,
    network_name: &str,
    grounding_doc_titles: &[String],
    inputs: &AgentEditorInputs<'_>,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
    on_save_text: Rc<SaveTextInputsCallback>,
) -> impl IntoElement {
    let prof_clone = profile.clone();

    match sub_tab {
        // SUB-TAB 0: Strategy, Pre-Post Reasoning & Dossier Injection Switches
        0 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "PROMPT ASSEMBLY STRATEGY & PRE-POST REASONING",
                    "Configure how all human traits are woven into the prompt before generating a post.",
                    theme,
                )
                .child(
                    div()
                        .w_full()
                        .flex()
                        .gap(theme.spacing_md)
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Prompt Assembly Strategy", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(PromptAssemblyStrategy::ALL.iter().map(|&strat| {
                                            let is_sel = profile.config.assembly_strategy == strat;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("prompt-strat-{:?}", strat),
                                                strat.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.assembly_strategy = strat;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(render_field_label("Pre-Post Internal Deliberation", theme))
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap(theme.spacing_xs)
                                        .children(PrePostReasoningMode::ALL.iter().map(|&mode| {
                                            let is_sel = profile.config.pre_post_reasoning == mode;
                                            let cb = on_update.clone();
                                            let p_next = prof_clone.clone();

                                            render_choice_chip(
                                                format!("reason-mode-{:?}", mode),
                                                mode.label(),
                                                is_sel,
                                                theme,
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.pre_post_reasoning = mode;
                                                    cb(p, w, cx);
                                                },
                                            )
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_field_label(
                            "Injected Human Dossier Sections (Included in Prompt)",
                            theme,
                        ))
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-identity-toggle",
                                            "1. Identity & Biography",
                                            "Name, age, gender, vocation, bio, hobbies & values",
                                            profile.config.inject_identity_and_bio,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_identity_and_bio =
                                                        !p.config.inject_identity_and_bio;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-psych-toggle",
                                            "2. Psychology & MBTI",
                                            "MBTI, cognitive stack, Big Five, Enneagram & attachment",
                                            profile.config.inject_psychological_profile,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_psychological_profile =
                                                        !p.config.inject_psychological_profile;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-intel-toggle",
                                            "3. Intelligence & Mind",
                                            "Cognitive style, epistemic stance, Gf/Gc/EQ/SQ & wit",
                                            profile.config.inject_intelligence_profile,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_intelligence_profile =
                                                        !p.config.inject_intelligence_profile;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-voice-toggle",
                                            "4. Voice & Grammar",
                                            "Languages, grammar, diction, punctuation & quirks",
                                            profile.config.inject_linguistic_stylistics,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_linguistic_stylistics =
                                                        !p.config.inject_linguistic_stylistics;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .w_full()
                                .flex()
                                .gap(theme.spacing_sm)
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-social-toggle",
                                            "5. @Mention & Schedule",
                                            "Chronotype, active hours, @mention attitude & conflict",
                                            profile.config.inject_social_and_schedule,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_social_and_schedule =
                                                        !p.config.inject_social_and_schedule;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .child(render_toggle_row(
                                            "inj-vault-toggle",
                                            "6. Knowledge Vault RAG",
                                            "Ground posts in the active network's Knowledge Vault",
                                            profile.config.inject_network_grounding,
                                            theme,
                                            {
                                                let cb = on_update.clone();
                                                let p_next = prof_clone.clone();
                                                move |w, cx| {
                                                    let mut p = p_next.clone();
                                                    p.config.inject_network_grounding =
                                                        !p.config.inject_network_grounding;
                                                    cb(p, w, cx);
                                                }
                                            },
                                        )),
                                ),
                        ),
                ),
            ),

        // SUB-TAB 1: Core Directive & Custom {{placeholder}} Template
        1 => div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(
                render_section_card(
                    "CORE DIRECTIVE & CUSTOM PROMPT TEMPLATE",
                    "Write the core personal directive and optional {{placeholder}} template.",
                    theme,
                )
                .child(render_labeled_textarea(
                    "Core Personal Directive & Intellectual Mission",
                    inputs.system_prompt_input,
                    theme,
                ))
                .child(render_labeled_textarea(
                    "Custom Prompt Assembly Template",
                    inputs.prompt_template_input,
                    theme,
                ))
                .child(
                    div()
                        .p(theme.spacing_sm)
                        .rounded(theme.radius_md)
                        .bg(theme.bg_callout)
                        .border(theme.border_width_hairline)
                        .border_color(theme.accent_brand_border)
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(theme.spacing_xs)
                        .child(
                            div()
                                .text_size(theme.font_size_2xs)
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(theme.accent_brand)
                                .child("Available Placeholders:"),
                        )
                        .children(
                            [
                                "{{name}}",
                                "{{handle}}",
                                "{{network_context}}",
                                "{{all_agent_info}}",
                                "{{identity}}",
                                "{{psychology}}",
                                "{{intelligence}}",
                                "{{linguistics}}",
                                "{{schedule}}",
                                "{{directive}}",
                            ]
                            .into_iter()
                            .map(|tok| {
                                div()
                                    .px(theme.spacing_xs)
                                    .py(theme.spacing_3xs)
                                    .rounded(theme.radius_xs)
                                    .bg(theme.bg_card)
                                    .border(theme.border_width_hairline)
                                    .border_color(theme.border_subtle)
                                    .font_family(theme.font_family_mono.clone())
                                    .text_size(theme.font_size_2xs)
                                    .text_color(theme.text_primary)
                                    .child(tok)
                            }),
                        ),
                )
                .child(render_save_footer("save-prompt-sub1-btn", on_save_text, theme)),
            ),

        // SUB-TAB 2: Live Compiled Prompt Inspector
        _ => {
            let compiled_system_prompt = profile
                .config
                .assemble_system_prompt(profile, network_name, grounding_doc_titles);
            let sample_user_turn = profile.config.assemble_user_prompt(
                profile,
                "Arya Wiratama",
                "@architect",
                "post_01",
                "How does our linguistic framing shape what we consider self-evident in this network?",
                true,
            );

            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                .child(
                    render_section_card(
                        "LIVE COMPILED PROMPT INSPECTOR",
                        "Real-time preview of the exact System Prompt and @Mention User Turn assembled from all human parameters.",
                        theme,
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_xs)
                            .child(render_field_label("1. Assembled System Prompt (Sent to LLM)", theme))
                            .child(
                                div()
                                    .id("compiled-system-prompt-preview-scroll")
                                    .w_full()
                                    .max_h(theme.prompt_preview_height)
                                    .overflow_y_scrollbar()
                                    .p(theme.spacing_md)
                                    .rounded(theme.radius_md)
                                    .bg(theme.bg_app)
                                    .border(theme.border_width_hairline)
                                    .border_color(theme.border_strong)
                                    .font_family(theme.font_family_mono.clone())
                                    .text_size(theme.font_size_xs)
                                    .text_color(theme.text_secondary)
                                    .child(compiled_system_prompt),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_xs)
                            .child(render_field_label("2. Assembled @Mention User Turn Preview", theme))
                            .child(
                                div()
                                    .id("compiled-user-turn-preview-scroll")
                                    .w_full()
                                    .max_h(theme.prompt_preview_height)
                                    .overflow_y_scrollbar()
                                    .p(theme.spacing_md)
                                    .rounded(theme.radius_md)
                                    .bg(theme.bg_app)
                                    .border(theme.border_width_hairline)
                                    .border_color(theme.border_subtle)
                                    .font_family(theme.font_family_mono.clone())
                                    .text_size(theme.font_size_xs)
                                    .text_color(theme.text_secondary)
                                    .child(sample_user_turn),
                            ),
                    ),
                )
        }
    }
}

// ============================================================================
// ATOMIC & MOLECULAR FORM HELPERS (100% TOKENIZED VIA THEMETOKENS)
// ============================================================================

fn render_section_card(title: &str, subtitle: &str, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .p(theme.spacing_md)
        .rounded(theme.radius_lg)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .pb(theme.spacing_xs)
                .border_b(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .font_family(theme.font_family_display.clone())
                        .text_size(theme.font_size_xs)
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_size(theme.font_size_2xs)
                        .text_color(theme.text_muted)
                        .child(subtitle.to_string()),
                ),
        )
}

fn render_save_footer(
    id: &'static str,
    on_save_text: Rc<SaveTextInputsCallback>,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .flex()
        .justify_end()
        .pt(theme.spacing_xs)
        .child(
            Button::new(id)
                .primary()
                .compact()
                .icon(Icon::new(LucideIcon::Check))
                .label(t!("lab.save_changes").to_string())
                .on_click(move |_, w, cx| on_save_text(w, cx)),
        )
}

fn render_field_label(label: &str, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_xs)
        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
        .text_color(theme.text_secondary)
        .child(label.to_string())
}

fn render_labeled_input(
    label: &str,
    input: &Entity<InputState>,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(render_field_label(label, theme))
        .child(
            div()
                .w_full()
                .rounded(theme.radius_md)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_app)
                .px(theme.spacing_xs)
                .py(theme.spacing_3xs)
                .child(Input::new(input).bordered(false).cleanable(true)),
        )
}

fn render_labeled_textarea(
    label: &str,
    textarea: &Entity<TextareaState>,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(render_field_label(label, theme))
        .child(
            div()
                .w_full()
                .rounded(theme.radius_md)
                .border(theme.border_width_hairline)
                .border_color(theme.border)
                .bg(theme.bg_app)
                .px(theme.spacing_xs)
                .py(theme.spacing_2xs)
                .child(Textarea::new(textarea).bordered(false)),
        )
}

fn render_choice_chip(
    id: impl Into<ElementId>,
    label: &str,
    is_selected: bool,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let hover_bg = theme.hover_bg;

    div()
        .id(id)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border(theme.border_width_hairline)
        .when(is_selected, |s| {
            s.bg(theme.accent_brand_subtle)
                .border_color(theme.accent_brand)
                .text_color(theme.text_primary)
        })
        .when(!is_selected, |s| {
            s.bg(theme.bg_app)
                .border_color(theme.border_subtle)
                .text_color(theme.text_secondary)
                .hover(move |h| h.bg(hover_bg))
        })
        .on_click(move |_, w, cx| {
            on_click(w, cx);
        })
        .text_size(theme.font_size_xs)
        .font_weight(if is_selected {
            gpui_kit::FontWeight::SEMIBOLD
        } else {
            gpui_kit::FontWeight::NORMAL
        })
        .child(label.to_string())
}

fn render_meta_badge(
    text: &str,
    text_color: Rgba,
    bg_color: Rgba,
    border_color: Rgba,
    theme: &ThemeTokens,
) -> impl IntoElement {
    div()
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .bg(bg_color)
        .border(theme.border_width_hairline)
        .border_color(border_color)
        .font_family(theme.font_family_ui.clone())
        .text_size(theme.font_size_2xs)
        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
        .text_color(text_color)
        .child(text.to_string())
}

/// Returns a human-friendly semantic interpretation label for a 0..=100 trait score.
const fn trait_semantic_label(value: u8) -> &'static str {
    match value {
        90..=100 => "Exceptional",
        75..=89 => "High",
        45..=74 => "Balanced",
        25..=44 => "Moderate-Low",
        _ => "Reserved",
    }
}

/// Renders a compact, scannable trait card suitable for 2-column grids, featuring a
/// semantic level badge (`High · 85%`), a 10-segment visual meter, and `-5 / +5` steppers.
fn render_compact_trait_card(
    id_prefix: &str,
    title: &str,
    subtitle: &str,
    value: u8,
    accent_color: Rgba,
    theme: &ThemeTokens,
    on_change: impl Fn(u8, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let clamped = value.min(100);
    let cb = Rc::new(on_change);
    let cb_minus5 = cb.clone();
    let cb_plus5 = cb.clone();
    let semantic = trait_semantic_label(clamped);

    div()
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        // Row 1: Title + Semantic Score Badge + Steppers
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(theme.spacing_xs)
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                .text_color(theme.text_primary)
                                .truncate()
                                .child(title.to_string()),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_2xs)
                                .text_color(theme.text_muted)
                                .truncate()
                                .child(subtitle.to_string()),
                        ),
                )
                .child(
                    div()
                        .flex_shrink_0()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_2xs)
                        .child(render_mini_step_btn(
                            format!("{}-m5", id_prefix),
                            "-5",
                            theme,
                            move |w, cx| cb_minus5(clamped.saturating_sub(5), w, cx),
                        ))
                        .child(
                            div()
                                .px(theme.spacing_xs)
                                .py(theme.spacing_3xs)
                                .rounded(theme.radius_xs)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .font_family(theme.font_family_mono.clone())
                                .text_size(theme.font_size_2xs)
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(accent_color)
                                .child(format!("{} · {}%", semantic, clamped)),
                        )
                        .child(render_mini_step_btn(
                            format!("{}-p5", id_prefix),
                            "+5",
                            theme,
                            move |w, cx| cb_plus5((clamped + 5).min(100), w, cx),
                        )),
                ),
        )
        // Row 2: 10-Segment Visual Meter (Click any segment to jump directly to 10%..100%)
        .child(
            div()
                .w_full()
                .h(theme.trait_bar_height)
                .rounded(theme.radius_full)
                .bg(theme.bg_card)
                .overflow_hidden()
                .flex()
                .gap(theme.spacing_3xs)
                .children((1..=10u8).map(|seg_idx| {
                    let threshold = seg_idx * 10;
                    let filled = clamped >= threshold.saturating_sub(5);
                    let cb_seg = cb.clone();
                    div()
                        .id(format!("{}-seg-{}", id_prefix, seg_idx))
                        .flex_1()
                        .h_full()
                        .cursor_pointer()
                        .bg(if filled {
                            accent_color
                        } else {
                            theme.border_subtle
                        })
                        .on_click(move |_, w, cx| {
                            cb_seg(threshold, w, cx);
                        })
                })),
        )
}

fn render_u8_stepper(
    id_prefix: &str,
    value: u8,
    min_val: u8,
    max_val: u8,
    unit: &str,
    theme: &ThemeTokens,
    on_change: impl Fn(u8, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let cb = Rc::new(on_change);
    let cb_m5 = cb.clone();
    let cb_m1 = cb.clone();
    let cb_p1 = cb.clone();
    let cb_p5 = cb.clone();

    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .child(render_mini_step_btn(
            format!("{}-m5", id_prefix),
            "-5",
            theme,
            move |w, cx| cb_m5(value.saturating_sub(5).max(min_val), w, cx),
        ))
        .child(render_mini_step_btn(
            format!("{}-m1", id_prefix),
            "-1",
            theme,
            move |w, cx| cb_m1(value.saturating_sub(1).max(min_val), w, cx),
        ))
        .child(
            div()
                .px(theme.spacing_sm)
                .py(theme.spacing_3xs)
                .rounded(theme.radius_sm)
                .bg(theme.bg_app)
                .border(theme.border_width_hairline)
                .border_color(theme.border_strong)
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .font_weight(gpui_kit::FontWeight::BOLD)
                .text_color(theme.accent_brand)
                .child(format!("{}{}", value, unit)),
        )
        .child(render_mini_step_btn(
            format!("{}-p1", id_prefix),
            "+1",
            theme,
            move |w, cx| cb_p1((value + 1).min(max_val), w, cx),
        ))
        .child(render_mini_step_btn(
            format!("{}-p5", id_prefix),
            "+5",
            theme,
            move |w, cx| cb_p5((value + 5).min(max_val), w, cx),
        ))
}

fn render_mini_step_btn(
    id: impl Into<ElementId>,
    label: &str,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, w, cx| on_click(w, cx))
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_secondary)
        .child(label.to_string())
}

fn render_toggle_row(
    id: impl Into<ElementId>,
    title: &str,
    subtitle: &str,
    is_enabled: bool,
    theme: &ThemeTokens,
    on_toggle: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let hover_bg = theme.hover_bg;

    div()
        .id(id)
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(if is_enabled {
            theme.accent_brand_border
        } else {
            theme.border_subtle
        })
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, w, cx| on_toggle(w, cx))
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .child(
                    div()
                        .text_size(theme.font_size_xs)
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_size(theme.font_size_2xs)
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(subtitle.to_string()),
                ),
        )
        .child(
            div()
                .flex_shrink_0()
                .px(theme.spacing_xs)
                .py(theme.spacing_3xs)
                .rounded(theme.radius_full)
                .bg(if is_enabled {
                    theme.accent_success_subtle
                } else {
                    theme.hover_bg
                })
                .border(theme.border_width_hairline)
                .border_color(if is_enabled {
                    theme.accent_success_border
                } else {
                    theme.border_subtle
                })
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .font_weight(gpui_kit::FontWeight::BOLD)
                .text_color(if is_enabled {
                    theme.accent_success
                } else {
                    theme.text_muted
                })
                .child(if is_enabled { "ON" } else { "OFF" }),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[core::prelude::v1::test]
    fn test_vertical_tabs_have_sub_tabs_and_live_summaries() {
        let profile = Profile::new_agent("agent_test", "Sokrates of Alopece", "@sokrates", "Classical Dialectician");
        for tab in AgentConfigTab::ALL {
            let sub_tabs = tab.sub_tabs();
            assert!(!sub_tabs.is_empty(), "{:?} must define horizontal sub-tabs", tab);
            let summary = tab.live_summary(&profile);
            assert!(!summary.trim().is_empty(), "{:?} must return a non-empty live summary", tab);
            assert!(!tab.localized_label().trim().is_empty());
        }
    }

    #[core::prelude::v1::test]
    fn test_archetype_presets_configure_cohesive_psychology() {
        let mut profile = Profile::new_agent("agent_preset", "Test Thinker", "@thinker", "Philosopher");

        apply_archetype_preset(&mut profile, 0);
        assert_eq!(profile.config.mbti, MbtiType::Entp);
        assert_eq!(profile.config.enneagram, EnneagramType::Type5w4);

        apply_archetype_preset(&mut profile, 1);
        assert_eq!(profile.config.mbti, MbtiType::Intj);
        assert_eq!(profile.config.enneagram, EnneagramType::Type5w6);

        apply_archetype_preset(&mut profile, 2);
        assert_eq!(profile.config.mbti, MbtiType::Infj);
        assert_eq!(profile.config.enneagram, EnneagramType::Type4w5);

        apply_archetype_preset(&mut profile, 3);
        assert_eq!(profile.config.mbti, MbtiType::Entj);
        assert_eq!(profile.config.enneagram, EnneagramType::Type8w7);
    }

    #[core::prelude::v1::test]
    fn test_trait_semantic_labels() {
        assert_eq!(trait_semantic_label(95), "Exceptional");
        assert_eq!(trait_semantic_label(80), "High");
        assert_eq!(trait_semantic_label(60), "Balanced");
        assert_eq!(trait_semantic_label(35), "Moderate-Low");
        assert_eq!(trait_semantic_label(15), "Reserved");
    }
}
