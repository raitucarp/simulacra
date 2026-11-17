//! [`PersistedSplit`]: a resizable panel group restored from, and saved to, the layout store.

use super::fit::{fit_panes, PaneBounds};
use super::handle::reset_on_double_click;
use super::layouts::restored_size;
use super::store::{record, saved_size, state_for};
use crate::theme::ThemeTokens;
use gpui_kit::component::resizable::{h_resizable, resizable_panel, v_resizable, ResizablePanel};
use gpui_kit::*;
use std::ops::Range;

/// Size rule of one pane.
enum PaneSize {
    /// Opens at the saved size (else `default`) within `range`, and keeps its size when the
    /// window resizes.
    Sized { default: Pixels, range: Range<Pixels> },
    /// Takes the remaining space, never less than `min`.
    Flex { min: Pixels },
}

/// One pane of a [`PersistedSplit`].
pub struct SplitPane {
    size: PaneSize,
    visible: bool,
    children: Vec<AnyElement>,
}

impl SplitPane {
    /// A pane the user sizes: `default` until dragged, always within `range`.
    pub fn sized(default: Pixels, range: Range<Pixels>) -> Self {
        Self { size: PaneSize::Sized { default, range }, visible: true, children: Vec::new() }
    }

    /// The pane that takes the remaining space (at least `min`).
    pub fn flex(min: Pixels) -> Self {
        Self { size: PaneSize::Flex { min }, visible: true, children: Vec::new() }
    }

    /// Hides the pane (its saved size is kept).
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Its bounds along the axis, opening at the `saved` size when there is one.
    fn bounds(&self, saved: Option<f32>) -> PaneBounds {
        match &self.size {
            PaneSize::Sized { default, range } => {
                let (min, max) = (range.start.as_f32(), range.end.as_f32());
                PaneBounds { min, max, want: restored_size(saved, default.as_f32(), min, max), flex: false }
            }
            PaneSize::Flex { min } => PaneBounds { min: min.as_f32(), max: Pixels::MAX.as_f32(), want: min.as_f32(), flex: true },
        }
    }

    fn into_panel(self, fitted: PaneBounds) -> ResizablePanel {
        let panel = resizable_panel().visible(self.visible).size_range(px(fitted.min)..px(fitted.max));
        let panel = if fitted.flex { panel } else { panel.size(px(fitted.want)).fixed() };
        panel.children(self.children)
    }
}

impl ParentElement for SplitPane {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

/// A resizable group whose sizes persist under its layout id.
#[derive(IntoElement)]
pub struct PersistedSplit {
    id: SharedString,
    axis: Axis,
    hit_padding: Pixels,
    panes: Vec<SplitPane>,
}

/// Side-by-side panes (vertical splitters).
pub fn persisted_h_resizable(id: impl Into<SharedString>, theme: &ThemeTokens) -> PersistedSplit {
    PersistedSplit { id: id.into(), axis: Axis::Horizontal, hit_padding: theme.splitter_hit_padding, panes: Vec::new() }
}

/// Stacked panes (horizontal splitters).
pub fn persisted_v_resizable(id: impl Into<SharedString>, theme: &ThemeTokens) -> PersistedSplit {
    PersistedSplit { axis: Axis::Vertical, ..persisted_h_resizable(id, theme) }
}

impl PersistedSplit {
    /// Appends a pane.
    pub fn pane(mut self, pane: SplitPane) -> Self {
        self.panes.push(pane);
        self
    }
}

impl RenderOnce for PersistedSplit {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = state_for(&self.id, cx);
        let group = match self.axis {
            Axis::Horizontal => h_resizable(self.id.clone()),
            Axis::Vertical => v_resizable(self.id.clone()),
        };
        let recorded = self.id.clone();
        let group = group
            .with_state(&state)
            .with_handle_appearance(reset_on_double_click(self.id.clone(), self.hit_padding))
            .on_resize(move |state, _, cx| {
                let sizes = state.read(cx).sizes().clone();
                record(&recorded, &sizes, cx);
            });
        // Hidden panes take no room; the visible ones fit the container measured last frame.
        let bounds: Vec<PaneBounds> = self.panes.iter().enumerate().map(|(ix, pane)| pane.bounds(saved_size(&self.id, ix, cx))).collect();
        let visible: Vec<PaneBounds> = self.panes.iter().zip(&bounds).filter(|(pane, _)| pane.visible).map(|(_, b)| *b).collect();
        let mut fitted = fit_panes(&visible, state.read(cx).container_size().as_f32()).into_iter();
        self.panes.into_iter().zip(bounds).fold(group, |group, (pane, own)| {
            let fit = if pane.visible { fitted.next().unwrap_or(own) } else { own };
            group.child(pane.into_panel(fit))
        })
    }
}
