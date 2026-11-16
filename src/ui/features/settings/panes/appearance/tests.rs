use super::contrast::{contrast_checks, ContrastGrade};
use super::filter::{is_visible, matches_query, ModeFilter};
use crate::theme::derive::derive_default_tokens;
use crate::theme::{PaletteMode, ThemeRegistry};
use gpui_kit::rgb;

#[test]
fn search_matches_name_id_and_credit_case_insensitively() {
    let registry = ThemeRegistry::global();
    let dracula = registry.get("dracula").expect("dracula");
    assert!(matches_query(dracula, ""));
    assert!(matches_query(dracula, "DRAC"));
    assert!(matches_query(dracula, "zeno rocha"));
    assert!(!matches_query(dracula, "solarized"));
}

#[test]
fn mode_filter_partitions_the_gallery() {
    let all = ThemeRegistry::global().all();
    let count = |filter| all.iter().filter(|p| is_visible(p, "", filter)).count();
    assert_eq!(count(ModeFilter::All), all.len());
    assert_eq!(count(ModeFilter::Dark) + count(ModeFilter::Light), all.len());
    assert!(all.iter().filter(|p| is_visible(p, "", ModeFilter::Light)).all(|p| p.mode == PaletteMode::Light));
    assert_eq!(count(ModeFilter::Light), ThemeRegistry::global().by_mode(PaletteMode::Light).count());
}

#[test]
fn contrast_grades_follow_wcag_thresholds() {
    assert_eq!(ContrastGrade::of(7.1, 4.5, 7.0), ContrastGrade::Aaa);
    assert_eq!(ContrastGrade::of(4.5, 4.5, 7.0), ContrastGrade::Aa);
    assert_eq!(ContrastGrade::of(4.49, 4.5, 7.0), ContrastGrade::Fail);
}

#[test]
fn every_bundled_palette_passes_every_badge() {
    for palette in ThemeRegistry::global().all() {
        let checks = contrast_checks(palette, &derive_default_tokens(palette));
        assert_eq!(checks.len(), 7, "fg & fg_muted on three surfaces + accent label");
        for check in checks {
            assert_ne!(check.grade, ContrastGrade::Fail, "{}: {} on {}", palette.id, check.fg_role, check.bg_role);
        }
    }
}

#[test]
fn a_bad_custom_color_fails_its_badge() {
    let mut palette = ThemeRegistry::global().default_palette().clone();
    palette.colors.fg_muted = rgb(0x202020);
    let checks = contrast_checks(&palette, &derive_default_tokens(&palette));
    assert!(checks.iter().any(|c| c.fg_role == "appearance.role_fg_muted" && c.grade == ContrastGrade::Fail));
}
