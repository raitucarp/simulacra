//! Which explainer cards are open, so Escape can close them before it closes the forge.
//!
//! Cards open on hover (kit hover card) or from the keyboard (Enter / Space on the focused
//! "?", "pinned"). Every card keys its state by the registry's generation, so bumping it
//! restarts — closes — all of them at once.

use gpui_kit::*;
use std::collections::HashSet;

/// Open cards and the generation every card's state is keyed by.
#[derive(Default)]
struct ExplainerRegistry {
    generation: usize,
    open: HashSet<SharedString>,
}

impl Global for ExplainerRegistry {}

/// Generation to key explainer state with this frame.
pub(super) fn generation(cx: &App) -> usize {
    cx.try_global::<ExplainerRegistry>().map_or_else(usize::default, |registry| registry.generation)
}

/// Records that the card `id` opened or closed.
pub(super) fn set_open(id: &SharedString, open: bool, cx: &mut App) {
    let registry = cx.default_global::<ExplainerRegistry>();
    if open {
        registry.open.insert(id.clone());
    } else {
        registry.open.remove(id);
    }
}

/// Closes every open explainer card; returns whether any was open. The caller re-renders.
pub fn close_explainers(cx: &mut App) -> bool {
    let registry = cx.default_global::<ExplainerRegistry>();
    let any_open = !registry.open.is_empty();
    registry.open.clear();
    registry.generation = registry.generation.wrapping_add(1);
    any_open
}

/// Keyboard state of one "?" trigger: its tab stop and whether Enter / Space pinned it open.
pub(super) struct PinState {
    pub handle: FocusHandle,
    pub pinned: bool,
}

impl PinState {
    pub fn new(cx: &mut App) -> Self {
        Self { handle: cx.focus_handle().tab_stop(true), pinned: false }
    }
}
