//! Auto-delete in the shell (Y7): setting when one of the user's own posts deletes itself
//! (asking first when others reply to or quote it, Y8), and forgetting what the shell kept
//! about the posts the heartbeat deleted.

use super::super::view::AppView;
use crate::domain::embedding::EmbeddingOwner;
use crate::domain::post::auto_delete::DeleteAfter;
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{format_time, tr};
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use jiff::Timestamp;
use rust_i18n::t;

impl AppView {
    /// Asks first when others reply to or quote post `id` (Y8), then sets it to delete itself
    /// `after` from now; keeping it (`None`) never asks.
    pub(in crate::ui::features::shell) fn request_auto_delete(&mut self, id: PostId, after: Option<DeleteAfter>, window: &mut Window, cx: &mut Context<Self>) {
        if after.is_none() || !self.state.answered_by_others(&id) {
            return self.set_auto_delete(&id, after, window, cx);
        }
        let view = cx.entity().downgrade();
        window.open_alert_dialog(cx, move |alert, _, _| {
            let (view, id) = (view.clone(), id.clone());
            alert
                .confirm()
                .title(tr("data.auto_delete.warn_title"))
                .description(tr("data.auto_delete.warn_desc"))
                .ok_text(tr("data.auto_delete.warn_ok"))
                .ok_variant(ButtonVariant::Danger)
                .cancel_text(t!("composer.cancel").to_string())
                .on_ok(move |_, window, cx| {
                    let _ = view.update(cx, |this, cx| this.set_auto_delete(&id, after, window, cx));
                    true
                })
        });
    }

    /// Sets post `id` to delete itself `after` from now (`None` keeps it).
    pub(in crate::ui::features::shell) fn set_auto_delete(&mut self, id: &PostId, after: Option<DeleteAfter>, window: &mut Window, cx: &mut Context<Self>) {
        let at = after.map(|after| after.delete_at(Timestamp::now()));
        match self.state.set_auto_delete(id, at) {
            Ok(()) => {
                let text = at.map_or_else(|| tr("data.auto_delete.cleared"), |at| t!("data.auto_delete.set", when = format_time(at)).to_string());
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
    }

    /// Drops what the shell kept about the deleted posts `ids`: their vectors, edit counts and
    /// the open thread.
    pub(in crate::ui::features::shell) fn forget_deleted_posts(&mut self, ids: &[PostId]) {
        for id in ids {
            self.semantic.index = std::mem::take(&mut self.semantic.index).without(&EmbeddingOwner::post(id));
            self.data.revision_counts.remove(id.as_str());
            if self.feed.selected_thread_post_id.as_ref() == Some(id) {
                self.feed.selected_thread_post_id = None;
            }
        }
    }
}
