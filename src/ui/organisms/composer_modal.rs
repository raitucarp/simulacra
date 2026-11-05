//! Threads-style composer modal dialog organism.
//! 100% tokenized via ThemeTokens adhering strictly to AGENTS.md.

use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_profile_avatar};
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::post_composer::{SubmitPostDraft, KEY_ENTER, KEY_ENTER_CR, KEY_ENTER_LF};
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;
use std::rc::Rc;

pub type ModalCloseCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalSubmitCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalActionCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalMode {
    NewThread,
    Reply(Box<Post>),
    Quote(Box<Post>),
}

#[derive(IntoElement)]
pub struct ComposerModalView {
    theme: ThemeTokens,
    profile: Option<Profile>,
    mode: ComposerModalMode,
    composer_input: Entity<TextareaState>,
    link_input: Entity<InputState>,
    image_input: Entity<InputState>,
    image_path: Option<String>,
    is_link_input_open: bool,
    is_image_input_open: bool,
    draft_length: usize,
    on_close: ModalCloseCallback,
    on_submit: ModalSubmitCallback,
    on_toggle_link: ModalActionCallback,
    on_toggle_image: ModalActionCallback,
    on_pick_image: ModalActionCallback,
    on_clear_image: ModalActionCallback,
    on_clear_link: ModalActionCallback,
}

impl ComposerModalView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        theme: ThemeTokens,
        profile: Option<Profile>,
        mode: ComposerModalMode,
        composer_input: Entity<TextareaState>,
        link_input: Entity<InputState>,
        image_input: Entity<InputState>,
        image_path: Option<String>,
        is_link_input_open: bool,
        is_image_input_open: bool,
        draft_length: usize,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
        on_submit: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_link: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_image: impl Fn(&mut Window, &mut App) + 'static,
        on_pick_image: impl Fn(&mut Window, &mut App) + 'static,
        on_clear_image: impl Fn(&mut Window, &mut App) + 'static,
        on_clear_link: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            theme,
            profile,
            mode,
            composer_input,
            link_input,
            image_input,
            image_path,
            is_link_input_open,
            is_image_input_open,
            draft_length,
            on_close: Rc::new(on_close),
            on_submit: Rc::new(on_submit),
            on_toggle_link: Rc::new(on_toggle_link),
            on_toggle_image: Rc::new(on_toggle_image),
            on_pick_image: Rc::new(on_pick_image),
            on_clear_image: Rc::new(on_clear_image),
            on_clear_link: Rc::new(on_clear_link),
        }
    }
}

impl RenderOnce for ComposerModalView {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        let on_close_backdrop = self.on_close;
        let on_close_cancel = on_close_backdrop.clone();
        let on_submit_btn = self.on_submit;
        let on_submit_key = on_submit_btn.clone();
        let on_submit_action = on_submit_btn.clone();
        let on_toggle_link = self.on_toggle_link;
        let on_toggle_image = self.on_toggle_image;
        let on_pick_image = self.on_pick_image;
        let on_pick_image_browse = on_pick_image.clone();
        let on_clear_image = self.on_clear_image;
        let on_clear_image_input = on_clear_image.clone();
        let on_clear_link = self.on_clear_link;

        let user_initial = self.profile.as_ref().map(|p| p.avatar_initial.as_str()).unwrap_or("U");
        let user_role = self.profile.as_ref().map(|p| &p.role).unwrap_or(&crate::domain::ProfileRole::UserPrimary);
        let user_handle = self.profile.as_ref().map(|p| p.display_handle()).unwrap_or_else(|| "@user".to_string());

        let title_text = match &self.mode {
            ComposerModalMode::NewThread => t!("composer.new_thread").to_string(),
            ComposerModalMode::Reply(parent) => format!("Reply to {}", parent.author.display_handle()),
            ComposerModalMode::Quote(target) => format!("Quote {}", target.author.display_handle()),
        };

        let manual_image_val = self.image_input.read(cx).value().trim().trim_matches('"').trim_matches('\'').to_string();
        let effective_image_path = self.image_path.clone().or_else(|| {
            if !manual_image_val.is_empty() && std::path::Path::new(&manual_image_val).exists() {
                Some(manual_image_val)
            } else {
                None
            }
        });

