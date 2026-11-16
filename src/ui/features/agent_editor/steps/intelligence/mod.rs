//! Vertical category 3: Intelligence & Mind (3 focused sub-tabs).

mod quotients;
mod style;
mod tempo;

use super::StepCtx;
use gpui_kit::*;

const STYLE_TAB: usize = 0;
const QUOTIENTS_TAB: usize = 1;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        STYLE_TAB => style::render(ctx).into_any_element(),
        QUOTIENTS_TAB => quotients::render(ctx).into_any_element(),
        _ => tempo::render(ctx).into_any_element(),
    }
}
