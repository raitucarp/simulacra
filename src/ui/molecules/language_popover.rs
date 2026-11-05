//! Language selector popover menu molecule.

use crate::i18n::Language;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, px, App, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, Styled, Window};
use rust_i18n::t;

pub type SelectLangCallback = Box<dyn Fn(Language, &mut Window, &mut App) + 'static>;
pub type DismissCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Atomic Floating Language Selector Popover.
#[derive(IntoElement)]
pub struct LanguagePopover {
    current_language: Language,
    theme: ThemeTokens,
    on_select: SelectLangCallback,
    on_dismiss: DismissCallback,
}

impl LanguagePopover {
    pub fn new(
        current_language: Language,
        theme: ThemeTokens,
        on_select: impl Fn(Language, &mut Window, &mut App) + 'static,
        on_dismiss: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            current_language,
            theme,
            on_select: Box::new(on_select),
            on_dismiss: Box::new(on_dismiss),
        }
    }
}

impl RenderOnce for LanguagePopover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let current_lang = self.current_language;
        let on_dismiss = self.on_dismiss;
        let on_select = std::rc::Rc::new(self.on_select);
        let hover_bg = self.theme.hover_bg;
        let accent_agent = self.theme.accent_agent;
        let text_primary = self.theme.text_primary;
        let text_muted = self.theme.text_muted;
        let border_color = self.theme.border;

        div()
            .absolute()
            .inset_0()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx);
            })
            .child(
                div()
                    .absolute()
                    .top(px(38.0))
                    .right(px(100.0))
                    .w(px(220.0))
                    .bg(self.theme.bg_card)
                    .border_1()
                    .border_color(border_color)
                    .rounded_lg()
                    .shadow_2xl()
                    .p_1p5()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    // Popover Header
                    .child(
                        div()
                            .px_2()
                            .pt_1()
                            .pb_1p5()
                            .flex()
                            .items_center()
                            .justify_between()
                            .border_b_1()
                            .border_color(border_color)
                            .mb_1()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(text_muted)
                                    .child(t!("language.select").to_string()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(accent_agent)
                                    .child(current_lang.code().to_uppercase()),
                            ),
                    )
                    // Compact Language Item List
                    .children(Language::ALL.iter().enumerate().map(|(idx, &lang)| {
                        let is_active = lang == current_lang;
                        let on_select = on_select.clone();

                        div()
                            .id(format!("lang-item-{}", idx))
                            .px_2p5()
                            .py_1p5()
                            .rounded_md()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_between()
                            .when(is_active, |this| this.bg(hover_bg))
                            .hover(move |s| s.bg(hover_bg))
                            .on_click(move |_, window, cx| {
                                on_select(lang, window, cx);
                            })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w_3p5()
                                            .h_3p5()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .when(is_active, |this| {
                                                this.text_color(accent_agent)
                                                    .child(Icon::new(LucideIcon::Check))
                                            }),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_medium()
                                            .text_color(if is_active {
                                                accent_agent
                                            } else {
                                                text_primary
                                            })
                                            .child(lang.native_label()),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_muted)
                                    .child(lang.code().to_uppercase()),
                            )
                    })),
            )
    }
}
