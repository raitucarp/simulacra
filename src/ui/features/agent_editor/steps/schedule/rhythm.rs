//! Sub-tab "Posting Rhythm": autonomy toggle, cadence, chronotype, hours and topics.

use crate::domain::{HumanChronotype, PostingCadence};
use crate::ui::components::{
    card_stack, field_label, labeled_group, labeled_input, section_card, split_row, toggle_row,
    u8_stepper, EnumChips, StepperSpec, ToggleSpec,
};
use crate::ui::features::agent_editor::steps::trait_grid::{trait_card, TraitField};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "POSTING SCHEDULE, CHRONOTYPE & CIRCADIAN RHYTHM";
const SUBTITLE: &str = "Configure when and how often this network member posts thoughts.";
const ACTIVE_TOGGLE_ID: &str = "agent-active-loop-toggle";
const ACTIVE_TITLE: &str = "Active in Network Autonomous Loop";
const ACTIVE_SUBTITLE: &str =
    "When enabled, this member participates in scheduled and reactive network threads.";
const CADENCE_LABEL: &str = "Posting Frequency & Cadence";
const CADENCE_CHIP_ID: &str = "post-cadence";
const CHRONOTYPE_LABEL: &str = "Circadian Chronotype";
const CHRONOTYPE_CHIP_ID: &str = "chrono";
const HOURS_LABEL: &str = "Active Hours Window (0–23h):";
const HOURS_SEPARATOR: &str = "to";
const START_HOUR_ID: &str = "start-hour-step";
const END_HOUR_ID: &str = "end-hour-step";
const HOUR_UNIT: &str = ":00";
const HOUR_MIN: u8 = 0;
const HOUR_MAX: u8 = 23;
const TOPICS_LABEL: &str = "Preferred Hashtags & Discourse Topics";
const SAVE_BUTTON_ID: &str = "save-schedule-sub0-btn";

const SPONTANEITY: TraitField = TraitField {
    id: "spontaneity-gauge",
    title: "Spontaneity & Serendipity Drive",
    subtitle: "Likelihood of initiating unprompted threads on a fresh spark of inspiration",
    accent: |t| t.accent_dialectic,
    get: |p| p.config.spontaneity_rate,
    set: |p, v| p.config.spontaneity_rate = v,
};

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    let active = ToggleSpec {
        title: ACTIVE_TITLE,
        subtitle: ACTIVE_SUBTITLE,
        enabled: ctx.profile.config.is_active,
    };
    let on_toggle = ctx.action(|p| p.config.is_active = !p.config.is_active);
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(toggle_row(ACTIVE_TOGGLE_ID, active, theme, on_toggle))
            .child(
                split_row(theme.spacing_md)
                    .w_full()
                    .child(cadence_group(ctx).flex_1())
                    .child(chronotype_group(ctx).flex_1()),
            )
            .child(active_hours(ctx))
            .child(trait_card(&SPONTANEITY, ctx))
            .child(labeled_input(TOPICS_LABEL, ctx.inputs.preferred_topics_input, theme))
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

fn cadence_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.posting_cadence;
    let chips = EnumChips::new(&PostingCadence::ALL, current, CADENCE_CHIP_ID, PostingCadence::label)
        .render(ctx.theme, ctx.edit(|p, v: PostingCadence| p.config.posting_cadence = v));
    labeled_group(CADENCE_LABEL, chips, ctx.theme)
}

fn chronotype_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.chronotype;
    let chips = EnumChips::new(&HumanChronotype::ALL, current, CHRONOTYPE_CHIP_ID, HumanChronotype::label)
        .render(ctx.theme, ctx.edit(|p, v: HumanChronotype| p.config.chronotype = v));
    labeled_group(CHRONOTYPE_LABEL, chips, ctx.theme)
}

fn hour_spec(value: u8) -> StepperSpec<'static> {
    StepperSpec { value, min: HOUR_MIN, max: HOUR_MAX, unit: HOUR_UNIT }
}

fn active_hours(ctx: &StepCtx<'_>) -> Div {
    let (theme, cfg) = (ctx.theme, &ctx.profile.config);
    let start = ctx.edit(|p, h: u8| p.config.active_hours_start = h);
    let end = ctx.edit(|p, h: u8| p.config.active_hours_end = h);
    div().flex().items_center().justify_between().gap(theme.spacing_md).child(
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .child(field_label(HOURS_LABEL, theme))
            .child(u8_stepper(START_HOUR_ID, hour_spec(cfg.active_hours_start), theme, start))
            .child(
                div()
                    .text_size(theme.font_size_xs)
                    .text_color(theme.text_muted)
                    .child(HOURS_SEPARATOR),
            )
            .child(u8_stepper(END_HOUR_ID, hour_spec(cfg.active_hours_end), theme, end)),
    )
}
