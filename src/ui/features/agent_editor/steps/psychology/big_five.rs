//! Sub-tab "Big Five (OCEAN) & HEXACO": six trait meters in a 2-column grid.

use crate::ui::components::{card_stack, section_card};
use crate::ui::features::agent_editor::steps::trait_grid::{trait_row, TraitField};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "BIG FIVE (OCEAN) & HEXACO DIMENSIONS";
const SUBTITLE: &str =
    "Empirical six-factor personality profile arranged in a balanced 2-column matrix.";
const COLUMNS: usize = 2;

const TRAITS: [TraitField; 6] = [
    TraitField {
        id: "ocean-card-0",
        title: "Openness",
        subtitle: "Curiosity, aesthetics & imagination",
        accent: |t| t.accent_brand,
        get: |p| p.config.openness,
        set: |p, v| p.config.openness = v,
    },
    TraitField {
        id: "ocean-card-1",
        title: "Conscientiousness",
        subtitle: "Discipline, precision & orderliness",
        accent: |t| t.accent_user,
        get: |p| p.config.conscientiousness,
        set: |p, v| p.config.conscientiousness = v,
    },
    TraitField {
        id: "ocean-card-2",
        title: "Extraversion",
        subtitle: "Social energy & assertiveness",
        accent: |t| t.accent_dialectic,
        get: |p| p.config.extraversion,
        set: |p, v| p.config.extraversion = v,
    },
    TraitField {
        id: "ocean-card-3",
        title: "Agreeableness",
        subtitle: "Warmth, empathy & cooperative harmony",
        accent: |t| t.accent_success,
        get: |p| p.config.agreeableness,
        set: |p, v| p.config.agreeableness = v,
    },
    TraitField {
        id: "ocean-card-4",
        title: "Neuroticism",
        subtitle: "Affective sensitivity & vigilance",
        accent: |t| t.accent_danger,
        get: |p| p.config.neuroticism,
        set: |p, v| p.config.neuroticism = v,
    },
    TraitField {
        id: "ocean-card-5",
        title: "Honesty-Humility (HEXACO)",
        subtitle: "Sincerity, modesty & epistemic candor",
        accent: |t| t.accent_agent,
        get: |p| p.config.honesty_humility,
        set: |p, v| p.config.honesty_humility = v,
    },
];

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    card_stack(ctx.theme).child(
        section_card(TITLE, SUBTITLE, ctx.theme)
            .children(TRAITS.chunks(COLUMNS).map(|row| trait_row(row, ctx))),
    )
}
