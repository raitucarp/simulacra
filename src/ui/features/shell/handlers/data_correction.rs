//! N14: retractions and corrections in the shell. Marking a post asks for the reason; taking
//! the mark back needs none. A locked post is never marked.

use super::super::view::AppView;
use crate::domain::post::correction::CorrectionKind;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, CorrectionDialog, CorrectionEvent};
use gpui_kit::*;

impl AppView {
    /// Routes one retraction or correction event.
    pub(in crate::ui::features::shell) fn on_correction_event(&mut self, event: CorrectionEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            CorrectionEvent::Open(id, kind) => self.open_correction(id, kind, window, cx),
            CorrectionEvent::Withdraw(id) => match self.state.withdraw_correction(&id) {
                Ok(()) => self.notify_data(tr("posts.correction.withdrawn"), window, cx),
                Err(err) => self.notify_data(publish_error_message(&err), window, cx),
            },
            CorrectionEvent::Submit => self.submit_correction(window, cx),
            CorrectionEvent::Close => self.close_correction(window, cx),
        }
    }

    fn open_correction(&mut self, id: PostId, kind: CorrectionKind, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.get_post(&id).is_none_or(|post| post.is_locked()) {
            return;
        }
        let reason = self.data.inputs.ideas.correction_reason.clone();
        reason.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.correction = Some(CorrectionDialog { post_id: id, kind, reason: reason.clone(), problem: None });
        reason.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_correction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.inputs.ideas.correction_reason.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.correction = None;
    }

    fn submit_correction(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.correction.as_ref() else { return };
        let (id, kind, reason) = (dialog.post_id.clone(), dialog.kind, dialog.reason.read(cx).value().to_string());
        match self.state.mark_correction(&id, kind, &reason, jiff::Timestamp::now()) {
            Ok(_) => {
                self.close_correction(window, cx);
                self.notify_data(tr(&format!("posts.correction.marked_{}", kind.key())), window, cx);
            }
            Err(err) => {
                let problem = if matches!(err, crate::error::AppError::EmptyContent) { tr("posts.correction.reason_first") } else { publish_error_message(&err) };
                if let Some(dialog) = self.data.ideas.correction.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }
}
