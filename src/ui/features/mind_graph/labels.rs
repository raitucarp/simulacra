//! Text labels painted under focused graph nodes, cut to `graph_label_max_width`.

use super::paint::ScenePainter;
use super::viewport::Frame;
use gpui_kit::{fill, font, point, px, size, App, Bounds, SharedString, TextAlign, TextRun, TruncateFrom, Window};

/// Divisor centering a label horizontally on its node.
const HALF: f32 = 2.0;
/// Marks a label cut to `graph_label_max_width`.
const ELLIPSIS: &str = "…";
/// From this zoom on, every lit node shows its label.
const LABEL_ALL_ZOOM: f32 = 2.5;
/// Every lit node shows its label when each node has at least this share of a square as wide
/// as the longest label (`graph_label_max_width`) to itself.
const LABEL_ROOM_SHARE: f32 = 0.5;

/// Whether a `frame` gives each of `count` nodes room for its label.
pub(super) fn room_for_labels(frame: &Frame, count: usize, label_width: f32) -> bool {
    let per_node = frame.width * frame.height / count.max(1) as f32;
    per_node >= label_width * label_width * LABEL_ROOM_SHARE
}

/// Top-left corner of a `(width, height)` label centered under `(x, y)`, kept inside `frame`
/// with an `inset` margin (near an edge it slides along it instead of being cut off).
pub(super) fn label_origin((x, y): (f32, f32), (width, height): (f32, f32), frame: &Frame, inset: f32) -> (f32, f32) {
    let left = (x - width / HALF).min(frame.x + frame.width - width - inset).max(frame.x + inset);
    let top = y.min(frame.y + frame.height - height - inset).max(frame.y + inset);
    (left, top)
}

impl ScenePainter<'_> {
    /// Labels of the focused nodes, or of every lit node when zoomed in or roomy.
    pub(super) fn paint_labels(&self, top: usize, window: &mut Window, cx: &mut App) {
        let count = self.scene.model.node_count();
        let roomy = room_for_labels(&self.frame, count, self.theme.graph_label_max_width.as_f32());
        let show_all = self.view.zoom >= LABEL_ALL_ZOOM || roomy;
        for idx in 0..self.scene.model.node_count() {
            let focused = self.focus.selected == Some(idx) || self.focus.hovered == Some(idx);
            if focused || (show_all && self.is_lit(idx)) {
                self.paint_label(idx, top, window, cx);
            }
        }
    }

    /// Paints the label of node `idx` centered below it on a bordered backdrop.
    fn paint_label(&self, idx: usize, top: usize, window: &mut Window, cx: &mut App) {
        let text = &self.scene.model.nodes()[idx].label;
        if text.is_empty() {
            return;
        }
        let theme = self.theme;
        let font_size = theme.font_size_xs;
        let line_height = font_size * theme.line_height_body;
        let run = TextRun {
            len: text.len(),
            font: font(theme.font_family_ui.clone()),
            color: theme.text_primary.into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let font = run.font.clone();
        let mut wrapper = cx.text_system().line_wrapper(font, font_size);
        let runs = [run];
        let (text, runs) = wrapper.truncate_line(SharedString::from(text.clone()), theme.graph_label_max_width, ELLIPSIS, &runs, TruncateFrom::End);
        let line = window.text_system().shape_line(text, font_size, &runs, None);
        let (x, y) = self.screen(idx);
        let pad = theme.spacing_xs;
        let below = (x, y + self.radius(idx, top) + pad.as_f32());
        let boxed = ((line.width() + pad * HALF).as_f32(), line_height.as_f32());
        let (left, top_y) = label_origin(below, boxed, &self.frame, pad.as_f32());
        let backdrop = Bounds::new(point(px(left), px(top_y)), size(px(boxed.0), line_height));
        let origin = point(backdrop.origin.x + pad, backdrop.origin.y);
        window.paint_quad(
            fill(backdrop, theme.bg_elevated)
                .corner_radii(theme.radius_xs)
                .border_widths(theme.border_width_hairline)
                .border_color(theme.border),
        );
        let _ = line.paint(origin, line_height, TextAlign::Left, None, window, cx);
    }
}
