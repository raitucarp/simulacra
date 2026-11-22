//! The name index of the downloaded icons (S15).

use crate::domain::icons::{name_tokens, SymbolIndex, SEARCH_LIMIT};
use crate::domain::symbols::SymbolRef;

fn index() -> SymbolIndex {
    let names = |list: &[&str]| list.iter().map(|name| name.to_string()).collect::<Vec<_>>();
    SymbolIndex::build([("lu", names(&["arrow-up", "zap", "arrow-up-right"])), ("fi", names(&["zap", "zoom-out", "map-pin"]))])
}

#[test]
fn names_split_into_lower_case_tokens() {
    assert_eq!(name_tokens("Arrow-Up_right2"), ["arrow", "up", "right2"]);
    assert!(name_tokens("--").is_empty());
}

#[test]
fn search_matches_token_prefixes_best_first() {
    let index = index();
    assert_eq!(index.len(), 6);
    assert!(index.search("   ", None).is_empty());
    assert_eq!(index.search("zap", None), [SymbolRef::new("fi", "zap"), SymbolRef::new("lu", "zap")], "exact names, by set");
    assert_eq!(index.search("arrow up", None), [SymbolRef::new("lu", "arrow-up"), SymbolRef::new("lu", "arrow-up-right")]);
    assert_eq!(index.search("UP", None), [SymbolRef::new("lu", "arrow-up"), SymbolRef::new("lu", "arrow-up-right")]);
    assert_eq!(index.search("pin", None), [SymbolRef::new("fi", "map-pin")], "any token may match");
    assert!(index.search("rrow", None).is_empty(), "only from the start of a token");
}

#[test]
fn a_set_filter_narrows_the_search() {
    let index = index();
    assert_eq!(index.search("zap", Some("lu")), [SymbolRef::new("lu", "zap")]);
    assert_eq!(index.search("set:fi z", None), [SymbolRef::new("fi", "zap"), SymbolRef::new("fi", "zoom-out")]);
    assert!(index.search("set:fi", None).is_empty(), "a set alone is not a query");
    let many: Vec<String> = (0..SEARCH_LIMIT + 5).map(|i| format!("dot-{i}")).collect();
    assert_eq!(SymbolIndex::build([("fi", many)]).search("dot", None).len(), SEARCH_LIMIT);
}
