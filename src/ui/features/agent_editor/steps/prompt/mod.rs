//! Vertical category 6: Prompt Engine (3 focused sub-tabs).

mod assembly;
mod directive;
mod preview;
mod sections;

use super::StepCtx;
use gpui_kit::*;

const ASSEMBLY_TAB: usize = 0;
const DIRECTIVE_TAB: usize = 1;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        ASSEMBLY_TAB => assembly::render(ctx).into_any_element(),
        DIRECTIVE_TAB => directive::render(ctx).into_any_element(),
        _ => preview::render(ctx).into_any_element(),
    }
}
