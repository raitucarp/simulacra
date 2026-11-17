//! Pure keyboard navigation over the flattened result list.

use super::rank::ResultGroup;

/// Number of selectable results.
pub fn result_count(groups: &[ResultGroup]) -> usize {
    groups.iter().map(|group| group.items.len()).sum()
}

/// The selection moved by `delta`, wrapping around; zero when there are no results.
pub fn stepped(selected: usize, delta: isize, count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    (selected as isize + delta).rem_euclid(count as isize) as usize
}

/// Flat index of the first result of every group.
fn group_starts(groups: &[ResultGroup]) -> Vec<usize> {
    groups.iter().scan(0, |start, group| {
        let this = *start;
        *start += group.items.len();
        Some(this)
    }).collect()
}

/// First result of the group after the selected one (wrapping to the first group).
pub fn next_group_start(groups: &[ResultGroup], selected: usize) -> usize {
    let starts = group_starts(groups);
    starts.iter().copied().find(|&start| start > selected).unwrap_or(0)
}

/// First result of the selected group, or of the previous group when already on it.
pub fn previous_group_start(groups: &[ResultGroup], selected: usize) -> usize {
    let starts = group_starts(groups);
    let current = starts.iter().copied().filter(|&start| start <= selected).last().unwrap_or(0);
    if current < selected {
        return current;
    }
    starts.iter().copied().filter(|&start| start < current).last().or_else(|| starts.last().copied()).unwrap_or(0)
}

/// The (group, item) position of a flat index.
pub fn locate(groups: &[ResultGroup], flat: usize) -> Option<(usize, usize)> {
    let starts = group_starts(groups);
    groups.iter().zip(starts).enumerate().find_map(|(group_index, (group, start))| {
        (flat >= start && flat < start + group.items.len()).then(|| (group_index, flat - start))
    })
}
