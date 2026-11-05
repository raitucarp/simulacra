//! Threads-style inline composer molecule.
//! Fully tokenized via ThemeTokens.
//! Features isolated View rendering for 120 FPS sub-millisecond typing latency.

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_char_meter, render_profile_avatar};
use crate::ui::icons::LucideIcon;
use gpui_kit::actions;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{InputEvent, Textarea, TextareaState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

actions!(simulacra, [SubmitPostDraft]);

pub const KEY_ENTER: &str = "enter";
pub const KEY_ENTER_CR: &str = "\r";
pub const KEY_ENTER_LF: &str = "\n";

pub type SubmitPostCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;

/// Isolated GPUI View component for the composer.
/// Renders independently without triggering full-window `AppView` re-renders on keystrokes.
pub struct PostComposerView {
    profile: Option<Profile>,
    composer_input: Entity<TextareaState>,
    theme: ThemeTokens,
    draft_length: usize,
    on_submit: SubmitPostCallback,
    _sub: Subscription,
}

impl PostComposerView {
    pub fn new(
        profile: Option<Profile>,
        composer_input: Entity<TextareaState>,
        theme: ThemeTokens,
        on_submit: impl Fn(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let initial_len = composer_input.read(cx).value().chars().count();
        cx.bind_keys([
            KeyBinding::new("ctrl-enter", SubmitPostDraft, Some("Input")),
            KeyBinding::new("cmd-enter", SubmitPostDraft, Some("Input")),
        ]);
        let sub = cx.subscribe_in(&composer_input, window, |this, entity, event: &InputEvent, window, cx| {
            match event {
                InputEvent::Change => {
                    let current_val = entity.read(cx).value();
                    let new_len = current_val.chars().count();
                    if this.draft_length != new_len {
                        this.draft_length = new_len;
                        cx.notify();
                    }
                }
                InputEvent::PressEnter { secondary: true, .. } => {
                    this.submit(window, cx);
                }
                _ => {}
            }
        });

        Self {
            profile,
            composer_input,
            theme,
            draft_length: initial_len,
            on_submit: Box::new(on_submit),
            _sub: sub,
        }
    }

    /// Submits the current draft content.
    pub fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.composer_input.read(cx).value().to_string();
        if !content.trim().is_empty() {
            (self.on_submit)(content, window, cx);
        }
    }

    pub fn set_profile(&mut self, profile: Option<&Profile>, cx: &mut Context<Self>) {
        if self.profile.as_ref() != profile {
            self.profile = profile.cloned();
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    pub fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
            state.focus(window, cx);
        });
        self.draft_length = 0;
        cx.notify();
    }
}

impl Render for PostComposerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let initial = self
            .profile
            .as_ref()
            .map(|p| p.avatar_initial.as_str())
            .unwrap_or("Y");
        let role = self
            .profile
            .as_ref()
            .map(|p| &p.role)
            .unwrap_or(&crate::domain::ProfileRole::UserPrimary);

        let is_draft_empty = self.draft_length == 0;

        let entity_link = self.composer_input.clone();
        let link_button = Button::new("insert-link-btn")
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Link2))
            .on_click(move |_, window, cx| {
                entity_link.update(cx, |state, cx| {
                    let cur = state.value().to_string();
                    state.set_value(format!("{} [[", cur), window, cx);
                    state.focus(window, cx);
                });
            });

        let entity_tag = self.composer_input.clone();
        let tag_button = Button::new("insert-tag-btn")
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Hash))
            .on_click(move |_, window, cx| {
                entity_tag.update(cx, |state, cx| {
                    let cur = state.value().to_string();
                    state.set_value(format!("{} #", cur), window, cx);
                    state.focus(window, cx);
                });
            });

        let submit_button = Button::new("submit-post-btn")
            .primary()
            .small()
            .disabled(is_draft_empty)
            .label(t!("composer.post_button").to_string())
            .on_click(cx.listener(|this, _, window, cx| {
                this.submit(window, cx);
            }));

        div()
            .w_full()
            .px(theme.spacing_lg)
            .py(theme.spacing_md)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_app)
            .flex()
            .gap(theme.spacing_md)
            // Left Column: Avatar + Threads vertical threadline
            .child(
                div()
                    .flex_shrink_0()
                    .w(theme.post_avatar_col_width)
                    .h_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .pt(theme.spacing_2xs)
                    .child(if let Some(ref prof) = self.profile {
                        render_profile_avatar(prof, theme.avatar_size_lg, theme)
                    } else {
                        render_avatar(initial, role, theme.avatar_size_lg, theme)
                    })
                    .child(
                        div()
                            .w(theme.border_width_threadline)
                            .flex_1()
                            .min_h(theme.threadline_min_height)
                            .bg(theme.border_subtle)
                            .rounded(theme.radius_full)
                            .mt(theme.spacing_xs)
                            .mb(theme.spacing_2xs),
                    ),
            )
            // Right Column: Textarea & Minimal Actions
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_xs)
                    .child(
                        div()
                            .id("composer-textarea-wrapper")
                            .w_full()
                            .on_action(cx.listener(|this, _: &SubmitPostDraft, window, cx| {
                                this.submit(window, cx);
                            }))
                            .capture_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                                let stroke = &event.keystroke;
                                let is_enter = stroke.key == KEY_ENTER || stroke.key == KEY_ENTER_CR || stroke.key == KEY_ENTER_LF;
                                let is_ctrl = stroke.modifiers.control || (cfg!(target_os = "macos") && stroke.modifiers.platform);
                                if is_enter && is_ctrl {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                    this.submit(window, cx);
                                }
                            }))
                            .child(Textarea::new(&self.composer_input).bordered(false)),
                    )
                    // Bottom Toolbar
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_between()
                            .pt(theme.spacing_xs)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(theme.spacing_xs)
                                    .child(link_button)
                                    .child(tag_button),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(theme.spacing_md)
                                    .child(render_char_meter(self.draft_length, theme))
                                    .child(submit_button),
                            ),
                    ),
            )
    }
}

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
    let initial = profile.map(|p| p.avatar_initial.as_str()).unwrap_or("Y");
    let role = profile
        .map(|p| &p.role)
        .unwrap_or(&crate::domain::ProfileRole::UserPrimary);

    div()
        .w_full()
        .px(theme.spacing_lg)
        .py(theme.spacing_md)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_app)
        .flex()
        .gap(theme.spacing_md)
        // Left Column: Avatar + Threads vertical threadline
        .child(
            div()
                .flex_shrink_0()
                .w(theme.post_avatar_col_width)
                .h_full()
                .flex()
                .flex_col()
                .items_center()
                .pt(theme.spacing_2xs)
                .child(render_avatar(initial, role, theme.avatar_size_lg, theme))
                .child(
                    div()
                        .w(theme.border_width_threadline)
                        .flex_1()
                        .min_h(theme.threadline_min_height)
                        .bg(theme.border_subtle)
                        .rounded(theme.radius_full)
                        .mt(theme.spacing_xs)
                        .mb(theme.spacing_2xs),
                ),
        )
        // Right Column: Textarea & Minimal Actions
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_xs)
                .child(
                    div()
                        .w_full()
                        .child(Textarea::new(composer_input).bordered(false)),
                )
                // Bottom Toolbar
                .child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .pt(theme.spacing_xs)
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_xs)
                                .child(link_button)
                                .child(tag_button),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(theme.spacing_md)
                                .child(render_char_meter(draft_length, theme))
                                .child(submit_button),
                        ),
                ),
        )
}
