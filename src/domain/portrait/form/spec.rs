//! [`FormSpec`]: the full, serializable description of a layered non-human body.

use super::catalog::{FaceAnchor, FormCatalog, Swatch};
use super::kind::FormKind;
use super::layer::{FormColor, FormLayer, FormSlot, NONE_VARIANT};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A form body: its kind, a variant per layer (`parts`: layer key → variant key) and two
/// swatch keys. Missing or unknown entries fall back to the catalog defaults, so older or
/// partial specs always load and render.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct FormSpec {
    pub kind: FormKind,
    pub parts: BTreeMap<String, String>,
    pub primary: String,
    pub secondary: String,
    /// Seed the spec was rolled from.
    pub seed: u64,
}

impl FormSpec {
    /// A spec of `kind` showing `form`, every other layer at its default.
    pub fn new(kind: FormKind, form: &str) -> Self {
        Self { kind, ..Self::default() }.with_part(FormLayer::Body, form)
    }

    /// The catalog of this spec's kind.
    pub fn catalog(&self) -> &'static FormCatalog {
        FormCatalog::of(self.kind)
    }

    /// The variant shown on `layer`: the chosen one when the catalog knows it, else the default.
    pub fn part(&self, layer: FormLayer) -> &str {
        let catalog = self.catalog();
        match self.parts.get(layer.key()) {
            Some(chosen) if catalog.variants(layer).contains(chosen) => chosen,
            _ => catalog.default_variant(layer),
        }
    }

    /// The form itself (the body variant, e.g. `tin_can`).
    pub fn form(&self) -> &str {
        self.part(FormLayer::Body)
    }

    /// This spec with `variant` on `layer`.
    pub fn with_part(mut self, layer: FormLayer, variant: &str) -> Self {
        self.parts.insert(layer.key().to_string(), variant.to_string());
        self
    }

    /// The swatches `color` offers.
    pub fn swatches(&self, color: FormColor) -> &'static [Swatch] {
        let catalog = self.catalog();
        match color {
            FormColor::Primary => &catalog.primary,
            FormColor::Secondary => &catalog.secondary,
        }
    }

    /// Index of the chosen swatch of `color` (0, the default, when unknown).
    pub fn swatch_index(&self, color: FormColor) -> usize {
        let key = match color {
            FormColor::Primary => &self.primary,
            FormColor::Secondary => &self.secondary,
        };
        self.swatches(color).iter().position(|swatch| &swatch.key == key).unwrap_or_default()
    }

    /// `#rrggbb` of `color` (empty when the kind has no swatches; the template falls back).
    pub fn hex(&self, color: FormColor) -> &'static str {
        self.swatches(color).get(self.swatch_index(color)).map_or("", |swatch| swatch.hex.as_str())
    }

    /// This spec with swatch `key` as `color`.
    pub fn with_color(mut self, color: FormColor, key: &str) -> Self {
        match color {
            FormColor::Primary => self.primary = key.to_string(),
            FormColor::Secondary => self.secondary = key.to_string(),
        }
        self
    }

    /// Option keys of `slot`: layer variants, or swatch keys for the colors.
    pub fn slot_keys(&self, slot: FormSlot) -> Vec<&'static str> {
        match slot {
            FormSlot::Layer(layer) => self.catalog().variants(layer).iter().map(String::as_str).collect(),
            FormSlot::Color(color) => self.swatches(color).iter().map(|swatch| swatch.key.as_str()).collect(),
        }
    }

    /// Index of the current choice of `slot` within [`Self::slot_keys`].
    pub fn slot_index(&self, slot: FormSlot) -> usize {
        match slot {
            FormSlot::Layer(layer) => self.slot_keys(slot).iter().position(|key| *key == self.part(layer)).unwrap_or_default(),
            FormSlot::Color(color) => self.swatch_index(color),
        }
    }

    /// This spec with option `index` of `slot` chosen (out-of-range indices change nothing).
    pub fn with_slot_index(self, slot: FormSlot, index: usize) -> Self {
        let Some(key) = self.slot_keys(slot).get(index).copied() else { return self };
        match slot {
            FormSlot::Layer(layer) => self.with_part(layer, key),
            FormSlot::Color(color) => self.with_color(color, key),
        }
    }

    /// Face placement of the current form.
    pub fn anchor(&self) -> FaceAnchor {
        self.catalog().hints(self.form()).anchor
    }

    /// (layer, asset path) of every drawn layer in paint order; `none` layers are skipped.
    pub fn layer_paths(&self) -> Vec<(FormLayer, String)> {
        let catalog = self.catalog();
        catalog
            .stack
            .iter()
            .map(|layer| (*layer, self.part(*layer)))
            .filter(|(_, variant)| *variant != NONE_VARIANT)
            .map(|(layer, variant)| (layer, catalog.asset_path(self.kind, layer, variant)))
            .collect()
    }
}
