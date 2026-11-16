//! Segmented `Network Agents | My Personas` switch plus active-network subtitle.

use crate::theme::ThemeTokens;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use super::params::{LabMode, RosterData, RosterParams};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const AGENTS_PILL_ID: &str = "mode-pill-network-agents";
const PERSONAS_PILL_ID: &str = "mode-pill-persona-lab";

/// The callback of a mode pill.
type SwitchCallback = Rc<dyn Fn(LabMode, &mut Window, &mut App)>;

pub(super) fn render_mode_switch(params: &RosterParams<'_>, data: &RosterData<'_>, on_switch: SwitchCallback) -> Div {
    let theme = params.theme;
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
                .child(mode_pill(LabMode::NetworkAgents, params.mode, data, theme, on_switch.clone()))
                .child(mode_pill(LabMode::PersonaLab, params.mode, data, theme, on_switch)),
        )
        .child(scope_subtitle(&data.network_name, data.contacts.len(), theme))
}

fn mode_pill(
    target: LabMode,
    current: LabMode,
    data: &RosterData<'_>,
    theme: &ThemeTokens,
    on_switch: SwitchCallback,
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
