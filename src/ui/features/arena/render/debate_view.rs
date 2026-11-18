//! The debate on show: the motion and where it stands, its controls (stop while running,
//! continue a stopped one, publish once judged, open the published thread, delete), then the
//! transcript, the verdict and the claims that still stand.

use super::super::data::RunState;
use super::super::events::ArenaEvent;
use super::super::kit::{block_title, caption, control_row, ghost_button, primary_button, prose, small_button, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{format_label, tr};
use super::two_columns;
use crate::domain::arena::debate::{Debate, Phase};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::base::Disableable;
use gpui_kit::*;
use rust_i18n::t;

const STOP_ID: &str = "arena-debate-stop";
const CONTINUE_ID: &str = "arena-debate-continue";
const PUBLISH_ID: &str = "arena-debate-publish";
const OPEN_ID: &str = "arena-debate-open-thread";
const DELETE_ID: &str = "arena-debate-delete";
const NEW_ID: &str = "arena-debate-new";

impl ArenaPanel {
    pub(super) fn render_debate_tab(&self, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let open = self.open_debate.as_ref().and_then(|id| self.data.debate(id)).cloned();
        let side = match &open {
            Some(_) => self.render_new_debate(cx),
            None => self.render_debate_setup(cx),
        };
        let side = div().flex().flex_col().gap(self.theme.section_gap).child(side).child(self.render_saved_debates(cx));
        let main_first = open.is_some();
        let main = match open {
            Some(debate) => self.render_debate(&debate, cx),
            None => self.idle_state("arena.debate.idle_title", "arena.debate.idle_hint"),
        };
        two_columns(class, side, main, main_first, &self.theme)
    }

    fn render_new_debate(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let entity = cx.entity().downgrade();
        let button = primary_button(NEW_ID, LucideIcon::Plus, tr("arena.debate.new")).on_click(move |_, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.new_debate(cx));
        });
        surface(theme).child(caption(tr("arena.debate.new_hint"), theme)).child(control_row(theme).child(button))
    }

    /// "Turn 3 of 8 · Ada is speaking", "Judging…", "Stopped after turn 3" or "Judged".
    fn status_line(&self, debate: &Debate) -> String {
        let (done, total) = (debate.turns.len(), debate.total_turns());
        match (&self.data.run, debate.phase()) {
            (RunState::Speaking(id, speaker), _) if *id == debate.id => t!("arena.status.speaking", turn = done + 1, total = total, name = self.data.agent_name(speaker)).to_string(),
            (RunState::Judging(id), _) if *id == debate.id => tr("arena.status.judging"),
            (_, Phase::Done) => tr("arena.status.judged"),
            (_, Phase::Judging) => tr("arena.status.awaiting_judge"),
            (_, Phase::Speaking(_)) => t!("arena.status.stopped", turn = done, total = total).to_string(),
        }
    }

    fn debate_controls(&self, debate: &Debate, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let running = self.data.run.runs(&debate.id);
        let emit = |event: ArenaEvent| {
            let entity = cx.entity().downgrade();
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let _ = entity.update(cx, |_, cx| cx.emit(event.clone()));
            }
        };
        let mut row = control_row(theme);
        if running {
            row = row.child(small_button(STOP_ID, LucideIcon::CircleStop, tr("arena.stop")).on_click(emit(ArenaEvent::Cancel)));
        } else if debate.phase() != Phase::Done {
            row = row.child(primary_button(CONTINUE_ID, LucideIcon::Play, tr("arena.debate.continue")).disabled(self.data.run.is_running()).on_click(emit(ArenaEvent::ContinueDebate(debate.id.clone()))));
        }
        row = match (&debate.published, debate.phase() == Phase::Done) {
            (Some(root), _) => row.child(small_button(OPEN_ID, LucideIcon::ExternalLink, tr("arena.open_thread")).on_click(emit(ArenaEvent::OpenPost(root.clone())))),
            (None, true) => row.child(primary_button(PUBLISH_ID, LucideIcon::Send, tr("arena.debate.publish")).on_click(emit(ArenaEvent::PublishDebate(debate.id.clone())))),
            (None, false) => row,
        };
        row.child(ghost_button(DELETE_ID, LucideIcon::Trash, tr("arena.delete")).disabled(running).on_click(emit(ArenaEvent::DeleteDebate(debate.id.clone()))))
    }

    fn render_debate(&self, debate: &Debate, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let meta = format!("{} · {}", format_label(debate.setup.format), t!("arena.debate.rounds_count", count = debate.setup.rounds));
        let head = surface(theme)
            .child(block_title(LucideIcon::Swords, theme.accent_brand, tr("arena.debate.motion"), theme))
            .child(prose(debate.setup.motion.clone(), theme).font_weight(FontWeight::SEMIBOLD))
            .child(caption(format!("{meta} · {}", self.status_line(debate)), theme))
            .child(self.debate_controls(debate, cx));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(head)
            .children(debate.verdict.as_ref().map(|verdict| self.render_verdict(debate, verdict)))
            .children(debate.map.as_ref().map(|map| self.render_claims(debate, map)))
            .child(self.render_turns(debate))
    }
}
