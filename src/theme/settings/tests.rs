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

mod appearance {
    use super::super::{AppearanceMode, PreferredThemes, ThemeSettings};
    use crate::theme::palette::PaletteMode::{Dark, Light};
    use crate::theme::registry::{DEFAULT_LIGHT_PALETTE_ID, DEFAULT_PALETTE_ID, OBSIDIAN_ZINC_ID};

    const ONE_LIGHT: &str = "one-light";

    #[test]
    fn modes_round_trip_cycle_and_resolve() {
        for mode in AppearanceMode::ALL {
            assert_eq!(AppearanceMode::from_id(mode.id()), Some(*mode));
        }
        assert_eq!(AppearanceMode::from_id("sepia"), None);
        let cycle: Vec<_> = AppearanceMode::ALL.iter().map(|mode| mode.next()).collect();
        assert_eq!(cycle, [AppearanceMode::Light, AppearanceMode::System, AppearanceMode::Dark]);
        assert_eq!(AppearanceMode::System.resolve(Light), Light);
        assert_eq!(AppearanceMode::Dark.resolve(Light), Dark);
    }

    #[test]
    fn system_mode_follows_the_os_with_each_preferred_palette() {
        let mut settings = ThemeSettings::default();
        settings.set_preferred(Light, ONE_LIGHT, Dark);
        assert_eq!(settings.preset_id, DEFAULT_PALETTE_ID, "light is not showing yet");
        settings.set_appearance(AppearanceMode::System, Light);
        assert_eq!(settings.preset_id, ONE_LIGHT);
        settings.apply_appearance(Dark);
        assert_eq!(settings.preset_id, DEFAULT_PALETTE_ID);
        assert_eq!(settings.tokens().mode, Dark);
    }

    #[test]
    fn picking_a_palette_makes_it_preferred_and_shows_its_mode() {
        let mut settings = ThemeSettings::default();
        settings.select(OBSIDIAN_ZINC_ID);
        assert_eq!(settings.preferred.dark, OBSIDIAN_ZINC_ID);
        settings.select(ONE_LIGHT);
        settings.follow_active_palette(Dark);
        assert_eq!(settings.appearance, AppearanceMode::Light);
        settings.apply_appearance(Dark);
        assert_eq!(settings.preset_id, ONE_LIGHT, "the pick is kept");
        settings.set_appearance(AppearanceMode::Dark, Light);
        assert_eq!(settings.preset_id, OBSIDIAN_ZINC_ID, "each mode remembers its palette");
    }

    #[test]
    fn missing_or_mismatched_preferences_fall_back_to_the_mode_default() {
        let preferred = PreferredThemes { dark: ONE_LIGHT.into(), light: "gone".into() };
        let settings = ThemeSettings { preferred, ..ThemeSettings::default() };
        assert_eq!(settings.preferred_id(Dark), DEFAULT_PALETTE_ID);
        assert_eq!(settings.preferred_id(Light), DEFAULT_LIGHT_PALETTE_ID);
    }

    #[test]
    fn old_settings_keep_their_palette_as_the_preferred_one_of_its_mode() {
        let mut settings = ThemeSettings { preset_id: ONE_LIGHT.into(), ..ThemeSettings::default() };
        settings.adopt_active_as_preferred();
        assert_eq!(settings.appearance, AppearanceMode::Light);
        assert_eq!(settings.preferred, PreferredThemes { dark: DEFAULT_PALETTE_ID.into(), light: ONE_LIGHT.into() });
        settings.apply_appearance(Dark);
        assert_eq!(settings.preset_id, ONE_LIGHT, "the look survives the upgrade");
    }
}
