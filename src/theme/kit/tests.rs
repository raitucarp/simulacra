use super::*;
use crate::theme::color::to_hex;
use crate::theme::derive::{derive_default_tokens, derive_tokens, Density, RadiusScale, TypeScale};
use crate::theme::registry::ThemeRegistry;
use gpui_kit::component::try_parse_color;
use gpui_kit::{Hsla, SharedString};

fn hex(color: gpui_kit::Rgba) -> Option<SharedString> {
    Some(to_hex(color).into())
}

fn hsla(color: gpui_kit::Rgba) -> Hsla {
    try_parse_color(&to_hex(color)).expect("hex parses")
}

#[test]
fn config_maps_the_core_tokens() {
    let tokens = crate::theme::minimalist_dark();
    let config = kit_theme_config(&tokens);
    let c = &config.colors;
    assert_eq!(config.mode, ThemeMode::Dark);
    assert_eq!(c.primary, hex(tokens.accent_brand));
    assert_eq!(c.primary_foreground, hex(tokens.on_accent));
    assert_eq!(c.background, hex(tokens.bg_app));
    assert_eq!(c.foreground, hex(tokens.text_primary));
    assert_eq!(c.muted_foreground, hex(tokens.text_muted));
    assert_eq!(c.border, hex(tokens.border));
    assert_eq!(c.ring, hex(tokens.focus_ring));
    assert_eq!(c.popover, hex(tokens.bg_elevated));
    assert_eq!(c.danger, hex(tokens.accent_danger));
    assert_eq!(c.selection, hex(tokens.selection));
    assert_eq!(c.scrollbar_thumb, hex(tokens.scrollbar));
    assert_eq!(c.chart_1, hex(tokens.accent_brand));
    assert_eq!(config.font_family.as_deref(), Some(tokens.font_family_ui.as_str()));
    assert_eq!(config.radius, Some(tokens.radius_sm.as_f32() as usize));
}

#[test]
fn every_preset_applies_to_a_kit_theme_without_a_window() {
    for palette in ThemeRegistry::global().all() {
        let tokens = derive_default_tokens(palette);
        let mut theme = Theme::default();
        theme.apply_config(&Rc::new(kit_theme_config(&tokens)));
        assert_eq!(theme.mode.is_dark(), tokens.mode.is_dark(), "{}", palette.id);
        assert_eq!(theme.primary, hsla(tokens.accent_brand), "{}", palette.id);
        assert_eq!(theme.primary_foreground, hsla(tokens.on_accent), "{}", palette.id);
        assert_eq!(theme.background, hsla(tokens.bg_app), "{}", palette.id);
        assert_eq!(theme.foreground, hsla(tokens.text_primary), "{}", palette.id);
        assert_eq!(theme.popover, hsla(tokens.bg_elevated), "{}", palette.id);
    }
}

#[test]
fn light_tokens_produce_a_light_kit_theme() {
    let palette = ThemeRegistry::global().get("github-light").expect("github-light");
    let config = kit_theme_config(&derive_default_tokens(palette));
    assert_eq!(config.mode, ThemeMode::Light);
}

fn scaled_tokens(type_scale: TypeScale, radius: RadiusScale) -> crate::theme::ThemeTokens {
    let palette = ThemeRegistry::global().all().first().expect("a preset palette");
    derive_tokens(palette, Density::default(), type_scale, radius)
}

#[test]
fn type_scale_reaches_the_kit_font_sizes() {
    let sizes: Vec<(f32, f32)> = [TypeScale::Small, TypeScale::Default, TypeScale::Large]
        .into_iter()
        .map(|scale| {
            let tokens = scaled_tokens(scale, RadiusScale::default());
            let config = kit_theme_config(&tokens);
            assert_eq!(config.font_size, Some(tokens.font_size_root.as_f32()));
            assert_eq!(config.mono_font_size, Some(tokens.font_size_sm.as_f32()));
            let mut theme = Theme::default();
            theme.apply_config(&Rc::new(config));
            assert_eq!(theme.font_size, tokens.font_size_root, "kit rem follows {scale:?}");
            assert_eq!(theme.mono_font_size, tokens.font_size_sm);
            (theme.font_size.as_f32(), theme.mono_font_size.as_f32())
        })
        .collect();
    assert!(sizes.windows(2).all(|pair| pair[0].0 < pair[1].0 && pair[0].1 < pair[1].1), "{sizes:?}");
}

#[test]
fn default_type_scale_keeps_the_kit_default_rem() {
    let kit_default = Theme::default().font_size;
    let tokens = scaled_tokens(TypeScale::Default, RadiusScale::default());
    assert_eq!(tokens.font_size_root, kit_default, "default scale must not resize kit widgets");
}

#[test]
fn radius_scale_reaches_the_kit_radii() {
    let sharp = kit_theme_config(&scaled_tokens(TypeScale::default(), RadiusScale::Sharp));
    let round = kit_theme_config(&scaled_tokens(TypeScale::default(), RadiusScale::Round));
    assert!(sharp.radius < round.radius && sharp.radius_lg < round.radius_lg);
}
