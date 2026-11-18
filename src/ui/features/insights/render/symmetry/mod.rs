//! Symmetry (group theory): the largest reply threads with the order of their automorphism
//! group and its orbits (drawn), the turn-taking permutation of their authors in cycle
//! notation, and the structurally equivalent posts found by colour refinement.

mod lists;
pub mod tree;

use super::common::{card, card_title, pair, tile, tile_row, tr};
use super::Room;
use crate::domain::insights::report::ThreadSymmetry;
use crate::domain::insights::InsightsReport;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::compact;
use crate::ui::features::insights::chart::tooltip::{floating_tooltip, tooltip_card};
use crate::ui::features::insights::panel::{Hover, InsightsPanel};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const TIP_HEIGHT: f32 = 70.0;
/// Largest group order written out in full; beyond it a power of ten.
const LOG_THRESHOLD: f64 = 9.0;

/// `|Aut|` as text: the factorial product and its value (or its power of ten).
pub fn group_order(thread: &ThreadSymmetry) -> String {
    let s = &thread.symmetry;
    if s.factorials.is_empty() {
        return "1".to_string();
    }
    let product = s.factorials.iter().map(|f| format!("{f}!")).collect::<Vec<_>>().join("·");
    match s.order.filter(|_| s.log10_order < LOG_THRESHOLD) {
        Some(order) => format!("{product} = {order}"),
        None => format!("{product} ≈ 10^{:.1}", s.log10_order),
    }
}

/// Cycle notation of the rotation with author names: `(Alice Bob)(Carol)`.
pub fn cycle_notation(thread: &ThreadSymmetry, report: &InsightsReport) -> String {
    let name = |a: &usize| report.authors.get(*a).map_or_else(String::new, |f| f.name.clone());
    thread.rotation.cycles.iter().map(|c| format!("({})", c.iter().map(name).collect::<Vec<_>>().join(" "))).collect()
}

impl InsightsPanel {
    fn tree_move(&mut self, at: (f32, f32), parents: &[Option<usize>], cx: &mut Context<Self>) {
        let reach = self.theme.insights_point_radius.as_f32() * 3.0;
        let hover = self.frames.tree.get().and_then(|frame| {
            let points = tree::place(&tree::tree_layout(parents), &frame, self.theme.insights_node_radius_max.as_f32());
            points.iter().enumerate().map(|(i, p)| (i, (p.0 - at.0).hypot(p.1 - at.1))).filter(|(_, d)| *d <= reach).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(i, _)| Hover::ThreadPost(i))
        });
        let hover = hover.unwrap_or(Hover::None);
        if hover != self.hover || self.pointer != at {
            (self.hover, self.pointer) = (hover, at);
            cx.notify();
        }
    }

    fn thread_card(&self, report: &Rc<InsightsReport>, thread: &ThreadSymmetry, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let hover = match self.hover { Hover::ThreadPost(i) => Some(i), _ => None };
        let colours = tree::orbit_colours(&thread.symmetry, theme);
        let surface = tree::tree_canvas(thread.parents.clone(), colours, hover, Rc::clone(&self.frames.tree), theme.clone());
        let tip = hover.zip(self.frames.tree.get()).and_then(|(i, frame)| {
            let post = report.posts.get(thread.posts[i])?;
            let author = report.authors.get(post.author).map(|a| a.name.clone()).unwrap_or_default();
            Some(floating_tooltip(&frame, self.pointer, tooltip_card(author, vec![post.excerpt.clone()], theme), TIP_HEIGHT, theme))
        });
        let (parents, posts) = (thread.parents.clone(), thread.posts.clone());
        let canvas = div()
            .id("insights-thread-tree")
            .relative()
            .w_full()
            .h(theme.insights_tree_height)
            .cursor_pointer()
            .on_mouse_move(cx.listener(move |p, e: &MouseMoveEvent, _, cx| p.tree_move((e.position.x.as_f32(), e.position.y.as_f32()), &parents, cx)))
            .on_hover(cx.listener(|p, hovered: &bool, _, cx| if !hovered { p.on_canvas_leave(cx) }))
            .on_click(cx.listener(move |p, _, _, cx| if let Hover::ThreadPost(i) = p.hover { p.open_post(posts[i], cx) }))
            .child(surface)
            .children(tip);
        let orbits = thread.symmetry.nontrivial_orbits().len();
        let summary = t!("insights.summary.thread", posts = thread.posts.len(), order = group_order(thread), orbits = orbits, depth = thread.symmetry.depth).to_string();
        let numbers = tile_row(theme)
            .child(tile(group_order(thread), tr("insights.tile.group_order"), theme.accent_brand, theme))
            .child(tile(compact(orbits as f64), tr("insights.tile.orbits"), theme.accent_success, theme))
            .child(tile(cycle_notation(thread, report), t!("insights.tile.rotation", order = thread.rotation.order, parity = tr(if thread.rotation.even { "insights.parity.even" } else { "insights.parity.odd" })).to_string(), theme.accent_dialectic, theme));
        card(theme)
            .child(card_title(LucideIcon::GitBranch, tr("insights.card.thread"), None, theme))
            .child(numbers)
            .child(canvas)
            .child(summary_line(summary, theme))
            .child(summary_line(tr("insights.explain.automorphisms"), theme))
    }

    pub(super) fn render_symmetry(&self, report: &Rc<InsightsReport>, room: Room, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let threads = &report.symmetry.threads;
        let Some(thread) = threads.get(self.state.thread.min(threads.len().saturating_sub(1))) else {
            return self.view_empty("insights.empty.symmetry", "insights.empty.symmetry_hint");
        };
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.thread_card(report, thread, cx))
            .child(pair(room, self.thread_list(report, cx), self.equivalence_card(report, cx), theme))
    }
}
