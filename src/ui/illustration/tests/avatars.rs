use super::super::photo::{is_croppable, photo_source, photo_template};
use super::super::portrait::*;
use super::super::*;
use super::{default_colors, parse};
use crate::domain::portrait::{AvatarMode, EmblemSpec, HairStyle, PhotoCrop, PortraitLayer, PortraitSpec};
use resvg::tiny_skia::{Pixmap, Transform};
use std::path::Path;

/// Number of portraits composed and parsed by the composition test.
const SAMPLE_PORTRAITS: u64 = 48;

fn render(svg: &str, size: u32) -> Pixmap {
    let tree = parse(svg, "render");
    let mut pixmap = Pixmap::new(size, size).unwrap();
    let scale = size as f32 / tree.size().width();
    resvg::render(&tree, Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    pixmap
}

#[test]
fn layer_bodies_are_extracted_and_stacked_in_order() {
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256" fill="none"><circle r="1"/></svg>"#;
    assert_eq!(layer_body(svg), Some(r#"<circle r="1"/>"#));
    assert_eq!(layer_body(r#"<svg viewBox="0 0 1 1"></svg>"#), Some(""));
    assert_eq!(layer_body("not svg"), None);
    let composed = compose_layers(["<a/>", "<b/>"]);
    assert!(composed.find("<a/>").unwrap() < composed.find("<b/>").unwrap());
    parse(&composed.replace("<a/>", "").replace("<b/>", ""), "empty stack");
}

#[test]
fn composed_portraits_fill_and_parse() {
    let colors = default_colors();
    for seed in 0..SAMPLE_PORTRAITS {
        let spec = PortraitSpec::random(seed);
        let template = compose_portrait_template(&spec);
        let groups = template.matches(r#"<g fill="none" stroke-linecap"#).count();
        assert_eq!(groups, PortraitLayer::STACK.len(), "seed {seed}");
        let filled = fill_template(&template, &colors, &TemplateExtras::portrait(spec.skin, spec.hair_color));
        assert!(!filled.contains("{{"), "seed {seed}");
        assert!(filled.contains(spec.skin.hex()), "seed {seed}: skin applied");
        parse(&filled, &format!("portrait seed {seed}"));
    }
}

#[test]
fn portrait_sources_ignore_seed_and_colors() {
    let spec = PortraitSpec::random(3);
    let same_look = PortraitSpec { seed: 99, ..spec };
    assert_eq!(portrait_source(&spec), portrait_source(&same_look));
    let other = PortraitSpec { hair: HairStyle::Bun, ..spec };
    assert_eq!(portrait_source(&other) == portrait_source(&spec), other.hair == spec.hair);
    let image = portrait_image(&spec, Some(64));
    assert!(String::from_utf8_lossy(&image.bytes).contains(r#"width="64""#));
}

#[test]
fn photos_are_clipped_to_a_circle() {
    let dir = std::env::temp_dir().join(format!("simulacra_photo_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let photo = dir.join("face & co.png");
    let emblem = fill_template(&EmblemSpec::default().to_svg_template(), &default_colors(), &TemplateExtras::none());
    render(&emblem, 64).save_png(&photo).unwrap();
    assert!(is_croppable(&photo) && !is_croppable(Path::new("a.bmp")));

    let svg = photo_template(&photo, PhotoCrop::new(2000, 250, 0), 1);
    assert!(svg.contains("&amp;"), "path is XML-escaped");
    let pixmap = render(&svg, 100);
    assert_eq!(pixmap.pixel(1, 1).unwrap().alpha(), 0, "corner outside the circle is clear");
    assert_eq!(pixmap.pixel(50, 50).unwrap().alpha(), u8::MAX, "centre shows the photo");
    assert_ne!(photo_source(&photo, PhotoCrop::default()), photo_source(&photo, PhotoCrop::new(2000, 0, 0)));

    let stored = photo.to_string_lossy().to_string();
    let first = avatar_source(&AvatarMode::photo(stored.clone()), gpui_kit::px(40.0));
    assert!(first.is_some(), "an absolute stored path resolves");
    assert!(!forget_stored_photo(&stored).is_empty(), "forgetting drops the cached framing");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn avatar_sources_cover_every_mode() {
    let size = gpui_kit::px(40.0);
    assert!(avatar_source(&AvatarMode::Pcg(PortraitSpec::random(1)), size).is_some());
    assert!(avatar_source(&AvatarMode::Emblem(EmblemSpec::default()), size).is_some());
    assert!(avatar_source(&AvatarMode::photo("avatars/definitely-missing.png"), size).is_none());
}
