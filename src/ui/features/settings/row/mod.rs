//! The setting row: label and description on the left, the control on the right, and a
//! "reset to default" button only while the value differs from its default. On a narrow
//! window the control wraps below the label.

mod highlight;
mod state;

pub(super) use highlight::highlighted;
pub(super) use state::{element_id as element_id_of, is_modified, RowContext, RowHighlight};

use super::catalog::{is_enabled, SettingRow};
use super::controls::render_control;
use super::tabs::{DESCRIPTION, TITLE};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use super::text::tr;
use rust_i18n::t;

const RESET_PART: &str = "reset";

/// One setting row.
pub(super) fn render_row(ctx: &RowContext, row: SettingRow, highlight: &RowHighlight) -> Div {
    let theme = ctx.theme;
    let enabled = is_enabled(row, &ctx.data.prefs);
    let reset = is_modified(row, ctx.data).then(|| reset_button(ctx, row));
    div()
        .w_full()
        .py(theme.spacing_sm)
        .flex()
        .flex_wrap()
        .items_center()
        .justify_between()
        .gap(theme.control_gap)
        .when(!enabled, |row| row.opacity(theme.settings_disabled_opacity))
        .child(render_label(theme, row, highlight))
        .child(div().flex_none().max_w_full().flex().items_center().gap(theme.inline_gap).children(reset).child(render_control(ctx, row, enabled)))
}

/// Title over description, both with the search matches highlighted.
pub(super) fn render_label(theme: &ThemeTokens, row: SettingRow, highlight: &RowHighlight) -> Div {
    let title = tr(&row.text_key(TITLE));
    let description = tr(&row.text_key(DESCRIPTION));
    div()
        .flex_1()
        .min_w(theme.settings_row_label_min_width)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(
            div()
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .child(highlighted(title, &highlight.title, theme)),
        )
        .child(
            div()
                .text_size(theme.font_size_hint)
                .line_height(relative(theme.line_height_body))
                .text_color(theme.text_muted)
                .child(highlighted(description, &highlight.description, theme)),
        )
}

/// The small "reset to default" button of a modified row.
fn reset_button(ctx: &RowContext, row: SettingRow) -> AnyElement {
    let on_reset = ctx.actions.on_reset.clone();
    Button::new(state::element_id(row, RESET_PART))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Undo2).size(ctx.theme.icon_size_sm))
        .tooltip(t!("prefs.ui.reset_to_default").to_string())
        .on_click(move |_, window, cx| on_reset(row, window, cx))
        .into_any_element()
}

