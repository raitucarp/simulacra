use super::LayoutClass;
use crate::theme::ThemeTokens;
use gpui_kit::px;

#[test]
fn widths_map_to_classes_at_the_token_breakpoints() {
    let theme = ThemeTokens::default();
    let medium = theme.layout_medium_min_width;
    let wide = theme.layout_wide_min_width;
    assert_eq!(LayoutClass::for_width(px(0.0), &theme), LayoutClass::Compact);
    assert_eq!(LayoutClass::for_width(medium - px(1.0), &theme), LayoutClass::Compact);
    assert_eq!(LayoutClass::for_width(medium, &theme), LayoutClass::Medium);
    assert_eq!(LayoutClass::for_width(wide - px(1.0), &theme), LayoutClass::Medium);
    assert_eq!(LayoutClass::for_width(wide, &theme), LayoutClass::Wide);
}

#[test]
fn the_audited_window_sizes_land_in_the_expected_classes() {
    let theme = ThemeTokens::default();
    let class = |width: f32| LayoutClass::for_width(px(width), &theme);
    assert_eq!(class(720.0), LayoutClass::Compact);
    assert_eq!(class(1024.0), LayoutClass::Medium);
    assert_eq!(class(1440.0), LayoutClass::Wide);
    assert!(theme.window_min_width <= px(720.0), "the smallest audited size is allowed");
}

#[test]
fn classes_only_grow_with_the_width_and_the_sidebar_narrows_below_wide() {
    let theme = ThemeTokens::default();
    let classes: Vec<_> = (0..40).map(|step| LayoutClass::for_width(px(step as f32 * 50.0), &theme)).collect();
    assert!(classes.windows(2).all(|pair| pair[0] <= pair[1]), "{classes:?}");
    let (wide, ..) = LayoutClass::Wide.right_sidebar_size(&theme);
    let (medium, min, max) = LayoutClass::Medium.right_sidebar_size(&theme);
    assert!(medium < wide && min <= medium && medium <= max);
    assert!(LayoutClass::Compact.is_compact() && !LayoutClass::Medium.is_compact());
}
