use super::*;
use crate::domain::traits::{ClassicTemperament, EnneagramType, MbtiType};
use crate::theme::PaletteColorKey;
use std::collections::HashSet;
use std::path::Path;

const ASSET_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
const SAMPLE_FILL: &str = "#808080";
const SEEDS: u64 = 256;

fn asset_exists(path: &str) -> bool {
    Path::new(ASSET_ROOT).join(path).is_file()
}

#[test]
fn every_part_variant_has_an_asset_and_every_asset_a_variant() {
    for layer in PortraitLayer::STACK {
        let keys = layer_variant_keys(layer);
        for key in &keys {
            assert!(asset_exists(&layer.asset_path(key)), "missing {}", layer.asset_path(key));
        }
        let dir = Path::new(ASSET_ROOT).join(PORTRAIT_ASSET_ROOT).join(layer.key());
        for entry in std::fs::read_dir(&dir).expect("layer dir").flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let stem = name.trim_end_matches(".svg");
            assert!(keys.contains(&stem), "{}/{name} has no typed variant", layer.key());
        }
    }
}

#[test]
fn part_serde_names_are_the_asset_keys() {
    for hair in HairStyle::ALL {
        assert_eq!(serde_json::to_string(hair).unwrap(), format!("\"{}\"", hair.key()));
    }
    for outfit in OutfitStyle::ALL {
        assert_eq!(serde_json::to_string(outfit).unwrap(), format!("\"{}\"", outfit.key()));
    }
    assert_eq!(serde_json::to_string(&HairColor::Platinum).unwrap(), "\"platinum\"");
}

#[test]
fn random_specs_are_deterministic_and_varied() {
    assert_eq!(PortraitSpec::random(42), PortraitSpec::random(42));
    assert_eq!(PortraitSpec::random(7).seed, 7);
    let specs: Vec<_> = (0..SEEDS).map(PortraitSpec::random).collect();
    let distinct: HashSet<_> = specs.iter().map(|s| (s.hair, s.face, s.skin, s.eyes, s.mouth)).collect();
    assert!(distinct.len() > SEEDS as usize / 2, "only {} distinct looks", distinct.len());
    let hairs: HashSet<_> = specs.iter().map(|s| s.hair).collect();
    let skins: HashSet<_> = specs.iter().map(|s| s.skin).collect();
    assert_eq!(hairs.len(), HairStyle::ALL.len(), "every hairstyle is reachable");
    assert_eq!(skins.len(), SkinTone::ALL.len(), "every skin tone is reachable");
    let bare = specs.iter().filter(|s| s.accessory == Accessory::None).count();
    assert!(bare > SEEDS as usize / 4 && bare < SEEDS as usize * 3 / 4, "{bare} bare of {SEEDS}");
}

#[test]
fn randomize_part_only_touches_its_slot() {
    let base = PortraitSpec::random(1);
    let mut rng = PortraitRng::new(99);
    for slot in PortraitSlot::ALL {
        let mut rolled = base;
        for _ in 0..32 {
            rolled.randomize_part(slot, &mut rng);
        }
        let mut restored = rolled;
        match slot {
            PortraitSlot::Back => restored.back = base.back,
            PortraitSlot::Face => restored.face = base.face,
            PortraitSlot::Skin => restored.skin = base.skin,
            PortraitSlot::Eyes => restored.eyes = base.eyes,
            PortraitSlot::Brows => restored.brows = base.brows,
            PortraitSlot::Mouth => restored.mouth = base.mouth,
            PortraitSlot::Hair => restored.hair = base.hair,
            PortraitSlot::HairColor => restored.hair_color = base.hair_color,
            PortraitSlot::Outfit => restored.outfit = base.outfit,
            PortraitSlot::Accessory => restored.accessory = base.accessory,
        }
        assert_eq!(restored, base, "{slot:?} leaked into other slots");
    }
}

