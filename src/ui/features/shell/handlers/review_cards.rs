//! Making flashcards (sb C1): the card maker opened on a post (post "…" menu, palette), a
//! vault document (its sentences, each citing its paragraph) or a blank note; saving its
//! cards; and the AI draft through the configured LLM client.

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::post::SourceLocator;
use crate::domain::review::card::{expand_draft, NewCardContext};
use crate::domain::review::quiz::prose_paragraphs;
use crate::domain::review::{CardDraft, CardId, CardSource};
use crate::domain::PostId;
use crate::i18n::t_count;
use crate::import::segment::{split_sentences, SentenceLocale};
use crate::ui::features::review::draft_request::request_drafts_blocking;
use crate::ui::features::review::{FlashcardMaker, MakerEvent, MakerRequest, MakerSentence};
use crate::agent::LlmClient;
use crate::domain::preferences::ToastKind;
use chrono::Utc;
use gpui_kit::component::notification::Notification;
use gpui_kit::*;

/// Most sentences offered from one document.
const MAX_SENTENCES: usize = 400;
/// Characters of a post shown in the maker's header.
const TITLE_CHARS: usize = 60;

/// The sentences of `text`, each with its paragraph and sentence number (one-based).
fn located_sentences(text: &str, locale: SentenceLocale) -> Vec<MakerSentence> {
    prose_paragraphs(text)
        .into_iter()
        .enumerate()
        .flat_map(|(paragraph, block)| {
            split_sentences(&block, locale).into_iter().enumerate().map(|(sentence, text)| MakerSentence {
                text: text.trim().to_string(),
                locator: Some(SourceLocator { chapter: 1, paragraph: Some(paragraph as u32 + 1), sentence: Some(sentence as u32 + 1), ..SourceLocator::default() }),
            }).collect::<Vec<_>>()
        })
        .filter(|sentence| !sentence.text.is_empty())
        .take(MAX_SENTENCES)
        .collect()
}

impl AppView {
    fn sentence_locale(&self) -> SentenceLocale {
        SentenceLocale::from_language_tag(Some(self.state.preferences().general.language.code()))
    }

    fn open_card_maker(&mut self, request: MakerRequest, window: &mut Window, cx: &mut Context<Self>) {
        self.review.maker.update(cx, |maker, cx| maker.open(request, window, cx));
        cx.notify();
    }

    /// Post "…" menu › Make flashcard.
    pub(crate) fn make_flashcard_post(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&post_id).cloned() else { return };
        let sentences: Vec<MakerSentence> = split_sentences(&post.content, self.sentence_locale()).into_iter().map(|text| MakerSentence { text: text.trim().to_string(), locator: None }).collect();
        let initial = if sentences.len() == 1 { post.content.trim().to_string() } else { String::new() };
        let title = crate::state::semantic::hits::shortened(&post.content, TITLE_CHARS);
        let request = MakerRequest { source: CardSource::post(&post.id), title, sentences, ai_text: post.content.clone(), initial_text: initial };
        self.open_card_maker(request, window, cx);
    }

    /// The palette's "Make flashcard": the thought open in the inspector, else a blank card.
    pub(crate) fn make_flashcard_open_thought(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.feed.selected_thread_post_id.clone() {
            Some(post_id) => self.make_flashcard_post(post_id, window, cx),
            None => self.open_blank_card_maker(window, cx),
        }
    }

    /// A vault entry's "Make flashcards": its sentences to pick from.
    pub(crate) fn make_flashcards_document(&mut self, doc_id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(doc) = self.state.documents.iter().find(|doc| doc.id == doc_id).cloned() else { return };
        let sentences = located_sentences(&doc.content, self.sentence_locale());
        let request = MakerRequest { source: CardSource::document(doc.id.clone(), None), title: doc.title.clone(), sentences, ai_text: doc.content.clone(), initial_text: String::new() };
        self.open_card_maker(request, window, cx);
    }

    /// The maker on a typed note (no stored source).
    pub(in crate::ui::features::shell) fn open_blank_card_maker(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let request = MakerRequest { source: CardSource::Sentence { text: String::new() }, title: String::new(), sentences: Vec::new(), ai_text: String::new(), initial_text: String::new() };
        self.open_card_maker(request, window, cx);
    }

    pub(in crate::ui::features::shell) fn handle_maker_event(&mut self, _: &Entity<FlashcardMaker>, event: &MakerEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            MakerEvent::Save(drafts) => self.save_card_drafts(drafts, window, cx),
            MakerEvent::Draft(text) => self.draft_cards_with_ai(text, window, cx),
        }
    }

    /// Expands and saves the drafts, closes the maker and confirms.
    fn save_card_drafts(&mut self, drafts: Vec<CardDraft>, window: &mut Window, cx: &mut Context<Self>) {
        let context = NewCardContext { network_id: self.state.active_network_id.clone(), created_at: Utc::now(), today: review_today() };
        let cards: Vec<_> = drafts.iter().map(cited_sentence).flat_map(|draft| expand_draft(&draft, &context, CardId::generate)).collect();
        if let Some(db) = &self.state.db {
            let _ = db.save_review_cards(cards.clone());
        }
        let count = cards.len();
        self.review.deck = cards.into_iter().fold(std::mem::take(&mut self.review.deck), |deck, card| deck.with_card(card));
        self.review.maker.update(cx, |maker, cx| maker.close(cx));
        self.toast(ToastKind::Confirmation, Notification::success(t_count("review.maker.saved", count)), window, cx);
        self.review_changed(cx);
    }

    /// Asks the configured LLM for cards about `text`; the maker shows the outcome.
    fn draft_cards_with_ai(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        let client = LlmClient::new(self.state.llm_settings.clone());
        let language = self.state.preferences().general.language.english_label();
        let task = cx.background_executor().spawn(async move { request_drafts_blocking(&client, &text, language) });
        let maker = self.review.maker.clone();
        cx.spawn_in(window, async move |_, cx| {
            let result = task.await;
            let _ = maker.update(cx, |maker, cx| maker.ai_finished(result, cx));
        })
        .detach();
    }
}

/// A note card cites its own text as its sentence source.
fn cited_sentence(draft: &CardDraft) -> CardDraft {
    match (&draft.source, &draft.kind) {
        (CardSource::Sentence { text }, kind) if text.is_empty() => {
            let text = crate::ui::features::review::maker::fields::draft_preview(kind);
            CardDraft { source: CardSource::Sentence { text }, ..draft.clone() }
        }
        _ => draft.clone(),
    }
}
