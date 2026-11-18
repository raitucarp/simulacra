//! What the Learn tab asks for (sb E2): goals and palaces saved or deleted (thread links
//! resolved to the thread's first post), the importer, a prefilled composer, the picker on
//! related posts, image ideas for a locus cue; tutor requests go to `learning_tutor`.

use super::super::view::AppView;
use crate::domain::learning::{Goal, GoalId, GoalLink, Palace, PalaceId, PlacedTarget, TutorSession};
use crate::domain::post::{WIKILINK_CLOSE, WIKILINK_OPEN};
use crate::ui::features::composer::ComposerModalMode;
use crate::ui::features::review::learning::{LearningData, LearningEvent};
use crate::ui::features::review::PickerMode;
use gpui_kit::*;

/// Posts the Learn tab shows: the week's posts, placed posts, linked posts, tutor sources.
pub(super) fn learning_post_ids(data: &LearningData) -> Vec<String> {
    let records = &data.records;
    let placed = records.palaces.iter().flat_map(|p| &p.rooms).flat_map(|r| &r.loci).filter_map(|l| match l.placement.as_ref().map(|p| &p.target) {
        Some(PlacedTarget::Post { id }) => Some(id.clone()),
        _ => None,
    });
    let linked = records.goals.iter().flat_map(|g| &g.links).filter_map(|link| match link {
        GoalLink::Post { id } | GoalLink::Thread { root_id: id } => Some(id.clone()),
        _ => None,
    });
    let sources = records.sessions.iter().filter_map(|s| s.source.post_id().map(str::to_string));
    data.digest.posts.iter().cloned().chain(placed).chain(linked).chain(sources).collect()
}

impl AppView {
    pub(super) fn handle_learning_event(&mut self, event: LearningEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            LearningEvent::StartTutor(source) => self.start_tutor_session(source, window, cx),
            LearningEvent::AnswerTutor(id, text) => self.answer_tutor(&id, &text, window, cx),
            LearningEvent::CloseTutor(id) => self.close_tutor(&id, window, cx),
            LearningEvent::RetryTutor(id) => self.run_tutor(&id, window, cx),
            LearningEvent::CancelTutor => self.cancel_tutor(cx),
            LearningEvent::DeleteSession(id) => self.delete_tutor_session(&id, cx),
            LearningEvent::CreateGapItems(id, chosen) => self.create_gap_items(&id, &chosen, cx),
            LearningEvent::SaveGoal(goal) => self.store_goal(goal, cx),
            LearningEvent::DeleteGoal(id) => self.delete_goal(&id, cx),
            LearningEvent::SavePalace(palace) => self.store_palace(palace, cx),
            LearningEvent::DeletePalace(id) => self.delete_palace(&id, cx),
            LearningEvent::ImportSource(_) => self.pick_import_files(window, cx),
            LearningEvent::WritePost(concept) => self.write_about(&concept, window, cx),
            LearningEvent::LearnRelated(concept) => self.learn_related(&concept, window, cx),
            LearningEvent::LocusImage(palace, room, slot) => self.open_image_ideas_for_locus(palace, room, slot, window, cx),
        }
    }

    /// Saves `session` to the records and the vault.
    pub(in crate::ui::features::shell) fn store_session(&mut self, session: TutorSession, cx: &mut Context<Self>) {
        if let Some(db) = &self.state.db {
            let _ = db.save_tutor_session(&session);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).with_session(session);
        self.review_changed(cx);
    }

    fn store_goal(&mut self, goal: Goal, cx: &mut Context<Self>) {
        let root = |id: &str| self.state.get_post_by_id(id).map_or_else(|| id.to_string(), |post| self.thread_root(post).id.as_str().to_string());
        let links = goal.links.iter().map(|link| match link {
            GoalLink::Thread { root_id } => GoalLink::Thread { root_id: root(root_id) },
            other => other.clone(),
        });
        let links = links.fold(Vec::new(), |mut kept: Vec<GoalLink>, link| {
            if !kept.contains(&link) {
                kept.push(link);
            }
            kept
        });
        let goal = Goal { links, ..goal };
        if let Some(db) = &self.state.db {
            let _ = db.save_goal(&goal);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).with_goal(goal);
        self.review_changed(cx);
    }

    fn delete_goal(&mut self, id: &GoalId, cx: &mut Context<Self>) {
        if let Some(db) = &self.state.db {
            let _ = db.delete_goal(&id.0);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).without_goal(id);
        self.review_changed(cx);
    }

    pub(in crate::ui::features::shell) fn store_palace(&mut self, palace: Palace, cx: &mut Context<Self>) {
        if let Some(db) = &self.state.db {
            let _ = db.save_palace(&palace);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).with_palace(palace);
        self.review_changed(cx);
    }

    fn delete_palace(&mut self, id: &PalaceId, cx: &mut Context<Self>) {
        if let Some(db) = &self.state.db {
            let _ = db.delete_palace(&id.0);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).without_palace(id);
        self.review_changed(cx);
    }

    /// The composer, opened on a new post about `concept`.
    fn write_about(&mut self, concept: &str, window: &mut Window, cx: &mut Context<Self>) {
        let text = format!("{WIKILINK_OPEN}{concept}{WIKILINK_CLOSE} ");
        self.open_composer_modal(ComposerModalMode::NewThread, window, cx);
        self.composer_modal.update(cx, |panel, cx| panel.prefill(&text, window, cx));
    }

    /// The post picker filtered by `concept`, to learn the posts about it.
    fn learn_related(&mut self, concept: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.open_review_picker(PickerMode::Learn, window, cx);
        self.review.panel.update(cx, |panel, cx| panel.filter_picker(concept, window, cx));
    }
}
