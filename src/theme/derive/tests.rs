use super::*;
use crate::theme::color::{composite, contrast_ratio, relative_luminance, AA_TEXT_CONTRAST, AA_UI_CONTRAST};
use crate::theme::palette::PaletteMode;
use crate::theme::registry::{ThemeRegistry, DEFAULT_PALETTE_ID};
use gpui_kit::{px, rgb, rgba, Rgba};

/// Largest per-channel drift (0–1) tolerated against the pre-palette Simulacra Midnight look.
const LEGACY_TOLERANCE: f32 = 16.0 / 255.0;

fn palette(id: &str) -> &'static crate::theme::ThemePalette {
    ThemeRegistry::global().get(id).unwrap_or_else(|| panic!("missing palette {id}"))
}

fn assert_close(name: &str, actual: Rgba, expected: Rgba) {
    let drift = [actual.r - expected.r, actual.g - expected.g, actual.b - expected.b, actual.a - expected.a]
        .map(f32::abs)
        .into_iter()
        .fold(0.0, f32::max);
    assert!(drift <= LEGACY_TOLERANCE, "{name}: {actual:?} drifted from {expected:?} by {drift}");
}

#[test]
fn simulacra_midnight_matches_the_legacy_look() {
    let t = derive_default_tokens(palette(DEFAULT_PALETTE_ID));
    let legacy = [
        ("bg_app", t.bg_app, rgb(0x000000)), ("bg_card", t.bg_card, rgb(0x08080a)),
        ("bg_elevated", t.bg_elevated, rgb(0x111115)), ("bg_feed_post", t.bg_feed_post, rgb(0x000000)),
        ("border", t.border, rgb(0x1e1e22)), ("border_subtle", t.border_subtle, rgb(0x18181b)),
        ("border_strong", t.border_strong, rgb(0x27272a)), ("border_focus", t.border_focus, rgb(0x8b5cf6)),
        ("text_primary", t.text_primary, rgb(0xf4f4f5)), ("text_secondary", t.text_secondary, rgb(0xa1a1aa)),
        ("text_muted", t.text_muted, rgb(0x71717a)), ("accent_brand", t.accent_brand, rgb(0x8b5cf6)),
        ("accent_user", t.accent_user, rgb(0x38bdf8)), ("accent_dialectic", t.accent_dialectic, rgb(0xeab308)),
        ("accent_success", t.accent_success, rgb(0x10b981)), ("accent_danger", t.accent_danger, rgb(0xef4444)),
        ("hover_bg", t.hover_bg, rgba(0xffffff0a)), ("bg_tab_active", t.bg_tab_active, rgba(0xffffff0d)),
        ("bg_callout", t.bg_callout, rgba(0x8b5cf612)), ("brand_subtle", t.accent_brand_subtle, rgba(0x8b5cf620)),
        ("bg_scrim", t.bg_scrim, rgba(0x000000dd)),
    ];
    for (name, actual, expected) in legacy {
        assert_close(name, actual, expected);
    }
    assert_eq!(t.mode, PaletteMode::Dark);
    assert_eq!(t.font_size_base, px(14.0));
    assert_eq!(t.spacing_lg, px(16.0));
    assert_eq!(t.radius_md, px(8.0));
}

#[test]
fn derivation_is_deterministic() {
    for palette in ThemeRegistry::global().all() {
        let a = derive_tokens(palette, Density::Cozy, TypeScale::Large, RadiusScale::Round);
        let b = derive_tokens(palette, Density::Cozy, TypeScale::Large, RadiusScale::Round);
        assert_eq!(a, b, "{}", palette.id);
        assert_eq!(serde_json::to_string(&a).ok(), serde_json::to_string(&b).ok());
    }
}

#[test]
fn every_preset_passes_the_contrast_guard() {
    for palette in ThemeRegistry::global().all() {
        let t = derive_default_tokens(palette);
        for (role, text) in [("text_primary", t.text_primary), ("text_muted", t.text_muted)] {
            for (surface, bg) in [("bg_app", t.bg_app), ("bg_card", t.bg_card), ("bg_elevated", t.bg_elevated)] {
                let ratio = contrast_ratio(text, bg);
                assert!(ratio >= AA_TEXT_CONTRAST, "{}: {role} on {surface} = {ratio:.2}", palette.id);
            }
        }
        let on_accent = contrast_ratio(t.on_accent, t.accent_brand);
        assert!(on_accent >= AA_UI_CONTRAST, "{}: on_accent = {on_accent:.2}", palette.id);
        assert!(contrast_ratio(t.on_danger, t.accent_danger) >= AA_UI_CONTRAST, "{}", palette.id);
    }
}

#[test]
fn light_palettes_get_light_mode_overlays() {
    let t = derive_default_tokens(palette("one-light"));
    assert_eq!(t.mode, PaletteMode::Light);
    assert_eq!(t.bg_feed_post, t.bg_app, "feed posts sit on the app background");
    let hovered = composite(t.hover_bg, t.bg_card);
    assert!(relative_luminance(hovered) < relative_luminance(t.bg_card), "hover must darken");
    let dark = derive_default_tokens(palette(DEFAULT_PALETTE_ID));
    assert!(t.shadow.a < dark.shadow.a && t.bg_scrim.a < dark.bg_scrim.a);
    assert!(relative_luminance(t.text_primary) < relative_luminance(t.bg_app));
}

#[test]
fn scales_resize_metrics_but_not_colors() {
    let p = palette(DEFAULT_PALETTE_ID);
    let compact = derive_tokens(p, Density::Compact, TypeScale::Small, RadiusScale::Sharp);
    let cozy = derive_tokens(p, Density::Cozy, TypeScale::Large, RadiusScale::Round);
    assert!(compact.spacing_lg < cozy.spacing_lg);
    assert!(compact.font_size_base < cozy.font_size_base);
    assert!(compact.radius_md < cozy.radius_md);
    assert_eq!(compact.radius_full, cozy.radius_full);
    assert_eq!(compact.bg_card, cozy.bg_card);
    assert_eq!(compact.titlebar_height, cozy.titlebar_height);
}

#[test]
fn semantic_spacing_follows_density_and_pairs_padding_with_gaps() {
    let p = palette(DEFAULT_PALETTE_ID);
    let t = derive_default_tokens(p);
    let roles = [
        (t.panel_padding, 16.0), (t.section_gap, 16.0), (t.panel_padding_compact, 8.0), (t.stack_gap, 12.0),
        (t.card_padding, 12.0), (t.control_gap, 8.0), (t.inline_gap, 4.0), (t.paragraph_break_height, 8.0),
    ];
    for (value, expected) in roles {
        assert_eq!(value, px(expected));
    }
    assert_eq!(t.panel_padding, t.section_gap);
    assert_eq!(t.card_padding, t.stack_gap);
    for density in [Density::Compact, Density::Cozy] {
        let scaled = derive_tokens(p, density, TypeScale::default(), RadiusScale::default());
        assert_eq!(scaled.panel_padding, scaled.spacing_lg);
        assert_eq!(scaled.card_padding, scaled.spacing_md);
        assert_ne!(scaled.panel_padding, t.panel_padding);
        assert_eq!(scaled.dot_size_sm, t.dot_size_sm, "indicator sizes do not follow density");
    }
}
