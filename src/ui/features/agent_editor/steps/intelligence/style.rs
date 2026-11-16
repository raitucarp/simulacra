//! Sub-tab "Cognitive Style": reasoning architecture and epistemic stance.

use crate::domain::{CognitiveIntelligenceStyle, EpistemicStance};
use crate::ui::components::{card_stack, section_card, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const COGNITION_TITLE: &str = "PRIMARY COGNITIVE & REASONING ARCHITECTURE";
const COGNITION_SUBTITLE: &str =
    "How this person processes information, constructs arguments, and synthesizes insights.";
const COGNITION_CHIP_ID: &str = "cog-style";
const STANCE_TITLE: &str = "EPISTEMIC STANCE (THEORY OF KNOWLEDGE)";
const STANCE_SUBTITLE: &str = "Criteria for truth, justification, and belief revision.";
const STANCE_CHIP_ID: &str = "epistemic-stance";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, cfg) = (ctx.theme, &ctx.profile.config);
    let cognition = EnumChips::new(
        &CognitiveIntelligenceStyle::ALL,
        cfg.intelligence_style,
        COGNITION_CHIP_ID,
        CognitiveIntelligenceStyle::label,
    )
    .render(theme, ctx.edit(|p, v: CognitiveIntelligenceStyle| p.config.intelligence_style = v));
    let stance = EnumChips::new(&EpistemicStance::ALL, cfg.epistemic_stance, STANCE_CHIP_ID, EpistemicStance::label)
        .render(theme, ctx.edit(|p, v: EpistemicStance| p.config.epistemic_stance = v));
    card_stack(theme)
        .child(section_card(COGNITION_TITLE, COGNITION_SUBTITLE, theme).child(cognition))
        .child(section_card(STANCE_TITLE, STANCE_SUBTITLE, theme).child(stance))
}
