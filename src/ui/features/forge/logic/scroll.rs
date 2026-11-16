//! Scroll identity of the options panel: keyed by step, so every step opens scrolled to the top.

use crate::state::forge::ForgeStep;
use gpui_kit::ElementId;

/// Prefix of the options panel's per-step scroll id.
const OPTIONS_SCROLL_PREFIX: &str = "forge-options-scroll";

/// The options panel's scroll id on `step`. The kit keeps a scroll position per id (dropped
/// once the id stops rendering), so a new step starts at the top instead of inheriting the
/// previous step's offset.
pub fn options_scroll_id(step: ForgeStep) -> ElementId {
    ElementId::named_usize(OPTIONS_SCROLL_PREFIX, step.index())
}
