//! Named literals of the application shell: column sizes, element ids and placeholders.

/// Pixels of stream rows rendered beyond the viewport so scrolling never pops in.
pub(super) const STREAM_LIST_OVERDRAW: f32 = 600.0;

/// Width of the left sidebar in its collapsed icon-rail mode.
pub(super) const LEFT_SIDEBAR_COLLAPSED_WIDTH: f32 = 64.0;
/// Resize range of the collapsed left sidebar.
pub(super) const LEFT_SIDEBAR_COLLAPSED_MIN: f32 = 60.0;
pub(super) const LEFT_SIDEBAR_COLLAPSED_MAX: f32 = 68.0;
/// Width of the expanded left sidebar.
pub(super) const LEFT_SIDEBAR_EXPANDED_WIDTH: f32 = 240.0;
/// Resize range of the expanded left sidebar.
pub(super) const LEFT_SIDEBAR_EXPANDED_MIN: f32 = 180.0;
pub(super) const LEFT_SIDEBAR_EXPANDED_MAX: f32 = 380.0;
/// Width of the right sidebar.
pub(super) const RIGHT_SIDEBAR_WIDTH: f32 = 320.0;
/// Resize range of the right sidebar.
pub(super) const RIGHT_SIDEBAR_MIN: f32 = 260.0;
pub(super) const RIGHT_SIDEBAR_MAX: f32 = 460.0;

/// Element id of the three-column layout while the left sidebar is collapsed.
pub(super) const LAYOUT_ID_COLLAPSED: &str = "simulacra-main-layout-collapsed";
/// Element id of the three-column layout while the left sidebar is expanded.
pub(super) const LAYOUT_ID_EXPANDED: &str = "simulacra-main-layout-expanded";
/// Element ids of the two-column layout while the Agent Forge is open.
pub(super) const LAYOUT_ID_FORGE_COLLAPSED: &str = "simulacra-forge-layout-collapsed";
pub(super) const LAYOUT_ID_FORGE_EXPANDED: &str = "simulacra-forge-layout-expanded";

/// Auto-grow row ranges `(min, max)` of the shell's text areas.
pub(super) const COMPOSER_ROWS: (usize, usize) = (1, 5);
pub(super) const DOC_CONTENT_ROWS: (usize, usize) = (3, 8);

/// Placeholders of the LLM settings inputs.
pub(super) const LLM_API_KEY_PLACEHOLDER: &str = "Enter Gemini API Key (AIzaSy...)";
pub(super) const LLM_MODEL_PLACEHOLDER: &str = "Model Identifier (e.g. gemini-1.5-flash)";
pub(super) const LLM_ENDPOINT_PLACEHOLDER: &str = "http://localhost:8000/v1/chat/completions";
