//! The card maker's body: the sentence picker, the fields (with clickable cloze words and a
//! preview), the tags, the problem line and the list of cards to save.

use super::fields::{clickable_words, draft_preview, MakerKind, MakerSentence};
use super::FlashcardMaker;
use crate::domain::review::card::card_faces;
use crate::domain::review::card::cloze::cloze_ordinals;
use crate::domain::review::{CardKind, DraftKind};
use crate::ui::components::{chip_row, choice_chip};
use crate::ui::features::review::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const SENTENCES_ID: &str = "maker-sentences";
const SENTENCE_ID_PREFIX: &str = "maker-sentence-";
const WORD_ID_PREFIX: &str = "maker-word-";
const REMOVE_ID_PREFIX: &str = "maker-remove-";

impl FlashcardMaker {
    fn label(&self, text: String) -> Div {
        div().text_size(self.theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(self.theme.text_secondary).child(text)
    }

    fn hint(&self, text: String) -> Div {
        div().text_size(self.theme.font_size_2xs).text_color(self.theme.text_muted).child(text)
    }

    fn field(&self, label: String, input: &Entity<InputState>) -> Div {
        let theme = &self.theme;
        let frame = div().w_full().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).px(theme.spacing_sm).child(Input::new(input).bordered(false));
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(label)).child(frame)
    }

    pub(super) fn render_sentences(&self, sentences: &[MakerSentence], cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let rows = sentences.iter().enumerate().map(|(index, sentence)| {
            let chosen = self.chosen == Some(index);
            div()
                .id(ElementId::NamedInteger(SENTENCE_ID_PREFIX.into(), index as u64))
                .px(theme.spacing_sm)
                .py(theme.spacing_xs)
                .rounded(theme.radius_sm)
                .text_size(theme.font_size_xs)
                .text_color(if chosen { theme.text_primary } else { theme.text_secondary })
                .when(chosen, |row| row.bg(theme.accent_brand_subtle))
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .on_click(cx.listener(move |this, _, window, cx| this.choose_sentence(index, window, cx)))
                .child(sentence.text.clone())
        });
        let list = div().id(SENTENCES_ID).max_h(theme.review_sentence_list_height).overflow_y_scroll().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border_subtle).p(theme.spacing_2xs).flex().flex_col().children(rows);
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(t!("review.maker.sentences", count = sentences.len()).to_string())).child(list)
    }

    pub(super) fn render_fields(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let key = self.kind.key();
        let front = self.field(tr(&format!("review.maker.{key}.front")), &self.front);
        let back = self.kind.has_back().then(|| self.field(tr(&format!("review.maker.{key}.back")), &self.back));
        let words = (self.kind == MakerKind::Cloze).then(|| self.render_words(cx));
        let problem = self.problem.map(|problem| div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(tr(problem.message_key())));
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(front)
            .children(words)
            .children(back)
            .child(self.hint(tr(&format!("review.maker.{key}.tip"))))
            .child(self.field(tr("review.maker.tags"), &self.tags))
            .children(problem)
    }

    /// The cloze text's words as chips (picked ones hidden on the card) and a preview.
    fn render_words(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let text = self.front.read(cx).value().to_string();
        let words = clickable_words(&text);
        let chips = words.iter().enumerate().map(|(index, word)| {
            let entity = cx.entity().downgrade();
            choice_chip(ElementId::NamedInteger(WORD_ID_PREFIX.into(), index as u64), word, self.picked.contains(&index), theme, move |_, cx| {
                let _ = entity.update(cx, |maker, cx| maker.toggle_word(index, cx));
            })
        });
        let preview = match self.current_draft(cx) {
            DraftKind::Cloze { text } => cloze_ordinals(&text).first().map(|ordinal| card_faces(&CardKind::Cloze { text: text.clone(), ordinal: *ordinal }).prompt_text()),
            _ => None,
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .when(!words.is_empty(), |block| block.child(self.hint(tr("review.maker.cloze.words"))).child(chip_row(theme).children(chips)))
            .children(preview.map(|preview| self.hint(t!("review.maker.preview", card = preview).to_string())))
    }

    pub(super) fn render_pending(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let rows = self.pending.iter().enumerate().map(|(index, draft)| {
            let remove = Button::new(ElementId::NamedInteger(REMOVE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.maker.remove"))
                .on_click(cx.listener(move |this, _, _, cx| this.remove_pending(index, cx)));
            div()
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .child(Icon::new(LucideIcon::Layers).size(theme.icon_size_xs).text_color(theme.text_muted))
                .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(draft_preview(&draft.kind)))
                .child(remove)
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(t!("review.maker.pending", count = self.pending.len()).to_string())).children(rows)
    }
}
