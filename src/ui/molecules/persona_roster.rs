//! Epistemic dialectic telemetry inspector molecule.
//! Provides real-time visibility into vector indexing, agent cadences, and active network members.

use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Renders the Dialectic Engine & Local Vector Telemetry Card in the right sidebar.
pub fn render_persona_roster(theme: &ThemeTokens) -> impl IntoElement {
    render_network_agent_roster(&[], theme)
}

/// Renders the Dialectic Telemetry Card dynamically reflecting the current network's human-like member agents.
pub fn render_network_agent_roster(agents: &[&Profile], theme: &ThemeTokens) -> impl IntoElement {
    let badge_names: Vec<(String, String)> = if agents.is_empty() {
        vec![
            ("Sokrates".to_string(), "ENTP-A".to_string()),
            ("Baruch de Spinoza".to_string(), "INTJ-A".to_string()),
        ]
    } else {
        agents
            .iter()
            .map(|a| (a.name.clone(), a.config.mbti_full_code()))
            .collect()
    };

    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        // Header
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_sm)
                        .child(
                            div()
                                .w(theme.font_size_base)
                                .h(theme.font_size_base)
                                .text_color(theme.accent_agent)
                                .child(Icon::new(LucideIcon::Cpu)),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_bold()
                                .text_color(theme.text_primary)
                                .child("NETWORK MEMBER TELEMETRY"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_xs)
                        .px(theme.spacing_sm)
                        .py(theme.spacing_2xs)
                        .rounded(theme.radius_full)
                        .bg(theme.hover_bg)
                        .border(theme.border_width_hairline)
                        .border_color(theme.border)
                        .child(
                            div()
                                .w(theme.font_size_xs)
                                .h(theme.font_size_xs)
                                .text_color(theme.accent_success)
                                .child(Icon::new(LucideIcon::Activity)),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_medium()
                                .text_color(theme.accent_success)
                                .child("Active"),
                        ),
                ),
        )
        // Telemetry Metrics
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme.spacing_sm)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .text_color(theme.text_muted)
                                .child("Vector Engine"),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_medium()
                                .text_color(theme.text_secondary)
                                .child("nano-vectordb (384d)"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .text_color(theme.text_muted)
                                .child("Psychology Engine"),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_medium()
                                .text_color(theme.accent_agent)
                                .child("MBTI · OCEAN · Enneagram"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .text_color(theme.text_muted)
                                .child("Network Members"),
                        )
                        .child(
                            div()
                                .text_size(theme.font_size_xs)
                                .font_medium()
                                .text_color(theme.text_secondary)
                                .child(format!("{} human-like agents", badge_names.len())),
                        ),
                ),
        )
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
                .children(badge_names.into_iter().map(|(name, mbti)| {
                    div()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_xs)
                        .px(theme.spacing_sm)
                        .py(theme.spacing_2xs)
                        .rounded(theme.radius_sm)
                        .bg(theme.hover_bg)
                        .text_size(theme.font_size_2xs)
                        .font_medium()
                        .text_color(theme.accent_agent)
                        .child(
                            div()
                                .w(theme.font_size_2xs)
                                .h(theme.font_size_2xs)
                                .child(Icon::new(LucideIcon::User)),
                        )
                        .child(format!("{} · {}", name, mbti))
                })),
        )
}
