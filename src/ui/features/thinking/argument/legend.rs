//! The map's legend: the three standings and the two edge kinds, and a hint while nothing
//! is being edited.

use super::super::labels::{edge_colour, edge_label, standing_colour, standing_label, tr};
use super::{ArgumentPanel, EditorMode};
use crate::domain::thinking::argument::{EdgeKind, Standing};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

impl ArgumentPanel {
    pub(super) fn legend(&self) -> Div {
        let theme = &self.theme;
        let swatch = |colour: Rgba, label: String| div().flex().items_center().gap(theme.spacing_xs).child(div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_full).bg(colour)).child(label);
        let line = |colour: Rgba, label: String| div().flex().items_center().gap(theme.spacing_xs).child(div().flex_none().w(theme.insights_swatch_size * 2.0).h(theme.thinking_edge_width).bg(colour)).child(label);
        let standings = Standing::ALL.iter().map(|s| swatch(standing_colour(*s, theme), standing_label(*s)));
        div()
            .flex()
            .flex_wrap()
            .gap(theme.control_gap)
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_muted)
            .children(standings)
            .child(line(edge_colour(EdgeKind::Support, theme), edge_label(EdgeKind::Support)))
            .child(line(edge_colour(EdgeKind::Attack, theme), edge_label(EdgeKind::Attack)))
            .when(matches!(self.editor, EditorMode::Closed), |row| row.child(div().text_color(theme.text_muted).child(tr("thinking.argument.click_hint"))))
    }
}
