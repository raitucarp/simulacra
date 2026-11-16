//! Title bar root layout: brand breadcrumb, drag zone, and action cluster.

use super::sections::{render_action_cluster, render_brand_section, ActionCallbacks};
use super::AppTitleBar;
use gpui_kit::{
    div, App, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled, Window,
    WindowControlArea,
};
use rust_i18n::t;

impl RenderOnce for AppTitleBar {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let active_net_name = self
            .active_network
            .map(|n| n.name)
            .unwrap_or_else(|| t!("feed.general_timeline").to_string());
        let is_maximized = window.is_maximized();
        let theme = self.theme;
        let callbacks = ActionCallbacks {
            on_toggle_right: self.on_toggle_right_sidebar,
            on_toggle_lang: self.on_toggle_lang,
            on_toggle_settings: self.on_toggle_settings,
        };

        div()
            .w_full()
            .h(theme.titlebar_height)
            .bg(theme.bg_card)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .px(theme.spacing_md)
            // 1. Left Section: Sidebar Toggle + Brand & Breadcrumbs
            .child(render_brand_section(self.on_toggle_left_sidebar, active_net_name, &theme))
            // 2. Center Draggable Area
            .child(div().flex_1().h_full().window_control_area(WindowControlArea::Drag))
            // 3. Right Section: PanelRight Toggle + Language + Settings + Window Controls
            .child(render_action_cluster(
                callbacks,
                self.current_language,
                is_maximized,
                &theme,
            ))
    }
}
