//! Sidebar workspace trigger button rendering.

use super::consts::{NO_NETWORK_LABEL, TRIGGER_BADGE_MIN_PX, TRIGGER_HEIGHT_PX, TRIGGER_ID};
use super::{docs_and_agents_caption, WorkspaceTrigger};
use crate::domain::Network;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, App, Div, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, Window,
};

impl RenderOnce for WorkspaceTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let theme = self.theme;
        let (is_open, is_collapsed, on_toggle) = (self.is_open, self.is_collapsed, self.on_toggle);
        let (active_name, active_meta) = active_labels(self.active_network.as_ref());
        let hover_bg = theme.hover_bg;
        let border_color = if is_open { theme.accent_agent } else { theme.border };
        let bg_color = if is_open { hover_bg } else { theme.bg_app };

        div()
            .id(TRIGGER_ID)
            .w_full()
            .h(px(TRIGGER_HEIGHT_PX))
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
            .when(!is_collapsed, |this| this.justify_between().px_2p5().py_1p5().gap_2())
            // Left: Workspace Icon / Badge
            .child(render_badge(&theme))
            // Center info + Right chevron (Expanded mode only)
            .when(!is_collapsed, |this| {
                this.child(render_info(active_name, active_meta, &theme))
                    .child(render_chevron(is_open, &theme))
            })
    }
}

/// Active network name (or a placeholder) and its "docs · agents" caption.
fn active_labels(active_network: Option<&Network>) -> (String, String) {
    let name = active_network
        .map(|n| n.name.clone())
        .unwrap_or_else(|| NO_NETWORK_LABEL.to_string());
    let meta = active_network.map(docs_and_agents_caption).unwrap_or_default();
    (name, meta)
}

fn render_badge(theme: &ThemeTokens) -> Div {
    div()
        .w_7()
        .h_7()
        .min_w(px(TRIGGER_BADGE_MIN_PX))
        .min_h(px(TRIGGER_BADGE_MIN_PX))
        .rounded_md()
        .bg(theme.bg_app)
        .border_1()
        .border_color(theme.border)
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.accent_agent)
        .child(Icon::new(LucideIcon::Boxes))
}

fn render_info(active_name: String, active_meta: String, theme: &ThemeTokens) -> Div {
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
        .child(div().text_xs().text_color(theme.text_muted).truncate().child(active_meta))
}

fn render_chevron(is_open: bool, theme: &ThemeTokens) -> Div {
    let icon = if is_open { LucideIcon::ChevronUp } else { LucideIcon::ChevronDown };
    div().flex_none().text_color(theme.text_muted).child(Icon::new(icon))
}
