//! N6: the first post of a what-if branch says what the branch supposes, quietly, under the
//! author; a locked one shows only the placeholder.

use crate::domain::Post;
use crate::state::store::branch_label;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// Punctuation around the "What if…" stem, left out when comparing a label with it.
const STEM_MARKS: &[char] = &['…', '.', '?', '？', '¿'];

/// "What if: <label>", or the label alone when it already begins with "What if" (people
/// usually write the whole question, and "What if: What if…" reads as a stutter).
pub(crate) fn what_if_text(label: &str) -> String {
    let stem = t!("posts.branch.label").trim_matches(|c: char| c.is_whitespace() || STEM_MARKS.contains(&c)).to_lowercase();
    let asks = label.trim_start_matches(|c: char| c.is_whitespace() || STEM_MARKS.contains(&c)).to_lowercase().starts_with(&stem);
    if asks && !stem.is_empty() {
        label.to_string()
    } else {
        t!("posts.branch.chip", label = label).to_string()
    }
}

/// The chip's text (the placeholder when the post is locked).
pub(crate) fn branch_chip_text(post: &Post) -> Option<String> {
    post.extras.branch.as_ref()?;
    Some(what_if_text(&branch_label(post)))
}

/// The chip of a branch's first post; `None` for every other post.
pub(super) fn render_branch_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    let text = branch_chip_text(post)?;
    let chip = div()
        .max_w_full()
        .min_w_0()
        .overflow_hidden()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_2xs)
        .text_color(theme.accent_dialectic)
        .child(Icon::new(LucideIcon::GitBranch).xsmall().text_color(theme.accent_dialectic))
        .child(div().min_w_0().truncate().child(text));
    Some(chip)
}
