//! The step-by-step creator: top bar, step rail (left), stage with the live character sheet
//! (center), options for the current step (right) and the sticky save bar.

mod options;
mod pages;
mod rail;
mod save_bar;
mod sheet;
mod stage;
mod topbar;

pub(super) use stage::{class_badge, stage_portrait};

use crate::domain::portrait::{AvatarMode, PortraitSpec};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::illustration::StepKey;
use crate::state::forge::ForgeStep;
use gpui_kit::*;

/// The banner key of a step (keys are 1:1 with `ForgeStep` keys).
pub(super) fn step_key(step: ForgeStep) -> StepKey {
    StepKey::ALL.iter().copied().find(|key| key.key() == step.key()).unwrap_or(StepKey::Origin)
}

/// The generated portrait of an avatar mode, if it is one.
pub(super) fn pcg_spec(mode: Option<&AvatarMode>) -> Option<PortraitSpec> {
    match mode {
        Some(AvatarMode::Pcg(spec)) => Some(*spec),
        _ => None,
    }
}

impl ForgeView {
    pub(super) fn steps_screen(&self, handle: &ForgeHandle) -> AnyElement {
        let columns = div()
            .flex_1()
            .min_h_0()
            .flex()
            .child(self.step_rail(handle))
            .child(self.stage(handle))
            .child(self.options_panel(handle));
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(self.top_bar(handle))
            .child(columns)
            .children(self.save_bar(handle))
            .into_any_element()
    }
}
