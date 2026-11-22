//! A persona's feeling as a journal check-in (M6).

use crate::domain::journal::mood::from_feeling::{checkin_from_feeling, feeling_affect};
use crate::domain::journal::mood::{AffectRegion, ENERGY_MAX, ENERGY_MIN};
use crate::domain::journal::privacy::{may_reach, JournalPrivacy, Surface};
use crate::domain::post::{Intensity, PostKind};

fn feeling(emojis: &[&str], level: u8, note: &str) -> PostKind {
    let emojis: Vec<String> = emojis.iter().map(|emoji| emoji.to_string()).collect();
    PostKind::feeling(&emojis, Intensity::new(level), note).expect("valid")
}

#[test]
fn a_feeling_becomes_a_check_in_with_its_note() {
    let draft = checkin_from_feeling(&feeling(&["😡"], 5, "the train again")).expect("a check-in");
    let affect = draft.affect.expect("a point");
    assert!(affect.valence < 0.0 && affect.arousal > 0.0, "{affect:?}");
    assert_eq!(draft.note, "the train again");
    assert!((ENERGY_MIN..=ENERGY_MAX).contains(&draft.energy) && draft.energy > 3, "an angry feeling is activated");
    assert_eq!(checkin_from_feeling(&PostKind::Question), None);
}

#[test]
fn intensity_moves_the_point_away_from_the_centre() {
    let faint = feeling_affect(&["😌".to_string()], Intensity::MIN);
    let strong = feeling_affect(&["😌".to_string()], Intensity::MAX);
    assert!(strong.valence > faint.valence && strong.arousal < faint.arousal);
    assert_eq!(strong.region(), AffectRegion::Calm);
    let unknown = feeling_affect(&["🦀".to_string()], Intensity::MAX);
    assert_eq!(unknown.region(), AffectRegion::Neutral, "a face the table does not know reads as neutral");
}

#[test]
fn the_logged_check_in_stays_private() {
    let privacy = JournalPrivacy::default();
    for surface in [Surface::Feed, Surface::SemanticSearch, Surface::Insights, Surface::AgentContext] {
        assert!(!may_reach(surface, privacy, false), "{surface:?}");
    }
}

#[test]
fn logging_feelings_is_off_until_the_user_turns_it_on() {
    use crate::domain::preferences::{SettingValue, WritingField};
    use crate::domain::preferences::WritingPrefs;
    assert!(!WritingPrefs::default().feeling_checkins);
    let on = WritingField::FeelingCheckins.set(WritingPrefs::default(), &SettingValue::Toggle(true));
    assert!(on.feeling_checkins);
    assert_eq!(WritingField::FeelingCheckins.get(&on), SettingValue::Toggle(true));
}
