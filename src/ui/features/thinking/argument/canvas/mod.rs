//! The argument canvas: node chips placed at the tiered layout's fractions of the container
//! (measured, so the map fills whatever width the inspector gives it) over a painted layer
//! of support and attack edges with arrowheads. A chip's frame shows its standing under the
//! grounded semantics: accepted chips are ringed in the success colour, rejected ones are
//! dimmed, undecided ones ringed in the warning colour.
//!
//! - `geometry`: chip boxes, edge ends and the canvas height (pure).
//! - `paint`: the edge strokes and arrowheads.

pub mod geometry;
mod paint;

use super::super::labels::{role_colour, role_label, standing_colour};
use crate::domain::thinking::argument::{NodeRole, Standing};
use crate::theme::ThemeTokens;
use geometry::{chip_origin, Frame};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// Lines of a chip's text.
const CHIP_LINES: usize = 2;

/// A click on node `index`.
pub type NodeAction = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// One edge to draw, by node index.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub from: usize,
    pub to: usize,
    pub colour: Rgba,
}

/// One node as its chip shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeChip {
    pub role: NodeRole,
    pub text: String,
    pub standing: Standing,
    pub selected: bool,
}

/// A map to draw.
#[derive(IntoElement)]
pub struct ArgumentCanvas {
    pub id: SharedString,
    /// The width the canvas fills (measured by its view).
    pub width: Pixels,
    pub chips: Vec<NodeChip>,
    pub positions: Vec<(f32, f32)>,
    pub rows: usize,
    pub strokes: Vec<Stroke>,
    pub on_node: Option<NodeAction>,
    pub theme: ThemeTokens,
}

impl ArgumentCanvas {
    fn chip(&self, index: usize, chip: &NodeChip, frame: &Frame) -> Stateful<Div> {
        let theme = &self.theme;
        let (x, y) = chip_origin(frame, self.positions.get(index).copied().unwrap_or_default());
        let ring = standing_colour(chip.standing, theme);
        let border = if chip.selected { theme.accent_brand } else { ring };
        let on_node = self.on_node.clone();
        let header = div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_full).bg(role_colour(chip.role, theme)))
            .child(div().min_w_0().truncate().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(role_label(chip.role)));
        div()
            .id(ElementId::NamedInteger(self.id.clone(), index as u64))
            .absolute()
            .left(px(x))
            .top(px(y))
            .w(px(frame.chip_width))
            .h(theme.thinking_chip_height)
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .px(theme.spacing_sm)
            .py(theme.spacing_xs)
            .rounded(theme.radius_md)
            .bg(if chip.standing == Standing::Accepted { theme.accent_success_subtle } else { theme.bg_card })
            .border(if chip.selected { theme.border_width_threadline } else { theme.border_width_hairline })
            .border_color(border)
            .when(chip.standing == Standing::Rejected && !chip.selected, |c| c.opacity(theme.forge_disabled_opacity))
            .hover(|c| c.border_color(theme.accent_brand_border))
            .overflow_hidden()
            .when_some(on_node, |c, action| c.cursor_pointer().on_click(move |_, window, cx| action(index, window, cx)))
            .child(header)
            .child(div().text_size(theme.font_size_2xs).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).line_clamp(CHIP_LINES).text_ellipsis().child(chip.text.clone()))
    }
}

impl RenderOnce for ArgumentCanvas {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme.clone();
        let frame = Frame::new(self.width, self.rows, &theme);
        let chips: Vec<Stateful<Div>> = self.chips.iter().enumerate().map(|(i, chip)| self.chip(i, chip, &frame)).collect();
        let (strokes, positions, width) = (Rc::new(self.strokes), Rc::new(self.positions), theme.thinking_edge_width.as_f32());
        let layer = canvas(|_, _, _| {}, move |bounds, (), window, _| paint::paint_strokes(&strokes, &positions, frame, bounds, width, window)).absolute().top_0().left_0().size_full();
        div()
            .relative()
            .w_full()
            .h(geometry::canvas_height(self.rows, &theme))
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_app)
            .overflow_hidden()
            .child(layer)
            .children(chips)
    }
}
