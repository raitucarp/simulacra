use super::super::template::*;
use super::{default_colors, parse};
use crate::domain::portrait::{HairColor, SkinTone};
use crate::theme::color::to_hex;
use crate::ui::assets::{template_text, EmbeddedAssets};

#[test]
fn every_embedded_template_fills_completely_and_parses() {
    let colors = default_colors();
    let extras = TemplateExtras::portrait(SkinTone::Honey, HairColor::Auburn);
    let mut count = 0;
    for path in EmbeddedAssets::iter() {
        let template = template_text(&path).expect("utf-8 template");
        let filled = fill_template(&template, &colors, &extras);
        assert!(!filled.contains(PLACEHOLDER_OPEN), "{path}: placeholder left");
        assert!(!filled.contains(PLACEHOLDER_CLOSE), "{path}: placeholder left");
        parse(&filled, &path);
        count += 1;
    }
    assert!(count > 200, "only {count} templates embedded");
}

#[test]
fn palette_extras_and_unknown_tokens_are_replaced() {
    let colors = default_colors();
    let extras = TemplateExtras::none().with(SKIN_PLACEHOLDER, "#123456");
    let out = fill_template("a{{accent}}b{{ skin }}c{{mystery}}d{{accent_2}}", &colors, &extras);
    let fallback = to_hex(FALLBACK_COLOR_KEY.get(&colors));
    let expected = format!("a{}b#123456c{fallback}d{}", to_hex(colors.accent), to_hex(colors.accent_2));
    assert_eq!(out, expected);
    let broken = fill_template("x{{accent", &colors, &extras);
    assert!(!broken.contains(PLACEHOLDER_OPEN), "{broken}");
    assert_eq!(fill_template("no tokens", &colors, &extras), "no tokens");
}

#[test]
fn extras_are_order_independent_and_replaceable() {
    let a = TemplateExtras::none().with(HAIR_PLACEHOLDER, "#111111").with(SKIN_PLACEHOLDER, "#222222");
    let b = TemplateExtras::none().with(SKIN_PLACEHOLDER, "#222222").with(HAIR_PLACEHOLDER, "#111111");
    assert_eq!(a, b);
    let c = a.clone().with(HAIR_PLACEHOLDER, "#333333");
    assert_eq!(c.get(HAIR_PLACEHOLDER), Some("#333333"));
    assert_ne!(a, c);
    assert_eq!(TemplateExtras::portrait(SkinTone::Ebony, HairColor::Jet).get(SKIN_PLACEHOLDER), Some(SkinTone::Ebony.hex()));
}

#[test]
fn raster_width_sets_the_intrinsic_size() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 120"><rect width="160" height="120"/></svg>"#;
    assert_eq!(view_box_size(svg), Some((160.0, 120.0)));
    assert_eq!(with_raster_width(svg, None), svg);
    let sized = with_raster_width(svg, Some(320));
    let tree = parse(&sized, "sized");
    assert_eq!((tree.size().width(), tree.size().height()), (320.0, 240.0));
    assert_eq!(with_raster_width("<svg/>", Some(64)), "<svg/>", "no viewBox: unchanged");
}

#[test]
fn raster_widths_snap_up_to_buckets() {
    assert_eq!(raster_width(1.0), 32);
    assert_eq!(raster_width(32.0), 32);
    assert_eq!(raster_width(40.0), 64);
    assert_eq!(raster_width(160.0), 256);
    assert_eq!(raster_width(5000.0), 1024);
    assert_eq!(raster_width(f32::NAN), 1024);
}
