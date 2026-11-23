//! N3: a garden note's card says so: its stage's art and name, quietly, under the author.

use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::views::{maturity_art, maturity_label};
use gpui_kit::*;
use rust_i18n::t;

/// The badge of a garden note; `None` for a post in the stream.
pub(super) fn render_garden_badge(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let note = post.extras.garden?;
    let label = t!("posts.garden.badge", stage = maturity_label(note.maturity)).to_string();
    let badge = div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(maturity_art(note.maturity, theme.symbol_badge_size))
        .child(label);
    Some(badge)
}
