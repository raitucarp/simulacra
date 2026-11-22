//! Completing to-do tokens in the composer (D11): while the word at the cursor of a to-do line
//! starts with `^`, `^^`, `&` or `@`, a row under the drafts offers what it could be (due
//! phrases with the day each names, repeat rules, contexts in use, agents); a click puts the
//! token in place of the word.

use crate::domain::todo::complete::Suggestion;
use crate::theme::ThemeTokens;
use crate::ui::features::post::todo_due_label;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::{div, App, Div, ParentElement, SharedString, Styled, Window};
use jiff::civil::Date;
use rust_i18n::t;
use std::rc::Rc;

/// Puts the picked token in place of the word being typed.
pub type PickCompletion = Rc<dyn Fn(String, &mut Window, &mut App) + 'static>;

const SUGGESTION_ID: &str = "composer-todo-suggestion";

/// What the row offers.
#[derive(Clone)]
pub struct TodoSuggestView {
    pub items: Vec<Suggestion>,
    pub today: Date,
    pub on_pick: PickCompletion,
}

/// "^fri · Friday, 2026-11-27": the token and, for a due phrase, its day.
pub fn suggestion_label(suggestion: &Suggestion, today: Date) -> String {
    match suggestion.day {
        Some(day) => t!("todo.complete.with_day", token = suggestion.token, day = todo_due_label(day, today)).to_string(),
        None => suggestion.token.clone(),
    }
}

/// The row of suggestions under the drafts.
pub(super) fn todo_suggest_row(view: &TodoSuggestView, theme: &ThemeTokens) -> Div {
    let buttons = view.items.iter().enumerate().map(|(n, item)| {
        let (on_pick, token) = (view.on_pick.clone(), item.token.clone());
        Button::new(SharedString::from(format!("{SUGGESTION_ID}-{n}")))
            .ghost()
            .xsmall()
            .label(suggestion_label(item, view.today))
            .on_click(move |_, window, cx| on_pick(token.clone(), window, cx))
    });
    let hint = div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("todo.complete.hint").to_string());
    div()
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.panel_padding)
        .py(theme.spacing_2xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(hint)
        .children(buttons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jiff::civil::date;

    #[test]
    fn a_due_phrase_shows_its_day_and_names_show_alone() {
        let today = date(2026, 11, 22);
        let fri = Suggestion { token: "^fri".into(), day: Some(date(2026, 11, 27)) };
        assert_eq!(suggestion_label(&fri, today), t!("todo.complete.with_day", token = "^fri", day = todo_due_label(date(2026, 11, 27), today)));
        assert_eq!(suggestion_label(&Suggestion { token: "&home".into(), day: None }, today), "&home");
        let view = TodoSuggestView { items: vec![fri], today, on_pick: Rc::new(|_, _, _| {}) };
        let _row = todo_suggest_row(&view, &ThemeTokens::default());
        assert_ne!(t!("todo.complete.hint", locale = "en"), "todo.complete.hint");
    }
}
