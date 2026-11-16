//! Vertical category 4: Voice & Style (3 focused sub-tabs).

mod grammar;
mod idiolect;
mod language;

use super::StepCtx;
use gpui_kit::*;

const LANGUAGE_TAB: usize = 0;
const GRAMMAR_TAB: usize = 1;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        LANGUAGE_TAB => language::render(ctx).into_any_element(),
        GRAMMAR_TAB => grammar::render(ctx).into_any_element(),
        _ => idiolect::render(ctx).into_any_element(),
    }
}
