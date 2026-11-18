//! Exclusive regions of a family of sets: each element belongs to exactly one region, named
//! by the bit mask of the sets that contain it (bit `i` = set `i`). A Venn diagram labels each
//! of its `2^k − 1` regions with these counts; an UpSet matrix lists the non-empty ones.

use super::NamedSet;
use std::collections::BTreeMap;

/// Most sets a mask can address.
pub const MAX_SETS: usize = 16;

/// Elements in exactly the sets of `mask`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub mask: u32,
    pub members: Vec<u32>,
}

impl Region {
    /// Indices of the sets in the mask, ascending.
    pub fn sets(&self) -> Vec<usize> {
        (0..MAX_SETS).filter(|i| self.mask & (1 << i) != 0).collect()
    }

    /// How many sets the region lies in.
    pub fn degree(&self) -> u32 {
        self.mask.count_ones()
    }
}

/// Every non-empty exclusive region of the first [`MAX_SETS`] sets, by mask.
pub fn exclusive_regions(sets: &[NamedSet]) -> Vec<Region> {
    let mut masks: BTreeMap<u32, u32> = BTreeMap::new();
    for (i, set) in sets.iter().take(MAX_SETS).enumerate() {
        set.members.iter().for_each(|&m| *masks.entry(m).or_default() |= 1 << i);
    }
    let mut regions: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    masks.into_iter().for_each(|(member, mask)| regions.entry(mask).or_default().push(member));
    regions.into_iter().map(|(mask, members)| Region { mask, members }).collect()
}

/// The `limit` largest regions for an UpSet matrix (ties: fewer sets first, then by mask).
pub fn upset_rows(sets: &[NamedSet], limit: usize) -> Vec<Region> {
    let mut regions = exclusive_regions(sets);
    regions.sort_by_key(|r| (std::cmp::Reverse(r.members.len()), r.degree(), r.mask));
    regions.truncate(limit);
    regions
}
