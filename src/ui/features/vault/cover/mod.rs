//! Generated covers: an abstract SVG template per document, seeded by its title and kind,
//! so every document has a recognisable face without imitating any real edition.
//!
//! The template uses palette placeholders only (`{{accent}}`, `{{fg}}`, ...) and `#000` /
//! `#fff` shading with an opacity, like the illustration assets, so covers follow the theme.

mod motifs;
mod seed;
#[cfg(test)]
mod tests;

pub use seed::CoverSeed;

use crate::domain::document::DocumentKind;
use crate::ui::illustration::{raster_width, with_active_cache, TemplateExtras};
use gpui_kit::gpui::Image;
use gpui_kit::Pixels;
use std::sync::Arc;

/// Cover canvas (2:3, like a book).
pub const COVER_WIDTH: f32 = 120.0;
pub const COVER_HEIGHT: f32 = 180.0;
/// Cache source prefix of generated covers.
const COVER_SOURCE_PREFIX: &str = "vault-cover/";
/// Palette hues a cover may be printed in (pairs are picked from these).
pub(super) const COVER_HUES: [&str; 7] = ["accent", "accent_2", "agent", "success", "warning", "info", "danger"];

/// The SVG template of the cover of `title` as a `kind`: deterministic for equal inputs.
pub fn cover_template(title: &str, kind: DocumentKind) -> String {
    let seed = CoverSeed::of(title, kind);
    let ground = seed.hue(0);
    let ink = seed.second_hue(ground);
    let body = [
        format!(r#"<rect width="{COVER_WIDTH}" height="{COVER_HEIGHT}" fill="{{{{{ground}}}}}"/>"#),
        format!(r##"<rect width="{COVER_WIDTH}" height="{COVER_HEIGHT}" fill="#000" fill-opacity=".18"/>"##),
        motifs::motif(kind, &seed, ink),
        motifs::finish(kind),
    ]
    .concat();
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {COVER_WIDTH} {COVER_HEIGHT}" fill="none" stroke-linecap="round" stroke-linejoin="round">{body}</svg>"#
    )
}

/// Cache key of a cover (the template is rebuilt only on a miss).
fn cover_source(title: &str, kind: DocumentKind) -> String {
    format!("{COVER_SOURCE_PREFIX}{}/{:016x}", kind.key(), CoverSeed::of(title, kind).value())
}

/// The themed cover image, rasterized for `width`.
pub fn cover_image(title: &str, kind: DocumentKind, width: Pixels) -> Arc<Image> {
    let source = cover_source(title, kind);
    let width = Some(raster_width(width.as_f32()));
    with_active_cache(|cache, colors| {
        cache.load_with(&source, || Some(cover_template(title, kind)), colors, &TemplateExtras::none(), width)
    })
}
