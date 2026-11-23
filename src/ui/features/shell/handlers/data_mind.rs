//! N19: "Changed my mind" in the shell. The sheet opens on the user's post (never a locked
//! one) with its old view and the post it revises, if it has them; the search lists the
//! user's earlier posts by their words (never a locked one); saving marks the post.

use super::super::view::AppView;
use crate::domain::views::rapid_log::one_line;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, MindDialog, MindEvent};
use gpui_kit::*;

/// The most earlier posts the search lists.
const CANDIDATES: usize = 6;

impl AppView {
    /// Routes one "Changed my mind" event.
    pub(in crate::ui::features::shell) fn on_mind_event(&mut self, event: MindEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            MindEvent::Open(id) => self.open_mind(id, window, cx),
            MindEvent::Unmark(id) => {
                let text = self.state.unmark_changed_mind(&id).map_or_else(|err| publish_error_message(&err), |()| tr("posts.mind.unmarked"));
                self.notify_data(text, window, cx);
            }
            MindEvent::Pick(target) => {
                if let Some(dialog) = self.data.ideas.thoughts.mind.as_mut() {
                    dialog.chosen = if dialog.chosen.as_ref() == Some(&target) { None } else { Some(target) };
                }
            }
            MindEvent::Save => self.save_mind(window, cx),
            MindEvent::Close => self.close_mind(window, cx),
        }
    }

    /// The user's earlier posts whose words match `find` (the newest first), never `id` nor a locked one.
    fn mind_candidates(&self, id: &PostId, find: &str) -> Vec<(PostId, String)> {
        let find = find.trim().to_lowercase();
        let own = |post: &&crate::domain::Post| self.state.author_of(post).is_human_persona() && &post.id != id && !post.is_locked();
        let matches = |post: &&crate::domain::Post| find.is_empty() || post.content.to_lowercase().contains(&find);
        self.state.current_network_posts().filter(own).filter(matches).take(CANDIDATES).map(|post| (post.id.clone(), one_line(post))).collect()
    }

    /// The sheet's search follows what is typed.
    pub(in crate::ui::features::shell) fn refresh_mind_candidates(&mut self, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.mind.as_ref() else { return };
        let candidates = self.mind_candidates(&dialog.post_id, dialog.find.read(cx).value().as_ref());
        if let Some(dialog) = self.data.ideas.thoughts.mind.as_mut() {
            dialog.candidates = candidates;
        }
    }

    fn open_mind(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&id).filter(|post| !post.is_locked()) else { return }; // X2
        let (was, chosen) = post.extras.mind_change.as_ref().map(|change| (change.was.clone(), change.revises.clone())).unwrap_or_default();
        let inputs = &self.data.inputs.ideas.thoughts;
        let (was_field, find_field) = (inputs.mind_was.clone(), inputs.mind_find.clone());
        was_field.update(cx, |field, cx| field.set_value(was, window, cx));
        find_field.update(cx, |field, cx| field.set_value("", window, cx));
        let candidates = self.mind_candidates(&id, "");
        self.data.ideas.thoughts.mind = Some(MindDialog { post_id: id, was: was_field.clone(), find: find_field, candidates, chosen, problem: None });
        was_field.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_mind(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let inputs = &self.data.inputs.ideas.thoughts;
        for field in [inputs.mind_was.clone(), inputs.mind_find.clone()] {
            field.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.thoughts.mind = None;
    }

    fn save_mind(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.mind.as_ref() else { return };
        let (id, chosen, was) = (dialog.post_id.clone(), dialog.chosen.clone(), dialog.was.read(cx).value().to_string());
        match self.state.mark_changed_mind(&id, chosen.as_ref(), &was, jiff::Timestamp::now()) {
            Ok(_) => {
                self.close_mind(window, cx);
                self.notify_data(tr("posts.mind.marked"), window, cx);
            }
            Err(err) => {
                let problem = if matches!(err, crate::error::AppError::EmptyContent) { tr("posts.mind.need_one") } else { publish_error_message(&err) };
                if let Some(dialog) = self.data.ideas.thoughts.mind.as_mut() {
                    dialog.problem = Some(problem);
                }
            }
        }
    }
}
