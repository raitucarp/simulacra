//! The turn-taking permutation of a debate.
//!
//! Each author is mapped to the author they reply to most often (ties to the smallest index).
//! That is a function `f` on the authors who reply; following it from any author eventually
//! enters a cycle, and on the authors lying on cycles `f` is a bijection, a permutation of
//! that set. Its cycle notation says how the floor rotates: `(a b)` is a duel, `(a b c)` a
//! round, `(a)` an author answering themselves.

use std::collections::BTreeMap;

/// The recurrent permutation of the "answers most" map.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rotation {
    /// Disjoint cycles, each starting at its smallest author, sorted by that author.
    pub cycles: Vec<Vec<usize>>,
    /// Least common multiple of the cycle lengths: how many turns until everyone is back.
    pub order: u64,
    /// Whether the permutation is even (an even number of transpositions).
    pub even: bool,
    /// Authors who feed into a cycle without lying on one, ascending.
    pub transient: Vec<usize>,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// The map each author → most answered author, from `(replier, replied_to)` pairs.
fn answers_most(replies: &[(usize, usize)]) -> BTreeMap<usize, usize> {
    let mut tally: BTreeMap<usize, BTreeMap<usize, usize>> = BTreeMap::new();
    replies.iter().for_each(|&(from, to)| *tally.entry(from).or_default().entry(to).or_default() += 1);
    tally
        .into_iter()
        .filter_map(|(from, targets)| {
            let top = targets.values().copied().max()?;
            targets.into_iter().find(|&(_, c)| c == top).map(|(to, _)| (from, to))
        })
        .collect()
}

/// The rotation of `replies`, `(replier, replied_to)` author index pairs.
pub fn rotation(replies: &[(usize, usize)]) -> Rotation {
    let map = answers_most(replies);
    let on_cycle = |start: usize| {
        let mut at = start;
        for _ in 0..=map.len() {
            match map.get(&at) {
                Some(&next) if next == start => return true,
                Some(&next) => at = next,
                None => return false,
            }
        }
        false
    };
    let recurrent: Vec<usize> = map.keys().copied().filter(|&a| on_cycle(a)).collect();
    let mut seen = std::collections::BTreeSet::new();
    let mut cycles = Vec::new();
    for &start in &recurrent {
        if !seen.insert(start) {
            continue;
        }
        let mut cycle = vec![start];
        let mut at = map[&start];
        while at != start {
            seen.insert(at);
            cycle.push(at);
            at = map[&at];
        }
        cycles.push(cycle);
    }
    let order = cycles.iter().fold(1u64, |acc, c| acc / gcd(acc, c.len() as u64) * c.len() as u64);
    let transpositions: usize = cycles.iter().map(|c| c.len() - 1).sum();
    let transient = map.keys().copied().filter(|a| !seen.contains(a)).collect();
    Rotation { cycles, order, even: transpositions % 2 == 0, transient }
}
