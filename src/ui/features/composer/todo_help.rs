//! The to-do syntax help card of the composer (D1): a footer button opens a card listing how
//! each piece of a to-do is typed (boxes, nesting, due, repeat, priority, context, estimate,
//! delegate) next to what it does.

use crate::domain::todo::syntax::{SyntaxRow, SyntaxTopic, SYNTAX_ROWS};
use crate::theme::ThemeTokens;
use crate::ui::features::data::tr;
use crate::ui::features::symbols::state_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, App, Div, FontWeight, ParentElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Opens or closes the card.
pub type TodoHelpToggle = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

const BUTTON_ID: &str = "composer-todo-help";

/// What a row says it does.
pub fn topic_label(topic: SyntaxTopic) -> String {
    match (topic, topic.label_key()) {
        (SyntaxTopic::Box(state), _) => state_label(state),
        (_, Some(key)) => tr(key),
        (_, None) => String::new(),
    }
}

/// The footer button, lit while the card is open.
pub(super) fn todo_help_button(open: bool, on_toggle: TodoHelpToggle) -> Button {
    Button::new(BUTTON_ID)
        .ghost()
        .small()
        .selected(open)
        .icon(Icon::new(LucideIcon::ListChecks))
        .tooltip(t!("todo.help.open").to_string())
        .on_click(move |_, window, cx| on_toggle(window, cx))
}

fn syntax_row(row: &SyntaxRow, theme: &ThemeTokens) -> Div {
    let sample = div().flex_none().w(theme.todo_help_sample_width).font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_primary).child(row.sample);
    let meaning = div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(topic_label(row.topic));
    div().flex().items_center().gap(theme.control_gap).child(sample).child(meaning)
}

/// The card under the drafts: a title, one row per piece of syntax and a closing note.
pub(super) fn todo_help_card(theme: &ThemeTokens) -> Div {
    let title = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(t!("todo.help.title").to_string());
    let note = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("todo.help.note").to_string());
    let card = div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .child(title)
        .children(SYNTAX_ROWS.iter().map(|row| syntax_row(row, theme)))
        .child(note);
    div().w_full().px(theme.panel_padding).py(theme.spacing_xs).border_t(theme.border_width_hairline).border_color(theme.border_subtle).child(card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::symbols::EntryState;

    #[test]
    fn every_row_has_a_translated_meaning() {
        for row in SYNTAX_ROWS {
            let label = topic_label(row.topic);
            assert!(!label.is_empty() && !label.starts_with("todo."), "{:?}", row.topic);
        }
        assert_eq!(topic_label(SyntaxTopic::Box(EntryState::Done)), state_label(EntryState::Done));
        let _card = todo_help_card(&ThemeTokens::default());
    }
}
