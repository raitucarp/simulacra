//! Illustration packs on screen (WS5, see [`crate::domain::packs`]): the registry of
//! available packs and the active one, the loader every [`Illustration`](super::Illustration)
//! goes through (active pack → key → raster file, else the Classic template), raster
//! preparation per size bucket, and composed avatars from raster layers.

mod layered;
mod loader;
mod raster;
mod refs;
mod registry;
mod source;

pub use refs::{key_colour_extras, reference_palette, reference_svg};

pub use layered::{compose, covers_all, is_blank_layer, LayerPlan};
pub use loader::{classic_keys, coverage, layer_plan, layer_tint, load_illustration, raster_avatar};
pub use raster::{decode, encode_png, file_format, prepare, resized, target_width};
pub use registry::{active_pack, active_pack_id, available_packs, configure_packs, find_pack, load_cached, raster_cache_len, set_active_pack, set_available_packs};
pub use source::RasterPack;
