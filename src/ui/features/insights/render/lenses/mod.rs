//! sb F3: the lens views of the Insights page (Concept timeline, Viewpoint diversity,
//! Readability), drawn from the lens report the shell hands the page, with the page's cards,
//! tiles and chart kit.

mod diversity;
mod readability;
mod timeline;

use super::common::tr;
use super::Room;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::features::day_filter::month_label;
use crate::ui::features::insights::view::InsightsView;
use chrono::{Datelike, NaiveDate};
use gpui_kit::*;

/// The "updating" badge while a new lens report is computed.
pub fn lens_updating(pending: bool, theme: &ThemeTokens) -> Option<AnyElement> {
    pending.then(|| {
        div()
            .px(theme.spacing_sm)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_full)
            .bg(theme.accent_brand_subtle)
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_secondary)
            .child(tr("insights.updating"))
            .into_any_element()
    })
}

/// A short axis label: month and day.
pub fn short_day(day: NaiveDate) -> String {
    format!("{} {}", month_label(day.month()), day.day())
}

/// A count of posts ("1 post", "5 posts").
pub fn posts_label(count: usize) -> String {
    crate::i18n::t_count("insights.posts", count)
}

/// A key number over its label, with a line saying what the number means.
pub fn explained_tile(value: String, label: String, meaning: String, colour: Rgba, theme: &ThemeTokens) -> Div {
    super::common::tile(value, label, colour, theme).child(div().text_size(theme.font_size_2xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(meaning))
}

impl InsightsPanel {
    pub(super) fn render_lens(&self, room: Room, cx: &mut Context<Self>) -> AnyElement {
        let Some(report) = self.lenses.report.clone() else { return self.render_loading().into_any_element() };
        match self.state.view {
            InsightsView::Timeline => self.render_concept_timeline(&report, room, cx).into_any_element(),
            InsightsView::Diversity => self.render_diversity(&report, room, cx).into_any_element(),
            _ => self.render_readability(&report, room, cx).into_any_element(),
        }
    }
}
