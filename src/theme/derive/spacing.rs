//! Semantic layout spacing: the roles UI code pads and spaces panels with, each mapped onto a
//! step of the density-scaled spacing grid so every role follows the user's density.
//!
//! The ladder pairs padding with gaps: a panel (`panel_padding`, 16px) stacks its sections
//! `section_gap` (16px) apart; a card (`card_padding`, 12px) stacks its blocks `stack_gap`
//! (12px) apart; a narrow rail or sidebar (`panel_padding_compact`, 8px) leaves the inset to
//! its rows; controls sit `control_gap` (8px) apart and an icon `inline_gap` (4px) from its
//! label. Pixel values are at the default density.

use crate::theme::ThemeTokens;

/// Maps every semantic spacing role onto the spacing grid; runs after the grid is derived.
pub(super) fn apply_semantic_spacing(t: &mut ThemeTokens) {
    set_tokens!(t {
        panel_padding: t.spacing_lg,
        panel_padding_compact: t.spacing_sm,
        section_gap: t.spacing_lg,
        stack_gap: t.spacing_md,
        inline_gap: t.spacing_xs,
        card_padding: t.spacing_md,
        control_gap: t.spacing_sm,
        paragraph_break_height: t.spacing_sm,
    });
}
