//! Post body renderer: word-level flex wrapping of marked text, web links, wikilink pills, hashtags,
//! inline code, spoilers and fenced code blocks.
//!
//! Feed cards render the tokens the store cached at insert time; clicking a hashtag or a
//! wikilink pill dispatches to [`BodyLinks`] instead of selecting the surrounding card.

mod code_block;
mod inline;
mod layout;
mod link;
mod marks;
mod spoiler;

use super::consts::LINE_BREAK;
use super::lookup::PostLookup;
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_hashtag_span, render_obsidian_pill};
use gpui_kit::{div, AnyElement, App, IntoElement, ParentElement, Styled, Window};
use inline::{body_text, clickable, with_trailing};
use layout::{blank_line_after, layout, Item};
use spoiler::{hidden_spoiler, is_revealed, revealed_spoiler, SpoilerKey};
use std::rc::Rc;

/// Opens the hashtag or wikilink target named by the string.
pub type OpenTextAction = Rc<dyn Fn(String, &mut Window, &mut App) + 'static>;

/// Click targets of the inline body links (absent handlers leave them inert).
#[derive(Clone, Default)]
pub struct BodyLinks {
    /// Hashtag click, with the tag including its `#`.
    pub on_tag: Option<OpenTextAction>,
    /// Wikilink pill click, with the raw `[[target]]`.
    pub on_wikilink: Option<OpenTextAction>,
    /// Source chip click (vault document, post, profile or web page an agent drew on).
    pub on_source: Option<super::extras::OpenSourceAction>,
    /// Image click: opens the lightbox on that image of the post (sb C3).
    pub on_media: Option<crate::ui::features::media::OpenMediaAction>,
    /// The post the body belongs to, which its spoilers' reveals are kept under.
    pub scope: String,
}

impl BodyLinks {
    /// These links for the body of post `scope`.
    pub fn scoped(&self, scope: impl Into<String>) -> Self {
        Self { scope: scope.into(), ..self.clone() }
    }
}

/// Tokenizes `content` and renders it with inert links (previews, tests, the spark feed); its
/// to-dos as a checklist written today (D5).
pub fn render_post_body<L: PostLookup>(content: &str, lookup: L, theme: &ThemeTokens) -> impl IntoElement {
    let ticks = crate::domain::todo::state::TodoState::default();
    let source = super::checklist::ChecklistSource { content, post_id: PREVIEW_SCOPE, written: crate::time::local_now().date(), ticks: &ticks, toggle: None };
    super::checklist::render_checklist(&source, lookup, theme, &BodyLinks::default()).unwrap_or_else(|| render_body_tokens(&tokenize_post_body(content), lookup, theme, &BodyLinks::default()))
}

/// The element id scope of a checklist that belongs to no post.
const PREVIEW_SCOPE: &str = "preview";

/// Renders pre-tokenized body content with clickable hashtags and wikilinks.
pub fn render_body_tokens<L: PostLookup>(
    tokens: &[BodyToken],
    lookup: L,
    theme: &ThemeTokens,
    links: &BodyLinks,
) -> AnyElement {
    if let [BodyToken::Text(text)] = tokens {
        if !text.contains(LINE_BREAK) {
            return body_text(theme).w_full().min_w_0().child(text.clone()).into_any_element();
        }
    }
    div()
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.inline_gap)
        .gap_y_0()
        .children(render_items(&layout(tokens), &lookup, theme, links))
        .into_any_element()
}

fn render_items<L: PostLookup>(items: &[Item<'_>], lookup: &L, theme: &ThemeTokens, links: &BodyLinks) -> Vec<AnyElement> {
    items.iter().enumerate().map(|(index, item)| render_item(items, index, item, lookup, theme, links)).collect()
}

/// One inline item as an element; breaks become full-width rows (taller before a blank line).
fn render_item<L: PostLookup>(
    items: &[Item<'_>],
    index: usize,
    item: &Item<'_>,
    lookup: &L,
    theme: &ThemeTokens,
    links: &BodyLinks,
) -> AnyElement {
    match item {
        Item::Word { text, marks } => {
            body_text(theme).min_w_0().child(marks::styled_word(text.clone(), marks, theme)).into_any_element()
        }
        Item::Break if blank_line_after(items, index) => div().w_full().h(theme.paragraph_break_height).into_any_element(),
        Item::Break => div().w_full().into_any_element(),
        Item::Code(code) => marks::inline_code(code, theme),
        Item::Block { lang, code } => code_block::code_block(index, *lang, code, theme),
        Item::Wikilink { target, trailing } => {
            let pill = render_obsidian_pill(&lookup.resolve_wikilink(target), theme);
            with_trailing(clickable(pill, target, links.on_wikilink.as_ref()), trailing, theme)
        }
        Item::Url { url, trailing } => with_trailing(link::link_span(index, url, theme), trailing, theme),
        Item::Hashtag { tag, trailing } => {
            let span = clickable(render_hashtag_span(tag, theme), tag, links.on_tag.as_ref());
            with_trailing(span, trailing, theme)
        }
        Item::Spoiler { ordinal, text, items: inner } => {
            let key = SpoilerKey { scope: links.scope.clone(), ordinal: *ordinal, text: text.clone() };
            match is_revealed(&key) {
                true => revealed_spoiler(render_items(inner, lookup, theme, links), theme),
                false => hidden_spoiler(index, key, theme),
            }
        }
    }
}

#[cfg(test)]
mod tests;
