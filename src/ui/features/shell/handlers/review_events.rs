//! What the Review page asks for (sb C1): answers applied to the deck and saved, sources
//! opened, cards suspended or deleted, and the page's data with a label for each source.

use super::super::review_host::learning_policy;
use super::super::view::AppView;
use crate::domain::review::{apply_answer, Answer, Card, CardId, CardSource};
use crate::domain::PostId;
use crate::state::semantic::hits::shortened;
use crate::ui::features::review::{ReviewData, ReviewEvent, ReviewPanel};
use chrono::{NaiveDate, Utc};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

/// Characters of a post shown as a card's source.
const SOURCE_CHARS: usize = 64;

impl AppView {
    pub(in crate::ui::features::shell) fn handle_review_event(&mut self, _: &Entity<ReviewPanel>, event: &ReviewEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            ReviewEvent::Answered(answer) => self.apply_review_answer(answer, cx),
            ReviewEvent::OpenSource(source) => self.open_card_source(source, window, cx),
            ReviewEvent::RequestQuiz(kind) => self.start_review_quiz(kind, window, cx),
            ReviewEvent::NewCard => self.open_blank_card_maker(window, cx),
            ReviewEvent::SetSuspended(id, suspended) => self.update_review_card(&id, |card| Card { suspended, ..card }, cx),
            ReviewEvent::DeleteCard(id) => {
                self.review.deck = std::mem::take(&mut self.review.deck).without_card(&id);
                if let Some(db) = &self.state.db {
                    let _ = db.delete_review_card(&id);
                }
                self.review_changed(cx);
            }
        }
    }

    /// The page's data: the active network's cards and log, today, Settings › Learning.
    pub(in crate::ui::features::shell) fn review_data(&self, today: NaiveDate) -> ReviewData {
        let deck = self.review.deck.of_network(&self.state.active_network_id);
        let (limits, policy) = learning_policy(&self.state.preferences().learning);
        let source_labels = deck.cards.iter().filter_map(|card| self.source_label(&card.source).map(|label| (card.source.group_key(), label))).collect::<HashMap<_, _>>();
        ReviewData { network_id: self.state.active_network_id.clone(), cards: deck.cards, log: deck.log, today, limits, policy, source_labels }
    }

    /// A post's opening words, or a document's title with the cited paragraph.
    fn source_label(&self, source: &CardSource) -> Option<String> {
        match source {
            CardSource::Post { id } => Some(self.state.get_post_by_id(id).map_or_else(|| t!("review.source.missing").to_string(), |post| shortened(&post.content, SOURCE_CHARS))),
            CardSource::Document { id, locator } => {
                let title = self.state.documents.iter().find(|doc| &doc.id == id).map(|doc| doc.title.clone()).unwrap_or_else(|| t!("review.source.missing").to_string());
                Some(match locator.as_ref().and_then(|l| l.paragraph) {
                    Some(paragraph) => t!("review.source.paragraph", title = title, paragraph = paragraph).to_string(),
                    None => title,
                })
            }
            CardSource::Sentence { .. } => None,
        }
    }

    /// Applies `answer` to its card, logs it and queues both for the vault.
    fn apply_review_answer(&mut self, answer: Answer, cx: &mut Context<Self>) {
        let Some(card) = self.review.deck.card(&answer.card_id).cloned() else { return };
        let (limits_policy, today) = (learning_policy(&self.state.preferences().learning), super::super::review_host::review_today());
        let (card, entry) = apply_answer(&card, &answer, Utc::now(), today, &limits_policy.1);
        if let Some(db) = &self.state.db {
            let _ = db.save_review_cards(vec![card.clone()]);
            let _ = db.append_review_entry(entry.clone());
        }
        self.review.deck = std::mem::take(&mut self.review.deck).with_card(card).with_entry(entry);
        self.review_changed(cx);
    }

    /// Replaces card `id` with `change(card)` and saves it.
    fn update_review_card(&mut self, id: &CardId, change: impl FnOnce(Card) -> Card, cx: &mut Context<Self>) {
        let Some(card) = self.review.deck.card(id).cloned().map(change) else { return };
        if let Some(db) = &self.state.db {
            let _ = db.save_review_cards(vec![card.clone()]);
        }
        self.review.deck = std::mem::take(&mut self.review.deck).with_card(card);
        self.review_changed(cx);
    }

    /// Opens the post or vault document a card cites.
    fn open_card_source(&mut self, source: CardSource, window: &mut Window, cx: &mut Context<Self>) {
        match source {
            CardSource::Post { id } => self.reveal_post(PostId::new(id), window, cx),
            CardSource::Document { id, .. } => self.open_vault_document(&id, window, cx),
            CardSource::Sentence { .. } => {}
        }
    }
}
