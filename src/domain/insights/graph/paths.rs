//! Shortest paths by breadth-first search (unweighted, undirected): O(n + m) per search.

use super::Graph;
use std::collections::VecDeque;

/// Hop distance from `source` to every node (`None` when unreachable).
pub fn distances(graph: &Graph, source: usize) -> Vec<Option<usize>> {
    let mut distance = vec![None; graph.node_count()];
    if source >= distance.len() {
        return distance;
    }
    distance[source] = Some(0);
    let mut queue = VecDeque::from([source]);
    while let Some(v) = queue.pop_front() {
        let next = distance[v].map(|d| d + 1);
        for &w in graph.neighbours(v) {
            if distance[w as usize].is_none() {
                distance[w as usize] = next;
                queue.push_back(w as usize);
            }
        }
    }
    distance
}

/// A shortest path from `from` to `to`, both ends included (`None` when unreachable).
pub fn shortest_path(graph: &Graph, from: usize, to: usize) -> Option<Vec<usize>> {
    let n = graph.node_count();
    if from >= n || to >= n {
        return None;
    }
    let mut parent = vec![usize::MAX; n];
    parent[from] = from;
    let mut queue = VecDeque::from([from]);
    while let Some(v) = queue.pop_front() {
        if v == to {
            break;
        }
        for &w in graph.neighbours(v) {
            if parent[w as usize] == usize::MAX {
                parent[w as usize] = v;
                queue.push_back(w as usize);
            }
        }
    }
    if parent[to] == usize::MAX {
        return None;
    }
    let mut path = vec![to];
    while let Some(&last) = path.last().filter(|&&v| v != from) {
        path.push(parent[last]);
    }
    path.reverse();
    Some(path)
}

/// A lower bound of the diameter of the component of `start` by the double sweep: search from
/// `start`, then from the farthest node found; the second eccentricity is the bound (exact on
/// trees). Two searches, O(n + m).
pub fn diameter_estimate(graph: &Graph, start: usize) -> usize {
    let farthest = |source: usize| {
        distances(graph, source)
            .into_iter()
            .enumerate()
            .filter_map(|(v, d)| d.map(|d| (d, v)))
            .max_by_key(|&(d, v)| (d, std::cmp::Reverse(v)))
            .unwrap_or((0, source))
    };
    if start >= graph.node_count() {
        return 0;
    }
    let (_, end) = farthest(start);
    farthest(end).0
}
