use super::*;
use std::collections::HashSet;

/// Reads a locale key in English (tests run in parallel, so never through the global locale).
fn english(key: &str) -> String {
    rust_i18n::t!(key, locale = "en").to_string()
}

#[test]
fn every_command_has_exactly_one_registry_row() {
    for id in CommandId::ALL {
        let rows = COMMANDS.iter().filter(|spec| spec.id == *id).count();
        assert_eq!(rows, 1, "{id:?} has {rows} registry rows");
    }
    assert_eq!(COMMANDS.len(), CommandId::ALL.len());
}

#[test]
fn wire_keys_are_unique_and_round_trip() {
    let mut seen = HashSet::new();
    for id in CommandId::ALL {
        assert!(seen.insert(id.wire_key()), "duplicate wire key {}", id.wire_key());
        assert_eq!(CommandId::from_wire_key(id.wire_key()), Some(*id));
    }
    assert_eq!(CommandId::from_wire_key("no_such_command"), None);
}

#[test]
fn wire_keys_never_change() {
    // Persisted in the palette history: renaming one forgets the user's recents.
    let pinned = [(CommandId::NewThread, "new_thread"), (CommandId::GoTimeline, "go_timeline"), (CommandId::OpenPalette, "open_palette")];
    for (id, key) in pinned {
        assert_eq!(id.wire_key(), key);
    }
}

#[test]
fn every_command_has_an_english_title_and_keywords() {
    for spec in COMMANDS {
        let title_key = spec.title_key();
        let keywords_key = format!("{title_key}_keywords");
        assert_ne!(english(&title_key), title_key, "missing title for {:?}", spec.id);
        assert_ne!(english(&keywords_key), keywords_key, "missing keywords for {:?}", spec.id);
    }
}

#[test]
fn availability_decides_listing_and_running() {
    let note = spec(CommandId::NewNote);
    assert_eq!(note.availability, Availability::Ready, "quick capture shipped with the import wave");
    assert!(note.is_listed() && note.is_runnable());
    assert!(spec(CommandId::OpenInbox).is_listed());
    let palette = spec(CommandId::OpenPalette);
    assert!(!palette.is_listed() && palette.is_runnable(), "shortcut-only commands still run");
    assert!(spec(CommandId::NewThread).is_listed());
}

#[test]
fn screens_are_the_go_commands() {
    let screens: Vec<CommandId> = COMMANDS.iter().filter(|spec| spec.kind == CommandKind::Screen).map(|spec| spec.id).collect();
    assert_eq!(screens, vec![CommandId::GoTimeline, CommandId::GoKnowledge, CommandId::GoAgents, CommandId::GoPersonas, CommandId::GoMindGraph, CommandId::GoReview, CommandId::GoInsights, CommandId::GoJournal, CommandId::GoArena]);
}
