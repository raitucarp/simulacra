//! N6: the branch sheet in the shell: opened from a post's menu, it forks the post into a
//! what-if branch with its first post. A locked post never opens it.

use super::super::view::AppView;
use crate::domain::post::branch::BranchPoint;
use crate::domain::PostId;
use crate::ui::features::data::{tr, BranchDialog, BranchEvent};
use gpui_kit::*;

impl AppView {
    /// Routes one branch sheet event.
    pub(in crate::ui::features::shell) fn on_branch_event(&mut self, event: BranchEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            BranchEvent::Fork(id) => self.open_branch(id, window, cx),
            BranchEvent::Submit => self.submit_branch(window, cx),
            BranchEvent::Close => self.close_branch(window, cx),
        }
    }

    fn open_branch(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.get_post(&id).is_none_or(|post| post.is_locked()) {
            return;
        }
        let (label, text) = (self.data.inputs.branch_label.clone(), self.data.inputs.branch_text.clone());
        for input in [&label, &text] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.branch = Some(BranchDialog { post_id: id, label: label.clone(), text, problem: None });
        label.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_branch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for input in [&self.data.inputs.branch_label, &self.data.inputs.branch_text] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.branch = None;
    }

    /// Publishes the branch's first post, or says why it cannot go yet.
    fn submit_branch(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.branch.as_ref() else { return };
        let (post, label, text) = (dialog.post_id.clone(), dialog.label.read(cx).value().to_string(), dialog.text.read(cx).value().trim().to_string());
        let outcome = match BranchPoint::named(&label) {
            None => Err(tr("posts.branch.name_first")),
            Some(_) if text.is_empty() => Err(tr("posts.branch.write_first")),
            Some(_) => self.state.fork_branch(&post, &label, &text).map_err(|_| tr("posts.branch.failed")),
        };
        match outcome {
            Ok(_) => {
                self.close_branch(window, cx);
                self.feed.selected_thread_post_id = Some(post);
                self.notify_data(tr("posts.branch.sent"), window, cx);
            }
            Err(problem) => {
                if let Some(dialog) = self.data.branch.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }
}
