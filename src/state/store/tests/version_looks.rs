//! Symbols and tint are part of a version (V10): changing them alone is a patch version,
//! every kept version keeps the look it showed, and switching to it brings that look back.

use crate::domain::data::history::{record_changes, Bump, PostRecord, SemVer};
use crate::domain::post::Tint;
use crate::domain::symbols::{EntrySymbol, PostSymbols};
use crate::state::store::{AppState, PostEdit};

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 11, 21, hour, 0, 0)
}

fn look(tint: Option<Tint>, symbols: PostSymbols) -> Option<PostRecord> {
    Some(PostRecord { symbols, tint, attachments: None, lock: None })
}

#[test]
fn a_new_tint_or_symbol_alone_is_a_patch_version_keeping_the_old_look() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Call the bakery").unwrap();
    let edit = PostEdit { look: look(Some(Tint::Amber), PostSymbols::entry(EntrySymbol::Task)), ..PostEdit::text("Call the bakery") };
    let kept = state.edit_post_version(&id, &edit, at(9)).unwrap().expect("a version");
    let head = state.post_head(&id).unwrap();
    assert_eq!((head.version, head.bump), (SemVer::new(1, 0, 1), Some(Bump::Patch)));
    let post = state.get_post(&id).unwrap();
    assert_eq!((post.extras.tint, post.extras.symbols.entry), (Some(Tint::Amber), Some(EntrySymbol::Task)));
    assert_eq!(PostRecord::parse(&kept.record_json), Some(PostRecord { attachments: Some(Default::default()), ..PostRecord::default() }), "the original kept its plain look");
    let same = PostEdit { look: look(Some(Tint::Amber), PostSymbols::entry(EntrySymbol::Task)), ..PostEdit::text("Call the bakery") };
    assert_eq!(state.edit_post_version(&id, &same, at(10)), Ok(None), "the same look is no change");
}

#[test]
fn switching_to_a_version_brings_back_its_symbols_and_tint() {
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("Call the bakery").unwrap();
    state.edit_post_version(&id, &PostEdit { look: look(Some(Tint::Amber), PostSymbols::entry(EntrySymbol::Task)), ..PostEdit::text("Call the bakery about bread") }, at(9)).unwrap();
    let original = state.post_revisions(&id).remove(0);
    state.switch_post_head(&id, &original.id, at(10)).unwrap();
    let post = state.get_post(&id).unwrap();
    assert_eq!((post.content.as_str(), post.extras.tint, post.extras.symbols.clone()), ("Call the bakery", None, PostSymbols::default()));
    let left = state.post_revisions(&id).remove(0);
    let record = PostRecord::parse(&left.record_json).expect("the version left keeps its look");
    assert_eq!((record.tint, record.symbols.entry), (Some(Tint::Amber), Some(EntrySymbol::Task)));
    let base = state.edit_base(&id, Some(&left.id)).unwrap();
    assert_eq!(base.record, Some(record), "editing from a version starts from its look");
}

#[test]
fn records_read_entry_moves_kept_before_tints_and_report_what_changed() {
    let symbols_only = serde_json::to_string(&PostSymbols::entry(EntrySymbol::Task)).unwrap();
    assert_eq!(PostRecord::parse(&symbols_only), Some(PostRecord { symbols: PostSymbols::entry(EntrySymbol::Task), ..PostRecord::default() }));
    assert_eq!(PostRecord::parse(""), None, "a text edit before V10 kept no record");
    assert_eq!(PostRecord::parse(r#"{"tint":"no-such-tint"}"#), Some(PostRecord::default()), "an unknown tint reads as none");
    let tinted = PostRecord { tint: Some(Tint::Amber), ..PostRecord::default() }.to_json();
    let changes = record_changes(&symbols_only, &tinted);
    assert_eq!((changes.tint, changes.symbols), (Some((None, Some(Tint::Amber))), true));
    assert_eq!(record_changes("", &tinted), Default::default(), "nothing to compare without a record");
}
