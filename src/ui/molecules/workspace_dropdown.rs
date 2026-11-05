//! Workspace/Network domain dropdown trigger and floating popover molecules with 100% Lucide SVG icons.

use crate::domain::{Network, NetworkId};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, px, App, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

pub type SelectNetworkCallback = Box<dyn Fn(NetworkId, &mut Window, &mut App) + 'static>;
pub type ToggleWorkspaceCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Atomic Workspace Trigger button rendered in the sidebar.
#[derive(IntoElement)]
pub struct WorkspaceTrigger {
    active_network: Option<Network>,
    is_open: bool,
    is_collapsed: bool,
    theme: ThemeTokens,
    on_toggle: ToggleWorkspaceCallback,
}

impl WorkspaceTrigger {
    pub fn new(
        active_network: Option<Network>,
        is_open: bool,
        is_collapsed: bool,
        theme: ThemeTokens,
        on_toggle: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            is_open,
            is_collapsed,
            theme,
            on_toggle: Box::new(on_toggle),
        }
    }
}

impl RenderOnce for WorkspaceTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let is_open = self.is_open;
        let is_collapsed = self.is_collapsed;
        let on_toggle = self.on_toggle;

        let active_name = self
            .active_network
            .as_ref()
            .map(|n| n.name.clone())
            .unwrap_or_else(|| "Select Domain".to_string());

        let active_meta = self
            .active_network
            .as_ref()
            .map(|n| {
                t!(
                    "sidebar.docs_and_agents",
                    docs = n.doc_count,
                    agents = n.agent_count
                )
                .to_string()
            })
            .unwrap_or_default();

        let hover_bg = theme.hover_bg;
        let border_color = if is_open {
            theme.accent_agent
        } else {
            theme.border
        };
        let bg_color = if is_open { hover_bg } else { theme.bg_app };

        div()
            .id("workspace-trigger-btn")
            .w_full()
            .h(px(46.0))
            .rounded_lg()
            .border_1()
            .border_color(border_color)
            .bg(bg_color)
            .cursor_pointer()
            .hover(move |s| s.bg(hover_bg))
            .on_click(move |_, window, cx| {
                on_toggle(window, cx);
            })
            .flex()
            .items_center()
            .when(is_collapsed, |this| this.justify_center().p_1())
            .when(!is_collapsed, |this| {
                this.justify_between().px_2p5().py_1p5().gap_2()
            })
            // Left: Workspace Icon / Badge
            .child(
                div()
                    .w_7()
                    .h_7()
                    .min_w(px(28.0))
                    .min_h(px(28.0))
                    .rounded_md()
                    .bg(theme.bg_app)
                    .border_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(theme.accent_agent)
                    .child(Icon::new(LucideIcon::Boxes)),
            )
            // Center info (Expanded mode only)
            .when(!is_collapsed, |this| {
                this.child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .flex()
                        .flex_col()
                        .overflow_hidden()
                        .child(
                            div()
                                .text_xs()
                                .font_bold()
                                .text_color(theme.text_primary)
                                .truncate()
                                .child(active_name),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .truncate()
                                .child(active_meta),
                        ),
                )
                // Right: Chevron Indicator
                .child(
                    div()
                        .flex_none()
                        .text_color(theme.text_muted)
                        .child(if is_open {
                            Icon::new(LucideIcon::ChevronUp)
                        } else {
                            Icon::new(LucideIcon::ChevronDown)
                        }),
                )
            })
    }
}

/// Atomic Floating Workspace Popover Menu rendered at root with 100% Lucide SVG icons.
#[derive(IntoElement)]
pub struct WorkspacePopover {
    active_network: Option<Network>,
    networks: Vec<Network>,
    is_collapsed: bool,
    theme: ThemeTokens,
    on_select: SelectNetworkCallback,
    on_dismiss: ToggleWorkspaceCallback,
}

impl WorkspacePopover {
    pub fn new(
        active_network: Option<Network>,
        networks: Vec<Network>,
        is_collapsed: bool,
        theme: ThemeTokens,
        on_select: impl Fn(NetworkId, &mut Window, &mut App) + 'static,
        on_dismiss: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            networks,
            is_collapsed,
            theme,
            on_select: Box::new(on_select),
            on_dismiss: Box::new(on_dismiss),
        }
    }
}

impl RenderOnce for WorkspacePopover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let is_collapsed = self.is_collapsed;
        let on_dismiss = self.on_dismiss;
        let on_select = Rc::new(self.on_select);
        let networks = self.networks;
        let active_id = self.active_network.as_ref().map(|n| n.id.clone());

        let hover_bg = theme.hover_bg;
        let border_color = theme.border;
        let accent_agent = theme.accent_agent;

        let left_offset = if is_collapsed { px(64.0) } else { px(12.0) };

        div()
            .absolute()
            .inset_0()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                on_dismiss(window, cx);
            })
            .child(
                div()
                    .absolute()
                    .top(px(46.0))
                    .left(left_offset)
                    .w(px(260.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(border_color)
                    .rounded_xl()
                    .shadow_2xl()
                    .p_1p5()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    })
                    // Popover Header
                    .child(
                        div()
                            .px_2p5()
                            .pt_1()
                            .pb_1p5()
                            .border_b_1()
                            .border_color(border_color)
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(theme.text_muted)
                                    .child(t!("workspace.switch_network").to_string()),
                            ),
                    )
                    // List of Networks
                    .children(networks.into_iter().enumerate().map(|(idx, net)| {
                        let is_curr = Some(&net.id) == active_id.as_ref();
                        let net_id = net.id.clone();
                        let cb = on_select.clone();

                        let docs_meta = t!(
                            "sidebar.docs_and_agents",
                            docs = net.doc_count,
                            agents = net.agent_count
                        )
                        .to_string();

                        div()
                            .id(format!("ws-item-{}", idx))
                            .px_2p5()
                            .py_2()
                            .rounded_lg()
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_between()
                            .when(is_curr, |this| this.bg(hover_bg))
                            .hover(move |s| s.bg(hover_bg))
                            .on_click(move |_, window, cx| {
                                cb(net_id.clone(), window, cx);
                            })
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2p5()
                                    .min_w_0()
                                    .child(
                                        div()
                                            .w_2()
                                            .h_2()
                                            .rounded_full()
                                            .bg(if is_curr {
                                                accent_agent
                                            } else {
                                                theme.border
                                            }),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .min_w_0()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_bold()
                                                    .text_color(if is_curr {
                                                        theme.text_primary
                                                    } else {
                                                        theme.text_secondary
                                                    })
                                                    .truncate()
                                                    .child(net.name),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .truncate()
                                                    .child(docs_meta),
                                            ),
                                    ),
                            )
                            .when(is_curr, |this| {
                                this.child(
                                    div()
                                        .text_color(accent_agent)
                                        .child(Icon::new(LucideIcon::Check)),
                                )
                            })
                    }))
                    // Footer: Create Workspace Action
                    .child(
                        div()
                            .mt_1()
                            .pt_1p5()
                            .border_t_1()
                            .border_color(border_color)
                            .px_2p5()
                            .py_1p5()
                            .rounded_lg()
                            .cursor_pointer()
                            .hover(move |s| s.bg(hover_bg))
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(accent_agent)
                                    .child(Icon::new(LucideIcon::Plus)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .font_medium()
                                    .text_color(theme.text_primary)
                                    .child(t!("workspace.create_network").to_string()),
                            ),
                    ),
            )
    }
}
