//! Left sidebar organism with Collapsible Icon-Rail (60px) to Expanded (260px) modes.
//! Features unified Personas & Agents navigation, per-network member roster, and interactive identity switching.

use crate::domain::ProfileId;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::{render_interactive_user_card, render_nav_button, WorkspaceTrigger};
use gpui_kit::base::StyledExt;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

pub type SelectNavCallback = Box<dyn Fn(NavigationTab, &mut Window, &mut App) + 'static>;
pub type ToggleWsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type NewThreadCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectSidebarAgentCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type SwitchUserPersonaCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

pub struct LeftSidebarParams<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    pub is_workspace_open: bool,
    pub is_collapsed: bool,
    pub on_toggle_workspace: ToggleWsCallback,
    pub on_select_nav: SelectNavCallback,
    pub on_new_thread: NewThreadCallback,
    pub on_select_agent: Option<SelectSidebarAgentCallback>,
    pub on_switch_persona: Option<SwitchUserPersonaCallback>,
}

pub fn render_left_sidebar(params: LeftSidebarParams<'_>) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;
    let is_collapsed = params.is_collapsed;
    let on_select_nav = Rc::new(params.on_select_nav);
    let on_select_agent = params.on_select_agent.map(Rc::new);
    let on_switch_persona = params.on_switch_persona.map(Rc::new);

    let nav_home = on_select_nav.clone();
    let nav_know = on_select_nav.clone();
    let nav_agents = on_select_nav.clone();
    let nav_graph = on_select_nav.clone();
    let nav_theme = on_select_nav.clone();
    let nav_user_card = on_select_nav.clone();

    let docs_in_network: Vec<_> = state.current_documents().collect();
    let doc_count = docs_in_network.len();
    let posts_count = state.current_network_post_count();
    let network_agents = state.current_network_agents();
    let network_agents_count = network_agents.len();
    let is_lab_active = matches!(
        state.active_tab,
        NavigationTab::AgentLab | NavigationTab::PersonaLab
    );

    let workspace_trigger = WorkspaceTrigger::new(
        state.current_network().cloned(),
        params.is_workspace_open,
        is_collapsed,
        theme.clone(),
        params.on_toggle_workspace,
    );

    div()
        .w_full()
        .h_full()
        .bg(theme.bg_sidebar)
        .flex()
        .flex_col()
        .justify_between()
        .p(theme.spacing_md)
        // Top section
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_md)
                // 1. Workspace Switcher Header
                .child(workspace_trigger)
                // 2. Primary Navigation Views with Lucide Icons (Network Agents & Persona Lab unified!)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(theme.spacing_xs)
                        .child(render_nav_button(
                            &t!("nav.home"),
                            LucideIcon::Activity,
                            state.active_tab == NavigationTab::Timeline,
                            is_collapsed,
                            Some(posts_count),
                            theme,
                            move |_, window, cx| {
                                nav_home(NavigationTab::Timeline, window, cx);
                            },
                        ))
                        .child(render_nav_button(
                            &t!("nav.knowledge"),
                            LucideIcon::BookOpen,
                            state.active_tab == NavigationTab::Knowledge,
                            is_collapsed,
                            Some(doc_count),
                            theme,
                            move |_, window, cx| {
                                nav_know(NavigationTab::Knowledge, window, cx);
                            },
                        ))
                        .child(render_nav_button(
                            &t!("nav.agent_lab"),
                            LucideIcon::Users,
                            is_lab_active,
                            is_collapsed,
                            Some(network_agents_count),
                            theme,
                            move |_, window, cx| {
                                nav_agents(NavigationTab::AgentLab, window, cx);
                            },
                        ))
                        .child(render_nav_button(
                            &t!("nav.mind_graph"),
                            LucideIcon::Share2,
                            state.active_tab == NavigationTab::MindGraph,
                            is_collapsed,
                            None,
                            theme,
                            move |_, window, cx| {
                                nav_graph(NavigationTab::MindGraph, window, cx);
                            },
                        ))
                        .child(render_nav_button(
                            &t!("nav.appearance"),
                            LucideIcon::Palette,
                            state.active_tab == NavigationTab::Appearance,
                            is_collapsed,
                            None,
                            theme,
                            move |_, window, cx| {
                                nav_theme(NavigationTab::Appearance, window, cx);
                            },
                        )),
                )
                // 2.5 Prominent "+ New Thread" Action Button
                .child({
                    let on_new = params.on_new_thread;
                    if is_collapsed {
                        div()
                            .w_full()
                            .flex()
                            .justify_center()
                            .py(theme.spacing_xs)
                            .child(
                                Button::new("new-thread-collapsed-btn")
                                    .primary()
                                    .icon(Icon::new(LucideIcon::Plus))
                                    .on_click(move |_, window, cx| (on_new)(window, cx)),
                            )
                            .into_any_element()
                    } else {
                        div()
                            .w_full()
                            .py(theme.spacing_xs)
                            .child(
                                Button::new("new-thread-expanded-btn")
                                    .primary()
                                    .w_full()
                                    .icon(Icon::new(LucideIcon::Plus))
                                    .label(t!("composer.new_thread").to_string())
                                    .on_click(move |_, window, cx| (on_new)(window, cx)),
                            )
                            .into_any_element()
                    }
                })
                // 3. Grounding Sources Channels (Only in Expanded mode & when docs exist or not in Lab view)
                .when(!is_collapsed && (!is_lab_active || doc_count > 0), |this| {
                    this.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_xs)
                            .pt(theme.spacing_sm)
                            .border_t(theme.border_width_hairline)
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px(theme.spacing_xs)
                                    .child(
                                        div()
                                            .text_size(theme.font_size_2xs)
                                            .font_bold()
                                            .text_color(theme.text_muted)
                                            .child(t!("sidebar.grounding_sources").to_string()),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_2xs)
                                            .text_color(theme.accent_agent)
                                            .child(format!("{doc_count}")),
                                    ),
                            )
                            .children(docs_in_network.into_iter().take(3).enumerate().map(|(idx, doc)| {
                                let hover_bg = theme.hover_bg;
                                let doc_title = doc.title.clone();
                                let nav_k = on_select_nav.clone();

                                div()
                                    .id(format!("ground-doc-sidebar-{}", idx))
                                    .px(theme.spacing_sm)
                                    .py(theme.spacing_xs)
                                    .rounded(theme.radius_sm)
                                    .cursor_pointer()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .hover(move |s| s.bg(hover_bg))
                                    .on_click(move |_, window, cx| {
                                        nav_k(NavigationTab::Knowledge, window, cx);
                                    })
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(theme.spacing_sm)
                                            .min_w_0()
                                            .child(
                                                div()
                                                    .text_color(theme.text_muted)
                                                    .child(Icon::new(LucideIcon::FileText).size(theme.font_size_xs)),
                                            )
                                            .child(
                                                div()
                                                    .text_size(theme.font_size_xs)
                                                    .text_color(theme.text_primary)
                                                    .min_w_0()
                                                    .truncate()
                                                    .child(doc_title),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_2xs)
                                            .text_color(theme.text_muted)
                                            .child(format!("{}c", doc.chunk_count)),
                                    )
                            })),
                    )
                })
                // 4. Network Member Agents Quick Roster
                // Progressive Disclosure: Hidden when `is_lab_active` because the adjacent Contact List Pane
                // already displays the full interactive member roster!
                .when(!is_collapsed && !is_lab_active, |this| {
                    this.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_xs)
                            .pt(theme.spacing_sm)
                            .border_t(theme.border_width_hairline)
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .px(theme.spacing_xs)
                                    .child(
                                        div()
                                            .text_size(theme.font_size_2xs)
                                            .font_bold()
                                            .text_color(theme.text_muted)
                                            .child(t!("sidebar.network_members").to_string()),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_2xs)
                                            .text_color(theme.accent_brand)
                                            .child(format!("{network_agents_count}")),
                                    ),
                            )
                            .children(network_agents.into_iter().enumerate().map(|(idx, profile)| {
                                let hover_bg = theme.hover_bg;
                                let nav_l = on_select_nav.clone();
                                let sel_ag = on_select_agent.clone();
                                let prof_id = profile.id.clone();

                                div()
                                    .id(format!("active-agent-sidebar-{}", idx))
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
                                        if let Some(ref cb) = sel_ag {
                                            cb(prof_id.clone(), window, cx);
                                        } else {
                                            nav_l(NavigationTab::AgentLab, window, cx);
                                        }
                                    })
                                    .child(
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
                                            ),
                                    )
                                    .child(
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
                                            ),
                                    )
                            })),
                    )
                }),
        )
        // Bottom User Profile Anchor (Click to open Persona mode in unified Lab, click Switch to cycle active persona)
        .child(
            div()
                .pt(theme.spacing_sm)
                .border_t(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .when(!is_collapsed, |this| {
                    let sw_cb = on_switch_persona.clone();
                    this.child(render_interactive_user_card(
                        state.current_profile(),
                        theme,
                        move |_, window, cx| {
                            nav_user_card(NavigationTab::PersonaLab, window, cx);
                        },
                        move |_, window, cx| {
                            if let Some(ref cb) = sw_cb {
                                cb(window, cx);
                            }
                        },
                    ))
                })
                .when(is_collapsed, |this| {
                    let nav_collapsed_user = on_select_nav.clone();
                    this.child(
                        div()
                            .id("collapsed-sidebar-user-avatar")
                            .w_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .cursor_pointer()
                            .on_click(move |_, window, cx| {
                                nav_collapsed_user(NavigationTab::PersonaLab, window, cx);
                            })
                            .child(if let Some(prof) = state.current_profile() {
                                render_profile_avatar(prof, theme.avatar_size_md, theme)
                            } else {
                                div()
                                    .w(theme.avatar_size_md)
                                    .h(theme.avatar_size_md)
                                    .rounded(theme.radius_full)
                                    .bg(theme.bg_app)
                            }),
                    )
                }),
        )
}
