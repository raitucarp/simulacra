//! Good starting picks of the choice parameters, why, and the combinations they are part of.

mod combos;
mod context_picks;
mod current;
mod engine;
mod options;
mod picks;

pub use combos::{active_clashes, combo_ids, combos_for};
pub use current::current_keys;
pub use options::{is_choice, option_keys, option_label_key, AVATAR_FORM, AVATAR_PORTRAIT, OFF, ON};

use super::context::SuggestContext;
use super::keys::why_key;
use super::model::{ChoiceHints, Phrase};
use super::param::Param;
use crate::domain::traits::BodyKind;

/// The starting picks of `param`, if it is a choice.
fn picks(param: Param, ctx: &SuggestContext) -> Option<Vec<&'static str>> {
    match param {
        Param::Trait(field) => {
            context_picks::context_picks(field, ctx).or_else(|| picks::static_picks(field).map(<[_]>::to_vec))
        }
        Param::Set(field) => Some(picks::static_set_picks(field).map_or_else(|| context_picks::lexical_picks(ctx.config()), <[_]>::to_vec)),
        Param::Toggle(_) => Some(vec![ON]),
        Param::Avatar(_) => Some(vec![if ctx.config().embodiment.body_kind == BodyKind::Human { AVATAR_PORTRAIT } else { AVATAR_FORM }]),
        Param::Engine(engine) => engine::engine_picks(engine, &ctx.config().engine),
        _ => None,
    }
}

/// The choice suggestions of `param`, when it is a choice.
pub(super) fn choice_hints(param: Param, ctx: &SuggestContext) -> Option<ChoiceHints> {
    let picks = picks(param, ctx)?;
    Some(ChoiceHints { picks, why: Phrase::key(why_key(param.path())), combos: combos_for(param) })
}
