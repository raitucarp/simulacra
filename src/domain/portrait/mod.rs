//! Agent avatars as pure data: procedurally generated layered portraits ([`PortraitSpec`]),
//! layered non-human bodies ([`FormSpec`]), personality emblems ([`EmblemSpec`]) and imported
//! photos ([`PhotoCrop`]), unified by [`AvatarMode`]. Rendering lives in `crate::ui::illustration`.

mod colors;
pub mod emblem;
pub mod form;
mod layer;
mod mode;
mod outfit;
mod parts;
mod photo;
mod rng;
mod spec;

pub use colors::{HairColor, SkinTone};
pub use emblem::{EmblemGlyph, EmblemHue, EmblemShape, EmblemSpec};
pub use form::{FormColor, FormKind, FormLayer, FormSlot, FormSpec};
pub use layer::{PortraitLayer, PortraitSlot, PORTRAIT_ASSET_EXT, PORTRAIT_ASSET_ROOT, PORTRAIT_I18N_ROOT};
pub use mode::AvatarMode;
pub use outfit::suggested_outfit;
pub use parts::{
    layer_variant_keys, slot_option_keys, Accessory, Backdrop, BrowStyle, EyeStyle, FaceShape, HairStyle, MouthStyle,
    OutfitStyle,
};
pub use photo::{CropFrame, PhotoCrop, PER_MILLE, ZOOM_MAX, ZOOM_MIN, ZOOM_STEP};
pub use rng::PortraitRng;
pub use spec::PortraitSpec;

#[cfg(test)]
mod tests;
