//! The learn sheet's requests (sb C1b): a document's sentences and paragraphs, each with its
//! place; saving its drafts; and the AI suggestion of the written face through the
//! configured LLM client.

use super::super::view::AppView;
use crate::agent::LlmClient;
use crate::domain::post::SourceLocator;
use crate::domain::review::quiz::prose_paragraphs;
use crate::domain::review::AnchorSide;
use crate::import::segment::{split_sentences, SentenceLocale};
use crate::ui::features::review::suggest_request::request_suggestion_blocking;
use crate::ui::features::review::{LearnEvent, LearnSheet, LearnSource, Passage};
use gpui_kit::*;

/// Most passages offered from one document, per granularity.
const MAX_PASSAGES: usize = 400;
/// Documents carry no chapters in plain text: every place is in the first.
const FIRST_CHAPTER: u32 = 1;

/// The place of paragraph `paragraph` (and sentence `sentence`), both zero-based.
fn place(paragraph: usize, sentence: Option<usize>) -> Option<SourceLocator> {
    let number = |n: usize| n as u32 + 1;
    Some(SourceLocator { chapter: FIRST_CHAPTER, paragraph: Some(number(paragraph)), sentence: sentence.map(number), ..SourceLocator::default() })
}

/// The sentences and the paragraphs of `text`, each with its place.
fn located_passages(text: &str, locale: SentenceLocale) -> (Vec<Passage>, Vec<Passage>) {
    let paragraphs = prose_paragraphs(text);
    let sentences = paragraphs
        .iter()
        .enumerate()
        .flat_map(|(p, block)| split_sentences(block, locale).into_iter().enumerate().map(move |(s, text)| Passage { text: text.trim().to_string(), locator: place(p, Some(s)) }).collect::<Vec<_>>())
        .filter(|passage| !passage.text.is_empty())
        .take(MAX_PASSAGES)
        .collect();
    let paragraphs = paragraphs.into_iter().enumerate().map(|(p, text)| Passage { text, locator: place(p, None) }).take(MAX_PASSAGES).collect();
    (sentences, paragraphs)
}

impl AppView {
    fn sentence_locale(&self) -> SentenceLocale {
        SentenceLocale::from_language_tag(Some(self.state.preferences().general.language.code()))
    }

    /// The learn source of document `doc_id`: its title, sentences and paragraphs.
    pub(in crate::ui::features::shell) fn document_source(&self, doc_id: &str) -> Option<LearnSource> {
        let doc = self.state.documents.iter().find(|doc| doc.id == doc_id)?;
        let (sentences, paragraphs) = located_passages(&doc.content, self.sentence_locale());
        Some(LearnSource::Document { id: doc.id.clone(), title: doc.title.clone(), sentences, paragraphs })
    }

    pub(in crate::ui::features::shell) fn handle_learn_event(&mut self, _: &Entity<LearnSheet>, event: &LearnEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            LearnEvent::Save { drafts, existing } => self.save_learn_drafts(drafts, existing, window, cx),
            LearnEvent::Suggest { text, side } => self.suggest_with_ai(text, side, window, cx),
        }
    }

    /// Asks the configured LLM for the written face; the sheet shows the outcome.
    fn suggest_with_ai(&mut self, text: String, side: AnchorSide, window: &mut Window, cx: &mut Context<Self>) {
        let client = LlmClient::new(self.state.llm_settings.clone());
        let language = self.state.preferences().general.language.english_label();
        let task = cx.background_executor().spawn(async move { request_suggestion_blocking(&client, &text, side, language) });
        let sheet = self.review.learn.clone();
        cx.spawn_in(window, async move |_, cx| {
            let result = task.await;
            let _ = sheet.update_in(cx, |sheet, window, cx| sheet.ai_finished(result, window, cx));
        })
        .detach();
    }
}
