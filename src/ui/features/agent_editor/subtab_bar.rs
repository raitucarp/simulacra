//! Horizontal sub-tab pill bar above the focused content area.

use super::callbacks::SelectSubTabCallback;
use super::config_tab::AgentConfigTab;
use crate::theme::ThemeTokens;
use crate::ui::components::{pill_tab, PillTab};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use std::rc::Rc;

const SUBTAB_ID_PREFIX: &str = "agent-horiz-subtab";

pub(crate) fn render_subtab_bar(
    active_tab: AgentConfigTab,
    active_sub_tab: usize,
    sub_tabs: Vec<(String, LucideIcon)>,
    theme: &ThemeTokens,
    on_select: Rc<SelectSubTabCallback>,
) -> Div {
    let pills = sub_tabs.into_iter().enumerate().map(|(idx, (label, icon))| {
        let cb = on_select.clone();
        let tab = PillTab {
            id: format!("{}-{:?}-{}", SUBTAB_ID_PREFIX, active_tab, idx).into(),
            icon,
            label,
            active: idx == active_sub_tab,
        };
        pill_tab(tab, theme, move |w, cx| cb(idx, w, cx))
    });
    div()
        .w_full()
        .px(theme.spacing_lg)
        .py(theme.spacing_sm)
        .bg(theme.bg_sidebar)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(div().flex().items_center().flex_wrap().gap(theme.spacing_xs).children(pills))
}
