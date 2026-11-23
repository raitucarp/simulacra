//! The history sheet's version list (V5): one row per version, newest edit first, its graph
//! lanes on the left, then what it is (current, original, earlier), its number and when it
//! was replaced.

use super::events::{DataAction, DataEvent};
use super::history::HistorySheet;
use super::sheet::heading;
use super::text::{format_time, tr};
use super::version_tree::render_lanes;
use crate::domain::data::history::{LaneRow, Version, VersionKind};
use crate::theme::ThemeTokens;
use crate::ui::features::composer::version_label;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Selectable, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID_PREFIX: &str = "data-history-version-";
const B_ID_PREFIX: &str = "data-history-compare-";

fn kind_label(version: &Version) -> String {
    match version.kind {
        VersionKind::Current => tr("data.history.current"),
        VersionKind::Earlier => tr("data.history.earlier"),
        VersionKind::Original => tr("data.history.original"),
    }
}

/// "Current v1.2.0": what the version is, and its number when it has one.
fn title_line(version: &Version, theme: &ThemeTokens) -> Div {
    let number = version.number.map(|n| div().font_family(theme.font_family_mono.clone()).text_color(theme.text_muted).child(version_label(n)));
    let rendering = crate::domain::data::history::PostRecord::parse(&version.record_json).and_then(|record| record.rendering).map(|depth| div().text_color(theme.text_muted).child(t!("posts.depth.history", depth = super::depth_dial::depth_label(Some(depth))).to_string())); // N5
    let kind = div().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(kind_label(version));
    div().flex().flex_wrap().items_center().gap(theme.inline_gap).text_size(theme.font_size_xs).child(kind).children(number).children(rendering)
}

/// The "B" button picking a row as the other side of the comparison (V6), lit on version B.
fn b_button(index: usize, compared: bool, on_event: &DataAction) -> Button {
    let on_event = on_event.clone();
    Button::new(SharedString::from(format!("{B_ID_PREFIX}{index}")))
        .ghost()
        .xsmall()
        .selected(compared)
        .label(tr("data.history.side_b"))
        .tooltip(tr("data.history.compare_with"))
        .on_click(move |_, window, cx| on_event(DataEvent::CompareWith(index), window, cx))
}

fn version_row(index: usize, version: &Version, lanes: (&LaneRow, usize), (selected, compared): (bool, bool), on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let pick = on_event.clone();
    let when = version.replaced_at.map_or_else(|| tr("data.history.now"), |at| t!("data.history.until", time = format_time(at)).to_string());
    let hover = theme.hover_bg;
    let time = div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(when);
    let text = div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).py(theme.spacing_xs).child(title_line(version, theme)).child(time);
    let main = div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{index}")))
        .flex_1()
        .min_w_0()
        .flex()
        .gap(theme.inline_gap)
        .cursor_pointer()
        .on_click(move |_, window, cx| pick(DataEvent::SelectVersion(index), window, cx))
        .child(render_lanes(lanes.0, lanes.1, version.kind == VersionKind::Current, theme))
        .child(text);
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .px(theme.spacing_sm)
        .rounded(theme.radius_md)
        .when(selected, |row| row.bg(theme.accent_brand_subtle))
        .when(!selected, |row| row.hover(move |s| s.bg(hover)))
        .child(main)
        .child(b_button(index, compared, on_event))
}

/// The list column: its heading and every row.
pub(super) fn version_list_column(sheet: &HistorySheet, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let rows = sheet.versions.iter().zip(&sheet.lanes).enumerate().map(|(i, (v, lanes))| version_row(i, v, (lanes, sheet.lane_count), (i == sheet.selected, i == sheet.compare), on_event, theme));
    div().w(theme.history_list_width).flex_none().flex().flex_col().child(heading(tr("data.history.versions"), theme)).children(rows)
}
