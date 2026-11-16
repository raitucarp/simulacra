use super::super::*;
use super::{default_colors, other_colors, parse};
use crate::domain::traits::MbtiType;
use crate::domain::TraitOption;
use gpui_kit::gpui::ImageFormat;
use std::sync::Arc;

#[test]
fn hits_misses_and_invalidation() {
    let mut cache = IllustrationCache::default();
    let (colors, other) = (default_colors(), other_colors());
    let path = MbtiType::Intp.illustration();
    let none = TemplateExtras::none();
    let first = cache.load(&path, &colors, &none);
    assert_eq!((cache.hits(), cache.misses()), (0, 1));
    assert_eq!(first.format, ImageFormat::Svg);
    let again = cache.load(&path, &colors, &none);
    assert!(Arc::ptr_eq(&first, &again), "a hit shares the image");
    assert_eq!((cache.hits(), cache.misses()), (1, 1));

    let themed = cache.load(&path, &other, &none);
    assert_ne!(themed.bytes, first.bytes, "another palette is another image");
    let sized = cache.load_sized(&path, &colors, &none, Some(512));
    assert_ne!(sized.id, first.id, "another raster width is another image");
    let extra = cache.load(&path, &colors, &TemplateExtras::none().with(template::SKIN_PLACEHOLDER, "#000000"));
    assert!(!Arc::ptr_eq(&extra, &first), "extras are part of the key");
    assert_eq!((cache.misses(), cache.len()), (4, 4));

    let evicted = cache.invalidate();
    assert_eq!(evicted.len(), 4);
    assert!(cache.is_empty());
    cache.load(&path, &colors, &none);
    assert_eq!(cache.misses(), 5, "invalidated entries are rebuilt");
}

#[test]
fn unknown_paths_render_the_neutral_fallback() {
    let mut cache = IllustrationCache::default();
    let colors = default_colors();
    let image = cache.load("illustrations/nope/missing.svg", &colors, &TemplateExtras::none());
    let svg = String::from_utf8(image.bytes.clone()).unwrap();
    assert_eq!(svg, fill_template(FALLBACK_TEMPLATE, &colors, &TemplateExtras::none()));
    parse(&svg, "fallback");
    let generated = cache.load_with("gen/none", || None, &colors, &TemplateExtras::none(), None);
    assert_eq!(generated.bytes, image.bytes);
}

#[test]
fn forget_source_drops_only_matching_entries() {
    let mut cache = IllustrationCache::default();
    let colors = default_colors();
    let none = TemplateExtras::none();
    cache.load_with("photo/a|1", || Some(FALLBACK_TEMPLATE.into()), &colors, &none, None);
    cache.load_with("photo/a|2", || Some(FALLBACK_TEMPLATE.into()), &colors, &none, Some(64));
    cache.load_with("photo/b|1", || Some(FALLBACK_TEMPLATE.into()), &colors, &none, None);
    assert_eq!(cache.forget_source("photo/a|").len(), 2);
    assert_eq!(cache.len(), 1);
}

#[test]
fn palette_hash_tracks_colors() {
    assert_eq!(palette_hash(&default_colors()), palette_hash(&default_colors()));
    assert_ne!(palette_hash(&default_colors()), palette_hash(&other_colors()));
}

#[test]
fn the_active_cache_is_invalidated_by_a_new_palette_only() {
    let (colors, other) = (default_colors(), other_colors());
    let _ = set_active_palette(&colors);
    let image = Illustration::guide(GuidePose::Cheer).image();
    assert!(Arc::ptr_eq(&image, &Illustration::guide(GuidePose::Cheer).image()));
    assert!(set_active_palette(&colors).is_empty(), "same palette keeps the cache");
    let evicted = set_active_palette(&other);
    assert!(evicted.iter().any(|e| Arc::ptr_eq(e, &image)), "theme change evicts");
    assert_eq!(active_palette(), other);
    assert!(!Arc::ptr_eq(&image, &Illustration::guide(GuidePose::Cheer).image()));
    let _ = set_active_palette(&colors);
}

#[test]
fn illustration_sizes_follow_the_canvas_aspect() {
    let card = Illustration::trait_card(MbtiType::Enfp);
    assert_eq!(card.display_size(), (gpui_kit::px(160.0), gpui_kit::px(120.0)));
    let banner = Illustration::step(StepKey::Mind).height(gpui_kit::px(60.0));
    assert_eq!(banner.display_size(), (gpui_kit::px(160.0), gpui_kit::px(60.0)));
    let class = Illustration::class(ClassKey::StoicSage).width(gpui_kit::px(100.0));
    assert_eq!(class.display_size().1, gpui_kit::px(120.0));
    let empty = Illustration::empty(EmptyKey::NoResults);
    assert!(String::from_utf8_lossy(&empty.image().bytes).contains(r#"width="256""#));
}
