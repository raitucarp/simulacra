//! Coherent random forms: preferred parts and colors from the catalog's per-form hints, and
//! the catalog's odds of leaving optional layers empty.

use super::catalog::LayerCatalog;
use super::kind::FormKind;
use super::layer::{FormColor, FormLayer, FormSlot, NONE_VARIANT};
use super::spec::FormSpec;
use crate::domain::portrait::PortraitRng;

/// Re-roll attempts before accepting an unchanged slot (single-option slots never change).
const REROLL_ATTEMPTS: usize = 8;

impl FormSpec {
    /// A random body of `kind` showing `form`; the same inputs always yield the same spec.
    pub fn random(kind: FormKind, form: &str, seed: u64) -> Self {
        let mut rng = PortraitRng::new(seed);
        let mut spec = Self { seed, ..Self::new(kind, form) };
        for slot in FormSlot::ALL.into_iter().filter(|slot| *slot != FormSlot::Layer(FormLayer::Body)) {
            spec.randomize_slot(slot, &mut rng);
        }
        spec
    }

    /// A random body of `kind`, the form included.
    pub fn random_any(kind: FormKind, seed: u64) -> Self {
        let mut rng = PortraitRng::new(seed);
        let forms = Self::new(kind, NONE_VARIANT).catalog().variants(FormLayer::Body);
        let form = forms.get(rng.below(forms.len())).map_or(NONE_VARIANT, String::as_str);
        Self::random(kind, form, rng.next_u64())
    }

    /// Re-rolls one slot, leaving every other choice untouched.
    pub fn randomize_slot(&mut self, slot: FormSlot, rng: &mut PortraitRng) {
        let hints = self.catalog().hints(self.form());
        match slot {
            FormSlot::Layer(layer) => {
                let entry = self.catalog().layers.get(&layer).cloned().unwrap_or_default();
                let variant = pick_variant(&entry, hints.prefer.get(layer.key()), rng);
                self.parts.insert(layer.key().to_string(), variant);
            }
            FormSlot::Color(color) => {
                let all: Vec<String> = self.swatches(color).iter().map(|swatch| swatch.key.clone()).collect();
                let keys = hints.prefer.get(color.key()).filter(|keys| !keys.is_empty()).unwrap_or(&all);
                if let Some(key) = keys.get(rng.below(keys.len())) {
                    *self = std::mem::take(self).with_color(color, key);
                }
            }
        }
    }

    /// This spec with `slot` re-rolled from `seed`, changed whenever the slot has a choice.
    pub fn rerolled(&self, slot: FormSlot, seed: u64) -> Self {
        let mut rng = PortraitRng::new(seed);
        let mut next = self.clone();
        for _ in 0..REROLL_ATTEMPTS {
            next.randomize_slot(slot, &mut rng);
            if next.slot_index(slot) != self.slot_index(slot) {
                break;
            }
        }
        next
    }

    /// Swatch key of `color` currently shown (the default when unset).
    pub fn color_key(&self, color: FormColor) -> &'static str {
        self.swatches(color).get(self.swatch_index(color)).map_or("", |swatch| swatch.key.as_str())
    }
}

/// A preferred variant when the form names some, else `none` at the layer's bare odds, else
/// any real variant.
fn pick_variant(entry: &LayerCatalog, prefer: Option<&Vec<String>>, rng: &mut PortraitRng) -> String {
    if let Some(preferred) = prefer.filter(|keys| !keys.is_empty()) {
        return preferred[rng.below(preferred.len())].clone();
    }
    let real: Vec<&String> = entry.variants.iter().filter(|v| *v != NONE_VARIANT).collect();
    let optional = entry.variants.iter().any(|v| v == NONE_VARIANT);
    let bare = optional && entry.bare.is_some_and(|(numerator, denominator)| rng.chance(numerator, denominator));
    match real.get(rng.below(real.len())) {
        Some(variant) if !bare => (*variant).clone(),
        _ => NONE_VARIANT.to_string(),
    }
}
