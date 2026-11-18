//! Network: graph metrics of the chosen scope (knowledge, thoughts or concepts) with the
//! graph drawn by community, kind or measure, a ranked list by the chosen measure, the picked
//! node's details, and the graph-level numbers.

mod canvas;
mod colours;
mod paint;
mod side;

use super::common::{card, card_title, option_chip, option_group, tile, tile_row, tr};
use super::Room;
use crate::domain::graph::NODE_KINDS;
use crate::domain::insights::graph::metrics::Measure;
use crate::domain::insights::{GraphScope, InsightsReport};
use crate::ui::features::insights::chart::legend::{legend, LegendEntry, ToggleCallback};
use crate::ui::features::insights::chart::palette::{node_kind_colour, series, SERIES_COUNT};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::{compact, percent};
use crate::ui::features::insights::panel::{InsightsPanel, LegendId};
use crate::ui::features::insights::state::NetworkColour;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

impl InsightsPanel {
    fn network_controls(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let id = |group: &str, key: &str| ElementId::Name(format!("insights-net-{group}-{key}").into());
        let scopes = GraphScope::ALL.map(|scope| option_chip(id("scope", scope.key()), tr(&format!("insights.scope.{}", scope.key())), self.state.scope == scope, theme, self.pick(cx, move |s| s.scope = scope)));
        let measures = Measure::ALL.map(|m| option_chip(id("measure", m.key()), tr(&format!("insights.measure.{}", m.key())), self.state.measure == m, theme, self.pick(cx, move |s| s.measure = m)));
        let colours = NetworkColour::ALL.map(|c| option_chip(id("colour", c.key()), tr(&format!("insights.colour.{}", c.key())), self.state.network_colour == c, theme, self.pick(cx, move |s| s.network_colour = c)));
        div()
            .w_full()
            .flex()
            .flex_wrap()
            .gap_x(theme.section_gap)
            .gap_y(theme.spacing_sm)
            .child(option_group(tr("insights.control.scope"), scopes.into(), theme))
            .child(option_group(tr("insights.control.measure"), measures.into(), theme))
            .child(option_group(tr("insights.control.colour"), colours.into(), theme))
    }

    fn network_numbers(&self, report: &InsightsReport) -> Div {
        let theme = &self.theme;
        let n = &report.network;
        let m = &n.metrics;
        let tiles = [
            (compact(n.nodes.len() as f64), "insights.tile.nodes"),
            (compact(n.graph.edge_count() as f64), "insights.tile.edges"),
            (format!("{:.4}", m.density), "insights.tile.density"),
            (format!("{} · {}", m.components.count(), percent(m.components.giant_share())), "insights.tile.components"),
            (compact(m.diameter as f64), "insights.tile.diameter"),
            (format!("{} · {}", m.cuts.bridges.len(), m.cuts.articulation_points.len()), "insights.tile.cuts"),
        ];
        tile_row(theme).children(tiles.into_iter().map(|(value, key)| tile(value, tr(key), theme.text_primary, theme)))
    }

    fn network_legend(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let hidden = |i: usize| self.is_hidden(LegendId::Network, i);
        let entries: Vec<LegendEntry> = match self.state.network_colour {
            NetworkColour::Community => {
                let sizes = &report.network.metrics.communities.sizes;
                let named = sizes.iter().take(SERIES_COUNT).enumerate().map(|(i, size)| LegendEntry { colour: series(i, theme), label: t!("insights.legend.community", number = i + 1).to_string(), value: Some(compact(*size as f64)), hidden: hidden(i) });
                let rest: usize = sizes.iter().skip(SERIES_COUNT).sum();
                let other = (rest > 0).then(|| LegendEntry { colour: theme.text_muted, label: tr("insights.legend.other"), value: Some(compact(rest as f64)), hidden: hidden(SERIES_COUNT) });
                named.chain(other).collect()
            }
            NetworkColour::Kind => NODE_KINDS.iter().enumerate().map(|(i, k)| LegendEntry { colour: node_kind_colour(*k, theme), label: tr(&format!("insights.node_kind.{}", i)), value: None, hidden: hidden(i) }).collect(),
            NetworkColour::Measure => vec![LegendEntry { colour: theme.accent_brand, label: tr(&format!("insights.measure.{}", self.state.measure.key())), value: None, hidden: false }],
        };
        let entity = cx.entity().downgrade();
        let toggle: ToggleCallback = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.toggle_legend(LegendId::Network, i, cx));
        });
        let toggles = (self.state.network_colour != NetworkColour::Measure).then_some(toggle);
        legend("insights-network-legend", &entries, toggles, theme)
    }

    fn network_summary(&self, report: &InsightsReport) -> String {
        let m = &report.network.metrics;
        let top = m.top(self.state.measure, 1, |_| true).first().map(|(i, _)| report.network.labels[*i].clone()).unwrap_or_default();
        let mut text = t!("insights.summary.network", nodes = report.network.nodes.len(), edges = report.network.graph.edge_count(), communities = m.communities.count(), modularity = format!("{:.2}", m.communities.modularity), top = top).to_string();
        if m.betweenness_sampled {
            text.push(' ');
            text.push_str(&tr("insights.summary.sampled"));
        }
        text
    }

    pub(super) fn render_network(&self, report: &Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        if report.network.nodes.is_empty() {
            return self.view_empty("insights.empty.network", "insights.empty.network_hint");
        }
        let canvas = div().flex_1().min_w_0().flex().flex_col().gap(theme.stack_gap).child(self.network_canvas(report, cx)).child(self.network_legend(report, cx));
        let side = self.network_side(report, cx);
        let main = if room == Room::Wide { div().w_full().flex().items_start().gap(theme.section_gap).child(canvas).child(side.w(theme.insights_side_width).flex_none()) } else { div().w_full().flex().flex_col().gap(theme.section_gap).child(canvas).child(side) };
        let body = card(theme)
            .child(card_title(LucideIcon::Network, tr("insights.card.network"), Some(self.reset_button(cx)), theme))
            .child(self.network_controls(cx))
            .child(main)
            .child(summary_line(self.network_summary(report), theme));
        div().w_full().flex().flex_col().gap(theme.section_gap).child(self.network_numbers(report)).child(body)
    }
}
