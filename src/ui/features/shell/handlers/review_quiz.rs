//! Preparing quizzes (sb C1, on posts and passages since sb C1b): the weakest items first,
//! distractors ranked by tag and by meaning (the embedding index), answers read from the
//! posts' current text, and ordering puzzles from the threads of learnt posts first.

use super::super::review_host::{learning_policy, review_today};
use super::super::view::AppView;
use crate::domain::post::readable_line;
use crate::domain::review::quiz::{build_choice, mix_seed, piece_text, prose_paragraphs, sentence_window, shuffled, thread_line, ChoiceSources, OrderPuzzle, QuizKind, SourceVectors, PIECE_CHARS};
use crate::domain::review::{Anchor, Policy, ReviewItem};
use crate::domain::PostId;
use crate::import::segment::{split_sentences, SentenceLocale};
use crate::ui::features::review::QuizContent;
use jiff::Timestamp;
use gpui_kit::*;
use rust_i18n::t;

/// Questions per quiz.
const QUIZ_LENGTH: usize = 10;
/// Sources tried for an ordering puzzle (each thread is walked through every post).
const MAX_ORDER_CANDIDATES: usize = 200;

/// The items a quiz asks: anchored ones, learnt first (least likely recalled first), then new.
fn quiz_items(items: &[ReviewItem], policy: &Policy, seed: u64) -> Vec<ReviewItem> {
    let today = review_today();
    let asked = |item: &&ReviewItem| item.is_active() && item.subject.anchor().is_some();
    let mut learnt: Vec<(f64, &ReviewItem)> = items.iter().filter(asked).filter(|i| !i.schedule.is_new()).map(|i| (i.schedule.retrievability(today, &policy.params).unwrap_or(1.0), i)).collect();
    learnt.sort_by(|a, b| a.0.total_cmp(&b.0));
    let fresh = items.iter().filter(asked).filter(|i| i.schedule.is_new());
    let picked: Vec<ReviewItem> = learnt.into_iter().map(|(_, i)| i).chain(fresh).take(QUIZ_LENGTH).cloned().collect();
    shuffled(picked, seed)
}

impl AppView {
    /// Builds the quiz of `kind` and starts it on the Review page.
    pub(in crate::ui::features::shell) fn start_review_quiz(&mut self, kind: QuizKind, window: &mut Window, cx: &mut Context<Self>) {
        let seed = Timestamp::now().as_nanosecond() as u64;
        let content = match kind {
            QuizKind::Choice => self.choice_quiz(seed),
            QuizKind::Recall => self.recall_quiz(seed),
            QuizKind::Order => self.order_quiz(seed),
        };
        self.review.panel.update(cx, |panel, cx| panel.begin_quiz(content, window, cx));
    }

    fn network_items(&self) -> Vec<ReviewItem> {
        self.review.deck.of_network(&self.state.active_network_id).items
    }

    /// An item's answer, its post read as it reads now, as plain text (shortened for an option).
    fn quiz_answer(&self, item: &ReviewItem) -> String {
        piece_text(&item.answer_text(&|id| self.state.get_post_by_id(id).map(|post| readable_line(&post.content))), PIECE_CHARS)
    }

    fn choice_quiz(&self, seed: u64) -> QuizContent {
        let pool = self.network_items();
        let policy = learning_policy(&self.state.preferences().learning).1;
        let vectors = SourceVectors::from_index(&self.semantic.index, &self.embedding_config().model);
        let answer_of = |item: &ReviewItem| self.quiz_answer(item);
        let questions: Vec<_> = quiz_items(&pool, &policy, seed)
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let affinity = |other: &ReviewItem| vectors.affinity(item, other);
                build_choice(item, &pool, &ChoiceSources { answer_of: &answer_of, affinity: &affinity }, mix_seed(seed, i as u64))
            })
            .collect();
        if questions.is_empty() { QuizContent::Unavailable("review.quiz.choice.unavailable") } else { QuizContent::Choice(questions) }
    }

    fn recall_quiz(&self, seed: u64) -> QuizContent {
        let policy = learning_policy(&self.state.preferences().learning).1;
        let items: Vec<_> = quiz_items(&self.network_items(), &policy, seed).into_iter().map(|item| item.id).collect();
        if items.is_empty() { QuizContent::Unavailable("review.quiz.recall.unavailable") } else { QuizContent::Recall(items) }
    }

    /// A thread or a passage: the learnt ones first, then any of the network.
    fn order_quiz(&self, seed: u64) -> QuizContent {
        let learnt = self.network_items().into_iter().filter_map(|item| item.subject.anchor().cloned());
        let posts = self.state.current_network_posts().map(|post| Anchor::post(&post.id));
        let documents = self.state.current_documents().map(|doc| Anchor::passage(doc.id.clone(), None, String::new()));
        let candidates: Vec<Anchor> = learnt.chain(shuffled(posts.chain(documents).collect(), seed)).take(MAX_ORDER_CANDIDATES).collect();
        let puzzle = candidates.iter().find_map(|anchor| self.order_puzzle(anchor, seed));
        puzzle.map_or(QuizContent::Unavailable("review.quiz.order.unavailable"), QuizContent::Order)
    }

    fn order_puzzle(&self, anchor: &Anchor, seed: u64) -> Option<OrderPuzzle> {
        let locale = SentenceLocale::from_language_tag(Some(self.state.preferences().general.language.code()));
        match anchor {
            Anchor::Post { id } => {
                let line = thread_line(self.state.posts(), &PostId::new(id.clone()));
                let pieces = line.iter().map(|post| piece_text(&readable_line(&post.content), PIECE_CHARS)).collect();
                OrderPuzzle::new(t!("review.order.thread_title").to_string(), pieces, seed)
            }
            Anchor::Passage { document_id, .. } => {
                let doc = self.state.documents.iter().find(|doc| &doc.id == document_id)?;
                let paragraphs = prose_paragraphs(&doc.content);
                let sentences: Vec<String> = paragraphs.iter().flat_map(|p| split_sentences(p, locale)).map(|s| piece_text(s, PIECE_CHARS)).collect();
                let window = sentence_window(&sentences, (seed % sentences.len().max(1) as u64) as usize);
                OrderPuzzle::new(t!("review.order.passage_title", title = doc.title.as_str()).to_string(), window, seed)
            }
        }
    }
}
