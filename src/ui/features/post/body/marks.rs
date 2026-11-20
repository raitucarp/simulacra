//! Marked words and code: bold, italic and underline as `StyledText` highlights, inline code
//! and fenced blocks in the mono face on the `bg_code` fill.
//!
//! Code blocks are a plain monospace block for now; highlighting and the language label come
//! with the code block card. Spoilers render as plain text until the reveal control exists.

use super::layout::MarkSet;
use crate::theme::ThemeTokens;
use gpui_kit::{
    div, relative, AnyElement, Div, FontStyle, FontWeight, HighlightStyle, IntoElement, ParentElement, Styled, StyledText,
    UnderlineStyle,
};
use std::ops::Range;

/// Text shown for an empty code line, so it keeps its height.
const EMPTY_CODE_LINE: &str = " ";

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

/// A fenced code block: full width, one row per source line, plain (no highlighting yet).
pub(super) fn code_block(code: &str, theme: &ThemeTokens) -> AnyElement {
    let lines = code.split('\n').map(|line| {
        let line = if line.is_empty() { EMPTY_CODE_LINE } else { line };
        div().min_w_0().child(line.to_string())
    });
    code_surface(theme)
        .w_full()
        .my(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .line_height(relative(theme.line_height_body))
        .flex()
        .flex_col()
        .children(lines)
        .into_any_element()
}
