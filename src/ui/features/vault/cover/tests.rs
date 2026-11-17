use super::{cover_template, CoverSeed, COVER_HUES};
use crate::domain::document::DocumentKind;
use crate::theme::ThemeRegistry;
use crate::ui::illustration::{fill_template, TemplateExtras};

#[test]
fn covers_are_deterministic_per_title_and_kind() {
    for kind in DocumentKind::ALL {
        assert_eq!(cover_template("Ethics", *kind), cover_template("  ethics ", *kind), "{kind:?}");
    }
    assert_ne!(cover_template("Ethics", DocumentKind::Book), cover_template("Tractatus", DocumentKind::Book));
    assert_ne!(cover_template("Ethics", DocumentKind::Book), cover_template("Ethics", DocumentKind::Note));
    assert_eq!(CoverSeed::of("Ethics", DocumentKind::Book), CoverSeed::of("ETHICS", DocumentKind::Book));
}

#[test]
fn the_seed_stream_is_repeatable_and_bounded() {
    let seed = CoverSeed::of("Tractatus", DocumentKind::Book);
    let first: Vec<u64> = (0..20).map(|n| seed.pick(n, 7)).collect();
    assert_eq!(first, (0..20).map(|n| seed.pick(n, 7)).collect::<Vec<_>>());
    assert!(first.iter().all(|v| *v < 7));
    assert_eq!(seed.pick(3, 0), 0);
    assert!((0..50).all(|n| (2.0..=5.0).contains(&seed.between(n, 2.0, 5.0))));
    let ground = seed.hue(0);
    assert_ne!(seed.second_hue(ground), ground);
    assert!(COVER_HUES.contains(&ground));
}

#[test]
fn covers_use_palette_colors_only_and_parse_after_theming() {
    let colors = ThemeRegistry::global().default_palette().colors;
    for kind in DocumentKind::ALL {
        let template = cover_template("A Treatise on Covers", *kind);
        assert!(!template.contains("<text") && !template.contains("<image"), "{kind:?}");
        let literals: Vec<&str> = template.match_indices('#').map(|(at, _)| &template[at..at + 4]).collect();
        assert!(literals.iter().all(|hex| *hex == "#fff" || *hex == "#000"), "{kind:?}: {literals:?}");
        let filled = fill_template(&template, &colors, &TemplateExtras::none());
        assert!(!filled.contains("{{"), "{kind:?} left a placeholder");
        resvg::usvg::Tree::from_str(&filled, &resvg::usvg::Options::default()).unwrap_or_else(|e| panic!("{kind:?}: {e}"));
    }
}

#[test]
fn the_vault_art_paths_match_their_typed_keys() {
    use crate::ui::features::vault::{EMPTY_VAULT_ART, NO_MATCHES_ART};
    use crate::ui::illustration::EmptyKey;
    assert_eq!(EmptyKey::VaultEmpty.path(), EMPTY_VAULT_ART);
    assert_eq!(EmptyKey::VaultNoMatches.path(), NO_MATCHES_ART);
}
