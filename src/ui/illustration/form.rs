//! Layered non-human bodies: a [`FormSpec`]'s part templates composed into one SVG in the
//! catalog's paint order. Clipped layers (finish, markings) are clipped to the body's
//! silhouette; anchored layers (eyes, mouth, accessory) move with the form's face anchor.

use super::active::with_active_cache;
use super::pack::{layer_plan, raster_avatar, LayerPlan};
use super::portrait::{compose_groups, layer_body, AVATAR_CANVAS};
use super::template::TemplateExtras;
use crate::domain::portrait::form::{FaceAnchor, FormLayer, FormSpec};
use crate::ui::assets::template_text;
use gpui_kit::gpui::Image;
use std::sync::Arc;

/// Cache source prefix of composed forms.
pub const FORM_SOURCE_PREFIX: &str = "form/";
/// Id of the silhouette clip path every body template defines (STYLE.md, "Forms").
pub const FORM_CLIP_ID: &str = "form-silhouette";
/// Separators of layer choices in a form cache source.
const PART_SEPARATOR: &str = ";";
const PART_ASSIGN: &str = "=";

/// Group attributes of `layer`: a silhouette clip, a face-anchor transform, or nothing.
fn layer_attributes(spec: &FormSpec, layer: FormLayer, anchor: FaceAnchor) -> String {
    let Some(entry) = spec.catalog().layers.get(&layer) else { return String::new() };
    if entry.clip {
        return format!(r#" clip-path="url(#{FORM_CLIP_ID})""#);
    }
    if entry.anchor && anchor != FaceAnchor::default() {
        let [a, b, c, d, e, f] = anchor.matrix();
        return format!(r#" transform="matrix({a} {b} {c} {d} {e:.2} {f:.2})""#);
    }
    String::new()
}

/// The composed (still placeholder-bearing) template of `spec`; missing parts are skipped.
pub fn compose_form_template(spec: &FormSpec) -> String {
    let anchor = spec.anchor();
    let texts: Vec<_> = spec
        .layer_paths()
        .into_iter()
        .filter_map(|(layer, path)| Some((layer_attributes(spec, layer, anchor), template_text(&path)?)))
        .collect();
    compose_groups(texts.iter().filter_map(|(attributes, text)| Some((attributes.clone(), layer_body(text)?))))
}

/// Cache source of a form's shape choices (colors travel in the extras).
pub fn form_source(spec: &FormSpec) -> String {
    let parts: Vec<String> =
        spec.catalog().stack.iter().map(|layer| format!("{}{PART_ASSIGN}{}", layer.key(), spec.part(*layer))).collect();
    format!("{FORM_SOURCE_PREFIX}{}/{}", spec.kind.key(), parts.join(PART_SEPARATOR))
}

/// The raster layers of `spec`: clipped layers keep their clip, anchored ones the face
/// anchor's matrix, as in [`compose_form_template`].
fn raster_layers(spec: &FormSpec, extras: &TemplateExtras) -> Vec<LayerPlan> {
    let anchor = spec.anchor();
    let catalog = spec.catalog();
    let plan = |(layer, path): (FormLayer, String)| {
        let entry = catalog.layers.get(&layer);
        let clip = entry.is_some_and(|entry| entry.clip);
        let matrix = entry.filter(|entry| entry.anchor && anchor != FaceAnchor::default()).map(|_| anchor.matrix());
        layer_plan(path, extras, matrix, clip)
    };
    spec.layer_paths().into_iter().map(plan).collect()
}

/// Themed form image rasterized `width` units wide (from the active pack's raster layers
/// when it covers this form).
pub fn form_image(spec: &FormSpec, width: Option<u16>) -> Arc<Image> {
    let extras = TemplateExtras::form(spec);
    if let Some(image) = raster_avatar(&form_source(spec), || raster_layers(spec, &extras), &extras, width.unwrap_or(AVATAR_CANVAS)) {
        return image;
    }
    with_active_cache(|cache, colors| cache.load_with(&form_source(spec), || Some(compose_form_template(spec)), colors, &extras, width))
}
