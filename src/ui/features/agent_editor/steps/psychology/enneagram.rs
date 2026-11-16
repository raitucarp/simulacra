//! Sub-tab "Enneagram": type & wing, instinctual stacking and tritype archetype.

use crate::domain::{EnneagramType, InstinctualVariant};
use crate::ui::components::{card_stack, fill, section_card, split_row, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const ENNEAGRAM_TITLE: &str = "ENNEAGRAM OF PERSONALITY & WING";
const ENNEAGRAM_SUBTITLE: &str = "Core motivational dynamics across the 9 Enneagram types and wings.";
const ENNEAGRAM_CHIP_ID: &str = "ennea-chip";
const INSTINCT_TITLE: &str = "INSTINCTUAL STACKING";
const INSTINCT_SUBTITLE: &str = "Dominant / secondary biological drive (sp/so/sx).";
const INSTINCT_CHIP_ID: &str = "instinct-chip";
const TRITYPE_TITLE: &str = "TRITYPE ARCHETYPE";
const TRITYPE_SUBTITLE: &str = "3-center triad (Head · Heart · Gut).";
const TRITYPE_CHIP_ID: &str = "tritype-preset";

/// (tritype code, chip label) presets.
const TRITYPES: [(&str, &str); 9] = [
    ("514", "514 · Philosopher"),
    ("583", "583 · Strategist"),
    ("549", "549 · Contemplative"),
    ("459", "459 · Mystic"),
    ("145", "145 · Researcher"),
    ("748", "748 · Messenger"),
    ("358", "358 · SolutionMinder"),
    ("853", "853 · Trailblazer"),
    ("954", "954 · Gentle Sage"),
];

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    card_stack(theme).child(enneagram_card(ctx)).child(
        split_row(theme.spacing_md)
            .w_full()
            .child(fill(instinct_card(ctx)))
            .child(fill(tritype_card(ctx))),
    )
}

fn enneagram_card(ctx: &StepCtx<'_>) -> Div {
    let cfg = &ctx.profile.config;
    let chips = EnumChips::new(&EnneagramType::ALL, cfg.enneagram, ENNEAGRAM_CHIP_ID, EnneagramType::title)
        .key(|e| e.code().to_string())
        .render(ctx.theme, ctx.edit(|p, e: EnneagramType| p.config.enneagram = e));
    section_card(ENNEAGRAM_TITLE, ENNEAGRAM_SUBTITLE, ctx.theme).child(chips)
}

fn instinct_card(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.instinctual_variant;
    let chips = EnumChips::new(&InstinctualVariant::ALL, current, INSTINCT_CHIP_ID, InstinctualVariant::label)
        .key(|i| i.code().to_string())
        .render(ctx.theme, ctx.edit(|p, i: InstinctualVariant| p.config.instinctual_variant = i));
    section_card(INSTINCT_TITLE, INSTINCT_SUBTITLE, ctx.theme).child(chips)
}

fn tritype_card(ctx: &StepCtx<'_>) -> Div {
    let current = TRITYPES.iter().copied().find(|(code, _)| *code == ctx.profile.config.tritype);
    let chips = EnumChips::new(&TRITYPES, current, TRITYPE_CHIP_ID, |(_, label)| label)
        .key(|(code, _)| code.to_string())
        .render(ctx.theme, ctx.edit(|p, (code, _): (&str, &str)| p.config.tritype = code.to_string()));
    section_card(TRITYPE_TITLE, TRITYPE_SUBTITLE, ctx.theme).child(chips)
}
