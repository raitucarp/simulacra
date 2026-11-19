//! Painting the embedding map: culled points coloured by cluster, author, kind or time, the
//! hovered point's nearest neighbours in meaning joined to it, and cluster labels at their
//! centres.

use crate::domain::insights::InsightsReport;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::cull::{point_visible, Rect};
use crate::ui::features::insights::chart::paint::{dot, label, label_width, LabelStyle, StrokeBatch};
use crate::ui::features::insights::chart::palette::{faded, kind_colour, mix, series};
use crate::ui::features::insights::chart::projection::Projected;
use crate::ui::features::insights::state::MapColour;
use gpui_kit::*;
use std::collections::HashSet;

const HALF: f32 = 2.0;

/// Legend slot of point `i` under `colouring` (time has a single slot).
pub fn map_slot(report: &InsightsReport, colouring: MapColour, i: usize) -> usize {
    let post = report.embedding_posts.get(i).and_then(|&p| report.posts.get(p));
    match colouring {
        MapColour::Cluster => report.embedding.clusters.get(i).copied().unwrap_or_default(),
        MapColour::Author => post.map_or(0, |p| p.author),
        MapColour::Kind => post.map_or(0, |p| p.kind.index()),
        MapColour::Time => 0,
    }
}

/// Colour of point `i`; `span` is the (oldest, newest) post time in seconds.
pub fn map_colour(report: &InsightsReport, colouring: MapColour, i: usize, span: (i64, i64), theme: &ThemeTokens) -> Rgba {
    let post = report.embedding_posts.get(i).and_then(|&p| report.posts.get(p));
    match colouring {
        MapColour::Kind => post.map_or(theme.text_muted, |p| kind_colour(p.kind, theme)),
        MapColour::Time => {
            let at = post.map_or(span.0, |p| p.created_at.timestamp());
            let share = (at - span.0) as f32 / (span.1 - span.0).max(1) as f32;
            mix(theme.border_strong, theme.accent_brand, share)
        }
        _ => series(map_slot(report, colouring, i), theme),
    }
}

/// One frame of map drawing.
pub struct MapPainter<'a> {
    pub report: &'a InsightsReport,
    pub projected: &'a Projected,
    pub colouring: MapColour,
    pub span: (i64, i64),
    pub hovered: Option<usize>,
    pub neighbours: &'a [(usize, f32)],
    pub hidden: &'a HashSet<usize>,
    pub frame: Rect,
    pub theme: &'a ThemeTokens,
}

impl MapPainter<'_> {
    pub fn paint(&self, window: &mut Window, cx: &mut App) {
        let theme = self.theme;
        let radius = theme.insights_point_radius.as_f32();
        let near: HashSet<usize> = self.neighbours.iter().map(|(i, _)| *i).collect();
        if let Some(h) = self.hovered {
            let mut lines = StrokeBatch::new(theme.insights_edge_width, theme.accent_brand);
            self.neighbours.iter().for_each(|(i, _)| lines.line(self.projected.points[h], self.projected.points[*i], window));
            lines.finish(window);
        }
        for (i, &at) in self.projected.points.iter().enumerate() {
            if self.hidden.contains(&map_slot(self.report, self.colouring, i)) || !point_visible(at, radius, self.frame) {
                continue;
            }
            let colour = map_colour(self.report, self.colouring, i, self.span, theme);
            let focused = self.hovered == Some(i) || near.contains(&i);
            let colour = if self.hovered.is_none() || focused { colour } else { faded(colour, theme.graph_dim_opacity) };
            let ring = focused.then_some((theme.border_width_hairline, theme.text_primary));
            dot(window, at, if self.hovered == Some(i) { radius * HALF } else { radius }, colour, ring);
        }
        self.paint_cluster_labels(window, cx);
    }

    /// Each cluster's concept at the mean position of its members.
    fn paint_cluster_labels(&self, window: &mut Window, cx: &mut App) {
        if self.colouring != MapColour::Cluster {
            return;
        }
        let t = self.theme;
        let style = LabelStyle { family: t.font_family_ui.clone().into(), size: t.font_size_xs, line_height: t.font_size_xs * t.line_height_body, colour: t.text_primary, max_width: t.insights_label_max_width };
        for (cluster, members) in self.report.embedding.cluster_members().iter().enumerate() {
            let text = self.report.cluster_labels.get(cluster).filter(|l| !l.is_empty());
            let (Some(text), false) = (text, members.is_empty() || self.hidden.contains(&cluster)) else { continue };
            let (sx, sy) = members.iter().fold((0.0, 0.0), |(x, y), &m| (x + self.projected.points[m].0, y + self.projected.points[m].1));
            let centre = (sx / members.len() as f32, sy / members.len() as f32);
            if point_visible(centre, 0.0, self.frame) {
                let width = label_width(window, text, &style);
                label(window, cx, text, (centre.0 - width / HALF, centre.1 - style.line_height.as_f32() / HALF), &style);
            }
        }
    }
}
