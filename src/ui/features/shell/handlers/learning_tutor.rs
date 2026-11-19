//! Tutor sessions in the shell (sb E2): what a dialogue reads (a post, a whole thread, a
//! passage, a document, or the learner's posts on a typed concept), starting, answering,
//! closing, cancelling and deleting a session, and the review items made from its gaps.

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::learning::concept::mentions;
use crate::domain::learning::tutor::machine::SessionStart;
use crate::domain::learning::tutor::source::{cut, thread_material, MAX_MATERIAL_CHARS};
use crate::domain::learning::{tutor::gap_drafts, SessionId, TutorSession, TutorSource};
use crate::domain::review::item::{new_item, NewItemContext};
use crate::domain::review::ItemId;
use crate::domain::Post;
use crate::state::semantic::hits::shortened;
use crate::state::{build_conversation, ReplyLimits};
use crate::ui::features::review::learning::TutorStatus;
use jiff::Timestamp;
use gpui_kit::*;

/// Characters of a title made from a post.
const TITLE_CHARS: usize = 60;
/// The learner's posts quoted for a typed concept.
const CONCEPT_POSTS: usize = 6;
const POST_BREAK: &str = "\n\n";

impl AppView {
    fn post_line(&self, post: &Post) -> (String, String) {
        (self.state.author_of(post).name.clone(), post.content.clone())
    }

    /// The thread from its first post: that post, then each reply and the replies to it.
    fn thread_lines(&self, root: &Post) -> Vec<(String, String)> {
        let conversation = build_conversation(&self.state, root, ReplyLimits::UNLIMITED);
        let replies = conversation.branches.iter().flat_map(|branch| std::iter::once(branch.reply).chain(branch.nested.iter().copied()));
        std::iter::once(root).chain(replies).map(|post| self.post_line(post)).collect()
    }

    fn concept_material(&self, name: &str) -> String {
        let own = self.state.current_network_posts().filter(|post| self.state.author_of(post).is_human_persona() && mentions(&post.content, name));
        let texts: Vec<String> = own.take(CONCEPT_POSTS).map(|post| post.content.clone()).collect();
        cut(&format!("{name}{POST_BREAK}{}", texts.join(POST_BREAK)), MAX_MATERIAL_CHARS)
    }

    /// Title and material of a dialogue on `source` (none when the source is gone).
    fn tutor_start(&self, source: TutorSource) -> Option<SessionStart> {
        let document = |id: &str| self.state.current_documents().find(|doc| doc.id == id).cloned();
        let (title, material) = match &source {
            TutorSource::Post { id } => self.state.get_post_by_id(id).map(|post| (shortened(&post.content, TITLE_CHARS), cut(&post.content, MAX_MATERIAL_CHARS)))?,
            TutorSource::Thread { root_id } => self.state.get_post_by_id(root_id).map(|root| (shortened(&root.content, TITLE_CHARS), thread_material(&self.thread_lines(root))))?,
            TutorSource::Passage { document_id, text, .. } => (document(document_id).map_or_else(|| shortened(text, TITLE_CHARS), |doc| doc.title), cut(text, MAX_MATERIAL_CHARS)),
            TutorSource::Document { id } => document(id).map(|doc| (doc.title.clone(), cut(&doc.content, MAX_MATERIAL_CHARS)))?,
            TutorSource::Concept { name } => (name.clone(), self.concept_material(name)),
        };
        Some(SessionStart { network_id: self.state.active_network_id.clone(), source, title, material })
    }

    /// `source` with a thread resolved to its first post.
    fn rooted(&self, source: TutorSource) -> TutorSource {
        match source {
            TutorSource::Thread { root_id } => {
                let root = self.state.get_post_by_id(&root_id).map_or(root_id.clone(), |post| self.thread_root(post).id.as_str().to_string());
                TutorSource::Thread { root_id: root }
            }
            other => other,
        }
    }

    pub(super) fn start_tutor_session(&mut self, source: TutorSource, window: &mut Window, cx: &mut Context<Self>) {
        let Some(start) = self.tutor_start(self.rooted(source)) else { return };
        let session = TutorSession::start(SessionId::generate(), start, Timestamp::now());
        let id = session.id.clone();
        self.store_session(session, cx);
        let shown = id.clone();
        self.review.panel.update(cx, |panel, cx| panel.show_session(shown, cx));
        self.run_tutor(&id, window, cx);
    }

    fn session(&self, id: &SessionId) -> Option<TutorSession> {
        self.review.learning.records.session(id).cloned()
    }

    pub(super) fn answer_tutor(&mut self, id: &SessionId, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.session(id) else { return };
        self.store_session(session.with_answer(text, Timestamp::now()), cx);
        self.run_tutor(id, window, cx);
    }

    pub(super) fn close_tutor(&mut self, id: &SessionId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.session(id) else { return };
        self.review.learning.cancel_running();
        self.store_session(session.close(Timestamp::now()), cx);
        self.run_tutor(id, window, cx);
    }

    pub(super) fn cancel_tutor(&mut self, cx: &mut Context<Self>) {
        self.review.learning.cancel_running();
        self.review.learning.tutor = TutorStatus::Idle;
        self.review_changed(cx);
    }

    pub(super) fn delete_tutor_session(&mut self, id: &SessionId, cx: &mut Context<Self>) {
        if self.review.learning.tutor.is_running(id) {
            self.cancel_tutor(cx);
        }
        if let Some(db) = &self.state.db {
            let _ = db.delete_tutor_session(&id.0);
        }
        self.review.learning.records = std::mem::take(&mut self.review.learning.records).without_session(id);
        self.review_changed(cx);
    }

    /// Review items anchored to the session's source, one per chosen summary question.
    pub(super) fn create_gap_items(&mut self, id: &SessionId, chosen: &[usize], cx: &mut Context<Self>) {
        let Some(session) = self.session(id) else { return };
        let context = NewItemContext { network_id: session.network_id.clone(), created_at: Timestamp::now(), today: review_today() };
        let items: Vec<_> = gap_drafts(&session, chosen).iter().filter_map(|draft| new_item(draft, &context, ItemId::generate())).collect();
        if items.is_empty() {
            return;
        }
        let count = items.len();
        self.store_review_items(items, cx);
        self.store_session(session.with_items_created(count, Timestamp::now()), cx);
    }
}
