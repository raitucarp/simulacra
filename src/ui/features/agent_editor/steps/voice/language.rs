//! Sub-tab "Language": primary / secondary languages, presets and code-switching.

use crate::domain::CodeSwitchingStyle;
use crate::ui::components::{
    card_stack, fill, labeled_group, labeled_input, section_card, split_row, EnumChips,
};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "LANGUAGES, FLUENCY & CODE-SWITCHING";
const SUBTITLE: &str =
    "Primary writing language, secondary languages, and bilingual code-switching behavior.";
const PRIMARY_LABEL: &str = "Primary Writing Language";
const SECONDARY_LABEL: &str = "Secondary Languages & Classical Roots";
const PRESETS_LABEL: &str = "Quick Primary Language Presets";
const PRESET_CHIP_ID: &str = "lang-preset";
const LANGUAGE_PRESETS: [&str; 6] = [
    "Bahasa Indonesia",
    "English",
    "Bilingual (Indonesian & English)",
    "Francais & English",
    "Deutsch & English",
    "Japanese & English",
];
const CODE_SWITCH_LABEL: &str = "Code-Switching & Bilingualism Style";
const CODE_SWITCH_CHIP_ID: &str = "code-switch";
const SAVE_BUTTON_ID: &str = "save-voice-sub0-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(
                split_row(theme.spacing_md)
                    .child(fill(labeled_input(PRIMARY_LABEL, inputs.primary_lang_input, theme)))
                    .child(fill(labeled_input(SECONDARY_LABEL, inputs.secondary_lang_input, theme))),
            )
            .child(language_presets(ctx))
            .child(code_switching(ctx))
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

fn language_presets(ctx: &StepCtx<'_>) -> Div {
    let current = &ctx.profile.config.primary_language;
    let chips = EnumChips::strings(&LANGUAGE_PRESETS, current, PRESET_CHIP_ID).render(
        ctx.theme,
        ctx.edit(|p, lang: &'static str| p.config.primary_language = lang.to_string()),
    );
    labeled_group(PRESETS_LABEL, chips, ctx.theme)
}

fn code_switching(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.code_switching;
    let chips = EnumChips::new(&CodeSwitchingStyle::ALL, current, CODE_SWITCH_CHIP_ID, CodeSwitchingStyle::label)
        .render(ctx.theme, ctx.edit(|p, v: CodeSwitchingStyle| p.config.code_switching = v));
    labeled_group(CODE_SWITCH_LABEL, chips, ctx.theme)
}
