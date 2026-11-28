//! N16–N20: what a post shows under its author after the N3–N15 lines: the lens a reply was
//! written through and the lens fan of a post (N16), the term a post defines (N17), whether a
//! question is open (N18), a change of mind or a later revision (N19), and the language a post
//! is shown in (N20). Each says nothing past a lock that the placeholder would not.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::Div;

/// The quiet N16–N19 marks of `post` (a few words each), shown together on one wrapping line.
pub(super) fn thought_marks<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Vec<Div> {
    let marks = [
        super::mind_banner::render_revised_chip(post, lookup, theme, handlers),
        super::lens_chip::render_lens_chip(post, theme),
        super::lens_chip::render_fan_strip(post, lookup, theme),
        super::term_chip::render_term_chip(post, theme),
        super::open_chip::render_open_chip(post, theme),
    ];
    marks.into_iter().flatten().collect()
}

/// The N19–N20 banners of `post` (sentences or buttons), each on its own line.
pub(super) fn thought_banners<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Vec<Div> {
    let banners = [super::mind_banner::render_mind_banner(post, lookup, theme, handlers), super::translation_line::render_translation_line(post, theme, handlers)];
    banners.into_iter().flatten().collect()
}
