//! The board (W6): one column per state (open, scheduled, delegated, migrated, done,
//! dropped) holding the to-dos in that state and the task posts beside them. A card shows
//! its text, its due day and priority; a click opens its post. The columns scroll sideways
//! when they do not fit.

use super::common::{empty_note, locked_note, section_heading};
use super::input::ViewInput;
use crate::domain::symbols::{EntrySymbol, SymbolArt};
use crate::domain::views::board::{board, card_count, BoardCard, BoardColumn};
use crate::ui::features::symbols::{render_symbol_art, state_label};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// The `!` a priority is written with.
const PRIORITY_MARK: &str = "!";

pub(super) fn render_board(input: &ViewInput<'_>) -> AnyElement {
    let columns = board(input.posts.iter().map(|post| (*post, crate::time::local_date(post.created_at))));
    if card_count(&columns) == 0 {
        return empty_note("views.board.empty", input.theme);
    }
    let theme = input.theme;
    let row = div().flex().items_start().gap(theme.control_gap).children(columns.iter().map(|column| render_column(column, input)));
    div().id(input.id("board")).flex_1().min_h_0().w_full().overflow_scroll().p(theme.panel_padding_compact).child(row).into_any_element()
}

fn render_column(column: &BoardColumn, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let title = format!("{} · {}", state_label(column.state), column.cards.len());
    let head = div().flex().items_center().gap(theme.inline_gap).child(render_symbol_art(SymbolArt::Entry(EntrySymbol::Task, Some(column.state)), theme.symbol_badge_size)).child(section_heading(title, theme));
    div()
        .flex_none()
        .w(theme.board_column_width)
        .p(theme.spacing_xs)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_sidebar)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(head)
        .children(column.cards.iter().map(|card| render_card(card, input)))
}

fn render_card(card: &BoardCard, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let text = match card.locked {
        true => locked_note(theme).into_any_element(),
        false => div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(card.text.clone()).into_any_element(),
    };
    let due = card.due.map(|due| t!("views.board.due", date = crate::ui::features::day_filter::format_day(due)).to_string());
    let priority = card.priority.map(|level| PRIORITY_MARK.repeat(usize::from(level)));
    let meta = div().flex().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).children(priority.map(|mark| div().text_color(theme.accent_danger).child(mark))).children(due);
    let part = format!("board-{}-{}", card.post_id, card.todo.map_or_else(|| "post".to_string(), |index| index.to_string()));
    let hover = theme.hover_bg;
    div()
        .id(input.id(part))
        .p(theme.spacing_xs)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |card| card.bg(hover))
        .on_click(input.opener(&card.post_id))
        .child(text)
        .when(card.due.is_some() || card.priority.is_some(), |card_el| card_el.child(meta))
}
