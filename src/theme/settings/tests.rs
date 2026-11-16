use super::*;
use crate::theme::palette::PaletteColorKey;
use crate::theme::registry::OBSIDIAN_ZINC_ID;
use gpui_kit::{px, rgb};

const DUPLICATE_NAME: &str = "My Theme";

#[test]
fn default_settings_reproduce_the_default_tokens() {
    assert_eq!(ThemeSettings::default().tokens(), crate::theme::minimalist_dark());
}

#[test]
fn unknown_preset_falls_back_to_the_default_palette() {
    let settings = ThemeSettings { preset_id: "gone".into(), ..ThemeSettings::default() };
    assert_eq!(settings.base_palette().id, DEFAULT_PALETTE_ID);
}

#[test]
fn bundled_palettes_are_not_editable() {
    let mut settings = ThemeSettings::default();
    assert!(!settings.set_color(PaletteColorKey::Accent, rgb(0xff0000)));
    assert!(settings.overrides.is_empty());
}

#[test]
fn duplicate_edit_select_and_reset_cycle() {
    let mut settings = ThemeSettings::default();
    settings.select(OBSIDIAN_ZINC_ID);
    let id = settings.duplicate_active(DUPLICATE_NAME.into());
    assert_eq!(settings.preset_id, id);
    assert_eq!(settings.base_palette().based_on.as_deref(), Some(OBSIDIAN_ZINC_ID));
    assert!(settings.set_color(PaletteColorKey::Accent, rgb(0xff0000)));
    assert_eq!(settings.tokens().accent_brand, rgb(0xff0000));

    settings.select(DEFAULT_PALETTE_ID);
    assert!(settings.overrides.is_empty(), "switching commits the edits");
    assert_eq!(settings.palette(&id).map(|p| p.colors.accent), Some(rgb(0xff0000)));

    settings.select(&id);
    settings.reset_to_preset();
    let origin = settings.palette(OBSIDIAN_ZINC_ID).map(|p| p.colors);
    assert_eq!(settings.palette(&id).map(|p| p.colors), origin);
}

#[test]
fn import_assigns_fresh_ids_and_delete_falls_back() {
    let mut settings = ThemeSettings::default();
    let imported = settings.palette(OBSIDIAN_ZINC_ID).cloned().expect("bundled");
    let first = settings.import(imported.clone());
    let second = settings.import(imported);
    assert_ne!(first, second);
    assert!(first.starts_with(CUSTOM_PALETTE_PREFIX));
    settings.delete_custom(&second);
    assert_eq!(settings.preset_id, DEFAULT_PALETTE_ID);
    assert!(settings.is_custom(&first) && !settings.is_custom(&second));
}

#[test]
fn scales_and_font_reach_the_tokens() {
    let settings = ThemeSettings {
        density: Density::Compact,
        type_scale: TypeScale::Large,
        font_family: "Georgia".into(),
        ..ThemeSettings::default()
    };
    let tokens = settings.tokens();
    assert!(tokens.spacing_lg < px(16.0) && tokens.font_size_base > px(14.0));
    assert_eq!(tokens.font_family_body, "Georgia");
    assert_eq!(tokens.font_family_mono, crate::theme::derive::DEFAULT_MONO_FONT_FAMILY);
}

#[test]
fn legacy_preset_tokens_select_the_preset() {
    let mut settings = ThemeSettings::default();
    let mut legacy = crate::theme::minimalist_dark();
    legacy.bg_app = rgb(0x09090b);
    legacy.accent_brand = rgb(0xa855f7);
    settings.adopt_legacy_tokens(&legacy);
    assert_eq!(settings.preset_id, OBSIDIAN_ZINC_ID);
    assert!(settings.custom.is_empty());
}

#[test]
fn customized_legacy_tokens_become_a_migrated_palette() {
    let mut settings = ThemeSettings::default();
    let mut legacy = crate::theme::minimalist_dark();
    legacy.accent_brand = rgb(0xff00aa);
    legacy.font_family_ui = "Georgia".into();
    settings.adopt_legacy_tokens(&legacy);
    settings.adopt_legacy_tokens(&legacy);
    assert_eq!(settings.preset_id, MIGRATED_PALETTE_ID);
    assert_eq!(settings.custom.len(), 1, "re-migration replaces, never duplicates");
    let palette = settings.base_palette();
    assert_eq!(palette.name, MIGRATED_PALETTE_NAME);
    assert_eq!(palette.colors.accent, rgb(0xff00aa));
    assert_eq!(palette.colors.bg, legacy.bg_app);
    assert_eq!(settings.font_family, "Georgia");
    assert_eq!(settings.tokens().accent_brand, rgb(0xff00aa));
}
