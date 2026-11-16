//! Standalone composer renderer for hosts that do not mount the `PostComposerView` entity.

use super::super::consts::INLINE_FALLBACK_INITIAL;
use super::layout::{avatar_column, composer_row, editor_column, toolbar};
use crate::domain::{Profile, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_avatar;
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::*;

/// Fallback standalone renderer for components that do not mount the View entity directly.
pub fn render_post_composer(
    profile: Option<&Profile>,
    composer_input: &Entity<TextareaState>,
    draft_length: usize,
    theme: &ThemeTokens,
    link_button: impl IntoElement,
    tag_button: impl IntoElement,
    submit_button: impl IntoElement,
) -> impl IntoElement {
    let initial = profile
        .map(|p| p.avatar_initial.as_str())
        .unwrap_or(INLINE_FALLBACK_INITIAL);
    let role = profile.map(|p| &p.role).unwrap_or(&ProfileRole::UserPrimary);
    let avatar = render_avatar(initial, role, theme.avatar_size_lg, theme);
    let editor = div()
        .w_full()
        .child(Textarea::new(composer_input).bordered(false));
    let actions = toolbar(link_button, tag_button, submit_button, draft_length, theme);
    composer_row(
        avatar_column(avatar, theme),
        editor_column(editor, actions, theme),
        theme,
    )
}
