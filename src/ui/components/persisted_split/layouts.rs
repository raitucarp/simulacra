//! Saved panel sizes of every resizable layout, keyed by layout id. Pure data: parsing is
//! tolerant (a corrupt row or entry is dropped, never fatal) and sizes are clamped on restore.

use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Layout id → panel sizes (logical pixels, in panel order).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PanelLayouts {
    sizes: BTreeMap<String, Vec<f32>>,
}

/// A stored size is usable when it is a finite, positive number.
fn usable(size: f32) -> bool {
    size.is_finite() && size > 0.0
}

/// The sizes of one stored entry, or `None` when any element is not a usable number.
fn entry_sizes(value: &Value) -> Option<Vec<f32>> {
    let sizes: Option<Vec<f32>> = value.as_array()?.iter().map(|v| v.as_f64().map(|n| n as f32)).collect();
    sizes.filter(|sizes| !sizes.is_empty() && sizes.iter().copied().all(usable))
}

impl PanelLayouts {
    /// Parses the stored JSON object; unreadable JSON gives no layouts and unreadable entries
    /// are skipped.
    pub fn from_json(raw: &str) -> Self {
        let Ok(Value::Object(map)) = serde_json::from_str::<Value>(raw) else {
            return Self::default();
        };
        let sizes = map.iter().filter_map(|(id, value)| Some((id.clone(), entry_sizes(value)?))).collect();
        Self { sizes }
    }

    /// The JSON object stored in the settings table.
    pub fn to_json(&self) -> String {
        let map: Map<String, Value> = self.sizes.iter().map(|(id, sizes)| (id.clone(), Value::from(sizes.clone()))).collect();
        Value::Object(map).to_string()
    }

    /// Saved sizes of layout `id`.
    pub fn get(&self, id: &str) -> Option<&[f32]> {
        self.sizes.get(id).map(Vec::as_slice)
    }

    /// Saved size of panel `ix` of layout `id`.
    pub fn size_of(&self, id: &str, ix: usize) -> Option<f32> {
        self.get(id).and_then(|sizes| sizes.get(ix)).copied()
    }

    /// Records the sizes of layout `id`; returns whether anything changed. Unusable sizes
    /// (zero, negative, not finite) are ignored.
    pub fn set(&mut self, id: &str, sizes: Vec<f32>) -> bool {
        if sizes.is_empty() || !sizes.iter().copied().all(usable) || self.get(id) == Some(sizes.as_slice()) {
            return false;
        }
        self.sizes.insert(id.to_string(), sizes);
        true
    }

    /// Forgets layout `id`; returns whether it was saved.
    pub fn remove(&mut self, id: &str) -> bool {
        self.sizes.remove(id).is_some()
    }

    /// Forgets every layout.
    pub fn clear(&mut self) {
        self.sizes.clear();
    }

    /// Whether no layout is saved.
    pub fn is_empty(&self) -> bool {
        self.sizes.is_empty()
    }
}

/// Size a pane opens at: the saved size clamped to `min..=max`, else the default.
pub fn restored_size(saved: Option<f32>, default: f32, min: f32, max: f32) -> f32 {
    let size = saved.filter(|size| usable(*size)).unwrap_or(default);
    size.max(min).min(max.max(min))
}
