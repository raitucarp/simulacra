//! The Argument view: a toolbar (how many claims stand, map with AI, add a statement), the
//! extraction status, the canvas with its legend, the findings, the node editor and the
//! statement list, in one scrolling column sized by the inspector.

use super::super::labels::{edge_colour, tr};
use super::canvas::geometry::columns_for;
use super::canvas::{ArgumentCanvas, NodeChip, Stroke};
use super::{ArgumentEvent, ArgumentPanel, ArgumentThread, ExtractStatus};
use crate::domain::thinking::argument::layout;
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, SpotKey};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SCROLL_ID: &str = "argument-scroll";
const CANVAS_ID: &str = "argument-canvas";
const EXTRACT_ID: &str = "argument-extract";
const CANCEL_ID: &str = "argument-cancel";
const ADD_ID: &str = "argument-add";

impl Render for ArgumentPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let probe = self.width_probe(cx);
        let measured = self.width.map(|width| width - theme.panel_padding * 2.0);
        let body = match self.thread.clone() {
            None => self.render_idle().into_any_element(),
            Some(thread) if thread.map.is_empty() => self.render_unmapped(&thread, cx).into_any_element(),
            Some(thread) => self.render_map(&thread, measured.unwrap_or(theme.thinking_chip_width * 2.0), cx).into_any_element(),
        };
        div()
            .id(SCROLL_ID)
            .relative()
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_y_scroll()
            .child(div().relative().w_full().flex().flex_col().gap(theme.stack_gap).p(theme.panel_padding).child(body))
            .child(probe)
    }
}

impl ArgumentPanel {
    /// Measures the view after layout and re-renders it when its width changed.
    fn width_probe(&self, cx: &mut Context<Self>) -> AnyElement {
        let entity = cx.entity().downgrade();
        let measure = move |bounds: Bounds<Pixels>, _: &mut Window, cx: &mut App| {
            let width = Some(bounds.size.width);
            let _ = entity.update(cx, |panel, cx| {
                if panel.width != width {
                    panel.width = width;
                    cx.notify();
                }
            });
        };
        canvas(measure, |_, _, _, _| {}).absolute().top_0().left_0().size_full().into_any_element()
    }

    fn render_idle(&self) -> Div {
        let art = Illustration::spot(SpotKey::Debate).width(self.theme.art_spot_width);
        art_state(ArtState::titled(art, tr("thinking.argument.idle_title"), tr("thinking.argument.idle_hint")), &self.theme)
    }

    fn render_unmapped(&self, thread: &ArgumentThread, cx: &mut Context<Self>) -> Div {
        let art = Illustration::spot(SpotKey::Debate).width(self.theme.art_spot_width);
        let state = art_state(ArtState::titled(art, tr("thinking.argument.empty_title"), tr("thinking.argument.empty_hint")), &self.theme);
        let buttons = div().flex().flex_wrap().justify_center().gap(self.theme.control_gap).child(self.extract_button(&thread.root, cx)).child(self.add_button(cx));
        div().w_full().flex().flex_col().items_center().gap(self.theme.stack_gap).child(state).child(buttons).children(self.status_line(cx)).children(self.render_editor(cx))
    }

    fn render_map(&self, thread: &ArgumentThread, width: Pixels, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let placed = layout(&thread.map, columns_for(width, theme));
        let edited = self.edited_id();
        let chips = thread.map.nodes.iter().zip(&self.report.standing).map(|(node, standing)| NodeChip { role: node.role, text: node.text.clone(), standing: *standing, selected: edited == Some(node.id.as_str()) });
        let strokes = thread.map.indexed_edges().into_iter().map(|(from, to, kind)| Stroke { from, to, colour: edge_colour(kind, theme) }).collect();
        let entity = cx.entity().downgrade();
        let on_node = Rc::new(move |index: usize, window: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.pick_node(index, window, cx));
        });
        let canvas = ArgumentCanvas { id: CANVAS_ID.into(), width, chips: chips.collect(), positions: placed.positions, rows: placed.rows, strokes, on_node: Some(on_node), theme: theme.clone() };
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.toolbar(thread, cx))
            .children(self.status_line(cx))
            .children(self.linking.map(|_| self.hint_line(tr("thinking.argument.linking_hint"))))
            .child(canvas)
            .child(self.legend())
            .children(self.render_findings(thread, cx))
            .children(self.render_editor(cx))
            .child(self.render_list(thread, cx))
    }

    fn toolbar(&self, thread: &ArgumentThread, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let claims = thread.map.nodes.iter().filter(|n| n.role == crate::domain::thinking::argument::NodeRole::Claim).count();
        let summary = t!("thinking.argument.claims_stand", stand = self.report.standing_claims(&thread.map), total = claims).to_string();
        let label = div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(summary);
        div().w_full().flex().flex_wrap().items_center().gap(theme.control_gap).child(label).child(self.extract_button(&thread.root, cx)).child(self.add_button(cx))
    }

    fn extract_button(&self, root: &crate::domain::PostId, cx: &mut Context<Self>) -> AnyElement {
        if self.status == ExtractStatus::Running {
            return Button::new(CANCEL_ID).ghost().xsmall().icon(Icon::new(LucideIcon::CircleStop)).label(tr("thinking.argument.stop")).on_click(cx.listener(|panel, _, _, cx| panel.request(ArgumentEvent::Cancel, cx))).into_any_element();
        }
        let root = root.clone();
        let label = if self.thread.as_ref().is_some_and(|t| t.map.is_empty()) { "thinking.argument.extract" } else { "thinking.argument.remap" };
        Button::new(EXTRACT_ID).ghost().xsmall().icon(Icon::new(LucideIcon::WandSparkles)).label(tr(label)).on_click(cx.listener(move |panel, _, _, cx| panel.request(ArgumentEvent::Extract(root.clone()), cx))).into_any_element()
    }

    fn add_button(&self, cx: &mut Context<Self>) -> Button {
        Button::new(ADD_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Plus)).label(tr("thinking.argument.add")).on_click(cx.listener(|panel, _, window, cx| panel.compose(window, cx)))
    }

    /// The extraction's progress or result, when there is something to say.
    fn status_line(&self, _: &mut Context<Self>) -> Option<Div> {
        match &self.status {
            ExtractStatus::Idle | ExtractStatus::Done { dropped: 0 } => None,
            ExtractStatus::Running => Some(self.hint_line(tr("thinking.argument.running"))),
            ExtractStatus::Failed(reason) => Some(self.hint_line(reason.clone()).text_color(self.theme.accent_danger)),
            ExtractStatus::Done { dropped } => Some(self.hint_line(t!("thinking.argument.dropped", count = dropped).to_string())),
        }
    }

    pub(super) fn hint_line(&self, text: String) -> Div {
        div().w_full().text_size(self.theme.font_size_2xs).line_height(relative(self.theme.line_height_body)).text_color(self.theme.text_muted).child(text)
    }
}
