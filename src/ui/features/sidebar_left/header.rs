//! The sidebar's top row: a one-line network switcher (just its badge in the rail) and the
//! collapse button.

use super::tooltip::with_tooltip;
use super::SidebarCallbacks;
use crate::domain::Network;
use crate::theme::ThemeTokens;
use super::icon_button::render_icon_button;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SWITCHER_ID: &str = "sidebar-network-switcher";
const COLLAPSE_ID: &str = "sidebar-collapse-toggle";
/// Shown in the badge when the network has no name.
const FALLBACK_INITIAL: &str = "?";

pub(super) fn render_network_switcher(network: Option<&Network>, is_open: bool, is_rail: bool, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let name = network.map(|n| n.name.clone()).unwrap_or_else(|| t!("workspace.no_network").to_string());
    let (hover_bg, on_command) = (theme.hover_bg, callbacks.on_command.clone());
    let on_key = on_command.clone();
    let switcher = div()
        .id(SWITCHER_ID)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .when(is_open, |row| row.bg(theme.hover_bg))
        .hover(move |style| style.bg(hover_bg))
        .keyboard_stop(theme, Rc::new(move |window, cx| on_key(CommandId::SwitchNetwork, window, cx)))
        .on_click(move |_, window, cx| on_command(CommandId::SwitchNetwork, window, cx))
        .child(render_badge(&name, theme));
    if is_rail {
        return with_tooltip(switcher.p(theme.spacing_xs), name, Some(CommandId::SwitchNetwork));
    }
    let chevron = if is_open { LucideIcon::ChevronUp } else { LucideIcon::ChevronsUpDown };
    switcher
        .flex_1()
        .min_w_0()
        .h(theme.nav_row_height)
        .px(theme.spacing_xs)
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_label).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(name))
        .child(div().flex_none().text_color(theme.text_muted).child(Icon::new(chevron).size(theme.icon_size_xs)))
}

/// The network's initial on a brand-tinted square.
fn render_badge(name: &str, theme: &ThemeTokens) -> Div {
    let initial = name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| FALLBACK_INITIAL.to_string());
    div()
        .size(theme.sidebar_badge_size)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .rounded(theme.radius_sm)
        .bg(theme.accent_brand_subtle)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.accent_brand)
        .child(initial)
}

/// The collapse (expanded) or expand (rail) button.
pub(super) fn render_collapse_toggle(is_rail: bool, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let icon = if is_rail { LucideIcon::PanelLeftOpen } else { LucideIcon::PanelLeftClose };
    let label_key = if is_rail { "sidebar.expand" } else { "sidebar.collapse" };
    render_icon_button(COLLAPSE_ID, icon, t!(label_key).to_string(), CommandId::ToggleSidebar, callbacks, theme)
}
