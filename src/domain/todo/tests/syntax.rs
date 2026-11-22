//! D1: every row of the help card is typed the way the parser reads it.

use super::super::syntax::{SyntaxTopic, SYNTAX_ROWS};
use super::super::*;

/// The item `[ ] task <variant>` parses to.
fn item_with(variant: &str) -> TodoItem {
    parse_todos(&format!("[ ] task {variant}")).remove(0)
}

fn variant_sets(topic: SyntaxTopic, meta: &TodoMeta) -> bool {
    match topic {
        SyntaxTopic::Due => meta.due.is_some(),
        SyntaxTopic::Repeat => meta.repeat.is_some() && meta.due.is_none(),
        SyntaxTopic::Priority => meta.priority.is_some(),
        SyntaxTopic::Context => !meta.contexts.is_empty(),
        SyntaxTopic::Estimate => meta.estimate_minutes.is_some(),
        SyntaxTopic::Delegate => meta.delegate.is_some(),
        SyntaxTopic::Box(_) | SyntaxTopic::Nesting => false,
    }
}

#[test]
fn every_box_row_parses_to_its_state() {
    for row in SYNTAX_ROWS {
        if let SyntaxTopic::Box(state) = row.topic {
            assert_eq!(parse_todos(&format!("{} task", row.sample))[0].state, state, "{}", row.sample);
            assert_eq!(row.topic.label_key(), None);
        }
    }
}

#[test]
fn the_nesting_row_nests_under_the_item_above() {
    let row = SYNTAX_ROWS.iter().find(|row| row.topic == SyntaxTopic::Nesting).expect("nesting row");
    let items = parse_todos(&format!("[ ] parent\n{} child", row.sample));
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].children[0].text, "child");
}

#[test]
fn every_token_variant_sets_what_its_row_says() {
    let token_rows = SYNTAX_ROWS.iter().filter(|row| !matches!(row.topic, SyntaxTopic::Box(_) | SyntaxTopic::Nesting));
    for row in token_rows {
        assert!(row.topic.label_key().is_some_and(|key| key.starts_with("todo.help.")));
        for variant in row.sample.split_whitespace() {
            let item = item_with(variant);
            assert!(variant_sets(row.topic, &item.meta), "{variant} should set {:?}", row.topic);
            assert_eq!(item.text, "task", "{variant} stays out of the text");
        }
    }
}

#[test]
fn every_label_key_is_translated() {
    for key in SYNTAX_ROWS.iter().filter_map(|row| row.topic.label_key()).chain(["todo.help.title", "todo.help.open"]) {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}
