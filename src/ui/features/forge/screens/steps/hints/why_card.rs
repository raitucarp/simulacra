//! The "Why?" card of a parameter, translated from its catalog entry: the tip or reason, the
//! typical range, starting picks, examples, and what pairs with or clashes against it.

use crate::state::forge::suggest::choice::{active_clashes, option_label_key};
use crate::state::forge::suggest::{suggestions_for, Combo, FieldSuggestions, OptionRef, Param, Phrase, SuggestContext, Verdict};
use crate::state::forge::field_label_key;
use crate::ui::features::forge::text::{tr, tr_opt, tr_with};
use crate::ui::features::forge::view::ForgeView;
use crate::ui::features::forge::widgets::suggest::{band_caption, WhyCard, WhySection, WhyTone};

const TITLE_KEY: &str = "suggest.ui.why_title";
const TIP_KEY: &str = "suggest.ui.tip_heading";
const WHY_KEY: &str = "suggest.ui.why_heading";
const PICKS_WHY_KEY: &str = "suggest.ui.picks_why_heading";
const GUIDE_KEY: &str = "suggest.ui.guide_heading";
const EXAMPLES_KEY: &str = "suggest.ui.examples_heading";
const CHIPS_HINT_KEY: &str = "suggest.ui.chips_hint";
const PICKS_KEY: &str = "suggest.ui.picks_heading";
const PAIRS_KEY: &str = "suggest.ui.pairs_heading";
const CLASHES_KEY: &str = "suggest.ui.clashes_heading";
const CLASH_NOW_KEY: &str = "suggest.ui.clash_now";
const ARG_FIELD: &str = "field";
const ARG_OPTION: &str = "option";
const ARG_REASON: &str = "reason";
const HEADING_SEPARATOR: &str = " · ";
const REASON_SEPARATOR: &str = " — ";
/// Examples listed in the card (the chips show them all).
const CARD_EXAMPLES: usize = 3;

pub(super) fn text(phrase: &Phrase) -> String {
    phrase.resolve(tr)
}

/// The label of one option.
pub(super) fn option_text(option: OptionRef) -> String {
    option_label_key(option.param, option.key).map_or_else(|| option.key.to_string(), |key| tr(&key))
}

fn section(heading_key: &str, lines: Vec<String>, tone: WhyTone) -> WhySection {
    WhySection { heading: tr(heading_key), lines, tone }
}

fn combo_lines(combos: &[Combo], verdict: Verdict) -> Vec<String> {
    combos
        .iter()
        .filter(|combo| combo.verdict == verdict)
        .map(|combo| format!("{} + {}{REASON_SEPARATOR}{}", option_text(combo.mine), option_text(combo.other), text(&combo.reason)))
        .collect()
}

/// The sections of one catalog entry.
fn sections(param: Param, suggestions: FieldSuggestions) -> Vec<WhySection> {
    match suggestions {
        FieldSuggestions::Text(hints) => vec![
            section(TIP_KEY, vec![text(&hints.tip)], WhyTone::Plain),
            WhySection {
                heading: format!("{}{HEADING_SEPARATOR}{}", tr(EXAMPLES_KEY), tr(CHIPS_HINT_KEY)),
                lines: hints.examples.iter().take(CARD_EXAMPLES).map(|example| text(&example.label)).collect(),
                tone: WhyTone::Plain,
            },
        ],
        FieldSuggestions::Range(hint) => vec![section(WHY_KEY, vec![text(&hint.why), band_caption(&hint)], WhyTone::Plain)],
        FieldSuggestions::Choice(hints) => vec![
            section(PICKS_KEY, hints.picks.iter().map(|key| option_text(OptionRef { param, key })).collect(), WhyTone::Good),
            section(PICKS_WHY_KEY, vec![text(&hints.why)], WhyTone::Plain),
            section(PAIRS_KEY, combo_lines(&hints.combos, Verdict::Pairs), WhyTone::Good),
            section(CLASHES_KEY, combo_lines(&hints.combos, Verdict::Clashes), WhyTone::Warn),
        ],
        FieldSuggestions::Guide(advice) => vec![section(GUIDE_KEY, vec![text(&advice)], WhyTone::Plain)],
    }
}

impl ForgeView {
    /// The "Why?" card of `param`, titled with its field name (or `fallback` when the field
    /// has no caption of its own, like the sampling sliders).
    pub(in super::super) fn why_card(&self, param: Param, fallback: Option<String>) -> WhyCard {
        let name = tr_opt(&field_label_key(param.path())).or(fallback).unwrap_or_default();
        let suggestions = suggestions_for(param, &SuggestContext::of(&self.draft));
        WhyCard { title: tr_with(TITLE_KEY, &[(ARG_FIELD, name)]), sections: sections(param, suggestions) }
    }

    /// Warning lines for clashes `param` is in right now.
    pub(in super::super) fn clash_lines(&self, param: Param) -> Vec<String> {
        active_clashes(param, self.draft.profile())
            .iter()
            .map(|combo| tr_with(CLASH_NOW_KEY, &[(ARG_OPTION, option_text(combo.other)), (ARG_REASON, text(&combo.reason))]))
            .collect()
    }
}
