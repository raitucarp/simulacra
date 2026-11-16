//! Floating workspace popover: scrim, card, header, network list and create footer.

use super::consts::{
    CREATE_NETWORK_ID, POPOVER_LEFT_COLLAPSED_PX, POPOVER_LEFT_EXPANDED_PX, POPOVER_TOP_PX,
    POPOVER_WIDTH_PX,
};
use super::popover_item::render_network_item;
use super::{CreateNetworkCallback, WorkspacePopover};
use crate::ui::atoms::ElevationExt;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, px, App, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    Stateful, StatefulInteractiveElement, Styled, Window,
};
use rust_i18n::t;
use std::rc::Rc;

impl RenderOnce for WorkspacePopover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let (on_dismiss, on_create) = (self.on_dismiss, self.on_create);
        let on_select = Rc::new(self.on_select);
        let active_id = self.active_network.as_ref().map(|n| n.id.clone());
        let left_offset = if self.is_collapsed {
            px(POPOVER_LEFT_COLLAPSED_PX)
        } else {
            px(POPOVER_LEFT_EXPANDED_PX)
        };
        let items = self.networks.into_iter().enumerate().map(|(idx, net)| {
            let is_curr = Some(&net.id) == active_id.as_ref();
            render_network_item(idx, net, is_curr, on_select.clone(), &theme)
        });

        div()
            .absolute()
            .inset_0()
            .occlude()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx);
            })
            .child(
                render_card(left_offset, &theme)
                    // Popover Header
                    .child(render_header(&theme))
                    // List of Networks
                    .children(items)
                    // Footer: Create Workspace Action
                    .child(render_create_footer(on_create, &theme)),
            )
    }
}

fn render_card(left_offset: gpui_kit::Pixels, theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .top(px(POPOVER_TOP_PX))
        .left(left_offset)
        .w(px(POPOVER_WIDTH_PX))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border)
        .rounded_xl()
        .elevation_shadow(theme)
        .p_1p5()
        .flex()
        .flex_col()
        .gap_0p5()
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
}

fn render_header(theme: &ThemeTokens) -> Div {
    div()
        .px_2p5()
        .pt_1()
        .pb_1p5()
        .border_b_1()
        .border_color(theme.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_xs()
                .font_bold()
                .text_color(theme.text_muted)
                .child(t!("workspace.switch_network").to_string()),
        )
}

/// "+ Create Network" row opening the naming dialog.
fn render_create_footer(on_create: CreateNetworkCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(CREATE_NETWORK_ID)
        .on_click(move |_, window, cx| on_create(window, cx))
        .mt_1()
        .pt_1p5()
        .border_t_1()
        .border_color(theme.border)
        .px_2p5()
        .py_1p5()
        .rounded_lg()
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .flex()
        .items_center()
        .gap_2()
        .child(div().text_color(theme.accent_agent).child(Icon::new(LucideIcon::Plus)))
        .child(
            div()
                .text_xs()
                .font_medium()
                .text_color(theme.text_primary)
                .child(t!("workspace.create_network").to_string()),
        )
}
