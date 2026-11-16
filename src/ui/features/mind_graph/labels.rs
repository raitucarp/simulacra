//! Text labels painted under focused graph nodes.

use super::paint::ScenePainter;
use gpui_kit::{fill, font, point, px, size, App, Bounds, SharedString, TextAlign, TextRun, Window};

/// Divisor centering a label horizontally on its node.
const HALF: f32 = 2.0;

impl ScenePainter<'_> {
    /// Paints the label of node `idx` centered below it on a bordered backdrop.
    pub(super) fn paint_label(&self, idx: usize, top: usize, window: &mut Window, cx: &mut App) {
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
        let line = window.text_system().shape_line(SharedString::from(text.clone()), font_size, &[run], None);
        let (x, y) = self.screen(idx);
        let pad = theme.spacing_xs;
        let origin = point(px(x) - line.width() / HALF, px(y + self.radius(idx, top)) + pad);
        let backdrop = Bounds::new(point(origin.x - pad, origin.y), size(line.width() + pad * HALF, line_height));
        window.paint_quad(
            fill(backdrop, theme.bg_elevated)
                .corner_radii(theme.radius_xs)
                .border_widths(theme.border_width_hairline)
                .border_color(theme.border),
        );
        let _ = line.paint(origin, line_height, TextAlign::Left, None, window, cx);
    }
}
