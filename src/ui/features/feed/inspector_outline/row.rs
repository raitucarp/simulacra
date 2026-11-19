//! One outline bullet: a chevron (or a dot for a leaf), the author's short name, dimmed, and
//! the post's text. The cursor row is tinted; the inspected post is marked with the accent.

use super::memory::OutlineCommand;
use super::tree::OutlineRow;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const ROW_ID_PREFIX: &str = "outline-row-";
const CHEVRON_ID_PREFIX: &str = "outline-chevron-";
const LEAF_MARK: &str = "•";
/// A double click opens the post.
const OPEN_CLICKS: usize = 2;

/// What a row sends to the outline.
pub(super) type RowAction = Rc<dyn Fn(OutlineCommand, &mut Window, &mut App) + 'static>;

/// How a row is marked.
#[derive(Clone, Copy)]
pub(super) struct RowMarks {
    pub is_cursor: bool,
    pub is_inspected: bool,
}

pub(super) fn render_row(row: &OutlineRow, marks: RowMarks, on_command: RowAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (focus_id, open_id) = (row.id.clone(), row.id.clone());
    let toggle = on_command.clone();
    let indent = theme.outline_indent * row.depth as f32;
    let hover = theme.hover_bg;
    div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{}", row.id)))
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .pl(indent)
        .pr(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .when(marks.is_cursor, |row| row.bg(theme.hover_bg))
        .hover(move |row| row.bg(hover))
        .on_click(move |event, window, cx| match event.click_count() >= OPEN_CLICKS {
            true => on_command(OutlineCommand::Open(open_id.clone()), window, cx),
            false => on_command(OutlineCommand::Focus(focus_id.clone()), window, cx),
        })
        .child(render_chevron(row, toggle, theme))
        .child(render_text(row, marks, theme))
}

/// A chevron that folds the branch, or a dot for a leaf.
fn render_chevron(row: &OutlineRow, on_command: RowAction, theme: &ThemeTokens) -> Stateful<Div> {
    let slot = div()
        .id(SharedString::from(format!("{CHEVRON_ID_PREFIX}{}", row.id)))
        .flex_none()
        .w(theme.outline_chevron_slot)
        .h(theme.font_size_sm * theme.line_height_body)
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.text_muted);
    if !row.has_children {
        return slot.text_size(theme.font_size_2xs).child(LEAF_MARK);
    }
    let icon = if row.collapsed { LucideIcon::ChevronRight } else { LucideIcon::ChevronDown };
    let (id, strong) = (row.id.clone(), theme.text_primary);
    slot.rounded(theme.radius_xs)
        .hover(move |s| s.text_color(strong))
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_command(OutlineCommand::Toggle(id.clone()), window, cx);
        })
        .child(Icon::new(icon).size(theme.icon_size_xs))
}

fn render_text(row: &OutlineRow, marks: RowMarks, theme: &ThemeTokens) -> Div {
    let line = theme.font_size_sm * theme.line_height_body;
    let author = div()
        .line_height(line)
        .flex_none()
        .max_w(theme.outline_author_max_width)
        .truncate()
        .text_size(theme.font_size_xs)
        .text_color(if marks.is_inspected { theme.accent_brand } else { theme.text_muted })
        .child(row.author.clone());
    let folded = (row.collapsed && row.hidden > 0).then(|| {
        div().flex_none().line_height(line).text_size(theme.font_size_2xs).text_color(theme.text_muted).font_family(theme.font_family_mono.clone()).child(format!("+{}", row.hidden))
    });
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .child(author)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_size(theme.font_size_sm)
                .line_height(line)
                .text_color(theme.text_primary)
                .when(marks.is_inspected, |text| text.font_weight(FontWeight::MEDIUM))
                .child(row.text.clone()),
        )
        .children(folded)
}
