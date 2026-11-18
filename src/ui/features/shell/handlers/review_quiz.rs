//! Preparing quizzes (sb C1): the weakest cards first, distractors ranked by tag and by
//! meaning (the embedding index), and ordering puzzles from threads and documents.

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::review::quiz::{build_choice, mix_seed, piece_text, prose_paragraphs, sentence_window, shuffled, thread_line, OrderPuzzle, QuizKind, SourceVectors, PIECE_CHARS};
use crate::domain::review::{Card, CardSource, Policy};
use crate::domain::PostId;
use crate::import::segment::{split_sentences, SentenceLocale};
use crate::ui::features::review::QuizContent;
use chrono::Utc;
use gpui_kit::*;
use rust_i18n::t;

/// Questions per quiz.
const QUIZ_LENGTH: usize = 10;
/// Sources tried for an ordering puzzle (each thread is walked through every post).
const MAX_ORDER_CANDIDATES: usize = 200;

/// The cards a quiz asks: learnt cards, least likely to be recalled first, then new ones.
fn quiz_cards(cards: &[Card], policy: &Policy, seed: u64) -> Vec<Card> {
    let today = review_today();
    let mut learnt: Vec<(f64, &Card)> = cards.iter().filter(|c| !c.suspended && !c.schedule.is_new()).map(|c| (c.schedule.retrievability(today, &policy.params).unwrap_or(1.0), c)).collect();
    learnt.sort_by(|a, b| a.0.total_cmp(&b.0));
    let fresh = cards.iter().filter(|c| !c.suspended && c.schedule.is_new());
    let picked: Vec<Card> = learnt.into_iter().map(|(_, c)| c).chain(fresh).take(QUIZ_LENGTH).cloned().collect();
    shuffled(picked, seed)
}

impl AppView {
    /// Builds the quiz of `kind` and starts it on the Review page.
    pub(in crate::ui::features::shell) fn start_review_quiz(&mut self, kind: QuizKind, window: &mut Window, cx: &mut Context<Self>) {
        let seed = Utc::now().timestamp_nanos_opt().unwrap_or_default() as u64;
        let content = match kind {
            QuizKind::Choice => self.choice_quiz(seed),
            QuizKind::Recall => self.recall_quiz(seed),
            QuizKind::Order => self.order_quiz(seed),
        };
        self.review.panel.update(cx, |panel, cx| panel.begin_quiz(content, window, cx));
    }

    fn network_cards(&self) -> Vec<Card> {
        self.review.deck.of_network(&self.state.active_network_id).cards
    }

    fn choice_quiz(&self, seed: u64) -> QuizContent {
        let pool = self.network_cards();
        let policy = super::super::review_host::learning_policy(&self.state.preferences().learning).1;
        let vectors = SourceVectors::from_index(&self.semantic.index, &self.embedding_config().model);
        let questions: Vec<_> = quiz_cards(&pool, &policy, seed)
            .iter()
            .enumerate()
            .filter_map(|(i, card)| build_choice(card, &pool, &|other: &Card| vectors.affinity(card, other), mix_seed(seed, i as u64)))
            .collect();
        if questions.is_empty() { QuizContent::Unavailable("review.quiz.choice.unavailable") } else { QuizContent::Choice(questions) }
    }

    fn recall_quiz(&self, seed: u64) -> QuizContent {
        let policy = super::super::review_host::learning_policy(&self.state.preferences().learning).1;
        let cards: Vec<_> = quiz_cards(&self.network_cards(), &policy, seed).into_iter().map(|card| card.id).collect();
        if cards.is_empty() { QuizContent::Unavailable("review.quiz.recall.unavailable") } else { QuizContent::Recall(cards) }
    }

    /// A thread or a passage: the cited ones first, then any of the network.
    fn order_quiz(&self, seed: u64) -> QuizContent {
        let cited = self.network_cards().into_iter().map(|card| card.source);
        let posts = self.state.current_network_posts().map(|post| CardSource::post(&post.id));
        let documents = self.state.current_documents().map(|doc| CardSource::document(doc.id.clone(), None));
        let candidates: Vec<CardSource> = cited.chain(shuffled(posts.chain(documents).collect(), seed)).take(MAX_ORDER_CANDIDATES).collect();
        let puzzle = candidates.iter().find_map(|source| self.order_puzzle(source, seed));
        puzzle.map_or(QuizContent::Unavailable("review.quiz.order.unavailable"), QuizContent::Order)
    }

    fn order_puzzle(&self, source: &CardSource, seed: u64) -> Option<OrderPuzzle> {
        let locale = SentenceLocale::from_language_tag(Some(self.state.preferences().general.language.code()));
        match source {
            CardSource::Post { id } => {
                let line = thread_line(self.state.posts(), &PostId::new(id.clone()));
                let pieces = line.iter().map(|post| piece_text(&post.content, PIECE_CHARS)).collect();
                OrderPuzzle::new(t!("review.order.thread_title").to_string(), pieces, seed)
            }
            CardSource::Document { id, .. } => {
                let doc = self.state.documents.iter().find(|doc| &doc.id == id)?;
                let paragraphs = prose_paragraphs(&doc.content);
                let sentences: Vec<String> = paragraphs.iter().flat_map(|p| split_sentences(p, locale)).map(|s| piece_text(s, PIECE_CHARS)).collect();
                let window = sentence_window(&sentences, (seed % sentences.len().max(1) as u64) as usize);
                OrderPuzzle::new(t!("review.order.passage_title", title = doc.title.as_str()).to_string(), window, seed)
            }
            CardSource::Sentence { .. } => None,
        }
    }
}
