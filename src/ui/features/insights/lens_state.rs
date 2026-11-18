//! sb F3: what the lens views (Concept timeline, Viewpoint diversity, Readability) keep on
//! the Insights page: their report (computed by the shell on the background executor), the
//! timeline's zoom, brush, picked concept and hover, and the diversity view's chosen
//! dimension. None of it is persisted.

use super::panel::InsightsPanel;
use crate::domain::insights::lenses::diversity::Dimension;
use crate::domain::insights::lenses::timeline::detail::PeriodRange;
use crate::domain::insights::lenses::LensReport;
use crate::ui::features::insights::chart::lanes::CellCache;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::Context;
use std::cell::Cell;
use std::rc::Rc;

/// The timeline chart's interaction state.
#[derive(Default)]
pub(crate) struct TimelineUi {
    /// The periods shown (all when `None`).
    pub zoom: Option<PeriodRange>,
    /// A drag across the chart: window x where it started and where it is now.
    pub brush: Option<(f32, f32)>,
    /// The picked concept (a lane index).
    pub selected: Option<usize>,
    /// The hovered lane and period.
    pub hover: Option<(usize, usize)>,
    /// The chart's frame as painted last.
    pub frame: Rc<Cell<Option<Frame>>>,
    /// The chart's cells, cached per report, range, shown concepts and size.
    pub cells: Rc<CellCache>,
}

/// Everything the lens views keep.
#[derive(Default)]
pub(crate) struct LensUi {
    pub report: Option<Rc<LensReport>>,
    /// Bumped whenever a new report arrives.
    pub generation: u64,
    /// A report for the current filter is being computed.
    pub pending: bool,
    pub timeline: TimelineUi,
    /// The dimension whose shares the diversity view lists.
    pub dimension: Dimension,
}

impl InsightsPanel {
    /// Swaps in a finished lens report (the timeline's zoom and pick are reset).
    pub fn set_lenses(&mut self, report: Rc<LensReport>, cx: &mut Context<Self>) {
        let lenses = &mut self.lenses;
        lenses.report = Some(report);
        lenses.generation += 1;
        lenses.pending = false;
        (lenses.timeline.zoom, lenses.timeline.brush, lenses.timeline.selected, lenses.timeline.hover) = (None, None, None, None);
        cx.notify();
    }

    /// Marks a lens recomputation as running (the previous report stays on screen).
    pub fn set_lens_pending(&mut self, pending: bool, cx: &mut Context<Self>) {
        if self.lenses.pending != pending {
            self.lenses.pending = pending;
            cx.notify();
        }
    }
}
