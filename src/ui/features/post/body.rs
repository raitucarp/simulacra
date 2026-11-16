//! Post body renderer: word-level flex wrapping of text, wikilink pills, and hashtags.

use super::consts::{CARRIAGE_RETURN, LINE_BREAK};
use super::lookup::PostLookup;
use crate::domain::post::{has_rich_markup, tokenize_post_body, BodyToken};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_hashtag_span, render_obsidian_pill};
use gpui_kit::{div, AnyElement, Div, IntoElement, ParentElement, Styled};

/// Parses post content into structured inline elements (tokenized words, obsidian links, hashtags)
/// with seamless word-level flex wrapping preventing text clipping at panel edges.
pub fn render_post_body<L: PostLookup>(content: &str, lookup: L, theme: &ThemeTokens) -> impl IntoElement {
    if !has_rich_markup(content) {
        return body_text(theme).w_full().child(content.to_string()).into_any_element();
    }

    let mut elements = Vec::new();
    for token in tokenize_post_body(content) {
        match token {
            BodyToken::Text(text) => push_text_lines(&text, theme, &mut elements),
            BodyToken::Wikilink { target, trailing } => {
                let display_title = lookup.resolve_wikilink(&target);
                let pill = render_obsidian_pill(&display_title, theme);
                elements.push(with_trailing(pill, trailing, theme));
            }
            BodyToken::Hashtag { tag, trailing } => {
                let span = render_hashtag_span(&tag, theme);
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

/// Base body prose text style.
fn body_text(theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_body.clone())
        .text_size(theme.font_size_md)
        .text_color(theme.text_primary)
}

/// Wraps an inline element with its trailing punctuation so they never wrap apart.
fn with_trailing(element: impl IntoElement, trailing: String, theme: &ThemeTokens) -> AnyElement {
    if trailing.is_empty() {
        return element.into_any_element();
    }
    div()
        .flex()
        .items_center()
        .gap_0()
        .child(element)
        .child(body_text(theme).child(trailing))
        .into_any_element()
}

/// Emits text lines, inserting full-width breaks (and paragraph spacers for blank lines).
fn push_text_lines(text: &str, theme: &ThemeTokens, elements: &mut Vec<AnyElement>) {
    for (index, line) in text.split(LINE_BREAK).enumerate() {
        let clean_line = line.trim_end_matches(CARRIAGE_RETURN);
        if index > 0 {
            let line_break = if clean_line.trim().is_empty() {
                div().w_full().h(theme.spacing_sm)
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
