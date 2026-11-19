//! Embedding map: posts placed by meaning (2D principal components of their embeddings),
//! clustered and coloured by cluster, author, kind or time; hovering joins a post to its
//! nearest neighbours in meaning, clicking opens it.

mod paint;

use super::common::{card, card_title, option_chip, option_group, tr};
use super::Room;
use crate::domain::insights::InsightsReport;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::features::insights::chart::legend::{legend, LegendEntry, ToggleCallback};
use crate::ui::features::insights::chart::palette::{kind_colour, series, SERIES_COUNT};
use crate::ui::features::insights::chart::projection::{fit_of, project, rect_of, ProjectionKey};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::insights::panel::interact::CanvasSurface;
use crate::ui::features::insights::panel::{Hover, InsightsPanel, LegendId};
use crate::ui::features::insights::state::MapColour;
use crate::ui::features::mind_graph::viewport::Frame;
use crate::ui::icons::LucideIcon;
use crate::domain::insights::PostKind;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use paint::MapPainter;
use rust_i18n::t;
use std::collections::HashSet;
use std::rc::Rc;

/// Neighbours joined to the hovered point.
const NEIGHBOURS: usize = 5;
const TIP_HEIGHT: f32 = 260.0;
/// Neighbours listed in the tooltip.
const TIP_NEIGHBOURS: usize = 3;

impl InsightsPanel {
    fn hovered_point(&self) -> Option<usize> {
        match self.hover { Hover::Point(i) => Some(i), _ => None }
    }

    fn map_neighbours(&self, report: &InsightsReport) -> Rc<Vec<(usize, f32)>> {
        let i = self.hovered_point().unwrap_or(usize::MAX);
        self.map.neighbours.get(&(self.generation, i), || report.embedding.neighbours(i, NEIGHBOURS))
    }

    fn map_surface(&self, report: Rc<InsightsReport>) -> AnyElement {
        let (cache, slot, view) = (Rc::clone(&self.map.cache), Rc::clone(&self.map.frame), self.map.pan.view);
        let (colouring, theme, generation, hovered) = (self.state.map_colour, self.theme.clone(), self.generation, self.hovered_point());
        let neighbours = self.map_neighbours(&report);
        let hidden: HashSet<usize> = self.hidden.iter().filter(|(l, _)| *l == LegendId::Map).map(|(_, i)| *i).collect();
        let times = report.posts.iter().map(|p| p.created_at.timestamp());
        let span = (times.clone().min().unwrap_or_default(), times.max().unwrap_or_default());
        let (padding, cell) = (self.canvas_padding(), theme.insights_point_radius.as_f32() * 4.0);
        canvas(move |bounds, _, _| slot.set(Some(Frame::from(bounds))), move |bounds, (), window, cx| {
            let frame = Frame::from(bounds);
            let points = &report.embedding.points;
            let projected = cache.get(&ProjectionKey::new(generation, &frame, &view), || project(points, &frame, &view, fit_of(points, padding), cell));
            let painter = MapPainter { report: &report, projected: &projected, colouring, span, hovered, neighbours: &neighbours, hidden: &hidden, frame: rect_of(&frame), theme: &theme };
            painter.paint(window, cx);
        })
        .absolute().top_0().left_0().size_full().into_any_element()
    }

    fn map_tip(&self, report: &InsightsReport, i: usize) -> Option<Div> {
        let post = report.posts.get(*report.embedding_posts.get(i)?)?;
        let author = report.authors.get(post.author).map(|a| a.name.clone()).unwrap_or_default();
        let near = self.map_neighbours(report).iter().take(TIP_NEIGHBOURS).filter_map(|(j, score)| {
            let other = report.posts.get(*report.embedding_posts.get(*j)?)?;
            Some(format!("{:.2} · {}", score, other.excerpt))
        }).collect::<Vec<_>>();
        let mut lines = vec![post.excerpt.clone(), tr("insights.map.nearest")];
        lines.extend(near);
        let frame = self.map.frame.get()?;
        Some(floating_tooltip(&frame, self.pointer, tooltip_card(author, lines, &self.theme), TIP_HEIGHT, &self.theme))
    }

