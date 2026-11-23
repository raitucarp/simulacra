//! N3, N6–N10: the quiet lines under a post's author, in order: its garden stage (N3), the
//! what-if branch it starts (N6) and who may read it (N8). Each says nothing past a lock that
//! the placeholder would not.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::Div;

/// The lines `post` shows under its author (none for a plain post).
pub(super) fn idea_lines<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, _handlers: &PostCardHandlers) -> Vec<Div> {
    let lines = [
        super::garden_badge::render_garden_badge(post, theme),
        super::branch_chip::render_branch_chip(post, theme),
        super::audience_badge::render_audience_badge(post, lookup, theme),
    ];
    lines.into_iter().flatten().collect()
}
