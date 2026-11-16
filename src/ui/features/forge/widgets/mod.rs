//! Reusable Agent Forge option widgets. Each is a `RenderOnce` element that keeps whatever
//! entity state it needs (input / slider / focus) in keyed element state, so a step only
//! passes the current draft value and a callback:
//!
//! - [`ForgeTextInput`]: single- or multi-line text bound to a value (`on_change`, `on_blur`).
//! - [`IllustratedOptionGrid`]: illustrated cards for any `TraitOption`, with arrow-key focus;
//!   [`trait_option_grid`] does the same for a runtime [`TraitField`](crate::state::forge::TraitField).
//! - [`ScoreSlider`]: kit slider for a `ScoreField`, with level name and budget clamp.
//! - [`NumberSlider`]: a labeled slider over any whole-number range (age, photo zoom).
//! - [`HourRangePicker`]: a 24-hour strip plus start / end steppers.
//! - [`ToggleCard`]: a switch card for a `ToggleField`.
//! - [`ExplainerButton`]: a "?" hover card saying what a setting is and how it changes posts.

mod explainer;
mod hour_range;
mod number_slider;
mod option_card;
mod option_grid;
mod score_slider;
mod slider_slot;
mod text_input;
mod toggle_card;
mod trait_grid;

pub use explainer::{Explainer, ExplainerButton};
pub use hour_range::HourRangePicker;
pub use number_slider::{NumberSlider, ValueFormat};
pub use option_card::{option_card, OptionCardSpec};
pub use option_grid::{IllustratedOptionGrid, GRID_COLUMNS};
pub use score_slider::ScoreSlider;
pub use text_input::ForgeTextInput;
pub use toggle_card::ToggleCard;
pub use trait_grid::{trait_option_grid, TraitPick};

use gpui_kit::{App, Window};
use std::rc::Rc;

/// A shared callback taking one value.
pub type ValueCallback<T> = Rc<dyn Fn(T, &mut Window, &mut App)>;
/// A shared callback without arguments.
pub type ActionCallback = Rc<dyn Fn(&mut Window, &mut App)>;
