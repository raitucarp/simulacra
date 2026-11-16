//! Floating workspace popover: scrim, card, header, network list and create footer.

use super::consts::CREATE_NETWORK_ID;
use super::popover_item::render_network_item;
use super::{CreateNetworkCallback, WorkspacePopover};
use crate::ui::atoms::ElevationExt;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, App, Div, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
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
        // Collapsed: just right of the icon rail; expanded: inset like the trigger.
        let left_offset =
            if self.is_collapsed { theme.left_sidebar_collapsed_width } else { theme.spacing_md };
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
        // Aligned to the trigger's bottom edge
        .top(theme.workspace_trigger_height)
        .left(left_offset)
        .w(theme.workspace_popover_width)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .rounded(theme.radius_lg)
        .elevation_shadow(theme)
        .p(theme.spacing_xs_plus)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
}

fn render_header(theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_sm_plus)
        .pt(theme.spacing_xs)
        .pb(theme.spacing_xs_plus)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(theme.font_size_label)
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
        .mt(theme.spacing_xs)
        .pt(theme.spacing_xs_plus)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border)
        .px(theme.spacing_sm_plus)
        .py(theme.spacing_xs_plus)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(div().text_color(theme.accent_agent).child(Icon::new(LucideIcon::Plus)))
        .child(
            div()
                .text_size(theme.font_size_label)
                .font_medium()
                .text_color(theme.text_primary)
                .child(t!("workspace.create_network").to_string()),
        )
}
