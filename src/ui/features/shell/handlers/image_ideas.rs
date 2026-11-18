//! Image ideas (sb D3): opening the sheet from the Forge, the composer or the palette, and
//! the picks that rebuild its prompt.

use super::super::view::AppView;
use crate::agent::runtime::resolve_image_job;
use crate::domain::engine::{ImageEngine, ImageEngineConfig};
use crate::domain::image_prompt::{
    assemble_prompt, distinct_names, engine_for, is_configured, surprise, ImageChoices, ImageContext, ImageSubject, ImageTarget, PromptEngine,
};
use crate::ui::features::image_prompt::{CandidatesStatus, GenerationStatus, IdeasDestination, IdeasFailure, IdeasPick, IdeasSheet};
use gpui_kit::*;

/// sRGB channel scale of the theme's float colours.
const CHANNEL_MAX: f32 = 255.0;
/// Aspect key of avatars (always square).
const AVATAR_ASPECT: &str = "square";

/// Who or what the sheet is for.
pub(in crate::ui::features::shell) struct IdeasOpen {
    pub target: ImageTarget,
    pub destination: IdeasDestination,
    pub about: String,
    pub subject: ImageSubject,
    /// The agent's or persona's own image engine (an unset one falls back to Settings › Images).
    pub own_engine: ImageEngineConfig,
}

impl AppView {
    /// Names of the active theme's main colours, for "match my theme".
    fn theme_colour_names(&self) -> Vec<String> {
        let theme = &self.theme;
        let channels = |c: Rgba| ((c.r * CHANNEL_MAX).round() as u8, (c.g * CHANNEL_MAX).round() as u8, (c.b * CHANNEL_MAX).round() as u8);
        distinct_names(&[theme.accent_brand, theme.accent_dialectic, theme.accent_success, theme.bg_app].map(channels))
    }

    /// Why the engine cannot draw, if it cannot.
    fn engine_problem(&self, config: &ImageEngineConfig) -> Option<IdeasFailure> {
        let job = resolve_image_job(config, &self.state.credentials, self.state.media_dir());
        let keyed = matches!(config.engine, ImageEngine::OpenAi | ImageEngine::Google);
        match (is_configured(&config.engine), keyed && job.secret.is_empty()) {
            (false, _) => Some(IdeasFailure::NoEngine),
            (true, true) => Some(IdeasFailure::NoImageKey),
            (true, false) => None,
        }
    }

    /// Opens the sheet with default picks and the prompt they make.
    pub(in crate::ui::features::shell) fn open_image_ideas(&mut self, open: IdeasOpen, window: &mut Window, cx: &mut Context<Self>) {
        self.image_ideas.cancel_running();
        let prefs = self.state.preferences().images.clone();
        let config = engine_for(&open.own_engine, &prefs);
        let engine = PromptEngine::of(&config, &prefs.style_prefix);
        let engine = if open.target == ImageTarget::Avatar { engine.with_aspect(AVATAR_ASPECT) } else { engine };
        let context = ImageContext { target: open.target, subject: open.subject, theme_colours: self.theme_colour_names() };
        let seed = chrono::Utc::now().timestamp_millis().unsigned_abs();
        self.image_ideas.sheet = Some(IdeasSheet {
            destination: open.destination,
            about: open.about,
            choices: ImageChoices::for_target(open.target),
            context,
            engine_problem: self.engine_problem(&config),
            engine,
            engine_config: config,
            negative: None,
            candidates: CandidatesStatus::Idle,
            generation: GenerationStatus::Idle,
            results: Vec::new(),
            seed,
        });
        self.rebuild_image_prompt(window, cx);
        window.focus(&self.image_ideas.focus, cx);
        cx.notify();
    }

    /// Writes the prompt of the current picks into the editor.
    pub(in crate::ui::features::shell) fn rebuild_image_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.image_ideas.sheet.as_mut() else { return };
        let prompt = assemble_prompt(&sheet.choices, &sheet.context, &sheet.engine);
        sheet.negative = prompt.negative.clone();
        self.image_ideas.prompt.update(cx, |editor, cx| editor.set_value(prompt.text, window, cx));
    }

    pub(in crate::ui::features::shell) fn pick_image_idea(&mut self, pick: IdeasPick, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sheet) = self.image_ideas.sheet.as_mut() {
            sheet.choices = pick.apply(sheet.choices.clone());
        }
        self.rebuild_image_prompt(window, cx);
        cx.notify();
    }

    /// "Surprise me": the next seeded set of coherent picks.
    pub(in crate::ui::features::shell) fn surprise_image_idea(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sheet) = self.image_ideas.sheet.as_mut() {
            sheet.seed = sheet.seed.wrapping_add(1);
            sheet.choices = surprise(sheet.context.target, sheet.seed);
        }
        self.rebuild_image_prompt(window, cx);
        cx.notify();
    }

    pub(in crate::ui::features::shell) fn close_image_ideas(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.image_ideas.cancel_running();
        self.image_ideas.sheet = None;
        window.focus(&self.palette.shell_focus, cx);
        cx.notify();
    }
}
