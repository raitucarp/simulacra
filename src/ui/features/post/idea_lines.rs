//! N3, N6–N10, N14: the lines under a post's author, in order: its retraction or correction (N14), its garden stage (N3), the
//! what-if branch it starts (N6) who may read it (N8) who it was asked of (N9) and its place in a series (N10). Each says nothing past a lock that
//! the placeholder would not.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::Div;

/// The lines `post` shows under its author (none for a plain post).
pub(super) fn idea_lines<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Vec<Div> {
    let lines = [
        super::correction_banner::render_correction_banner(post, theme), // N14
        super::garden_badge::render_garden_badge(post, theme),
        super::branch_chip::render_branch_chip(post, theme),
        super::audience_badge::render_audience_badge(post, lookup, theme),
        super::question_chip::render_question_chip(post, lookup, theme),
        super::series_strip::render_series_strip(post, lookup, theme, handlers),
    ];
    lines.into_iter().flatten().collect()
}
