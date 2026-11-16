//! Bottom identity anchor: full user card when expanded, avatar-only when collapsed.

use super::user_card::render_interactive_user_card;
use super::{NavHandle, SwitchUserPersonaCallback};
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
};
use std::rc::Rc;

/// Element id of the collapsed-rail avatar button.
const COLLAPSED_AVATAR_ID: &str = "collapsed-sidebar-user-avatar";

pub(super) fn render_user_anchor(
    state: &AppState,
    is_collapsed: bool,
    on_select_nav: &NavHandle,
    on_switch_persona: Option<Rc<SwitchUserPersonaCallback>>,
    theme: &ThemeTokens,
) -> Div {
    let nav_user_card = on_select_nav.clone();
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
            this.child(render_collapsed_avatar(state, on_select_nav.clone(), theme))
        })
}

fn render_collapsed_avatar(state: &AppState, nav: NavHandle, theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(COLLAPSED_AVATAR_ID)
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .on_click(move |_, window, cx| {
            nav(NavigationTab::PersonaLab, window, cx);
        })
        .child(if let Some(prof) = state.current_profile() {
            render_profile_avatar(prof, theme.avatar_size_md, theme)
        } else {
            div()
                .w(theme.avatar_size_md)
                .h(theme.avatar_size_md)
                .rounded(theme.radius_full)
                .bg(theme.bg_app)
        })
}
