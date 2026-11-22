//! Renders the panel through the stateless [`ComposerModalView`] dialog.

use super::super::format::FormatCallback;
use super::super::messages::visible_problem_message;
use super::super::symbol_picker::SymbolCallback;
use super::super::tint_picker::TintCallback;
use super::super::modal::{ComposerModalView, DraftActionCallback, DraftEditor, ModalCallbacks, PasteCallback};
use super::ComposerModalPanel;
use gpui_kit::*;
use std::rc::Rc;

type PanelAction = fn(&mut ComposerModalPanel, &mut Window, &mut Context<ComposerModalPanel>);
type PanelDraftAction = fn(&mut ComposerModalPanel, usize, &mut Window, &mut Context<ComposerModalPanel>);

/// Wraps a panel method as a `(Window, App)` callback.
fn act(entity: &Entity<ComposerModalPanel>, action: PanelAction) -> Rc<dyn Fn(&mut Window, &mut App)> {
    let entity = entity.clone();
    Rc::new(move |window, cx| entity.update(cx, |this, cx| action(this, window, cx)))
}

/// Wraps a per-draft panel method as an `(index, Window, App)` callback.
fn act_at(entity: &Entity<ComposerModalPanel>, action: PanelDraftAction) -> DraftActionCallback {
    let entity = entity.clone();
    Rc::new(move |index, window, cx| entity.update(cx, |this, cx| action(this, index, window, cx)))
}

/// Routes a draft's paste to the panel (images are attached instead of pasted as text).
fn paste(entity: &Entity<ComposerModalPanel>) -> PasteCallback {
    let entity = entity.clone();
    Rc::new(move |item, window, cx| entity.update(cx, |this, cx| this.paste_media(item, window, cx)))
}

fn callbacks(entity: &Entity<ComposerModalPanel>) -> ModalCallbacks {
    type P = ComposerModalPanel;
    ModalCallbacks {
        on_close: act(entity, P::close),
        on_submit: act(entity, P::submit),
        on_toggle_link: act(entity, P::toggle_link),
        on_toggle_image: act(entity, P::toggle_image),
        on_pick_image: act(entity, P::pick_image),
        on_clear_image: act(entity, P::clear_image),
        on_clear_link: act(entity, P::clear_link),
        on_add_draft: act(entity, P::add_draft),
        on_insert_tag: act(entity, P::insert_tag),
        on_image_ideas: act(entity, P::image_ideas),
        on_remove_draft: act_at(entity, P::remove_draft),
        on_split_draft: act_at(entity, P::split_draft),
        on_remove_image: act_at(entity, P::remove_image),
        on_remove_file: act_at(entity, P::remove_file),
        on_paste: paste(entity),
        on_format: format_draft(entity),
        on_toggle_preview: act(entity, P::toggle_preview),
        on_pick_tint: pick_tint(entity),
        on_toggle_schedule: act(entity, P::toggle_schedule),
        on_pick_symbol: pick_symbol(entity),
        on_toggle_symbol_search: act(entity, P::toggle_symbol_search),
        on_toggle_free: act(entity, P::toggle_free),
        on_toggle_todo_help: act(entity, P::toggle_todo_help),
    }
}

/// Routes a toolbar mark to the last focused draft.
fn format_draft(entity: &Entity<ComposerModalPanel>) -> FormatCallback {
    let entity = entity.clone();
    Rc::new(move |mark, window, cx| entity.update(cx, |this, cx| this.format_active(mark, window, cx)))
}

/// Routes a picked symbol to the panel (S5).
fn pick_symbol(entity: &Entity<ComposerModalPanel>) -> SymbolCallback {
    let entity = entity.clone();
    Rc::new(move |choice, _, cx| entity.update(cx, |this, cx| this.pick_symbol(choice, cx)))
}

/// Routes a picked tint to the panel (sb C3).
fn pick_tint(entity: &Entity<ComposerModalPanel>) -> TintCallback {
    let entity = entity.clone();
    Rc::new(move |tint, _, cx| entity.update(cx, |this, cx| this.pick_tint(tint, cx)))
}

impl Render for ComposerModalPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let problem = self.submit_problem(cx);
        let notice = self
            .error
            .clone()
            .or_else(|| visible_problem_message(problem, self.drafts.len()));
        let drafts = self
            .drafts
            .iter()
            .map(|slot| {
                let text = slot.input.read(cx).value().to_string();
                let length = crate::domain::post::visible_text_len(&text);
                DraftEditor { input: slot.input.clone(), length, text }
            })
            .collect();
        ComposerModalView {
            theme: self.theme.clone(),
            profile: self.profile.clone(),
            mode: self.mode.clone(),
            drafts,
            link_input: self.link_input.clone(),
            image_input: self.image_input.clone(),
            images: self.image_views(),
            pending_images: self.pending_images(), files: self.files.attached.clone(), pending_files: self.files.pending, // L12: file chips
            is_link_input_open: self.is_link_input_open,
            is_image_input_open: self.is_image_input_open,
            notice,
            can_post: problem.is_none() && !matches!(self.schedule_choice(cx), Some(Err(_))),
            preview: self.preview,
            tint: self.tint,
            schedule: self.schedule_view(cx),
            symbols: self.symbol_view(cx),
            version: self.version_view(cx),
            free: self.free_view(cx),
            todo_help: self.todo_help_open(), todo_suggest: self.todo_suggest_view(cx), // D1, D11
            atomic_hint: self.atomic_hint_view(cx),
            readability: self.readability_view(cx),
            callbacks: callbacks(&cx.entity()),
        }
    }
}
