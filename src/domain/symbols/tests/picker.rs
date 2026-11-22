//! The composer's symbol picker (S5): the core set, toggling, recent symbols and search.

use crate::domain::symbols::*;

fn label(choice: &SymbolChoice) -> String {
    match choice {
        SymbolChoice::Entry(EntrySymbol::Task) => "Task".into(),
        SymbolChoice::Entry(EntrySymbol::Event) => "Event".into(),
        SymbolChoice::Entry(EntrySymbol::Note) => "Note".into(),
        SymbolChoice::Signifier(Signifier::Priority) => "Priority".into(),
        SymbolChoice::Signifier(Signifier::Inspiration) => "Inspiration".into(),
        SymbolChoice::Signifier(Signifier::Explore) => "Explore".into(),
        SymbolChoice::Custom(symbol) => symbol.name.clone(),
    }
}

#[test]
fn the_core_set_is_three_entries_then_three_signifiers() {
    let core = core_choices();
    assert_eq!(core.len(), 6);
    assert_eq!(core[0], SymbolChoice::Entry(EntrySymbol::Task));
    assert_eq!(core[3], SymbolChoice::Signifier(Signifier::Priority));
}

#[test]
fn toggling_adds_replaces_and_removes() {
    let mut symbols = PostSymbols::default();
    SymbolChoice::Entry(EntrySymbol::Task).toggle(&mut symbols);
    assert_eq!(symbols, PostSymbols::entry(EntrySymbol::Task), "a task starts open");
    SymbolChoice::Entry(EntrySymbol::Note).toggle(&mut symbols);
    assert_eq!((symbols.entry, symbols.state), (Some(EntrySymbol::Note), None), "a new entry replaces the old one");
    SymbolChoice::Signifier(Signifier::Priority).toggle(&mut symbols);
    let zap = SymbolChoice::Custom(SymbolRef::new("fi", "zap"));
    zap.toggle(&mut symbols);
    assert!(zap.is_chosen(&symbols) && symbols.is_consistent());
    assert_eq!(SymbolChoice::of(&symbols), [SymbolChoice::Entry(EntrySymbol::Note), SymbolChoice::Signifier(Signifier::Priority), zap.clone()]);
    for choice in SymbolChoice::of(&symbols) {
        choice.toggle(&mut symbols);
    }
    assert!(symbols.is_empty(), "toggling every choice again clears the set");
}

#[test]
fn recent_symbols_are_newest_first_without_repeats_and_capped() {
    let mut memory = SymbolMemory::default();
    for i in 0..RECENT_SYMBOLS + 3 {
        memory.used(SymbolChoice::Custom(SymbolRef::new("fi", format!("icon-{i}"))));
    }
    memory.used(SymbolChoice::Custom(SymbolRef::new("fi", "icon-5")));
    assert_eq!(memory.recent.len(), RECENT_SYMBOLS);
    assert_eq!(memory.recent[0], SymbolChoice::Custom(SymbolRef::new("fi", "icon-5")));
    let json = serde_json::to_string(&memory).unwrap();
    assert!(json.contains(r#"{"custom":{"set":"fi","name":"icon-5"}}"#), "{json}");
    let doubled = SymbolMemory { recent: [memory.recent.clone(), memory.recent.clone()].concat() };
    assert_eq!(doubled.validated(), memory);
}

#[test]
fn search_finds_core_names_then_library_icons_by_word_start() {
    let library = [SymbolRef::new("fi", "zoom-out"), SymbolRef::new("fi", "zap"), SymbolRef::new("lu", "task-list")];
    assert!(search_symbols("  ", label, &library).is_empty());
    assert_eq!(search_symbols("ta", label, &library), [SymbolChoice::Entry(EntrySymbol::Task), SymbolChoice::Custom(SymbolRef::new("lu", "task-list"))]);
    assert_eq!(search_symbols("OUT", label, &library), [SymbolChoice::Custom(SymbolRef::new("fi", "zoom-out"))]);
    assert_eq!(search_symbols("z o", label, &library), [SymbolChoice::Custom(SymbolRef::new("fi", "zoom-out"))], "every term starts a word");
    assert!(words_match("Explore", &["exp"]) && !words_match("Explore", &["plore"]));
}
