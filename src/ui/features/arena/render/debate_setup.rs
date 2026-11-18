//! The debate setup card: the motion (typed, or from the thread open in the inspector), the
//! format, the debaters and their sides, rounds, moderator and judge, and Start, with the
//! first thing still missing said plainly.

use super::super::kit::{area_field, block_title, caption, control_row, ghost_button, labelled, primary_button, section_label, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{format_hint, format_label, setup_problem_text, tr};
use crate::domain::arena::debate::{DebateFormat, Judge};
use crate::ui::components::{chip_row, choice_chip, mini_step_button};
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::*;

const THREAD_ID: &str = "arena-motion-from-thread";
const START_ID: &str = "arena-start-debate";
const FORMAT_PREFIX: &str = "arena-format-";
const ROUNDS_LESS_ID: &str = "arena-rounds-less";
const ROUNDS_MORE_ID: &str = "arena-rounds-more";
const LESS: &str = "−";
const MORE: &str = "+";

impl ArenaPanel {
    pub(super) fn render_debate_setup(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let entity = cx.entity().downgrade();
        let from_thread = self.data.open_thread.as_ref().map(|_| {
            let entity = entity.clone();
            ghost_button(THREAD_ID, LucideIcon::MessageSquare, tr("arena.debate.from_thread")).on_click(move |_, window, cx| {
                let _ = entity.update(cx, |panel, cx| panel.motion_from_thread(window, cx));
            })
        });
        let problem = self.setup_problem(cx);
        let start = primary_button(START_ID, LucideIcon::Swords, tr("arena.debate.start")).disabled(problem.is_some() || self.data.run.is_running()).on_click(move |_, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.start_debate(cx));
        });
        surface(theme)
            .child(block_title(LucideIcon::Swords, theme.accent_brand, tr("arena.debate.setup_title"), theme))
            .child(section_label(tr("arena.debate.motion"), theme))
            .child(area_field(&self.fields.motion, theme))
            .children(from_thread.map(|button| control_row(theme).child(button)))
            .child(self.render_formats(cx))
            .child(self.render_debaters(cx))
            .child(self.render_rounds(cx))
            .child(self.render_moderator(cx))
            .child(self.render_judges(cx))
            .children(problem.map(|problem| caption(setup_problem_text(problem), theme)))
            .child(control_row(theme).child(start))
    }

    fn render_formats(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = DebateFormat::ALL.iter().map(|format| {
            let (format, entity) = (*format, cx.entity().downgrade());
            choice_chip(ElementId::Name(format!("{FORMAT_PREFIX}{}", format.key()).into()), &format_label(format), self.draft.format == format, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| {
                    panel.draft = panel.draft.with_format(format);
                    cx.notify();
                });
            })
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.debate.format"), theme)).child(chip_row(theme).children(chips)).child(caption(format_hint(self.draft.format), theme))
    }

    fn render_rounds(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let step = |id: &'static str, label: &'static str, delta: i32| {
            let entity = cx.entity().downgrade();
            mini_step_button(id, label, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| {
                    panel.draft = panel.draft.with_rounds(delta);
                    cx.notify();
                });
            })
        };
        let value = div().min_w(theme.key_cap_min_width).text_center().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).child(self.draft.rounds.to_string());
        labelled(tr("arena.debate.rounds"), control_row(theme).child(step(ROUNDS_LESS_ID, LESS, -1)).child(value).child(step(ROUNDS_MORE_ID, MORE, 1)), theme)
    }

    fn render_judges(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let options = std::iter::once((Judge::GlobalModel, tr("arena.debate.judge_global")))
            .chain(self.data.agents.iter().filter(|agent| self.draft.side_of(&agent.id).is_none()).map(|agent| (Judge::Agent(agent.id.clone()), agent.name.clone())));
        let chips = options.enumerate().map(|(index, (judge, label))| {
            let entity = cx.entity().downgrade();
            let selected = self.draft.judge == judge;
            choice_chip(("arena-judge", index), &label, selected, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| {
                    panel.draft = panel.draft.with_judge(judge.clone());
                    cx.notify();
                });
            })
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.debate.judge"), theme)).child(chip_row(theme).children(chips))
    }
}
