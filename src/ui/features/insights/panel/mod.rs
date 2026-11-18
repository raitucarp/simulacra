//! The Insights page entity: the remembered page state, the latest report (computed by the
//! shell on the background executor), hover and legend choices, and the pan/zoom canvases.
//! State changes go out as [`InsightsEvent::Changed`]; the shell saves them and recomputes
//! the report when the filter or the network scope changed.

mod canvas_state;
mod hover;
pub mod interact;
pub mod keys;

pub use canvas_state::CanvasState;
pub use hover::{Hover, LegendId};

use super::events::InsightsEvent;
use super::state::PageState;
use super::view::InsightsView;
use crate::domain::insights::InsightsReport;
use crate::theme::ThemeTokens;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::{Context, EventEmitter, FocusHandle};
use std::cell::Cell;
use std::collections::HashSet;
use std::rc::Rc;

/// The canvas frames painted last (for hit testing outside the pan/zoom canvases).
#[derive(Default)]
pub struct Frames {
    pub rhythm: Rc<Cell<Option<Frame>>>,
    pub radar: Rc<Cell<Option<Frame>>>,
    pub venn: Rc<Cell<Option<Frame>>>,
    pub diagram: Rc<Cell<Option<Frame>>>,
    pub tree: Rc<Cell<Option<Frame>>>,
}

/// The Insights page.
pub struct InsightsPanel {
    pub(super) theme: ThemeTokens,
    pub(super) state: PageState,
    pub(super) report: Option<Rc<InsightsReport>>,
    /// Bumped whenever a new report arrives (keys the cached geometry).
    pub(super) generation: u64,
    /// A report for the current state is being computed.
    pub(super) pending: bool,
    pub(super) hover: Hover,
    /// Pointer position of the hover, in window pixels.
    pub(super) pointer: (f32, f32),
    /// The node picked in the Network view (its neighbourhood stays highlighted).
    pub(super) selected: Option<usize>,
    pub(super) hidden: HashSet<(LegendId, usize)>,
    pub(super) network: CanvasState,
    pub(super) map: CanvasState,
    pub(super) frames: Frames,
    pub(super) focus: FocusHandle,
    /// Authors of the network (id, name), busiest first, for the author filter.
    pub(super) authors: Vec<(String, String)>,
    pub(crate) thinking: crate::ui::features::thinking::insights::ThinkingInsights, // sb E1: Calibration and Tensions data
    pub(crate) lenses: super::lens_state::LensUi, // sb F3: the lens views
}

impl EventEmitter<InsightsEvent> for InsightsPanel {}

impl InsightsPanel {
    pub fn new(state: PageState, theme: ThemeTokens, cx: &mut Context<Self>) -> Self {
        Self {
            theme,
            state,
            report: None,
            generation: 0,
            pending: true,
            hover: Hover::None,
            pointer: (0.0, 0.0),
            selected: None,
            hidden: HashSet::new(),
            network: CanvasState::default(),
            map: CanvasState::default(),
            frames: Frames::default(),
            focus: cx.focus_handle(),
            authors: Vec::new(), thinking: Default::default(), lenses: Default::default(),
        }
    }

    /// The authors offered by the author filter.
    pub fn set_authors(&mut self, authors: Vec<(String, String)>, cx: &mut Context<Self>) {
        if self.authors != authors {
            self.authors = authors;
            cx.notify();
        }
    }

    pub fn state(&self) -> &PageState {
        &self.state
    }

    /// Swaps in a finished report.
    pub fn set_report(&mut self, report: Rc<InsightsReport>, cx: &mut Context<Self>) {
        self.report = Some(report);
        self.generation += 1;
        (self.pending, self.hover, self.selected) = (false, Hover::None, None);
        self.network.reset();
        self.map.reset();
        cx.notify();
    }

    /// Marks a recomputation as running (the previous report stays on screen).
    pub fn set_pending(&mut self, pending: bool, cx: &mut Context<Self>) {
        if self.pending != pending {
            self.pending = pending;
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Opens `view` (from the tabs or a palette command).
    pub fn show(&mut self, view: InsightsView, cx: &mut Context<Self>) {
        self.update_state(cx, |state| state.view = view);
    }

    /// Applies `change` to the page state, announcing it when it changed anything.
    pub(super) fn update_state(&mut self, cx: &mut Context<Self>, change: impl FnOnce(&mut PageState)) {
        let mut next = self.state.clone();
        change(&mut next);
        if next == self.state {
            return;
        }
        if next.view != self.state.view {
            self.hover = Hover::None;
        }
        self.state = next;
        cx.emit(InsightsEvent::Changed(self.state.clone()));
        cx.notify();
    }

    /// Toggles a legend entry; returns nothing, repaints.
    pub(super) fn toggle_legend(&mut self, legend: LegendId, index: usize, cx: &mut Context<Self>) {
        if !self.hidden.remove(&(legend, index)) {
            self.hidden.insert((legend, index));
        }
        cx.notify();
    }

    pub(super) fn is_hidden(&self, legend: LegendId, index: usize) -> bool {
        self.hidden.contains(&(legend, index))
    }
}
