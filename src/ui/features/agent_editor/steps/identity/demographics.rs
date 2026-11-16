//! Sub-tab "Demographics & Mood": roots, pronouns, gender, age and current mood.

use crate::domain::HumanGender;
use crate::ui::components::{
    card_stack, fill, labeled_group, labeled_input, section_card, split_row, u8_stepper,
    EnumChips, StepperSpec,
};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const DEMOGRAPHICS_TITLE: &str = "DEMOGRAPHICS & CULTURAL ROOTS";
const DEMOGRAPHICS_SUBTITLE: &str =
    "Geographic background, personal pronouns, gender identity, and chronological age.";
const LOCATION_LABEL: &str = "Cultural & Geographic Roots";
const PRONOUNS_LABEL: &str = "Pronouns";
const GENDER_LABEL: &str = "Gender Identity";
const GENDER_CHIP_ID: &str = "gender-chip";
const AGE_LABEL: &str = "Chronological Age";
const AGE_STEPPER_ID: &str = "age-stepper";
const AGE_UNIT: &str = " yrs";
const AGE_MIN: u8 = 16;
const AGE_MAX: u8 = 110;

const MOOD_TITLE: &str = "CURRENT AFFECTIVE MOOD";
const MOOD_SUBTITLE: &str = "Immediate emotional tone coloring this person's thoughts today.";
const MOOD_CHIP_ID: &str = "mood-preset";
const MOOD_PRESETS: [&str; 7] = [
    "Contemplative & Lucid",
    "Playfully Ironic & Inquisitive",
    "Focused & Pragmatic",
    "Serene & Sub Specie Aeternitatis",
    "Luminous Attention",
    "Electric & Associative",
    "Lyrical & Earthy",
];
const SAVE_BUTTON_ID: &str = "save-identity-sub1-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    card_stack(ctx.theme).child(demographics_card(ctx)).child(mood_card(ctx))
}

fn demographics_card(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    section_card(DEMOGRAPHICS_TITLE, DEMOGRAPHICS_SUBTITLE, theme)
        .child(
            split_row(theme.spacing_md)
                .child(fill(labeled_input(LOCATION_LABEL, inputs.location_input, theme)))
                .child(fill(labeled_input(PRONOUNS_LABEL, inputs.pronouns_input, theme))),
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(theme.spacing_md)
                .child(gender_group(ctx))
                .child(age_group(ctx)),
        )
}

fn gender_group(ctx: &StepCtx<'_>) -> Div {
    let chips = EnumChips::new(
        &HumanGender::ALL,
        ctx.profile.config.gender,
        GENDER_CHIP_ID,
        HumanGender::label,
    )
    .render(
        ctx.theme,
        ctx.edit(|p, gender: HumanGender| {
            p.config.gender = gender;
            p.config.pronouns = gender.default_pronouns().to_string();
        }),
    );
    labeled_group(GENDER_LABEL, chips, ctx.theme).flex_1()
}

fn age_group(ctx: &StepCtx<'_>) -> Div {
    let spec = StepperSpec {
        value: ctx.profile.config.age.clamp(AGE_MIN.into(), AGE_MAX.into()) as u8,
        min: AGE_MIN,
        max: AGE_MAX,
        unit: AGE_UNIT,
    };
    let stepper = u8_stepper(
        AGE_STEPPER_ID,
        spec,
        ctx.theme,
        ctx.edit(|p, age: u8| p.config.age = u16::from(age)),
    );
    labeled_group(AGE_LABEL, stepper, ctx.theme).flex_shrink_0().items_end()
}

fn mood_card(ctx: &StepCtx<'_>) -> Div {
    let chips = EnumChips::strings(&MOOD_PRESETS, &ctx.profile.config.current_mood, MOOD_CHIP_ID)
        .render(ctx.theme, ctx.edit(|p, mood: &'static str| p.config.current_mood = mood.to_string()));
    section_card(MOOD_TITLE, MOOD_SUBTITLE, ctx.theme)
        .child(chips)
        .child(ctx.save_footer(SAVE_BUTTON_ID))
}
