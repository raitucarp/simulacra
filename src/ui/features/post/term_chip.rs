//! N17: a post where a term is defined says so under its author, naming the term. A locked
//! post never shows one (it cannot define a term; a forged definition is not shown either).

use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The chip's text: "Defines: entropy" (`None`: defines nothing, or locked).
pub(crate) fn term_chip_text(post: &Post) -> Option<String> {
    let definition = post.extras.defines.as_deref().filter(|_| !post.is_locked())?;
    Some(t!("posts.glossary.chip", term = definition.term.clone()).to_string())
}

/// The chip; `None` for every other post.
pub(super) fn render_term_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let text = term_chip_text(post)?;
    let chip = div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.accent_brand).child(Icon::new(LucideIcon::BookOpen).xsmall().text_color(theme.accent_brand)).child(div().min_w_0().truncate().child(text));
    Some(chip)
}
