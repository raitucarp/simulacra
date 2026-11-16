//! Layout constants and element ids for the workspace trigger and popover.

/// Height of the sidebar workspace trigger button.
pub(super) const TRIGGER_HEIGHT_PX: f32 = 46.0;
/// Minimum edge length of the trigger's workspace badge.
pub(super) const TRIGGER_BADGE_MIN_PX: f32 = 28.0;
/// Popover top offset (aligned to the trigger's bottom edge).
pub(super) const POPOVER_TOP_PX: f32 = 46.0;
/// Popover left offset when the sidebar is collapsed to its icon rail.
pub(super) const POPOVER_LEFT_COLLAPSED_PX: f32 = 64.0;
/// Popover left offset when the sidebar is expanded.
pub(super) const POPOVER_LEFT_EXPANDED_PX: f32 = 12.0;
/// Popover width.
pub(super) const POPOVER_WIDTH_PX: f32 = 260.0;

/// Fallback trigger label when no network is active.
pub(super) const NO_NETWORK_LABEL: &str = "Select Domain";
/// Element id of the trigger button.
pub(super) const TRIGGER_ID: &str = "workspace-trigger-btn";
/// Element id prefix of each network row (suffixed with the row index).
pub(super) const ITEM_ID_PREFIX: &str = "ws-item-";
