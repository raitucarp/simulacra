//! A word diff as one flowing text: insertions tinted, deletions struck through.

use crate::domain::data::history::{diff_stats, DiffOp, DiffStats};
use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::ops::Range;

/// The text of `ops` and the styled range of each insertion and deletion.
fn styled_runs(ops: &[DiffOp], theme: &ThemeTokens) -> (String, Vec<(Range<usize>, HighlightStyle)>) {
    let added = HighlightStyle { color: Some(theme.accent_success.into()), background_color: Some(theme.accent_success_subtle.into()), ..HighlightStyle::default() };
    let strike = StrikethroughStyle { thickness: theme.border_width_hairline, color: Some(theme.accent_danger.into()) };
    let removed = HighlightStyle { color: Some(theme.accent_danger.into()), strikethrough: Some(strike), ..HighlightStyle::default() };
    ops.iter().enumerate().fold((String::new(), Vec::new()), |(mut text, mut runs), (at, op)| {
        let (run, style) = match op {
            DiffOp::Same(t) => (t, None),
            DiffOp::Added(t) => (t, Some(added)),
            DiffOp::Removed(t) => (t, Some(removed)),
        };
        if at > 0 && touching_change(&ops[at - 1], op) {
            text.push(' ');
        }
        let start = text.len();
        text.push_str(run);
        if let Some(style) = style {
            runs.push((start..text.len(), style));
        }
        (text, runs)
    })
}

/// A removal and an addition that meet with no space between them ("internal" replaced by
/// "public"): they get one, so the two words read apart.
fn touching_change(before: &DiffOp, op: &DiffOp) -> bool {
    let (left, right) = match (before, op) {
        (DiffOp::Removed(left), DiffOp::Added(right)) | (DiffOp::Added(left), DiffOp::Removed(right)) => (left, right),
        _ => return false,
    };
    !left.ends_with(char::is_whitespace) && !right.starts_with(char::is_whitespace)
}

/// The diff as wrapped prose.
pub(super) fn render_diff(ops: &[DiffOp], theme: &ThemeTokens) -> Div {
    let (text, runs) = styled_runs(ops, theme);
    div()
        .w_full()
        .min_h(theme.history_diff_min_height)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .text_size(theme.font_size_sm)
        .line_height(relative(theme.line_height_body))
        .text_color(theme.text_primary)
        .child(StyledText::new(text).with_highlights(runs))
}

/// "+3 words · −1 word" style counts of `ops`.
pub(super) fn render_stats(ops: &[DiffOp], theme: &ThemeTokens) -> Div {
    let DiffStats { added, removed } = diff_stats(ops);
    let part = |text: String, colour: Rgba| div().text_color(colour).child(text);
    div()
        .flex()
        .gap(theme.control_gap)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .child(part(rust_i18n::t!("data.history.added", count = added).to_string(), theme.accent_success))
        .child(part(rust_i18n::t!("data.history.removed", count = removed).to_string(), theme.accent_danger))
}
