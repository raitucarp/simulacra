//! Pure ranking: the query, the item snapshot and the history in, ordered result groups out.

use super::history::PaletteHistory;
use super::items::{PaletteGroup, PaletteItem};
use super::query::{PaletteMode, PaletteQuery};
use crate::ui::features::command_palette::search::fuzzy_match;
use std::collections::HashMap;

/// Most results per group while searching.
const GROUP_LIMIT: usize = 6;
/// Recent and frequent results shown for an empty query.
const RECENT_LIMIT: usize = 5;
const FREQUENT_LIMIT: usize = 4;
/// Score added per past use, and its cap, so often-opened results rise.
const USE_BONUS: i32 = 3;
const MAX_USE_BONUS: i32 = 30;
/// A match found only in the keywords ranks this much below a title match.
const KEYWORD_PENALTY: i32 = 24;
/// Most skipped characters per matched one before a title match counts as noise.
const MAX_GAP_PER_CHAR: usize = 3;

/// A result: the index of its item in the snapshot and the matched title characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedItem {
    pub index: usize,
    pub positions: Vec<usize>,
}

/// A titled group of results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultGroup {
    pub group: PaletteGroup,
    pub items: Vec<RankedItem>,
}

/// The result groups for `query`; the best match comes first.
pub fn rank(items: &[PaletteItem], query: &PaletteQuery, history: &PaletteHistory) -> Vec<ResultGroup> {
    if query.mode == PaletteMode::Semantic {
        return Vec::new();
    }
    if query.is_blank() {
        return blank_groups(items, query.mode, history);
    }
    let mut scored: Vec<(i32, PaletteGroup, RankedItem)> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| in_mode(item, query.mode))
        .filter_map(|(index, item)| {
            let (score, positions) = score_item(item, &query.text, history)?;
            Some((score, item.group, RankedItem { index, positions }))
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    grouped_by_best(scored)
}

fn in_mode(item: &PaletteItem, mode: PaletteMode) -> bool {
    match mode {
        PaletteMode::Everything => true,
        PaletteMode::Commands => item.target.is_command(),
        PaletteMode::Semantic => false,
    }
}

/// Whether matched positions sit close enough together to be a real hit (scattered letters
/// across a long post are noise).
pub fn is_compact(positions: &[usize]) -> bool {
    match (positions.first(), positions.last()) {
        (Some(first), Some(last)) => last - first + 1 - positions.len() <= positions.len() * MAX_GAP_PER_CHAR,
        _ => true,
    }
}

/// Title match (highlighted, compact) or keyword match (a literal substring, lower), plus the
/// use bonus.
fn score_item(item: &PaletteItem, text: &str, history: &PaletteHistory) -> Option<(i32, Vec<usize>)> {
    let title = fuzzy_match(text, &item.title).filter(|found| is_compact(&found.positions)).map(|found| (found.score, found.positions));
    let keywords = || {
        let literal = item.keywords.to_lowercase().contains(&text.to_lowercase());
        literal.then(|| fuzzy_match(text, &item.keywords)).flatten().map(|found| (found.score - KEYWORD_PENALTY, Vec::new()))
    };
    let (score, positions) = title.or_else(keywords)?;
    let uses = history.uses(&item.target.history_key()) as i32;
    Some((score + (uses * USE_BONUS).min(MAX_USE_BONUS), positions))
}

/// Groups sorted results, ordering the groups by their best result.
fn grouped_by_best(scored: Vec<(i32, PaletteGroup, RankedItem)>) -> Vec<ResultGroup> {
    scored.into_iter().fold(Vec::new(), |mut groups: Vec<ResultGroup>, (_, group, item)| {
        match groups.iter_mut().find(|existing| existing.group == group) {
            Some(existing) if existing.items.len() < GROUP_LIMIT => existing.items.push(item),
            Some(_) => {}
            None => groups.push(ResultGroup { group, items: vec![item] }),
        }
        groups
    })
}

/// Recent, then frequent, then every screen and command.
fn blank_groups(items: &[PaletteItem], mode: PaletteMode, history: &PaletteHistory) -> Vec<ResultGroup> {
    let by_key: HashMap<String, usize> = items.iter().enumerate().filter(|(_, item)| in_mode(item, mode)).map(|(index, item)| (item.target.history_key(), index)).collect();
    let resolve = |key: &str| by_key.get(key).map(|&index| RankedItem { index, positions: Vec::new() });
    let recent: Vec<RankedItem> = history.recent().filter_map(resolve).take(RECENT_LIMIT).collect();
    let frequent: Vec<RankedItem> = history.frequent().into_iter().filter_map(resolve).filter(|item| !recent.contains(item)).take(FREQUENT_LIMIT).collect();
    let listed = |group: PaletteGroup| items.iter().enumerate().filter(|(_, item)| item.group == group).map(|(index, _)| RankedItem { index, positions: Vec::new() }).collect();
    [(PaletteGroup::Recent, recent), (PaletteGroup::Frequent, frequent), (PaletteGroup::Screens, listed(PaletteGroup::Screens)), (PaletteGroup::Commands, listed(PaletteGroup::Commands))]
        .into_iter()
        .filter(|(_, items)| !items.is_empty())
        .map(|(group, items)| ResultGroup { group, items })
        .collect()
}
