//! [`PersistedSplit`]: a resizable panel group restored from, and saved to, the layout store.

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

    fn into_panel(self, saved: Option<f32>) -> ResizablePanel {
        let panel = resizable_panel().visible(self.visible);
        let panel = match self.size {
            PaneSize::Sized { default, range } => {
                let size = restored_size(saved, default.as_f32(), range.start.as_f32(), range.end.as_f32());
                panel.size(px(size)).size_range(range).fixed()
            }
            PaneSize::Flex { min } => panel.size_range(min..Pixels::MAX),
        };
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
        let id = self.id;
        self.panes.into_iter().enumerate().fold(group, |group, (ix, pane)| group.child(pane.into_panel(saved_size(&id, ix, cx))))
    }
}
