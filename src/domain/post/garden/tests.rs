use super::*;

#[test]
fn a_note_grows_from_seed_to_evergreen_and_back() {
    assert_eq!(Maturity::Seed.next(), Some(Maturity::Budding));
    assert_eq!(Maturity::Budding.next(), Some(Maturity::Evergreen));
    assert_eq!(Maturity::Evergreen.next(), None);
    assert_eq!(Maturity::Evergreen.previous(), Some(Maturity::Budding));
    assert_eq!(Maturity::Seed.previous(), None);
    assert_eq!(Maturity::ALL.iter().map(|m| m.key()).collect::<Vec<_>>(), vec!["seed", "budding", "evergreen"]);
}

#[test]
fn tending_records_when_the_stage_changed() {
    let (planted, later) = (crate::time::utc(2026, 11, 1, 9, 0, 0), crate::time::utc(2026, 11, 20, 9, 0, 0));
    let note = GardenNote::planted(planted);
    assert_eq!((note.maturity, note.tended_at), (Maturity::Seed, None));
    let grown = note.tended(Maturity::Budding, later);
    assert_eq!((grown.maturity, grown.planted_at, grown.tended_at), (Maturity::Budding, planted, Some(later)));
    assert_eq!(grown.tended(Maturity::Budding, crate::time::utc(2026, 12, 1, 9, 0, 0)), grown, "no change, no new date");
}

#[test]
fn a_garden_note_round_trips_and_old_posts_have_none() {
    let note = GardenNote::planted(crate::time::utc(2026, 11, 1, 9, 0, 0)).tended(Maturity::Evergreen, crate::time::utc(2026, 11, 2, 9, 0, 0));
    let json = serde_json::to_string(&note).expect("json");
    assert!(json.contains("\"evergreen\""));
    assert_eq!(serde_json::from_str::<GardenNote>(&json).expect("back"), note);
    let extras: crate::domain::post::PostExtras = serde_json::from_str("{}").expect("old extras");
    assert!(extras.garden.is_none());
}
