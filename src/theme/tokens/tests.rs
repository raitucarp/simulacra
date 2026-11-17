use super::*;

#[test]
fn test_theme_tokens_default() {
    let default_tokens = ThemeTokens::default();
    assert_eq!(default_tokens.titlebar_height, px(38.0));
    assert_eq!(default_tokens.feed_header_height, px(44.0));
    assert_eq!(default_tokens.line_height_body, 1.45);
    assert_eq!(default_tokens.bg_feed_post, gpui_kit::rgb(0x000000));
}

#[test]
fn test_theme_tokens_serde_roundtrip() {
    let tokens = ThemeTokens::default();
    let json = serde_json::to_string(&tokens).expect("serialize failed");
    let deserialized: ThemeTokens = serde_json::from_str(&json).expect("deserialize failed");
    assert_eq!(tokens, deserialized);
}

#[test]
fn test_theme_tokens_serde_backward_compatibility() {
    // Test that an older JSON representation missing new fields deserializes with defaults
    let old_json = r#"{
        "font_family_ui":"Segoe UI","font_family_display":"Segoe UI","font_family_body":"Segoe UI","font_family_mono":"Consolas",
        "bg_app":0,"bg_card":0,"bg_elevated":789518,"bg_sidebar":0,"bg_callout":2338125330,"hover_bg":4294967048,
        "border":505291266,"border_subtle":404233218,"border_strong":656878338,"border_focus":2338125566,
        "text_primary":4109694463,"text_secondary":2711726847,"text_muted":1903262463,
        "accent_user":951974143,"accent_alt":3937601791,"accent_agent":2824206335,"accent_brand":2338125566,
        "accent_dialectic":3937601791,"accent_success":280592895,
        "font_size_2xs":10.0,"font_size_xs":11.5,"font_size_sm":13.0,"font_size_base":14.0,"font_size_md":15.0,"font_size_lg":18.0,"font_size_xl":22.0,
        "spacing_2xs":2.0,"spacing_xs":4.0,"spacing_sm":8.0,"spacing_md":12.0,"spacing_lg":16.0,"spacing_xl":20.0,"spacing_2xl":24.0,
        "radius_xs":3.0,"radius_sm":6.0,"radius_md":8.0,"radius_lg":12.0,"radius_xl":16.0,"radius_full":9999.0,
        "border_width_hairline":1.0,"border_width_threadline":2.0
    }"#;
    let deserialized: ThemeTokens = serde_json::from_str(old_json).expect("backward-compatible deserialize failed");
    assert_eq!(deserialized.titlebar_height, px(38.0));
    assert_eq!(deserialized.feed_header_height, px(44.0));
    assert_eq!(deserialized.bg_feed_post, gpui_kit::rgb(0x000000));
    assert_eq!(deserialized.line_height_body, 1.45);
}

#[test]
fn persisted_tokens_with_retired_spacing_steps_still_load() {
    // A pre-audit custom theme: it still carries the retired `spacing_px3` and `spacing_md_plus`
    // steps and has none of the semantic layout or dot size tokens.
    let old_json = r#"{
        "font_family_ui":"Segoe UI","bg_app":0,"text_primary":4109694463,
        "spacing_3xs":1.0,"spacing_2xs":2.0,"spacing_px3":3.0,"spacing_xs":4.0,"spacing_sm":8.0,
        "spacing_md":12.0,"spacing_lg":16.0,"spacing_xl":20.0,"spacing_2xl":24.0,
        "spacing_xs_plus":6.0,"spacing_sm_plus":10.0,"spacing_md_plus":14.0
    }"#;
    let loaded: ThemeTokens = serde_json::from_str(old_json).expect("old custom theme JSON must still load");
    let defaults = ThemeTokens::default();
    assert_eq!(loaded.spacing_md, px(12.0));
    for (name, value, default) in [
        ("panel_padding", loaded.panel_padding, defaults.panel_padding),
        ("panel_padding_compact", loaded.panel_padding_compact, defaults.panel_padding_compact),
        ("section_gap", loaded.section_gap, defaults.section_gap),
        ("stack_gap", loaded.stack_gap, defaults.stack_gap),
        ("inline_gap", loaded.inline_gap, defaults.inline_gap),
        ("card_padding", loaded.card_padding, defaults.card_padding),
        ("control_gap", loaded.control_gap, defaults.control_gap),
        ("dot_size_sm", loaded.dot_size_sm, defaults.dot_size_sm),
    ] {
        assert_eq!(value, default, "{name} falls back to the default theme");
        assert!(value > px(0.0), "{name} is set");
    }
    let saved = serde_json::to_string(&loaded).expect("serialize");
    assert!(!saved.contains("spacing_px3") && !saved.contains("spacing_md_plus"), "retired steps are dropped on save");
}
