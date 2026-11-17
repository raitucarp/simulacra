//! Right sidebar feature: unified Mind & Cadence / Vault & Telemetry panel, the
//! secondary and legacy sidebars, and their cards (knowledge, topics, roster, agents).

pub mod dot_graph;
pub mod heatmap;
mod heatmap_chrome;
mod heatmap_geometry;
mod heatmap_grid;
pub mod knowledge_card;
pub mod roster;
pub mod secondary;
mod section;
pub mod spark_feed;
pub mod topic_item;
mod trending;
pub mod unified;

#[cfg(test)]
mod tests;

pub use knowledge_card::render_knowledge_card;
pub use roster::render_network_agent_roster;
pub use topic_item::render_topic_item;
pub use unified::{render_unified_right_sidebar, RightSidebarTab, UnifiedRightSidebarParams};
