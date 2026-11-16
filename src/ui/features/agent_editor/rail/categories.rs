//! Tier 2: vertical primary category rail with live summaries and archetype presets.

use super::presets::render_presets;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::components::{vertical_tab, VerticalTab};
use crate::ui::features::agent_editor::callbacks::{SelectConfigTabCallback, SharedCallbacks};
use crate::ui::features::agent_editor::config_tab::AgentConfigTab;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;
use std::rc::Rc;

const RAIL_ID: &str = "agent-vertical-category-rail";
const TAB_ID_PREFIX: &str = "agent-vert-tab";

pub(crate) fn render_category_rail(
    profile: &Profile,
    active_tab: AgentConfigTab,
    theme: &ThemeTokens,
    callbacks: &SharedCallbacks,
) -> impl IntoElement {
    div()
        .id(RAIL_ID)
        .w(theme.agent_category_rail_width)
        .flex_shrink_0()
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .p(theme.spacing_sm)
        .flex()
        .flex_col()
        .justify_between()
        .overflow_y_scrollbar()
        .child(category_tabs(profile, active_tab, theme, callbacks.select_tab.clone()))
        .child(render_presets(profile, theme, callbacks.update_profile.clone()))
}

fn category_tabs(
    profile: &Profile,
    active_tab: AgentConfigTab,
    theme: &ThemeTokens,
    on_select: Rc<SelectConfigTabCallback>,
) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .children(AgentConfigTab::ALL.iter().map(|&tab| {
            let cb = on_select.clone();
            let spec = VerticalTab {
                id: format!("{}-{:?}", TAB_ID_PREFIX, tab).into(),
                icon: tab.icon(),
                title: tab.localized_label(),
                subtitle: tab.live_summary(profile),
                active: tab == active_tab,
            };
            vertical_tab(spec, theme, move |w, cx| cb(tab, w, cx))
        }))
}
