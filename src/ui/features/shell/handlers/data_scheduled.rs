//! The scheduled queue in the shell (Y4): the sheet of waiting posts, and its Edit (the
//! composer on the post, its moment in the picker), Publish now and Cancel.

use super::super::view::AppView;
use crate::domain::post::Tint;
use crate::domain::preferences::ToastKind;
use crate::domain::PostId;
use crate::state::store::ScheduledEdit;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, DataEvent, ScheduledRow};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use jiff::Timestamp;

impl AppView {
    pub(in crate::ui::features::shell) fn on_scheduled_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        let done = match event {
            DataEvent::OpenScheduled => {
                self.chrome.is_settings_open = false;
                self.data.scheduled_open = true;
                return;
            }
            DataEvent::EditScheduled(id) => return self.open_scheduled_editor(&id, window, cx),
            DataEvent::PublishScheduledNow(id) => self.state.publish_scheduled_now(&id, Timestamp::now()).map(|ids| crate::i18n::t_count("post.scheduled.published", ids.len())),
            DataEvent::CancelScheduled(id) => self.state.cancel_scheduled(&id).map(|_| tr("data.scheduled.cancelled")),
            _ => {
                self.data.scheduled_open = false;
                return;
            }
        };
        match done {
            Ok(text) => self.toast(ToastKind::Confirmation, Notification::success(text), window, cx),
            Err(err) => window.push_notification(Notification::error(publish_error_message(&err)), cx),
        }
    }

    /// The waiting posts as the sheet lists them, soonest first.
    pub(in crate::ui::features::shell) fn scheduled_rows(&self) -> Vec<ScheduledRow> {
        let row = |post: &crate::domain::Post| ScheduledRow { id: post.id.clone(), text: post.content.clone(), publish_at: post.lifecycle.publish_at };
        self.state.scheduled_posts().iter().map(row).collect()
    }

    /// Closes the sheet and opens the composer on the waiting post `id`.
    fn open_scheduled_editor(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.scheduled_post(id) else { return };
        let target = self.state.authored(post);
        self.data.scheduled_open = false;
        self.composer_modal.update(cx, |panel, cx| panel.open_edit(target, window, cx));
    }

    /// Saves the composer's edit of the waiting post `id`: its text, tint and moment (none
    /// publishes its thread at once).
    pub(crate) fn save_scheduled_edit(&mut self, id: PostId, (content, tint): (String, Option<Tint>), publish_at: Option<Timestamp>, window: &mut Window, cx: &mut Context<Self>) {
        match self.state.edit_scheduled(&id, ScheduledEdit { content, tint, publish_at }, Timestamp::now()) {
            Ok(()) => {
                self.close_composer_modal(window, cx);
                let text = if publish_at.is_some() { tr("data.scheduled.saved") } else { crate::i18n::t_count("post.scheduled.published", 1) };
                self.toast(ToastKind::Confirmation, Notification::success(text), window, cx);
            }
            Err(err) => {
                let message = publish_error_message(&err);
                self.composer_modal.update(cx, |panel, cx| panel.set_error(message, cx));
            }
        }
    }
}
