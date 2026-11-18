//! The verdict: the judge's summary, which side leads, and each debater's four rubric scores
//! as bars with the judge's note.

use super::super::kit::{block_title, caption, prose, side_badge, surface, value_bar};
use super::super::panel::ArenaPanel;
use super::super::text::{criterion_label, side_label, tr};
use crate::domain::arena::debate::{Criterion, Debate, Judge, Score, Side, Verdict, MAX_SCORE};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

impl ArenaPanel {
    fn render_score(&self, debate: &Debate, score: &Score) -> Div {
        let theme = &self.theme;
        let side = debate.setup.side_of(&score.agent);
        let total = t!("arena.verdict.total", score = score.total(), max = u32::from(MAX_SCORE) * Criterion::ALL.len() as u32).to_string();
        let head = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.inline_gap)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).child(self.data.agent_name(&score.agent)))
            .children(side.filter(|side| *side != Side::Open).map(|side| side_badge(side, side_label(side), theme)))
            .child(caption(total, theme));
        let rows = Criterion::ALL.iter().map(|criterion| {
            let mark = score.mark(*criterion);
            let share = f32::from(mark) / f32::from(MAX_SCORE);
            let label = div().w(theme.arena_score_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(criterion_label(*criterion));
            let value = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(mark.to_string());
            div().flex().items_center().gap(theme.control_gap).child(label).child(value_bar(share, theme.arena_score_bar_width, theme.accent_brand, theme)).child(value)
        });
        let note = (!score.note.trim().is_empty()).then(|| caption(score.note.clone(), theme));
        div().w_full().flex().flex_col().gap(theme.spacing_xs).p(theme.card_padding).rounded(theme.radius_md).bg(theme.bg_elevated).child(head).children(rows).children(note)
    }

    pub(super) fn render_verdict(&self, debate: &Debate, verdict: &Verdict) -> Div {
        let theme = &self.theme;
        let judge = match &verdict.judged_by {
            Judge::GlobalModel => tr("arena.debate.judge_global"),
            Judge::Agent(agent) => self.data.agent_name(agent),
        };
        let leading = match verdict.leading_side(&debate.setup) {
            Some(side) => t!("arena.verdict.leads", side = side_label(side)).to_string(),
            None if debate.setup.on_side(Side::For).is_empty() => String::new(),
            None => tr("arena.verdict.tied"),
        };
        let summary = (!verdict.summary.trim().is_empty()).then(|| prose(verdict.summary.clone(), theme));
        surface(theme)
            .child(block_title(LucideIcon::Trophy, theme.accent_dialectic, tr("arena.verdict.title"), theme))
            .child(caption(t!("arena.verdict.judged_by", judge = judge).to_string(), theme))
            .children(summary)
            .children((!leading.is_empty()).then(|| div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_dialectic).child(leading)))
            .child(div().w_full().flex().flex_wrap().gap(theme.stack_gap).children(verdict.scores.iter().map(|score| self.render_score(debate, score).flex_1().min_w(theme.arena_score_label_width + theme.arena_score_bar_width + theme.arena_row_label_width))))
    }
}
