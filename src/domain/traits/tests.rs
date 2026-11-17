use super::*;
use crate::domain::TraitOption;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;

/// Converts a CamelCase identifier to snake_case (digits stay attached to the previous word).
fn snake_case(ident: &str) -> String {
    let mut out = String::new();
    for (idx, ch) in ident.chars().enumerate() {
        if ch.is_ascii_uppercase() && idx > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn check_option_group<T>()
where
    T: TraitOption + Debug + PartialEq + Default + Serialize + DeserializeOwned,
{
    let mut keys = HashSet::new();
    assert!(!T::ALL.is_empty());
    assert!(T::ALL.contains(&T::default()));
    for &option in T::ALL {
        let ident = format!("{option:?}");
        assert_eq!(option.key(), snake_case(&ident), "key of {ident}");
        assert!(keys.insert(option.key()), "duplicate key {}", option.key());
        assert!(!option.label().is_empty());
        assert!(!option.short_label().is_empty());
        let wire = serde_json::to_string(&option).expect("serialize trait option");
        assert_eq!(wire, format!("\"{ident}\""), "serde wire format must be the variant ident");
        let back: T = serde_json::from_str(&wire).expect("deserialize trait option");
        assert_eq!(back, option);
    }
}

macro_rules! check_all_groups {
    ($($ty:ty),+ $(,)?) => { $( check_option_group::<$ty>(); )+ };
}

#[test]
fn every_trait_enum_has_snake_case_unique_keys_and_stable_wire_format() {
    check_all_groups!(
        HumanGender, MbtiType, IdentityVariant, EnneagramType, InstinctualVariant,
        AttachmentStyle, MoralFoundation, ClassicTemperament, LocusOfControl,
        CognitiveIntelligenceStyle, EpistemicStance, CognitivePace, HumorStyle,
        CodeSwitchingStyle, GrammarStyle, DictionRegister, CapitalizationStyle,
        PunctuationHabit, EmojiUsage, SentenceLengthPreference, PostingCadence,
        HumanChronotype, MentionAttitude, ConflictHandlingStyle, PromptAssemblyStrategy,
        PrePostReasoningMode,
        BodyKind, BodyForm, Material, Scale, Temporality, Perception, Mobility, Anthropomorphism,
        RelationToHumans, Withdrawal, RhetoricalMode, RhetoricalDevice, StylisticSchool, SemioticMode,
        SentenceStructure, PostFormat, LexicalField, Weekday,
    );
}

#[test]
fn inherent_all_arrays_keep_their_lengths() {
    assert_eq!(HumanGender::ALL.len(), 4);
    assert_eq!(MbtiType::ALL.len(), 16);
    assert_eq!(EnneagramType::ALL.len(), 18);
    assert_eq!(InstinctualVariant::ALL.len(), 6);
    assert_eq!(DictionRegister::ALL.len(), 7);
    assert_eq!(<MbtiType as TraitOption>::ALL, &MbtiType::ALL[..]);
}

#[test]
fn defaults_match_original_variants() {
    assert_eq!(HumanGender::default(), HumanGender::Male);
    assert_eq!(MbtiType::default(), MbtiType::Intj);
    assert_eq!(EnneagramType::default(), EnneagramType::Type5w4);
    assert_eq!(InstinctualVariant::default(), InstinctualVariant::SpSo);
    assert_eq!(EmojiUsage::default(), EmojiUsage::Never);
}

#[test]
fn per_enum_data_tables_are_preserved() {
    assert_eq!(HumanGender::NonBinary.label(), "Non-Binary");
    assert_eq!(HumanGender::Custom.label(), "Custom / Unspecified");
    assert_eq!(HumanGender::Custom.short_label(), "Custom");
    assert_eq!(HumanGender::Male.short_label(), "Male");
    assert_eq!(HumanGender::Custom.default_pronouns(), "ia/dia");
    assert_eq!(HumanGender::NonBinary.to_string(), "Non-Binary");
    assert_eq!(MbtiType::Entp.code(), "ENTP");
    assert_eq!(MbtiType::Entp.archetype_name(), "Dialectician");
    assert_eq!(MbtiType::Esfp.cognitive_functions(), "Se · Fi · Te · Ni");
    assert!(MbtiType::Istj.cognitive_description().starts_with("Introverted Sensing (Si)"));
    assert_eq!(MbtiType::Infj.to_string(), "INFJ (Advocate)");
    assert_eq!(IdentityVariant::Turbulent.suffix(), "T");
    assert_eq!(IdentityVariant::Turbulent.label(), "Turbulent (-T)");
    assert_eq!(EnneagramType::Type8w9.code(), "8w9");
    assert_eq!(EnneagramType::Type8w9.title(), "8w9 · The Diplomat-Bear");
    assert_eq!(InstinctualVariant::SoSx.code(), "so/sx");
    assert_eq!(DictionRegister::WarmEmpathetic.short_label(), "Empathetic");
    assert_eq!(GrammarStyle::FragmentedStream.short_label(), GrammarStyle::FragmentedStream.label());
    assert_eq!(
        PromptAssemblyStrategy::CustomTemplate.label(),
        "Custom Template with {{Placeholders}} + Full Dossier"
    );
}

#[test]
fn trait_option_keys_paths_and_trait_dispatch() {
    let option = DictionRegister::SocraticIronic;
    assert_eq!(TraitOption::label(option), option.label());
    assert_eq!(TraitOption::short_label(option), "Socratic");
    assert_eq!(option.label_key(), "traits.diction_register.socratic_ironic");
    assert_eq!(option.description_key(), "traits.diction_register.socratic_ironic.desc");
    assert_eq!(option.illustration(), "illustrations/diction_register/socratic_ironic.svg");
    assert_eq!(<EnneagramType as TraitOption>::GROUP, "enneagram");
    assert_eq!(EnneagramType::Type1w9.key(), "type1w9");
}