#[test]
fn layer_paths_follow_the_stack_and_exist() {
    let spec = PortraitSpec::random(5);
    let paths = spec.layer_paths();
    assert_eq!(paths.len(), PortraitLayer::STACK.len());
    assert!(paths[0].contains("/back/") && paths[7].contains("/accessory/"));
    assert!(paths.iter().all(|p| asset_exists(p)), "{paths:?}");
}

#[test]
fn slot_keys_and_label_keys_are_stable() {
    let keys: HashSet<_> = PortraitSlot::ALL.iter().map(|s| s.key()).collect();
    assert_eq!(keys.len(), PortraitSlot::ALL.len());
    assert_eq!(PortraitSlot::HairColor.label_key(), "portrait.hair_color");
    assert_eq!(PortraitSlot::Hair.option_label_key(HairStyle::SidePart.key()), "portrait.hair_side_part");
}

#[test]
fn spec_serde_round_trips_and_tolerates_partial_json() {
    let spec = PortraitSpec::random(123);
    let json = serde_json::to_string(&spec).unwrap();
    assert_eq!(serde_json::from_str::<PortraitSpec>(&json).unwrap(), spec);
    let partial: PortraitSpec = serde_json::from_str(r#"{"hair":"bob","skin":"ebony"}"#).unwrap();
    assert_eq!((partial.hair, partial.skin, partial.face), (HairStyle::Bob, SkinTone::Ebony, FaceShape::Oval));
}

#[test]
fn avatar_mode_serde_round_trips_every_mode() {
    let modes = [
        AvatarMode::Pcg(PortraitSpec::random(9)),
        AvatarMode::Emblem(EmblemSpec::default()),
        AvatarMode::Photo { path: "avatars/p1.png".into(), crop: PhotoCrop::new(2000, 100, -100) },
    ];
    for mode in modes {
        let json = serde_json::to_string(&mode).unwrap();
        assert_eq!(serde_json::from_str::<AvatarMode>(&json).unwrap(), mode, "{json}");
    }
    let photo: AvatarMode = serde_json::from_str(r#"{"mode":"photo","path":"avatars/x.jpg"}"#).unwrap();
    assert_eq!(photo, AvatarMode::photo("avatars/x.jpg"));
}

#[test]
fn outfits_follow_the_occupation() {
    assert_eq!(suggested_outfit("Research Scientist"), Some(OutfitStyle::LabCoat));
    assert_eq!(suggested_outfit("philosophy professor"), Some(OutfitStyle::ScholarGown));
    assert_eq!(suggested_outfit("Stoic philosopher"), Some(OutfitStyle::Toga));
    assert_eq!(suggested_outfit("Software engineer"), Some(OutfitStyle::Hoodie));
    assert_eq!(suggested_outfit("Penulis lepas"), Some(OutfitStyle::Cardigan));
    assert_eq!(suggested_outfit("Astronaut"), None);
}

#[test]
fn photo_crop_clamps_and_frames() {
    let crop = PhotoCrop::new(9000, 30000, -30000);
    assert_eq!(crop.zoom, ZOOM_MAX);
    assert_eq!((crop.offset_x, crop.offset_y), (1500, -1500));
    assert_eq!(PhotoCrop::new(1000, 400, 0).offset_x, 0, "1x zoom cannot pan");
    let frame = PhotoCrop::default().frame(100.0);
    assert_eq!((frame.x, frame.y, frame.side), (0.0, 0.0, 100.0));
    let zoomed = PhotoCrop::default().zoomed(10).panned(500, 0);
    let frame = zoomed.frame(100.0);
    assert_eq!(frame.side, 200.0);
    assert_eq!(frame.x, 0.0, "panned to the left edge of the photo");
    assert!(frame.x <= 0.0 && frame.x + frame.side >= 100.0, "circle stays covered");
    assert_eq!(zoomed.zoomed(-100).zoom, ZOOM_MIN);
}

#[test]
fn skin_and_hair_palettes_are_distinct_hex_colors() {
    let hexes: Vec<&str> = SkinTone::ALL.iter().map(|s| s.hex()).chain(HairColor::ALL.iter().map(|h| h.hex())).collect();
    assert!(SkinTone::ALL.len() >= 8 && HairColor::ALL.len() >= 10);
    for hex in &hexes {
        assert!(hex.len() == 7 && hex.starts_with('#') && hex[1..].chars().all(|c| c.is_ascii_hexdigit()), "{hex}");
    }
    assert_eq!(hexes.iter().collect::<HashSet<_>>().len(), hexes.len());
}

fn filled(template: &str) -> String {
    let mut out = template.to_string();
    for key in PaletteColorKey::ALL {
        out = out.replace(&format!("{{{{{}}}}}", key.id()), SAMPLE_FILL);
    }
    out
}

#[test]
fn emblems_are_deterministic_distinct_and_parse() {
    let mut glyphs = HashSet::new();
    let mut svgs = HashSet::new();
    for mbti in MbtiType::ALL.iter() {
        for temperament in ClassicTemperament::ALL.iter() {
            for enneagram in EnneagramType::ALL.iter() {
                let spec = EmblemSpec::from_traits(*mbti, *enneagram, *temperament);
                assert_eq!(spec, EmblemSpec::from_traits(*mbti, *enneagram, *temperament));
                glyphs.insert(spec.glyph);
                svgs.insert(spec.to_svg_template());
            }
        }
    }
    assert_eq!(glyphs.len(), MbtiType::ALL.len(), "one glyph per MBTI type");
    let combos = MbtiType::ALL.len() * ClassicTemperament::ALL.len() * EnneagramType::ALL.len();
    assert_eq!(svgs.len(), combos, "every personality draws a different emblem");
    for svg in svgs.iter().take(64) {
        let svg = filled(svg);
        assert!(!svg.contains("{{"), "unfilled placeholder in {svg}");
        resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default()).expect("emblem parses");
    }
}

#[test]
fn emblem_traits_map_as_documented() {
    let spec = EmblemSpec::from_traits(MbtiType::Entj, EnneagramType::Type8w7, ClassicTemperament::Choleric);
    assert_eq!(spec.glyph, EmblemGlyph { outward: true, intuitive: true, thinking: true, judging: true });
    assert_eq!((spec.shape, spec.wing), (EmblemShape::Triangle, 7));
    assert_eq!((spec.primary, spec.secondary), (EmblemHue::Danger, EmblemHue::Warning));
    assert!(!EmblemSpec::from_traits(MbtiType::Infp, EnneagramType::Type4w5, ClassicTemperament::Melancholic)
        .to_svg_template()
        .is_empty());
    let ids: HashSet<_> = PaletteColorKey::ALL.iter().map(|k| k.id()).collect();
    let hues = [EmblemHue::Accent, EmblemHue::Accent2, EmblemHue::Agent, EmblemHue::Success, EmblemHue::Warning, EmblemHue::Danger, EmblemHue::Info];
    assert!(hues.iter().all(|h| ids.contains(h.palette_key())));
    let json = serde_json::to_string(&spec).unwrap();
    assert_eq!(serde_json::from_str::<EmblemSpec>(&json).unwrap(), spec);
}

#[test]
fn every_slot_and_option_has_en_and_id_labels() {
    let mut missing = Vec::new();
    for slot in PortraitSlot::ALL {
        let keys = std::iter::once(slot.label_key())
            .chain(slot_option_keys(slot).into_iter().map(|key| slot.option_label_key(key)));
        for key in keys {
            for locale in ["en", "id"] {
                if rust_i18n::t!(&key, locale = locale) == key.as_str() {
                    missing.push(format!("{locale}: {key}"));
                }
            }
        }
    }
    assert!(missing.is_empty(), "missing portrait labels:
{}", missing.join("
"));
}
