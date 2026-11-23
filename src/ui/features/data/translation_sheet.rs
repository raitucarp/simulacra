//! N20: bilingual posts from a post's menu and card: show it in the language the user is
//! learning (asking the AI for it the first time), show the original again, and send words of
//! the translation to review from the sheet listing them.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, hint, sheet_frame, SheetParts};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-study-words";
const WORD_ID: &str = "data-study-word-";

/// One interaction with a bilingual post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslateEvent {
    /// Shows the post in the language the user is learning.
    Show(PostId),
    /// Shows the post as written again.
    Original(PostId),
    /// Opens the sheet of the translation's words.
    Words(PostId),
    ToggleWord(String),
    SendWords,
    CloseWords,
}

/// The words sheet's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordsDialog {
    pub post_id: PostId,
    /// The native name of the translation's language.
    pub language: String,
    pub words: Vec<String>,
    pub chosen: Vec<String>,
}

impl WordsDialog {
    /// `word` picked or dropped.
    pub fn toggle(&mut self, word: &str) {
        match self.chosen.iter().position(|chosen| chosen == word) {
            Some(at) => _ = self.chosen.remove(at),
            None => self.chosen.push(word.to_string()),
        }
    }
}

fn send(on_event: &DataAction, event: TranslateEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Thought(ThoughtEvent::Translate(event.clone())), window, cx)
}

/// The overlay.
pub fn render_words_sheet(dialog: &WordsDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let words = dialog.words.iter().enumerate().map(|(at, word)| {
        let chosen = dialog.chosen.contains(word);
        Button::new(SharedString::from(format!("{WORD_ID}{at}"))).ghost().small().selected(chosen).label(word.clone()).on_click(send(&on_event, TranslateEvent::ToggleWord(word.clone())))
    });
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.translation.words_explain"), theme)).child(div().flex().flex_wrap().gap(theme.inline_gap).children(words));
    let label = t!("posts.translation.words_send", count = dialog.chosen.len()).to_string();
    let buttons = footer(theme)
        .child(Button::new("data-study-words-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, TranslateEvent::CloseWords)))
        .child(Button::new("data-study-words-send").primary().small().icon(Icon::new(LucideIcon::GraduationCap)).label(label).on_click(send(&on_event, TranslateEvent::SendWords)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::Languages,
        title: tr("posts.translation.words_title"),
        subtitle: Some(t!("posts.translation.words_subtitle", language = dialog.language.clone()).to_string()),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Thought(ThoughtEvent::Translate(TranslateEvent::CloseWords)), window, cx)),
    };
    sheet_frame(parts, theme)
}
