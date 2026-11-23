//! N6: the first post of a what-if branch says what the branch supposes, quietly, under the
//! author; a locked one shows only the placeholder.

use crate::domain::Post;
use crate::state::store::branch_label;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The chip's text: "What if: <label>" (the placeholder when the post is locked).
pub(crate) fn branch_chip_text(post: &Post) -> Option<String> {
    post.extras.branch.as_ref()?;
    Some(t!("posts.branch.chip", label = branch_label(post)).to_string())
}

/// The chip of a branch's first post; `None` for every other post.
pub(super) fn render_branch_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let text = branch_chip_text(post)?;
    let chip = div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_2xs)
        .text_color(theme.accent_dialectic)
        .child(Icon::new(LucideIcon::GitBranch).xsmall().text_color(theme.accent_dialectic))
        .child(div().min_w_0().truncate().child(text));
    Some(chip)
}
