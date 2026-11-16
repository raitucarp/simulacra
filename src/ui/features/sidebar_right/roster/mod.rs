//! Epistemic dialectic telemetry inspector molecule.
//! Provides real-time visibility into vector indexing, agent cadences, and active network members.

mod parts;

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use parts::{render_agent_pill, render_header, render_metric_row};

/// Placeholder members shown when the network has no agents: `(name, MBTI code)`.
const DEFAULT_ROSTER: [(&str, &str); 2] = [("Sokrates", "ENTP-A"), ("Baruch de Spinoza", "INTJ-A")];
const VECTOR_ENGINE_LABEL: &str = "Vector Engine";
const VECTOR_ENGINE_VALUE: &str = "nano-vectordb (384d)";
const PSYCHOLOGY_ENGINE_LABEL: &str = "Psychology Engine";
const PSYCHOLOGY_ENGINE_VALUE: &str = "MBTI · OCEAN · Enneagram";
const NETWORK_MEMBERS_LABEL: &str = "Network Members";

/// Renders the Dialectic Engine & Local Vector Telemetry Card in the right sidebar.
pub fn render_persona_roster(theme: &ThemeTokens) -> impl IntoElement {
    render_network_agent_roster(&[], theme)
}

fn badge_names(agents: &[&Profile]) -> Vec<(String, String)> {
    if agents.is_empty() {
        DEFAULT_ROSTER
            .iter()
            .map(|(name, mbti)| (name.to_string(), mbti.to_string()))
            .collect()
    } else {
        agents
            .iter()
            .map(|a| (a.name.clone(), a.config.mbti_full_code()))
            .collect()
    }
}

fn render_metrics(member_count: usize, theme: &ThemeTokens) -> Div {
    let members = format!("{} human-like agents", member_count);
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(render_metric_row(VECTOR_ENGINE_LABEL, VECTOR_ENGINE_VALUE.to_string(), theme.text_secondary, theme))
        .child(render_metric_row(PSYCHOLOGY_ENGINE_LABEL, PSYCHOLOGY_ENGINE_VALUE.to_string(), theme.accent_agent, theme))
        .child(render_metric_row(NETWORK_MEMBERS_LABEL, members, theme.text_secondary, theme))
}

/// Renders the Dialectic Telemetry Card dynamically reflecting the current network's human-like member agents.
pub fn render_network_agent_roster(agents: &[&Profile], theme: &ThemeTokens) -> impl IntoElement {
    let badge_names = badge_names(agents);

    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(render_header(theme))
        .child(render_metrics(badge_names.len(), theme))
        // Active Network Agents Pill List
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(theme.spacing_xs)
                .pt(theme.spacing_xs)
                .border_t(theme.border_width_hairline)
                .border_color(theme.border)
                .children(
                    badge_names
                        .into_iter()
                        .map(|(name, mbti)| render_agent_pill(name, mbti, theme)),
                ),
        )
}
