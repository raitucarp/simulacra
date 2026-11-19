//! Activity: posts over time and per author stacked by kind (the kind legend hides series),
//! the weekly rhythm as a radial heatmap, agents' trait profiles as a radar, and the
//! distributions of post length and sentence length.

use super::super::panel::{InsightsPanel, LegendId};
use super::common::{card, card_title, pair, tr};
use super::Room;
use crate::domain::insights::activity::{Histogram, KindCounts};
use crate::domain::insights::{InsightsReport, PostKind};
use crate::ui::features::day_filter::{format_day, month_label};
use jiff::civil::Date;
use crate::ui::features::insights::chart::columns::{Column, ColumnChart};
use crate::ui::features::insights::chart::legend::{legend, LegendEntry};
use crate::ui::features::insights::chart::palette::kind_colour;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::compact;
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// A short axis label: month and day.
fn short_day(day: Date) -> String {
    format!("{} {}", month_label(day.month()), day.day())
}

impl InsightsPanel {
    /// Stacked segments of `counts`, without the hidden kinds.
    fn stack(&self, counts: &KindCounts) -> Vec<(f64, Rgba)> {
        PostKind::ALL.iter().filter(|k| !self.is_hidden(LegendId::Kinds, k.index())).map(|k| (counts[k.index()] as f64, kind_colour(*k, &self.theme))).collect()
    }

    fn kind_legend(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let entries: Vec<LegendEntry> = PostKind::ALL
            .iter()
            .map(|k| LegendEntry { colour: kind_colour(*k, &self.theme), label: tr(&format!("insights.kind.{}", k.key())), value: Some(compact(report.activity.kinds[k.index()] as f64)), hidden: self.is_hidden(LegendId::Kinds, k.index()) })
            .collect();
        let entity = cx.entity().downgrade();
        let toggle = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.toggle_legend(LegendId::Kinds, i, cx));
        });
        legend("insights-kind-legend", &entries, Some(toggle), &self.theme)
    }

    /// Posts over time; `height` sets the plot (small multiple or full chart).
    pub(super) fn timeline_card(&self, report: &InsightsReport, height: Pixels, trailing: Option<AnyElement>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let buckets = &report.activity.timeline;
        let columns = buckets.iter().map(|b| {
            let span = if b.days > 1 { format!("{} – {}", format_day(b.start), format_day(b.end())) } else { format_day(b.start) };
            Column { segments: self.stack(&b.counts), tip: format!("{span}: {}", crate::i18n::t_count("insights.posts", b.total())) }
        });
        let labels = buckets.iter().map(|b| short_day(b.start)).collect();
        let chart = ColumnChart { id: "insights-timeline".into(), columns: columns.collect(), labels, height, on_click: None, theme: theme.clone() };
        let busiest = buckets.iter().max_by_key(|b| b.total());
        let summary = busiest.map_or_else(|| tr("insights.summary.no_posts"), |b| t!("insights.summary.timeline", count = report.posts.len(), day = format_day(b.start), most = b.total()).to_string());
        card(theme).child(card_title(LucideIcon::ChartColumn, tr("insights.card.timeline"), trailing, theme)).child(chart).child(self.kind_legend(report, cx)).child(summary_line(summary, theme))
    }

    fn authors_card(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let authors = &report.activity.authors;
        let name = |i: usize| report.authors.get(i).map(|a| a.name.clone()).unwrap_or_default();
        let columns = authors.iter().map(|(a, counts)| Column { segments: self.stack(counts), tip: format!("{}: {}", name(*a), crate::i18n::t_count("insights.posts", counts.iter().sum())) });
        let ids: Vec<_> = authors.iter().filter_map(|(a, _)| report.authors.get(*a).map(|f| f.id.clone())).collect();
        let entity = cx.entity().downgrade();
        let open = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let id = ids[i].clone();
            let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenAuthor(id)));
        });
        let chart = ColumnChart { id: "insights-authors".into(), columns: columns.collect(), labels: authors.iter().map(|(a, _)| name(*a)).collect(), height: theme.insights_chart_height, on_click: Some(open), theme: theme.clone() };
        let summary = authors.first().map_or_else(|| tr("insights.summary.no_posts"), |(a, c)| t!("insights.summary.authors", name = name(*a), count = c.iter().sum::<usize>()).to_string());
        card(theme).child(card_title(LucideIcon::Users, tr("insights.card.authors"), None, theme)).child(chart).child(summary_line(summary, theme))
    }

    fn histogram_card(&self, id: &'static str, title: String, histogram: &Histogram, summary: String) -> Div {
        let theme = &self.theme;
        let range = |i: usize| {
            let (low, high) = histogram.bin(i);
            format!("{}–{}", compact(low), compact(high))
        };
        let columns = histogram.counts.iter().enumerate().map(|(i, &count)| Column { segments: vec![(count as f64, theme.info)], tip: format!("{}: {}", range(i), crate::i18n::t_count("insights.posts", count)) });
        let labels = (0..histogram.counts.len()).map(|i| compact(histogram.bin(i).0)).collect();
        let chart = ColumnChart { id: id.into(), columns: columns.collect(), labels, height: theme.insights_chart_height, on_click: None, theme: theme.clone() };
        card(theme).child(card_title(LucideIcon::ChartColumn, title, None, theme)).child(chart).child(summary_line(summary, theme))
    }

    pub(super) fn render_activity(&self, report: &Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let activity = &report.activity;
        let lengths = self.histogram_card("insights-lengths", tr("insights.card.lengths"), &activity.lengths, t!("insights.summary.lengths", median = compact(activity.lengths.median)).to_string());
        let sentences = self.histogram_card("insights-sentences", tr("insights.card.sentences"), &activity.sentence_lengths, t!("insights.summary.sentences", median = compact(activity.sentence_lengths.median)).to_string());
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.timeline_card(report, theme.insights_chart_height, None, cx))
            .child(pair(room, self.rhythm_card(report, cx), self.radar_card(report, cx), theme))
            .child(self.authors_card(report, cx))
            .child(pair(room, lengths, sentences, theme))
    }
}
