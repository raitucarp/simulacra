use super::super::*;
use crate::domain::traits::{BodyForm, BodyKind, Material, Scale, StylisticSchool};
use crate::domain::AgentConfig;

#[test]
fn a_human_is_drawn_by_occupation() {
    let config = AgentConfig { occupation: "Engineer".into(), current_mood: "Curious".into(), ..AgentConfig::default() };
    let subject = subject_from_config(&config);
    assert_eq!(subject.noun, "an engineer");
    assert!(subject.details.contains(&"feeling curious".to_string()), "{subject:?}");
}

#[test]
fn a_non_human_is_drawn_by_form_material_and_scale() {
    let mut config = AgentConfig { occupation: "Watcher".into(), current_mood: String::new(), palette: Some("ember_glow".into()), ..AgentConfig::default() };
    config.embodiment.set_kind(BodyKind::Creature);
    config.embodiment.set_form(BodyForm::Crow);
    config.embodiment.material = Material::Metal;
    config.embodiment.scale = Scale::Planetary;
    config.rhetoric.stylistic_school = StylisticSchool::Baroque;
    let subject = subject_from_config(&config);
    assert_eq!(subject.noun, "a crow");
    assert_eq!(subject.details, vec!["made of metal".to_string(), "planetary scale".into(), "baroque sensibility".into()]);
    assert_eq!(subject.palette_hint.as_deref(), Some("ember_glow"));
}

#[test]
fn articles_follow_the_first_letter() {
    assert_eq!(with_article("Owl keeper"), "an owl keeper");
    assert_eq!(with_article(" poet "), "a poet");
}

#[test]
fn a_post_gist_is_its_first_clean_sentence() {
    let text = "Freedom is **understanding** [[necessity]]. Then a second thought. https://example.test/x";
    assert_eq!(post_gist(text), "Freedom is understanding necessity");
    assert_eq!(post_gist("#stoicism   calm   minds"), "stoicism calm minds");
    let long = "word ".repeat(60);
    assert_eq!(post_gist(&long).split_whitespace().count(), subject::GIST_MAX_WORDS);
    assert!(post_gist("").is_empty() && subject_from_post("  ").is_empty());
}
