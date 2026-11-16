//! Knowledge Vault tab pane: what the local vault actually stores for the active network.

use super::common::{card, muted_note, section, section_label};
use crate::theme::ThemeTokens;
use gpui_kit::{div, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// Helper function to render Knowledge Vault Tab Pane
pub(in crate::ui::features::settings) fn render_vault_pane(theme: &ThemeTokens) -> impl IntoElement {
    div().flex().flex_col().gap_5().child(section(
        section_label(theme, t!("settings.vault_title").to_string()),
        card(theme).gap_3().child(muted_note(theme, t!("settings.vault_note").to_string())),
    ))
}
