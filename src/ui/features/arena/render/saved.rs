//! The saved debates and experiments: newest first, each with its time and how far it got;
//! a click shows it (a stopped one can then be continued).

use super::super::kit::{block_title, caption, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{stamp, tr};
use crate::domain::arena::debate::Phase;
use crate::domain::arena::lab::cells;
use crate::domain::arena::ArenaId;
use crate::ui::components::selectable_row;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;
use rust_i18n::t;

const DEBATES_ID: &str = "arena-saved-debates";
const EXPERIMENTS_ID: &str = "arena-saved-experiments";

impl ArenaPanel {
    fn saved_row(&self, id: (&'static str, usize), title: String, meta: String, selected: bool, on_click: impl Fn(&mut Window, &mut App) + 'static) -> Stateful<Div> {
        let theme = &self.theme;
        let body = div()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .min_w_0()
            .child(div().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(title))
            .child(caption(meta, theme));
        selectable_row(id, selected, theme.accent_brand_border, theme).on_click(move |_, window, cx| on_click(window, cx)).child(body)
    }

    /// A titled list of `rows`, or `empty` said when there are none.
    fn saved_list(&self, id: &'static str, title: String, rows: Vec<Stateful<Div>>, empty: String) -> Div {
        let theme = &self.theme;
        let none = rows.is_empty().then(|| caption(empty, theme));
        let list = div().id(id).max_h(theme.arena_saved_max_height).overflow_y_scrollbar().flex().flex_col().gap(theme.spacing_2xs).children(rows);
        surface(theme).child(block_title(LucideIcon::Archive, theme.text_secondary, title, theme)).child(list).children(none)
    }

    pub(super) fn render_saved_debates(&self, cx: &mut Context<Self>) -> Div {
        let rows: Vec<Stateful<Div>> = self.data.debates.iter().enumerate().map(|(index, debate)| {
            let progress = match debate.phase() {
                Phase::Done => tr("arena.status.judged"),
                _ => t!("arena.saved.progress", turn = debate.turns.len(), total = debate.total_turns()).to_string(),
            };
            let (id, entity) = (debate.id.clone(), cx.entity().downgrade());
            let selected = self.open_debate.as_ref() == Some(&debate.id);
            self.saved_row(("arena-saved-debate", index), debate.setup.motion.clone(), format!("{} · {progress}", stamp(debate.updated_at)), selected, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.open_debate(id.clone(), cx));
            })
        }).collect();
        self.saved_list(DEBATES_ID, tr("arena.saved.debates"), rows, tr("arena.saved.no_debates"))
    }

    pub(super) fn render_saved_experiments(&self, cx: &mut Context<Self>) -> Div {
        let rows: Vec<Stateful<Div>> = self.data.experiments.iter().enumerate().map(|(index, experiment)| {
            let total = cells(&experiment.scenario).len() * experiment.agents.len();
            let progress = t!("arena.saved.answers", done = experiment.answers.len(), total = total).to_string();
            let (id, entity): (ArenaId, _) = (experiment.id.clone(), cx.entity().downgrade());
            let selected = self.open_experiment.as_ref() == Some(&experiment.id);
            self.saved_row(("arena-saved-experiment", index), experiment.scenario.title.clone(), format!("{} · {progress}", stamp(experiment.updated_at)), selected, move |window, cx| {
                let _ = entity.update(cx, |panel, cx| panel.load_experiment(id.clone(), window, cx));
            })
        }).collect();
        self.saved_list(EXPERIMENTS_ID, tr("arena.saved.experiments"), rows, tr("arena.saved.no_experiments"))
    }
}
