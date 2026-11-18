//! The comparison: how many cases the agents disagree on, how often each pair agrees, and how
//! often changing each variable flipped an agent's answer (the variable the answers turn on
//! comes first). Every number says what it counts.

use super::super::kit::{block_title, caption, section_label, surface, value_bar};
use super::super::panel::ArenaPanel;
use super::super::text::tr;
use crate::domain::arena::lab::{cells, disagreements, flips, pairwise, Experiment, Stance};
use crate::theme::ThemeTokens;
use crate::ui::components::BadgeColors;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

/// The badge colours of a stance.
pub(super) fn stance_colors(stance: Stance, theme: &ThemeTokens) -> BadgeColors {
    match stance {
        Stance::Yes => BadgeColors::new(theme.accent_success, theme.accent_success_subtle, theme.accent_success_border),
        Stance::No => BadgeColors::new(theme.accent_dialectic, theme.accent_dialectic_subtle, theme.accent_dialectic_border),
        Stance::Undecided => BadgeColors::new(theme.text_secondary, theme.bg_elevated, theme.border_subtle),
    }
}

fn share(part: usize, whole: usize) -> f32 {
    if whole == 0 {
        0.0
    } else {
        part as f32 / whole as f32
    }
}

impl ArenaPanel {
    fn bar_row(&self, label: String, part: usize, whole: usize, color: Rgba) -> Div {
        let theme = &self.theme;
        let text = div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(label);
        div().w_full().flex().items_center().gap(theme.control_gap).child(text).child(value_bar(share(part, whole), theme.arena_score_bar_width, color, theme))
    }

    pub(super) fn render_compare(&self, experiment: &Experiment) -> Div {
        let theme = &self.theme;
        let total = cells(&experiment.scenario).len();
        let split = disagreements(experiment).len();
        let pairs = pairwise(experiment).into_iter().map(|pair| {
            let label = t!("arena.compare.pair", first = self.data.agent_name(&pair.first), second = self.data.agent_name(&pair.second), agreed = pair.agreed, compared = pair.compared).to_string();
            self.bar_row(label, pair.agreed, pair.compared, theme.accent_success)
        });
        let variables = flips(experiment).into_iter().filter_map(|flip| {
            let name = experiment.scenario.variables.get(flip.variable)?.name.clone();
            let label = t!("arena.compare.flips", variable = name, flips = flip.flips, comparisons = flip.comparisons).to_string();
            Some(self.bar_row(label, flip.flips, flip.comparisons, theme.accent_dialectic))
        });
        let pair_section = (experiment.agents.len() > 1).then(|| div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.compare.agreement"), theme)).children(pairs));
        surface(theme)
            .child(block_title(LucideIcon::GitCompareArrows, theme.accent_dialectic, tr("arena.compare.title"), theme))
            .child(caption(t!("arena.compare.disagree", split = split, total = total).to_string(), theme))
            .children(pair_section)
            .child(div().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("arena.compare.flip_title"), theme)).children(variables))
            .child(caption(tr("arena.compare.flip_hint"), theme))
    }
}
