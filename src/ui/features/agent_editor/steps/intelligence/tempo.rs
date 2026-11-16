//! Sub-tab "Tempo & Humor": System 1 / System 2 pace and sense of humor.

use crate::domain::{CognitivePace, HumorStyle};
use crate::ui::components::{card_stack, section_card, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const PACE_TITLE: &str = "COGNITIVE TEMPO (SYSTEM 1 / SYSTEM 2)";
const PACE_SUBTITLE: &str = "Deliberation speed and associative rhythm.";
const PACE_CHIP_ID: &str = "cog-pace";
const HUMOR_TITLE: &str = "SENSE OF HUMOR & WIT";
const HUMOR_SUBTITLE: &str = "How this person uses irony, levity, or deadpan wit in conversation.";
const HUMOR_CHIP_ID: &str = "humor-style";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, cfg) = (ctx.theme, &ctx.profile.config);
    let pace = EnumChips::new(&CognitivePace::ALL, cfg.cognitive_pace, PACE_CHIP_ID, CognitivePace::label)
        .render(theme, ctx.edit(|p, v: CognitivePace| p.config.cognitive_pace = v));
    let humor = EnumChips::new(&HumorStyle::ALL, cfg.humor_style, HUMOR_CHIP_ID, HumorStyle::label)
        .render(theme, ctx.edit(|p, v: HumorStyle| p.config.humor_style = v));
    card_stack(theme)
        .child(section_card(PACE_TITLE, PACE_SUBTITLE, theme).child(pace))
        .child(section_card(HUMOR_TITLE, HUMOR_SUBTITLE, theme).child(humor))
}
