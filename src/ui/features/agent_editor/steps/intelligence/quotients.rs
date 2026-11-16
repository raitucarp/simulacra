//! Sub-tab "Intelligence Quotients": Gf / Gc / EQ / SQ / existential trait meters.

use crate::ui::components::{card_stack, section_card};
use crate::ui::features::agent_editor::steps::trait_grid::{trait_card, trait_row, TraitField};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "MULTIPLE INTELLIGENCE QUOTIENTS (CHC / GARDNER / GOLEMAN)";
const SUBTITLE: &str = "Configure fluid reasoning (Gf), crystallized knowledge (Gc), emotional intelligence (EQ), social intuition (SQ), and existential depth.";

const COGNITIVE_ROW: [TraitField; 2] = [
    TraitField {
        id: "iq-fluid",
        title: "Fluid Intelligence (Gf)",
        subtitle: "Novel pattern recognition & abstract reasoning",
        accent: |t| t.accent_brand,
        get: |p| p.config.fluid_intelligence,
        set: |p, v| p.config.fluid_intelligence = v,
    },
    TraitField {
        id: "iq-crystallized",
        title: "Crystallized Intelligence (Gc)",
        subtitle: "Historical, cultural & domain mastery",
        accent: |t| t.accent_user,
        get: |p| p.config.crystallized_intelligence,
        set: |p, v| p.config.crystallized_intelligence = v,
    },
];

const SOCIAL_ROW: [TraitField; 2] = [
    TraitField {
        id: "iq-emotional",
        title: "Emotional Intelligence (EQ)",
        subtitle: "Affective attunement & empathy",
        accent: |t| t.accent_success,
        get: |p| p.config.emotional_intelligence,
        set: |p, v| p.config.emotional_intelligence = v,
    },
    TraitField {
        id: "iq-social",
        title: "Social Intelligence (SQ)",
        subtitle: "Pragmatics, diplomacy & group dynamics",
        accent: |t| t.accent_dialectic,
        get: |p| p.config.social_intelligence,
        set: |p, v| p.config.social_intelligence = v,
    },
];

const EXISTENTIAL: TraitField = TraitField {
    id: "iq-existential",
    title: "Existential & Philosophical Intelligence",
    subtitle: "Capacity for ultimate questions of meaning, ethics, and being",
    accent: |t| t.accent_agent,
    get: |p| p.config.existential_intelligence,
    set: |p, v| p.config.existential_intelligence = v,
};

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    card_stack(ctx.theme).child(
        section_card(TITLE, SUBTITLE, ctx.theme)
            .child(trait_row(&COGNITIVE_ROW, ctx))
            .child(trait_row(&SOCIAL_ROW, ctx))
            .child(trait_card(&EXISTENTIAL, ctx)),
    )
}
