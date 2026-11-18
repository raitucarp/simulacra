//! Who debates: one row per agent of the network with its side choices (For / Against, or
//! In for a Socratic round; clicking the chosen side again takes the agent out), and the
//! moderator's chair.

use super::super::kit::{caption, section_label};
use super::super::panel::ArenaPanel;
use super::super::text::{side_label, tr};
use crate::domain::arena::debate::{DebateFormat, Side, MAX_DEBATERS, MIN_DEBATERS};
use crate::domain::ProfileId;
use crate::ui::components::{chip_row, choice_chip};
use gpui_kit::*;

const SIDE_PREFIX: &str = "arena-side-";
const MODERATOR_PREFIX: &str = "arena-moderator-";

impl ArenaPanel {
    /// The sides an agent can take in the current format.
    fn sides(&self) -> &'static [Side] {
        match self.draft.format {
            DebateFormat::Socratic => &[Side::Open],
            _ => &[Side::For, Side::Against],
        }
    }

    fn side_chip(&self, index: usize, agent: &ProfileId, side: Side, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let chosen = self.draft.side_of(agent) == Some(side);
        let (agent, entity) = (agent.clone(), cx.entity().downgrade());
        let label = if side == Side::Open { tr("arena.debate.joins") } else { side_label(side) };
        choice_chip(ElementId::Name(format!("{SIDE_PREFIX}{index}-{}", side.key()).into()), &label, chosen, theme, move |_, cx| {
            let _ = entity.update(cx, |panel, cx| {
                let next = (!chosen).then_some(side);
                panel.draft = panel.draft.with_side(&agent, next);
                cx.notify();
            });
        })
    }

    pub(super) fn render_debaters(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let mut column = div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.debate.debaters"), theme));
        if self.data.agents.len() < MIN_DEBATERS {
            return column.child(caption(tr("arena.debate.need_agents"), theme));
        }
        for (index, agent) in self.data.agents.iter().enumerate() {
            let name = div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(agent.name.clone());
            let chips: Vec<Stateful<Div>> = self.sides().iter().map(|side| self.side_chip(index, &agent.id, *side, cx)).collect();
            column = column.child(div().w_full().flex().items_center().gap(theme.control_gap).child(name).child(chip_row(theme).children(chips)));
        }
        column.child(caption(rust_i18n::t!("arena.debate.debaters_hint", min = MIN_DEBATERS, max = MAX_DEBATERS).to_string(), theme))
    }

    pub(super) fn render_moderator(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let free = self.data.agents.iter().filter(|agent| self.draft.side_of(&agent.id).is_none());
        let options = std::iter::once((None, tr("arena.debate.no_moderator"))).chain(free.map(|agent| (Some(agent.id.clone()), agent.name.clone())));
        let chips = options.enumerate().map(|(index, (moderator, label))| {
            let (selected, entity) = (self.draft.moderator == moderator, cx.entity().downgrade());
            choice_chip(ElementId::Name(format!("{MODERATOR_PREFIX}{index}").into()), &label, selected, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| {
                    panel.draft = panel.draft.with_moderator(moderator.clone());
                    cx.notify();
                });
            })
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.debate.moderator"), theme)).child(chip_row(theme).children(chips))
    }
}