        let has_link = !self.link_input.read(cx).value().trim().is_empty();
        let has_image = effective_image_path.is_some();
        let can_post = match &self.mode {
            ComposerModalMode::NewThread | ComposerModalMode::Reply(_) => {
                self.draft_length > 0 || has_image || has_link
            }
            ComposerModalMode::Quote(_) => self.draft_length > 0,
        };

        div()
            .id("composer-modal-backdrop")
            .absolute()
            .inset_0()
            .bg(theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                (on_close_backdrop)(window, cx);
            })
            .child(
                div()
                    .id("composer-modal-dialog")
                    .w(theme.modal_composer_width)
                    .max_h(theme.modal_composer_max_height)
                    .rounded(theme.radius_lg)
                    .border(theme.border_width_hairline)
                    .border_color(theme.border)
                    .bg(theme.bg_elevated)
                    .shadow_2xl()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                        cx.stop_propagation();
                    })
                    // 1. Header: Cancel, Title, Options
                    .child(
                        div()
                            .w_full()
                            .h(theme.feed_header_height)
                            .px(theme.spacing_lg)
                            .border_b(theme.border_width_hairline)
                            .border_color(theme.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .id("modal-cancel-btn")
                                    .cursor_pointer()
                                    .text_size(theme.font_size_sm)
                                    .text_color(theme.text_secondary)
                                    .hover(move |s| s.text_color(theme.text_primary))
                                    .on_click(move |_, window, cx| {
                                        (on_close_cancel)(window, cx);
                                    })
                                    .child(t!("composer.cancel").to_string()),
                            )
                            .child(
                                div()
                                    .font_family(theme.font_family_display.clone())
                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                    .text_size(theme.font_size_base)
                                    .text_color(theme.text_primary)
                                    .child(title_text),
                            )
                            .child(
                                div()
                                    .text_color(theme.text_muted)
                                    .child(Icon::new(LucideIcon::MoreHorizontal)),
                            ),
                    )
                    // 2. Parent Context (if Reply mode)
                    .when_some(if let ComposerModalMode::Reply(ref parent) = self.mode { Some(parent) } else { None }, |this, parent| {
                        this.child(
                            div()
                                .w_full()
                                .px(theme.spacing_lg)
                                .pt(theme.spacing_md)
                                .flex()
                                .gap(theme.spacing_md)
                                .child(
                                    div()
                                        .w(theme.post_avatar_col_width)
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .child(render_profile_avatar(&parent.author, theme.avatar_size_sm, theme))
                                        .child(
                                            div()
                                                .w(theme.border_width_threadline)
                                                .flex_1()
                                                .min_h(theme.threadline_min_height)
                                                .bg(theme.border_subtle)
                                                .rounded(theme.radius_full)
                                                .mt(theme.spacing_2xs),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .flex()
                                        .flex_col()
                                        .gap(theme.spacing_2xs)
                                        .child(
                                            div()
                                                .font_family(theme.font_family_display.clone())
                                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                .text_size(theme.font_size_xs)
                                                .text_color(theme.text_secondary)
                                                .child(parent.author.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_size(theme.font_size_sm)
                                                .text_color(theme.text_muted)
                                                .truncate()
                                                .child(parent.content.clone()),
                                        ),
                                ),
                        )
                    })
                    // 3. Main Composer Input Row
                    .child(
                        div()
                            .w_full()
                            .px(theme.spacing_lg)
                            .py(theme.spacing_md)
                            .flex()
                            .gap(theme.spacing_md)
                            .child(
                                div()
                                    .w(theme.post_avatar_col_width)
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .child(if let Some(ref prof) = self.profile {
                                        render_profile_avatar(prof, theme.avatar_size_md, theme)
                                    } else {
                                        render_avatar(user_initial, user_role, theme.avatar_size_md, theme)
                                    }),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .min_w_0()
                                    .flex()
                                    .flex_col()
                                    .gap(theme.spacing_xs)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(theme.spacing_xs)
                                            .child(
                                                div()
                                                    .font_family(theme.font_family_display.clone())
                                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                    .text_size(theme.font_size_sm)
                                                    .text_color(theme.text_primary)
                                                    .child(user_handle),
                                            )
                                            .child(
                                                div()
                                                    .text_size(theme.font_size_xs)
                                                    .text_color(theme.text_muted)
                                                    .child(format!("> {}", t!("composer.community_topic"))),
                                            ),
                                    )
                                    // Textarea wrapper with Ctrl+Enter interceptor
                                    .child(
                                        div()
                                            .id("modal-composer-textarea-wrapper")
                                            .w_full()
                                            .on_action(move |_: &SubmitPostDraft, window, cx| {
                                                if can_post {
                                                    (on_submit_action)(window, cx);
                                                }
                                            })
                                            .capture_key_down(move |event: &KeyDownEvent, window, cx| {
                                                let stroke = &event.keystroke;
                                                let is_enter = stroke.key == KEY_ENTER || stroke.key == KEY_ENTER_CR || stroke.key == KEY_ENTER_LF;
                                                let is_ctrl = stroke.modifiers.control || (cfg!(target_os = "macos") && stroke.modifiers.platform);
                                                if is_enter && is_ctrl {
                                                    window.prevent_default();
                                                    cx.stop_propagation();
                                                    if can_post {
                                                        (on_submit_key)(window, cx);
                                                    }
                                                }
                                            })
                                            .child(Textarea::new(&self.composer_input).bordered(false)),
                                    )
                                    // Quoted Post Preview (if Quote mode)
                                    .when_some(if let ComposerModalMode::Quote(ref target) = self.mode { Some(target) } else { None }, |this, target| {
                                        this.child(
                                            div()
                                                .w_full()
                                                .p(theme.spacing_sm)
                                                .rounded(theme.radius_md)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.border)
                                                .bg(theme.bg_card)
                                                .flex()
                                                .flex_col()
                                                .gap(theme.spacing_2xs)
                                                .child(
                                                    div()
                                                        .font_family(theme.font_family_display.clone())
                                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                        .text_size(theme.font_size_xs)
                                                        .text_color(theme.text_primary)
                                                        .child(format!("{} {}", target.author.name, target.author.display_handle())),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(theme.font_size_sm)
                                                        .text_color(theme.text_secondary)
                                                        .child(target.content.clone()),
                                                ),
                                        )
                                    })
                                    // Dedicated Link Input Row (if open)
                                    .when(self.is_link_input_open, |this| {
                                        this.child(
                                            div()
                                                .w_full()
                                                .px(theme.spacing_sm)
                                                .py(theme.spacing_2xs)
                                                .rounded(theme.radius_md)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.border_strong)
                                                .bg(theme.bg_app)
                                                .flex()
                                                .items_center()
                                                .gap(theme.spacing_xs)
                                                .child(Icon::new(LucideIcon::Link2))
                                                .child(div().flex_1().child(Input::new(&self.link_input).appearance(false)))
                                                .child(
                                                    div()
                                                        .id("modal-clear-link-btn")
                                                        .cursor_pointer()
                                                        .text_color(theme.text_muted)
                                                        .hover(move |s| s.text_color(theme.text_primary))
                                                        .on_click(move |_, window, cx| (on_clear_link)(window, cx))
                                                        .child(Icon::new(LucideIcon::X)),
                                                ),
                                        )
                                    })
                                    // Dedicated Image Path Input Row (if open)
                                    .when(self.is_image_input_open, |this| {
                                        this.child(
                                            div()
                                                .id("modal-image-input-row")
                                                .w_full()
                                                .px(theme.spacing_sm)
                                                .py(theme.spacing_2xs)
                                                .rounded(theme.radius_md)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.border_strong)
                                                .bg(theme.bg_app)
                                                .flex()
                                                .items_center()
                                                .gap(theme.spacing_xs)
                                                .child(
                                                    div()
                                                        .text_color(theme.text_muted)
                                                        .child(Icon::new(LucideIcon::Image)),
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .child(Input::new(&self.image_input).appearance(false)),
                                                )
                                                .child(
                                                    Button::new("modal-browse-image-btn")
                                                        .ghost()
                                                        .small()
                                                        .icon(Icon::new(LucideIcon::Folder))
                                                        .tooltip(t!("composer.pick_image").to_string())
                                                        .on_click(move |_, window, cx| (on_pick_image_browse)(window, cx)),
                                                )
                                                .child(
                                                    div()
                                                        .id("modal-clear-image-input-btn")
                                                        .cursor_pointer()
                                                        .text_color(theme.text_muted)
                                                        .hover(move |s| s.text_color(theme.text_primary))
                                                        .on_click(move |_, window, cx| (on_clear_image_input)(window, cx))
                                                        .child(Icon::new(LucideIcon::X)),
                                                ),
                                        )
                                    })
                                    // Local Image Preview Card (when a valid local file path exists)
                                    .when_some(effective_image_path.as_ref(), |this, path| {
                                        this.child(
                                            div()
                                                .id("modal-image-preview-card")
                                                .w_full()
                                                .relative()
                                                .rounded(theme.radius_md)
                                                .border(theme.border_width_hairline)
                                                .border_color(theme.border)
                                                .overflow_hidden()
                                                .child(
                                                    img(PathBuf::from(path))
                                                        .w_full()
                                                        .max_h(theme.post_image_preview_height)
                                                        .object_fit(ObjectFit::Cover),
                                                )
                                                .child(
                                                    div()
                                                        .id("modal-clear-image-btn")
                                                        .absolute()
                                                        .top(theme.spacing_xs)
                                                        .right(theme.spacing_xs)
                                                        .p(theme.spacing_2xs)
                                                        .rounded_full()
                                                        .bg(theme.bg_scrim)
                                                        .cursor_pointer()
                                                        .text_color(theme.text_white)
                                                        .on_click(move |_, window, cx| (on_clear_image)(window, cx))
                                                        .child(Icon::new(LucideIcon::X)),
                                                ),
                                        )
                                    })
                                    // Attachment Action Bar
                                    .child(
                                        div()
                                            .w_full()
                                            .flex()
                                            .items_center()
                                            .gap(theme.spacing_xs)
                                            .pt(theme.spacing_xs)
                                            .child(
                                                Button::new("modal-pick-image-btn")
                                                    .ghost()
                                                    .small()
                                                    .icon(Icon::new(LucideIcon::Image))
                                                    .tooltip(t!("composer.pick_image").to_string())
                                                    .on_click(move |_, window, cx| (on_pick_image)(window, cx)),
                                            )
                                            .child(
                                                Button::new("modal-toggle-image-input-btn")
                                                    .ghost()
                                                    .small()
                                                    .icon(Icon::new(LucideIcon::Folder))
                                                    .tooltip(t!("composer.toggle_image_input").to_string())
                                                    .on_click(move |_, window, cx| (on_toggle_image)(window, cx)),
                                            )
                                            .child(
                                                Button::new("modal-toggle-link-btn")
                                                    .ghost()
                                                    .small()
                                                    .icon(Icon::new(LucideIcon::Link2))
                                                    .on_click(move |_, window, cx| (on_toggle_link)(window, cx)),
                                            )
                                            .child(
                                                Button::new("modal-tag-btn")
                                                    .ghost()
                                                    .small()
                                                    .icon(Icon::new(LucideIcon::Hash)),
                                            ),
                                    ),
                            ),
                    )
                    // 4. Modal Footer: Privacy indicator & Post button
                    .child(
                        div()
                            .w_full()
                            .px(theme.spacing_lg)
                            .py(theme.spacing_md)
                            .border_t(theme.border_width_hairline)
                            .border_color(theme.border)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_size(theme.font_size_xs)
                                    .text_color(theme.text_muted)
                                    .child(t!("composer.post_options").to_string()),
                            )
                            .child(
                                Button::new("modal-submit-post-btn")
                                    .primary()
                                    .small()
                                    .disabled(!can_post)
                                    .label(t!("composer.post_button").to_string())
                                    .on_click(move |_, window, cx| {
                                        (on_submit_btn)(window, cx);
                                    }),
                            ),
                    ),
            )
    }
}
