//! Post body renderer: word-level flex wrapping of text, wikilink pills, and hashtags.
//!
//! Feed cards render the tokens the store cached at insert time; clicking a hashtag or a
//! wikilink pill dispatches to [`BodyLinks`] instead of selecting the surrounding card.

use super::consts::{CARRIAGE_RETURN, LINE_BREAK};
use super::lookup::PostLookup;
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_hashtag_span, render_obsidian_pill};
use gpui_kit::{
    div, AnyElement, App, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, Styled, Window,
};
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
            return body_text(theme).w_full().child(text.clone()).into_any_element();
        }
    }
    let mut elements = Vec::with_capacity(tokens.len());
    for token in tokens {
        match token {
            BodyToken::Text(text) => push_text_lines(text, theme, &mut elements),
            BodyToken::Wikilink { target, trailing } => {
                let pill = render_obsidian_pill(&lookup.resolve_wikilink(target), theme);
                let pill = clickable(pill, target, links.on_wikilink.as_ref());
                elements.push(with_trailing(pill, trailing, theme));
            }
            BodyToken::Hashtag { tag, trailing } => {
                let span = clickable(render_hashtag_span(tag, theme), tag, links.on_tag.as_ref());
                elements.push(with_trailing(span, trailing, theme));
            }
        }
    }
    div()
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.spacing_xs)
        .gap_y(theme.spacing_2xs)
        .children(elements)
        .into_any_element()
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

/// Emits text lines, inserting full-width breaks (and paragraph spacers for blank lines).
fn push_text_lines(text: &str, theme: &ThemeTokens, elements: &mut Vec<AnyElement>) {
    for (index, line) in text.split(LINE_BREAK).enumerate() {
        let clean_line = line.trim_end_matches(CARRIAGE_RETURN);
        if index > 0 {
            let line_break = if clean_line.trim().is_empty() {
                div().w_full().h(theme.paragraph_break_height)
            } else {
                div().w_full()
            };
            elements.push(line_break.into_any_element());
        }
        if !clean_line.is_empty() {
            elements.push(body_text(theme).child(clean_line.to_string()).into_any_element());
        }
    }
}
