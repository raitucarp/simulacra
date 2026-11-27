//! The Network view's side panel: the ranking by the chosen measure (rows pick the node; the
//! picked row opens it) and the picked node's details, plus the "fit to view" button.

use super::super::common::tr;
use super::colours::node_colour;
use crate::domain::insights::graph::metrics::Measure;
use crate::domain::insights::InsightsReport;
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::ticks::{compact, percent};
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::base::Disableable;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Rows of the ranking.
const RANKED_ROWS: usize = 12;

/// The value of `measure` as shown in lists.
pub fn measure_text(measure: Measure, value: f64) -> String {
    match measure {
        Measure::Degree => compact(value),
        Measure::Influence => format!("{value:.4}"),
        Measure::Betweenness => format!("{value:.3}"),
        Measure::Clustering => percent(value),
    }
}

impl InsightsPanel {
    pub(super) fn reset_button(&self, cx: &mut Context<Self>) -> AnyElement {
        Button::new("insights-network-reset")
            .ghost()
            .xsmall()
            .icon(Icon::new(LucideIcon::Maximize2))
            .label(tr("insights.fit_view"))
            .disabled(self.network.pan.is_reset())
            .on_click(cx.listener(|panel, _, _, cx| {
                panel.network.reset();
                cx.notify();
            }))
            .into_any_element()
    }

    fn picked_details(&self, report: &InsightsReport, i: usize, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let m = &report.network.metrics;
        let line = |text: String| div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(text);
        let degree = m.degrees[i];
        let open = Button::new("insights-network-open").small().primary().label(tr("insights.open")).on_click(cx.listener(move |panel, _, _, cx| panel.open_node(i, cx)));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .p(theme.card_padding)
            .rounded(theme.radius_md)
            .bg(theme.bg_elevated)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(report.network.labels[i].clone()))
            .child(line(t!("insights.tip.degree", total = degree.total, incoming = degree.incoming, outgoing = degree.outgoing).to_string()))
            .child(line(t!("insights.tip.influence", value = measure_text(Measure::Influence, m.value(Measure::Influence, i))).to_string()))
            .child(line(t!("insights.tip.betweenness", value = measure_text(Measure::Betweenness, m.value(Measure::Betweenness, i))).to_string()))
            .child(line(t!("insights.tip.clustering", value = percent(m.value(Measure::Clustering, i))).to_string()))
            .child(line(t!("insights.tip.community", number = m.communities.of_node[i] + 1).to_string()))
            .child(div().flex().child(open))
    }

    pub(super) fn network_side(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let measure = self.state.measure;
        let top = report.network.metrics.top(measure, RANKED_ROWS, |_| true);
        let rows: Vec<RankedRow> = top
            .iter()
            .map(|&(i, value)| RankedRow { label: report.network.labels[i].clone(), value, value_text: measure_text(measure, value), colour: node_colour(report, &self.state, i, theme), active: self.selected == Some(i) })
            .collect();
        let nodes: Vec<usize> = top.iter().map(|(i, _)| *i).collect();
        let entity = cx.entity().downgrade();
        let pick = Rc::new(move |row: usize, _: &mut Window, cx: &mut App| {
            let node = nodes[row];
            let _ = entity.update(cx, |panel, cx| {
                if panel.selected == Some(node) {
                    panel.open_node(node, cx);
                }
                panel.selected = Some(node);
                cx.notify();
            });
        });
        let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(t!("insights.ranked_by", measure = tr(&format!("insights.measure.{}", measure.key()))).to_string());
        let details = self.selected.filter(|&i| i < report.network.nodes.len()).map(|i| self.picked_details(report, i, cx));
        // Under the graph the picked node's details sit beside the list, which then keeps a
        // readable length; in the narrow side column they wrap below it.
        let list = div().flex_1().min_w(theme.insights_side_width).child(ranked_list("insights-network-ranked", &rows, Some(pick), theme));
        let body = div().flex().flex_wrap().items_start().gap(theme.section_gap).child(list).children(details.map(|d| d.w(theme.insights_side_width).flex_none()));
        div().min_w_0().flex().flex_col().gap(theme.stack_gap).child(heading).child(body)
    }
}
