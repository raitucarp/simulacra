use super::*;
mod picker;
use crate::domain::post::PostExtras;
use EntryState::*;

fn full_set() -> PostSymbols {
    PostSymbols {
        entry: Some(EntrySymbol::Task),
        state: Some(Delegated),
        signifiers: vec![Signifier::Priority, Signifier::Explore],
        custom: vec![SymbolRef::new("core", "flask")],
    }
}

#[test]
fn enums_serialize_as_snake_case_strings() {
    assert_eq!(serde_json::to_string(&EntrySymbol::Event).unwrap(), "\"event\"");
    assert_eq!(serde_json::to_string(&Scheduled).unwrap(), "\"scheduled\"");
    assert_eq!(serde_json::to_string(&Signifier::Inspiration).unwrap(), "\"inspiration\"");
    for state in EntryState::ALL {
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(serde_json::from_str::<EntryState>(&json).unwrap(), state);
    }
}

#[test]
fn symbol_set_round_trips_through_json() {
    let set = full_set();
    let json = serde_json::to_value(&set).unwrap();
    assert_eq!(
        json,
        serde_json::json!({
            "entry": "task",
            "state": "delegated",
            "signifiers": ["priority", "explore"],
            "custom": [{"set": "core", "name": "flask"}],
        })
    );
    assert_eq!(serde_json::from_value::<PostSymbols>(json).unwrap(), set);
}

