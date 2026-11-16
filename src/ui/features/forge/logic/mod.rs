//! Pure, render-free decisions behind the Agent Forge screens, kept apart so they can be
//! unit-tested without a window: step-rail marks, the save bar, the autosave cadence,
//! Mnemo's pose, the option-grid keyboard cursor, slider and active-hours keys, portrait slot
//! edits, score bands, the options panel's per-step scroll id and what Escape dismisses.

mod autosave;
mod bands;
mod dismiss;
mod focus;
mod guide;
mod keys;
mod occupation;
mod portrait_edit;
mod rail;
mod save_bar;
mod scroll;
mod seed;

pub use autosave::{AutosavePolicy, AUTOSAVE_EVERY_EDITS};
pub use bands::{score_level_key, ScoreBand, BAND_THRESHOLDS};
pub use dismiss::{dismiss_step, DismissStep, TEXT_INPUT_KEY_CONTEXT};
pub use focus::{GridCursor, NavKey, ACTIVATE_KEYS};
pub use guide::{guide_pose, guide_tip_key, GuideMoment};
pub use keys::{hour_key_change, slider_key_value, PAGE_NOTCHES};
pub use occupation::OccupationPreset;
pub use portrait_edit::{randomized_slot, slot_index, slot_len, with_slot_index};
pub use rail::{rail_items, RailItem, RailStatus};
pub use save_bar::{save_bar_state, SaveBarState};
pub use scroll::options_scroll_id;
pub use seed::time_seed;

#[cfg(test)]
mod tests;
