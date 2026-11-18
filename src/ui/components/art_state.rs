//! An illustrated state (sb D3): themed art over an optional title and a hint, centred. Used
//! by empty states, notices (no key, offline, a failure) and work in progress, so every
//! screen says "nothing here yet" or "something went wrong" the same way.

use crate::theme::ThemeTokens;
use crate::ui::illustration::Illustration;
use gpui_kit::*;

/// What the state says.
pub struct ArtState {
    pub art: Illustration,
    pub title: Option<String>,
    pub hint: String,
}

impl ArtState {
    /// Art and a hint, without a title.
    pub fn hint(art: Illustration, hint: impl Into<String>) -> Self {
        Self { art, title: None, hint: hint.into() }
    }

    /// Art, a title and a hint.
    pub fn titled(art: Illustration, title: impl Into<String>, hint: impl Into<String>) -> Self {
        Self { art, title: Some(title.into()), hint: hint.into() }
    }
}

/// The state as a centred column; callers may chain buttons as more children.
pub fn art_state(state: ArtState, theme: &ThemeTokens) -> Div {
    let title = state.title.map(|title| {
        div()
            .font_family(theme.font_family_display.clone())
            .text_size(theme.font_size_md)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.text_primary)
            .text_center()
            .child(title)
    });
    let hint = div()
        .max_w(theme.reader_measure_narrow_width)
        .text_size(theme.font_size_sm)
        .text_color(theme.text_muted)
        .text_center()
        .line_height(relative(theme.line_height_body))
        .child(state.hint);
    div().flex().flex_col().items_center().gap(theme.stack_gap).child(state.art).children(title).child(hint)
}
