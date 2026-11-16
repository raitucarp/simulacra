//! Vertical category 1: Identity & Bio (3 focused sub-tabs).

mod biography;
mod demographics;
mod profile;

use super::StepCtx;
use gpui_kit::*;

const PROFILE_TAB: usize = 0;
const DEMOGRAPHICS_TAB: usize = 1;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        PROFILE_TAB => profile::render(ctx).into_any_element(),
        DEMOGRAPHICS_TAB => demographics::render(ctx).into_any_element(),
        _ => biography::render(ctx).into_any_element(),
    }
}
