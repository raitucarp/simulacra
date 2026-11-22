use super::*;
use EntryState::*;
use TodoTokenKind as K;

fn one(text: &str) -> TodoItem {
    let mut items = parse_todos(text);
    assert_eq!(items.len(), 1, "expected one item in {text:?}");
    items.remove(0)
}

fn token_texts<'a>(source: &'a str, item: &TodoItem) -> Vec<(TodoTokenKind, &'a str)> {
    item.tokens.iter().map(|t| (t.kind, &source[t.span.clone()])).collect()
}

#[test]
fn every_box_maps_to_its_state() {
    let text = "[ ] a\n[x] b\n[X] c\n[>] d\n[<] e\n[~] f\n[/] g";
    let states: Vec<_> = parse_todos(text).iter().map(|i| i.state).collect();
    assert_eq!(states, vec![Open, Done, Done, Migrated, Scheduled, Dropped, Delegated]);
}

#[test]
fn non_todo_lines_are_ignored() {
    let text = "Shopping today\n[ ] milk\njust a note\n[?] unknown box\n[x]glued\n- plain bullet";
    let items = parse_todos(text);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].text, "milk");
    assert!(parse_todos("").is_empty());
}

#[test]
fn bullets_before_the_box_are_allowed() {
    let items = parse_todos("- [ ] dash\n* [x] star");
    assert_eq!(items.iter().map(|i| i.text.as_str()).collect::<Vec<_>>(), vec!["dash", "star"]);
    assert_eq!(items[1].state, Done);
}

#[test]
fn empty_box_is_an_item_with_empty_text() {
    let item = one("[ ]");
    assert_eq!(item.text, "");
    assert_eq!(item.marker_span, 0..3);
}

#[test]
fn all_tokens_are_read_and_removed_from_the_text() {
    let src = "[ ] call the bank ^fri ^^mon,wed !! &phone ~1h30m @sam";
    let item = one(src);
    assert_eq!(item.text, "call the bank");
    let meta = &item.meta;
    assert_eq!(meta.due.as_deref(), Some("fri"));
    assert_eq!(meta.repeat.as_deref(), Some("mon,wed"));
    assert_eq!(meta.priority, Some(2));
    assert_eq!(meta.contexts, vec!["phone".to_string()]);
    assert_eq!(meta.estimate_minutes, Some(90));
    assert_eq!(meta.delegate.as_deref(), Some("sam"));
    let expected = vec![
        (K::Due, "^fri"),
        (K::Repeat, "^^mon,wed"),
        (K::Priority, "!!"),
        (K::Context, "&phone"),
        (K::Estimate, "~1h30m"),
        (K::Delegate, "@sam"),
    ];
    assert_eq!(token_texts(src, &item), expected);
}

#[test]
fn due_phrases_stay_raw() {
    assert_eq!(one("[ ] a ^2026-12-01").meta.due.as_deref(), Some("2026-12-01"));
    assert_eq!(one("[ ] a ^+3d").meta.due.as_deref(), Some("+3d"));
    assert_eq!(one("[ ] a ^^15th").meta.repeat.as_deref(), Some("15th"));
    assert_eq!(one("[ ] a ^^daily").meta.due, None);
}

#[test]
fn tokens_inside_words_are_text() {
    let src = "[ ] email x@y.z about a&b and a^b wow! ok~5m";
    let item = one(src);
    assert!(item.tokens.is_empty(), "{:?}", item.tokens);
    assert_eq!(item.text, "email x@y.z about a&b and a^b wow! ok~5m");
}

#[test]
fn malformed_tokens_are_text() {
    let src = "[ ] ^ ^^ & @ ~ !!!! ~5 ~1.5h ~0m ~5x &-x @.a ^fri. ^^^x";
    let item = one(src);
    assert!(item.tokens.is_empty(), "{:?}", item.tokens);
    assert_eq!(item.text, "^ ^^ & @ ~ !!!! ~5 ~1.5h ~0m ~5x &-x @.a ^fri. ^^^x");
}

#[test]
fn standalone_bangs_are_priority_levels() {
    assert_eq!(one("[ ] a !").meta.priority, Some(1));
    assert_eq!(one("[ ] !!! a").meta.priority, Some(3));
    assert_eq!(one("[ ] a !!").text, "a");
}

