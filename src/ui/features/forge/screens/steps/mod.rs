//! The step-by-step creator: top bar, step rail (left), stage with the live character sheet
//! (center), options for the current step (right) and the sticky save bar.

mod options;
mod pages;
mod hints;
mod palette_strip;
mod rail;
mod save_bar;
mod sheet;
mod stage;
mod topbar;

pub(super) use stage::{palette_badge, stage_portrait};

use crate::domain::portrait::{AvatarMode, PortraitSpec};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::illustration::StepKey;
use crate::state::forge::ForgeStep;
use crate::ui::components::{persisted_h_resizable, SplitPane};
use gpui_kit::*;

/// Layout id of the rail | stage | options split (sizes persist).
const STEPS_SPLIT_ID: &str = "forge-steps-split";

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
        let t = &self.theme;
        let split = persisted_h_resizable(STEPS_SPLIT_ID, t)
            .pane(SplitPane::sized(t.forge_rail_width, t.forge_rail_min..t.forge_rail_max).child(self.step_rail(handle)))
            .pane(SplitPane::flex(t.split_flex_min).child(self.stage(handle)))
            .pane(SplitPane::sized(t.forge_options_width, t.forge_options_min..t.forge_options_max).child(self.options_panel(handle)));
        let columns = div().flex_1().min_h_0().flex().child(split);
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
