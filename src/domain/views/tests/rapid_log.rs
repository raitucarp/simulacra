use super::{date, locked, post_at, utc_day};
use crate::domain::symbols::{EntryState, EntrySymbol, PostSymbols, Signifier, SymbolArt, SymbolGlyph};
use crate::domain::views::rapid_log::rapid_log;

#[test]
fn days_read_newest_first_and_entries_in_the_order_written() {
    let posts = [
        post_at("a", "first  of\nmonday", "2026-11-02T09:00:00Z"),
        post_at("b", "tuesday", "2026-11-03T08:00:00Z"),
        post_at("c", "later monday", "2026-11-02T18:00:00Z"),
    ];
    let log = rapid_log(&posts, utc_day);
    assert_eq!(log.iter().map(|day| day.day).collect::<Vec<_>>(), [date("2026-11-03"), date("2026-11-02")]);
    let monday: Vec<_> = log[1].entries.iter().map(|entry| entry.text.as_str()).collect();
    assert_eq!(monday, ["first of monday", "later monday"]);
}

#[test]
fn a_symbol_is_the_bullet_and_signifiers_mark_it() {
    let mut task = post_at("t", "call the bank", "2026-11-02T09:00:00Z");
    task.extras.symbols = PostSymbols::entry(EntrySymbol::Task);
    task.extras.symbols.add_signifier(Signifier::Priority);
    let plain = post_at("p", "plain thought", "2026-11-02T10:00:00Z");
    let log = rapid_log([&task, &plain], utc_day);
    let entries = &log[0].entries;
    assert_eq!(entries[0].bullet, Some(SymbolGlyph::Core(SymbolArt::Entry(EntrySymbol::Task, Some(EntryState::Open)))));
    assert_eq!(entries[0].marks, [SymbolArt::Signifier(Signifier::Priority)]);
    assert_eq!((entries[1].bullet.clone(), entries[1].marks.len()), (None, 0));
}

#[test]
fn a_locked_post_keeps_only_its_placeholder() {
    let secret = locked(post_at("s", "the secret plan", "2026-11-02T09:00:00Z"));
    let log = rapid_log([&secret], utc_day);
    let entry = &log[0].entries[0];
    assert!(entry.locked);
    assert!(entry.text.is_empty());
}

#[test]
fn no_posts_make_an_empty_log() {
    assert!(rapid_log(std::iter::empty(), utc_day).is_empty());
}
