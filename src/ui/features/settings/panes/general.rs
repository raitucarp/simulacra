//! General tab pane: autonomous engine, local storage vault and interface locale.

use super::super::consts::{ICON_LG, MONO_FONT_STACK, TEXT_SMALL};
use super::super::general_props::GeneralPaneProps;
use super::common::{body_text, card, muted_note, section, section_label, spaced_row, title_with_subtitle};
use super::engine::render_engine_section;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, px, Div, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// Helper function to render General Tab Pane
pub(in crate::ui::features::settings) fn render_general_pane(
    theme: &ThemeTokens,
    mut props: GeneralPaneProps,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(render_engine_section(theme, &mut props))
        .child(render_storage_section(theme, &props.db_path))
        .child(render_locale_section(theme))
}

/// Section 2: Local SQLite Vault Storage
fn render_storage_section(theme: &ThemeTokens, db_path: &str) -> Div {
    section(
        section_label(theme, "LOCAL STORAGE & EPISTEMIC VAULT"),
        card(theme)
            .gap_3()
            .child(
                spaced_row()
                    .child(title_with_subtitle(
                        theme,
                        t!("scheduler.db_path_title").to_string(),
                        db_path.to_string(),
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
