//! Per-kind layer catalogs (`assets/illustrations/forms/<kind>/catalog.json`): every layer's
//! variants in paint order, the primary/secondary swatches and per-form hints.

use super::kind::FormKind;
use super::layer::{FormLayer, NONE_VARIANT};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Asset folder of form parts, relative to the asset root.
pub const FORM_ASSET_ROOT: &str = "illustrations/forms";
/// Folder of the parts every kind shares (backdrops, the non-human face set).
pub const SHARED_DIR: &str = "shared";
/// File name of a kind's catalog.
pub const CATALOG_FILE: &str = "catalog.json";
const ASSET_EXT: &str = "svg";

/// The catalog files, embedded (served to the UI through `AppAssets` as well).
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/illustrations/forms"]
#[include = "*/catalog.json"]
pub struct FormCatalogFiles;

/// One `#rrggbb` color a form can be painted with.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Swatch {
    pub key: String,
    pub hex: String,
}

/// Where a layer's variants live and how the composer stacks them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct LayerCatalog {
    /// Variant keys; the first is the default. [`NONE_VARIANT`] draws nothing.
    pub variants: Vec<String>,
    /// Asset folder under `forms/` (the kind's own folder when absent).
    pub dir: Option<String>,
    /// Clip this layer to the body silhouette.
    pub clip: bool,
    /// Move this layer with the form's face anchor.
    pub anchor: bool,
    /// Odds (numerator, denominator) that a random roll leaves this layer empty.
    pub bare: Option<(u64, u64)>,
}

/// Default face centre on the 256-unit canvas (eyes at y 124, mouth at y 152; see STYLE.md).
pub const FACE_CENTER: (f32, f32) = (128.0, 136.0);

/// Face placement of one form: offset (`dx`, `dy`) and `scale` around [`FACE_CENTER`].
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct FaceAnchor(pub f32, pub f32, pub f32);

impl FaceAnchor {
    /// SVG `matrix(a b c d e f)` values moving the default face onto this anchor.
    pub fn matrix(self) -> [f32; 6] {
        let Self(dx, dy, scale) = self;
        let (cx, cy) = FACE_CENTER;
        [scale, 0.0, 0.0, scale, cx * (1.0 - scale) + dx, cy * (1.0 - scale) + dy]
    }
}

impl Default for FaceAnchor {
    fn default() -> Self {
        Self(0.0, 0.0, 1.0)
    }
}

/// Per-form hints: face anchor and the parts/colors a random roll prefers.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct FormHints {
    pub anchor: FaceAnchor,
    /// Layer key (or `primary` / `secondary`) → preferred variant or swatch keys.
    pub prefer: BTreeMap<String, Vec<String>>,
}

/// One kind's catalog.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct FormCatalog {
    /// Paint order, bottom to top.
    pub stack: Vec<FormLayer>,
    pub layers: BTreeMap<FormLayer, LayerCatalog>,
    pub primary: Vec<Swatch>,
    pub secondary: Vec<Swatch>,
    pub forms: BTreeMap<String, FormHints>,
}

impl FormCatalog {
    /// The embedded catalog of `kind` (empty when missing or unreadable, so nothing panics).
    pub fn of(kind: FormKind) -> &'static FormCatalog {
        static CATALOGS: OnceLock<BTreeMap<FormKind, FormCatalog>> = OnceLock::new();
        static EMPTY: OnceLock<FormCatalog> = OnceLock::new();
        let all = CATALOGS.get_or_init(|| FormKind::ALL.into_iter().map(|kind| (kind, Self::load(kind))).collect());
        all.get(&kind).unwrap_or_else(|| EMPTY.get_or_init(FormCatalog::default))
    }

    /// Parses the embedded catalog of `kind`.
    fn load(kind: FormKind) -> FormCatalog {
        FormCatalogFiles::get(&format!("{}/{CATALOG_FILE}", kind.key()))
            .and_then(|file| serde_json::from_slice(&file.data).ok())
            .unwrap_or_default()
    }

    /// The variants of `layer` (empty when the kind has no such layer).
    pub fn variants(&self, layer: FormLayer) -> &[String] {
        self.layers.get(&layer).map_or(&[], |entry| entry.variants.as_slice())
    }

    /// The default variant of `layer`.
    pub fn default_variant(&self, layer: FormLayer) -> &str {
        self.variants(layer).first().map_or(NONE_VARIANT, String::as_str)
    }

    /// Hints of `form` (defaults when it has none).
    pub fn hints(&self, form: &str) -> FormHints {
        self.forms.get(form).cloned().unwrap_or_default()
    }

    /// Asset path of `variant` on `layer` for `kind`: `illustrations/forms/<dir>/<layer>/<variant>.svg`.
    pub fn asset_path(&self, kind: FormKind, layer: FormLayer, variant: &str) -> String {
        let dir = self.layers.get(&layer).and_then(|entry| entry.dir.as_deref()).unwrap_or(kind.key());
        format!("{FORM_ASSET_ROOT}/{dir}/{}/{variant}.{ASSET_EXT}", layer.key())
    }
}
