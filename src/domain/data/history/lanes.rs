//! Graph lanes of a version tree (V5), as a history graph draws them: versions newest first,
//! one row each, every version in a lane (a column). A version continues the lane of its
//! newest child; the lanes of its other children end at it, where those branches forked.
//! A lane frees up below the original of its line and is reused by the next branch.

/// One version of the tree: its id and the id of the version it was edited from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeNode<'a> {
    pub id: &'a str,
    pub parent: Option<&'a str>,
}

/// How one row of the tree is drawn.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LaneRow {
    /// The lane holding this version.
    pub lane: usize,
    /// Lanes of other lines passing this row.
    pub through: Vec<usize>,
    /// Lanes of branches edited from this version (they end here).
    pub forks: Vec<usize>,
    /// Whether the line continues below (the version was edited from another).
    pub continues: bool,
    /// Whether a later version continues this line above.
    pub continued: bool,
}

/// The lanes of `nodes`, given newest first (every version before the one it was edited
/// from); a parent missing from `nodes` ends its line.
pub fn lane_rows(nodes: &[TreeNode<'_>]) -> Vec<LaneRow> {
    let known: Vec<&str> = nodes.iter().map(|node| node.id).collect();
    let mut lanes: Vec<Option<&str>> = Vec::new();
    nodes.iter().map(|node| place(node, &known, &mut lanes)).collect()
}

/// Lanes in use by `rows` (the widest row).
pub fn lane_count(rows: &[LaneRow]) -> usize {
    rows.iter().flat_map(|row| row.through.iter().chain(&row.forks).chain([&row.lane])).max().map_or(0, |widest| widest + 1)
}

/// Places `node` in the lanes (each waiting for the version it is drawn down to).
fn place<'a>(node: &TreeNode<'a>, known: &[&str], lanes: &mut Vec<Option<&'a str>>) -> LaneRow {
    let waiting: Vec<usize> = (0..lanes.len()).filter(|&i| lanes[i] == Some(node.id)).collect();
    let through: Vec<usize> = (0..lanes.len()).filter(|i| lanes[*i].is_some() && !waiting.contains(i)).collect();
    let lane = waiting.first().copied().unwrap_or_else(|| free_lane(lanes));
    let forks: Vec<usize> = waiting.iter().skip(1).copied().collect();
    forks.iter().for_each(|&fork| lanes[fork] = None);
    let parent = node.parent.filter(|parent| known.contains(parent));
    lanes[lane] = parent;
    while lanes.last().is_some_and(Option::is_none) {
        lanes.pop();
    }
    LaneRow { lane, through, forks, continues: parent.is_some(), continued: !waiting.is_empty() }
}

/// The first free lane, opened when every lane is in use.
fn free_lane(lanes: &mut Vec<Option<&str>>) -> usize {
    lanes.iter().position(Option::is_none).unwrap_or_else(|| {
        lanes.push(None);
        lanes.len() - 1
    })
}

/// What one lane draws on one row: the line above and below its middle, a dot in it, and the
/// connector to its left and right (a fork joining the version's lane).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LaneCell {
    pub up: bool,
    pub down: bool,
    pub dot: bool,
    pub left: bool,
    pub right: bool,
}

/// How `row` draws lane `lane`: a fork's connector spans the lanes between it and the
/// version's lane (the left half of the far end, the right half of the near one).
pub fn lane_cell(row: &LaneRow, lane: usize) -> LaneCell {
    let through = row.through.contains(&lane);
    let fork = row.forks.contains(&lane);
    let span = |f: &usize| (row.lane.min(*f), row.lane.max(*f));
    let left = row.forks.iter().map(span).any(|(low, high)| low < lane && lane <= high);
    let right = row.forks.iter().map(span).any(|(low, high)| low <= lane && lane < high);
    let own = lane == row.lane;
    LaneCell { up: through || fork || (own && row.continued), down: through || (own && row.continues), dot: own, left, right }
}
