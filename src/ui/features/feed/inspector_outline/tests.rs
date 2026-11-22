use super::*;
use crate::domain::{Post, PostId};
use std::collections::BTreeSet;

/// root ─┬─ a ─── a1
///       └─ b
fn posts() -> Vec<Post> {
    let post = |id: &str, parent: Option<&str>, text: &str| {
        let mut post = Post::create(id, "net", "author", text).expect("valid post");
        post.reply_to_id = parent.map(PostId::new);
        post
    };
    vec![post("root", None, "Root  thought\nwith lines"), post("a", Some("root"), "A"), post("a1", Some("a"), "A one"), post("b", Some("root"), "B")]
}

fn tree_of(posts: &[Post]) -> OutlineNode {
    let replies = |id: &PostId| posts.iter().filter(|p| p.reply_to_id.as_ref() == Some(id)).collect::<Vec<_>>();
    build_outline(&posts[0], &replies, &|post: &Post| format!("@{}", post.id))
}

fn ids(rows: &[OutlineRow]) -> Vec<String> {
    rows.iter().map(|row| row.id.to_string()).collect()
}

fn id(raw: &str) -> PostId {
    PostId::new(raw)
}

#[test]
fn the_tree_nests_replies_and_flattens_text() {
    let posts = posts();
    let tree = tree_of(&posts);
    assert_eq!(tree.text, "Root thought with lines");
    assert_eq!(tree.descendants(), 3);
    assert_eq!(tree.branch_ids(), BTreeSet::from([id("root"), id("a")]));
    let rows = visible_rows(&tree, &BTreeSet::new());
    assert_eq!(ids(&rows), ["root", "a", "a1", "b"]);
    assert_eq!(rows.iter().map(|r| r.depth).collect::<Vec<_>>(), [0, 1, 2, 1]);
    assert_eq!(rows[2].parent, Some(id("a")));
}

#[test]
fn the_root_is_found_from_any_reply_even_with_a_cycle() {
    let mut posts = posts();
    let lookup = |posts: &[Post], raw: &PostId| posts.iter().position(|p| &p.id == raw);
    assert_eq!(thread_root(&posts[2], |pid| lookup(&posts, pid).map(|i| &posts[i])).id, id("root"));
    posts[0].reply_to_id = Some(id("a1"));
    let found = thread_root(&posts[2], |pid| lookup(&posts, pid).map(|i| &posts[i]));
    assert!(["root", "a", "a1"].contains(&found.id.as_str()), "stops on a cycle");
}

#[test]
fn folding_hides_branches_and_counts_them() {
    let posts = posts();
    let tree = tree_of(&posts);
    let rows = visible_rows(&tree, &BTreeSet::from([id("a")]));
    assert_eq!(ids(&rows), ["root", "a", "b"]);
    assert!(rows[1].collapsed && rows[1].hidden == 1);
    let leaf_folded = visible_rows(&tree, &BTreeSet::from([id("b")]));
    assert!(!leaf_folded[3].collapsed, "a leaf never folds");
}

#[test]
fn toggle_collapse_all_and_expand_all() {
    let posts = posts();
    let tree = tree_of(&posts);
    let (thread, _) = apply_command(OutlineThread::default(), &tree, &OutlineCommand::Toggle(id("a")));
    assert!(thread.collapsed.contains(&id("a")));
    let (thread, _) = apply_command(thread, &tree, &OutlineCommand::Toggle(id("a")));
    assert!(thread.collapsed.is_empty());
    let (all, _) = apply_command(thread, &tree, &OutlineCommand::CollapseAll);
    assert_eq!(ids(&visible_rows(&tree, &all.collapsed)), ["root"]);
    let (none, _) = apply_command(all, &tree, &OutlineCommand::ExpandAll);
    assert!(none.collapsed.is_empty());
}

#[test]
fn arrows_move_fold_and_unfold() {
    let posts = posts();
    let tree = tree_of(&posts);
    let key = |thread: OutlineThread, key: OutlineKey| apply_command(thread, &tree, &OutlineCommand::Key(key)).0;
    let start = OutlineThread { cursor: Some(id("root")), ..OutlineThread::default() };
    let down = key(key(start.clone(), OutlineKey::Down), OutlineKey::Down);
    assert_eq!(down.cursor, Some(id("a1")));
    let up_to_parent = key(down, OutlineKey::Left);
    assert_eq!(up_to_parent.cursor, Some(id("a")), "a leaf moves to its parent");
    let folded = key(up_to_parent, OutlineKey::Left);
    assert!(folded.collapsed.contains(&id("a")));
    let unfolded = key(folded, OutlineKey::Right);
    assert!(unfolded.collapsed.is_empty());
    assert_eq!(key(unfolded, OutlineKey::Right).cursor, Some(id("a1")), "into the first reply");
    assert_eq!(key(start.clone(), OutlineKey::End).cursor, Some(id("b")));
    assert_eq!(key(key(start, OutlineKey::Up), OutlineKey::Home).cursor, Some(id("root")));
}

#[test]
fn enter_and_double_click_open_the_post() {
    let posts = posts();
    let tree = tree_of(&posts);
    let at_b = OutlineThread { cursor: Some(id("b")), ..OutlineThread::default() };
    assert_eq!(apply_command(at_b, &tree, &OutlineCommand::Key(OutlineKey::Enter)).1, Some(id("b")));
    assert_eq!(apply_command(OutlineThread::default(), &tree, &OutlineCommand::Open(id("a1"))).1, Some(id("a1")));
}

#[test]
fn memory_keeps_each_thread_apart_and_modes_round_trip() {
    let folded = OutlineThread { collapsed: BTreeSet::from([id("a")]), cursor: None };
    let memory = OutlineMemory::default().with_thread(id("root"), folded.clone());
    assert_eq!(memory.thread(&id("root")), folded);
    assert_eq!(memory.thread(&id("other")), OutlineThread::default());
    assert_eq!(InspectorMode::from_key(InspectorMode::Outline.key()), Some(InspectorMode::Outline));
    assert_eq!(InspectorMode::default().toggled(), InspectorMode::Outline);
}

#[test]
fn a_tinted_post_carries_its_tint_to_its_bullet() {
    use crate::domain::post::Tint;
    let mut posts = posts();
    posts[1].extras.tint = Some(Tint::Mint);
    let rows = visible_rows(&tree_of(&posts), &BTreeSet::new());
    assert_eq!(rows.iter().map(|row| row.tint).collect::<Vec<_>>(), [None, Some(Tint::Mint), None, None]);
}

#[test]
fn a_post_with_symbols_shows_its_primary_symbol_on_its_bullet() {
    use crate::domain::symbols::{EntrySymbol, PostSymbols, SymbolArt, SymbolGlyph};
    let mut posts = posts();
    posts[2].extras.symbols = PostSymbols::entry(EntrySymbol::Task);
    let rows = visible_rows(&tree_of(&posts), &BTreeSet::new());
    let task = SymbolGlyph::Core(SymbolArt::Entry(EntrySymbol::Task, Some(crate::domain::symbols::EntryState::Open)));
    assert_eq!(rows.iter().map(|row| row.symbol.clone()).collect::<Vec<_>>(), [None, None, Some(task), None]);
}
