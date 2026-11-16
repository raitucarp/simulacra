//! Left brand breadcrumb and right action-button cluster of the title bar.

use super::window_controls::render_window_controls;
use super::TitleBarAction;
use crate::i18n::Language;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::{div, Div, ParentElement, Styled};
use rust_i18n::t;

const TOGGLE_LEFT_SIDEBAR_ID: &str = "toggle-left-sidebar-btn";
const TOGGLE_RIGHT_SIDEBAR_ID: &str = "toggle-right-sidebar-btn";
const LANG_MENU_ID: &str = "lang-menu-btn";
const SETTINGS_ID: &str = "settings-btn";

/// Callbacks owned by the right-hand action cluster.
pub(super) struct ActionCallbacks {
    pub on_toggle_right: TitleBarAction,
    pub on_toggle_lang: TitleBarAction,
    pub on_toggle_settings: TitleBarAction,
}

/// Compact ghost icon button that forwards clicks to a title bar action.
fn ghost_icon_button(id: &'static str, icon: LucideIcon, action: TitleBarAction) -> Button {
    Button::new(id)
        .ghost()
        .compact()
        .icon(Icon::new(icon))
        .on_click(move |_, window, cx| {
            action(window, cx);
        })
}

/// Sidebar toggle followed by the `app title › network` breadcrumb.
pub(super) fn render_brand_section(
    on_toggle_left: TitleBarAction,
    active_net_name: String,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(ghost_icon_button(TOGGLE_LEFT_SIDEBAR_ID, LucideIcon::PanelLeft, on_toggle_left))
        .child(render_breadcrumb(active_net_name, theme))
}

fn render_breadcrumb(active_net_name: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1p5()
        .child(
            div()
                .text_xs()
                .font_bold()
                .text_color(theme.text_primary)
                .child(t!("app.title").to_string()),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::ChevronRight)),
        )
        .child(
            div()
                .text_xs()
                .font_medium()
                .text_color(theme.accent_agent)
                .child(active_net_name),
        )
}

/// Right sidebar toggle, language trigger, settings trigger and window controls.
pub(super) fn render_action_cluster(
    callbacks: ActionCallbacks,
    current_language: Language,
    is_maximized: bool,
    theme: &ThemeTokens,
) -> Div {
    let on_toggle_lang = callbacks.on_toggle_lang;
    div()
        .flex()
        .items_center()
        .gap_1p5()
        // Toggle Right Sidebar Inspector
        .child(ghost_icon_button(
            TOGGLE_RIGHT_SIDEBAR_ID,
            LucideIcon::PanelRight,
            callbacks.on_toggle_right,
        ))
        // Language Dropdown Trigger
        .child(
            Button::new(LANG_MENU_ID)
                .ghost()
                .compact()
                .icon(Icon::new(LucideIcon::Globe))
                .label(current_language.code().to_uppercase())
                .on_click(move |_, window, cx| {
                    on_toggle_lang(window, cx);
                }),
        )
        // Global Settings Trigger
        .child(ghost_icon_button(SETTINGS_ID, LucideIcon::Settings, callbacks.on_toggle_settings))
        // Window Controls (Min, Max, Close)
        .child(render_window_controls(is_maximized, theme))
}
