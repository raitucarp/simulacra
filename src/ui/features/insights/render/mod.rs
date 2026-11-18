//! The Insights page: header with the view tabs, the filter bar and the open view, in a
//! centred column. Layout follows the column's measured width (not the window's), so the
//! page arranges itself the same in the main column, beside sidebars or full width.

mod activity;
mod category;
mod common;
mod embedding;
mod filters;
mod header;
mod network;
mod overview;
mod radial;
mod sets;
mod symmetry;
mod thinking;
#[cfg(test)]
mod tests;

use super::panel::InsightsPanel;
use super::view::InsightsView;
use crate::domain::insights::InsightsReport;
use crate::ui::components::measured_width;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const ROOT_ID: &str = "insights-root";
const SCROLL_ID: &str = "insights-scroll";
const WIDTH_ID: &str = "insights-width";

/// How much room the page has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Room {
    /// One column of charts.
    Narrow,
    /// Two columns where charts pair up.
    Medium,
    /// Side panels beside canvases.
    Wide,
}

impl Room {
    /// Room for a column `width` wide, against the tile width (`tile`): fewer than five tiles
    /// fit → narrow, fewer than eight → medium.
    pub fn of(width: Pixels, tile: Pixels) -> Self {
        let tiles = (width / tile).floor() as usize;
        match tiles {
            0..=4 => Self::Narrow,
            5..=7 => Self::Medium,
            _ => Self::Wide,
        }
    }
}

impl Render for InsightsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        crate::ui::render_stats::record(ROOT_ID);
        let theme = self.theme.clone();
        let (measured, probe) = measured_width(WIDTH_ID, window, cx);
        let room = Room::of(measured.unwrap_or(theme.insights_page_max_width).min(theme.insights_page_max_width), theme.insights_tile_width);
        let narrow = room == Room::Narrow;
        let column = div()
            .relative()
            .w_full()
            .max_w(theme.insights_page_max_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(probe)
            .child(self.render_header(room, cx))
            .when(!self.state.view.is_thinking(), |column| column.child(self.render_filters(room, cx)))
            .child(self.render_body(room, cx));
        div().id(ROOT_ID).size_full().bg(theme.bg_app).font_family(theme.font_family_ui.clone()).child(
            div()
                .id(SCROLL_ID)
                .size_full()
                .overflow_y_scroll()
                .flex()
                .justify_center()
                .items_start()
                .px(if narrow { theme.panel_padding_compact } else { theme.panel_padding })
                .py(theme.panel_padding)
                .child(column),
        )
    }
}

impl InsightsPanel {
    fn render_body(&self, room: Room, cx: &mut Context<Self>) -> AnyElement {
        if self.state.view.is_thinking() {
            return self.render_thinking(room, cx);
        }
        let Some(report) = self.report.clone() else { return self.render_loading().into_any_element() };
        if report.posts.is_empty() {
            return self.render_empty().into_any_element();
        }
        self.render_view(report, room, cx)
    }

    fn render_view(&self, report: Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> AnyElement {
        match self.state.view {
            InsightsView::Overview => self.render_overview(&report, room, cx).into_any_element(),
            InsightsView::Network => self.render_network(&report, room, cx).into_any_element(),
            InsightsView::Sets => self.render_sets(&report, room, cx).into_any_element(),
            InsightsView::Symmetry => self.render_symmetry(&report, room, cx).into_any_element(),
            InsightsView::Categories => self.render_categories(&report, room, cx).into_any_element(),
            InsightsView::Embedding => self.render_embedding(&report, room, cx).into_any_element(),
            InsightsView::Activity => self.render_activity(&report, room, cx).into_any_element(),
            InsightsView::Calibration | InsightsView::Tensions => self.render_thinking(room, cx),
        }
    }
}
