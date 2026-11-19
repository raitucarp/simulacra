//! The Viewpoint diversity view: key numbers that each say what they measure (equally active
//! voices, different stances, different schools, spread in meaning, replies that push back),
//! the shares of the chosen dimension, what was mostly read lately, the trend per period and
//! the variety of the vault reading. It describes what was read; it does not judge it.

mod cards;
mod lately;

use super::super::common::{tile_columns, tile_row, tr};
use super::super::Room;
use super::explained_tile;
use crate::domain::insights::lenses::diversity::{DiversityReport, Dimension};
use crate::domain::insights::lenses::LensReport;
use crate::ui::features::insights::chart::ticks::percent;
use crate::ui::features::insights::panel::InsightsPanel;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Decimals shown for the spread in meaning.
const SPREAD_DIGITS: usize = 2;

impl InsightsPanel {
    pub(in super::super) fn render_diversity(&self, report: &Rc<LensReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let diversity = &report.diversity;
        if report.posts.is_empty() {
            return self.view_empty("insight.diversity.empty_title", "insight.diversity.empty_hint");
        }
        let tiles = self.diversity_tiles(diversity);
        let row = tile_row(tile_columns(room, tiles.len()), theme).children(tiles);
        let shares = self.shares_card(diversity, cx);
        let lately = self.lately_card(report, cx);
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(row)
            .child(super::super::common::pair(room, shares, lately, theme))
            .child(super::super::common::pair(room, self.variety_trend_card(diversity), self.reading_card(diversity), theme))
    }

    fn diversity_tiles(&self, diversity: &DiversityReport) -> Vec<Div> {
        let theme = &self.theme;
        let simpson_tile = |dimension: Dimension| {
            let shares = diversity.feed_shares(dimension).cloned().unwrap_or_default();
            let key = format!("insight.diversity.{}_meaning", dimension.key());
            let meaning = t!(key.as_str(), items = shares.variety.items, sources = shares.variety.sources).to_string();
            explained_tile(percent(shares.variety.simpson), tr(&format!("insight.diversity.{}_label", dimension.key())), meaning, theme.accent_brand, theme)
        };
        let authors = diversity.feed_shares(Dimension::Author).cloned().unwrap_or_default();
        let voices = explained_tile(
            format!("{:.1}", authors.variety.effective),
            tr("insight.diversity.voices_label"),
            t!("insight.diversity.voices_meaning", bits = format!("{:.1}", authors.variety.entropy), authors = authors.variety.sources).to_string(),
            theme.accent_success,
            theme,
        );
        let spread = match diversity.spread {
            Some(spread) => explained_tile(format!("{spread:.SPREAD_DIGITS$}"), tr("insight.diversity.spread_label"), t!("insight.diversity.spread_meaning", posts = super::posts_label(diversity.embedded)).to_string(), theme.info, theme),
            None => explained_tile(tr("insight.none"), tr("insight.diversity.spread_label"), tr("insight.diversity.spread_missing"), theme.text_muted, theme),
        };
        let replies = &diversity.replies;
        let back = if replies.total() == 0 { 0.0 } else { replies.pushes_back as f64 / replies.total() as f64 };
        let reply_tile = explained_tile(percent(back), tr("insight.diversity.replies_label"), t!("insight.diversity.replies_meaning", agree = replies.agrees, back = replies.pushes_back, neither = replies.neither).to_string(), theme.accent_dialectic, theme);
        vec![voices, simpson_tile(Dimension::Author), simpson_tile(Dimension::Stance), simpson_tile(Dimension::School), spread, reply_tile]
    }
}
