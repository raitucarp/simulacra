//! Text with the search matches highlighted.

use crate::theme::ThemeTokens;
use gpui_kit::{FontWeight, HighlightStyle, StyledText};
use std::ops::Range;

/// `text` with every range in `ranges` tinted and emboldened.
pub(in crate::ui::features::settings) fn highlighted(text: String, ranges: &[Range<usize>], theme: &ThemeTokens) -> StyledText {
    let style = HighlightStyle {
        color: Some(theme.text_primary.into()),
        background_color: Some(theme.accent_dialectic_subtle.into()),
        font_weight: Some(FontWeight::BOLD),
        ..HighlightStyle::default()
    };
    let valid: Vec<(Range<usize>, HighlightStyle)> = ranges
        .iter()
        .filter(|range| range.end <= text.len() && text.is_char_boundary(range.start) && text.is_char_boundary(range.end))
        .map(|range| (range.clone(), style))
        .collect();
    StyledText::new(text).with_highlights(valid)
}
