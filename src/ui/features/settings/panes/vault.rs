//! Knowledge Vault tab pane: vector embedding engine and semantic resonance threshold.

use super::super::consts::{TEXT_CAPTION, TEXT_SMALL};
use super::common::{body_text, card, muted_note, section, section_label, spaced_row, title_with_subtitle};
use crate::theme::ThemeTokens;
use gpui_kit::{div, px, Div, FontWeight, IntoElement, ParentElement, Styled};

/// Helper function to render Knowledge Vault Tab Pane
pub(in crate::ui::features::settings) fn render_vault_pane(theme: &ThemeTokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(render_embedding_section(theme))
        .child(render_resonance_section(theme))
}

/// Vector RAG Engine
fn render_embedding_section(theme: &ThemeTokens) -> Div {
    section(
        section_label(theme, "DENSE VECTOR EMBEDDING ENGINE"),
        card(theme)
            .gap_3()
            .child(
                spaced_row()
                    .child(title_with_subtitle(
                        theme,
                        "Embedding Engine",
                        "nano-vectordb (384-dimensional dense vectors)",
                        theme.font_family_mono.clone(),
                    ))
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_full()
                            .bg(theme.accent_success_subtle)
                            .text_size(px(TEXT_CAPTION))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.accent_success)
                            .child("Optimized"),
                    ),
            )
            .child(muted_note(theme, "Embeddings run strictly on local memory for fast similarity matching without cloud latency.")),
    )
}

/// Semantic Resonance
fn render_resonance_section(theme: &ThemeTokens) -> Div {
    section(
        section_label(theme, "SEMANTIC RESONANCE & THRESHOLD"),
        card(theme)
            .gap_3()
            .child(
                spaced_row()
                    .child(body_text(theme, "Grounding Resonance Threshold"))
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded_md()
                            .bg(theme.accent_dialectic_subtle)
                            .border_1()
                            .border_color(theme.accent_dialectic_border)
                            .text_size(px(TEXT_SMALL))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.accent_dialectic)
                            .child("94% Strict Resonance"),
                    ),
            )
            .child(muted_note(theme, "Dialectic sparks are filtered to ensure high epistemic coherence against existing vault memories.")),
    )
}
