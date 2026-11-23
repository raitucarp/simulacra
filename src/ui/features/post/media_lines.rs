//! N11, N12: what a post carries besides its text and attachments, under its body: its voice
//! note (N11) and its drawing (N12). Neither shows on a locked post.

use super::controls::PostCardHandlers;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use gpui_kit::Div;

/// The voice note and drawing cards of `post` (none for a plain post).
pub(super) fn media_lines(post: &Post, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Vec<Div> {
    [super::voice_card::render_voice_card(post, theme, handlers), super::ink_card::render_ink_card(post, theme)].into_iter().flatten().collect()
}
