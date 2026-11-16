//! Sidebar mini Mind Graph: the same cached scene as the full view, fitted into a small
//! canvas (no labels), with matching node/edge counts. Clicking it opens the full graph.

use super::section::{render_divided_section, render_mono_badge, render_section_header, render_section_title};
use crate::state::graph::GraphScene;
use crate::theme::ThemeTokens;
use crate::ui::features::mind_graph::paint::{Focus, PaintOptions, ScenePainter};
use crate::ui::features::mind_graph::viewport::{Frame, Viewport};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Callback opening the full Mind Graph view.
pub type OpenGraphCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

const MINI_GRAPH_ID: &str = "sidebar-mini-graph";
/// Hub nodes in the mini graph are at most this many times the base radius.
const MINI_HUB_SCALE: f32 = 2.0;

fn render_graph_header(scene: &GraphScene, theme: &ThemeTokens) -> Div {
    let counts = t!(
        "graph.mini_counts",
        nodes = scene.model.node_count(),
        edges = scene.model.edge_count()
    );
    render_section_header(
        render_section_title(
            LucideIcon::Share2,
            theme.accent_agent,
            t!("sidebar.mind_graph_title").to_string(),
            theme,
        ),
        render_mono_badge(counts.to_string(), theme.accent_agent, theme).rounded_md(),
    )
}

fn mini_options(theme: &ThemeTokens) -> PaintOptions {
    let base = theme.graph_mini_node_radius.as_f32();
    PaintOptions {
        padding: theme.spacing_md.as_f32(),
        radius_range: (base, base * MINI_HUB_SCALE),
        labels: false,
    }
}

/// Clickable canvas painting the whole scene at the default viewport.
fn render_canvas(scene: Rc<GraphScene>, on_open: OpenGraphCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let painter_theme = theme.clone();
    let surface = canvas(
        |_, _, _| {},
        move |bounds, (), window, cx| {
            let options = mini_options(&painter_theme);
            let view = (Viewport::default(), Frame::from(bounds));
            ScenePainter::new(&scene, view, &options, Focus::default(), &painter_theme).paint(window, cx);
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full();
    div()
        .id(MINI_GRAPH_ID)
        .relative()
        .w_full()
        .h(theme.graph_canvas_height)
        .rounded(theme.radius_lg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_app)
        .overflow_hidden()
        .cursor_pointer()
        .hover(|s| s.border_color(theme.border_strong))
        .on_click(move |_, window, cx| on_open(window, cx))
        .child(surface)
}

/// Renders the mini Mind Graph section of the right sidebar.
pub fn render_mini_graph(scene: Rc<GraphScene>, on_open: OpenGraphCallback, theme: &ThemeTokens) -> impl IntoElement {
    render_divided_section(theme)
        .child(render_graph_header(&scene, theme))
        .child(render_canvas(scene, on_open, theme))
}
