//! Post body renderer: word-level flex wrapping of marked text, wikilink pills, hashtags,
//! inline code and fenced code blocks.
//!
//! Feed cards render the tokens the store cached at insert time; clicking a hashtag or a
//! wikilink pill dispatches to [`BodyLinks`] instead of selecting the surrounding card.

mod layout;
mod marks;

use super::consts::LINE_BREAK;
use super::lookup::PostLookup;
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_hashtag_span, render_obsidian_pill};
use gpui_kit::{
    div, AnyElement, App, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, Window, relative,
};
use layout::{blank_line_after, layout, Item};
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
}

/// Tokenizes `content` and renders it with inert links (previews, tests, the spark feed).
pub fn render_post_body<L: PostLookup>(content: &str, lookup: L, theme: &ThemeTokens) -> impl IntoElement {
    render_body_tokens(&tokenize_post_body(content), lookup, theme, &BodyLinks::default())
}

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
    let items = layout(tokens);
    let elements: Vec<AnyElement> =
        items.iter().enumerate().map(|(index, item)| render_item(&items, index, item, &lookup, theme, links)).collect();
    div()
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.inline_gap)
        .gap_y_0()
        .children(elements)
        .into_any_element()
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
        Item::Block { code, .. } => marks::code_block(code, theme),
        Item::Wikilink { target, trailing } => {
            let pill = render_obsidian_pill(&lookup.resolve_wikilink(target), theme);
            with_trailing(clickable(pill, target, links.on_wikilink.as_ref()), trailing, theme)
        }
        Item::Hashtag { tag, trailing } => {
            let span = clickable(render_hashtag_span(tag, theme), tag, links.on_tag.as_ref());
            with_trailing(span, trailing, theme)
        }
    }
}

/// Makes an inline link dispatch `action(value)` on press without selecting the card.
fn clickable(element: impl IntoElement, value: &str, action: Option<&OpenTextAction>) -> AnyElement {
    let Some(action) = action.cloned() else {
        return element.into_any_element();
    };
    let value = value.to_string();
    div()
        .child(element)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
            cx.stop_propagation();
            action(value.clone(), window, cx);
        })
        .into_any_element()
}

/// Base body prose text style.
fn body_text(theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_body.clone())
        .text_size(theme.font_size_md)
        .text_color(theme.text_primary)
        .line_height(relative(theme.line_height_body))
}

/// Wraps an inline element with its trailing punctuation so they never wrap apart.
fn with_trailing(element: AnyElement, trailing: &str, theme: &ThemeTokens) -> AnyElement {
    if trailing.is_empty() {
        return element;
    }
    div()
        .flex()
        .items_center()
        .gap_0()
        .child(element)
        .child(body_text(theme).child(trailing.to_string()))
        .into_any_element()
}

#[cfg(test)]
mod tests;
