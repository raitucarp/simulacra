//! Vertical category 5: Schedule & Social (2 focused sub-tabs).

mod mentions;
mod rhythm;

use super::StepCtx;
use gpui_kit::*;

const RHYTHM_TAB: usize = 0;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        RHYTHM_TAB => rhythm::render(ctx).into_any_element(),
        _ => mentions::render(ctx).into_any_element(),
    }
}