#[test]
fn empty_symbol_set_stores_nothing() {
    assert!(PostSymbols::default().is_empty());
    assert_eq!(serde_json::to_string(&PostSymbols::default()).unwrap(), "{}");
    let extras = PostExtras { mood: Some("calm".into()), ..PostExtras::default() };
    assert_eq!(serde_json::to_string(&extras).unwrap(), r#"{"mood":"calm"}"#);
}

#[test]
fn extras_round_trip_with_symbols() {
    let extras = PostExtras { symbols: full_set(), ..PostExtras::default() };
    let json = serde_json::to_string(&extras).unwrap();
    assert!(json.contains("\"symbols\""));
    assert_eq!(serde_json::from_str::<PostExtras>(&json).unwrap(), extras);
    assert!(!extras.is_empty());
    assert!(!extras.has_details(), "symbols replace the avatar; they are not details");
}

#[test]
fn old_extras_without_symbols_still_load() {
    let old = r#"{"mood":"wry","tools":[{"name":"search","summary":"q","failed":false}]}"#;
    let extras: PostExtras = serde_json::from_str(old).unwrap();
    assert!(extras.symbols.is_empty());
    assert_eq!(extras.mood.as_deref(), Some("wry"));
    let partial: PostExtras = serde_json::from_str(r#"{"symbols":{"entry":"note"}}"#).unwrap();
    assert_eq!(partial.symbols, PostSymbols { entry: Some(EntrySymbol::Note), ..PostSymbols::default() });
}

#[test]
fn notes_have_no_states_and_events_a_subset() {
    assert!(allowed_states(EntrySymbol::Note).is_empty());
    assert_eq!(initial_state(EntrySymbol::Note), None);
    assert_eq!(allowed_states(EntrySymbol::Task), &EntryState::ALL);
    assert_eq!(allowed_states(EntrySymbol::Event), &[Open, Done, Scheduled, Dropped]);
    assert!(!state_allowed(EntrySymbol::Event, Migrated));
    assert!(!state_allowed(EntrySymbol::Event, Delegated));
    assert_eq!(initial_state(EntrySymbol::Event), Some(Open));
}

#[test]
fn open_entries_move_anywhere_settled_ones_only_reopen() {
    for to in [Done, Migrated, Scheduled, Dropped, Delegated] {
        assert!(can_transition(EntrySymbol::Task, Open, to), "open -> {to:?}");
    }
    for to in [Migrated, Scheduled, Dropped, Delegated] {
        assert!(!can_transition(EntrySymbol::Task, Done, to), "done -> {to:?}");
    }
    assert!(can_transition(EntrySymbol::Task, Done, Open));
    assert!(can_transition(EntrySymbol::Task, Dropped, Open));
    assert!(!can_transition(EntrySymbol::Task, Dropped, Done));
}

#[test]
fn pending_entries_reopen_finish_or_drop() {
    for from in [Migrated, Scheduled, Delegated] {
        for to in [Open, Done, Dropped] {
            assert!(can_transition(EntrySymbol::Task, from, to), "{from:?} -> {to:?}");
        }
    }
    assert!(!can_transition(EntrySymbol::Task, Scheduled, Delegated));
    assert!(!can_transition(EntrySymbol::Task, Migrated, Scheduled));
}

#[test]
fn transitions_respect_the_entry_kind() {
    assert!(!can_transition(EntrySymbol::Event, Open, Migrated));
    assert!(can_transition(EntrySymbol::Event, Open, Scheduled));
    assert!(can_transition(EntrySymbol::Event, Scheduled, Done));
    for from in EntryState::ALL {
        for to in EntryState::ALL {
            assert!(!can_transition(EntrySymbol::Note, from, to));
        }
        assert!(!can_transition(EntrySymbol::Task, from, from), "no self transition");
    }
}

#[test]
fn set_state_applies_only_allowed_moves() {
    let mut task = PostSymbols::entry(EntrySymbol::Task);
    assert_eq!(task.state, Some(Open));
    assert!(task.set_state(Done));
    assert!(!task.set_state(Migrated));
    assert_eq!(task.state, Some(Done));
    let mut note = PostSymbols::entry(EntrySymbol::Note);
    assert!(!note.set_state(Done));
    assert_eq!(note.state, None);
    assert!(!PostSymbols::default().set_state(Open));
}

#[test]
fn consistency_checks_state_against_entry() {
    assert!(PostSymbols::default().is_consistent());
    assert!(PostSymbols::entry(EntrySymbol::Note).is_consistent());
    assert!(full_set().is_consistent());
    let bad_note = PostSymbols { state: Some(Open), ..PostSymbols::entry(EntrySymbol::Note) };
    assert!(!bad_note.is_consistent());
    let bad_event = PostSymbols { state: Some(Migrated), ..PostSymbols::entry(EntrySymbol::Event) };
    assert!(!bad_event.is_consistent());
    let stateless_task = PostSymbols { state: None, ..PostSymbols::entry(EntrySymbol::Task) };
    assert!(!stateless_task.is_consistent());
    let orphan_state = PostSymbols { state: Some(Done), ..PostSymbols::default() };
    assert!(!orphan_state.is_consistent());
}

#[test]
fn signifiers_and_custom_symbols_do_not_duplicate() {
    let mut set = PostSymbols::default();
    set.add_signifier(Signifier::Priority);
    set.add_signifier(Signifier::Priority);
    set.add_signifier(Signifier::Inspiration);
    assert_eq!(set.signifiers, vec![Signifier::Priority, Signifier::Inspiration]);
    set.remove_signifier(Signifier::Priority);
    assert_eq!(set.signifiers, vec![Signifier::Inspiration]);
    set.add_custom(SymbolRef::new("core", "star"));
    set.add_custom(SymbolRef::new("core", "star"));
    assert_eq!(set.custom.len(), 1);
    assert!(EntryState::Done.is_closed() && !EntryState::Delegated.is_closed());
}

#[test]
fn the_face_shows_the_entry_first_then_a_custom_symbol_then_a_signifier() {
    assert_eq!(PostSymbols::default().face(), None, "no symbols: the avatar");
    let face = full_set().face().expect("a face");
    assert_eq!(face.primary, SymbolGlyph::Core(SymbolArt::Entry(EntrySymbol::Task, Some(Delegated))));
    assert_eq!(face.badges, [SymbolArt::Signifier(Signifier::Priority), SymbolArt::Signifier(Signifier::Explore)]);
    let custom = PostSymbols { custom: vec![SymbolRef::new("fi", "zap")], signifiers: vec![Signifier::Inspiration], ..PostSymbols::default() };
    let face = custom.face().unwrap();
    assert_eq!(face.primary, SymbolGlyph::Custom(SymbolRef::new("fi", "zap")));
    assert_eq!(face.badges, [SymbolArt::Signifier(Signifier::Inspiration)]);
}

#[test]
fn a_signifier_alone_is_the_primary_symbol_and_badges_stop_at_two() {
    let only = PostSymbols { signifiers: Signifier::ALL.to_vec(), ..PostSymbols::default() };
    let face = only.face().unwrap();
    assert_eq!(face.primary, SymbolGlyph::Core(SymbolArt::Signifier(Signifier::Priority)));
    assert_eq!(face.badges, [SymbolArt::Signifier(Signifier::Inspiration), SymbolArt::Signifier(Signifier::Explore)]);
    let mut many = PostSymbols { signifiers: Signifier::ALL.to_vec(), ..PostSymbols::entry(EntrySymbol::Note) };
    many.add_signifier(Signifier::Priority);
    let face = many.face().unwrap();
    assert_eq!(face.primary, SymbolGlyph::Core(SymbolArt::Entry(EntrySymbol::Note, None)));
    assert_eq!(face.badges.len(), MAX_BADGES);
    assert_eq!(SymbolRef::new("fi", "zap").asset_path(), "sym:fi/zap");
}
