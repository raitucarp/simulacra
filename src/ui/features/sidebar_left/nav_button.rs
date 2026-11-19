//! One compact navigation row: icon, label and count, an accent bar while active, a keyboard
//! focus ring (Tab, then Enter / Space) and, in the rail, an icon-only button with a tooltip.

use super::tooltip::with_tooltip;
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::forge::widgets::{ActionCallback, KeyboardStop};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Element id prefix of each nav button (suffixed with its label).
const NAV_BUTTON_ID_PREFIX: &str = "nav-btn-";

/// What one nav row shows.
pub struct NavButtonSpec {
    pub label: String,
    pub icon: LucideIcon,
    pub is_active: bool,
    pub count: Option<usize>,
    /// The command whose shortcut the rail tooltip shows.
    pub command: Option<CommandId>,
}

pub fn render_nav_button(spec: NavButtonSpec, is_rail: bool, theme: &ThemeTokens, on_click: ActionCallback) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let on_key = on_click.clone();
    let button = div()
        .id(SharedString::from(format!("{NAV_BUTTON_ID_PREFIX}{}", spec.label)))
        .relative()
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .when(spec.is_active, |row| row.bg(theme.bg_tab_active))
        .when(!spec.is_active, |row| row.hover(move |style| style.bg(hover_bg)))
        .keyboard_stop(theme, on_key)
        .on_click(move |_, window, cx| on_click(window, cx))
        .when(spec.is_active, |row| row.child(render_active_bar(theme)))
        .child(render_icon(spec.icon, spec.is_active, theme));
    if is_rail {
        let rail = button.size(theme.nav_rail_button_size).justify_center();
        return with_tooltip(rail, spec.label, spec.command);
    }
    let text_color = if spec.is_active { theme.text_primary } else { theme.text_secondary };
    button
        .w_full()
        .h(theme.nav_row_height)
        .px(theme.spacing_sm)
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_label).font_weight(FontWeight::MEDIUM).text_color(text_color).child(spec.label))
        .when_some(spec.count.filter(|count| *count > 0), |row, count| row.child(render_count(count, theme)))
}

fn render_icon(icon: LucideIcon, is_active: bool, theme: &ThemeTokens) -> Div {
    let color = if is_active { theme.accent_brand } else { theme.text_muted };
    div()
        .w(theme.nav_icon_slot_width)
        .flex()
        .flex_none()
        .justify_center()
        .text_color(color)
        .child(Icon::new(icon).size(theme.icon_size_sm))
}

/// The accent bar on the row's left edge.
fn render_active_bar(theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .left_0()
        .top_0()
        .bottom_0()
        .flex()
        .items_center()
        .child(div().w(theme.nav_active_bar_width).h(theme.nav_active_bar_height).rounded(theme.radius_full).bg(theme.accent_brand))
}

fn render_count(count: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex_none()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(count.to_string())
}
