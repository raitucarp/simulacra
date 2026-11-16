//! Sub-tab "Idiolect & Quirks": capitalization, emoji, punctuation and signature phrasing.

use crate::domain::{CapitalizationStyle, EmojiUsage, PunctuationHabit};
use crate::ui::components::{
    card_stack, fill, labeled_group, labeled_input, section_card, split_row, EnumChips,
};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "TYPING MECHANICS, PUNCTUATION & IDIOLECT";
const SUBTITLE: &str = "Capitalization, punctuation habits, emoji usage, signature expressions, and avoided clichés.";
const CAPITALIZATION_LABEL: &str = "Capitalization Habit";
const CAPITALIZATION_CHIP_ID: &str = "cap-style";
const EMOJI_LABEL: &str = "Emoji & Emoticon Habit";
const EMOJI_CHIP_ID: &str = "emoji-usage";
const PUNCTUATION_LABEL: &str = "Punctuation Habit";
const PUNCTUATION_CHIP_ID: &str = "punc-habit";
const QUIRKS_LABEL: &str = "Specific Typing Quirks (e.g. em-dashes, numbered premises, parentheses)";
const SIGNATURE_LABEL: &str = "Signature Phrases & Idioms";
const AVOIDED_LABEL: &str = "Avoided Words & Taboo Clichés";
const SAVE_BUTTON_ID: &str = "save-voice-sub2-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(
                split_row(theme.spacing_md)
                    .w_full()
                    .child(capitalization_group(ctx).flex_1())
                    .child(emoji_group(ctx).flex_1()),
            )
            .child(punctuation_group(ctx))
            .child(labeled_input(QUIRKS_LABEL, inputs.typing_quirks_input, theme))
            .child(
                split_row(theme.spacing_md)
                    .child(fill(labeled_input(SIGNATURE_LABEL, inputs.signature_phrases_input, theme)))
                    .child(fill(labeled_input(AVOIDED_LABEL, inputs.avoided_words_input, theme))),
            )
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

fn capitalization_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.capitalization;
    let chips = EnumChips::new(
        &CapitalizationStyle::ALL,
        current,
        CAPITALIZATION_CHIP_ID,
        CapitalizationStyle::label,
    )
    .render(ctx.theme, ctx.edit(|p, v: CapitalizationStyle| p.config.capitalization = v));
    labeled_group(CAPITALIZATION_LABEL, chips, ctx.theme)
}

fn emoji_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.emoji_usage;
    let chips = EnumChips::new(&EmojiUsage::ALL, current, EMOJI_CHIP_ID, EmojiUsage::label)
        .render(ctx.theme, ctx.edit(|p, v: EmojiUsage| p.config.emoji_usage = v));
    labeled_group(EMOJI_LABEL, chips, ctx.theme)
}

fn punctuation_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.punctuation_habit;
    let chips = EnumChips::new(&PunctuationHabit::ALL, current, PUNCTUATION_CHIP_ID, PunctuationHabit::label)
        .render(ctx.theme, ctx.edit(|p, v: PunctuationHabit| p.config.punctuation_habit = v));
    labeled_group(PUNCTUATION_LABEL, chips, ctx.theme)
}
