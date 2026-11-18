//! Post actions sheet (sb C2): opening it from a post's "…" menu or the palette, and every
//! event it sends (picking an action, its choice, copy, reply, save, open a hit, close).

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::post_actions::{extract_concepts, ActionInput, ActionKind, ActionRequest, Audience, TargetLanguage};
use crate::domain::PostId;
use crate::state::semantic::hits::shortened;
use crate::ui::features::feed::PostActionCallback;
use crate::ui::features::post_actions::{ActionChoice, ActionSheet, ActionStatus, SheetEvent};
use gpui_kit::*;
use rust_i18n::t;

/// Characters of the thought shown in the sheet's header.
const SOURCE_CHARS: usize = 72;

impl AppView {
    /// The feed's "…" › Actions callback.
    pub(in crate::ui::features::shell) fn post_action_callback(bind: &Binder) -> PostActionCallback {
        let entity = bind.entity().clone();
        Box::new(move |post_id, kind, window, cx| entity.update(cx, |this, cx| this.open_post_actions(post_id, Some(kind), window, cx)))
    }

    /// Opens the sheet on `post_id`, running `kind` at once when given.
    pub(crate) fn open_post_actions(&mut self, post_id: PostId, kind: Option<ActionKind>, window: &mut Window, cx: &mut Context<Self>) {
        let post_id = self.state.action_target(&post_id);
        let Some(post) = self.state.get_post(&post_id) else { return };
        self.post_actions.cancel_running();
        let concepts = extract_concepts(&post.content, |link| self.state.get_post_by_id(link).is_some());
        let first = concepts.first().cloned().unwrap_or_default();
        let request = ActionRequest::new(kind.unwrap_or(ActionKind::SearchConcept), self.state.preferences().general.language, &first);
        let source = shortened(&post.content, SOURCE_CHARS);
        self.post_actions.sheet = Some(ActionSheet { post_id, source, request, concepts, status: ActionStatus::Picking, hits: None });
        window.focus(&self.post_actions.focus, cx);
        if let Some(kind) = kind {
            self.pick_post_action(kind, window, cx);
        }
        cx.notify();
    }

    /// The palette's "Actions on the open thought".
    pub(crate) fn open_actions_for_open_thought(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(post_id) = self.feed.selected_thread_post_id.clone() {
            self.open_post_actions(post_id, None, window, cx);
        }
    }

    pub(crate) fn on_action_sheet_event(&mut self, event: SheetEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            SheetEvent::Pick(kind) => self.pick_post_action(kind, window, cx),
            SheetEvent::Choose(choice) => self.choose_post_action_option(choice, window, cx),
            SheetEvent::ApplyTyped => self.apply_typed_choice(window, cx),
            SheetEvent::Rerun => self.run_post_action(true, window, cx),
            SheetEvent::Cancel => self.cancel_post_action(cx),
            SheetEvent::Copy => self.copy_action_answer(window, cx),
            SheetEvent::PostAsReply => self.reply_with_action_answer(window, cx),
            SheetEvent::SaveToVault => self.save_action_answer(window, cx),
            SheetEvent::OpenHit(target) => {
                self.close_action_sheet(window, cx);
                self.open_similar_hit(target, window, cx);
            }
            SheetEvent::Close => self.close_action_sheet(window, cx),
        }
    }

    fn pick_post_action(&mut self, kind: ActionKind, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.post_actions.sheet.as_mut() else { return };
        sheet.request = sheet.request.clone().with_kind(kind);
        sheet.hits = None;
        let placeholder = match kind.input() {
            ActionInput::Audience => t!("post_actions.typed_audience"),
            ActionInput::Language => t!("post_actions.typed_language"),
            ActionInput::Concept | ActionInput::None => t!("post_actions.typed_concept"),
        };
        self.post_actions.input.update(cx, |input, cx| input.set_placeholder(placeholder.to_string(), window, cx));
        self.run_post_action(false, window, cx);
    }

    fn choose_post_action_option(&mut self, choice: ActionChoice, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.post_actions.sheet.as_mut() else { return };
        match choice {
            ActionChoice::Audience(preset) => sheet.request.audience = Audience::Preset(preset),
            ActionChoice::Language(language) => sheet.request.language = TargetLanguage::Locale(language),
            ActionChoice::Concept(concept) => sheet.request.concept = concept,
        }
        self.run_post_action(false, window, cx);
    }

    /// Uses the typed audience, language or concept, then runs.
    fn apply_typed_choice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let typed = self.post_actions.input.read(cx).value().trim().to_string();
        let Some(sheet) = self.post_actions.sheet.as_mut().filter(|_| !typed.is_empty()) else { return };
        match sheet.request.kind.input() {
            ActionInput::Audience => sheet.request.audience = Audience::Custom(typed),
            ActionInput::Language => sheet.request.language = TargetLanguage::Typed(typed),
            ActionInput::Concept | ActionInput::None => {
                if !sheet.concepts.contains(&typed) {
                    sheet.concepts.insert(0, typed.clone());
                }
                sheet.request.concept = typed;
            }
        }
        self.post_actions.input.update(cx, |input, cx| input.set_value("", window, cx));
        self.run_post_action(false, window, cx);
    }

    fn cancel_post_action(&mut self, cx: &mut Context<Self>) {
        self.post_actions.cancel_running();
        if let Some(sheet) = self.post_actions.sheet.as_mut() {
            sheet.status = ActionStatus::Failed(t!("post_actions.failure.cancelled").to_string());
        }
        cx.notify();
    }

    /// Closes the sheet (stopping its request) and gives the keyboard back to the shell.
    pub(crate) fn close_action_sheet(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.post_actions.cancel_running();
        self.post_actions.sheet = None;
        window.focus(&self.palette.shell_focus, cx);
        cx.notify();
    }
}
