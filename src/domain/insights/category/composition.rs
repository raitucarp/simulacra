//! Composition of morphisms along two-step paths, and squares.
//!
//! Every pair `x —f→ y —g→ z` (with `x ≠ z`) is a composable pair whose composite has type
//! `(f, g)`. Enumeration visits at most [`MAX_PATHS`] pairs (hubs with thousands of posts
//! would otherwise dominate), so the cost is O(m + min(P, MAX_PATHS)) for `P` two-step paths.

use super::schema::{Morphism, MorphismKind};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Composable pairs enumerated at most.
pub const MAX_PATHS: usize = 2_000_000;
/// Example squares kept.
pub const MAX_SQUARE_EXAMPLES: usize = 24;

/// One composite type `g ∘ f` and its pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Composite {
    pub first: MorphismKind,
    pub second: MorphismKind,
    pub count: usize,
    /// Pairs whose ends are also joined by a direct morphism (the composite is realised).
    pub realised: usize,
    /// One instance `(x, y, z)`.
    pub example: (u32, u32, u32),
}

/// Two different two-step paths `x → y → z` and `x → y' → z`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    pub from: u32,
    pub to: u32,
    pub via: (u32, u32),
    pub left: (MorphismKind, MorphismKind),
    pub right: (MorphismKind, MorphismKind),
}

impl Square {
    /// Whether both paths carry the same relations.
    pub fn commutes_in_type(&self) -> bool {
        self.left == self.right
    }
}

/// Composition statistics of one diagram.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompositionReport {
    /// Composite types, most frequent first.
    pub composites: Vec<Composite>,
    pub commuting_squares: usize,
    pub mixed_squares: usize,
    pub examples: Vec<Square>,
    /// Whether enumeration stopped at [`MAX_PATHS`].
    pub truncated: bool,
}

/// Composes every two-step path of `morphisms` over `count` objects.
pub fn compose(count: usize, morphisms: &[Morphism]) -> CompositionReport {
    let mut out: Vec<Vec<(u32, MorphismKind)>> = vec![Vec::new(); count];
    morphisms.iter().filter(|m| (m.source as usize) < count).for_each(|m| out[m.source as usize].push((m.target, m.kind)));
    let direct: HashSet<(u32, u32)> = morphisms.iter().map(|m| (m.source, m.target)).collect();
    let mut composites: BTreeMap<(MorphismKind, MorphismKind), Composite> = BTreeMap::new();
    let (mut report, mut visited) = (CompositionReport::default(), 0usize);
    for x in 0..count as u32 {
        let mut ends: HashMap<u32, Vec<(u32, MorphismKind, MorphismKind)>> = HashMap::new();
        for &(y, f) in &out[x as usize] {
            for &(z, g) in out.get(y as usize).map_or(&[][..], Vec::as_slice) {
                if z == x {
                    continue;
                }
                visited += 1;
                let entry = composites.entry((f, g)).or_insert(Composite { first: f, second: g, count: 0, realised: 0, example: (x, y, z) });
                entry.count += 1;
                entry.realised += usize::from(direct.contains(&(x, z)));
                ends.entry(z).or_default().push((y, f, g));
            }
        }
        collect_squares(x, ends, &mut report);
        if visited >= MAX_PATHS {
            report.truncated = true;
            break;
        }
    }
    report.composites = composites.into_values().collect();
    report.composites.sort_by_key(|c| (std::cmp::Reverse(c.count), c.first, c.second));
    report
}

/// Counts the squares from `x` (pairs of paths sharing an end) and keeps a few examples.
fn collect_squares(x: u32, ends: HashMap<u32, Vec<(u32, MorphismKind, MorphismKind)>>, report: &mut CompositionReport) {
    let mut ends: Vec<(u32, Vec<(u32, MorphismKind, MorphismKind)>)> = ends.into_iter().filter(|(_, p)| p.len() > 1).collect();
    ends.sort_by_key(|(z, _)| *z);
    for (z, paths) in ends {
        let mut by_type: BTreeMap<(MorphismKind, MorphismKind), usize> = BTreeMap::new();
        paths.iter().for_each(|&(_, f, g)| *by_type.entry((f, g)).or_default() += 1);
        let pairs = |k: usize| k * k.saturating_sub(1) / 2;
        let commuting: usize = by_type.values().map(|&k| pairs(k)).sum();
        report.commuting_squares += commuting;
        report.mixed_squares += pairs(paths.len()) - commuting;
        if report.examples.len() < MAX_SQUARE_EXAMPLES {
            let (a, b) = (paths[0], paths[1]);
            report.examples.push(Square { from: x, to: z, via: (a.0, b.0), left: (a.1, a.2), right: (b.1, b.2) });
        }
    }
}
