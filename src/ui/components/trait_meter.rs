//! Compact 0..=100 trait card: title, semantic score badge, ±5 steppers and a 10-segment meter.

use super::badge::mono_tag;
use super::stepper::{mini_step_button, step_id, MINUS_LARGE, PLUS_LARGE, STEP_LARGE};
use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::rc::Rc;

pub use super::trait_level::{trait_semantic_label, TRAIT_MAX};

/// Number of clickable meter segments.
const SEGMENT_COUNT: u8 = 10;
/// Score span covered by one meter segment.
const SEGMENT_SPAN: u8 = TRAIT_MAX / SEGMENT_COUNT;
/// A segment renders filled once the score is within this distance of its threshold.
const SEGMENT_FILL_TOLERANCE: u8 = 5;
const SEGMENT_ID_INFIX: &str = "seg";

/// Static description of one trait meter.
#[derive(Debug, Clone, Copy)]
pub struct TraitMeterSpec<'a> {
    pub id_prefix: &'a str,
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: u8,
    pub accent: Rgba,
}

type ChangeCallback = Rc<dyn Fn(u8, &mut Window, &mut App)>;

/// Renders a compact, scannable trait card suitable for 2-column grids, featuring a
/// semantic level badge (`High · 85%`), a 10-segment visual meter, and `-5 / +5` steppers.
pub fn trait_meter(
    spec: TraitMeterSpec<'_>,
    theme: &ThemeTokens,
    on_change: impl Fn(u8, &mut Window, &mut App) + 'static,
) -> Div {
    let spec = TraitMeterSpec { value: spec.value.min(TRAIT_MAX), ..spec };
    let cb: ChangeCallback = Rc::new(on_change);
    div()
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(header_row(&spec, theme, &cb))
        .child(segment_bar(&spec, theme, &cb))
}

fn header_row(spec: &TraitMeterSpec<'_>, theme: &ThemeTokens, cb: &ChangeCallback) -> Div {
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_xs)
        .child(title_block(spec, theme))
        .child(stepper_cluster(spec, theme, cb))
}

fn title_block(spec: &TraitMeterSpec<'_>, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .truncate()
                .child(spec.title.to_string()),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .truncate()
                .child(spec.subtitle.to_string()),
        )
}

fn stepper_cluster(spec: &TraitMeterSpec<'_>, theme: &ThemeTokens, cb: &ChangeCallback) -> Div {
    let value = spec.value;
    let (cb_minus, cb_plus) = (cb.clone(), cb.clone());
    let (minus_suffix, minus_label) = MINUS_LARGE;
    let (plus_suffix, plus_label) = PLUS_LARGE;
    div()
        .flex_shrink_0()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .child(mini_step_button(step_id(spec.id_prefix, minus_suffix), minus_label, theme, {
            move |w, cx| cb_minus(value.saturating_sub(STEP_LARGE), w, cx)
        }))
        .child(score_badge(spec, theme))
        .child(mini_step_button(step_id(spec.id_prefix, plus_suffix), plus_label, theme, {
            move |w, cx| cb_plus((value + STEP_LARGE).min(TRAIT_MAX), w, cx)
        }))
}

fn score_badge(spec: &TraitMeterSpec<'_>, theme: &ThemeTokens) -> Div {
    let text = format!("{} · {}%", trait_semantic_label(spec.value), spec.value);
    mono_tag(text, spec.accent, theme).font_weight(FontWeight::BOLD)
}

/// 10-segment meter; clicking a segment jumps directly to 10%..100%.
fn segment_bar(spec: &TraitMeterSpec<'_>, theme: &ThemeTokens, cb: &ChangeCallback) -> Div {
    div()
        .w_full()
        .h(theme.trait_bar_height)
        .rounded(theme.radius_full)
        .bg(theme.bg_card)
        .overflow_hidden()
        .flex()
        .gap(theme.spacing_3xs)
        .children((1..=SEGMENT_COUNT).map(|seg_idx| segment(spec, seg_idx, theme, cb.clone())))
}

fn segment(spec: &TraitMeterSpec<'_>, seg_idx: u8, theme: &ThemeTokens, cb: ChangeCallback) -> Stateful<Div> {
    let threshold = seg_idx * SEGMENT_SPAN;
    let filled = spec.value >= threshold.saturating_sub(SEGMENT_FILL_TOLERANCE);
    div()
        .id(format!("{}-{SEGMENT_ID_INFIX}-{}", spec.id_prefix, seg_idx))
        .flex_1()
        .h_full()
        .cursor_pointer()
        .bg(if filled { spec.accent } else { theme.border_subtle })
        .on_click(move |_, w, cx| cb(threshold, w, cx))
}
