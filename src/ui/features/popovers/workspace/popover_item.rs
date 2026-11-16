//! A single selectable network row inside the workspace popover.

use super::consts::ITEM_ID_PREFIX;
use super::{docs_and_agents_caption, SelectNetworkCallback};
use crate::domain::Network;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
};
use std::rc::Rc;

/// Renders one network row; clicking it selects that network.
pub(super) fn render_network_item(
    idx: usize,
    net: Network,
    is_curr: bool,
    on_select: Rc<SelectNetworkCallback>,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let accent_agent = theme.accent_agent;
    let net_id = net.id.clone();
    let docs_meta = docs_and_agents_caption(&net);

    div()
        .id(format!("{ITEM_ID_PREFIX}{idx}"))
        .px(theme.spacing_sm_plus)
        .py(theme.spacing_sm)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_between()
        .when(is_curr, |this| this.bg(hover_bg))
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, window, cx| {
            on_select(net_id.clone(), window, cx);
        })
        .child(render_item_label(net.name, docs_meta, is_curr, theme))
        .when(is_curr, |this| {
            this.child(div().text_color(accent_agent).child(Icon::new(LucideIcon::Check)))
        })
}

fn render_item_label(name: String, docs_meta: String, is_curr: bool, theme: &ThemeTokens) -> Div {
    let dot_color = if is_curr { theme.accent_agent } else { theme.border };
    let name_color = if is_curr { theme.text_primary } else { theme.text_secondary };
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm_plus)
        .min_w_0()
        .child(div().w(theme.workspace_dot_size).h(theme.workspace_dot_size).rounded(theme.radius_full).bg(dot_color))
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(div().text_size(theme.font_size_label).font_bold().text_color(name_color).truncate().child(name))
                .child(div().text_size(theme.font_size_label).text_color(theme.text_muted).truncate().child(docs_meta)),
        )
}
