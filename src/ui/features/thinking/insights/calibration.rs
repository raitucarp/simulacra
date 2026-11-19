//! Pieces of the Calibration view: the key figures (resolved, Brier, log score, confidence
//! bias), the Murphy decomposition as bars, the forecast histogram's columns, and the list of
//! open predictions by due date with one-click resolution.

use super::super::labels::{keyed, outcome_label, tr};
use super::{CalibrationData, PredictionRow, RequestAction, ThinkingRequest};
use crate::domain::thinking::credence::{CalibrationSummary, Outcome};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::columns::Column;
use crate::ui::features::insights::chart::ranked::RankedRow;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID: &str = "calibration-row";
const OUTCOME_ID: &str = "calibration-outcome";
const EXCERPT_ID: &str = "calibration-excerpt";
const DAY_FORMAT: &str = "%Y-%m-%d";
const PERCENT: f64 = 100.0;

/// `(value, label, colour)` of each key figure.
pub fn key_figures(summary: &CalibrationSummary, theme: &ThemeTokens) -> Vec<(String, String, Rgba)> {
    let score = |value: Option<f64>| value.map_or_else(|| "—".to_string(), |v| format!("{v:.3}"));
    let bias = summary.bias.map_or_else(
        || ("—".to_string(), tr("thinking.calibration.bias")),
        |(bias, gap)| (format!("{:+.0} pt", gap * PERCENT), keyed("thinking.calibration.bias_kind", bias.key())),
    );
    vec![
        (summary.resolved.to_string(), tr("thinking.calibration.resolved"), theme.accent_brand),
        (summary.open.to_string(), tr("thinking.calibration.open"), theme.accent_user),
        (summary.ambiguous.to_string(), tr("thinking.calibration.ambiguous"), theme.text_secondary),
        (score(summary.brier), tr("thinking.calibration.brier"), theme.accent_success),
        (score(summary.log), tr("thinking.calibration.log"), theme.info),
        (bias.0, bias.1, theme.accent_dialectic),
    ]
}

/// Reliability, resolution and uncertainty as ranked bars (empty before any resolution).
pub fn murphy_rows(summary: &CalibrationSummary, theme: &ThemeTokens) -> Vec<RankedRow> {
    let Some(m) = summary.murphy else { return Vec::new() };
    [(m.reliability, "reliability", theme.accent_danger), (m.resolution, "resolution", theme.accent_success), (m.uncertainty, "uncertainty", theme.text_muted)]
        .into_iter()
        .map(|(value, key, colour)| RankedRow { label: keyed("thinking.calibration.murphy", key), value, value_text: format!("{value:.3}"), colour, active: false })
        .collect()
}

/// One column per bin: how many forecasts were given that chance, split into held / failed.
pub fn histogram(summary: &CalibrationSummary, theme: &ThemeTokens) -> (Vec<Column>, Vec<String>) {
    let columns = summary.bins.iter().map(|bin| {
        let tip = t!("thinking.calibration.bin_tip", low = (bin.low * PERCENT).round(), high = (bin.high * PERCENT).round(), hits = bin.hits, count = bin.count).to_string();
        Column { segments: vec![(bin.hits as f64, theme.accent_success), ((bin.count - bin.hits) as f64, theme.accent_danger)], tip }
    });
    let labels = summary.bins.iter().map(|bin| format!("{:.0}", bin.low * PERCENT)).collect();
    (columns.collect(), labels)
}

/// The open predictions (only the due ones when asked), each with its resolve buttons.
pub fn prediction_list(data: &CalibrationData, on_request: &RequestAction, theme: &ThemeTokens) -> Div {
    let rows: Vec<&PredictionRow> = data.open.iter().filter(|row| !data.due_only || row.is_due).collect();
    let empty = rows.is_empty().then(|| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(if data.due_only { "thinking.calibration.none_due" } else { "thinking.calibration.none_open" })));
    div().w_full().flex().flex_col().gap(theme.spacing_3xs).children(rows.into_iter().enumerate().map(|(i, row)| prediction_row(i, row, on_request, theme))).children(empty)
}

fn prediction_row(index: usize, row: &PredictionRow, on_request: &RequestAction, theme: &ThemeTokens) -> Stateful<Div> {
    let due = row.due.map_or_else(|| tr("thinking.credence.no_date"), |d| d.strftime(DAY_FORMAT).to_string());
    let due_colour = if row.is_due { theme.warning } else { theme.text_muted };
    let outcomes = Outcome::ALL.iter().enumerate().map(|(o, outcome)| {
        let (on_request, request) = (on_request.clone(), ThinkingRequest::ResolvePrediction(row.post_id.clone(), *outcome));
        Button::new(ElementId::NamedInteger(OUTCOME_ID.into(), (index * Outcome::ALL.len() + o) as u64)).ghost().xsmall().label(outcome_label(*outcome)).on_click(move |_, window, cx| on_request(request.clone(), window, cx))
    });
    let (open, post) = (on_request.clone(), row.post_id.clone());
    div()
        .id(ElementId::NamedInteger(ROW_ID.into(), index as u64))
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .hover(|r| r.bg(theme.hover_bg))
        .child(div().flex_none().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_2xs).text_color(due_colour).child(Icon::new(LucideIcon::CalendarClock).size(theme.icon_size_xs)).child(due))
        .child(div().flex_none().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_brand).child(format!("{}%", row.percent)))
        .child(
            div()
                .id(ElementId::NamedInteger(EXCERPT_ID.into(), index as u64))
                .flex_1()
                .min_w(theme.insights_rank_label_width)
                .truncate()
                .cursor_pointer()
                .text_size(theme.font_size_xs)
                .text_color(theme.text_primary)
                .on_click(move |_, window, cx| open(ThinkingRequest::OpenPost(post.clone()), window, cx))
                .child(row.excerpt.clone()),
        )
        .child(div().flex().gap(theme.spacing_3xs).children(outcomes))
}
