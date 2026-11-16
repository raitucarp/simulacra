//! Context-aware dialectic threadline style and the avatar column it decorates.

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use gpui_kit::{div, Div, ParentElement, Rgba, Styled};

/// Context-aware dialectic threadline rendering style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThreadlineStyle {
    /// Standalone thought with no threadline connections.
    #[default]
    None,
    /// Root/parent thought in a thread; line extends from below avatar down to bottom of card.
    ParentHasReplies,
    /// Reply in middle of a thread; continuous vertical line entering from top, passing avatar, and extending to bottom.
    MiddleReply,
    /// Final reply in a thread; line enters from top of card and terminates at the avatar.
    LastReply,
}

impl ThreadlineStyle {
    #[inline]
    pub fn has_top_line(&self) -> bool {
        matches!(self, Self::MiddleReply | Self::LastReply)
    }

    #[inline]
    pub fn has_bottom_line(&self) -> bool {
        matches!(self, Self::ParentHasReplies | Self::MiddleReply)
    }
}

/// Visible threadline color when connected, transparent otherwise.
fn line_color(connected: bool, theme: &ThemeTokens) -> Rgba {
    if connected {
        theme.border_subtle
    } else {
        theme.bg_transparent
    }
}

/// Left column: author avatar with continuous threadline segments (zero-gap card boundary connection).
pub(crate) fn render_avatar_column(
    author: &Profile,
    style: ThreadlineStyle,
    has_repost_header: bool,
    theme: &ThemeTokens,
) -> Div {
    // Top segment also spans the repost header row so the line meets the avatar.
    let top_height = if has_repost_header {
        theme.spacing_md + theme.font_size_xs + theme.spacing_2xs
    } else {
        theme.spacing_md
    };
    div()
        .flex_shrink_0()
        .w(theme.post_avatar_col_width)
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        // Top line segment entering from card top boundary
        .child(
            div()
                .w(theme.border_width_threadline)
                .h(top_height)
                .bg(line_color(style.has_top_line(), theme)),
        )
        .child(render_profile_avatar(author, theme.avatar_size_lg, theme))
        // Bottom line segment extending to card bottom boundary
        .child(
            div()
                .w(theme.border_width_threadline)
                .flex_1()
                .min_h(theme.threadline_min_height)
                .mt(theme.spacing_xs)
                .bg(line_color(style.has_bottom_line(), theme)),
        )
}
