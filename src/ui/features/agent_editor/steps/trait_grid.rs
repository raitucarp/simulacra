//! Declarative 0..=100 profile trait fields rendered as [`trait_meter`] cards.

use super::StepCtx;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::components::{fill, split_row, trait_meter, TraitMeterSpec};
use gpui_kit::*;

/// A numeric profile trait: how to read it, write it and tint it.
pub(super) struct TraitField {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub accent: fn(&ThemeTokens) -> Rgba,
    pub get: fn(&Profile) -> u8,
    pub set: fn(&mut Profile, u8),
}

/// One trait meter card bound to `field`.
pub(super) fn trait_card(field: &TraitField, ctx: &StepCtx<'_>) -> Div {
    let spec = TraitMeterSpec {
        id_prefix: field.id,
        title: field.title,
        subtitle: field.subtitle,
        value: (field.get)(ctx.profile),
        accent: (field.accent)(ctx.theme),
    };
    trait_meter(spec, ctx.theme, ctx.edit(field.set))
}

/// Full-width row of equally-sized trait cards.
pub(super) fn trait_row(fields: &[TraitField], ctx: &StepCtx<'_>) -> Div {
    split_row(ctx.theme.spacing_md)
        .w_full()
        .children(fields.iter().map(|field| fill(trait_card(field, ctx))))
}
