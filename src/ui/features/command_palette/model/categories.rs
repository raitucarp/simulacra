//! M1: the Commands group gathered under its category headings (Navigate, Create, Post…).
//! While searching, the categories follow their best result; with an empty query they follow
//! the registry's order. Results keep their order within a category, so the keyboard walks
//! them top to bottom as drawn.

use super::items::{PaletteGroup, PaletteItem, PaletteTarget};
use super::rank::{RankedItem, ResultGroup};
use crate::ui::features::command_palette::registry::CommandCategory;

/// The category heading `item` is listed under (commands only).
pub fn category_of(item: &PaletteItem) -> Option<CommandCategory> {
    match item.target {
        PaletteTarget::Command(id) => Some(id.category()),
        _ => None,
    }
}

/// `groups` with the Commands group's results gathered by category; `ranked` orders the
/// categories by their best result instead of the registry's order.
pub fn by_category(groups: Vec<ResultGroup>, items: &[PaletteItem], ranked: bool) -> Vec<ResultGroup> {
    groups
        .into_iter()
        .map(|group| match group.group {
            PaletteGroup::Commands => ResultGroup { items: gathered(group.items, items, ranked), ..group },
            _ => group,
        })
        .collect()
}

fn gathered(results: Vec<RankedItem>, items: &[PaletteItem], ranked: bool) -> Vec<RankedItem> {
    let category = |result: &RankedItem| category_of(&items[result.index]);
    let mut order: Vec<Option<CommandCategory>> = Vec::new();
    for result in &results {
        if !order.contains(&category(result)) {
            order.push(category(result));
        }
    }
    if !ranked {
        order.sort();
    }
    let mut out = Vec::with_capacity(results.len());
    for heading in order {
        out.extend(results.iter().filter(|result| category(result) == heading).cloned());
    }
    out
}
