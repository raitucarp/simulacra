use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::domain::traits::*;
use std::collections::HashSet;
use std::path::Path;

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
const MIN_NAME_SUGGESTIONS: usize = 4;

#[test]
fn every_class_has_a_complete_preset_and_illustration() {
    for class in ArchetypeClass::ALL {
        let preset = class.preset();
        assert!(!preset.config.description.is_empty(), "{class:?} description");
        assert!(!preset.config.bio.is_empty(), "{class:?} bio");
        assert!(preset.names.len() >= MIN_NAME_SUGGESTIONS, "{class:?} names");
        let unique: HashSet<_> = preset.names.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(unique.len(), preset.names.len(), "{class:?} duplicate names");
        assert_eq!(preset.enneagram_options.first(), Some(&preset.config.enneagram));
        assert!(Path::new(ASSETS_DIR).join(class.illustration()).is_file(), "{class:?} illustration");
        assert_eq!(ArchetypeClass::from_key(class.key()), Some(class));
    }
}

#[test]
fn class_presets_are_distinct() {
    let configs: Vec<AgentConfig> = ArchetypeClass::ALL.iter().map(|c| c.config()).collect();
    for (i, a) in configs.iter().enumerate() {
        for b in &configs[i + 1..] {
            assert_ne!(a, b);
            assert_ne!(a.description, b.description);
        }
    }
}

#[test]
fn class_i18n_keys_follow_the_namespace() {
    let class = ArchetypeClass::StoicSage;
    assert_eq!(class.label_key(), "forge.class.stoic_sage.name");
    assert_eq!(class.pitch_key(), "forge.class.stoic_sage.pitch");
    assert_eq!(class.description_key(), "forge.class.stoic_sage.desc");
    assert_eq!(class.illustration(), "illustrations/classes/stoic_sage.svg");
}

/// The nine fields the legacy rail presets set, per mapped preset (copied from the rail).
#[test]
fn legacy_presets_fold_into_matching_classes() {
    type Legacy = (MbtiType, EnneagramType, InstinctualVariant, CognitiveIntelligenceStyle, EpistemicStance, HumorStyle, DictionRegister, MentionAttitude);
    let legacy: [Legacy; 3] = [
        (MbtiType::Entp, EnneagramType::Type5w4, InstinctualVariant::SoSx, CognitiveIntelligenceStyle::FirstPrinciplesDeductive,
         EpistemicStance::FallibilistSkeptic, HumorStyle::SocraticIrony, DictionRegister::SocraticIronic, MentionAttitude::SocraticQuestioning),
        (MbtiType::Intj, EnneagramType::Type5w6, InstinctualVariant::SpSo, CognitiveIntelligenceStyle::SystemsArchitect,
         EpistemicStance::RationalistDeductive, HumorStyle::DryDeadpanWit, DictionRegister::AcademicPhilosophical, MentionAttitude::AnalyticalDebater),
        (MbtiType::Infj, EnneagramType::Type4w5, InstinctualVariant::SxSo, CognitiveIntelligenceStyle::PoeticIntuitive,
         EpistemicStance::PhenomenologicalExistential, HumorStyle::WarmSelfDeprecating, DictionRegister::LiteraryPoetic, MentionAttitude::EmpatheticListener),
    ];
    for (index, expected) in legacy.iter().enumerate() {
        let class = ArchetypeClass::from_legacy_preset(index).expect("mapped legacy preset");
        let c = class.config();
        let actual = (c.mbti, c.enneagram, c.instinctual_variant, c.intelligence_style, c.epistemic_stance, c.humor_style, c.diction_register, c.mention_attitude);
        assert_eq!(&actual, expected, "{class:?}");
        assert_eq!(c.identity_variant, IdentityVariant::Assertive);
    }
    assert_eq!(ArchetypeClass::from_legacy_preset(3), None, "Pragmatic Builder has no class");
    assert_eq!(ArchetypeClass::from_legacy_preset(99), None);
}

#[test]
fn apply_keeps_personal_fields_and_leaves_nothing_to_change() {
    let mut cfg = AgentConfig { age: 77, pronouns: "xe/xem".into(), primary_language: "Javanese".into(), ..AgentConfig::default() };
    let class = ArchetypeClass::ContrarianTrickster;
    let planned = class.changes_for(&cfg);
    assert!(planned.iter().all(|c| !CLASS_PRESERVED_FIELDS.contains(&c.field.as_str())));
    assert!(planned.iter().any(|c| c.field == "mbti"));
    class.apply_to(&mut cfg);
    assert_eq!(cfg.age, 77);
    assert_eq!(cfg.pronouns, "xe/xem");
    assert_eq!(cfg.primary_language, "Javanese");
    assert_eq!(cfg.mbti, class.config().mbti);
    assert!(class.changes_for(&cfg).is_empty());
}

#[test]
fn config_changes_reports_only_differences() {
    let a = AgentConfig::default();
    let b = AgentConfig { openness: 3, ..a.clone() };
    let changes = config_changes(&a, &b, &[]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].field, "openness");
    assert!(config_changes(&a, &b, &["openness"]).is_empty());
}
