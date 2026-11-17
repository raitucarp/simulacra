//! "Roll a being": a coherent random body. A normal roll keeps whatever the draft is (people
//! stay people); rolling a being picks a form within the current kind, or any form at all.

use super::avatar::avatar_for_body;
use super::rng::SplitMix64;
use crate::domain::traits::{BodyForm, BodyKind, Embodiment, RelationToHumans, Withdrawal};
use crate::domain::TraitOption;
use crate::state::forge::draft::AgentDraft;
use crate::state::forge::edit::DraftEdit;
use crate::state::forge::fields::TraitValue;

/// Where a rolled form may come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeingScope {
    /// Another form of the draft's current kind (a human stays a person).
    SameKind,
    /// Any form of any kind, human included.
    AnyBeing,
}

/// A uniformly chosen option of `T`.
fn pick<T: TraitOption>(rng: &mut SplitMix64) -> T {
    T::ALL[rng.below(T::ALL.len())]
}

/// A random coherent body: the form's own nature, with a random stance towards people and a
/// random depth of withdrawal (a person keeps the human defaults).
pub fn rolled_embodiment(seed: u64, current: BodyKind, scope: BeingScope) -> Embodiment {
    let mut rng = SplitMix64::new(seed);
    let forms = match scope {
        BeingScope::SameKind => current.forms(),
        BeingScope::AnyBeing => BodyForm::ALL.to_vec(),
    };
    let form = forms.get(rng.below(forms.len())).copied().unwrap_or_default();
    let mut body = Embodiment::natural(form);
    if !body.is_human() {
        body.relation_to_humans = pick::<RelationToHumans>(&mut rng);
        body.withdrawal = pick::<Withdrawal>(&mut rng);
    }
    body
}

/// Re-rolls the draft's body (and a generated avatar to match it) as one undo step and
/// returns it.
pub fn roll_being(draft: &mut AgentDraft, seed: u64, scope: BeingScope) -> Embodiment {
    let body = rolled_embodiment(seed, draft.profile().config.embodiment.body_kind, scope);
    draft.transform(|profile| {
        profile.config.embodiment = body;
        if let Some(avatar) = avatar_for_body(body, profile.config.avatar.as_ref(), seed) {
            profile.config.avatar = Some(avatar);
        }
    });
    body
}

/// The body after picking `form`: the form's own physical nature. Its stance towards people
/// (human-likeness, relation, withdrawal) is kept while the kind stays the same.
pub fn body_for_form(current: Embodiment, form: BodyForm) -> Embodiment {
    let mut body = Embodiment::natural(form);
    if current.body_kind == form.kind() {
        body.anthropomorphism = current.anthropomorphism;
        body.relation_to_humans = current.relation_to_humans;
        body.withdrawal = current.withdrawal;
    }
    body
}

/// The body after picking `kind`: unchanged when it already is one, else the kind's first form
/// left to its nature.
pub fn body_for_kind(current: Embodiment, kind: BodyKind) -> Embodiment {
    if current.body_kind == kind {
        current
    } else {
        Embodiment::natural(kind.default_form())
    }
}

/// The edits that turn any body into `body` (applied together as one undo step).
pub fn body_edits(body: Embodiment) -> Vec<DraftEdit> {
    [
        TraitValue::Form(body.form),
        TraitValue::Material(body.material),
        TraitValue::Scale(body.scale),
        TraitValue::Temporality(body.temporality),
        TraitValue::Perception(body.perception),
        TraitValue::Mobility(body.mobility),
        TraitValue::Anthropomorphism(body.anthropomorphism),
        TraitValue::RelationToHumans(body.relation_to_humans),
        TraitValue::Withdrawal(body.withdrawal),
    ]
    .into_iter()
    .map(DraftEdit::Trait)
    .collect()
}
