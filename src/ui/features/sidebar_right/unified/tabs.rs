//! Segmented tab switcher: Mind & Cadence vs Vault & Telemetry.

use std::rc::Rc;

use super::{RightSidebarTab, SelectTabCallback};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// Static description of one segmented tab.
struct TabSpec {
    tab: RightSidebarTab,
    id: &'static str,
    icon: LucideIcon,
    label_key: &'static str,
}

const TABS: [TabSpec; 2] = [
    TabSpec {
        tab: RightSidebarTab::MindCadence,
        id: "tab-switch-mind-cadence",
        icon: LucideIcon::Share2,
        label_key: "sidebar.mind_cadence",
    },
    TabSpec {
        tab: RightSidebarTab::VaultTelemetry,
        id: "tab-switch-vault-telemetry",
        icon: LucideIcon::Activity,
        label_key: "sidebar.vault_telemetry",
    },
];

fn render_tab_label(spec: &TabSpec, theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_ui.clone())
        .text_xs()
        .font_medium()
        .child(t!(spec.label_key).to_string())
}

fn render_tab(
    spec: &TabSpec,
    active_tab: RightSidebarTab,
    on_select: Rc<SelectTabCallback>,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let is_active = active_tab == spec.tab;
    let tab = spec.tab;
    div()
        .id(spec.id)
        .flex_1()
        .py_1p5()
        .rounded_lg()
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .gap_1p5()
        .when(is_active, |s| {
            s.bg(theme.hover_bg)
                .text_color(theme.text_primary)
                .border_1()
                .border_color(theme.border)
        })
        .when(!is_active, |s| s.text_color(theme.text_muted))
        .hover(move |s| s.text_color(theme.text_primary))
        .on_click(move |_, window, cx| {
            on_select(tab, window, cx);
        })
        .child(div().w_3p5().h_3p5().child(Icon::new(spec.icon)))
        .child(render_tab_label(spec, theme))
}

/// Renders the top segmented tab switcher.
pub(super) fn render_tab_switcher(
    active_tab: RightSidebarTab,
    on_select: Rc<SelectTabCallback>,
    theme: &ThemeTokens,
) -> Div {
    let tabs = TABS
        .iter()
        .map(|spec| render_tab(spec, active_tab, on_select.clone(), theme));
    div()
        .w_full()
        .px(theme.spacing_md)
        .pt(theme.spacing_md)
        .child(
            div()
                .w_full()
                .p_1()
                .rounded_xl()
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border)
                .flex()
                .items_center()
                .gap_1()
                .children(tabs),
        )
}
