//! Marked words and inline code: bold, italic and underline as `StyledText` highlights, and
//! inline code in the mono face on the `bg_code` fill (fenced blocks live in `code_block`).
//!
//! A spoiler draws no highlight of its own: `spoiler` hides and reveals the whole group.

use super::layout::MarkSet;
use crate::theme::ThemeTokens;
use gpui_kit::{
    div, AnyElement, Div, FontStyle, FontWeight, HighlightStyle, IntoElement, ParentElement, Styled, StyledText, UnderlineStyle,
};
use std::ops::Range;

/// The highlight a set of marks draws; `None` when it changes nothing (plain or spoiler only).
pub(super) fn mark_highlight(marks: MarkSet, theme: &ThemeTokens) -> Option<HighlightStyle> {
    let underline = UnderlineStyle { thickness: theme.border_width_hairline, color: None, wavy: false };
    let style = HighlightStyle {
        font_weight: marks.bold.then_some(FontWeight::BOLD),
        font_style: marks.italic.then_some(FontStyle::Italic),
        underline: marks.underline.then_some(underline),
        ..HighlightStyle::default()
    };
    (style != HighlightStyle::default()).then_some(style)
}

/// A word with its marked ranges drawn as highlights.
pub(super) fn styled_word(text: String, marks: &[(Range<usize>, MarkSet)], theme: &ThemeTokens) -> StyledText {
    let highlights: Vec<(Range<usize>, HighlightStyle)> = marks
        .iter()
        .filter_map(|(range, set)| mark_highlight(*set, theme).map(|style| (range.clone(), style)))
        .collect();
    StyledText::new(text).with_highlights(highlights)
}

/// Mono face on the code fill, shared by spans and blocks.
fn code_surface(theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_sm)
        .text_color(theme.text_primary)
        .bg(theme.bg_code)
}

/// An inline `code` span.
pub(super) fn inline_code(code: &str, theme: &ThemeTokens) -> AnyElement {
    code_surface(theme)
        .min_w_0()
        .px(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .child(code.to_string())
        .into_any_element()
}

