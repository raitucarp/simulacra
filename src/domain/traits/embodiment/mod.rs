//! Flat ontology: an agent's body may be a person, a crow, a tin can, a river or justice
//! itself. Its psychology, intelligence and voice stay parameters of whatever it is.

mod dims;
mod kinds;
mod nature;
mod nouns;

pub use dims::{Anthropomorphism, Material, Mobility, Perception, RelationToHumans, Scale, Temporality, Withdrawal};
pub use kinds::{BodyForm, BodyKind};
pub use nature::Nature;

use serde::{Deserialize, Serialize};

/// The body of an agent. Flattened into `AgentConfig`'s JSON (each field is a top-level key),
/// so configs saved before it existed load as a human (see `StoredEmbodiment`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(from = "StoredEmbodiment")]
pub struct Embodiment {
    pub body_kind: BodyKind,
    pub form: BodyForm,
    pub material: Material,
    pub scale: Scale,
    pub temporality: Temporality,
    pub perception: Perception,
    pub mobility: Mobility,
    pub anthropomorphism: Anthropomorphism,
    pub relation_to_humans: RelationToHumans,
    pub withdrawal: Withdrawal,
}

/// [`Embodiment`] as stored: any key may be missing, and a kind and form that disagree
/// (hand-edited or imported JSON) are reconciled on load.
#[derive(Deserialize)]
#[serde(default)]
struct StoredEmbodiment {
    body_kind: BodyKind,
    form: BodyForm,
    material: Material,
    scale: Scale,
    temporality: Temporality,
    perception: Perception,
    mobility: Mobility,
    anthropomorphism: Anthropomorphism,
    relation_to_humans: RelationToHumans,
    withdrawal: Withdrawal,
}

impl Default for StoredEmbodiment {
    fn default() -> Self {
        let Embodiment { body_kind, form, material, scale, temporality, perception, mobility, anthropomorphism, relation_to_humans, withdrawal } =
            Embodiment::default();
        Self { body_kind, form, material, scale, temporality, perception, mobility, anthropomorphism, relation_to_humans, withdrawal }
    }
}

impl From<StoredEmbodiment> for Embodiment {
    /// A stored non-person form decides the kind; otherwise the kind picks its first form.
    fn from(s: StoredEmbodiment) -> Self {
        let mut body = Self {
            body_kind: s.body_kind, form: s.form, material: s.material, scale: s.scale, temporality: s.temporality,
            perception: s.perception, mobility: s.mobility, anthropomorphism: s.anthropomorphism,
            relation_to_humans: s.relation_to_humans, withdrawal: s.withdrawal,
        };
        if body.form != BodyForm::Person {
            body.set_form(body.form);
        } else {
            body.set_kind(body.body_kind);
        }
        body
    }
}

impl BodyKind {
    /// `(anthropomorphism, relation to humans, withdrawal)` a being of this kind starts with.
    pub const fn temperament(self) -> (Anthropomorphism, RelationToHumans, Withdrawal) {
        use Anthropomorphism as A;
        use RelationToHumans as R;
        use Withdrawal as W;
        match self {
            Self::Human => (A::Full, R::Kin, W::Open),
            Self::Creature => (A::Partial, R::Companion, W::Guarded),
            Self::Plant => (A::Subtle, R::Witness, W::Guarded),
            Self::Object => (A::Subtle, R::Servant, W::Elusive),
            Self::Element => (A::Subtle, R::Indifferent, W::Elusive),
            Self::Machine => (A::Partial, R::Servant, W::Guarded),
            Self::Place => (A::Subtle, R::Witness, W::Guarded),
            Self::Abstraction => (A::Partial, R::Deity, W::DeeplyWithdrawn),
        }
    }
}

impl Embodiment {
    /// `form` left entirely to its nature (and to its kind's temperament).
    pub fn natural(form: BodyForm) -> Self {
        let (material, scale, temporality, perception, mobility) = form.nature();
        let (anthropomorphism, relation_to_humans, withdrawal) = form.kind().temperament();
        Self {
            body_kind: form.kind(),
            form,
            material,
            scale,
            temporality,
            perception,
            mobility,
            anthropomorphism,
            relation_to_humans,
            withdrawal,
        }
    }

    /// Whether this is an ordinary human body (the prompt then reads exactly as for people).
    pub const fn is_human(&self) -> bool {
        matches!(self.body_kind, BodyKind::Human)
    }

    /// Whether the form belongs to the kind.
    pub fn is_coherent(&self) -> bool {
        self.form.kind() == self.body_kind
    }

    /// Sets the kind; a form of another kind is replaced by the kind's first form.
    pub fn set_kind(&mut self, kind: BodyKind) {
        self.body_kind = kind;
        if self.form.kind() != kind {
            self.form = kind.default_form();
        }
    }

    /// Sets the form; the kind follows it.
    pub fn set_form(&mut self, form: BodyForm) {
        self.form = form;
        self.body_kind = form.kind();
    }
}

#[cfg(test)]
mod tests;
