//! Sub-tab "Attachment & Ethics": attachment, moral foundation, temperament, locus of control.

use crate::domain::{AttachmentStyle, ClassicTemperament, LocusOfControl, MoralFoundation};
use crate::ui::components::{card_stack, fill, section_card, split_row, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const ATTACHMENT_TITLE: &str = "ATTACHMENT STYLE (BOWLBY / AINSWORTH)";
const ATTACHMENT_SUBTITLE: &str = "Relational trust and interpersonal bonding pattern.";
const ATTACHMENT_CHIP_ID: &str = "attach-chip";
const MORAL_TITLE: &str = "MORAL FOUNDATION (JONATHAN HAIDT)";
const MORAL_SUBTITLE: &str = "Primary ethical lens when evaluating dilemmas.";
const MORAL_CHIP_ID: &str = "moral-chip";
const TEMPERAMENT_TITLE: &str = "CLASSICAL TEMPERAMENT";
const TEMPERAMENT_SUBTITLE: &str = "Foundational affective temperament.";
const TEMPERAMENT_CHIP_ID: &str = "temp-chip";
const LOCUS_TITLE: &str = "LOCUS OF CONTROL (ROTTER)";
const LOCUS_SUBTITLE: &str = "Agency attribution in human affairs.";
const LOCUS_CHIP_ID: &str = "locus-chip";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    card_stack(theme)
        .child(
            split_row(theme.spacing_md)
                .w_full()
                .child(fill(attachment_card(ctx)))
                .child(fill(moral_card(ctx))),
        )
        .child(
            split_row(theme.spacing_md)
                .w_full()
                .child(fill(temperament_card(ctx)))
                .child(fill(locus_card(ctx))),
        )
}

fn attachment_card(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.attachment_style;
    let chips = EnumChips::new(&AttachmentStyle::ALL, current, ATTACHMENT_CHIP_ID, AttachmentStyle::label)
        .render(ctx.theme, ctx.edit(|p, v: AttachmentStyle| p.config.attachment_style = v));
    section_card(ATTACHMENT_TITLE, ATTACHMENT_SUBTITLE, ctx.theme).child(chips)
}

fn moral_card(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.moral_foundation;
    let chips = EnumChips::new(&MoralFoundation::ALL, current, MORAL_CHIP_ID, MoralFoundation::label)
        .render(ctx.theme, ctx.edit(|p, v: MoralFoundation| p.config.moral_foundation = v));
    section_card(MORAL_TITLE, MORAL_SUBTITLE, ctx.theme).child(chips)
}

fn temperament_card(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.temperament;
    let chips = EnumChips::new(&ClassicTemperament::ALL, current, TEMPERAMENT_CHIP_ID, ClassicTemperament::label)
        .render(ctx.theme, ctx.edit(|p, v: ClassicTemperament| p.config.temperament = v));
    section_card(TEMPERAMENT_TITLE, TEMPERAMENT_SUBTITLE, ctx.theme).child(chips)
}

fn locus_card(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.locus_of_control;
    let chips = EnumChips::new(&LocusOfControl::ALL, current, LOCUS_CHIP_ID, LocusOfControl::label)
        .render(ctx.theme, ctx.edit(|p, v: LocusOfControl| p.config.locus_of_control = v));
    section_card(LOCUS_TITLE, LOCUS_SUBTITLE, ctx.theme).child(chips)
}
