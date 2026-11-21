use super::*;
use crate::theme::color::{contrast_ratio, relative_luminance, with_hue, AA_TEXT_CONTRAST};
use crate::theme::derive::derive_default_tokens;
use crate::theme::palette::PaletteMode;
use crate::theme::registry::ThemeRegistry;
use gpui_kit::rgb;

#[test]
fn text_on_every_tint_meets_aa_in_every_palette() {
    let mut modes = Vec::new();
    for palette in ThemeRegistry::global().all() {
        let t = derive_default_tokens(palette);
        modes.push(t.mode);
        for &tint in Tint::ALL {
            let background = tint_colors(tint, &t).background;
            for (role, text) in [("text_primary", t.text_primary), ("text_secondary", t.text_secondary), ("text_muted", t.text_muted)] {
                let ratio = contrast_ratio(text, background);
                assert!(ratio >= AA_TEXT_CONTRAST, "{}: {role} on {} = {ratio:.2}", palette.id, tint.key());
            }
        }
    }
    assert!(modes.contains(&PaletteMode::Light) && modes.contains(&PaletteMode::Dark), "both modes are covered");
}

#[test]
fn every_tint_is_visible_and_distinct() {
    for palette in ThemeRegistry::global().all() {
        let t = derive_default_tokens(palette);
        let colors: Vec<TintColors> = Tint::ALL.iter().map(|&tint| tint_colors(tint, &t)).collect();
        for (at, (tint, color)) in Tint::ALL.iter().zip(&colors).enumerate() {
            assert_ne!(color.background, t.bg_app, "{}: {} background is invisible", palette.id, tint.key());
            assert_ne!(color.border, t.bg_app, "{}: {} border is invisible", palette.id, tint.key());
            assert!(colors[..at].iter().all(|other| other.background != color.background), "{}: {} repeats a tint", palette.id, tint.key());
        }
    }
}

#[test]
fn tints_follow_the_palette_mode() {
    let registry = ThemeRegistry::global();
    let (dark, light) = (derive_default_tokens(registry.get("simulacra-midnight").unwrap()), derive_default_tokens(registry.get("one-light").unwrap()));
    for &tint in Tint::ALL {
        let (on_dark, on_light) = (tint_colors(tint, &dark), tint_colors(tint, &light));
        assert!(relative_luminance(on_dark.background) < relative_luminance(on_light.background), "{}", tint.key());
        assert!(relative_luminance(on_dark.border) > relative_luminance(on_dark.background), "{}: dark border lifts", tint.key());
        assert!(relative_luminance(on_light.border) < relative_luminance(on_light.background), "{}: light border deepens", tint.key());
    }
}

#[test]
fn a_new_hue_keeps_saturation_and_lightness() {
    let sky = rgb(0x38bdf8);
    let violet = with_hue(sky, 262.0);
    assert!(violet.b > violet.r && violet.r > violet.g, "{violet:?} is violet");
    let lightness = |c: gpui_kit::Rgba| (c.r.max(c.g).max(c.b) + c.r.min(c.g).min(c.b)) / 2.0;
    assert!((lightness(violet) - lightness(sky)).abs() < 1e-5);
    let back = with_hue(violet, 199.0);
    assert!((back.r - sky.r).abs() < 0.02 && (back.g - sky.g).abs() < 0.02 && (back.b - sky.b).abs() < 0.02, "{back:?}");
    assert_eq!(with_hue(rgb(0x808080), 120.0), rgb(0x808080), "grey has no hue to turn");
}

#[test]
fn tokens_without_tints_fall_back_to_the_default_theme() {
    let loaded: ThemeTokens = serde_json::from_str(r#"{"bg_app":0}"#).expect("old theme JSON loads");
    assert_eq!(tint_colors(Tint::Amber, &loaded), tint_colors(Tint::Amber, &ThemeTokens::default()));
}
