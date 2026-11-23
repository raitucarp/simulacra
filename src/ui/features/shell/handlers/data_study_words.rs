//! N20: sending words of a post's translation to review. The sheet lists the translation's
//! words; each word picked becomes a review item anchored to the post (the post is the answer:
//! the learner recalls what the word means there), tagged with its language. A locked post has
//! no translation, so nothing of it is ever sent.

use super::super::review_host::review_today;
use super::super::view::AppView;
use crate::domain::post::translation::study_words;
use crate::domain::review::item::{new_item, Anchor, AnchorSide, ItemDraft, ItemId, NewItemContext};
use crate::i18n::Language;
use crate::ui::features::data::{tr, TranslateEvent, WordsDialog};
use gpui_kit::*;
use rust_i18n::t;

/// The tag every word item carries (with its language's code).
const VOCABULARY_TAG: &str = "vocabulary";

impl AppView {
    /// Routes the words sheet's events.
    pub(super) fn on_words_event(&mut self, event: TranslateEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            TranslateEvent::Words(id) => {
                let Some(kept) = self.state.get_post(&id).filter(|post| !post.is_locked()).and_then(|post| post.extras.translation.as_deref()) else { return };
                let language = Language::ALL.into_iter().find(|language| language.code() == kept.language).map_or_else(|| kept.language.clone(), |language| language.native_label().to_string());
                self.data.ideas.thoughts.words = Some(WordsDialog { post_id: id, language, words: study_words(&kept.text), chosen: Vec::new() });
            }
            TranslateEvent::ToggleWord(word) => {
                if let Some(dialog) = self.data.ideas.thoughts.words.as_mut() {
                    dialog.toggle(&word);
                }
            }
            TranslateEvent::SendWords => self.send_study_words(window, cx),
            _ => self.data.ideas.thoughts.words = None,
        }
    }

    fn send_study_words(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.thoughts.words.clone().filter(|dialog| !dialog.chosen.is_empty()) else { return self.notify_data(tr("posts.translation.words_pick"), window, cx) };
        let Some(post) = self.state.get_post(&dialog.post_id).filter(|post| !post.is_locked()) else { return };
        let code = post.extras.translation.as_ref().map(|kept| kept.language.clone()).unwrap_or_default();
        let context = NewItemContext { network_id: post.network_id.clone(), created_at: jiff::Timestamp::now(), today: review_today() };
        let draft = |word: &String| ItemDraft { anchor: Anchor::post(&dialog.post_id), side: AnchorSide::Back, written: t!("posts.translation.review_question", word = word.clone(), language = dialog.language.clone()).to_string(), tags: vec![VOCABULARY_TAG.to_string(), code.clone()] };
        let items: Vec<_> = dialog.chosen.iter().filter_map(|word| new_item(&draft(word), &context, ItemId::generate())).collect();
        let count = items.len();
        self.store_review_items(items, cx);
        self.data.ideas.thoughts.words = None;
        self.notify_data(crate::i18n::t_count("posts.translation.words_sent", count), window, cx);
    }
}
