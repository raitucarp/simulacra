//! N15: the root of a thread distilled into the vault says so under its author, naming the
//! document. A locked root says nothing more than its placeholder.

use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The chip's text (`None`: never distilled, or locked).
pub(crate) fn distill_text(post: &Post) -> Option<String> {
    let distilled = post.extras.distilled.as_deref().filter(|_| !post.is_locked())?;
    Some(t!("posts.distill.chip", title = distilled.title.clone()).to_string())
}

/// The chip; `None` on a post never distilled.
pub(super) fn render_distill_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let text = distill_text(post)?;
    // Capped at the mark line's width, so a long document title ends in an ellipsis.
    let chip = div().max_w_full().min_w_0().overflow_hidden().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(Icon::new(LucideIcon::BookMarked).xsmall().text_color(theme.text_muted)).child(div().min_w_0().truncate().child(text));
    Some(chip)
}
