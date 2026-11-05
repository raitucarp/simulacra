//! Root 3-column full-width responsive application view.

use crate::domain::{PostId, Profile, ProfileId};
use crate::i18n::Language;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::molecules::{LanguagePopover, PostComposerView, WorkspacePopover};
use crate::ui::organisms::{
    render_agent_workspace_view, render_left_sidebar, render_main_feed,
    render_unified_right_sidebar, AgentConfigTab, AgentEditorInputs, AgentWorkspaceParams,
    AppTitleBar, ComposerModalMode, ComposerModalView, FeedParams, LabMode, LeftSidebarParams,
    RightSidebarTab, SettingsModal, SettingsTab, UnifiedRightSidebarParams,
};
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Main window root view container.
pub struct AppView {
    pub state: AppState,
    pub theme: ThemeTokens,
    pub current_language: Language,
    pub is_lang_menu_open: bool,
    pub is_settings_open: bool,
    pub active_settings_tab: SettingsTab,
    pub selected_thread_post_id: Option<PostId>,
    pub is_workspace_open: bool,
    pub is_left_sidebar_collapsed: bool,
    pub is_right_sidebar_open: bool,
    pub right_sidebar_tab: RightSidebarTab,
    pub custom_feed_profile_id: Option<ProfileId>,
    pub selected_calendar_day: Option<usize>,
    pub is_composer_modal_open: bool,
    pub composer_modal_mode: ComposerModalMode,
    pub modal_composer_input: Entity<TextareaState>,
    pub modal_link_input: Entity<InputState>,
    pub modal_image_input: Entity<InputState>,
    pub modal_image_path: Option<String>,
    pub is_modal_link_input_open: bool,
    pub is_modal_image_input_open: bool,
    pub feed_visible_count: usize,
    pub composer_input: Entity<TextareaState>,
    pub composer_view: Entity<PostComposerView>,
    pub search_input: Entity<InputState>,
    pub doc_title_input: Entity<InputState>,
    pub doc_content_input: Entity<TextareaState>,
    pub llm_api_key_input: Entity<InputState>,
    pub llm_model_input: Entity<InputState>,
    pub llm_endpoint_input: Entity<InputState>,

    // Agent & Persona Lab Two-Pane State & Text Inputs
    pub selected_lab_profile_id: Option<ProfileId>,
    pub active_agent_config_tab: AgentConfigTab,
    pub active_agent_sub_tab: usize,
    pub agent_name_input: Entity<InputState>,
    pub agent_handle_input: Entity<InputState>,
    pub agent_description_input: Entity<InputState>,
    pub agent_occupation_input: Entity<InputState>,
    pub agent_location_input: Entity<InputState>,
    pub agent_pronouns_input: Entity<InputState>,
    pub agent_avatar_path_input: Entity<InputState>,
    pub agent_bio_input: Entity<TextareaState>,
    pub agent_hobbies_input: Entity<InputState>,
    pub agent_values_input: Entity<InputState>,
    pub agent_routine_input: Entity<InputState>,
    pub agent_primary_lang_input: Entity<InputState>,
    pub agent_secondary_lang_input: Entity<InputState>,
    pub agent_typing_quirks_input: Entity<InputState>,
    pub agent_signature_phrases_input: Entity<InputState>,
    pub agent_avoided_words_input: Entity<InputState>,
    pub agent_preferred_topics_input: Entity<InputState>,
    pub agent_mention_instruction_input: Entity<TextareaState>,
    pub agent_system_prompt_input: Entity<TextareaState>,
    pub agent_prompt_template_input: Entity<TextareaState>,

    _subscriptions: Vec<Subscription>,
}

impl AppView {
    /// Constructs a new AppView with default state, frameless window setup, and dark minimalist theme.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let initial_lang = Language::English;
        crate::i18n::set_locale(initial_lang);

        let composer_placeholder = t!("composer.placeholder").to_string();
        let search_placeholder = t!("sidebar.search_placeholder").to_string();
        let doc_title_placeholder = t!("vault.doc_title_placeholder").to_string();
        let doc_content_placeholder = t!("vault.doc_content_placeholder").to_string();

