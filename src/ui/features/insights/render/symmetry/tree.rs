//! A tidy drawing of one reply thread: leaves spread evenly in reading order, parents centred
//! over their children, one row per depth; posts in the same orbit share a colour.

use crate::domain::insights::symmetry::TreeSymmetry;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, StrokeBatch};
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;
use std::cell::Cell;
use std::rc::Rc;

/// Unit-square positions (x by leaf order, y by depth) of the forest given by `parents`.
pub fn tree_layout(parents: &[Option<usize>]) -> Vec<(f32, f32)> {
    let n = parents.len();
    let mut children = vec![Vec::new(); n];
    parents.iter().enumerate().filter_map(|(v, p)| p.map(|p| (p, v))).for_each(|(p, v)| children[p].push(v));
    let mut depth = vec![0usize; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut stack: Vec<usize> = (0..n).filter(|&v| parents[v].is_none()).rev().collect();
    while let Some(v) = stack.pop() {
        order.push(v);
        for &c in children[v].iter().rev() {
            depth[c] = depth[v] + 1;
            stack.push(c);
        }
    }
    let leaves: Vec<usize> = order.iter().copied().filter(|&v| children[v].is_empty()).collect();
    let mut x = vec![0.0f32; n];
    leaves.iter().enumerate().for_each(|(i, &v)| x[v] = (i as f32 + 0.5) / leaves.len() as f32);
    for &v in order.iter().rev().filter(|&&v| !children[v].is_empty()) {
        x[v] = children[v].iter().map(|&c| x[c]).sum::<f32>() / children[v].len() as f32;
    }
    let deepest = depth.iter().copied().max().unwrap_or(0).max(1) as f32;
    (0..n).map(|v| (x[v], depth[v] as f32 / deepest)).collect()
}

/// Positions of `layout` in `frame`, inset by `inset` pixels.
pub fn place(layout: &[(f32, f32)], frame: &Frame, inset: f32) -> Vec<(f32, f32)> {
    let (w, h) = ((frame.width - inset * 2.0).max(1.0), (frame.height - inset * 2.0).max(1.0));
    layout.iter().map(|(x, y)| (frame.x + inset + x * w, frame.y + inset + y * h)).collect()
}

/// Colour of each post: its orbit's series colour when the orbit has two or more posts.
pub fn orbit_colours(symmetry: &TreeSymmetry, theme: &ThemeTokens) -> Vec<Rgba> {
    let classes = symmetry.nontrivial_orbits();
    (0..symmetry.orbits.len())
        .map(|v| classes.iter().position(|c| c.contains(&v)).map_or(theme.text_muted, |i| series(i, theme)))
        .collect()
}

/// The thread canvas.
pub fn tree_canvas(parents: Vec<Option<usize>>, colours: Vec<Rgba>, hover: Option<usize>, slot: Rc<Cell<Option<Frame>>>, theme: ThemeTokens) -> AnyElement {
    let layout = tree_layout(&parents);
    canvas(move |bounds, _, _| slot.set(Some(Frame::from(bounds))), move |bounds, (), window, _| {
        let frame = Frame::from(bounds);
        let inset = theme.insights_node_radius_max.as_f32();
        let points = place(&layout, &frame, inset);
        let mut edges = StrokeBatch::new(theme.insights_edge_width, theme.graph_edge);
        parents.iter().enumerate().filter_map(|(v, p)| p.map(|p| (v, p))).for_each(|(v, p)| edges.line(points[p], points[v], window));
        edges.finish(window);
        let radius = theme.insights_point_radius.as_f32() * 1.5;
        for (v, &at) in points.iter().enumerate() {
            let ring = (hover == Some(v)).then_some((theme.border_width_threadline, theme.text_primary));
            dot(window, at, radius, colours[v], ring);
        }
    })
    .absolute().top_0().left_0().size_full().into_any_element()
}
