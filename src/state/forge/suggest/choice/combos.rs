//! Known combinations: choices that strengthen each other and choices that pull a voice apart.
//! Each rule names two options of two parameters and a `suggest.combo.<id>` reason.

use super::current::current_keys;
use super::super::engine_param::EngineParam;
use super::super::keys::combo_key;
use super::super::model::{Combo, OptionRef, Phrase, Verdict};
use super::super::param::Param;
use crate::domain::Profile;
use crate::state::forge::TraitField as F;

struct Rule {
    id: &'static str,
    verdict: Verdict,
    left: (Param, &'static str),
    right: (Param, &'static str),
}

const fn rule(id: &'static str, verdict: Verdict, left: (F, &'static str), right: (F, &'static str)) -> Rule {
    Rule { id, verdict, left: (Param::Trait(left.0), left.1), right: (Param::Trait(right.0), right.1) }
}

const OUTPUT: Param = Param::Engine(EngineParam::Output);
const TRIGGER: Param = Param::Engine(EngineParam::Trigger);
use Verdict::{Clashes, Pairs};

const RULES: [Rule; 21] = [
    rule("dialectic_dialogue", Pairs, (F::RhetoricalMode, "dialectic"), (F::PostFormat, "dialogue")),
    rule("narrative_field_note", Pairs, (F::RhetoricalMode, "narrative"), (F::PostFormat, "field_note")),
    rule("meditative_aphorism", Pairs, (F::RhetoricalMode, "meditative"), (F::PostFormat, "aphorism")),
    rule("logos_list", Pairs, (F::RhetoricalMode, "logos"), (F::PostFormat, "list")),
    rule("minimalist_simple", Pairs, (F::StylisticSchool, "minimalist"), (F::SentenceStructure, "simple")),
    rule("baroque_periodic", Pairs, (F::StylisticSchool, "baroque"), (F::SentenceStructure, "periodic")),
    rule("skeptic_socratic", Pairs, (F::EpistemicStance, "fallibilist_skeptic"), (F::MentionAttitude, "socratic_questioning")),
    rule("irony_register", Pairs, (F::HumorStyle, "socratic_irony"), (F::DictionRegister, "socratic_ironic")),
    rule("night_owl_cadence", Pairs, (F::Chronotype, "night_owl_insomniac"), (F::PostingCadence, "night_owl_hours")),
    Rule { id: "json_when_asks", verdict: Pairs, left: (OUTPUT, "json_schema"), right: (TRIGGER, "when_model_asks") },
    rule("narrative_aphorism", Clashes, (F::RhetoricalMode, "narrative"), (F::PostFormat, "aphorism")),
    rule("polemic_deescalate", Clashes, (F::RhetoricalMode, "polemic"), (F::ConflictHandling, "deescalate_harmonize")),
    rule("minimalist_stream", Clashes, (F::StylisticSchool, "minimalist"), (F::SentenceStructure, "stream_of_consciousness")),
    rule("concise_thread", Clashes, (F::SentenceLength, "ultra_concise_aphorism"), (F::PostFormat, "thread")),
    rule("concise_baroque", Clashes, (F::SentenceLength, "ultra_concise_aphorism"), (F::StylisticSchool, "baroque")),
    rule("lark_night_cadence", Clashes, (F::Chronotype, "early_bird_lark"), (F::PostingCadence, "night_owl_hours")),
    rule("emoji_academic", Clashes, (F::EmojiUsage, "moderate_expressive"), (F::DictionRegister, "academic_philosophical")),
    rule("lowercase_formal", Clashes, (F::Capitalization, "all_lowercase"), (F::GrammarStyle, "standard_formal")),
    rule("no_human_kin", Clashes, (F::Anthropomorphism, "none"), (F::RelationToHumans, "kin")),
    rule("cosmic_walking", Clashes, (F::Scale, "cosmic"), (F::Mobility, "walking")),
    Rule { id: "free_text_when_asks", verdict: Clashes, left: (OUTPUT, "free_text"), right: (TRIGGER, "when_model_asks") },
];

fn option((param, key): (Param, &'static str)) -> OptionRef {
    OptionRef { param, key }
}

/// The rule seen from `param`'s side, if it involves `param`.
fn seen_from(rule: &Rule, param: Param) -> Option<Combo> {
    let (mine, other) = if rule.left.0 == param {
        (rule.left, rule.right)
    } else if rule.right.0 == param {
        (rule.right, rule.left)
    } else {
        return None;
    };
    Some(Combo { verdict: rule.verdict, mine: option(mine), other: option(other), reason: Phrase::key(combo_key(rule.id)) })
}

/// Every known combination involving `param`.
pub fn combos_for(param: Param) -> Vec<Combo> {
    RULES.iter().filter_map(|rule| seen_from(rule, param)).collect()
}

/// The clashes `param` is in right now on `profile`.
pub fn active_clashes(param: Param, profile: &Profile) -> Vec<Combo> {
    let has = |option: OptionRef| current_keys(option.param, profile).contains(&option.key);
    combos_for(param).into_iter().filter(|combo| combo.verdict == Verdict::Clashes && has(combo.mine) && has(combo.other)).collect()
}

/// Ids of every rule (for the locale coverage test).
pub fn combo_ids() -> Vec<&'static str> {
    RULES.iter().map(|rule| rule.id).collect()
}
