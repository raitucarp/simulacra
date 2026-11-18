use super::super::*;
use crate::domain::TraitOption;

#[test]
fn every_option_has_a_phrase_and_chip_art_path() {
    assert!(ImageStyle::ALL.iter().all(|style| !style.phrase().is_empty()));
    assert!(ImageMood::ALL.iter().all(|mood| !mood.phrase().is_empty()));
    assert!(ImageAvoid::ALL.iter().all(|hint| !hint.phrase().is_empty() && !hint.positive().is_empty()));
    assert_eq!(ImageStyle::Ink.illustration(), "illustrations/image_style/ink.svg");
    assert_eq!(ImagePalette::MatchTheme.label_key(), "traits.image_palette.match_theme");
}

#[test]
fn targets_split_the_framings() {
    assert!(ImageTarget::Avatar.allows(ImageFraming::Emblem) && !ImageTarget::Avatar.allows(ImageFraming::Hero));
    assert!(ImageTarget::Post.allows(ImageFraming::Diagram) && !ImageTarget::Post.allows(ImageFraming::FullFigure));
    let all: Vec<ImageFraming> = [ImageTarget::Avatar, ImageTarget::Post].iter().flat_map(|t| t.framings().to_vec()).collect();
    assert_eq!(all.len(), ImageFraming::ALL.len());
    assert_eq!(ImageChoices::for_target(ImageTarget::Post).framing, ImageFraming::Hero);
}

#[test]
fn choices_change_without_mutation_and_avoid_toggles_in_order() {
    let base = ImageChoices::for_target(ImageTarget::Avatar);
    let inked = base.clone().with_style(ImageStyle::Ink).with_mood(ImageMood::Dramatic);
    assert_eq!(base.style, ImageStyle::Illustration);
    assert_eq!((inked.style, inked.mood), (ImageStyle::Ink, ImageMood::Dramatic));
    let toggled = base.clone().toggle_avoid(ImageAvoid::Blur).toggle_avoid(ImageAvoid::Text);
    assert_eq!(toggled.avoid, vec![ImageAvoid::Watermark, ImageAvoid::Blur]);
    assert_eq!(toggled.toggle_avoid(ImageAvoid::Text).avoid, vec![ImageAvoid::Text, ImageAvoid::Watermark, ImageAvoid::Blur]);
    assert_eq!(base.clone().retargeted(ImageTarget::Post).framing, ImageFraming::Hero);
    let abstract_post = ImageChoices::for_target(ImageTarget::Post).with_framing(ImageFraming::Abstract);
    assert_eq!(abstract_post.retargeted(ImageTarget::Post).framing, ImageFraming::Abstract);
}

#[test]
fn surprise_is_seeded_and_coherent() {
    for target in [ImageTarget::Avatar, ImageTarget::Post] {
        for seed in 0..500u64 {
            let picks = surprise(target, seed);
            assert_eq!(picks, surprise(target, seed), "same seed, same picks");
            assert!(moods_for(picks.style).contains(&picks.mood), "{picks:?}");
            assert!(palettes_for(picks.style).contains(&picks.palette), "{picks:?}");
            assert!(target.allows(picks.framing), "{picks:?}");
            assert!(picks.avoid.contains(&ImageAvoid::Text) && picks.avoid.contains(&ImageAvoid::Watermark));
        }
    }
    let styles: std::collections::HashSet<ImageStyle> = (0..200u64).map(|seed| surprise(ImageTarget::Post, seed).style).collect();
    assert_eq!(styles.len(), ImageStyle::ALL.len(), "every style shows up");
}

#[test]
fn avoid_hints_follow_style_and_framing() {
    let photo = avoid_for(ImageStyle::Photographic, ImageFraming::Hero);
    assert!(photo.contains(&ImageAvoid::Blur) && photo.contains(&ImageAvoid::Distortion));
    assert!(avoid_for(ImageStyle::Ink, ImageFraming::Diagram).contains(&ImageAvoid::Clutter));
    assert!(!avoid_for(ImageStyle::Ink, ImageFraming::Abstract).contains(&ImageAvoid::Distortion));
}

#[test]
fn theme_colours_get_plain_names() {
    assert_eq!(colour_name(0x8b, 0x5c, 0xf6), "violet");
    assert_eq!(colour_name(0xea, 0xb3, 0x08), "gold");
    assert_eq!(colour_name(0x10, 0xb9, 0x81), "green");
    assert_eq!(colour_name(0x00, 0x00, 0x00), "near-black");
    assert_eq!(colour_name(0xf5, 0xf5, 0xf5), "off-white");
    assert_eq!(colour_name(0x80, 0x80, 0x80), "grey");
    assert_eq!(colour_name(0xe0, 0x30, 0x30), "red");
    let names = distinct_names(&[(0x8b, 0x5c, 0xf6), (0x7c, 0x3a, 0xed), (0xea, 0xb3, 0x08)]);
    assert_eq!(names, vec!["violet".to_string(), "gold".to_string()]);
}
