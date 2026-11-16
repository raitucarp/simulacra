//! Render implementation of the isolated inline composer view.

use super::super::avatar::composer_avatar;
use super::super::consts::{
    HASHTAG_SNIPPET, INLINE_FALLBACK_INITIAL, INLINE_TEXTAREA_WRAPPER_ID, INSERT_LINK_BTN_ID,
    INSERT_TAG_BTN_ID, SUBMIT_POST_BTN_ID, WIKILINK_OPEN_SNIPPET,
};
use super::super::keys::{is_submit_chord, SubmitPostDraft};
use super::layout::{avatar_column, composer_row, editor_column, toolbar};
use super::PostComposerView;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Textarea, TextareaState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

impl Render for PostComposerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let avatar = composer_avatar(
            self.profile.as_ref(),
            INLINE_FALLBACK_INITIAL,
            theme.avatar_size_lg,
            theme,
        );
        let actions = toolbar(
            insert_snippet_button(INSERT_LINK_BTN_ID, LucideIcon::Link2, WIKILINK_OPEN_SNIPPET, &self.composer_input),
            insert_snippet_button(INSERT_TAG_BTN_ID, LucideIcon::Hash, HASHTAG_SNIPPET, &self.composer_input),
            self.submit_button(cx),
            self.draft_length,
            theme,
        );
        composer_row(
            avatar_column(avatar, theme),
            editor_column(self.textarea(cx), actions, theme),
            theme,
        )
    }
}

impl PostComposerView {
    fn submit_button(&self, cx: &mut Context<Self>) -> Button {
        Button::new(SUBMIT_POST_BTN_ID)
            .primary()
            .small()
            .disabled(self.draft_length == 0)
            .label(t!("composer.post_button").to_string())
            .on_click(cx.listener(|this, _, window, cx| {
                this.submit(window, cx);
            }))
    }

    /// Textarea wrapper that submits on the action binding or a Ctrl/Cmd+Enter keystroke.
    fn textarea(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .id(INLINE_TEXTAREA_WRAPPER_ID)
            .w_full()
            .on_action(cx.listener(|this, _: &SubmitPostDraft, window, cx| {
                this.submit(window, cx);
            }))
            .capture_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                if is_submit_chord(&event.keystroke) {
                    window.prevent_default();
                    cx.stop_propagation();
                    this.submit(window, cx);
                }
            }))
            .child(Textarea::new(&self.composer_input).bordered(false))
    }
}

/// Ghost button that appends `snippet` to the draft and refocuses the textarea.
fn insert_snippet_button(
    id: &'static str,
    icon: LucideIcon,
    snippet: &'static str,
    composer_input: &Entity<TextareaState>,
) -> Button {
    let entity = composer_input.clone();
    Button::new(id)
        .ghost()
        .small()
        .icon(Icon::new(icon))
        .on_click(move |_, window, cx| {
            entity.update(cx, |state, cx| {
                let cur = state.value().to_string();
                state.set_value(format!("{cur}{snippet}"), window, cx);
                state.focus(window, cx);
            });
        })
}
