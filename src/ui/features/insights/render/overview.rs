//! Overview: key numbers, then small multiples (posts over time, kinds, the most influential
//! items, the largest concepts), each opening its full view.

use super::super::panel::InsightsPanel;
use super::super::view::InsightsView;
use super::common::{card, card_title, pair, tile, tile_row, tr};
use super::Room;
use crate::domain::insights::graph::metrics::Measure;
use crate::domain::insights::sets::SetDimension;
use crate::domain::insights::{InsightsReport, PostKind};
use crate::ui::features::insights::chart::palette::{kind_colour, node_kind_colour, series};
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::{compact, percent};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Rows of each small ranking.
const SMALL_RANKING: usize = 5;

impl InsightsPanel {
    fn key_numbers(&self, report: &InsightsReport) -> Div {
        let theme = &self.theme;
        let metrics = &report.network.metrics;
        let agents = report.authors.iter().filter(|a| a.is_agent).count();
        let tiles = [
            (compact(report.posts.len() as f64), "insights.tile.posts", theme.accent_brand),
            (format!("{}/{}", agents, report.authors.len()), "insights.tile.agents", theme.accent_agent),
            (compact(report.documents as f64), "insights.tile.documents", theme.accent_user),
            (compact(metrics.communities.count() as f64), "insights.tile.communities", theme.accent_success),
            (format!("{:.2}", metrics.communities.modularity), "insights.tile.modularity", theme.accent_dialectic),
            (percent(metrics.average_clustering), "insights.tile.clustering", theme.info),
        ];
        tile_row(theme).children(tiles.into_iter().map(|(value, key, colour)| tile(value, tr(key), colour, theme)))
    }

    /// A "see more" button opening `view`.
    fn more_button(&self, card: &str, view: InsightsView, cx: &mut Context<Self>) -> AnyElement {
        Button::new(ElementId::Name(format!("insights-more-{card}-{}", view.key()).into()))
            .ghost()
            .xsmall()
            .label(tr("insights.more"))
            .on_click(cx.listener(move |panel, _, _, cx| panel.show(view, cx)))
            .into_any_element()
    }

    fn kinds_card(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows: Vec<RankedRow> = PostKind::ALL
            .iter()
            .map(|kind| {
                let count = report.activity.kinds[kind.index()];
                RankedRow { label: tr(&format!("insights.kind.{}", kind.key())), value: count as f64, value_text: compact(count as f64), colour: kind_colour(*kind, theme), active: false }
            })
            .collect();
        let top = PostKind::ALL.iter().max_by_key(|k| report.activity.kinds[k.index()]).copied().unwrap_or(PostKind::Thought);
        let summary = t!("insights.summary.kinds", kind = tr(&format!("insights.kind.{}", top.key())), share = percent(report.activity.kinds[top.index()] as f64 / report.posts.len().max(1) as f64)).to_string();
        card(theme)
            .child(card_title(LucideIcon::Layers, tr("insights.card.kinds"), Some(self.more_button("kinds", InsightsView::Activity, cx)), theme))
            .child(ranked_list("insights-overview-kinds", &rows, None, theme))
            .child(summary_line(summary, theme))
    }

    fn influence_card(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let network = &report.network;
        let top = network.metrics.top(Measure::Influence, SMALL_RANKING, |_| true);
        let rows: Vec<RankedRow> = top
            .iter()
            .map(|&(i, value)| RankedRow { label: network.labels[i].clone(), value, value_text: format!("{:.3}", value), colour: node_kind_colour(network.kind(i), theme), active: false })
            .collect();
        let nodes: Vec<usize> = top.iter().map(|(i, _)| *i).collect();
        let entity = cx.entity().downgrade();
        let open = Rc::new(move |row: usize, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.open_node(nodes[row], cx));
        });
        let summary = top.first().map_or_else(|| tr("insights.summary.no_network"), |&(i, _)| t!("insights.summary.influence", name = network.labels[i].clone()).to_string());
        card(theme)
            .child(card_title(LucideIcon::Network, tr("insights.card.influence"), Some(self.more_button("influence", InsightsView::Network, cx)), theme))
            .child(ranked_list("insights-overview-influence", &rows, Some(open), theme))
            .child(summary_line(summary, theme))
    }

    fn concepts_card(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let concepts = report.families.iter().find(|(d, _)| *d == SetDimension::Concept).map(|(_, s)| s.as_slice()).unwrap_or_default();
        let rows: Vec<RankedRow> = concepts
            .iter()
            .take(SMALL_RANKING)
            .enumerate()
            .map(|(i, set)| RankedRow { label: set.label.clone(), value: set.len() as f64, value_text: compact(set.len() as f64), colour: series(i, theme), active: false })
            .collect();
        let summary = concepts.first().map_or_else(|| tr("insights.summary.no_concepts"), |set| t!("insights.summary.concepts", name = set.label.clone(), count = set.len()).to_string());
        card(theme)
            .child(card_title(LucideIcon::Hash, tr("insights.card.concepts"), Some(self.more_button("concepts", InsightsView::Sets, cx)), theme))
            .child(ranked_list("insights-overview-concepts", &rows, None, theme))
            .child(summary_line(summary, theme))
    }

    pub(super) fn render_overview(&self, report: &Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let timeline = self.timeline_card(report, theme.insights_spark_height, Some(self.more_button("timeline", InsightsView::Activity, cx)), cx);
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.key_numbers(report))
            .child(pair(room, timeline, self.kinds_card(report, cx), theme))
            .child(pair(room, self.influence_card(report, cx), self.concepts_card(report, cx), theme))
    }
}
