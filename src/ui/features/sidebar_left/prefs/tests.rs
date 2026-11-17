use super::*;
use crate::domain::{PostId, ProfileId};
use crate::storage::{Database, Vault};

fn thread(id: &str) -> NavRef {
    NavRef::Thread(PostId::new(id))
}

#[test]
fn sections_fold_and_unfold() {
    let prefs = SidebarPrefs::default();
    assert!(!prefs.is_collapsed(SidebarSection::Recent));
    let folded = prefs.with_section_toggled(SidebarSection::Recent);
    assert!(folded.is_collapsed(SidebarSection::Recent));
    assert!(!folded.is_collapsed(SidebarSection::Pinned));
    assert!(!folded.with_section_toggled(SidebarSection::Recent).is_collapsed(SidebarSection::Recent));
    assert!(!prefs.is_collapsed(SidebarSection::Recent), "the original is unchanged");
}

#[test]
fn pins_toggle_newest_first() {
    let prefs = SidebarPrefs::default().with_pin_toggled(&thread("a")).with_pin_toggled(&NavRef::Document("d".into()));
    assert_eq!(prefs.pins, vec![NavRef::Document("d".into()), thread("a")]);
    let unpinned = prefs.with_pin_toggled(&thread("a"));
    assert_eq!(unpinned.pins, vec![NavRef::Document("d".into())]);
}

#[test]
fn pins_are_capped() {
    let prefs = (0..PIN_CAPACITY + 3).fold(SidebarPrefs::default(), |prefs, at| prefs.with_pin_toggled(&thread(&at.to_string())));
    assert_eq!(prefs.pins.len(), PIN_CAPACITY);
    assert_eq!(prefs.pins[0], thread(&(PIN_CAPACITY + 2).to_string()));
}

#[test]
fn recents_move_to_the_front_without_duplicates_and_are_capped() {
    let prefs = SidebarPrefs::default().with_recent(&thread("a")).with_recent(&thread("b")).with_recent(&thread("a"));
    assert_eq!(prefs.recents, vec![thread("a"), thread("b")]);
    let many = (0..RECENT_CAPACITY + 5).fold(prefs, |prefs, at| prefs.with_recent(&thread(&at.to_string())));
    assert_eq!(many.recents.len(), RECENT_CAPACITY);
}

#[test]
fn prefs_round_trip_through_json_with_stable_wire_names() {
    let prefs = SidebarPrefs::default()
        .with_preference(SidebarPreference::Rail)
        .with_section_toggled(SidebarSection::Members)
        .with_pin_toggled(&NavRef::Profile(ProfileId::new("socrates")))
        .with_recent(&thread("p1"));
    let json = prefs.to_json();
    assert!(json.contains(r#""preference":"rail""#), "{json}");
    assert!(json.contains(r#"{"kind":"profile","id":"socrates"}"#), "{json}");
    assert!(json.contains(r#""members""#), "{json}");
    assert_eq!(SidebarPrefs::from_json(&json), prefs);
}

#[test]
fn unreadable_or_partial_json_falls_back_to_defaults() {
    assert_eq!(SidebarPrefs::from_json("not json"), SidebarPrefs::default());
    let partial = SidebarPrefs::from_json(r#"{"preference":"expanded"}"#);
    assert_eq!(partial.preference, SidebarPreference::Expanded);
    assert!(partial.pins.is_empty());
}

#[test]
fn prefs_persist_in_the_vault_settings_table() {
    let vault = Vault::spawn(Database::in_memory().expect("in-memory db")).expect("vault");
    assert_eq!(load_prefs(Some(&vault)), SidebarPrefs::default());
    let prefs = SidebarPrefs::default().with_pin_toggled(&thread("p9"));
    save_prefs(Some(&vault), &prefs);
    assert_eq!(load_prefs(Some(&vault)), prefs);
    assert_eq!(load_prefs(None), SidebarPrefs::default());
}
