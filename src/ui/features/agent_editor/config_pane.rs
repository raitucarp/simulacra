//! Tiers 2 & 3: right configuration pane (hero header, category rail, sub-tab content).

use super::callbacks::SharedCallbacks;
use super::header::render_hero_header;
use super::header_actions::HeaderFlags;
use super::params::WorkspaceScope;
use super::rail::render_category_rail;
use super::steps::{render_step, StepCtx};
use super::subtab_bar::render_subtab_bar;
use super::workspace::WorkspaceData;
use crate::domain::Profile;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const PANE_ID: &str = "agent-workspace-right-config-pane";
const BODY_SCROLL_ID: &str = "agent-config-subtab-scroll-body";

pub(crate) fn render_config_pane(
    profile: &Profile,
    scope: &WorkspaceScope<'_>,
    data: &WorkspaceData<'_>,
    callbacks: &SharedCallbacks,
) -> Stateful<Div> {
    let theme = scope.theme;
    let flags = HeaderFlags {
        active_user_persona: scope.state.active_profile_id == profile.id,
        can_delete: data.contact_list.len() > 1,
    };
    div()
        .id(PANE_ID)
        .flex_1()
        .min_w_0()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        .child(render_hero_header(profile, flags, theme, callbacks))
        .child(
            div()
                .w_full()
                .flex_1()
                .min_h_0()
                .flex()
                .flex_row()
                .overflow_hidden()
                .child(render_category_rail(profile, scope.active_config_tab, theme, callbacks))
                .child(content_area(profile, scope, data, callbacks)),
        )
}

/// Sub-tab bar plus the scrollable body of the active sub-tab.
fn content_area(
    profile: &Profile,
    scope: &WorkspaceScope<'_>,
    data: &WorkspaceData<'_>,
    callbacks: &SharedCallbacks,
) -> Div {
    let theme = scope.theme;
    let active_tab = scope.active_config_tab;
    let sub_tabs = active_tab.sub_tabs();
    let active_sub_tab = scope.active_sub_tab.min(sub_tabs.len().saturating_sub(1));
    let ctx = StepCtx {
        profile,
        inputs: &scope.inputs,
        theme,
        network_name: &data.network_name,
        grounding_doc_titles: &data.grounding_doc_titles,
        on_update: callbacks.update_profile.clone(),
        on_save_text: callbacks.save_text.clone(),
        on_pick_avatar: callbacks.pick_avatar.clone(),
    };
    let bar = render_subtab_bar(active_tab, active_sub_tab, sub_tabs, theme, callbacks.select_sub_tab.clone());
    div()
        .flex_1()
        .min_w_0()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        .child(bar)
        .child(
            div()
                .id(BODY_SCROLL_ID)
                .flex_1()
                .overflow_y_scrollbar()
                .p(theme.spacing_lg)
                .child(render_step(active_tab, active_sub_tab, &ctx)),
        )
}
