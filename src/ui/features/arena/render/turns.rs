//! The transcript: each turn under its speaker, side and role, joined by a threadline in the
//! speaker's side colour, and a quiet "is speaking" row while the next turn is on its way.

use super::super::data::RunState;
use super::super::kit::{block_title, caption, prose, side_badge, side_color, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{role_label, side_label, tr};
use crate::domain::arena::debate::{Debate, Side, Turn};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

impl ArenaPanel {
    fn render_turn(&self, turn: &Turn) -> Div {
        let theme = &self.theme;
        let slot = &turn.slot;
        let speaker = div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(self.data.agent_name(&slot.speaker));
        let role = if slot.round == 0 { role_label(slot.role) } else { format!("{} · {}", role_label(slot.role), t!("arena.debate.round", round = slot.round)) };
        let head = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.inline_gap)
            .child(speaker)
            .children((slot.side != Side::Open).then(|| side_badge(slot.side, side_label(slot.side), theme)))
            .child(caption(role, theme));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .pl(theme.spacing_md)
            .border_l(theme.border_width_threadline)
            .border_color(side_color(slot.side, theme).border)
            .child(head)
            .child(prose(turn.text.clone(), theme))
    }

    pub(super) fn render_turns(&self, debate: &Debate) -> Div {
        let theme = &self.theme;
        let speaking = match &self.data.run {
            RunState::Speaking(id, speaker) if *id == debate.id => Some(t!("arena.debate.is_speaking", name = self.data.agent_name(speaker)).to_string()),
            _ => None,
        };
        let empty = (debate.turns.is_empty() && speaking.is_none()).then(|| caption(tr("arena.debate.no_turns"), theme));
        surface(theme)
            .child(block_title(LucideIcon::MessageSquare, theme.text_secondary, tr("arena.debate.transcript"), theme))
            .children(debate.turns.iter().map(|turn| self.render_turn(turn)))
            .children(speaking.map(|line| caption(line, theme).italic()))
            .children(empty)
    }
}
