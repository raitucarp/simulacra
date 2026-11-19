//! What the pointer is over, and which legends can hide series.

/// The hovered element of the open view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Hover {
    #[default]
    None,
    /// A node of the Network view.
    Node(usize),
    /// A point of the embedding map.
    Point(usize),
    /// A cell of the weekly rhythm: weekday, hour.
    Rhythm(usize, usize),
    /// An axis of the trait radar.
    Trait(usize),
    /// A Venn region (bit mask of sets).
    Region(u32),
    /// An object kind of the category diagram.
    Object(usize),
    /// A post of the drawn thread (its position in the thread).
    ThreadPost(usize),
}

/// A legend whose entries toggle series.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LegendId {
    /// Post kinds in the timeline and per-author bars.
    Kinds,
    /// Network colours (communities or node kinds).
    Network,
    /// Embedding map colours.
    Map,
    /// Agents on the trait radar.
    Radar,
}
