use super::*;
use gpui_kit::{rgb, rgba};

#[test]
fn contrast_of_black_on_white_is_21() {
    let ratio = contrast_ratio(BLACK, WHITE);
    assert!((ratio - 21.0).abs() < 0.01, "got {ratio}");
    assert!((contrast_ratio(WHITE, WHITE) - 1.0).abs() < f32::EPSILON);
}

#[test]
fn known_contrast_pair_matches_reference() {
    // #767676 on white is the classic 4.54:1 AA boundary color.
    let ratio = contrast_ratio(rgb(0x767676), WHITE);
    assert!((ratio - 4.54).abs() < 0.02, "got {ratio}");
}

#[test]
fn mix_interpolates_and_clamps() {
    assert_eq!(mix(BLACK, WHITE, 0.0), BLACK);
    assert_eq!(mix(BLACK, WHITE, 1.0), WHITE);
    assert_eq!(mix(BLACK, WHITE, 2.0), WHITE);
    let mid = mix(BLACK, WHITE, 0.5);
    assert!((mid.r - 0.5).abs() < f32::EPSILON);
}

#[test]
fn hex_roundtrips_opaque_and_translucent() {
    assert_eq!(to_hex(rgb(0x8b5cf6)), "#8b5cf6");
    assert_eq!(to_hex(rgba(0x8b5cf620)), "#8b5cf620");
    assert_eq!(parse_hex("#8b5cf6"), Some(rgb(0x8b5cf6)));
    assert_eq!(parse_hex("8b5cf6"), None);
    assert_eq!(parse_hex("#zzzzzz"), None);
}

#[test]
fn ensure_contrast_moves_only_when_needed() {
    let bg = rgb(0x000000);
    let readable = rgb(0xf4f4f5);
    assert_eq!(ensure_contrast(readable, WHITE, &[bg], 4.5), readable);
    let dim = rgb(0x333333);
    let fixed = ensure_contrast(dim, WHITE, &[bg], 4.5);
    assert!(contrast_ratio(fixed, bg) >= 4.5);
}

#[test]
fn composite_blends_alpha_over_opaque() {
    let over = with_alpha(WHITE, 0.5);
    let out = composite(over, BLACK);
    assert!((out.r - 0.5).abs() < f32::EPSILON);
    assert!((out.a - 1.0).abs() < f32::EPSILON);
}

#[test]
fn readable_on_prefers_light_when_it_passes() {
    assert_eq!(readable_on(rgb(0x0969da), WHITE, BLACK, 3.0), WHITE);
    assert_eq!(readable_on(rgb(0xfe8019), WHITE, BLACK, 3.0), BLACK);
}
