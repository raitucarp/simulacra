//! N9: asking the room from a post's menu. The question is routed to the agents that may read
//! it (N8) whose knowledge fits best: by embeddings (the question's vector against each
//! agent's interest), or by shared words while either is not embedded. Each agent asked
//! answers through the usual generation path (its engine, the queue and the spend rules); a
//! locked post is never asked. Accepting an answer pins it to the top of the answers.

use super::super::view::AppView;
use crate::agent::JobKind;
use crate::domain::embedding::{centroid, EmbeddingOwner};
use crate::domain::post::question::{route_by_meaning, route_by_words, Asked};
use crate::domain::spend::SpendEstimate;
use crate::domain::{Post, PostId, ProfileId};
use crate::state::recommend::Recommender;
use crate::state::store::agent_knowledge;
use crate::ui::features::data::{tr, QuestionEvent};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one question event.
    pub(in crate::ui::features::shell) fn on_question_event(&mut self, event: QuestionEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            QuestionEvent::Ask(id) => self.ask_the_room(&id, window, cx),
            QuestionEvent::Accept(question, answer) => {
                if self.state.accept_answer(&question, &answer).is_err() {
                    self.notify_data(tr("posts.question.failed"), window, cx);
                }
            }
        }
    }

    /// The agents to ask `post`, best fit first.
    fn route_question(&self, post: &Post) -> Vec<Asked> {
        let room = self.state.room_for(post);
        let model = self.embedding_config().model;
        let recommender = Recommender { state: &self.state, index: &self.semantic.index, model: &model, now: jiff::Timestamp::now() };
        let vectors: Vec<(ProfileId, Vec<f32>)> = room.iter().filter_map(|agent| Some((agent.id.clone(), recommender.interest(&agent.id, &post.network_id)?))).collect();
        match centroid(&self.semantic.index, &EmbeddingOwner::post(&post.id), &model) {
            Some(question) if !vectors.is_empty() => route_by_meaning(&question, &vectors),
            _ => route_by_words(&post.content, &room.iter().map(|agent| (agent.id.clone(), agent_knowledge(agent))).collect::<Vec<_>>()),
        }
    }

    /// Asks the user's post `id` of the room and starts each asked agent's answer.
    fn ask_the_room(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(id).filter(|post| !post.is_locked()) else { return };
        let (asked, parent, network) = (self.route_question(post), self.state.authored(post), post.network_id.clone());
        if asked.is_empty() {
            return self.notify_data(tr("posts.question.nobody"), window, cx);
        }
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::requests(asked.len() as u32), Box::new(move |this, window, cx| this.on_question_event(QuestionEvent::Ask(resume), window, cx)), cx) {
            return;
        }
        if self.state.ask_room(id, asked.clone()).is_err() {
            return self.notify_data(tr("posts.question.failed"), window, cx);
        }
        let agents: Vec<_> = asked.iter().filter_map(|a| self.state.user_profiles.iter().find(|p| p.id == a.agent).cloned()).collect();
        let names = agents.iter().map(|agent| agent.name.clone()).collect::<Vec<_>>().join(", ");
        for agent in agents {
            if !self.engine_allowed(&agent) || !self.scheduler.queue.start_now(&agent.id) {
                continue;
            }
            self.spawn_generation(agent, JobKind::Reply(parent.clone()), network.clone(), window, cx);
        }
        self.feed.selected_thread_post_id = Some(id.clone());
        self.notify_data(t!("posts.question.asked", names = names).to_string(), window, cx);
    }
}
