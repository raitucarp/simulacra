//! Network Member Agents quick roster.

use super::section::{section_container, section_header};
use super::{NavHandle, SelectSidebarAgentCallback};
use crate::domain::Profile;
use crate::state::NavigationTab;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use gpui_kit::base::StyledExt;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::rc::Rc;

/// Element id prefix of each member row (suffixed with the row index).
const MEMBER_ROW_ID_PREFIX: &str = "active-agent-sidebar-";

pub(super) fn render_members_section(
    agents: Vec<&Profile>,
    on_select_nav: &NavHandle,
    on_select_agent: &Option<Rc<SelectSidebarAgentCallback>>,
    theme: &ThemeTokens,
) -> Div {
    let header = section_header(
        t!("sidebar.network_members").to_string(),
        agents.len(),
        theme.accent_brand,
        theme,
    );
    let rows = agents.into_iter().enumerate().map(|(idx, profile)| {
        render_member_row(idx, profile, on_select_nav.clone(), on_select_agent.clone(), theme)
    });
    section_container(theme).child(header).children(rows)
}

/// One member row; clicking selects the agent (or falls back to opening the Agent Lab).
fn render_member_row(
    idx: usize,
    profile: &Profile,
    nav: NavHandle,
    on_select_agent: Option<Rc<SelectSidebarAgentCallback>>,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let prof_id = profile.id.clone();
    div()
        .id(format!("{MEMBER_ROW_ID_PREFIX}{idx}"))
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, window, cx| {
            if let Some(ref cb) = on_select_agent {
                cb(prof_id.clone(), window, cx);
            } else {
                nav(NavigationTab::AgentLab, window, cx);
            }
        })
        .child(render_member_identity(profile, theme))
        .child(render_mbti_chip(profile, theme))
}

fn render_member_identity(profile: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .min_w_0()
        .child(render_profile_avatar(profile, theme.avatar_size_sm, theme))
        .child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .child(
                    div()
                        .text_size(theme.font_size_xs)
                        .font_medium()
                        .text_color(theme.text_primary)
                        .truncate()
                        .child(profile.name.clone()),
                )
                .child(
                    div()
                        .text_size(theme.font_size_2xs)
                        .text_color(theme.text_muted)
                        .truncate()
                        .child(profile.display_handle()),
                ),
        )
}

fn render_mbti_chip(profile: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .flex_shrink_0()
        .child(
            div()
                .px(theme.spacing_xs)
                .py(theme.spacing_3xs)
                .rounded(theme.radius_xs)
                .bg(theme.bg_card)
                .border(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .text_color(theme.accent_agent)
                .child(profile.config.mbti.code()),
        )
}
