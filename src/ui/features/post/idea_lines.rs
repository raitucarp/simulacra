//! N3, N6–N10, N13–N15 and N16–N20: the lines under a post's author. What asks for attention
//! keeps a line of its own: a retraction or correction (N14), "Still true?" on a due claim
//! (N13), the series strip (N10), a change of mind and a translation (N19, N20). The quiet
//! marks of a few words each (garden stage N3, what-if N6, audience N8, asked of N9, distilled
//! N15, lens, term, open question, revised N16–N19) share one wrapping line, so five marks
//! cost one line above the text instead of five. Each says nothing past a lock that the
//! placeholder would not.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::{div, Div, ParentElement, Styled};

/// The lines `post` shows under its author (none for a plain post).
pub(super) fn idea_lines<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Vec<Div> {
    let alerts = [
        super::correction_banner::render_correction_banner(post, theme), // N14
        super::still_true_strip::render_still_true_strip(post, theme, handlers), // N13
    ];
    let marks: Vec<Div> = [
        super::garden_badge::render_garden_badge(post, theme),
        super::branch_chip::render_branch_chip(post, theme),
        super::audience_badge::render_audience_badge(post, lookup, theme),
        super::question_chip::render_question_chip(post, lookup, theme),
        super::distill_chip::render_distill_chip(post, theme), // N15
    ]
    .into_iter()
    .flatten()
    .chain(super::thought_lines::thought_marks(post, lookup, theme, handlers))
    .collect();
    let mark_line = (!marks.is_empty()).then(|| div().w_full().min_w_0().flex().flex_wrap().items_center().gap_x(theme.spacing_md).gap_y(theme.spacing_3xs).children(marks));
    let series = super::series_strip::render_series_strip(post, lookup, theme, handlers); // N10
    alerts.into_iter().flatten().chain(mark_line).chain(series).chain(super::thought_lines::thought_banners(post, lookup, theme, handlers)).collect()
}
