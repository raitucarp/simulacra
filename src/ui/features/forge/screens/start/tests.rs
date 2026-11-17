//! Unit tests for the palette gallery's search.

use super::gallery::gallery_matches;
use crate::domain::palette::{Palette, PaletteCategory};

#[test]
fn a_query_searches_every_shelf_not_only_the_selected_one() {
    let target = Palette::full(Some(PaletteCategory::Creatures)).next().expect("a creature palette");
    let query = target.key.replace('_', " ");
    let other_shelf = Some(PaletteCategory::Machines);
    assert!(gallery_matches(other_shelf, &query).iter().any(|p| p.key == target.key));
    assert!(gallery_matches(other_shelf, "").iter().all(|p| p.category() == other_shelf));
    assert!(gallery_matches(None, "zzzz-no-such-palette").is_empty());
}
