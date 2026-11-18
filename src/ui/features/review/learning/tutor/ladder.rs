//! Bloom's ladder as a row of six rungs: the rungs shown so far filled, the rung the open
//! question aims at ringed; and the colour of a verdict.

use super::super::super::text::tr;
use crate::domain::learning::{BloomLevel, Verdict};
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// The accent of a verdict.
pub fn verdict_color(verdict: Verdict, theme: &ThemeTokens) -> Rgba {
    match verdict {
        Verdict::Solid => theme.accent_success,
        Verdict::Partial => theme.warning,
        Verdict::Missing => theme.accent_danger,
    }
}

/// A level's name.
pub fn level_label(level: BloomLevel) -> String {
    tr(&format!("learning.bloom.{}", level.key()))
}

fn rung(level: BloomLevel, shown: bool, aimed: bool, theme: &ThemeTokens) -> Div {
    let dot = div()
        .flex_none()
        .size(theme.learning_ladder_dot_size)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(if aimed { theme.accent_dialectic } else { theme.border })
        .bg(if shown { theme.accent_success } else { theme.bg_transparent })
        .when(aimed, |d| d.border(theme.focus_ring_width));
    let color = if aimed { theme.text_primary } else { theme.text_muted };
    div().flex().items_center().gap(theme.spacing_3xs).child(dot).child(div().text_size(theme.font_size_2xs).text_color(color).child(level_label(level)))
}

/// The ladder with `mastery` reached and `aimed` asked about.
pub fn render_ladder(mastery: Option<BloomLevel>, aimed: Option<BloomLevel>, theme: &ThemeTokens) -> Div {
    let rungs = BloomLevel::ALL.iter().map(|level| rung(*level, mastery.is_some_and(|m| *level <= m), aimed == Some(*level), theme));
    div().flex().flex_wrap().items_center().gap(theme.control_gap).children(rungs)
}
