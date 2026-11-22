//! One to-do row: indentation for its depth, its box (the task symbol of its state), its
//! text (struck through once done or dropped) and its chips.

use super::super::body::{render_body_tokens, BodyLinks};
use super::super::lookup::PostLookup;
use super::chips::todo_chips;
use crate::domain::post::{tokenize_post_body, visible_text};
use crate::domain::symbols::{EntrySymbol, SymbolArt};
use crate::domain::todo::entries::TodoEntry;
use crate::theme::ThemeTokens;
use crate::ui::features::symbols::{render_symbol_art, state_label};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;
use jiff::civil::Date;

/// Element id prefix of a box (suffixed with the post id and the item's index).
const BOX_ID_PREFIX: &str = "todo-box-";

/// What every row of one checklist shares.
pub(super) struct RowContext<'a> {
    pub post_id: String,
    /// The day the post was written, which due phrases count from.
    pub written: Date,
    pub today: Date,
    pub theme: &'a ThemeTokens,
}

/// The box of `entry`: the task symbol of its state, named in its tooltip.
pub(super) fn todo_box(entry: &TodoEntry, context: &RowContext<'_>) -> Stateful<Div> {
    let theme = context.theme;
    let label = state_label(entry.state);
    div()
        .id(SharedString::from(format!("{BOX_ID_PREFIX}{}-{}", context.post_id, entry.index)))
        .flex_none()
        .mt(theme.spacing_3xs)
        .child(render_symbol_art(SymbolArt::Entry(EntrySymbol::Task, Some(entry.state)), theme.todo_box_size))
        .tooltip(move |window, cx| Tooltip::new(label.clone()).build(window, cx))
}

fn todo_text<L: PostLookup>(entry: &TodoEntry, lookup: L, links: &BodyLinks, theme: &ThemeTokens) -> AnyElement {
    let tokens = tokenize_post_body(&entry.item.text);
    if !entry.state.is_closed() {
        return render_body_tokens(&tokens, lookup, theme, links);
    }
    div().font_family(theme.font_family_body.clone()).text_size(theme.font_size_md).text_color(theme.text_muted).line_through().child(visible_text(&tokens)).into_any_element()
}

/// The row of `entry`.
pub(super) fn todo_row<L: PostLookup>(entry: &TodoEntry, lookup: L, links: &BodyLinks, context: &RowContext<'_>) -> AnyElement {
    let theme = context.theme;
    let content = div().flex_1().min_w_0().flex().flex_wrap().items_center().gap(theme.inline_gap).child(todo_text(entry, lookup, links, theme)).children(todo_chips(entry, context));
    div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .pl(theme.todo_indent * entry.depth as f32)
        .child(todo_box(entry, context))
        .child(content)
        .into_any_element()
}
