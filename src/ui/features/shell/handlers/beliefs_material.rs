//! What each belief update reads (sb F1): a reply with the post it answers, a debate's
//! transcript, an agent's own experiment answers, or a thread the user hands it. Only these
//! network records ever become material; the source of each is kept for the revision log.

use super::super::view::AppView;
use crate::domain::arena::debate::Debate;
use crate::domain::arena::lab::{cells, conditions, Experiment};
use crate::domain::beliefs::extract::Material;
use crate::domain::beliefs::{BeliefOrigin, BeliefSource};
use crate::domain::{Post, ProfileId};
use gpui_kit::*;

/// Characters of a post kept in a source label.
const LABEL_CHARS: usize = 60;
const ELLIPSIS: char = '…';

/// The start of `text` as a short label.
fn label_of(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= LABEL_CHARS {
        return flat;
    }
    flat.chars().take(LABEL_CHARS).chain(std::iter::once(ELLIPSIS)).collect()
}

impl AppView {
    /// After `reply` was generated: its author read the parent post and answered it.
    pub(in crate::ui::features::shell) fn update_beliefs_from_reply(&mut self, reply: &Post, cx: &mut Context<Self>) {
        if !self.state.llm_settings.has_api_key() {
            return;
        }
        let parent = reply.reply_to_id.as_ref().and_then(|id| self.state.get_post(id)).cloned();
        let mut passages: Vec<(String, String)> = parent.iter().map(|post| (self.state.author_of(post).name.clone(), post.content.clone())).collect();
        passages.push((self.name_of(&reply.author_id), reply.content.clone()));
        let (origin, label) = match &parent {
            Some(post) => (BeliefOrigin::Reply, label_of(&post.content)),
            None => (BeliefOrigin::Post, label_of(&reply.content)),
        };
        let material = Material { kind: "a reply they wrote to a post they read".into(), title: label.clone(), passages };
        self.queue_belief_update(&reply.author_id.clone(), BeliefSource::new(origin, reply.id.to_string(), label), material, cx);
    }

    /// After `debate` was judged: `agent` took part in it.
    pub(super) fn update_beliefs_from_debate(&mut self, agent: &ProfileId, debate: &Debate, cx: &mut Context<Self>) {
        let passages = debate.turns.iter().map(|turn| (self.name_of(&turn.slot.speaker), turn.text.clone())).collect();
        let title = label_of(&debate.setup.motion);
        let material = Material { kind: "a debate they took part in".into(), title: debate.setup.motion.clone(), passages };
        self.queue_belief_update(agent, BeliefSource::new(BeliefOrigin::Debate, debate.id.to_string(), title), material, cx);
    }

    /// After `experiment` was complete: each agent reads back its own answers.
    pub(super) fn update_beliefs_from_experiment(&mut self, experiment: &Experiment, cx: &mut Context<Self>) {
        let grid = cells(&experiment.scenario);
        for agent in &experiment.agents {
            let passages = experiment.answers.iter().filter(|answer| &answer.agent == agent).filter_map(|answer| {
                let case: Vec<String> = conditions(&experiment.scenario, grid.get(answer.cell)?).into_iter().map(|(name, value)| format!("{name}: {value}")).collect();
                Some((case.join("; "), answer.as_line()))
            });
            let mut passages: Vec<(String, String)> = passages.collect();
            passages.insert(0, ("Scenario".into(), format!("{}\n{}", experiment.scenario.premise, experiment.scenario.question)));
            let title = label_of(&experiment.scenario.title);
            let material = Material { kind: "a thought experiment they answered".into(), title: experiment.scenario.title.clone(), passages };
            self.queue_belief_update(agent, BeliefSource::new(BeliefOrigin::Experiment, experiment.id.to_string(), title), material, cx);
        }
    }

    /// "Read the open thread": `agent` reads the thread open in the inspector.
    pub(super) fn read_open_thread(&mut self, agent: &ProfileId, cx: &mut Context<Self>) {
        let Some(root) = self.feed.selected_thread_post_id.clone().and_then(|id| self.root_of(&id)) else { return };
        let passages: Vec<(String, String)> = self.thread_posts(&root).into_iter().map(|post| (post.author, post.text)).collect();
        let title = passages.first().map(|(_, text)| label_of(text)).unwrap_or_default();
        let material = Material { kind: "a thread they read".into(), title: title.clone(), passages };
        self.queue_belief_update(agent, BeliefSource::new(BeliefOrigin::Thread, root.to_string(), title), material, cx);
    }
}
