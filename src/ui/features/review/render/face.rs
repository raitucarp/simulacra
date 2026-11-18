//! A card face as one wrapped paragraph: blanks drawn as a tinted gap (with the hint),
//! revealed answers in the accent, bold.

use crate::domain::review::card::FacePart;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;
use std::ops::Range;

/// Visible filler of a blank without a hint.
const BLANK_FILL: &str = "\u{2002}\u{2002}…\u{2002}\u{2002}";
const BLANK_PAD: &str = "\u{2002}";

/// The paragraph text and the styled ranges of `parts`.
fn styled_runs(parts: &[FacePart], theme: &ThemeTokens) -> (String, Vec<(Range<usize>, HighlightStyle)>) {
    let blank = HighlightStyle {
        color: Some(theme.accent_brand.into()),
        background_color: Some(theme.accent_brand_subtle.into()),
        font_weight: Some(FontWeight::SEMIBOLD),
        ..Default::default()
    };
    let answer = HighlightStyle { color: Some(theme.accent_success.into()), font_weight: Some(FontWeight::BOLD), ..Default::default() };
    parts.iter().fold((String::new(), Vec::new()), |(mut text, mut runs), part| {
        let start = text.len();
        let style = match part {
            FacePart::Text(plain) => {
                text.push_str(plain);
                None
            }
            FacePart::Blank(hint) => {
                let hint = hint.as_ref().map(|hint| t!("review.face.hint", hint = hint).to_string());
                text.push_str(&hint.map_or_else(|| BLANK_FILL.to_string(), |hint| format!("{BLANK_PAD}{hint}{BLANK_PAD}")));
                Some(blank)
            }
            FacePart::Answer(shown) => {
                text.push_str(shown);
                Some(answer)
            }
        };
        runs.extend(style.filter(|_| text.len() > start).map(|style| (start..text.len(), style)));
        (text, runs)
    })
}

/// `parts` in `size`, wrapped, with the body leading.
pub fn render_face(parts: &[FacePart], size: Pixels, color: Rgba, theme: &ThemeTokens) -> Div {
    let (text, runs) = styled_runs(parts, theme);
    div()
        .w_full()
        .text_size(size)
        .line_height(relative(theme.line_height_reading))
        .text_color(color)
        .child(StyledText::new(text).with_highlights(runs))
}
