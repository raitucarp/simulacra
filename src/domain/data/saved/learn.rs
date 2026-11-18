//! "Learn this collection": the collection, in its order, as a connection map for the Review
//! page (sb C1b), each item joined to the next. The learner refines the map in its editor.

use super::{CollectionEntry, ItemRef};
use crate::domain::review::connections::{ConnectionMap, MapEdge, MapPoint, MAX_POINTS};

fn point_of(item: &ItemRef) -> MapPoint {
    match item {
        ItemRef::Post(id) => MapPoint::Post { id: id.as_str().to_string() },
        ItemRef::Document(id) => MapPoint::Passage { document_id: id.as_str().to_string(), locator: None, text: String::new() },
    }
}

/// The first [`MAX_POINTS`] items as points, each directed to the one after it.
pub fn collection_map(title: String, items: &[CollectionEntry]) -> ConnectionMap {
    let points: Vec<MapPoint> = items.iter().take(MAX_POINTS).map(|entry| point_of(&entry.item)).collect();
    let edges = (1..points.len()).map(|to| MapEdge { directed: true, ..MapEdge::new(to - 1, to) }).collect();
    ConnectionMap { title, points, edges }
}
