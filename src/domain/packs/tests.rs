//! Pack rules: keys, manifest parsing and validation, coverage, checksums, the downloadable
//! list and layer tinting.

use super::tint::{parse_hex, tint_rgb, tint_rgba, LayerRole, Tint, PRIMARY_KEY, SECONDARY_KEY};
use super::*;

const HASH: &str = "f163c1d3c6409fe1868690c1f3d9799352d3bf1dd5267c610a3877dffa0a47d4";

fn manifest_json(id: &str, key: &str, file: &str, sha: &str) -> String {
    format!(r#"{{"id":"{id}","name":"Test","version":"1.0.0","coverage":{{"{key}":{{"file":"{file}","sha256":"{sha}","width":320,"height":240}}}}}}"#)
}

#[test]
fn keys_are_classic_paths_without_root_and_extension() {
    assert_eq!(key_of_asset_path("illustrations/empty/no_posts.svg"), Some("empty/no_posts"));
    assert_eq!(key_of_asset_path("illustrations/forms/object/body/tin_can.svg"), Some("forms/object/body/tin_can"));
    assert_eq!(asset_path_of_key("guide/cheer"), "illustrations/guide/cheer.svg");
    for path in ["icons/x.svg", "illustrations/x.png", "illustrations/top.svg", "illustrations/../x/y.svg", "illustrations/A/b.svg"] {
        assert_eq!(key_of_asset_path(path), None, "{path}");
    }
    assert!(is_layered_key("portrait/face/round") && is_layered_key("forms/plant/body/oak") && !is_layered_key("body_form/bee"));
    assert!(is_new_art_key("journey/badge/first") && is_new_art_key("guides/feed") && !is_new_art_key("empty/no_posts"));
}

#[test]
fn ids_and_files_never_climb_out_of_a_pack() {
    assert!(is_safe_id("clay3d") && is_safe_id("my-pack_2"));
    for id in ["", "../x", "a/b", "Flat", "a.b", "a b"] {
        assert!(!is_safe_id(id), "{id:?}");
    }
    assert!(is_safe_file("empty/no_posts.webp") && is_safe_file("a.png"));
    for file in ["../x.webp", "/abs.webp", "a\\b.webp", "a/b.svg", "a/b", "a/.webp", "a//b.webp", "C:/x.webp"] {
        assert!(!is_safe_file(file), "{file:?}");
    }
}

#[test]
fn a_valid_manifest_parses_and_counts_its_coverage() {
    let manifest = PackManifest::parse(&manifest_json("sample", "spot/ideas", "spot/ideas.webp", HASH)).expect("valid");
    assert!(manifest.covers("spot/ideas") && !manifest.covers("spot/calm"));
    assert_eq!(manifest.covered_of(["spot/ideas", "spot/calm", "empty/no_posts"]), 1);
    assert_eq!(manifest.file("spot/ideas").map(|f| (f.width, f.height)), Some((320, 240)));
    assert_eq!(PackManifest::parse(&manifest.to_json()), Ok(manifest), "round trip");
    let bare = PackManifest::parse(r#"{"id":"empty","name":"Empty","version":"1"}"#).expect("optional fields default");
    assert!(bare.coverage.is_empty() && bare.samples.is_empty());
}

#[test]
fn invalid_manifests_are_refused_with_the_broken_rule() {
    let rule = |json: &str| match PackManifest::parse(json) {
        Err(PackError::Invalid(rule, _)) => Some(rule),
        _ => None,
    };
    assert_eq!(rule(&manifest_json("classic", "spot/ideas", "spot/ideas.webp", HASH)), Some(PackRule::Id));
    assert_eq!(rule(&manifest_json("../up", "spot/ideas", "spot/ideas.webp", HASH)), Some(PackRule::Id));
    assert_eq!(rule(&manifest_json("p", "../ideas", "spot/ideas.webp", HASH)), Some(PackRule::Key));
    assert_eq!(rule(&manifest_json("p", "spot/ideas", "../../evil.webp", HASH)), Some(PackRule::File));
    assert_eq!(rule(&manifest_json("p", "spot/ideas", "spot/ideas.webp", "abc")), Some(PackRule::Checksum));
    assert_eq!(rule(&manifest_json("p", "spot/ideas", "spot/ideas.webp", &HASH.to_uppercase())), Some(PackRule::Checksum));
    assert_eq!(rule(r#"{"id":"p","name":" ","version":"1"}"#), Some(PackRule::Name));
    assert_eq!(rule(r#"{"id":"p","name":"P","version":""}"#), Some(PackRule::Version));
    let huge = manifest_json("p", "spot/ideas", "spot/ideas.webp", HASH).replace("320", "99999");
    assert_eq!(rule(&huge), Some(PackRule::Size));
    assert!(matches!(PackManifest::parse("{not json"), Err(PackError::Manifest(_))));
    assert!(matches!(PackManifest::parse(r#"{"id":"p"}"#), Err(PackError::Manifest(_))), "name and version are required");
}

#[test]
fn checksums_are_hex_in_manifests_and_sri_for_downloads() {
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    let sri = sha256_integrity(&sha256_hex(b"abc")).expect("hex digest");
    assert_eq!(sri, "sha256-ungWv48Bz+pBQUDeXa4iI7ADYaOWF3qctBD/YfIAFa0=");
    assert!(crate::domain::icons::matches_integrity(b"abc", &sri));
    assert!(!crate::domain::icons::matches_integrity(b"abd", &sri));
    assert_eq!(sha256_integrity("nothex"), None);
}

#[test]
fn the_downloadable_list_ships_empty_and_keeps_only_safe_entries() {
    assert!(builtin_downloadable_packs().is_empty(), "packs are listed once a release is published");
    let entry = |id: &str, url: &str, sha: &str, bytes: u64| {
        format!(r#"{{"id":"{id}","name":"N","version":"1.0.0","url":"{url}","sha256":"{sha}","bytes":{bytes}}}"#)
    };
    let good = entry("clay3d", "https://example.invalid/clay3d.simpack", HASH, 1024);
    let list = format!(
        r#"{{"packs":[{good},{},{},{},{}]}}"#,
        entry("flat", "https://example.invalid/f.simpack", HASH, 1),
        entry("cozy", "http://example.invalid/c.simpack", HASH, 1),
        entry("iso", "https://example.invalid/i.simpack", "beef", 1),
        entry("big", "https://example.invalid/b.simpack", HASH, 0),
    );
    let packs = parse_downloadable(&list);
    assert_eq!(packs.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), ["clay3d"]);
    assert!(parse_downloadable("garbage").is_empty());
}

#[test]
fn built_in_packs_are_classic_and_flat_with_flat_the_default() {
    assert!(is_built_in(CLASSIC_PACK_ID) && is_built_in(FLAT_PACK_ID) && !is_built_in("clay3d"));
    assert_eq!(DEFAULT_PACK_ID, FLAT_PACK_ID);
}

#[test]
fn layer_roles_follow_the_layer_folder() {
    assert_eq!(LayerRole::of_key("portrait/face/round"), LayerRole::Skin);
    assert_eq!(LayerRole::of_key("portrait/hair/bob"), LayerRole::Hair);
    assert_eq!(LayerRole::of_key("portrait/brows/flat"), LayerRole::Hair);
    assert_eq!(LayerRole::of_key("forms/plant/body/oak"), LayerRole::Body);
    assert_eq!(LayerRole::of_key("forms/plant/markings/spots"), LayerRole::Markings);
    for key in ["portrait/eyes/round", "forms/shared/eyes/dot", "empty/no_posts", "forms/plant/aura/glow"] {
        assert_eq!(LayerRole::of_key(key), LayerRole::Fixed, "{key}");
    }
}

#[test]
fn key_colours_take_the_target_and_keep_their_shading() {
    let skin = parse_hex("#e0ac7e").expect("hex");
    let tint = Tint { primary: Some(skin), secondary: Some([0x20, 0x60, 0xd0]) };
    let close = |a: [u8; 3], b: [u8; 3]| a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2);
    assert!(close(tint_rgb(PRIMARY_KEY, &tint), skin), "the key grey becomes the target");
    let shadow = tint_rgb([0x40, 0x40, 0x40], &tint);
    let light = tint_rgb([0xc0, 0xc0, 0xc0], &tint);
    let sum = |c: [u8; 3]| c.iter().map(|v| u32::from(*v)).sum::<u32>();
    assert!(sum(shadow) < sum(skin) && sum(skin) < sum(light), "{shadow:?} {skin:?} {light:?}");
    assert!(shadow[0] > shadow[2], "a shadow keeps the target's warm hue");
    assert!(close(tint_rgb(SECONDARY_KEY, &tint), [0x20, 0x60, 0xd0]), "cyan becomes the secondary colour");
    assert_eq!(tint_rgb([255, 255, 255], &tint), [255, 255, 255], "eye whites stay white");
    assert_eq!(tint_rgb([0, 0, 0], &tint), [0, 0, 0], "ink stays black");
    assert_eq!(tint_rgb([0xe0, 0x30, 0x30], &tint), [0xe0, 0x30, 0x30], "other colours stay as drawn");
}

#[test]
fn tinting_a_buffer_skips_transparent_pixels_and_identity_tints() {
    let mut pixels = vec![0x80, 0x80, 0x80, 0, 0x80, 0x80, 0x80, 255];
    tint_rgba(&mut pixels, &Tint { primary: Some([10, 200, 30]), secondary: None });
    assert_eq!(&pixels[..4], &[0x80, 0x80, 0x80, 0], "fully transparent pixels are left alone");
    assert!(pixels[4].abs_diff(10) <= 2 && pixels[5].abs_diff(200) <= 2 && pixels[6].abs_diff(30) <= 2 && pixels[7] == 255, "{pixels:?}");
    let mut untouched = vec![0x80, 0x80, 0x80, 255];
    tint_rgba(&mut untouched, &Tint::default());
    assert_eq!(untouched, [0x80, 0x80, 0x80, 255]);
    assert_eq!(parse_hex("#12ab"), None);
    assert_eq!(parse_hex("12abef"), None);
}
