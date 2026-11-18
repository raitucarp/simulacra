//! Text examples computed from the rest of the draft.

use super::super::context::SuggestContext;
use super::super::keys::{example_key, keyed_examples, literal_examples};
use super::super::model::{Example, Phrase};
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::{BodyKind, EnneagramType, HumanGender};
use crate::domain::{Palette, TraitOption};
use crate::state::forge::{suggest_handle, TextField};

/// Examples offered when nothing better can be derived.
const MIN_EXAMPLES: usize = 4;
/// Most examples derived from a list (names, tritypes).
const MAX_EXAMPLES: usize = 6;
/// Handles offered before a name exists (handles are ASCII ids, not translated).
const FALLBACK_HANDLES: [&str; 4] = ["quiet_reader", "margin_notes", "slow_thinker", "night_essays"];
/// Initials offered before a name exists.
const FALLBACK_INITIALS: [&str; 4] = ["✦", "◆", "∴", "☉"];
const HANDLE_JOINER: &str = "_";
/// The three Enneagram centers (gut, heart, head).
const CENTERS: [[char; 3]; 3] = [['8', '9', '1'], ['2', '3', '4'], ['5', '6', '7']];
/// Where the core digit sits in a type key (`type5w4`).
const CORE_AT: usize = 4;
/// Where the wing digit sits in a type key.
const WING_AT: usize = 6;
/// Pronoun example numbers: he, she, they, it.
const PRONOUN_HE: usize = 1;
const PRONOUN_SHE: usize = 2;
const PRONOUN_THEY: usize = 3;
const PRONOUN_IT: usize = 4;

/// `values` deduplicated (blanks dropped), padded from `fallback` up to [`MIN_EXAMPLES`] and
/// cut at [`MAX_EXAMPLES`].
fn padded(values: Vec<String>, fallback: &[&str]) -> Vec<Example> {
    let unique = values.into_iter().filter(|value| !value.is_empty()).fold(Vec::new(), |mut seen: Vec<String>, value| {
        if !seen.contains(&value) {
            seen.push(value);
        }
        seen
    });
    let missing = MIN_EXAMPLES.saturating_sub(unique.len());
    let extra = fallback.iter().map(|text| text.to_string()).filter(|text| !unique.contains(text)).take(missing).collect::<Vec<_>>();
    literal_examples(&unique.into_iter().chain(extra).take(MAX_EXAMPLES).collect::<Vec<_>>())
}

/// Names of the draft's palette (or of every full palette) other than the current one.
pub(super) fn names(ctx: &SuggestContext) -> Vec<Example> {
    let own = ctx.palette.into_iter().flat_map(|palette| palette.names.iter());
    let all = Palette::full(None).flat_map(|palette| palette.names.iter());
    let names = own.chain(all).map(|idea| idea.name.clone()).filter(|name| *name != ctx.profile.name);
    padded(names.take(MAX_EXAMPLES * 2).collect(), &[])
}

/// Handles built from the name: the forge's own suggestion, joined, initial + surname, first name.
pub(super) fn handles(ctx: &SuggestContext) -> Vec<Example> {
    let name = ctx.profile.name.trim();
    if name.is_empty() {
        return literal_examples(&FALLBACK_HANDLES);
    }
    let first = suggest_handle(name, &ctx.profile.id, &[]);
    let words: Vec<String> = first.trim_start_matches('@').split(HANDLE_JOINER).map(str::to_string).collect();
    let initial = words.first().and_then(|word| word.chars().next()).map(String::from).unwrap_or_default();
    let tail = words.iter().skip(1).cloned().collect::<Vec<_>>().join(HANDLE_JOINER);
    let derived = vec![
        first.clone(),
        format!("@{}", words.concat()),
        format!("@{initial}{}", if tail.is_empty() { words.concat() } else { tail.clone() }),
        format!("@{}", words.first().cloned().unwrap_or_default()),
    ];
    padded(derived, &FALLBACK_HANDLES)
}

/// One- and two-letter initials of the name.
pub(super) fn initials(name: &str) -> Vec<Example> {
    let words: Vec<&str> = name.split_whitespace().collect();
    let letter = |word: &str| word.chars().next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default();
    let first = words.first().map(|word| letter(word)).unwrap_or_default();
    let pair: String = words.iter().take(2).map(|word| letter(word)).collect();
    let two: String = words.first().map(|word| word.chars().take(2).collect::<String>()).unwrap_or_default();
    let last = words.last().map(|word| letter(word)).unwrap_or_default();
    padded(vec![first, pair, two, last], &FALLBACK_INITIALS)
}

/// Tritypes led by the Enneagram core, one digit from each other center (wing first).
pub(super) fn tritypes(enneagram: EnneagramType) -> Vec<Example> {
    let key: Vec<char> = enneagram.key().chars().collect();
    let (core, wing) = (key.get(CORE_AT).copied().unwrap_or('5'), key.get(WING_AT).copied().unwrap_or('4'));
    let others: Vec<Vec<char>> = CENTERS
        .iter()
        .filter(|center| !center.contains(&core))
        .map(|center| {
            let mut digits = center.to_vec();
            digits.sort_by_key(|digit| *digit != wing);
            digits
        })
        .collect();
    let (left, right) = (others.first().cloned().unwrap_or_default(), others.get(1).cloned().unwrap_or_default());
    let combos = left.iter().flat_map(|a| right.iter().map(move |b| format!("{core}{a}{b}"))).take(MAX_EXAMPLES).collect();
    padded(combos, &[])
}

/// The translated pronoun examples, the ones matching the gender (or a non-human body) first.
pub(super) fn pronouns(cfg: &AgentConfig) -> Vec<Example> {
    let lead = match (cfg.embodiment.body_kind, cfg.gender) {
        (kind, _) if kind != BodyKind::Human => PRONOUN_IT,
        (_, HumanGender::Female) => PRONOUN_SHE,
        (_, HumanGender::NonBinary) => PRONOUN_THEY,
        (_, HumanGender::Male) => PRONOUN_HE,
        (_, HumanGender::Custom) => PRONOUN_THEY,
    };
    let path = TextField::Pronouns.path();
    let lead_example = Example::same(Phrase::key(example_key(path, lead)));
    let rest: Vec<Example> = keyed_examples(path).into_iter().filter(|example| *example != lead_example).collect();
    std::iter::once(lead_example).chain(rest).collect()
}
