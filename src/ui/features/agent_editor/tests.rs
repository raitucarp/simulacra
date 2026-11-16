use super::rail::archetypes::apply_archetype_preset;
use super::AgentConfigTab;
use crate::domain::{EnneagramType, MbtiType, Profile};
use crate::ui::components::trait_semantic_label;

#[core::prelude::v1::test]
fn test_vertical_tabs_have_sub_tabs_and_live_summaries() {
    let profile = Profile::new_agent("agent_test", "Sokrates of Alopece", "@sokrates", "Classical Dialectician");
    for tab in AgentConfigTab::ALL {
        let sub_tabs = tab.sub_tabs();
        assert!(!sub_tabs.is_empty(), "{:?} must define horizontal sub-tabs", tab);
        let summary = tab.live_summary(&profile);
        assert!(!summary.trim().is_empty(), "{:?} must return a non-empty live summary", tab);
        assert!(!tab.localized_label().trim().is_empty());
    }
}

#[core::prelude::v1::test]
fn test_archetype_presets_configure_cohesive_psychology() {
    let mut profile = Profile::new_agent("agent_preset", "Test Thinker", "@thinker", "Philosopher");

    apply_archetype_preset(&mut profile, 0);
    assert_eq!(profile.config.mbti, MbtiType::Entp);
    assert_eq!(profile.config.enneagram, EnneagramType::Type5w4);

    apply_archetype_preset(&mut profile, 1);
    assert_eq!(profile.config.mbti, MbtiType::Intj);
    assert_eq!(profile.config.enneagram, EnneagramType::Type5w6);

    apply_archetype_preset(&mut profile, 2);
    assert_eq!(profile.config.mbti, MbtiType::Infj);
    assert_eq!(profile.config.enneagram, EnneagramType::Type4w5);

    apply_archetype_preset(&mut profile, 3);
    assert_eq!(profile.config.mbti, MbtiType::Entj);
    assert_eq!(profile.config.enneagram, EnneagramType::Type8w7);
}

#[core::prelude::v1::test]
fn test_trait_semantic_labels() {
    assert_eq!(trait_semantic_label(95), "Exceptional");
    assert_eq!(trait_semantic_label(80), "High");
    assert_eq!(trait_semantic_label(60), "Balanced");
    assert_eq!(trait_semantic_label(35), "Moderate-Low");
    assert_eq!(trait_semantic_label(15), "Reserved");
}
