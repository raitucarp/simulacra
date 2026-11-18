//! Thinking tools (sb E1), UI side: the inspector's Argument view, the credence chip and its
//! popover on post cards, the Calibration and Tensions views of the Insights page, and the
//! atomic-note hint in the composer with its "Check atomicity" sheet.
//!
//! The logic is pure in `domain::thinking`; the shell (`shell::thinking_host` and the
//! `thinking_*` handlers) loads and saves rows and runs AI requests through the post actions
//! runner.

pub mod argument;
pub mod atomic;
pub mod credence;
pub mod insights;
pub mod labels;
