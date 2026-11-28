//! WS5 loader: the active pack's WebP draws a key it covers, Classic draws everything else
//! (missing keys, unknown packs, damaged files), a theme change keeps raster images, a pack
//! switch evicts them, and composed avatars use raster layers only when all are covered.
//! The sample pack is the processed fixture in `tests/fixtures/packs/sample`.

use super::super::pack::*;
use super::super::*;
use super::{default_colors, other_colors};
use crate::domain::packs::tint::Tint;
use gpui_kit::gpui::ImageFormat;
use gpui_kit::px;
use std::path::PathBuf;
use std::sync::Arc;

fn sample_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/packs/sample")
}

/// Activates the sample pack on this test thread (each test thread has its own registry).
fn use_sample() -> Arc<RasterPack> {
    let pack = Arc::new(RasterPack::from_dir(&sample_dir()).expect("the sample pack loads"));
    let _ = set_available_packs(vec![pack.clone()]);
    let _ = set_active_pack("sample");
    pack
}

fn decoded(image: &gpui_kit::gpui::Image) -> image::RgbaImage {
    image::load_from_memory(&image.bytes).expect("decodable").to_rgba8()
}

#[test]
fn without_a_pack_every_key_draws_classic() {
    let _ = set_available_packs(Vec::new());
    let _ = set_active_pack("flat");
    assert_eq!(active_pack_id(), "classic", "an unavailable pack falls back to Classic");
    assert_eq!(Illustration::spot(SpotKey::Ideas).image().format, ImageFormat::Svg);
}

#[test]
fn a_covered_key_draws_the_pack_webp_and_a_missing_key_draws_classic() {
    use_sample();
    let ideas = Illustration::spot(SpotKey::Ideas).image();
    assert_eq!(ideas.format, ImageFormat::Webp, "displayed at its canvas size the file is used as is");
    assert_eq!(decoded(&ideas).dimensions(), (320, 240));
    assert!(Arc::ptr_eq(&ideas, &Illustration::spot(SpotKey::Ideas).image()), "cached");
    assert_eq!(Illustration::spot(SpotKey::Calm).image().format, ImageFormat::Svg, "uncovered keys stay Classic");
    let small = Illustration::spot(SpotKey::Ideas).width(px(32.0)).image();
    assert_eq!((small.format, decoded(&small).dimensions()), (ImageFormat::Png, (64, 48)), "small displays get a downscaled copy");
}

#[test]
fn a_preview_can_name_another_pack_and_unknown_packs_draw_classic() {
    use_sample();
    let _ = set_active_pack("classic");
    assert_eq!(Illustration::spot(SpotKey::Ideas).image().format, ImageFormat::Svg);
    assert_eq!(Illustration::spot(SpotKey::Ideas).pack("sample").image().format, ImageFormat::Webp);
    assert_eq!(Illustration::spot(SpotKey::Ideas).pack("nope").image().format, ImageFormat::Svg);
}

#[test]
fn a_theme_change_keeps_rasters_and_a_pack_switch_evicts_them() {
    use_sample();
    let _ = set_active_palette(&default_colors());
    let raster = Illustration::spot(SpotKey::Ideas).image();
    let evicted = set_active_palette(&other_colors());
    assert!(!evicted.iter().any(|image| Arc::ptr_eq(image, &raster)), "rasters do not depend on the palette");
    assert!(Arc::ptr_eq(&raster, &Illustration::spot(SpotKey::Ideas).image()));
    assert!(set_active_pack("sample").is_empty(), "the same pack keeps its images");
    let stale = set_active_pack("classic");
    assert!(stale.iter().any(|image| Arc::ptr_eq(image, &raster)), "switching packs hands the old images back");
    assert_eq!(raster_cache_len(), 0);
    let _ = set_active_palette(&default_colors());
}

#[test]
fn a_damaged_file_draws_classic() {
    let dir = std::env::temp_dir().join(format!("simulacra-pack-damaged-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("spot")).unwrap();
    std::fs::copy(sample_dir().join("pack.json"), dir.join("pack.json")).unwrap();
    std::fs::write(dir.join("spot").join("ideas.webp"), b"edited").unwrap();
    let pack = Arc::new(RasterPack::from_dir(&dir).expect("manifest"));
    let _ = set_available_packs(vec![pack]);
    let _ = set_active_pack("sample");
    assert_eq!(Illustration::spot(SpotKey::Ideas).image().format, ImageFormat::Svg, "a checksum mismatch never draws");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn coverage_counts_classic_keys() {
    let pack = use_sample();
    let (covered, total) = coverage(Some(pack.as_ref()));
    assert_eq!(covered, 2);
    assert_eq!(total, classic_keys().len());
    assert!(total > 800, "{total}");
    assert_eq!(coverage(None), (total, total), "Classic covers everything");
    let flat = RasterPack::flat().expect("the built-in Flat manifest is valid");
    assert!(flat.is_built_in() && flat.id() == "flat");
    let kinds = [("empty/no_posts", IllustrationKind::Empty), ("guide/cheer", IllustrationKind::Guide), ("vault/subject_index", IllustrationKind::Step), ("palace/hall", IllustrationKind::TraitCard)];
    for (key, kind) in kinds {
        assert_eq!(IllustrationKind::of_key(key), kind, "{key}");
    }
}

#[test]
fn raster_layers_compose_only_when_every_drawable_layer_is_covered() {
    let pack = use_sample();
    let face = |tint: Tint| LayerPlan { path: "illustrations/portrait/face/round.svg".into(), tint, matrix: None, clip: false };
    let blank = LayerPlan { path: "illustrations/portrait/accessory/none.svg".into(), tint: Tint::default(), matrix: None, clip: false };
    let missing = LayerPlan { path: "illustrations/portrait/hair/bob.svg".into(), tint: Tint::default(), matrix: None, clip: false };
    assert!(is_blank_layer(&blank.path));
    assert!(covers_all(&pack, &[face(Tint::default()), blank.clone()]), "a blank layer needs no file");
    assert!(!covers_all(&pack, &[face(Tint::default()), missing]), "a missing drawable layer keeps the avatar Classic");
    let skin = Tint { primary: Some([0xe0, 0xac, 0x7e]), secondary: None };
    let grey = decoded(&compose(&pack, &[face(Tint::default()), blank.clone()], 256).expect("composed"));
    let tinted = decoded(&compose(&pack, &[face(skin), blank], 256).expect("composed"));
    assert_eq!(grey.dimensions(), (512, 512));
    let (g, t) = (grey.get_pixel(256, 256).0, tinted.get_pixel(256, 256).0);
    assert!(g[0].abs_diff(g[2]) < 8, "the key layer is grey: {g:?}");
    assert!(t[0] > t[2] + 40, "the skin tint is warm: {t:?}");
    let spec = crate::domain::portrait::PortraitSpec::random(3);
    assert_eq!(portrait::portrait_image(&spec, Some(128)).format, ImageFormat::Svg, "the sample pack lacks most layers");
}

#[test]
fn reference_svgs_fill_every_placeholder_with_key_colours_for_layers() {
    for key in classic_keys() {
        let svg = reference_svg(&crate::domain::packs::asset_path_of_key(key)).unwrap_or_else(|| panic!("{key}"));
        assert!(!svg.contains("{{"), "{key}");
    }
    let face = reference_svg("illustrations/portrait/face/round.svg").expect("face");
    assert!(face.contains("#808080"), "skin is drawn in the grey key");
}
