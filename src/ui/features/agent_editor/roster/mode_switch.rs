//! Segmented `Network Agents | My Personas` switch plus active-network subtitle.

use crate::theme::ThemeTokens;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::agent_editor::callbacks::SwitchLabModeCallback;
use crate::ui::features::agent_editor::config_tab::LabMode;
use crate::ui::features::agent_editor::params::WorkspaceScope;
use crate::ui::features::agent_editor::workspace::WorkspaceData;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const AGENTS_PILL_ID: &str = "mode-pill-network-agents";
const PERSONAS_PILL_ID: &str = "mode-pill-persona-lab";

pub(super) fn render_mode_switch(
    scope: &WorkspaceScope<'_>,
    data: &WorkspaceData<'_>,
    on_switch: Rc<SwitchLabModeCallback>,
) -> Div {
    let theme = scope.theme;
    div()
        .w_full()
        .p(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(
            segmented_track(theme)
                .child(mode_pill(LabMode::NetworkAgents, scope.mode, data, theme, on_switch.clone()))
                .child(mode_pill(LabMode::PersonaLab, scope.mode, data, theme, on_switch)),
        )
        .child(scope_subtitle(&data.network_name, data.contact_list.len(), theme))
}

fn mode_pill(
    target: LabMode,
    current: LabMode,
    data: &WorkspaceData<'_>,
    theme: &ThemeTokens,
    on_switch: Rc<SwitchLabModeCallback>,
) -> Stateful<Div> {
    let segment = match target {
        LabMode::NetworkAgents => Segment {
            id: AGENTS_PILL_ID.into(),
            icon: LucideIcon::Users,
            label: format!("{} ({})", t!("lab.mode_agents"), data.agents_count),
            active: current == target,
            active_icon_color: theme.accent_brand,
        },
        LabMode::PersonaLab => Segment {
            id: PERSONAS_PILL_ID.into(),
            icon: LucideIcon::UserCheck,
            label: format!("{} ({})", t!("lab.mode_personas"), data.personas_count),
            active: current == target,
            active_icon_color: theme.accent_user,
        },
    };
    segment_pill(segment, theme, move |w, cx| on_switch(target, w, cx))
}

/// Upper-cased active network name with the roster count on the right.
fn scope_subtitle(network_name: &str, count: usize, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_xs)
        .pt(theme.spacing_2xs)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .truncate()
                .child(network_name.to_uppercase()),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.accent_brand)
                .child(format!("{}", count)),
        )
}
