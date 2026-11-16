//! Interactive Mind Graph: a pan/zoom canvas over the cached force-directed scene of the
//! active network, plus the paint helpers the sidebar mini graph reuses.

mod details;
mod geometry;
mod header;
mod input;
mod labels;
mod legend;
pub(crate) mod paint;
mod panel;
mod render;
mod strokes;
pub(crate) mod viewport;
#[cfg(test)]
mod tests;

pub use panel::{MindGraphEvent, MindGraphPanel};
