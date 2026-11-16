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
