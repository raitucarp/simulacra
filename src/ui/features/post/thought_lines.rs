//! N16–N20: the lines under a post's author after the N3–N15 ones: the lens a reply was
//! written through and the lens fan of a post (N16), the term a post defines (N17), and whether a question is open (N18). Each says nothing past a lock that the
//! placeholder would not.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::Div;

/// The N16–N20 lines `post` shows under its author (none for a plain post).
pub(super) fn thought_lines<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, _handlers: &PostCardHandlers) -> Vec<Div> {
    let lines = [super::lens_chip::render_lens_chip(post, theme), super::lens_chip::render_fan_strip(post, lookup, theme), super::term_chip::render_term_chip(post, theme), super::open_chip::render_open_chip(post, theme)];
    lines.into_iter().flatten().collect()
}
