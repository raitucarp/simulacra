//! The rail of every action, grouped (Understand, Challenge, Explore), the active one marked.

use super::labels::{group_title, kind_icon, kind_title};
use super::model::SheetEvent;
use super::sheet::SheetAction;
use crate::domain::post_actions::{ActionGroup, ActionKind};
use crate::theme::ThemeTokens;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const RAIL_ID: &str = "post-actions-rail";
const ROW_ID_PREFIX: &str = "post-action-";

/// The scrollable rail.
pub(super) fn render_rail(active: Option<ActionKind>, on_event: &SheetAction, theme: &ThemeTokens) -> Stateful<Div> {
    let sections = ActionGroup::ALL.iter().map(|group| render_group(*group, active, on_event, theme));
    div()
        .id(RAIL_ID)
        .w(theme.post_actions_rail_width)
        .flex_none()
        .h_full()
        .overflow_y_scroll()
        .py(theme.spacing_xs)
        .px(theme.panel_padding_compact)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .children(sections)
}

fn render_group(group: ActionGroup, active: Option<ActionKind>, on_event: &SheetAction, theme: &ThemeTokens) -> Div {
    let heading = div()
        .px(theme.spacing_xs)
        .pt(theme.spacing_xs)
        .text_size(theme.font_size_micro)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.text_muted)
        .child(group_title(group).to_uppercase());
    let rows = ActionKind::in_group(group).map(|kind| render_row(kind, active == Some(kind), on_event.clone(), theme));
    div().flex().flex_col().gap(theme.spacing_3xs).child(heading).children(rows)
}

fn render_row(kind: ActionKind, is_active: bool, on_event: SheetAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (text, icon) = if is_active { (theme.text_primary, theme.accent_brand) } else { (theme.text_secondary, theme.text_muted) };
    let hover = theme.hover_bg;
    div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{}", kind.key())))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .when(is_active, |row| row.bg(theme.accent_brand_subtle))
        .hover(move |row| row.bg(hover))
        .text_size(theme.font_size_xs)
        .text_color(text)
        .on_click(move |_, window, cx| on_event(SheetEvent::Pick(kind), window, cx))
        .child(Icon::new(kind_icon(kind)).size(theme.icon_size_sm).text_color(icon))
        .child(div().min_w_0().truncate().child(kind_title(kind)))
}
