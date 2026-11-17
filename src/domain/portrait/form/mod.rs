//! Non-human bodies ("forms") as layered portraits: a creature, plant, object, element,
//! machine, place or abstraction composed from per-kind part catalogs
//! (`assets/illustrations/forms/<kind>/catalog.json`) and painted with two swatch colors.
//!
//! Keys are plain snake_case strings (`object` / `tin_can`) so the agent's embodiment can be
//! bridged onto a [`FormSpec`] with a tiny mapping.

mod catalog;
mod emblem;
mod kind;
mod layer;
mod random;
mod spec;

pub use catalog::{
    FaceAnchor, FormCatalog, FACE_CENTER, FormCatalogFiles, FormHints, LayerCatalog, Swatch, CATALOG_FILE, FORM_ASSET_ROOT, SHARED_DIR,
};
pub use kind::{FormKind, FORM_I18N_ROOT};
pub use layer::{FormColor, FormLayer, FormSlot, NONE_VARIANT};
pub use spec::FormSpec;

#[cfg(test)]
mod tests;
