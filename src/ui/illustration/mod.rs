//! Themed illustration and avatar rendering.
//!
//! Illustration assets are SVG *templates* (`assets/illustrations/STYLE.md`): `{{token}}`
//! placeholders are filled from the active theme palette (plus portrait skin/hair or form
//! primary/secondary), and the result is handed to gpui as an in-memory SVG
//! [`Image`](gpui_kit::gpui::Image), cached per (source, palette, extras, raster width) in a
//! UI-thread [`IllustrationCache`].
//!
//! **Raster size.** gpui rasterizes an SVG `img` at 2x its intrinsic size and ignores the
//! window's DPI. Templates carry only a viewBox, so every load sets an explicit intrinsic
//! width snapped to [`RASTER_WIDTHS`] at or above the displayed width: crisp at up to 200 %
//! scaling, small for tiny avatars, and a bounded number of cache variants per template.
//!
//! **Packs** (WS5, [`pack`]): every image resolves through the active illustration pack
//! first; a raster pack's WebP file draws a key it covers and Classic (these templates)
//! draws everything else. Raster images are cached apart from the themed ones, so a theme
//! change keeps them and a pack switch clears them.
//!
//! **Portraits** are composed into one SVG (one raster, exact layer alignment) instead of
//! eight stacked images, and **forms** likewise from their kind's catalog (see [`form`]);
//! **photos** are clipped to a circle inside a tiny SVG because gpui content masks are
//! rectangular (see [`photo`]).

mod active;
mod cache;
mod element;
pub mod form;
mod keys;
pub mod pack; // WS5
pub mod photo;
pub mod portrait;
mod portrait_view;
pub mod template;

pub use active::{active_palette, forget_illustrations, set_active_palette, with_active_cache};
pub use cache::{palette_hash, IllustrationCache, FALLBACK_TEMPLATE};
pub use element::Illustration;
pub use keys::{ClassKey, EmptyKey, GuidePose, IllustrationKind, NoticeKey, SpotKey, StepKey, ILLUSTRATION_EXT, ILLUSTRATION_ROOT};
pub use pack::{active_pack_id, configure_packs, set_active_pack, RasterPack};
pub use form::{compose_form_template, form_image, form_source};
pub use photo::{forget_photo, forget_stored_photo};
pub use portrait_view::{avatar_source, render_avatar_mode, PortraitView};
pub use template::{fill_template, raster_width, TemplateExtras, RASTER_WIDTHS};

#[cfg(test)]
mod tests;
