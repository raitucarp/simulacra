//! The diversity view's cards: the shares of the chosen dimension (authors open their
//! profile), the variety per period, and the vault reading by subject and creator.

use super::super::super::common::{card, card_title, option_chip, option_group, tr};
use super::super::{posts_label, short_day};
use crate::domain::insights::lenses::diversity::{Dimension, DiversityReport, ShareLabel, Shares};
use crate::domain::ProfileId;
use crate::ui::features::day_filter::format_day;
use crate::ui::features::insights::chart::columns::{Column, ColumnChart};
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::percent;
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Shares are charted as percentages.
const PERCENT: f64 = 100.0;
/// Sources listed per dimension.
const SHARES_SHOWN: usize = 8;

/// The text of a share's label.
pub fn label_text(label: &ShareLabel) -> String {
    match label {
        ShareLabel::Name(name) => name.clone(),
        ShareLabel::Key(key) => tr(key),
    }
}

fn share_rows(shares: &Shares, theme: &crate::theme::ThemeTokens) -> Vec<RankedRow> {
    shares.shares.iter().take(SHARES_SHOWN).enumerate().map(|(i, s)| RankedRow { label: label_text(&s.label), value: s.count as f64, value_text: percent(shares.share(i)), colour: series(i, theme), active: false }).collect()
}

impl InsightsPanel {
    fn pick_dimension(&self, dimension: Dimension, cx: &mut Context<Self>) -> Rc<dyn Fn(&mut Window, &mut App)> {
        let entity = cx.entity().downgrade();
        Rc::new(move |_, cx| {
            let _ = entity.update(cx, |panel, cx| {
                panel.lenses.dimension = dimension;
                cx.notify();
            });
        })
    }

    pub(super) fn shares_card(&self, diversity: &DiversityReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chosen = self.lenses.dimension;
        let chips = Dimension::FEED.iter().map(|&d| option_chip(ElementId::Name(format!("insight-dimension-{}", d.key()).into()), tr(&format!("insight.dimension.{}", d.key())), d == chosen, theme, self.pick_dimension(d, cx))).collect();
        let shares = diversity.feed_shares(chosen).cloned().unwrap_or_default();
        let ids: Vec<ProfileId> = shares.shares.iter().take(SHARES_SHOWN).map(|s| ProfileId::from(s.key.as_str())).collect();
        let entity = cx.entity().downgrade();
        let open = (chosen == Dimension::Author).then(|| -> crate::ui::features::insights::chart::ranked::RowAction {
            Rc::new(move |i, _, cx| {
                let id = ids[i].clone();
                let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenAuthor(id)));
            })
        });
        let summary = if shares.is_empty() { tr("insight.diversity.no_agents") } else { t!("insight.diversity.evenness", evenness = format!("{:.2}", shares.variety.evenness), sources = shares.variety.sources).to_string() };
        card(theme)
            .child(card_title(LucideIcon::Users, tr("insight.diversity.shares"), None, theme))
            .child(option_group(tr("insight.diversity.by"), chips, theme))
            .child(ranked_list("insight-shares", &share_rows(&shares, theme), open, theme))
            .child(summary_line(summary, theme))
    }

    pub(super) fn variety_trend_card(&self, diversity: &DiversityReport) -> Div {
        let theme = &self.theme;
        let periods = &diversity.periods;
        let columns = diversity.trend.iter().map(|point| {
            let span = format!("{} – {}", format_day(periods.start_of(point.period)), format_day(periods.end_of(point.period)));
            let tip = match point.authors {
                Some(value) => t!("insight.diversity.trend_tip", span = span, authors = percent(value), stances = point.stances.map_or_else(|| tr("insight.none"), percent), posts = posts_label(point.posts)).to_string(),
                None => t!("insight.diversity.trend_tip_few", span = span, posts = posts_label(point.posts)).to_string(),
            };
            Column { segments: vec![(point.authors.unwrap_or_default() * PERCENT, theme.accent_brand)], tip }
        });
        let labels = diversity.trend.iter().map(|p| short_day(periods.start_of(p.period))).collect();
        let chart = ColumnChart { id: "insight-variety-trend".into(), columns: columns.collect(), labels, height: theme.insights_chart_height, on_click: None, theme: theme.clone() };
        card(theme).child(card_title(LucideIcon::ChartColumn, tr("insight.diversity.trend"), None, theme)).child(chart).child(summary_line(tr("insight.diversity.trend_hint"), theme))
    }

    pub(super) fn reading_card(&self, diversity: &DiversityReport) -> Div {
        let theme = &self.theme;
        let title = card_title(LucideIcon::Library, tr("insight.diversity.reading"), None, theme);
        if diversity.documents_read == 0 {
            return card(theme).child(title).child(summary_line(tr("insight.diversity.reading_none"), theme));
        }
        let subjects = diversity.reading.iter().find(|s| s.dimension == Dimension::Subject).cloned().unwrap_or_default();
        let creators = diversity.reading.iter().find(|s| s.dimension == Dimension::Creator).cloned().unwrap_or_default();
        let summary = t!("insight.diversity.reading_summary", documents = diversity.documents_read, subjects = subjects.variety.sources, subject_variety = percent(subjects.variety.simpson), creators = creators.variety.sources).to_string();
        card(theme).child(title).child(ranked_list("insight-reading", &share_rows(&subjects, theme), None, theme)).child(summary_line(summary, theme))
    }
}
