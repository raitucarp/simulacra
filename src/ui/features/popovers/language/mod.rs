//! Language selector popover menu molecule.

mod item;

use crate::i18n::Language;
use crate::theme::ThemeTokens;
use gpui_kit::base::StyledExt;
use gpui_kit::{
    div, px, App, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    Styled, Window,
};
use rust_i18n::t;

pub type SelectLangCallback = Box<dyn Fn(Language, &mut Window, &mut App) + 'static>;
pub type DismissCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Popover top offset (directly below the title bar).
const POPOVER_TOP_PX: f32 = 38.0;
/// Popover right offset (under the language trigger button).
const POPOVER_RIGHT_PX: f32 = 100.0;
/// Popover width.
const POPOVER_WIDTH_PX: f32 = 220.0;

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
        let theme = self.theme;
        let items = Language::ALL.iter().enumerate().map(|(idx, &lang)| {
            item::render_language_item(idx, lang, lang == current_lang, on_select.clone(), &theme)
        });

        div()
            .absolute()
            .inset_0()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx);
            })
            .child(
                render_card(&theme)
                    // Popover Header
                    .child(render_header(current_lang, &theme))
                    // Compact Language Item List
                    .children(items),
            )
    }
}

fn render_card(theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .top(px(POPOVER_TOP_PX))
        .right(px(POPOVER_RIGHT_PX))
        .w(px(POPOVER_WIDTH_PX))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border)
        .rounded_lg()
        .shadow_2xl()
        .p_1p5()
        .flex()
        .flex_col()
        .gap_0p5()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
}

fn render_header(current_lang: Language, theme: &ThemeTokens) -> Div {
    div()
        .px_2()
        .pt_1()
        .pb_1p5()
        .flex()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(theme.border)
        .mb_1()
        .child(
            div()
                .text_xs()
                .font_bold()
                .text_color(theme.text_muted)
                .child(t!("language.select").to_string()),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.accent_agent)
                .child(current_lang.code().to_uppercase()),
        )
}
