//! The embedded palette catalog, parsed once on first use. Where a file sits decides what it is:
//! `<category>/<key>.json` is a full palette, `steps/<step>/<key>.json` a step palette.

use super::{NameSuggestion, Palette, PaletteCategory, PaletteScope, STEPS_DIR};
use crate::domain::EnneagramType;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::sync::OnceLock;

/// Every palette file (read from disk in debug builds, embedded in release builds).
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/palettes"]
#[include = "**/*.json"]
pub struct PaletteFiles;

const PATH_SEPARATOR: char = '/';
const JSON_SUFFIX: &str = ".json";

/// A palette file as stored.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct PaletteFile {
    order: u16,
    names: Vec<NameSuggestion>,
    enneagram_options: Vec<EnneagramType>,
    config: Map<String, Value>,
}

/// The scope and key a catalog path stands for (`None` for a misplaced file).
pub(super) fn scope_of(path: &str) -> Option<(PaletteScope, String)> {
    let stem = path.strip_suffix(JSON_SUFFIX)?;
    let parts: Vec<&str> = stem.split(PATH_SEPARATOR).collect();
    match parts.as_slice() {
        [STEPS_DIR, step, key] => Some((PaletteScope::Step((*step).to_string()), (*key).to_string())),
        [category, key] => PaletteCategory::from_key(category).map(|c| (PaletteScope::Full(c), (*key).to_string())),
        _ => None,
    }
}

/// Parses the catalog file at `path` (`None` when misplaced or malformed; the catalog tests
/// reject both, so this never drops a shipped palette).
pub(super) fn parse(path: &str, bytes: &[u8]) -> Option<Palette> {
    let (scope, key) = scope_of(path)?;
    let file: PaletteFile = serde_json::from_slice(bytes).ok()?;
    Some(Palette { key, scope, order: file.order, names: file.names, enneagram_options: file.enneagram_options, config: file.config })
}

/// Sort key: full palettes by category, step palettes by step, then order and key.
fn rank(palette: &Palette) -> (u8, Option<PaletteCategory>, String, u16, String) {
    let step = palette.step_key().unwrap_or_default().to_string();
    (u8::from(!palette.is_full()), palette.category(), step, palette.order, palette.key.clone())
}

fn load() -> Vec<Palette> {
    let mut all: Vec<Palette> = PaletteFiles::iter()
        .filter_map(|path| PaletteFiles::get(&path).and_then(|file| parse(&path, &file.data)))
        .collect();
    all.sort_by_key(rank);
    all
}

impl Palette {
    /// Every palette: full ones by category, then step ones by step.
    pub fn all() -> &'static [Palette] {
        static ALL: OnceLock<Vec<Palette>> = OnceLock::new();
        ALL.get_or_init(load)
    }

    /// The palette with key `key`.
    pub fn find(key: &str) -> Option<&'static Palette> {
        Self::all().iter().find(|palette| palette.key == key)
    }

    /// Full palettes of `category` (every category when `None`), in gallery order.
    pub fn full(category: Option<PaletteCategory>) -> impl Iterator<Item = &'static Palette> {
        Self::all().iter().filter(move |p| p.is_full() && (category.is_none() || p.category() == category))
    }

    /// Step palettes of the step with key `step`, in strip order.
    pub fn for_step(step: &str) -> impl Iterator<Item = &'static Palette> + '_ {
        Self::all().iter().filter(move |palette| palette.step_key() == Some(step))
    }
}
