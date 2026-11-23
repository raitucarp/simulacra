//! N7: a post's pinned quote under its text: the passage word for word, the document it is
//! from, and a flag once that document has changed (still there, gone, or the document lost).
//! A locked post shows no quote at all: only its placeholder.

use super::lookup::PostLookup;
use crate::domain::post::pinned::QuoteDrift;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// What the card shows: the passage, its source line, and the flag's text when it drifted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PinnedView {
    pub passage: String,
    pub source: String,
    pub flag: Option<String>,
}

/// The view of `post`'s pinned quote at `drift` (`None` when it has none or is locked).
pub(crate) fn pinned_view(post: &Post, drift: Option<QuoteDrift>) -> Option<PinnedView> {
    let quote = post.extras.pinned.as_ref().filter(|_| !post.is_locked())?;
    let flag = drift.filter(|drift| drift.flagged()).map(|drift| {
        let key = format!("posts.pinned.drift.{}", drift.key());
        t!(&key).to_string()
    });
    Some(PinnedView { passage: quote.passage.clone(), source: t!("posts.pinned.source", title = quote.title.clone()).to_string(), flag })
}

/// The card; `None` for a post without a pinned quote, and for a locked one.
pub(super) fn render_pinned_quote<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens) -> Option<Div> {
    let view = pinned_view(post, lookup.store().and_then(|state| state.quote_drift(post)))?;
    let edge = if view.flag.is_some() { theme.warning } else { theme.accent_brand };
    let passage = div().italic().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(t!("post.margin.quote", quote = view.passage).to_string());
    let source = div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(Icon::new(LucideIcon::Quote).xsmall().text_color(theme.text_muted)).child(view.source);
    let flag = view.flag.map(|text| div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.warning).child(Icon::new(LucideIcon::CircleAlert).xsmall().text_color(theme.warning)).child(text));
    let card = div()
        .w_full()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border_l(theme.border_width_threadline)
        .border_color(edge)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(passage)
        .child(source)
        .children(flag);
    Some(card)
}
