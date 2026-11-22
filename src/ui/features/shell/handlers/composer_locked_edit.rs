//! Editing a locked post from the composer (X5): the composer opens on the text unlocked
//! this session (a locked post is unlocked first), with "Lock with password" open and
//! required; saving seals the new text in the background and keeps it unlocked.

use super::super::view::AppView;
use crate::domain::crypto::KdfParams;
use crate::domain::data::history::PostRecord;
use crate::domain::post::lock::{seal_text, LockPassword, PostLock};
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::error::{AppError, Result};
use crate::state::PostEdit;
use crate::ui::features::composer::{publish_error_message, version_label, ComposerSubmission};
use crate::ui::features::data::{tr, UnlockEvent};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use jiff::Timestamp;
use std::time::Instant;
use zeroize::Zeroizing;

impl AppView {
    /// The composer on the locked post `id`'s unlocked text, or the unlock sheet first.
    pub(super) fn open_locked_editor(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let (Some(text), Some(post)) = (self.data.locks.unlocked.text(id).map(str::to_string), self.state.get_post(id)) else {
            return self.on_unlock_event(UnlockEvent::Open(id.clone()), window, cx);
        };
        let mut target = self.state.authored(post);
        target.post.content = text.clone();
        let Some(mut base) = self.state.edit_base(id, None) else { return };
        base.text = text;
        self.hand_completion_sources(cx); // D11
        self.composer_modal.update(cx, |panel, cx| {
            panel.open_edit_version(target, base, window, cx);
            panel.require_lock(window, cx);
        });
    }

    /// Seals the edited text under `password` in the background, then saves the version.
    pub(super) fn save_locked_edit(&mut self, id: PostId, submission: &ComposerSubmission, password: LockPassword, window: &mut Window, cx: &mut Context<Self>) {
        let Some(previous) = self.data.locks.unlocked.text(&id).map(|text| Zeroizing::new(text.to_string())) else {
            return self.locked_edit_failed(&AppError::PostLocked, cx);
        };
        let text = submission.drafts.first().cloned().unwrap_or_default();
        let look = PostRecord { symbols: submission.symbols.clone(), tint: submission.tint, ..PostRecord::default() };
        let edit = PostEdit { look: Some(look), ..submission.edit.clone().unwrap_or_else(|| PostEdit::text(text)) };
        let content = Zeroizing::new(edit.content.clone());
        cx.spawn_in(window, async move |view, cx| {
            let sealed = cx.background_executor().spawn(async move { seal_text(&content, &password, KdfParams::DEFAULT) }).await;
            let _ = view.update_in(cx, |this, window, cx| this.store_locked_edit(id, edit, &previous, sealed, window, cx));
        })
        .detach();
    }

    fn store_locked_edit(&mut self, id: PostId, edit: PostEdit, previous: &str, sealed: Result<PostLock>, window: &mut Window, cx: &mut Context<Self>) {
        let saved = sealed.and_then(|lock| self.state.edit_locked_post(&id, &edit, lock, previous, Timestamp::now()));
        self.composer_modal.update(cx, |panel, cx| panel.finish_sealing(cx));
        if let Err(err) = saved {
            return self.locked_edit_failed(&err, cx);
        }
        *self.data.revision_counts.entry(id.as_str().to_string()).or_default() += 1;
        self.data.locks.unlocked.insert(id.clone(), Zeroizing::new(edit.content), Instant::now());
        self.feed.stream.remeasure();
        self.close_composer_modal(window, cx);
        let version = self.state.get_post(&id).and_then(|post| post.extras.head.as_ref()).map(|head| version_label(head.version));
        let text = version.map_or_else(|| tr("data.edit.saved"), |version| rust_i18n::t!("data.edit.saved_version", version = version).to_string());
        self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
    }

    fn locked_edit_failed(&mut self, err: &AppError, cx: &mut Context<Self>) {
        let message = publish_error_message(err);
        self.composer_modal.update(cx, |panel, cx| {
            panel.finish_sealing(cx);
            panel.set_error(message, cx);
        });
    }
}
