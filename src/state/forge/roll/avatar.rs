//! Keeping the picture in step with the body: a tin can gets a tin-can form avatar, a person
//! a generated portrait. Emblems and photos are the user's own choice and are never replaced.

use crate::domain::portrait::{AvatarMode, FormKind, FormSpec, PortraitSpec};
use crate::domain::traits::Embodiment;
use crate::domain::TraitOption;
use crate::state::forge::edit::DraftEdit;
use crate::state::forge::fields::AvatarField;

/// The avatar that suits `body`, or `None` to keep `current`: a generated portrait or form
/// body is swapped for a form body of the same kind and form (a person gets a portrait back).
pub fn avatar_for_body(body: Embodiment, current: Option<&AvatarMode>, seed: u64) -> Option<AvatarMode> {
    if matches!(current, Some(AvatarMode::Emblem(_) | AvatarMode::Photo { .. })) {
        return None;
    }
    let Some(kind) = FormKind::from_key(body.body_kind.key()) else {
        let shows_form = matches!(current, Some(AvatarMode::Form(_)));
        return shows_form.then(|| AvatarMode::Pcg(PortraitSpec::random(seed)));
    };
    let form = body.form.key();
    let matching = matches!(current, Some(AvatarMode::Form(spec)) if spec.kind == kind && spec.form() == form);
    (!matching).then(|| AvatarMode::Form(FormSpec::random(kind, form, seed)))
}

/// [`avatar_for_body`] as an edit to append to the body's own edits (empty when unchanged).
pub fn avatar_edits(body: Embodiment, current: Option<&AvatarMode>, seed: u64) -> Vec<DraftEdit> {
    avatar_for_body(body, current, seed).map(|mode| DraftEdit::Avatar(AvatarField::Avatar, Some(mode))).into_iter().collect()
}
