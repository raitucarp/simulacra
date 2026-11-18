//! Cards by state (one stacked bar with a legend) and confidence calibration (a bar per
//! confidence: how often an answer given at that confidence was right).

use super::super::super::panel::ReviewPanel;
use super::super::super::text::{percent, tr};
use super::super::common::{block_title, caption, control_row, legend_item, progress_bar, surface};
use crate::domain::review::stats::{calibration, state_counts, CardState};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

/// The color of a card state.
fn state_color(state: CardState, theme: &ThemeTokens) -> Rgba {
    match state {
        CardState::New => theme.info,
        CardState::Learning => theme.warning,
        CardState::Young => theme.accent_success,
        CardState::Mature => theme.accent_agent,
        CardState::Suspended => theme.text_muted,
    }
}

impl ReviewPanel {
    pub(super) fn states_panel(&self) -> Div {
        let theme = &self.theme;
        let counts = state_counts(&self.data.cards);
        let total = counts.total().max(1) as f32;
        let segments = CardState::ALL.into_iter().filter(|s| counts.get(*s) > 0).map(|state| {
            div().h_full().w(relative(counts.get(state) as f32 / total)).bg(state_color(state, theme))
        });
        let legend = CardState::ALL.into_iter().map(|state| {
            legend_item(state_color(state, theme), format!("{} {}", tr(&format!("review.state.{}", state.key())), counts.get(state)), theme)
        });
        surface(theme)
            .child(block_title(LucideIcon::Layers, theme.accent_brand, tr("review.stats.states"), theme))
            .child(div().w_full().h(theme.review_swatch_size).flex().rounded(theme.radius_full).overflow_hidden().bg(theme.border_subtle).children(segments))
            .child(control_row(theme).children(legend))
            .child(caption(tr("review.stats.states_caption"), theme))
    }

    pub(super) fn calibration_panel(&self) -> Div {
        let theme = &self.theme;
        let rows = calibration(&self.data.log);
        let lines = rows.iter().map(|row| {
            let label = tr(&format!("review.confidence.{}", row.confidence.key()));
            let value = row.accuracy().map_or_else(|| tr("review.none"), percent);
            div()
                .flex()
                .items_center()
                .gap(theme.control_gap)
                .child(div().w(theme.review_stat_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
                .child(div().flex_1().child(progress_bar(row.accuracy().unwrap_or_default(), theme.accent_dialectic, theme)))
                .child(div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("review.stats.calibration_row", share = value, count = row.answers).to_string()))
        });
        let answered = rows.iter().any(|row| row.answers > 0);
        surface(theme)
            .child(block_title(LucideIcon::Brain, theme.accent_dialectic, tr("review.stats.calibration"), theme))
            .children(lines)
            .child(caption(tr(if answered { "review.stats.calibration_caption" } else { "review.stats.calibration_empty" }), theme))
    }
}
