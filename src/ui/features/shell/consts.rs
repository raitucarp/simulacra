//! Named literals of the application shell: list overdraw, element ids and placeholders.
//! Column sizes are theme tokens (`left_sidebar_*`, `right_sidebar_*`).

/// Pixels of stream rows rendered beyond the viewport so scrolling never pops in.
pub(super) const STREAM_LIST_OVERDRAW: f32 = 600.0;

/// Element id of the three-column layout while the left sidebar is collapsed.
pub(super) const LAYOUT_ID_COLLAPSED: &str = "simulacra-main-layout-collapsed";
/// Element id of the three-column layout while the left sidebar is expanded.
pub(super) const LAYOUT_ID_EXPANDED: &str = "simulacra-main-layout-expanded";
/// Element ids of the three-column layout in the medium layout class (narrower right column).
pub(super) const LAYOUT_ID_MEDIUM_COLLAPSED: &str = "simulacra-main-layout-medium-collapsed";
pub(super) const LAYOUT_ID_MEDIUM_EXPANDED: &str = "simulacra-main-layout-medium-expanded";
/// Element ids of the two-column layout while the Agent Forge is open.
pub(super) const LAYOUT_ID_FORGE_COLLAPSED: &str = "simulacra-forge-layout-collapsed";
pub(super) const LAYOUT_ID_FORGE_EXPANDED: &str = "simulacra-forge-layout-expanded";

/// Auto-grow row ranges `(min, max)` of the shell's text areas.
pub(super) const COMPOSER_ROWS: (usize, usize) = (1, 5);
pub(super) const DOC_CONTENT_ROWS: (usize, usize) = (3, 8);

/// Placeholder of the custom LLM endpoint input (a URL, not translated).
pub(super) const LLM_ENDPOINT_PLACEHOLDER: &str = "http://localhost:8000/v1/chat/completions";
