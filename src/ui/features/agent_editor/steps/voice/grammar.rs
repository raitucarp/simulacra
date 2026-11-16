//! Sub-tab "Grammar & Diction": syntax style, register, vocabulary and sentence rhythm.

use crate::domain::{DictionRegister, GrammarStyle, SentenceLengthPreference};
use crate::ui::components::{card_stack, labeled_group, section_card, split_row, EnumChips};
use crate::ui::features::agent_editor::steps::trait_grid::{trait_card, TraitField};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "GRAMMAR, DICTION REGISTER & LEXICAL COMPLEXITY";
const SUBTITLE: &str = "Syntactic structure, vocabulary register, and sentence pacing.";
const GRAMMAR_LABEL: &str = "Grammar & Syntax Style";
const GRAMMAR_CHIP_ID: &str = "grammar-style";
const DICTION_LABEL: &str = "Diction & Lexical Register";
const DICTION_CHIP_ID: &str = "diction-reg";
const SENTENCE_LABEL: &str = "Sentence Length & Rhythm";
const SENTENCE_CHIP_ID: &str = "sentence-len";

const VOCABULARY: TraitField = TraitField {
    id: "vocab-complexity-gauge",
    title: "Vocabulary Complexity",
    subtitle: "0% = Everyday Plain Speech · 100% = Erudite Polymathic Lexicon",
    accent: |t| t.accent_brand,
    get: |p| p.config.vocabulary_complexity,
    set: |p, v| p.config.vocabulary_complexity = v,
};

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(
                split_row(theme.spacing_md)
                    .w_full()
                    .child(grammar_group(ctx).flex_1())
                    .child(diction_group(ctx).flex_1()),
            )
            .child(trait_card(&VOCABULARY, ctx))
            .child(sentence_group(ctx)),
    )
}

fn grammar_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.grammar_style;
    let chips = EnumChips::new(&GrammarStyle::ALL, current, GRAMMAR_CHIP_ID, GrammarStyle::label)
        .render(ctx.theme, ctx.edit(|p, v: GrammarStyle| p.config.grammar_style = v));
    labeled_group(GRAMMAR_LABEL, chips, ctx.theme)
}

fn diction_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.diction_register;
    let chips = EnumChips::new(&DictionRegister::ALL, current, DICTION_CHIP_ID, DictionRegister::label)
        .render(ctx.theme, ctx.edit(|p, v: DictionRegister| p.config.diction_register = v));
    labeled_group(DICTION_LABEL, chips, ctx.theme)
}

fn sentence_group(ctx: &StepCtx<'_>) -> Div {
    let current = ctx.profile.config.sentence_length;
    let chips = EnumChips::new(
        &SentenceLengthPreference::ALL,
        current,
        SENTENCE_CHIP_ID,
        SentenceLengthPreference::label,
    )
    .render(ctx.theme, ctx.edit(|p, v: SentenceLengthPreference| p.config.sentence_length = v));
    labeled_group(SENTENCE_LABEL, chips, ctx.theme)
}