        let composer_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(composer_placeholder)
                .auto_grow(1, 5)
        });

        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder(search_placeholder));

        let doc_title_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(doc_title_placeholder));

        let doc_content_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(doc_content_placeholder)
                .auto_grow(3, 8)
        });

        let modal_composer_placeholder = t!("composer.placeholder").to_string();
        let modal_composer_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(modal_composer_placeholder)
                .auto_grow(2, 8)
        });

        let modal_link_input = cx.new(|cx| InputState::new(window, cx).placeholder("https://..."));

        let modal_image_placeholder = t!("composer.image_path_placeholder").to_string();
        let modal_image_input =
            cx.new(|cx| InputState::new(window, cx).placeholder(modal_image_placeholder));

        let sub_search = cx.subscribe(&search_input, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.feed_visible_count = crate::ui::organisms::DEFAULT_FEED_WINDOW_SIZE;
                cx.notify();
            }
        });

        let sub_modal_composer = cx.subscribe_in(
            &modal_composer_input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Change => {
                    cx.notify();
                }
                InputEvent::PressEnter { secondary: true, .. } => {
                    this.submit_composer_modal(window, cx);
                }
                _ => {}
            },
        );

        let sub_modal_link = cx.subscribe(&modal_link_input, |_, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        });

        let sub_modal_image = cx.subscribe(&modal_image_input, |_, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        });

        let state = AppState::new();

        let entity_self = cx.entity().clone();
        let composer_view = cx.new(|cx| {
            PostComposerView::new(
                state.current_profile().cloned(),
                composer_input.clone(),
                state.theme_tokens.clone(),
                move |content, window, cx| {
                    entity_self.update(cx, |this, cx| {
                        this.submit_post_content(&content, window, cx);
                    });
                },
                window,
                cx,
            )
        });

        let initial_api_key = state.llm_settings.api_key.clone();
        let initial_model = state.llm_settings.model.clone();
        let initial_endpoint = state.llm_settings.custom_endpoint.clone();

        let llm_api_key_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_api_key)
                .placeholder("Enter Gemini API Key (AIzaSy...)")
        });

        let llm_model_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_model)
                .placeholder("Model Identifier (e.g. gemini-1.5-flash)")
        });

        let llm_endpoint_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_endpoint)
                .placeholder("http://localhost:8000/v1/chat/completions")
        });

        // Initialize Agent/Persona Editor Inputs from the first network agent (or first profile)
        let initial_lab_profile = state
            .current_network_agents()
            .first()
            .copied()
            .or_else(|| state.user_profiles.first())
            .cloned()
            .unwrap_or_else(|| Profile::new_agent("agent_default", "Sokrates", "@sokrates", "Philosopher"));

        let selected_lab_profile_id = Some(initial_lab_profile.id.clone());

        let agent_name_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.name)
                .placeholder("Full Human Name...")
        });
        let agent_handle_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.handle)
                .placeholder("@username")
        });
        let agent_description_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.description)
                .placeholder("Short role / headline...")
        });
        let agent_occupation_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.occupation)
                .placeholder("Occupation / Vocation...")
        });
        let agent_location_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.location_culture)
                .placeholder("City, Country / Cultural background...")
        });
        let agent_pronouns_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.pronouns)
                .placeholder("he/him, she/her, they/them, ia/dia...")
        });
        let agent_avatar_path_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(initial_lab_profile.avatar_path.as_deref().unwrap_or(""))
                .placeholder("Local image file path for avatar...")
        });
        let agent_bio_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value(&initial_lab_profile.config.bio)
                .placeholder("Write a rich human biography, life background, and intellectual journey...")
                .auto_grow(3, 6)
        });
        let agent_hobbies_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.hobbies)
                .placeholder("Hobbies, passions, crafts, leisure activities...")
        });
        let agent_values_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.core_values)
                .placeholder("Core moral & intellectual values...")
        });
        let agent_routine_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.daily_routine)
                .placeholder("Daily routine & habits...")
        });
        let agent_primary_lang_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.primary_language)
                .placeholder("Primary language (e.g. Indonesian, English)...")
        });
        let agent_secondary_lang_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.secondary_languages)
                .placeholder("Secondary languages...")
        });
        let agent_typing_quirks_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.typing_quirks)
                .placeholder("Specific typing quirks (e.g. em-dashes, lowercase)...")
        });
        let agent_signature_phrases_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.signature_phrases)
                .placeholder("Signature expressions & idioms...")
        });
        let agent_avoided_words_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.avoided_words)
                .placeholder("Words or clichés this person never uses...")
        });
        let agent_preferred_topics_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(&initial_lab_profile.config.preferred_topics)
                .placeholder("#Topics and themes...")
        });
        let agent_mention_instruction_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value(&initial_lab_profile.config.custom_mention_instruction)
                .placeholder("How should this person respond when @mentioned in a post?")
                .auto_grow(2, 5)
        });
        let agent_system_prompt_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value(&initial_lab_profile.config.system_prompt)
                .placeholder("Core directive for this person...")
                .auto_grow(3, 6)
        });
        let agent_prompt_template_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .default_value(&initial_lab_profile.config.prompt_template)
                .placeholder("Custom prompt template with {{name}}, {{all_agent_info}}, {{directive}}...")
                .auto_grow(3, 6)
        });

        Self {
            theme: state.theme_tokens.clone(),
            state,
            current_language: initial_lang,
            is_lang_menu_open: false,
            is_settings_open: false,
            active_settings_tab: SettingsTab::AiIntelligence,
            selected_thread_post_id: None,
            is_workspace_open: false,
            is_left_sidebar_collapsed: false,
            is_right_sidebar_open: true,
            right_sidebar_tab: RightSidebarTab::MindCadence,
            custom_feed_profile_id: None,
            selected_calendar_day: None,
            is_composer_modal_open: false,
            composer_modal_mode: ComposerModalMode::NewThread,
            modal_composer_input,
            modal_link_input,
            modal_image_input,
            modal_image_path: None,
            is_modal_link_input_open: false,
            is_modal_image_input_open: false,
            feed_visible_count: crate::ui::organisms::DEFAULT_FEED_WINDOW_SIZE,
            composer_input,
            composer_view,
            search_input,
            doc_title_input,
            doc_content_input,
            llm_api_key_input,
            llm_model_input,
            llm_endpoint_input,
            selected_lab_profile_id,
            active_agent_config_tab: AgentConfigTab::IdentityAndBio,
            active_agent_sub_tab: 0,
            agent_name_input,
            agent_handle_input,
            agent_description_input,
            agent_occupation_input,
            agent_location_input,
            agent_pronouns_input,
            agent_avatar_path_input,
            agent_bio_input,
            agent_hobbies_input,
            agent_values_input,
            agent_routine_input,
            agent_primary_lang_input,
            agent_secondary_lang_input,
            agent_typing_quirks_input,
            agent_signature_phrases_input,
            agent_avoided_words_input,
            agent_preferred_topics_input,
            agent_mention_instruction_input,
            agent_system_prompt_input,
            agent_prompt_template_input,
            _subscriptions: vec![sub_search, sub_modal_composer, sub_modal_link, sub_modal_image],
        }
    }

    /// Synchronizes all 20 text inputs in the Agent/Persona configuration pane with the given `Profile`.
    pub fn sync_agent_editor_inputs(
        &mut self,
        profile: &Profile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = profile.name.clone();
        let handle = profile.handle.clone();
        let desc = profile.config.description.clone();
        let occ = profile.config.occupation.clone();
        let loc = profile.config.location_culture.clone();
        let pron = profile.config.pronouns.clone();
        let av_path = profile.avatar_path.clone().unwrap_or_default();
        let bio = profile.config.bio.clone();
        let hobbies = profile.config.hobbies.clone();
        let values = profile.config.core_values.clone();
        let routine = profile.config.daily_routine.clone();
        let plang = profile.config.primary_language.clone();
        let slang = profile.config.secondary_languages.clone();
        let quirks = profile.config.typing_quirks.clone();
        let phrases = profile.config.signature_phrases.clone();
        let avoided = profile.config.avoided_words.clone();
        let topics = profile.config.preferred_topics.clone();
        let mention_inst = profile.config.custom_mention_instruction.clone();
        let sys_prompt = profile.config.system_prompt.clone();
        let tmpl = profile.config.prompt_template.clone();

        self.agent_name_input.update(cx, |s, cx| s.set_value(&name, window, cx));
        self.agent_handle_input.update(cx, |s, cx| s.set_value(&handle, window, cx));
        self.agent_description_input.update(cx, |s, cx| s.set_value(&desc, window, cx));
        self.agent_occupation_input.update(cx, |s, cx| s.set_value(&occ, window, cx));
        self.agent_location_input.update(cx, |s, cx| s.set_value(&loc, window, cx));
        self.agent_pronouns_input.update(cx, |s, cx| s.set_value(&pron, window, cx));
        self.agent_avatar_path_input.update(cx, |s, cx| s.set_value(&av_path, window, cx));
        self.agent_bio_input.update(cx, |s, cx| s.set_value(&bio, window, cx));
        self.agent_hobbies_input.update(cx, |s, cx| s.set_value(&hobbies, window, cx));
        self.agent_values_input.update(cx, |s, cx| s.set_value(&values, window, cx));
        self.agent_routine_input.update(cx, |s, cx| s.set_value(&routine, window, cx));
        self.agent_primary_lang_input.update(cx, |s, cx| s.set_value(&plang, window, cx));
        self.agent_secondary_lang_input.update(cx, |s, cx| s.set_value(&slang, window, cx));
        self.agent_typing_quirks_input.update(cx, |s, cx| s.set_value(&quirks, window, cx));
        self.agent_signature_phrases_input.update(cx, |s, cx| s.set_value(&phrases, window, cx));
        self.agent_avoided_words_input.update(cx, |s, cx| s.set_value(&avoided, window, cx));
        self.agent_preferred_topics_input.update(cx, |s, cx| s.set_value(&topics, window, cx));
        self.agent_mention_instruction_input.update(cx, |s, cx| s.set_value(&mention_inst, window, cx));
        self.agent_system_prompt_input.update(cx, |s, cx| s.set_value(&sys_prompt, window, cx));
        self.agent_prompt_template_input.update(cx, |s, cx| s.set_value(&tmpl, window, cx));
    }

    /// Reads all text input fields and persists them to the currently selected agent/persona profile.
    pub fn save_agent_editor_text_inputs(&mut self, cx: &mut Context<Self>) {
        let target_id = match self.selected_lab_profile_id.clone() {
            Some(id) => id,
            None => return,
        };
        let mut prof = match self.state.user_profiles.iter().find(|p| p.id == target_id).cloned() {
            Some(p) => p,
            None => return,
        };

        let name = self.agent_name_input.read(cx).value().trim().to_string();
        if !name.is_empty() {
            prof.name = name;
        }
        let handle = self.agent_handle_input.read(cx).value().trim().to_string();
        if !handle.is_empty() {
            prof.handle = if handle.starts_with('@') {
                handle
            } else {
                format!("@{}", handle)
            };
        }
        prof.config.description = self.agent_description_input.read(cx).value().trim().to_string();
        prof.config.occupation = self.agent_occupation_input.read(cx).value().trim().to_string();
        prof.config.location_culture = self.agent_location_input.read(cx).value().trim().to_string();
        prof.config.pronouns = self.agent_pronouns_input.read(cx).value().trim().to_string();

        let av_path = self
            .agent_avatar_path_input
            .read(cx)
            .value()
            .trim()
            .trim_matches('"')
            .to_string();
        prof.avatar_path = if av_path.is_empty() {
            None
        } else {
            Some(av_path)
        };

        prof.config.bio = self.agent_bio_input.read(cx).value().to_string();
        prof.config.hobbies = self.agent_hobbies_input.read(cx).value().to_string();
        prof.config.core_values = self.agent_values_input.read(cx).value().to_string();
        prof.config.daily_routine = self.agent_routine_input.read(cx).value().to_string();

        let plang = self.agent_primary_lang_input.read(cx).value().trim().to_string();
        if !plang.is_empty() {
            prof.config.primary_language = plang;
        }
        prof.config.secondary_languages = self.agent_secondary_lang_input.read(cx).value().to_string();
        prof.config.typing_quirks = self.agent_typing_quirks_input.read(cx).value().to_string();
        prof.config.signature_phrases = self.agent_signature_phrases_input.read(cx).value().to_string();
        prof.config.avoided_words = self.agent_avoided_words_input.read(cx).value().to_string();
        prof.config.preferred_topics = self.agent_preferred_topics_input.read(cx).value().to_string();
        prof.config.custom_mention_instruction = self.agent_mention_instruction_input.read(cx).value().to_string();
        prof.config.system_prompt = self.agent_system_prompt_input.read(cx).value().to_string();
        prof.config.prompt_template = self.agent_prompt_template_input.read(cx).value().to_string();

        let _ = self.state.update_profile(prof);
        cx.notify();
    }

    /// Selects a specific agent or persona in the Two-Pane Lab and syncs all text inputs.
    pub fn select_lab_profile(
        &mut self,
        profile_id: ProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.selected_lab_profile_id = Some(profile_id.clone());
        if let Some(prof) = self.state.user_profiles.iter().find(|p| p.id == profile_id).cloned() {
            self.sync_agent_editor_inputs(&prof, window, cx);
        }
        cx.notify();
    }

    /// Ensures `selected_lab_profile_id` points to a valid entry for the active mode and network.
    pub fn ensure_valid_lab_selection(
        &mut self,
        mode: LabMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let valid_ids: Vec<ProfileId> = match mode {
            LabMode::NetworkAgents => self
                .state
                .current_network_agents()
                .into_iter()
                .map(|p| p.id.clone())
                .collect(),
            LabMode::PersonaLab => self
                .state
                .current_network_personas()
                .into_iter()
                .map(|p| p.id.clone())
                .collect(),
        };

        let needs_reselect = match &self.selected_lab_profile_id {
            Some(id) => !valid_ids.contains(id),
            None => true,
        };

        if needs_reselect {
            if let Some(first_id) = valid_ids.first().cloned() {
                self.select_lab_profile(first_id, window, cx);
            }
        }
    }

    /// Submits the thought content to the active network timeline.
    pub fn submit_post_content(&mut self, content: &str, window: &mut Window, cx: &mut Context<Self>) {
        if content.trim().is_empty() {
            return;
        }

        if let Ok(post_id) = self.state.submit_post(content) {
            self.selected_thread_post_id = Some(post_id);
            let composer_view = self.composer_view.clone();
            window.defer(cx, move |window, cx| {
                composer_view.update(cx, |composer, cx| {
                    composer.reset(window, cx);
                });
            });
            self.state.draft_text.clear();
            cx.notify();
        }
    }

    /// Submits the current thought draft to the active network timeline.
    pub fn submit_post(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.composer_input.read(cx).value().to_string();
        self.submit_post_content(&content, window, cx);
    }

    /// Opens the Threads-style composer modal in the specified mode (NewThread, Reply, Quote).
    pub fn open_composer_modal(
        &mut self,
        mode: ComposerModalMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.composer_modal_mode = mode;
        self.is_composer_modal_open = true;
        self.is_modal_link_input_open = false;
        self.is_modal_image_input_open = false;
        self.modal_image_path = None;
        self.modal_link_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.modal_image_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.modal_composer_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
            state.focus(window, cx);
        });
        cx.notify();
    }

    /// Closes the composer modal and clears modal state.
    pub fn close_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_composer_modal_open = false;
        self.is_modal_link_input_open = false;
        self.is_modal_image_input_open = false;
        self.modal_image_path = None;
        self.modal_link_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.modal_image_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        self.modal_composer_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });
        cx.notify();
    }

    /// Submits the composer modal content according to its mode (NewThread, Reply, Quote).
    pub fn submit_composer_modal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.modal_composer_input.read(cx).value().to_string();
        let link_val = self.modal_link_input.read(cx).value().to_string();
        let link_opt = if !link_val.trim().is_empty() {
            Some(link_val.trim().to_string())
        } else {
            None
        };

        let manual_img = self
            .modal_image_input
            .read(cx)
            .value()
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        let image_opt = self.modal_image_path.clone().or_else(|| {
            if !manual_img.is_empty() && std::path::Path::new(&manual_img).exists() {
                Some(manual_img)
            } else {
                None
            }
        });

        if content.trim().is_empty() && link_opt.is_none() && image_opt.is_none() {
            return;
        }

        let result = match &self.composer_modal_mode {
            ComposerModalMode::NewThread => {
                self.state
                    .submit_post_with_attachments(&content, link_opt, image_opt, None, None)
            }
            ComposerModalMode::Reply(parent) => {
                self.state
                    .submit_reply(&parent.id, &content, link_opt, image_opt)
            }
            ComposerModalMode::Quote(quoted) => {
                self.state
                    .submit_quote(&quoted.id, &content, link_opt, image_opt)
            }
        };

        if let Ok(post_id) = result {
            if !matches!(self.composer_modal_mode, ComposerModalMode::Reply(_)) {
                self.selected_thread_post_id = Some(post_id);
            }
            self.close_composer_modal(window, cx);
        }
    }

    /// Adds a new grounding document to the Knowledge Vault.
    pub fn add_document(
        &mut self,
        title: String,
        content: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.state.add_document(&title, &content).is_ok() {
            self.doc_title_input.update(cx, |state, cx| {
                state.set_value("", window, cx);
            });
            self.doc_content_input.update(cx, |state, cx| {
                state.set_value("", window, cx);
            });
            cx.notify();
        }
    }

    /// Removes a document from the vault.
    pub fn delete_document(&mut self, doc_id: String, cx: &mut Context<Self>) {
        let _ = self.state.delete_document(&doc_id);
        cx.notify();
    }

    /// Triggers an autonomous synthesis for a specific network member agent (or the first active agent in the current network).
    pub fn trigger_simulation_for_agent(
        &mut self,
        agent_id: Option<ProfileId>,
        cx: &mut Context<Self>,
    ) {
        let active_net = self.state.active_network_id.clone();
        let agent_opt = if let Some(ref target_id) = agent_id {
            self.state
                .user_profiles
                .iter()
                .find(|p| &p.id == target_id && p.is_agent())
                .cloned()
        } else {
            self.state
                .current_network_agents()
                .into_iter()
                .find(|p| p.config.is_active)
                .or_else(|| self.state.current_network_agents().first().copied())
                .cloned()
        };

        let last_post_opt = self.state.current_network_posts().next().cloned();
        if let (Some(agent), Some(last_post)) = (agent_opt, last_post_opt) {
            let client = crate::agent::LlmClient::new(self.state.llm_settings.clone());
            let rx = crate::agent::DialecticEngine::spawn_background_simulation(
                last_post,
                agent,
                active_net,
                client,
            );

            cx.spawn(async |entity, cx| {
                if let Ok(reply) = rx.await {
                    let _ = entity.update(cx, |this, cx| {
                        let parent_idx_opt = reply
                            .reply_to_id
                            .as_ref()
                            .and_then(|parent_id| this.state.post_index_by_id.get(parent_id).copied());

                        if let (Some(ref parent_id), Some(parent_idx)) =
                            (&reply.reply_to_id, parent_idx_opt)
                        {
                            if let Some(parent) = this.state.posts.get_mut(parent_idx) {
                                parent.replies_count += 1;
                                if let Some(ref db) = this.state.db {
                                    let _ =
                                        db.update_post_replies_count(parent_id, parent.replies_count);
                                }
                            }
                        }

                        if let Some(ref db) = this.state.db {
                            let _ = db.save_post(&reply);
                        }
                        let excerpt = crate::state::AppState::compute_wikilink_excerpt(&reply);
                        this.state
                            .wikilink_cache
                            .insert(reply.id.as_str().to_string(), excerpt);
                        if let Some(parent_idx) = parent_idx_opt {
                            let insert_pos = (parent_idx + 1).min(this.state.posts.len());
                            this.state.posts.insert(insert_pos, reply);
                        } else {
                            this.state.posts.insert(0, reply);
                        }
                        this.state.sync_post_index();
                        cx.notify();
                    });
                }
            })
            .detach();
        }
    }

    /// Triggers an autonomous dialectic heartbeat simulation in the current network.
    pub fn trigger_simulation(&mut self, cx: &mut Context<Self>) {
        self.trigger_simulation_for_agent(None, cx);
    }

    /// Appends helper syntax such as `[[` or `#` to the composer and focuses it.
    pub fn insert_composer_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_input.update(cx, |state, cx| {
            let mut current = state.value().to_string();
            current.push_str(text);
            state.set_value(&current, window, cx);
            state.focus(window, cx);
        });
        self.state.draft_text = self.composer_input.read(cx).value().to_string();
        cx.notify();
    }

    /// Switches the active application language and updates all interface tokens dynamically.
    pub fn switch_language(&mut self, lang: Language, cx: &mut Context<Self>) {
        self.current_language = lang;
        crate::i18n::set_locale(lang);
        cx.notify();
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let search_query = self.search_input.read(cx).value().to_string();

        let current_profile = self.state.current_profile();
        let current_theme = &self.theme;
        self.composer_view.update(cx, |composer, cx| {
            composer.set_profile(current_profile, cx);
            composer.set_theme(current_theme, cx);
        });

        let entity_lang = cx.entity().clone();
        let entity_set = cx.entity().clone();
        let entity_left_tog = cx.entity().clone();
        let entity_right_tog = cx.entity().clone();

        let title_bar = AppTitleBar::new(
            self.state.current_network().cloned(),
            self.current_language,
            self.theme.clone(),
            move |_, cx| {
                entity_left_tog.update(cx, |this, cx| {
                    this.is_left_sidebar_collapsed = !this.is_left_sidebar_collapsed;
                    cx.notify();
                });
            },
            move |_, cx| {
                entity_right_tog.update(cx, |this, cx| {
                    this.is_right_sidebar_open = !this.is_right_sidebar_open;
                    cx.notify();
                });
            },
            move |_, cx| {
                entity_lang.update(cx, |this, cx| {
                    this.is_lang_menu_open = !this.is_lang_menu_open;
                    this.is_settings_open = false;
                    this.is_workspace_open = false;
                    cx.notify();
                });
            },
            move |_, cx| {
                entity_set.update(cx, |this, cx| {
                    this.is_settings_open = !this.is_settings_open;
                    this.is_lang_menu_open = false;
                    this.is_workspace_open = false;
                    cx.notify();
                });
            },
        );

        let entity_sel = cx.entity().clone();
        let entity_dis = cx.entity().clone();

        let lang_menu = if self.is_lang_menu_open {
            Some(LanguagePopover::new(
                self.current_language,
                self.theme.clone(),
                move |lang, _, cx| {
                    entity_sel.update(cx, |this, cx| {
                        this.switch_language(lang, cx);
                        this.is_lang_menu_open = false;
                    });
                },
                move |_, cx| {
                    entity_dis.update(cx, |this, cx| {
                        this.is_lang_menu_open = false;
                        cx.notify();
                    });
                },
            ))
        } else {
            None
        };

        let entity_close = cx.entity().clone();
        let entity_save_settings = cx.entity().clone();
        let entity_prov = cx.entity().clone();
        let entity_preset = cx.entity().clone();
        let entity_settings_tab = cx.entity().clone();
        let entity_theme = cx.entity().clone();

        let settings_modal = if self.is_settings_open {
            Some(SettingsModal::new(
                self.theme.clone(),
                self.state.llm_settings.clone(),
                self.active_settings_tab,
                self.llm_api_key_input.clone(),
                self.llm_model_input.clone(),
                self.llm_endpoint_input.clone(),
                move |tab, _, cx| {
                    entity_settings_tab.update(cx, |this, cx| {
                        this.active_settings_tab = tab;
                        cx.notify();
                    });
                },
                move |new_theme, _, cx| {
                    entity_theme.update(cx, |this, cx| {
                        this.state.update_theme_tokens(new_theme.clone());
                        this.theme = new_theme;
                        cx.notify();
                    });
                },
                move |provider, window, cx| {
                    entity_prov.update(cx, |this, cx| {
                        this.state.llm_settings.provider = provider;
                        let def_model = provider.default_model();
                        this.llm_model_input.update(cx, |st, cx| {
                            st.set_value(def_model, window, cx);
                        });
                        this.state.llm_settings.model = def_model.to_string();
                        cx.notify();
                    });
                },
                move |preset, window, cx| {
                    entity_preset.update(cx, |this, cx| {
                        this.llm_model_input.update(cx, |st, cx| {
                            st.set_value(preset, window, cx);
                        });
                        this.state.llm_settings.model = preset.to_string();
                        cx.notify();
                    });
                },
                move |_, cx| {
                    entity_save_settings.update(cx, |this, cx| {
                        let key = this.llm_api_key_input.read(cx).value().to_string();
                        let model = this.llm_model_input.read(cx).value().to_string();
                        let endpoint = this.llm_endpoint_input.read(cx).value().to_string();

                        this.state.llm_settings.api_key = key;
                        this.state.llm_settings.model = if model.trim().is_empty() {
                            this.state.llm_settings.provider.default_model().to_string()
                        } else {
                            model
                        };
                        this.state.llm_settings.custom_endpoint = endpoint;

                        this.state.update_llm_settings(this.state.llm_settings.clone());
                        this.is_settings_open = false;
                        cx.notify();
                    });
                },
                move |_, cx| {
                    entity_close.update(cx, |this, cx| {
                        this.is_settings_open = false;
                        cx.notify();
                    });
                },
            ))
        } else {
            None
        };

        let entity_ws_sel = cx.entity().clone();
        let entity_ws_dis = cx.entity().clone();

        let workspace_popover = if self.is_workspace_open {
            Some(WorkspacePopover::new(
                self.state.current_network().cloned(),
                self.state.networks.clone(),
                self.is_left_sidebar_collapsed,
                self.theme.clone(),
                move |net_id, window, cx| {
                    entity_ws_sel.update(cx, |this, cx| {
                        let _ = this.state.switch_network(&net_id);
                        this.is_workspace_open = false;
                        let mode = if this.state.active_tab == NavigationTab::PersonaLab {
                            LabMode::PersonaLab
                        } else {
                            LabMode::NetworkAgents
                        };
                        this.ensure_valid_lab_selection(mode, window, cx);
                        cx.notify();
                    });
                },
                move |_, cx| {
                    entity_ws_dis.update(cx, |this, cx| {
                        this.is_workspace_open = false;
                        cx.notify();
                    });
                },
            ))
        } else {
            None
        };

        let entity_nav = cx.entity().clone();
        let entity_sidebar_agent = cx.entity().clone();
        let entity_sidebar_switch = cx.entity().clone();
        let entity_like = cx.entity().clone();
        let entity_reply = cx.entity().clone();
        let entity_repost = cx.entity().clone();
        let entity_quote = cx.entity().clone();
        let entity_new_thread = cx.entity().clone();
        let entity_topic = cx.entity().clone();
        let entity_ws_tog = cx.entity().clone();
        let entity_add_doc = cx.entity().clone();
        let entity_del_doc = cx.entity().clone();
        let entity_sim = cx.entity().clone();
        let entity_cycle = cx.entity().clone();
        let entity_day = cx.entity().clone();
        let entity_side_tab = cx.entity().clone();
        let entity_select_post = cx.entity().clone();
        let entity_load_more = cx.entity().clone();

        let entity_close_composer = cx.entity().clone();
        let entity_submit_composer = cx.entity().clone();
        let entity_tog_modal_link = cx.entity().clone();
        let entity_tog_modal_img = cx.entity().clone();
        let entity_pick_modal_img = cx.entity().clone();
        let entity_clear_modal_img = cx.entity().clone();
        let entity_clear_modal_link = cx.entity().clone();

        let composer_modal = if self.is_composer_modal_open {
            let draft_length = self.modal_composer_input.read(cx).value().chars().count();
            Some(ComposerModalView::new(
                self.theme.clone(),
                self.state.current_profile().cloned(),
                self.composer_modal_mode.clone(),
                self.modal_composer_input.clone(),
                self.modal_link_input.clone(),
                self.modal_image_input.clone(),
                self.modal_image_path.clone(),
                self.is_modal_link_input_open,
                self.is_modal_image_input_open,
                draft_length,
                move |window, cx| {
                    entity_close_composer.update(cx, |this, cx| {
                        this.close_composer_modal(window, cx);
                    });
                },
                move |window, cx| {
                    entity_submit_composer.update(cx, |this, cx| {
                        this.submit_composer_modal(window, cx);
                    });
                },
                move |window, cx| {
                    entity_tog_modal_link.update(cx, |this, cx| {
                        this.is_modal_link_input_open = !this.is_modal_link_input_open;
                        if this.is_modal_link_input_open {
                            this.modal_link_input.update(cx, |st, cx| {
                                st.focus(window, cx);
                            });
                        }
                        cx.notify();
                    });
                },
                move |window, cx| {
                    entity_tog_modal_img.update(cx, |this, cx| {
                        this.is_modal_image_input_open = !this.is_modal_image_input_open;
                        if this.is_modal_image_input_open {
                            this.modal_image_input.update(cx, |st, cx| {
                                st.focus(window, cx);
                            });
                        }
                        cx.notify();
                    });
                },
                move |_, cx| {
                    entity_pick_modal_img.update(cx, |_this, cx| {
                        cx.spawn(async |entity, cx| {
                            let file = rfd::AsyncFileDialog::new()
                                .set_title("Select Image")
                                .add_filter("Image Files", &["png", "jpg", "jpeg", "webp", "gif", "bmp"])
                                .pick_file()
                                .await;

                            if let Some(file_handle) = file {
                                let path = file_handle.path().to_string_lossy().to_string();
                                let _ = entity.update(cx, |this, cx| {
                                    if std::path::Path::new(&path).exists() {
                                        this.modal_image_path = Some(path);
                                        this.is_modal_image_input_open = true;
                                        cx.notify();
                                    }
                                });
                            }
                        })
                        .detach();
                    });
                },
                move |window, cx| {
                    entity_clear_modal_img.update(cx, |this, cx| {
                        this.modal_image_path = None;
                        this.is_modal_image_input_open = false;
                        this.modal_image_input.update(cx, |st, cx| {
                            st.set_value("", window, cx);
                        });
                        cx.notify();
                    });
                },
                move |window, cx| {
                    entity_clear_modal_link.update(cx, |this, cx| {
                        this.modal_link_input.update(cx, |st, cx| {
                            st.set_value("", window, cx);
                        });
                        this.is_modal_link_input_open = false;
                        cx.notify();
                    });
                },
            ))
        } else {
            None
        };

        let custom_profile = self
            .custom_feed_profile_id
            .as_ref()
            .and_then(|id| self.state.user_profiles.iter().find(|p| &p.id == id));

        let posts = self.state.posts();
        let docs = self.state.documents();
        let selected_day = self.selected_calendar_day;
        let is_lab_view = matches!(
            self.state.active_tab,
            NavigationTab::AgentLab | NavigationTab::PersonaLab
        );

        let center_panel_content = if is_lab_view {
            let lab_mode = if self.state.active_tab == NavigationTab::PersonaLab {
                LabMode::PersonaLab
            } else {
                LabMode::NetworkAgents
            };

            let entity_sel_prof = cx.entity().clone();
            let entity_sel_tab = cx.entity().clone();
            let entity_sel_sub_tab = cx.entity().clone();
            let entity_sw_mode = cx.entity().clone();
            let entity_upd_prof = cx.entity().clone();
            let entity_save_txt = cx.entity().clone();
            let entity_create_prof = cx.entity().clone();
            let entity_del_prof = cx.entity().clone();
            let entity_pick_av = cx.entity().clone();
            let entity_act_pers = cx.entity().clone();
            let entity_trig_ag = cx.entity().clone();

            render_agent_workspace_view(AgentWorkspaceParams {
                state: &self.state,
                mode: lab_mode,
                selected_profile_id: self.selected_lab_profile_id.as_ref(),
                active_config_tab: self.active_agent_config_tab,
                active_sub_tab: self.active_agent_sub_tab,
                inputs: AgentEditorInputs {
                    name_input: &self.agent_name_input,
                    handle_input: &self.agent_handle_input,
                    description_input: &self.agent_description_input,
                    occupation_input: &self.agent_occupation_input,
                    location_input: &self.agent_location_input,
                    pronouns_input: &self.agent_pronouns_input,
                    avatar_path_input: &self.agent_avatar_path_input,
                    bio_input: &self.agent_bio_input,
                    hobbies_input: &self.agent_hobbies_input,
                    values_input: &self.agent_values_input,
                    routine_input: &self.agent_routine_input,
                    primary_lang_input: &self.agent_primary_lang_input,
                    secondary_lang_input: &self.agent_secondary_lang_input,
                    typing_quirks_input: &self.agent_typing_quirks_input,
                    signature_phrases_input: &self.agent_signature_phrases_input,
                    avoided_words_input: &self.agent_avoided_words_input,
                    preferred_topics_input: &self.agent_preferred_topics_input,
                    mention_instruction_input: &self.agent_mention_instruction_input,
                    system_prompt_input: &self.agent_system_prompt_input,
                    prompt_template_input: &self.agent_prompt_template_input,
                },
                theme: &self.theme,
                on_select_profile: Box::new(move |prof_id, window, cx| {
                    entity_sel_prof.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        this.select_lab_profile(prof_id, window, cx);
                    });
                }),
                on_select_tab: Box::new(move |tab, _window, cx| {
                    entity_sel_tab.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        if this.active_agent_config_tab != tab {
                            this.active_agent_config_tab = tab;
                            this.active_agent_sub_tab = 0;
                        }
                        cx.notify();
                    });
                }),
                on_select_sub_tab: Box::new(move |sub_idx, _window, cx| {
                    entity_sel_sub_tab.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        this.active_agent_sub_tab = sub_idx;
                        cx.notify();
                    });
                }),
                on_switch_mode: Box::new(move |new_mode, window, cx| {
                    entity_sw_mode.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        this.state.active_tab = match new_mode {
                            LabMode::NetworkAgents => NavigationTab::AgentLab,
                            LabMode::PersonaLab => NavigationTab::PersonaLab,
                        };
                        this.ensure_valid_lab_selection(new_mode, window, cx);
                        cx.notify();
                    });
                }),
                on_update_profile: Box::new(move |updated_prof, window, cx| {
                    entity_upd_prof.update(cx, |this, cx| {
                        let _ = this.state.update_profile(updated_prof.clone());
                        this.sync_agent_editor_inputs(&updated_prof, window, cx);
                        cx.notify();
                    });
                }),
                on_save_text_inputs: Box::new(move |_window, cx| {
                    entity_save_txt.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                    });
                }),
                on_create_profile: Box::new(move |window, cx| {
                    entity_create_prof.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        let res = if this.state.active_tab == NavigationTab::PersonaLab {
                            let count = this.state.current_network_personas().len() + 1;
                            let curated = [
                                ("Elena Vance", "@elena_vance", "Literary Essayist & Cultural Critic"),
                                ("Reza Mahendra", "@reza_m", "Systems Architect & Phenomenologist"),
                                ("Clara Lindqvist", "@clara_l", "Cognitive Scientist & Researcher"),
                                ("Julian Thorne", "@julian_t", "Historian of Ideas"),
                            ];
                            let (name, handle, role) = curated
                                .get(count.saturating_sub(2))
                                .copied()
                                .unwrap_or(("Nadia Prameswari", "@nadia_p", "Independent Writer"));
                            let unique_handle = if this.state.user_profiles.iter().any(|p| p.handle == handle) {
                                format!("{}_{}", handle, count)
                            } else {
                                handle.to_string()
                            };
                            this.state.create_human_persona(name, &unique_handle, role)
                        } else {
                            let count = this.state.current_network_agents().len() + 1;
                            let curated = [
                                ("Michel de Montaigne", "@montaigne", "Humanist Essayist & Skeptic"),
                                ("Ibn Khaldun", "@ibn_khaldun", "Historian & Social Theorist"),
                                ("Susanne Langer", "@susanne_langer", "Philosopher of Mind & Art"),
                                ("Tan Malaka", "@tan_malaka", "Materialist Dialectician & Educator"),
                            ];
                            let (name, handle, role) = curated
                                .get(count.saturating_sub(3))
                                .copied()
                                .unwrap_or(("Arthur Schopenhauer", "@schopenhauer", "Metaphysician & Aesthete"));
                            let unique_handle = if this.state.user_profiles.iter().any(|p| p.handle == handle) {
                                format!("{}_{}", handle, count)
                            } else {
                                handle.to_string()
                            };
                            this.state.create_network_agent(name, &unique_handle, role)
                        };
                        if let Ok(new_id) = res {
                            this.active_agent_config_tab = AgentConfigTab::IdentityAndBio;
                            this.active_agent_sub_tab = 0;
                            this.select_lab_profile(new_id, window, cx);
                        }
                    });
                }),
                on_delete_profile: Box::new(move |prof_id, window, cx| {
                    entity_del_prof.update(cx, |this, cx| {
                        let _ = this.state.delete_profile(&prof_id);
                        let mode = if this.state.active_tab == NavigationTab::PersonaLab {
                            LabMode::PersonaLab
                        } else {
                            LabMode::NetworkAgents
                        };
                        this.ensure_valid_lab_selection(mode, window, cx);
                        cx.notify();
                    });
                }),
                on_pick_avatar: Box::new(move |prof_id, _window, cx| {
                    entity_pick_av.update(cx, |_this, cx| {
                        cx.spawn(async move |entity, cx| {
                            let file = rfd::AsyncFileDialog::new()
                                .set_title("Select Avatar Portrait Image")
                                .add_filter("Image Files", &["png", "jpg", "jpeg", "webp", "gif", "bmp"])
                                .pick_file()
                                .await;

                            if let Some(file_handle) = file {
                                let path = file_handle.path().to_string_lossy().to_string();
                                let _ = entity.update(cx, |this, cx| {
                                    if std::path::Path::new(&path).exists() {
                                        if let Some(mut prof) = this
                                            .state
                                            .user_profiles
                                            .iter()
                                            .find(|p| p.id == prof_id)
                                            .cloned()
                                        {
                                            prof.avatar_path = Some(path);
                                            let _ = this.state.update_profile(prof);
                                            cx.notify();
                                        }
                                    }
                                });
                            }
                        })
                        .detach();
                    });
                }),
                on_activate_persona: Box::new(move |prof_id, _window, cx| {
                    entity_act_pers.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        let _ = this.state.switch_profile(&prof_id);
                        cx.notify();
                    });
                }),
                on_trigger_agent: Box::new(move |prof_id, _window, cx| {
                    entity_trig_ag.update(cx, |this, cx| {
                        this.save_agent_editor_text_inputs(cx);
                        this.trigger_simulation_for_agent(Some(prof_id), cx);
                    });
                }),
            })
            .into_any_element()
        } else {
            render_main_feed(FeedParams {
                state: &self.state,
                custom_profile,
                selected_post_id: self.selected_thread_post_id.clone(),
                composer: self.composer_view.clone(),
                doc_title_input: &self.doc_title_input,
                doc_content_input: &self.doc_content_input,
                search_query: &search_query,
                theme: &self.theme,
                feed_visible_count: Some(self.feed_visible_count),
                on_load_more: Some(Box::new(move |_, cx| {
                    entity_load_more.update(cx, |this, cx| {
                        this.feed_visible_count += crate::ui::organisms::FEED_WINDOW_INCREMENT;
                        cx.notify();
                    });
                })),
                on_select_post: Box::new(move |post_id: PostId, _, cx| {
                    entity_select_post.update(cx, |this, cx| {
                        this.selected_thread_post_id = Some(post_id);
                        cx.notify();
                    });
                }),
                on_like_post: Box::new(move |post_id: PostId, _, cx| {
                    entity_like.update(cx, |this, cx| {
                        this.state.toggle_like(&post_id);
                        cx.notify();
                    });
                }),
                on_reply_post: Box::new(move |post_id: PostId, _author_handle: &str, window, cx| {
                    entity_reply.update(cx, |this, cx| {
                        if let Some(target) = this.state.get_post(&post_id).cloned() {
                            this.open_composer_modal(
                                ComposerModalMode::Reply(Box::new(target)),
                                window,
                                cx,
                            );
                        }
                    });
                }),
                on_repost_post: Box::new(move |post_id: PostId, _window, cx| {
                    entity_repost.update(cx, |this, cx| {
                        let _ = this.state.repost(&post_id);
                        cx.notify();
                    });
                }),
                on_quote_post: Box::new(move |post_id: PostId, window, cx| {
                    entity_quote.update(cx, |this, cx| {
                        if let Some(target) = this.state.get_post(&post_id).cloned() {
                            this.open_composer_modal(
                                ComposerModalMode::Quote(Box::new(target)),
                                window,
                                cx,
                            );
                        }
                    });
                }),
                on_add_document: Box::new(move |title, content, window, cx| {
                    entity_add_doc.update(cx, |this, cx| {
                        this.add_document(title, content, window, cx);
                    });
                }),
                on_delete_document: Box::new(move |doc_id, _, cx| {
                    entity_del_doc.update(cx, |this, cx| {
                        this.delete_document(doc_id, cx);
                    });
                }),
                on_trigger_simulation: Box::new(move |_, cx| {
                    entity_sim.update(cx, |this, cx| {
                        this.trigger_simulation(cx);
                    });
                }),
                on_cycle_profile: Box::new(move |_, cx| {
                    entity_cycle.update(cx, |this, cx| {
                        let _ = this.state.cycle_active_persona();
                        cx.notify();
                    });
                }),
            })
            .into_any_element()
        };

        div()
            .w_full()
            .h_full()
            .bg(self.theme.bg_app)
            .relative()
            .flex()
            .flex_col()
            .child(title_bar)
            // Fully Resizable Multi-Column Layout Container
            .child(
                div()
                    .w_full()
                    .flex_1()
                    .overflow_hidden()
                    .child(
                        h_resizable(if self.is_left_sidebar_collapsed {
                            "simulacra-main-layout-collapsed"
                        } else {
                            "simulacra-main-layout-expanded"
                        })
                        // Panel 1: Left Navigation Sidebar
                        .child(
                            resizable_panel()
                                .size(if self.is_left_sidebar_collapsed {
                                    px(64.0)
                                } else {
                                    px(240.0)
                                })
                                .size_range(if self.is_left_sidebar_collapsed {
                                    px(60.0)..px(68.0)
                                } else {
                                    px(180.0)..px(380.0)
                                })
                                .child(render_left_sidebar(LeftSidebarParams {
                                    state: &self.state,
                                    theme: &self.theme,
                                    is_workspace_open: self.is_workspace_open,
                                    is_collapsed: self.is_left_sidebar_collapsed,
                                    on_toggle_workspace: Box::new(move |_, cx| {
                                        entity_ws_tog.update(cx, |this, cx| {
                                            this.is_workspace_open = !this.is_workspace_open;
                                            this.is_lang_menu_open = false;
                                            this.is_settings_open = false;
                                            cx.notify();
                                        });
                                    }),
                                    on_select_nav: Box::new(move |tab: NavigationTab, window, cx| {
                                        entity_nav.update(cx, |this, cx| {
                                            if matches!(
                                                this.state.active_tab,
                                                NavigationTab::AgentLab | NavigationTab::PersonaLab
                                            ) {
                                                this.save_agent_editor_text_inputs(cx);
                                            }
                                            if tab == NavigationTab::Appearance {
                                                this.is_settings_open = true;
                                                this.active_settings_tab = SettingsTab::Appearance;
                                                cx.notify();
                                                return;
                                            }
                                            this.state.active_tab = tab;
                                            if tab == NavigationTab::AgentLab {
                                                this.ensure_valid_lab_selection(
                                                    LabMode::NetworkAgents,
                                                    window,
                                                    cx,
                                                );
                                            } else if tab == NavigationTab::PersonaLab {
                                                // Default to selecting the currently active user persona when opening Persona Lab
                                                let active_id = this.state.active_profile_id.clone();
                                                this.select_lab_profile(active_id, window, cx);
                                            }
                                            cx.notify();
                                        });
                                    }),
                                    on_new_thread: Box::new(move |window, cx| {
                                        entity_new_thread.update(cx, |this, cx| {
                                            this.open_composer_modal(
                                                ComposerModalMode::NewThread,
                                                window,
                                                cx,
                                            );
                                        });
                                    }),
                                    on_select_agent: Some(Box::new(move |agent_id, window, cx| {
                                        entity_sidebar_agent.update(cx, |this, cx| {
                                            if matches!(
                                                this.state.active_tab,
                                                NavigationTab::AgentLab | NavigationTab::PersonaLab
                                            ) {
                                                this.save_agent_editor_text_inputs(cx);
                                            }
                                            this.state.active_tab = NavigationTab::AgentLab;
                                            this.select_lab_profile(agent_id, window, cx);
                                        });
                                    })),
                                    on_switch_persona: Some(Box::new(move |window, cx| {
                                        entity_sidebar_switch.update(cx, |this, cx| {
                                            if let Some(new_id) = this.state.cycle_active_persona() {
                                                if this.state.active_tab == NavigationTab::PersonaLab {
                                                    this.select_lab_profile(new_id, window, cx);
                                                }
                                                cx.notify();
                                            }
                                        });
                                    })),
                                })),
                        )
                        // Panel 2: Center Main Dual Feed OR Two-Pane Agent/Persona Lab
                        .child(resizable_panel().child(center_panel_content))
                        // Panel 3: Unified Intelligence & Telemetry Sidebar
                        .child(
                            resizable_panel()
                                .size(px(320.0))
                                .size_range(px(260.0)..px(460.0))
                                .visible(self.is_right_sidebar_open)
                                .child(render_unified_right_sidebar(UnifiedRightSidebarParams {
                                    active_tab: self.right_sidebar_tab,
                                    on_select_tab: Box::new(move |tab, _, cx| {
                                        entity_side_tab.update(cx, |this, cx| {
                                            this.right_sidebar_tab = tab;
                                            cx.notify();
                                        });
                                    }),
                                    state: &self.state,
                                    posts,
                                    documents: docs,
                                    selected_day,
                                    on_click_day: Rc::new(move |day_idx, _, cx| {
                                        entity_day.update(cx, |this, cx| {
                                            if this.selected_calendar_day == Some(day_idx) {
                                                this.selected_calendar_day = None;
                                            } else {
                                                this.selected_calendar_day = Some(day_idx);
                                            }
                                            cx.notify();
                                        });
                                    }),
                                    search_input: &self.search_input,
                                    on_click_topic: Box::new(move |tag, window, cx| {
                                        entity_topic.update(cx, |this, cx| {
                                            this.search_input.update(cx, |input, cx| {
                                                input.set_value(tag, window, cx);
                                            });
                                            cx.notify();
                                        });
                                    }),
                                    theme: &self.theme,
                                })),
                        ),
                    ),
            )
            .children(lang_menu)
            .children(settings_modal)
            .children(workspace_popover)
            .children(composer_modal)
    }
}
