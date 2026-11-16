use super::*;
use std::collections::HashSet;

const BUNDLED_PALETTE_COUNT: usize = 28;
const THEMES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/themes");

#[test]
fn every_bundled_palette_parses() {
    for (id, json) in EMBEDDED_PALETTES {
        let palette = ThemePalette::from_json(json).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(&palette.id, id, "file name and `id` must agree");
        assert!(!palette.name.is_empty() && !palette.credit.is_empty(), "{id}");
    }
    assert_eq!(ThemeRegistry::global().len(), BUNDLED_PALETTE_COUNT);
}

#[test]
fn every_asset_file_is_embedded_exactly_once() {
    let on_disk: HashSet<String> = std::fs::read_dir(THEMES_DIR)
        .expect("themes dir")
        .flatten()
        .filter_map(|entry| entry.file_name().to_str()?.strip_suffix(".json").map(str::to_string))
        .collect();
    let embedded: HashSet<String> = EMBEDDED_PALETTES.iter().map(|(id, _)| id.to_string()).collect();
    assert_eq!(embedded.len(), EMBEDDED_PALETTES.len(), "duplicate embedded id");
    assert_eq!(on_disk, embedded);
}

#[test]
fn lookup_order_and_mode_filter() {
    let registry = ThemeRegistry::global();
    assert_eq!(registry.all()[0].id, DEFAULT_PALETTE_ID);
    assert_eq!(registry.default_palette().id, DEFAULT_PALETTE_ID);
    assert!(registry.get(OBSIDIAN_ZINC_ID).is_some());
    assert!(registry.get(MIDNIGHT_SLATE_ID).is_some());
    assert!(registry.get("no-such-theme").is_none());
    let dark = registry.by_mode(PaletteMode::Dark).count();
    let light = registry.by_mode(PaletteMode::Light).count();
    assert_eq!(dark + light, registry.len());
    assert!(light > 0 && dark > light);
    assert!(registry.by_mode(PaletteMode::Light).all(|p| p.mode == PaletteMode::Light));
}

#[test]
fn every_palette_credit_is_acknowledged_in_third_party_notices() {
    const NOTICES: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/THIRD_PARTY.md"));
    const LICENSE_SUFFIX: &str = " (MIT)";
    for palette in ThemeRegistry::global().all() {
        let author = palette.credit.trim_end_matches(LICENSE_SUFFIX);
        assert!(NOTICES.contains(author), "{}: `{author}` missing from THIRD_PARTY.md", palette.id);
        assert!(NOTICES.contains(palette.name.split_whitespace().next().unwrap_or_default()), "{}", palette.id);
    }
}
