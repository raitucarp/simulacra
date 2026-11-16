//! What Escape does inside the forge: close open hint cards first, else leave a focused text
//! field (so typing never loses the whole forge), else close the forge.

use gpui_kit::KeyContext;

/// Key context of gpui-component's text inputs and areas (their Escape propagates to us
/// when they have nothing of their own to dismiss).
pub const TEXT_INPUT_KEY_CONTEXT: &str = "Input";

/// The effect of one Escape press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissStep {
    /// Hint cards were open and are now closed; nothing else happens.
    CloseHints,
    /// A text field had focus: move focus back to the forge, keep it open.
    LeaveTextField,
    /// Leave the forge.
    CloseForge,
}

/// The step for an Escape press, given whether it just closed hint cards and the key
/// contexts on the focus path (innermost last).
pub fn dismiss_step(closed_hints: bool, focus_contexts: &[KeyContext]) -> DismissStep {
    if closed_hints {
        DismissStep::CloseHints
    } else if focus_contexts.iter().any(|context| context.contains(TEXT_INPUT_KEY_CONTEXT)) {
        DismissStep::LeaveTextField
    } else {
        DismissStep::CloseForge
    }
}
