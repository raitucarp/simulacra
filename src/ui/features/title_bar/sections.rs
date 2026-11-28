//! Left brand breadcrumb and right action-button cluster of the title bar.

use super::appearance_toggle::render_appearance_toggle;
use super::window_controls::render_window_controls;
use super::TitleBarAction;
use crate::i18n::Language;
use crate::theme::{AppearanceMode, ThemeTokens};
use crate::ui::features::command_palette::CommandId;
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::{chord_text, shortcut_for};
use gpui_kit::base::StyledExt;
use crate::ui::features::popovers::language::{language_menu_spec, SelectLangCallback};
use crate::ui::menus::render_menu;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Anchor, Div, ParentElement, Styled};
use rust_i18n::t;

const TOGGLE_LEFT_SIDEBAR_ID: &str = "toggle-left-sidebar-btn";
const TOGGLE_RIGHT_SIDEBAR_ID: &str = "toggle-right-sidebar-btn";
const LANG_MENU_ID: &str = "lang-menu-btn";
const SETTINGS_ID: &str = "settings-btn";

/// Callbacks owned by the right-hand action cluster.
pub(super) struct ActionCallbacks {
    pub on_toggle_right: TitleBarAction,
    pub on_select_lang: SelectLangCallback,
    pub on_toggle_settings: TitleBarAction,
    pub appearance: Option<(AppearanceMode, TitleBarAction)>,
}

/// What a title bar button does, and its shortcut: "Open settings (Ctrl+,)".
fn command_tooltip(command: CommandId, label_key: &str) -> String {
    let label = t!(label_key).to_string();
    match shortcut_for(command) {
        Some(chord) => t!("title_bar.tooltip_shortcut", label = label, keys = chord_text(chord)).to_string(),
        None => label,
    }
}

/// Compact ghost icon button that forwards clicks to a title bar action, named by `tooltip`.
fn ghost_icon_button(id: &'static str, icon: LucideIcon, tooltip: String, action: TitleBarAction) -> Button {
    Button::new(id)
        .ghost()
        .compact()
        .icon(Icon::new(icon))
        .tooltip(tooltip)
        .on_click(move |_, window, cx| {
            action(window, cx);
        })
}

/// The ☰ app menu (M1), the sidebar toggle, then the `app title › network` breadcrumb.
pub(super) fn render_brand_section(
    on_toggle_left: TitleBarAction,
    app_menu: Option<gpui_kit::FocusHandle>,
    active_net_name: String,
    theme: &ThemeTokens,
) -> Div {
    div()
        .flex()
        .items_center()
        .min_w_0()
        .gap(theme.spacing_sm)
        .children(app_menu.map(super::app_menu::render_app_menu))
        .child(ghost_icon_button(TOGGLE_LEFT_SIDEBAR_ID, LucideIcon::PanelLeft, command_tooltip(CommandId::ToggleSidebar, "palette.cmd.toggle_sidebar"), on_toggle_left))
        .child(render_breadcrumb(active_net_name, theme))
}

fn render_breadcrumb(active_net_name: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .min_w_0()
        .gap(theme.spacing_xs_plus)
        .child(
            div()
                .flex_none()
                .text_size(theme.font_size_label)
                .font_bold()
                .text_color(theme.text_primary)
                .child(t!("app.title").to_string()),
        )
        .child(
            div()
                .flex_none()
                .text_size(theme.font_size_label)
                .text_color(theme.text_muted)
                .child(Icon::new(LucideIcon::ChevronRight)),
        )
        .child(
            div()
                .min_w_0()
                .truncate()
                .text_size(theme.font_size_label)
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
    let on_select_lang = callbacks.on_select_lang;
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.spacing_xs_plus)
        // Toggle Right Sidebar Inspector
        .child(ghost_icon_button(
            TOGGLE_RIGHT_SIDEBAR_ID,
            LucideIcon::PanelRight,
            command_tooltip(CommandId::ToggleRightSidebar, "palette.cmd.toggle_right_sidebar"),
            callbacks.on_toggle_right,
        ))
        // Language menu (M1: a kit dropdown, the current language ticked)
        .child(
            Button::new(LANG_MENU_ID)
                .ghost()
                .compact()
                .icon(Icon::new(LucideIcon::Globe))
                .label(current_language.code().to_uppercase())
                .tooltip(t!("language.select").to_string())
                .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| render_menu(menu, &language_menu_spec(current_language, &on_select_lang), window, cx)),
        )
        // Dark / Light / System toggle
        .when_some(callbacks.appearance, |row, (mode, on_cycle)| row.child(render_appearance_toggle(mode, on_cycle)))
        // Global Settings Trigger
        .child(ghost_icon_button(SETTINGS_ID, LucideIcon::Settings, command_tooltip(CommandId::OpenSettings, "palette.cmd.open_settings"), callbacks.on_toggle_settings))
        // Window Controls (Min, Max, Close)
        .child(render_window_controls(is_maximized, theme))
}
