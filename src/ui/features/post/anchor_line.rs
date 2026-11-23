//! N2: a margin reply shows the words it answers above its own text, quoted from its parent,
//! with a quiet note once that passage has changed; under a locked parent it shows only the
//! placeholder, never the quoted words.

use super::lookup::PostLookup;
use crate::domain::post::anchor::{AnchorPlace, SpanAnchor};
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

/// What the quote line of `anchor` (on a reply to `parent`) shows: the words, and whether the
/// passage changed since.
pub(crate) fn anchor_quote(anchor: &SpanAnchor, parent: Option<&Post>) -> (String, bool) {
    match parent {
        Some(parent) if parent.is_locked() => (LOCKED_PLACEHOLDER.to_string(), false),
        Some(parent) => (anchor.quote.clone(), anchor.place(&parent.content) == AnchorPlace::Gone),
        None => (anchor.quote.clone(), false),
    }
}

/// The quote line of a margin reply; `None` for every other post.
pub(super) fn render_anchor_line<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens) -> Option<Div> {
    let anchor = post.extras.anchor.as_ref()?;
    let parent = post.reply_to_id.as_ref().and_then(|id| lookup.find_post(id.as_str()));
    let (quote, changed) = anchor_quote(anchor, parent);
    let words = div().italic().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(t!("post.margin.quote", quote = quote).to_string());
    let note = changed.then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("post.margin.changed").to_string()));
    let line = div()
        .w_full()
        .pl(theme.spacing_sm)
        .border_l(theme.border_width_threadline)
        .border_color(theme.accent_dialectic)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(words)
        .children(note);
    Some(line)
}
