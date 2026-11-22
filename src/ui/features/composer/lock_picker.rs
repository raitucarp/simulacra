//! The "Lock with password" row of the composer (X3): a footer toggle opens the password,
//! typed twice, with the strength meter and its tip, and the plain warning that a forgotten
//! password cannot be recovered. Every post of the thread is sealed with it.

use super::modal::ModalActionCallback;
use crate::domain::data::snapshot::Strength;
use crate::domain::post::lock::{LockProblem, MIN_LOCK_PASSWORD_CHARS};
use crate::theme::ThemeTokens;
use crate::ui::features::data::{strength_meter, tr};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, Div, Entity, ParentElement, Styled};
use rust_i18n::t;

const BUTTON_ID: &str = "composer-lock";

/// The open lock row.
#[derive(Clone)]
pub struct LockView {
    pub password: Entity<InputState>,
    pub repeat: Entity<InputState>,
    pub strength: Strength,
    /// Why the thread cannot be locked yet; `None` when it can.
    pub problem: Option<LockProblem>,
    /// The thread is being sealed (the key derivation takes a moment).
    pub sealing: bool,
}

/// The line explaining `problem`.
pub fn problem_text(problem: LockProblem) -> String {
    match problem {
        LockProblem::TooShort => t!("composer.lock.problem.too_short", count = MIN_LOCK_PASSWORD_CHARS).to_string(),
        other => tr(&format!("composer.lock.problem.{}", other.key())),
    }
}

/// The footer's lock toggle, highlighted while the row is open.
pub(super) fn lock_button(open: bool, on_toggle: ModalActionCallback) -> Button {
    Button::new(BUTTON_ID)
        .ghost()
        .small()
        .selected(open)
        .icon(Icon::new(LucideIcon::LockKeyhole))
        .tooltip(t!("composer.lock.button").to_string())
        .on_click(move |_, window, cx| on_toggle(window, cx))
}

fn field(label: String, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    let label = div().w(theme.data_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(label);
    div().flex().items_center().gap(theme.control_gap).child(label).child(div().flex_1().child(Input::new(input).small().mask_toggle()))
}

fn line(text: String, colour: gpui_kit::Rgba, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).text_color(colour).child(text)
}

/// The row: both fields, the meter and tip, the problem (or "sealing") and the warning.
pub(super) fn lock_row(view: &LockView, theme: &ThemeTokens) -> Div {
    let status = match (view.sealing, view.problem) {
        (true, _) => Some(line(t!("composer.lock.sealing").to_string(), theme.text_secondary, theme)),
        (false, Some(problem)) => Some(line(problem_text(problem), theme.accent_danger, theme)),
        (false, None) => None,
    };
    let tip = line(tr(&format!("data.snapshot.tip.{}", view.strength.key())), theme.text_secondary, theme);
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .px(theme.panel_padding)
        .py(theme.spacing_xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(field(t!("composer.lock.password").to_string(), &view.password, theme))
        .child(strength_meter(view.strength, theme))
        .child(tip)
        .child(field(t!("composer.lock.repeat").to_string(), &view.repeat, theme))
        .children(status)
        .child(line(t!("composer.lock.unrecoverable").to_string(), theme.warning, theme))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_lock_text_is_translated() {
        let keys = ["composer.lock.button", "composer.lock.password", "composer.lock.repeat", "composer.lock.sealing", "composer.lock.unrecoverable"];
        for key in keys {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
        for problem in [LockProblem::Attachments, LockProblem::TooShort, LockProblem::Mismatch] {
            assert!(!problem_text(problem).starts_with("composer."), "{problem:?}");
        }
    }
}
