//! Relationships (agents): how they answer an @mention and how they handle disagreement, as
//! illustrated cards, summed up on top as two rows of heart pips (warmth and calm).

use super::super::super::common::{card, heading, muted};
use crate::domain::{ConflictHandlingStyle, MentionAttitude};
use crate::theme::ThemeTokens;
use crate::state::forge::{TextField, TraitField};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

const RAPPORT_KEY: &str = "forge.ui.relationships.rapport";
const RAPPORT_HINT_KEY: &str = "forge.ui.relationships.rapport_hint";
const WARMTH_KEY: &str = "forge.ui.relationships.warmth";
const CALM_KEY: &str = "forge.ui.relationships.calm";
const MENTIONS_KEY: &str = "forge.ui.relationships.mentions";
const CONFLICT_KEY: &str = "forge.ui.relationships.conflict";
/// Heart pips per meter.
pub(super) const MAX_HEARTS: u8 = 5;

/// How warmly they greet whoever @mentions them, in hearts.
pub(super) const fn warmth(attitude: MentionAttitude) -> u8 {
    match attitude {
        MentionAttitude::AloofSelective => 1,
        MentionAttitude::AnalyticalDebater | MentionAttitude::DevilAdvocate => 2,
        MentionAttitude::SocraticQuestioning => 3,
        MentionAttitude::PlayfulWitty => 4,
        MentionAttitude::WarmAndReceptive | MentionAttitude::EmpatheticListener => 5,
    }
}

/// How calm and conciliatory they stay in a disagreement, in hearts.
pub(super) const fn calm(style: ConflictHandlingStyle) -> u8 {
    match style {
        ConflictHandlingStyle::PassionateConviction => 1,
        ConflictHandlingStyle::SardonicWit => 2,
        ConflictHandlingStyle::CalmRationalRefutation => 3,
        ConflictHandlingStyle::DialecticalSynthesis => 4,
        ConflictHandlingStyle::DeescalateHarmonize => 5,
    }
}

/// A caption and `filled` of [`MAX_HEARTS`] hearts.
fn heart_row(caption: String, filled: u8, theme: &ThemeTokens) -> Div {
    let pips = (0..MAX_HEARTS).map(|at| {
        let color = if at < filled { theme.accent_danger } else { theme.border_strong };
        div().text_color(color).child(Icon::new(LucideIcon::Heart).size(theme.forge_heart_size))
    });
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(caption))
        .child(div().flex().gap(theme.spacing_2xs).children(pips))
}

impl ForgeView {
    pub(in super::super) fn relationships_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let cfg = &self.draft.profile().config;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .child(
                card(theme)
                    .child(heading(tr(RAPPORT_KEY), theme))
                    .child(heart_row(tr(WARMTH_KEY), warmth(cfg.mention_attitude), theme))
                    .child(heart_row(tr(CALM_KEY), calm(cfg.conflict_handling), theme))
                    .child(muted(tr(RAPPORT_HINT_KEY), theme)),
            )
            .child(
                card(theme)
                    .child(heading(tr(MENTIONS_KEY), theme))
                    .child(self.trait_block(TraitField::MentionAttitude, handle))
                    .child(self.text_row(TextField::CustomMentionInstruction, handle)),
            )
            .child(card(theme).child(heading(tr(CONFLICT_KEY), theme)).child(self.trait_block(TraitField::ConflictHandling, handle)))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests;
