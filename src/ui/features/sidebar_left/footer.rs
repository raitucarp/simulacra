//! The footer: preferences (Settings, Appearance, Keyboard shortcuts) beside the identity card.
//! The rail stacks the same buttons and the avatar, then the expand button.

use super::header::render_collapse_toggle;
use super::icon_button::render_icon_button;
use super::user_card::render_interactive_user_card;
use super::SidebarCallbacks;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SETTINGS_ROW_ID: &str = "sidebar-footer-settings";
const APPEARANCE_ID: &str = "sidebar-footer-appearance";
const SHORTCUTS_ID: &str = "sidebar-footer-shortcuts";
const SETTINGS_ID: &str = "sidebar-footer-settings-icon";
const RAIL_AVATAR_ID: &str = "sidebar-rail-avatar";

pub(super) fn render_footer(state: &AppState, is_rail: bool, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Div {
    let footer = div().flex().flex_col().gap(theme.spacing_2xs).pt(theme.spacing_xs).border_t(theme.border_width_hairline).border_color(theme.border_subtle);
    let appearance = render_icon_button(APPEARANCE_ID, LucideIcon::Palette, spec(CommandId::OpenAppearance).title(), CommandId::OpenAppearance, callbacks, theme);
    let shortcuts = render_icon_button(SHORTCUTS_ID, LucideIcon::Keyboard, spec(CommandId::ShowShortcuts).title(), CommandId::ShowShortcuts, callbacks, theme);
    if is_rail {
        let settings = render_icon_button(SETTINGS_ID, LucideIcon::Settings, spec(CommandId::OpenSettings).title(), CommandId::OpenSettings, callbacks, theme);
        return footer
            .items_center()
            .child(settings)
            .child(appearance)
            .child(shortcuts)
            .child(render_rail_avatar(state, callbacks, theme))
            .child(render_collapse_toggle(true, callbacks, theme));
    }
    let (nav, switch) = (callbacks.on_select_nav.clone(), callbacks.on_command.clone());
    let preferences = div().flex().items_center().gap(theme.spacing_2xs).child(render_settings_row(callbacks, theme)).child(appearance).child(shortcuts);
    footer.child(preferences).child(render_interactive_user_card(
        state.current_profile(),
        theme,
        move |_, window, cx| nav(NavigationTab::PersonaLab, window, cx),
        move |_, window, cx| switch(CommandId::SwitchPersona, window, cx),
    ))
}

/// "Settings" with its icon, filling the row beside the icon buttons.
fn render_settings_row(callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let (hover_bg, on_click) = (theme.hover_bg, callbacks.on_command.clone());
    let on_key = on_click.clone();
    div()
        .id(SETTINGS_ROW_ID)
        .flex_1()
        .min_w_0()
        .h(theme.nav_row_height)
        .px(theme.spacing_sm)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .text_color(theme.text_secondary)
        .hover(move |style| style.bg(hover_bg))
        .keyboard_stop(theme, Rc::new(move |window, cx| on_key(CommandId::OpenSettings, window, cx)))
        .on_click(move |_, window, cx| on_click(CommandId::OpenSettings, window, cx))
        .child(div().w(theme.nav_icon_slot_width).flex().justify_center().text_color(theme.text_muted).child(Icon::new(LucideIcon::Settings).size(theme.icon_size_sm)))
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_label).child(t!("sidebar.settings").to_string()))
}

/// The active persona's avatar; opens the Persona Lab.
fn render_rail_avatar(state: &AppState, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let nav = callbacks.on_select_nav.clone();
    div()
        .id(RAIL_AVATAR_ID)
        .p(theme.spacing_xs)
        .cursor_pointer()
        .on_click(move |_, window, cx| nav(NavigationTab::PersonaLab, window, cx))
        .when_some(state.current_profile(), |button, profile| button.child(render_profile_avatar(profile, theme.avatar_size_sm, theme)))
}
