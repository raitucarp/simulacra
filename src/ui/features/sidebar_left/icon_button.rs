//! A square icon-only button that runs a command, with a tooltip naming it and its shortcut.

use super::tooltip::with_tooltip;
use super::SidebarCallbacks;
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use std::rc::Rc;

pub(super) fn render_icon_button(
    id: &'static str,
    icon: LucideIcon,
    label: String,
    command: CommandId,
    callbacks: &SidebarCallbacks,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let (hover_bg, hover_text) = (theme.hover_bg, theme.text_primary);
    let on_click = callbacks.on_command.clone();
    let on_key = on_click.clone();
    let button = div()
        .id(id)
        .size(theme.nav_rail_button_size)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .rounded(theme.radius_md)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |style| style.bg(hover_bg).text_color(hover_text))
        .keyboard_stop(theme, Rc::new(move |window, cx| on_key(command, window, cx)))
        .on_click(move |_, window, cx| on_click(command, window, cx))
        .child(Icon::new(icon).size(theme.icon_size_sm));
    with_tooltip(button, label, Some(command))
}
