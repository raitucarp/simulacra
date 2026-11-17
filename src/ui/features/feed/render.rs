//! Main feed entry point: the timeline (the Knowledge Vault and the Mind Graph are routed by
//! the shell as their own views).

use super::inputs::split_params;
use super::params::FeedParams;
use super::timeline::render_timeline;
use crate::state::NavigationTab;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, IntoElement, ParentElement, Styled};

pub fn render_main_feed<C: IntoElement>(params: FeedParams<'_, C>) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;
    let (callbacks, timeline_inputs) = split_params(params);
    let timeline = render_timeline(state, theme, &callbacks, timeline_inputs);
    div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        .when(state.active_tab == NavigationTab::Timeline, |this| this.child(timeline))
}
