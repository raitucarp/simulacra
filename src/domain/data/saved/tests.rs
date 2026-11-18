use super::*;
use crate::domain::review::connections::MapPoint;
use chrono::TimeZone;

fn at(minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 18, 9, minute, 0).unwrap()
}

fn shelf_id() -> CollectionId {
    CollectionId::new("coll_shelf")
}

fn entry(item: ItemRef, position: i64) -> CollectionEntry {
    CollectionEntry { collection: shelf_id(), item, position, note: String::new(), added_at: at(position as u32) }
}

fn post(id: &str) -> ItemRef {
    ItemRef::Post(PostId::new(id))
}

fn doc(id: &str) -> ItemRef {
    ItemRef::Document(DocumentId::new(id))
}

fn ids(items: &[CollectionEntry]) -> Vec<&str> {
    items.iter().map(|e| e.item.id()).collect()
}

#[test]
fn item_refs_round_trip_their_stored_parts() {
    for item in [post("post_1"), doc("doc_1")] {
        assert_eq!(ItemRef::from_parts(item.kind(), item.id()), Some(item.clone()));
    }
    assert_eq!(ItemRef::from_parts("thread", "x"), None);
}

#[test]
fn collection_items_follow_positions_and_append_unfiled_documents() {
    let entries = vec![entry(post("p2"), 1), entry(doc("d1"), 0), entry(post("p1"), 2)];
    let members = vec![DocumentId::new("d1"), DocumentId::new("d9")];
    let items = collection_items(&shelf_id(), &entries, &members, &|_| true, at(30));
    assert_eq!(ids(&items), vec!["d1", "p2", "p1", "d9"]);
    assert_eq!(items[3].position, 3);
}

#[test]
fn collection_items_drop_missing_posts_and_unfiled_documents() {
    let entries = vec![entry(post("gone"), 0), entry(doc("moved"), 1), entry(post("kept"), 2)];
    let exists = |item: &ItemRef| item.id() != "gone";
    let items = collection_items(&shelf_id(), &entries, &[], &exists, at(30));
    assert_eq!(ids(&items), vec!["kept"]);
}

#[test]
fn collection_items_ignore_other_collections() {
    let other = CollectionEntry { collection: CollectionId::new("coll_other"), ..entry(post("x"), 0) };
    assert!(collection_items(&shelf_id(), &[other], &[], &|_| true, at(1)).is_empty());
}

#[test]
fn moving_an_item_renumbers_every_position() {
    let items = vec![entry(post("a"), 0), entry(post("b"), 5), entry(post("c"), 9)];
    let order = moved(&items, 2, 0);
    assert_eq!(ids(&order), vec!["c", "a", "b"]);
    assert_eq!(order.iter().map(|e| e.position).collect::<Vec<_>>(), vec![0, 1, 2]);
    assert_eq!(ids(&moved(&items, 0, 99)), vec!["b", "c", "a"]);
    assert_eq!(ids(&moved(&items, 7, 0)), vec!["a", "b", "c"]);
}

#[test]
fn next_position_follows_the_last_item() {
    assert_eq!(next_position(&[], &shelf_id()), 0);
    assert_eq!(next_position(&[entry(post("a"), 4), entry(post("b"), 2)], &shelf_id()), 5);
}

#[test]
fn bookmarks_toggle_newest_first() {
    let first = toggled_bookmark(&[], &PostId::new("a"), at(1));
    let both = toggled_bookmark(&first, &PostId::new("b"), at(2));
    assert_eq!(both.iter().map(|b| b.post.as_str()).collect::<Vec<_>>(), vec!["b", "a"]);
    let shelf = SavedShelf { bookmarks: both.clone(), entries: Vec::new() };
    assert!(shelf.is_bookmarked(&PostId::new("a")));
    let removed = toggled_bookmark(&both, &PostId::new("a"), at(3));
    assert_eq!(removed.len(), 1);
    assert_eq!(removed[0].post.as_str(), "b");
}

#[test]
fn shelf_lists_the_collections_of_an_item() {
    let shelf = SavedShelf { bookmarks: Vec::new(), entries: vec![entry(post("a"), 0)] };
    assert_eq!(shelf.collections_of(&post("a")), vec![shelf_id()]);
    assert!(shelf.collections_of(&post("b")).is_empty());
    assert_eq!(shelf.entries_of(&shelf_id()).count(), 1);
}

#[test]
fn a_collection_becomes_a_chained_connection_map() {
    let items = vec![entry(post("a"), 0), entry(doc("d"), 1), entry(post("b"), 2)];
    let map = collection_map("Shelf".into(), &items);
    assert_eq!(map.points.len(), 3);
    assert!(matches!(&map.points[1], MapPoint::Passage { document_id, .. } if document_id == "d"));
    assert_eq!(map.edges.iter().map(|e| (e.from, e.to, e.directed)).collect::<Vec<_>>(), vec![(0, 1, true), (1, 2, true)]);
    let many: Vec<CollectionEntry> = (0..40).map(|i| entry(post(&format!("p{i}")), i)).collect();
    assert_eq!(collection_map("Many".into(), &many).points.len(), crate::domain::review::connections::MAX_POINTS);
}
