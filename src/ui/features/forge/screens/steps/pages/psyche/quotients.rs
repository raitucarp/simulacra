//! The intelligence quotients as skills: each a slider named by its level (Novice …
//! Legendary) over a six-rung ladder, with the points still needed for the next level.

use super::ladder::LadderStep;
use crate::state::forge::attributes::INTELLIGENCE_FIELDS;
use crate::state::forge::{DraftEdit, QuotientLevel, ScoreField};
use crate::ui::features::forge::screens::common::{card, heading, muted};
use crate::ui::features::forge::text::{tr, tr_count};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ScoreSlider;
use gpui_kit::*;

const SKILLS_KEY: &str = "forge.ui.mind.skills";
const SKILLS_HINT_KEY: &str = "forge.ui.mind.skills_hint";
const NEXT_KEY: &str = "forge.ui.mind.next_level";
const PEAK_KEY: &str = "forge.ui.mind.peak_level";
const ARG_LEVEL: &str = "level";

impl ForgeView {
    /// The skills card with one row per quotient.
    pub(super) fn quotient_skills(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let rows: Vec<Div> = INTELLIGENCE_FIELDS.iter().map(|&field| self.skill_row(field, handle)).collect();
        card(theme).gap(theme.section_gap).child(heading(tr(SKILLS_KEY), theme)).child(muted(tr(SKILLS_HINT_KEY), theme)).children(rows)
    }

    fn skill_row(&self, field: ScoreField, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let cfg = &self.draft.profile().config;
        let step = LadderStep::of(*field.of(cfg));
        let edit = handle.edit(move |value: u8| DraftEdit::Score(field, value));
        let next = match step.next {
            Some((level, points)) => tr_count(NEXT_KEY, usize::from(points), &[(ARG_LEVEL, tr(&level.label_key()))]),
            None => tr(PEAK_KEY),
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(ScoreSlider::new(field, cfg, theme, move |v, w, cx| edit(v, w, cx)))
            .child(self.ladder(step))
            .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(next))
            .children(self.issue_lines(field.path()))
    }

    /// Six rungs, lit up to the current level.
    fn ladder(&self, step: LadderStep) -> Div {
        let theme = &self.theme;
        let rungs = QuotientLevel::ALL.into_iter().map(|rung| {
            let color = if step.lit(rung) { theme.accent_brand } else { theme.bg_elevated };
            div().flex_1().h(theme.forge_skill_bar_height).rounded(theme.radius_full).bg(color)
        });
        div().flex().gap(theme.spacing_2xs).children(rungs)
    }
}
