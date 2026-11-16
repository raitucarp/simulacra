//! Epistemic dialectic telemetry inspector molecule.
//! Shows the psychology model in use and the network's actual member agents.

mod parts;

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use parts::{render_agent_pill, render_header, render_metric_row};
use crate::i18n::t_count;
use rust_i18n::t;

/// Model names shown as the psychology engine (proper nouns, not translated).
const PSYCHOLOGY_ENGINE_VALUE: &str = "MBTI · OCEAN · Enneagram";
/// Count-aware "N human-like agents" metric (`_one` / `_other` variants).
const MEMBER_AGENTS_KEY: &str = "sidebar.member_agents";

fn render_metrics(member_count: usize, theme: &ThemeTokens) -> Div {
    let members = t_count(MEMBER_AGENTS_KEY, member_count);
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(render_metric_row(t!("sidebar.psychology_engine").to_string(), PSYCHOLOGY_ENGINE_VALUE.to_string(), theme.accent_agent, theme))
        .child(render_metric_row(t!("sidebar.network_members_label").to_string(), members, theme.text_secondary, theme))
}

/// Renders the Dialectic Telemetry Card dynamically reflecting the current network's human-like member agents.
pub fn render_network_agent_roster(agents: &[&Profile], theme: &ThemeTokens) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .child(render_header(theme))
        .child(render_metrics(agents.len(), theme))
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
                .children(agents.iter().map(|agent| render_agent_pill(agent, theme))),
        )
}
