//! Generated avatar templates: PCG portraits composed from their layer templates into one
//! SVG (one raster, exact alignment) and personality emblems, both cached per look.

use super::active::with_active_cache;
use super::template::TemplateExtras;
use crate::domain::portrait::{EmblemSpec, PortraitLayer, PortraitSpec};
use crate::ui::assets::template_text;
use gpui_kit::gpui::Image;
use std::sync::Arc;

/// Cache source prefixes of generated avatars.
pub const PORTRAIT_SOURCE_PREFIX: &str = "pcg/";
pub const EMBLEM_SOURCE_PREFIX: &str = "emblem/";
/// Root of a composed portrait (the shared 256-unit part canvas).
const COMPOSED_ROOT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256">"#;
/// Group re-applying the part templates' root presentation attributes to each layer (closed
/// after any per-layer attributes, e.g. a clip path or transform).
const LAYER_GROUP_OPEN: &str = r#"<g fill="none" stroke-linecap="round" stroke-linejoin="round""#;
const LAYER_GROUP_CLOSE: &str = "</g>";
const SVG_OPEN: &str = "<svg";
const SVG_CLOSE: &str = "</svg>";
const TAG_END: char = '>';
/// Separator of variant keys in a portrait cache source.
const KEY_SEPARATOR: &str = "/";

/// The markup between a template's root `<svg …>` and `</svg>` tags.
pub fn layer_body(svg: &str) -> Option<&str> {
    let start = svg.find(SVG_OPEN)?;
    let body_start = start + svg[start..].find(TAG_END)? + TAG_END.len_utf8();
    let body_end = svg.rfind(SVG_CLOSE)?;
    (body_end >= body_start).then(|| &svg[body_start..body_end])
}

/// One SVG stacking `bodies` bottom to top, each in its own presentation group.
pub fn compose_layers<'a>(bodies: impl IntoIterator<Item = &'a str>) -> String {
    compose_groups(bodies.into_iter().map(|body| (String::new(), body)))
}

/// [`compose_layers`] with extra attributes on each layer's group (`(attributes, body)`).
pub fn compose_groups<'a>(layers: impl IntoIterator<Item = (String, &'a str)>) -> String {
    let mut out = String::from(COMPOSED_ROOT);
    for (attributes, body) in layers {
        out.push_str(LAYER_GROUP_OPEN);
        out.push_str(&attributes);
        out.push(TAG_END);
        out.push_str(body);
        out.push_str(LAYER_GROUP_CLOSE);
    }
    out.push_str(SVG_CLOSE);
    out
}

/// The composed (still placeholder-bearing) template of `spec`; missing parts are skipped.
pub fn compose_portrait_template(spec: &PortraitSpec) -> String {
    let texts: Vec<_> = spec.layer_paths().iter().filter_map(|path| template_text(path)).collect();
    compose_layers(texts.iter().filter_map(|text| layer_body(text)))
}

/// Cache source of a portrait's shape choices (colors travel in the extras).
pub fn portrait_source(spec: &PortraitSpec) -> String {
    let keys: Vec<&str> = PortraitLayer::STACK.iter().map(|layer| spec.variant_key(*layer)).collect();
    format!("{PORTRAIT_SOURCE_PREFIX}{}", keys.join(KEY_SEPARATOR))
}

/// Themed portrait image rasterized `width` units wide.
pub fn portrait_image(spec: &PortraitSpec, width: Option<u16>) -> Arc<Image> {
    let extras = TemplateExtras::portrait(spec.skin, spec.hair_color);
    let source = portrait_source(spec);
    with_active_cache(|cache, colors| {
        cache.load_with(&source, || Some(compose_portrait_template(spec)), colors, &extras, width)
    })
}

/// Themed emblem image rasterized `width` units wide.
pub fn emblem_image(spec: &EmblemSpec, width: Option<u16>) -> Arc<Image> {
    let source = format!("{EMBLEM_SOURCE_PREFIX}{spec:?}");
    with_active_cache(|cache, colors| {
        cache.load_with(&source, || Some(spec.to_svg_template()), colors, &TemplateExtras::none(), width)
    })
}
