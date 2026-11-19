use super::{day, entry, noon};
use crate::domain::journal::calendar::{heat_strip, words_by_day};
use crate::domain::journal::entry::{entries_on, newest_first};
use crate::domain::journal::records::{find, removed, upserted};
use crate::domain::journal::search::{excerpt_around, search_entries};
use crate::domain::journal::streak::{streaks, Streaks};
use crate::domain::journal::templates::{template_body, Template};
use crate::domain::journal::RecordId;

#[test]
fn titles_skip_headings_and_shorten() {
    assert_eq!(entry("a", 0, "\n## Morning page\nbody").title(), "Morning page");
    let long = "word ".repeat(40);
    assert!(entry("b", 0, &long).title().ends_with('…'));
    assert!(entry("c", 0, "  \n ").is_blank());
}

#[test]
fn edits_keep_the_day_and_creation_time() {
    let original = entry("a", 0, "first");
    let edited = original.edited("second", noon(3));
    assert_eq!((edited.day, edited.created_at, edited.updated_at, edited.body.as_str()), (original.day, original.created_at, noon(3), "second"));
}

#[test]
fn records_upsert_in_place_and_remove() {
    let list = vec![entry("a", 0, "x"), entry("b", 1, "y")];
    let replaced = upserted(&list, entry("a", 0, "z"));
    assert_eq!(replaced.iter().map(|e| e.body.as_str()).collect::<Vec<_>>(), ["z", "y"], "replaced where it was");
    assert_eq!(upserted(&list, entry("c", 2, "w")).len(), 3);
    assert_eq!(removed(&list, &RecordId::new("a")).len(), 1);
    assert!(find(&list, &RecordId::new("b")).is_some());
}

#[test]
fn ordering_by_day() {
    let list = vec![entry("a", 0, "x"), entry("b", 2, "y"), entry("c", 0, "z")];
    assert_eq!(newest_first(&list)[0].id.as_str(), "b");
    assert_eq!(entries_on(&list, day(0)).len(), 2);
}

#[test]
fn search_needs_every_word_in_any_case() {
    let list = vec![entry("a", 0, "Walked by the river, felt calm"), entry("b", 1, "River of thoughts; anxious"), entry("c", 2, "Nothing")];
    let ids = |q: &str| search_entries(&list, q).iter().map(|e| e.id.as_str().to_string()).collect::<Vec<_>>();
    assert_eq!(ids("river"), ["b", "a"]);
    assert_eq!(ids("RIVER calm"), ["a"]);
    assert_eq!(ids("").len(), 3);
    assert!(ids("ocean").is_empty());
    assert!(excerpt_around(&list[1], "anxious", 8).starts_with('…'));
    assert_eq!(excerpt_around(&entry("d", 0, "## Head\n- item"), "", 40), "Head item", "line marks are dropped");
}

#[test]
fn templates_are_headed_sections() {
    let body = template_body(&["One".into(), "Two".into()]);
    assert_eq!(body, "## One\n\n## Two\n");
    assert_eq!(Template::Evening.section_keys().len(), 4);
    assert!(Template::ALL.iter().all(|t| t.section_keys().iter().all(|k| k.starts_with("journal.section."))));
}

#[test]
fn streaks_forgive_an_open_today() {
    let days = [day(0), day(1), day(2), day(5), day(6)];
    assert_eq!(streaks(days, day(6)), Streaks { current: 2, longest: 3, days: 5 });
    assert_eq!(streaks(days, day(7)).current, 2, "today not written yet keeps yesterday's streak");
    assert_eq!(streaks(days, day(8)).current, 0);
    assert_eq!(streaks([day(9)], day(3)), Streaks::default(), "future days are ignored");
}

#[test]
fn heat_strip_levels_words_per_day() {
    let list = vec![entry("a", 0, "one two three four"), entry("b", 0, "five"), entry("c", 1, "six")];
    let words = words_by_day(&list);
    assert_eq!(words.get(&day(0)), Some(&5));
    let strip = heat_strip(&words, day(1), 2);
    assert_eq!((strip.len(), strip[0].len()), (2, 7));
    let cell = |d| strip.iter().flatten().find(|c| c.day == d).copied().unwrap();
    assert_eq!((cell(day(0)).level, cell(day(1)).level), (4, 1));
    assert!(strip.iter().flatten().filter(|c| c.day > day(1)).all(|c| c.future && c.count == 0));
}
