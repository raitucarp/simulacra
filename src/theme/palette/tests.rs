use super::*;
use gpui_kit::rgb;

const SAMPLE: &str = r##"{
  "id": "sample", "name": "Sample", "mode": "light", "credit": "Test (MIT)",
  "colors": {
    "bg": "#fafafa", "surface": "#f0f0f1", "elevated": "#e5e5e6", "border": "#d4d4d6",
    "fg": "#383a42", "fg_muted": "#626570", "accent": "#4078f2", "accent_2": "#a626a4",
    "agent": "#0184bc", "success": "#50a14f", "warning": "#986801", "danger": "#e45649",
    "info": "#0184bc", "selection": "#d7e3fc"
  }
}"##;

#[test]
fn parses_the_asset_schema() {
    let palette = ThemePalette::from_json(SAMPLE).expect("parse");
    assert_eq!(palette.mode, PaletteMode::Light);
    assert_eq!(palette.colors.accent, rgb(0x4078f2));
    assert_eq!(palette.based_on, None);
}

#[test]
fn json_roundtrip_is_lossless() {
    let palette = ThemePalette::from_json(SAMPLE).expect("parse");
    let json = palette.to_json_pretty().expect("serialize");
    assert!(!json.contains("based_on"), "absent base must not serialize");
    assert_eq!(ThemePalette::from_json(&json).expect("reparse"), palette);
}

#[test]
fn rejects_malformed_colors() {
    let broken = SAMPLE.replace("#fafafa", "not-a-color");
    assert!(ThemePalette::from_json(&broken).is_err());
}

#[test]
fn overrides_are_partial_and_serialize_by_field_name() {
    let palette = ThemePalette::from_json(SAMPLE).expect("parse");
    let mut overrides = PaletteOverrides::default();
    overrides.set(PaletteColorKey::Accent, rgb(0xff0000));
    let json = serde_json::to_string(&overrides).expect("serialize");
    assert_eq!(json, r##"{"accent":"#ff0000"}"##);
    let back: PaletteOverrides = serde_json::from_str(&json).expect("parse");
    let applied = palette.with_overrides(&back);
    assert_eq!(applied.colors.accent, rgb(0xff0000));
    assert_eq!(applied.colors.bg, palette.colors.bg);
}

#[test]
fn every_key_reads_and_writes_its_own_field() {
    let mut colors = ThemePalette::from_json(SAMPLE).expect("parse").colors;
    for (index, key) in PaletteColorKey::ALL.iter().enumerate() {
        let marker = rgb(index as u32 + 1);
        key.set(&mut colors, marker);
        assert_eq!(key.get(&colors), marker, "{}", key.id());
    }
    assert_eq!(PaletteColorKey::ALL.len(), 14);
}