#[test]
fn first_single_value_wins_and_later_ones_stay_text() {
    let item = one("[ ] a ^fri ^sat ! !!! @ann @bob ~5m ~1h");
    assert_eq!(item.meta.due.as_deref(), Some("fri"));
    assert_eq!(item.meta.priority, Some(1));
    assert_eq!(item.meta.delegate.as_deref(), Some("ann"));
    assert_eq!(item.meta.estimate_minutes, Some(5));
    assert_eq!(item.text, "a ^sat !!! @bob ~1h");
}

#[test]
fn contexts_collect_without_duplicates() {
    let item = one("[ ] a &home &work &home");
    assert_eq!(item.meta.contexts, vec!["home".to_string(), "work".to_string()]);
    assert_eq!(item.tokens.len(), 3);
    assert_eq!(item.text, "a");
}

#[test]
fn estimates_read_minutes_and_hours() {
    assert_eq!(parse_estimate("25m"), Some(25));
    assert_eq!(parse_estimate("2h"), Some(120));
    assert_eq!(parse_estimate("1h30m"), Some(90));
    assert_eq!(parse_estimate("0h45m"), Some(45));
    for bad in ["", "h", "m", "30", "1h30", "30m1h", "1.5h", "0m", "99999999999h", "-5m"] {
        assert_eq!(parse_estimate(bad), None, "{bad:?}");
    }
}

#[test]
fn spans_cover_the_line_box_and_tokens() {
    let src = "intro\r\n  - [x] done thing &home\r\nafter";
    let item = one(src);
    assert_eq!(&src[item.line_span.clone()], "  - [x] done thing &home");
    assert_eq!(&src[item.marker_span.clone()], "[x]");
    assert_eq!(token_texts(src, &item), vec![(K::Context, "&home")]);
}

#[test]
fn spans_are_byte_offsets_with_unicode_text() {
    let src = "日記 ✨\n[ ] 買い物 &家 ~20m";
    let item = one(src);
    assert_eq!(item.text, "買い物");
    assert_eq!(item.meta.contexts, vec!["家".to_string()]);
    assert_eq!(token_texts(src, &item), vec![(K::Context, "&家"), (K::Estimate, "~20m")]);
    assert_eq!(&src[item.line_span.clone()], "[ ] 買い物 &家 ~20m");
}

#[test]
fn indentation_nests_items() {
    let src = "[ ] trip\n  [x] book train\n  [ ] pack\n    [ ] socks\n[ ] after";
    let items = parse_todos(src);
    assert_eq!(items.len(), 2);
    let trip = &items[0];
    assert_eq!(trip.children.iter().map(|c| c.text.as_str()).collect::<Vec<_>>(), vec!["book train", "pack"]);
    assert_eq!(trip.children[1].children[0].text, "socks");
    let order: Vec<_> = flatten_todos(&items).iter().map(|i| i.text.as_str()).collect();
    assert_eq!(order, vec!["trip", "book train", "pack", "socks", "after"]);
}

#[test]
fn other_lines_do_not_break_nesting() {
    let src = "[ ] parent\nsome note in between\n\n  [ ] child";
    let items = parse_todos(src);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].children[0].text, "child");
}

#[test]
fn dedent_jumps_attach_to_the_nearest_shallower_item() {
    let src = "[ ] a\n      [ ] deep\n  [ ] mid\n\t[ ] tab child";
    let items = parse_todos(src);
    let texts: Vec<_> = items[0].children.iter().map(|c| c.text.as_str()).collect();
    assert_eq!(texts, vec!["deep", "mid", "tab child"]);
}

#[test]
fn indented_first_item_is_a_root() {
    let items = parse_todos("    [ ] starts deep\n[ ] root");
    assert_eq!(items.len(), 2);
}

#[test]
fn lines_inside_code_fences_are_not_items() {
    let src = "[ ] real\n```\n[ ] in code\n```\n[ ] also real";
    let texts: Vec<_> = parse_todos(src).iter().map(|i| i.text.clone()).collect();
    assert_eq!(texts, vec!["real", "also real"]);
}

#[test]
fn whitespace_between_words_is_collapsed() {
    assert_eq!(one("[ ]   lots \t of   space  ").text, "lots of space");
}

mod syntax;
mod due;
mod repeat;
mod entries;