    fn map_canvas(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let tip = self.hovered_point().filter(|_| !self.map.pan.is_dragging()).and_then(|i| self.map_tip(report, i));
        div()
            .id("insights-map-canvas")
            .relative()
            .w_full()
            .h(theme.insights_canvas_height)
            .min_h(theme.insights_canvas_min_height)
            .overflow_hidden()
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_app)
            .tab_stop_ring(theme)
            .map(|d| if self.hovered_point().is_some() { d.cursor_pointer() } else { d.cursor_grab() })
            .on_mouse_move(cx.listener(|p, e: &MouseMoveEvent, _, cx| p.on_canvas_move(CanvasSurface::Map, e, cx)))
            .on_mouse_down(MouseButton::Left, cx.listener(|p, e: &MouseDownEvent, w, cx| p.on_canvas_press(CanvasSurface::Map, e, w, cx)))
            .on_mouse_up(MouseButton::Left, cx.listener(|p, e: &MouseUpEvent, _, cx| p.on_canvas_release(CanvasSurface::Map, e, cx)))
            .on_scroll(cx.listener(|p, e: &ScrollEvent, w, cx| p.on_canvas_scroll(CanvasSurface::Map, e, w, cx)))
            .on_hover(cx.listener(|p, hovered: &bool, _, cx| if !hovered { p.on_canvas_leave(cx) }))
            .child(self.map_surface(Rc::clone(report)))
            .children(tip)
    }

    fn map_legend(&self, report: &InsightsReport, cx: &mut Context<Self>) -> Option<Div> {
        let theme = &self.theme;
        let entry = |i: usize, colour: Rgba, label: String| LegendEntry { colour, label, value: None, hidden: self.is_hidden(LegendId::Map, i) };
        let entries: Vec<LegendEntry> = match self.state.map_colour {
            MapColour::Cluster => report.cluster_labels.iter().enumerate().map(|(i, l)| entry(i, series(i, theme), if l.is_empty() { t!("insights.legend.cluster", number = i + 1).to_string() } else { l.clone() })).collect(),
            MapColour::Author => report.authors.iter().take(SERIES_COUNT).enumerate().map(|(i, a)| entry(i, series(i, theme), a.name.clone())).collect(),
            MapColour::Kind => PostKind::ALL.iter().map(|k| entry(k.index(), kind_colour(*k, theme), tr(&format!("insights.kind.{}", k.key())))).collect(),
            MapColour::Time => return Some(summary_line(tr("insights.map.time_legend"), theme)),
        };
        let entity = cx.entity().downgrade();
        let toggle: ToggleCallback = Rc::new(move |i, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.toggle_legend(LegendId::Map, i, cx));
        });
        Some(legend("insights-map-legend", &entries, Some(toggle), theme))
    }

    pub(super) fn render_embedding(&self, report: &Rc<InsightsReport>, _room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        if report.embedding.len() < 2 {
            return self.view_empty("insights.empty.embedding", "insights.empty.embedding_hint");
        }
        let chips = MapColour::ALL.map(|c| option_chip(ElementId::Name(format!("insights-map-{}", c.key()).into()), tr(&format!("insights.colour.{}", c.key())), self.state.map_colour == c, theme, self.pick(cx, move |s| s.map_colour = c)));
        let fit = Button::new("insights-map-fit").ghost().xsmall().icon(Icon::new(LucideIcon::Maximize2)).label(tr("insights.fit_view")).on_click(cx.listener(|p, _, _, cx| { p.map.reset(); cx.notify(); }));
        let summary = t!("insights.summary.embedding", count = report.embedding.len(), clusters = report.embedding.cluster_count).to_string();
        card(theme)
            .child(card_title(LucideIcon::Compass, tr("insights.card.embedding"), Some(fit.into_any_element()), theme))
            .child(option_group(tr("insights.control.colour"), chips.into(), theme))
            .child(self.map_canvas(report, cx))
            .children(self.map_legend(report, cx))
            .child(summary_line(summary, theme))
    }
}
