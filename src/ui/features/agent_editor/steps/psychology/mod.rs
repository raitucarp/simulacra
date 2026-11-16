//! Vertical category 2: Psychology & MBTI (4 focused sub-tabs).

mod big_five;
mod enneagram;
mod mbti;
mod relational;

use super::StepCtx;
use gpui_kit::*;

const MBTI_TAB: usize = 0;
const BIG_FIVE_TAB: usize = 1;
const ENNEAGRAM_TAB: usize = 2;

pub(super) fn render(sub_tab: usize, ctx: &StepCtx<'_>) -> AnyElement {
    match sub_tab {
        MBTI_TAB => mbti::render(ctx).into_any_element(),
        BIG_FIVE_TAB => big_five::render(ctx).into_any_element(),
        ENNEAGRAM_TAB => enneagram::render(ctx).into_any_element(),
        _ => relational::render(ctx).into_any_element(),
    }
}
