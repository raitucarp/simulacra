//! Sub-tab "@Mention & Conflict": reaction when tagged and disagreement style.

use crate::domain::{ConflictHandlingStyle, MentionAttitude};
use crate::ui::components::{card_stack, labeled_group, labeled_textarea, section_card, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "ATTITUDE WHEN @MENTIONED & INTELLECTUAL DISAGREEMENT";
const SUBTITLE: &str =
    "How this person reacts when tagged (@handle) in a post or challenged in debate.";
const MENTION_LABEL: &str = "Attitude When @Mentioned";
const MENTION_CHIP_ID: &str = "mention-att";
const CONFLICT_LABEL: &str = "Intellectual Disagreement & Conflict Style";
const CONFLICT_CHIP_ID: &str = "conflict-style";
const INSTRUCTION_LABEL: &str = "Custom @Mention Behavioral Instruction";
const SAVE_BUTTON_ID: &str = "save-schedule-sub1-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(mention_group(ctx))
            .child(conflict_group(ctx))
            .child(labeled_textarea(INSTRUCTION_LABEL, ctx.inputs.mention_instruction_input, theme))
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

fn mention_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.mention_attitude;
    let chips = EnumChips::new(&MentionAttitude::ALL, current, MENTION_CHIP_ID, MentionAttitude::label)
        .render(ctx.theme, ctx.edit(|p, v: MentionAttitude| p.config.mention_attitude = v));
    labeled_group(MENTION_LABEL, chips, ctx.theme)
}

fn conflict_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.conflict_handling;
    let chips = EnumChips::new(
        &ConflictHandlingStyle::ALL,
        current,
        CONFLICT_CHIP_ID,
        ConflictHandlingStyle::label,
    )
    .render(ctx.theme, ctx.edit(|p, v: ConflictHandlingStyle| p.config.conflict_handling = v));
    labeled_group(CONFLICT_LABEL, chips, ctx.theme)
}
