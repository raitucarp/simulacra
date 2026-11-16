//! Agent Forge: the Stardew / Skyrim-style character creator for agents and personas.
//!
//! - `logic`: pure, unit-tested decisions (rail marks, save bar, autosave, guide pose, focus).
//! - `widgets`: reusable illustrated option widgets the step pages are built from.

pub mod logic;
mod screens;
pub mod view;
pub mod text;
pub mod widgets;

#[cfg(test)]
mod tests;
