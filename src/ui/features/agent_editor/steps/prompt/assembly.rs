//! Sub-tab "Prompt Assembly": strategy, pre-post reasoning and dossier section switches.

use super::sections::{DossierSection, DOSSIER_SECTIONS};
use crate::domain::{PrePostReasoningMode, PromptAssemblyStrategy};
use crate::ui::components::{
    card_stack, field_label, fill, labeled_group, section_card, split_row, toggle_row,
    EnumChips, ToggleSpec,
};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "PROMPT ASSEMBLY STRATEGY & PRE-POST REASONING";
const SUBTITLE: &str =
    "Configure how all human traits are woven into the prompt before generating a post.";
const STRATEGY_LABEL: &str = "Prompt Assembly Strategy";
const STRATEGY_CHIP_ID: &str = "prompt-strat";
const REASONING_LABEL: &str = "Pre-Post Internal Deliberation";
const REASONING_CHIP_ID: &str = "reason-mode";
const SECTIONS_LABEL: &str = "Injected Human Dossier Sections (Included in Prompt)";
const SECTIONS_PER_ROW: usize = 2;

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(
                split_row(theme.spacing_md)
                    .w_full()
                    .child(strategy_group(ctx).flex_1())
                    .child(reasoning_group(ctx).flex_1()),
            )
            .child(sections_group(ctx)),
    )
}

fn strategy_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.assembly_strategy;
    let chips = EnumChips::new(
        &PromptAssemblyStrategy::ALL,
        current,
        STRATEGY_CHIP_ID,
        PromptAssemblyStrategy::label,
    )
    .render(ctx.theme, ctx.edit(|p, v: PromptAssemblyStrategy| p.config.assembly_strategy = v));
    labeled_group(STRATEGY_LABEL, chips, ctx.theme)
}

fn reasoning_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.pre_post_reasoning;
    let chips = EnumChips::new(
        &PrePostReasoningMode::ALL,
        current,
        REASONING_CHIP_ID,
        PrePostReasoningMode::label,
    )
    .render(ctx.theme, ctx.edit(|p, v: PrePostReasoningMode| p.config.pre_post_reasoning = v));
    labeled_group(REASONING_LABEL, chips, ctx.theme)
}

fn sections_group(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(field_label(SECTIONS_LABEL, theme))
        .children(DOSSIER_SECTIONS.chunks(SECTIONS_PER_ROW).map(|row| {
            split_row(theme.spacing_sm)
                .w_full()
                .children(row.iter().map(|section| fill(section_toggle(section, ctx))))
        }))
}

fn section_toggle(section: &DossierSection, ctx: &StepCtx<'_>) -> Stateful<Div> {
    let spec = ToggleSpec {
        title: section.title,
        subtitle: section.subtitle,
        enabled: (section.enabled)(ctx.profile),
    };
    toggle_row(section.id, spec, ctx.theme, ctx.action(section.flip))
}
