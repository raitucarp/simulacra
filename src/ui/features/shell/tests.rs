//! Unit tests of the agent editor field rules that guard against draft / data loss.

use super::editor_field::EditorTextField as F;
use crate::domain::Profile;

fn sample() -> Profile {
    let mut profile = Profile::new_agent("agent_test", "Sokrates", "@sokrates", "Philosopher");
    profile.config.primary_language = "Greek".to_string();
    profile.avatar_path = Some("old.png".to_string());
    profile
}

#[test]
fn all_lists_every_field_in_declaration_order() {
    // `AgentEditorFields::slot` indexes its table by discriminant.
    for (idx, field) in F::ALL.into_iter().enumerate() {
        assert_eq!(field as usize, idx, "{field:?} is out of order");
    }
}

#[test]
fn empty_required_fields_keep_the_stored_value() {
    let mut profile = sample();
    for field in [F::Name, F::Handle, F::PrimaryLanguage] {
        field.apply(&mut profile, "   ");
    }
    assert_eq!(profile, sample());
}

#[test]
fn handle_gets_the_at_prefix_once() {
    let mut profile = sample();
    F::Handle.apply(&mut profile, " plato ");
    assert_eq!(profile.handle, "@plato");
    F::Handle.apply(&mut profile, "@aristotle");
    assert_eq!(profile.handle, "@aristotle");
}

#[test]
fn avatar_path_strips_quotes_and_clears_when_empty() {
    let mut profile = sample();
    F::AvatarPath.apply(&mut profile, "\"C:\\img\\me.png\"");
    assert_eq!(profile.avatar_path.as_deref(), Some("C:\\img\\me.png"));
    F::AvatarPath.apply(&mut profile, "  ");
    assert_eq!(profile.avatar_path, None);
}

#[test]
fn short_fields_trim_and_long_fields_keep_whitespace() {
    let mut profile = sample();
    F::Occupation.apply(&mut profile, "  Teacher  ");
    F::Bio.apply(&mut profile, "  line one\n");
    assert_eq!(profile.config.occupation, "Teacher");
    assert_eq!(profile.config.bio, "  line one\n");
}

#[test]
fn display_then_apply_round_trips_every_field() {
    let original = sample();
    let mut copy = original.clone();
    for field in F::ALL {
        field.apply(&mut copy, &field.display_value(&original));
    }
    assert_eq!(copy, original, "unchanged inputs must not modify the profile");
}
