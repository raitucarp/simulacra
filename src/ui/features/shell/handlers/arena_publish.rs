//! Publishing from the Arena (sb F1): a judged debate becomes a real thread in the active
//! network (the motion by the user's persona, one post per turn by its speaker, the verdict
//! by an agent judge), and a finished experiment becomes a summary post by the user's
//! persona. Both go through one batch write; the feed then shows the thread.

use super::super::view::AppView;
use crate::domain::arena::debate::{Debate, Judge};
use crate::domain::arena::lab::summary::{summary_text, SummaryWords};
use crate::domain::arena::lab::{Experiment, Stance};
use crate::domain::arena::publish::{build_thread, debate_parts, split_parts, ThreadPart};
use crate::domain::arena::ArenaId;
use crate::domain::PostId;
use crate::import::plan::effective_post_limit;
use crate::state::AppState;
use crate::ui::features::arena::text::{format_label, stance_label, tr};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Splits `parts` to the post limit, publishes them as one thread and opens it.
    fn publish_parts(&mut self, parts: &[ThreadPart], window: &mut Window, cx: &mut Context<Self>) -> Option<PostId> {
        let limit = effective_post_limit(self.state.preferences().writing.post_length_limit);
        let posts = build_thread(&split_parts(parts, limit), &self.state.active_network_id, chrono::Utc::now(), AppState::fresh_post_id).ok()?;
        let root = posts.first()?.id.clone();
        if let Err(err) = self.state.publish_batch(posts) {
            self.report_storage_failure(&err, window, cx);
            return None;
        }
        self.reveal_post(root.clone(), window, cx);
        Some(root)
    }

    /// The opening post of `debate`'s thread.
    fn motion_post(debate: &Debate) -> String {
        let format = format_label(debate.setup.format);
        t!("arena.publish.motion", motion = debate.setup.motion.trim(), format = format).to_string()
    }

    /// The verdict post, when an agent judged.
    fn verdict_part(&self, debate: &Debate) -> Option<ThreadPart> {
        let verdict = debate.verdict.as_ref()?;
        let Judge::Agent(judge) = &verdict.judged_by else { return None };
        let text = t!("arena.publish.verdict", summary = verdict.summary.trim()).to_string();
        Some(ThreadPart { author: judge.clone(), text })
    }

    pub(super) fn publish_debate(&mut self, id: &ArenaId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(debate) = self.arena.debates.iter().find(|debate| &debate.id == id).cloned() else { return };
        let author = self.state.active_profile_id.clone();
        let root = ThreadPart { author, text: Self::motion_post(&debate) };
        let parts = debate_parts(&debate, root, self.verdict_part(&debate));
        if let Some(root) = self.publish_parts(&parts, window, cx) {
            self.save_debate(debate.with_published(root, chrono::Utc::now()));
            self.arena_changed(cx);
        }
    }

    /// The summary's words in the user's language.
    fn summary_words(experiment: &Experiment) -> SummaryWords {
        SummaryWords {
            heading: t!("arena.publish.experiment", title = experiment.scenario.title.trim()).to_string(),
            stances: Stance::ALL.iter().map(|stance| stance_label(*stance)).collect(),
            turns_on: tr("arena.publish.turns_on"),
            steady: tr("arena.publish.steady"),
        }
    }

    pub(super) fn publish_experiment(&mut self, id: &ArenaId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(experiment) = self.arena.experiments.iter().find(|kept| &kept.id == id).cloned() else { return };
        let text = summary_text(&experiment, &Self::summary_words(&experiment), &|agent| self.name_of(agent));
        let part = ThreadPart { author: self.state.active_profile_id.clone(), text };
        self.publish_parts(&[part], window, cx);
    }
}
