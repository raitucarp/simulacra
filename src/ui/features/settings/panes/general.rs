//! General tab pane: autonomous engine, local storage vault and interface locale.

use super::super::consts::{ICON_LG, ICON_XS, MONO_FONT_STACK, TEXT_SMALL};
use super::common::{body_text, card, muted_note, section, section_label, spaced_row, title_with_subtitle};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, px, Div, FontWeight, IntoElement, ParentElement, Styled};

/// Helper function to render General Tab Pane
pub(in crate::ui::features::settings) fn render_general_pane(theme: &ThemeTokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(render_engine_section(theme))
        .child(render_storage_section(theme))
        .child(render_locale_section(theme))
}

/// Section 1: Autonomous Dialectic Engine
fn render_engine_section(theme: &ThemeTokens) -> Div {
    section(
        spaced_row()
            .child(section_label(theme, "AUTONOMOUS DIALECTIC ENGINE"))
            .child(render_active_loop_badge(theme)),
        card(theme)
            .gap_2()
            .child(body_text(theme, "Autonomous Background Heartbeat: 15-minute cycle"))
            .child(muted_note(theme, "Simulacra's autonomous loop ticks on new thoughts and scheduled intervals, enabling each network's human-like member agents to dialectically question and link memory nodes.")),
    )
}

/// Emerald "Active Loop" status pill.
fn render_active_loop_badge(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1p5()
        .px_2()
        .py_0p5()
        .rounded_full()
        .bg(theme.accent_success_subtle)
        .border_1()
        .border_color(theme.accent_success_border)
        .child(Icon::new(LucideIcon::Activity).size(px(ICON_XS)).text_color(theme.accent_success))
        .child(
            div()
                .text_size(px(TEXT_SMALL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.accent_success)
                .child("Active Loop"),
        )
}

/// Section 2: Local SQLite Vault Storage
fn render_storage_section(theme: &ThemeTokens) -> Div {
    section(
        section_label(theme, "LOCAL STORAGE & EPISTEMIC VAULT"),
        card(theme)
            .gap_3()
            .child(
                spaced_row()
                    .child(title_with_subtitle(
                        theme,
                        "Database Path",
                        "~/.simulacra/knowledge.db (WAL Mode)",
                        MONO_FONT_STACK,
                    ))
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(theme.hover_bg)
                            .border_1()
                            .border_color(theme.border)
                            .text_size(px(TEXT_SMALL))
                            .text_color(theme.text_secondary)
                            .child("Schema v1.4"),
                    ),
            )
            .child(muted_note(theme, "All dialectic posts, wikilinks, vectors, and API keys are stored in your encrypted local SQLite database. No telemetry is sent to any centralized cloud.")),
    )
}

/// Section 3: Interface Language & Locale
fn render_locale_section(theme: &ThemeTokens) -> Div {
    section(
        section_label(theme, "INTERFACE LOCALE"),
        div()
            .p_3()
            .rounded_lg()
            .bg(theme.bg_card)
            .border_1()
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(Icon::new(LucideIcon::Globe).size(px(ICON_LG)).text_color(theme.text_muted))
                    .child(body_text(theme, "Dialectic Language Support")),
            )
            .child(
                div()
                    .text_size(px(TEXT_SMALL))
                    .text_color(theme.text_secondary)
                    .child("English (US) / Indonesian (Bilingual)"),
            ),
    )
}
