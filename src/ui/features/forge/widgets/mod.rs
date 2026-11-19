//! Reusable Agent Forge option widgets. Each is a `RenderOnce` element that keeps whatever
//! entity state it needs (input / slider / focus) in keyed element state, so a step only
//! passes the current draft value and a callback:
//!
//! - [`ForgeTextInput`]: single- or multi-line text bound to a value (`on_change`, `on_blur`).
//! - [`CardGrid`]: any selectable cells as one tab stop with an arrow-key cursor;
//!   [`IllustratedOptionGrid`] fills it with the cards of a `TraitOption`, and
//!   [`trait_option_grid`] does the same for a runtime [`TraitField`](crate::state::forge::TraitField).
//! - [`MultiSelectGrid`]: toggleable illustrated cards for a multi-select (`OptionSet`);
//!   [`trait_set_grid`] for a runtime [`TraitSet`](crate::state::forge::TraitSet).
//! - [`KeyboardStop`]: tab stop + theme focus ring + Enter / Space for custom controls.
//! - [`ScoreSlider`]: kit slider for a `ScoreField`, with level name and budget clamp.
//! - [`NumberSlider`]: a labeled slider over any whole-number range (age, photo zoom).
//! - [`HourRangePicker`]: a 24-hour strip plus start / end steppers.
//! - [`ToggleCard`]: a switch card for a `ToggleField`.
//! - [`ExplainerButton`]: a "?" hover card saying what a setting is and how it changes posts.
//! - [`KeyboardHint`]: a keyboard icon whose hover card lists the forge's shortcuts.
//! - [`suggest`]: example chips, the typical band under sliders, "Suggested" badges and the
//!   "Why?" card (sb D1).

mod card_grid;
mod explainer;
mod explainer_card;
mod explainer_state;
mod focus_ring;
mod grid_cell;
mod hour_range;
mod keyboard_hint;
mod multi_grid;
mod number_slider;
mod option_card;
mod option_grid;
mod score_slider;
mod slider_slot;
pub mod suggest;
mod text_input;
mod text_input_slot;
mod toggle_card;
mod trait_grid;

pub use card_grid::CardGrid;
pub use grid_cell::GridCell;
pub use explainer::{close_explainers, CardBody, ExplainerButton};
pub use explainer_card::Explainer;
pub use focus_ring::{focus_ring_shadow, is_activation, KeyboardStop};
pub use hour_range::{hour_keys, HourRangePicker};
pub use keyboard_hint::KeyboardHint;
pub use multi_grid::{trait_set_grid, MultiSelectGrid};
pub use number_slider::{NumberSlider, ValueFormat};
pub use option_card::{option_card, OptionCardSpec};
pub use option_grid::{IllustratedOptionGrid, GRID_COLUMNS};
pub use score_slider::ScoreSlider;
pub use text_input::ForgeTextInput;
pub use toggle_card::ToggleCard;
pub use trait_grid::{trait_option_grid, TraitGridLook, TraitPick, SMALL_COLUMNS};

use gpui_kit::{App, Window};
use std::rc::Rc;

/// A shared callback taking one value.
pub type ValueCallback<T> = Rc<dyn Fn(T, &mut Window, &mut App)>;
/// A shared callback without arguments.
pub type ActionCallback = Rc<dyn Fn(&mut Window, &mut App)>;
