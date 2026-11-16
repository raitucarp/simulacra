//! [`GridCell`]: one selectable cell of a [`CardGrid`](super::CardGrid).

use super::option_card::{option_card, OptionCardSpec};
use super::ActionCallback;
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Builds a cell from whether the keyboard cursor is on it and its click handler.
type CellBuilder = Box<dyn FnOnce(bool, ActionCallback) -> Stateful<Div>>;

/// One cell of a [`CardGrid`].
pub struct GridCell {
    pub(super) build: CellBuilder,
    pub(super) pick: ActionCallback,
    pub(super) hover: Option<ActionCallback>,
    pub(super) selected: bool,
    pub(super) disabled: bool,
}

impl GridCell {
    /// A custom cell; `build` gets `(focused, on_click)` and must wire the click itself.
    pub fn new(selected: bool, build: impl FnOnce(bool, ActionCallback) -> Stateful<Div> + 'static, pick: ActionCallback) -> Self {
        Self { build: Box::new(build), pick, hover: None, selected, disabled: false }
    }

    /// An [`option_card`] cell (the grid sets `spec.focused`).
    pub fn card(spec: OptionCardSpec, theme: &ThemeTokens, pick: ActionCallback) -> Self {
        let (selected, disabled, theme) = (spec.selected, spec.disabled, theme.clone());
        let build = move |focused: bool, click: ActionCallback| option_card(OptionCardSpec { focused, ..spec }, &theme, move |w, cx| click(w, cx));
        Self { disabled, ..Self::new(selected, build, pick) }
    }

    /// Runs `hover` when the pointer enters the cell or the keyboard cursor lands on it.
    pub fn on_hover(mut self, hover: ActionCallback) -> Self {
        self.hover = Some(hover);
        self
    }
}
