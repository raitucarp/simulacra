//! Root composition: context row, create-and-find row, places, the scrolling sections (or the
//! rail's pins) and the footer.

use super::footer::render_footer;
use super::header::{render_collapse_toggle, render_network_switcher};
use super::icon_button::render_icon_button;
use super::mode::SidebarMode;
use super::nav_list::render_nav_list;
use super::new_menu::render_new_button;
use super::sections::{render_rail_pins, render_sections};
use super::LeftSidebarParams;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::icons::LucideIcon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const SEARCH_ID: &str = "sidebar-open-palette";
const SECTIONS_ID: &str = "sidebar-sections-scroll";

pub fn render_left_sidebar(params: LeftSidebarParams<'_>) -> impl IntoElement {
    let LeftSidebarParams { state, theme, sidebar, is_workspace_open, shell_focus, callbacks } = params;
    let is_rail = sidebar.mode(theme.left_sidebar_rail_breakpoint) == SidebarMode::Rail;
    let switcher = render_network_switcher(state.current_network(), is_workspace_open, is_rail, &callbacks, theme);
    let search = render_icon_button(SEARCH_ID, LucideIcon::Search, spec(CommandId::OpenPalette).title(), CommandId::OpenPalette, &callbacks, theme);
    let new_button = div().when(!is_rail, |slot| slot.flex_1().min_w_0()).child(render_new_button(is_rail, shell_focus, theme));
    let top = div()
        .flex()
        .flex_col()
        .flex_none()
        .gap(theme.stack_gap)
        .when(is_rail, |top| top.items_center())
        .child(if is_rail { div().child(switcher) } else { div().flex().items_center().gap(theme.spacing_2xs).child(switcher).child(render_collapse_toggle(false, &callbacks, theme)) })
        .child(if is_rail { div().flex().flex_col().items_center().gap(theme.spacing_2xs).child(new_button).child(search) } else { div().flex().items_center().gap(theme.spacing_2xs).child(new_button).child(search) })
        .child(render_nav_list(state, is_rail, &callbacks, theme));
    let middle = div().id(SECTIONS_ID).flex_1().min_h_0().overflow_y_scroll().flex().flex_col().pt(theme.stack_gap);
    let middle = if is_rail {
        middle.items_center().gap(theme.spacing_2xs).children(render_rail_pins(state, sidebar, &callbacks, theme))
    } else {
        middle.gap(theme.stack_gap).children(render_sections(state, sidebar, &callbacks, theme))
    };
    div()
        .size_full()
        .bg(theme.bg_sidebar)
        .flex()
        .flex_col()
        .p(theme.panel_padding_compact)
        .font_family(theme.font_family_ui.clone())
        .child(top)
        .child(middle)
        .child(render_footer(state, is_rail, &callbacks, theme))
}
