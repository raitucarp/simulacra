//! N1: an embedded post inside a body: its author, the version it is pinned to (with the
//! version it shows now when that moved on) and that version's text. A locked post shows
//! only its placeholder, even while it is unlocked this session; an embed inside an embed
//! is a link, so embeds never nest.

use super::super::lookup::PostLookup;
use super::inline::clickable;
use super::{render_body_tokens, BodyLinks};
use crate::domain::data::history::SemVer;
use crate::domain::post::tokenize_post_body;
use crate::theme::ThemeTokens;
use crate::ui::features::composer::version_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// What an embed shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EmbedView {
    /// No such post.
    Missing,
    /// The post is locked: its placeholder only.
    Locked { author: String },
    Shown { author: String, pinned: SemVer, now: SemVer, text: Rc<str>, kept: bool },
}

/// What the embed of `target` at `version` shows.
pub(crate) fn embed_view<L: PostLookup>(target: &str, version: Option<SemVer>, lookup: L) -> EmbedView {
    let Some(post) = lookup.find_post(target) else { return EmbedView::Missing };
    let author = lookup.author_of(post).name.clone();
    if post.is_locked() {
        return EmbedView::Locked { author };
    }
    let now = post.extras.head.as_ref().map_or_else(SemVer::default, |head| head.version);
    let pinned = version.unwrap_or(now);
    match lookup.version_text(target, Some(pinned)) {
        Some(text) => EmbedView::Shown { author, pinned, now, text, kept: true },
        None => EmbedView::Shown { author, pinned, now, text: Rc::from(post.content.as_str()), kept: false },
    }
}

fn muted(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text)
}

fn header(author: &str, versions: Option<(SemVer, SemVer)>, theme: &ThemeTokens) -> Div {
    let name = div().font_weight(FontWeight::SEMIBOLD).text_size(theme.font_size_xs).text_color(theme.text_primary).child(author.to_string());
    let version = versions.map(|(pinned, _)| muted(version_label(pinned), theme).font_family(theme.font_family_mono.clone()));
    let moved = versions.filter(|(pinned, now)| pinned != now).map(|(_, now)| muted(t!("post.embed.now", version = version_label(now)).to_string(), theme));
    div().flex().items_center().gap(theme.inline_gap).child(Icon::new(LucideIcon::Layers).xsmall().text_color(theme.text_muted)).child(name).children(version).children(moved)
}

/// The card of one embed (the `index`-th item of the body).
pub(super) fn embed_card<L: PostLookup>(index: usize, target: &str, version: Option<SemVer>, lookup: L, theme: &ThemeTokens, links: &BodyLinks) -> AnyElement {
    let (head, body) = match embed_view(target, version, lookup) {
        EmbedView::Missing => return muted(t!("post.embed.unavailable", id = target).to_string(), theme).into_any_element(),
        EmbedView::Locked { author } => (header(&author, None, theme), super::super::render_locked_placeholder(None, theme)),
        EmbedView::Shown { author, pinned, now, text, kept } => {
            let inner = BodyLinks { nested: true, code: None, ..links.scoped(format!("{}-embed-{index}", links.scope)) };
            let tokens = tokenize_post_body(&text);
            let body = div().w_full().flex().flex_col().gap(theme.spacing_3xs).when(!kept, |b| b.child(muted(t!("post.embed.not_kept", version = version_label(pinned)).to_string(), theme))).child(render_body_tokens(&tokens, lookup, theme, &inner));
            (header(&author, Some((pinned, now)), theme), body.into_any_element())
        }
    };
    let hover = theme.border_strong;
    let card = div()
        .id(SharedString::from(format!("post-embed-{}-{index}", links.scope)))
        .w_full()
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .hover(move |card| card.border_color(hover))
        .cursor_pointer()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(head)
        .child(body);
    div().w_full().child(clickable(card, target, links.on_wikilink.as_ref())).into_any_element()
}
