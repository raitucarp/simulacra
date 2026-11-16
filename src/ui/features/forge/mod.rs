//! Agent Forge: the Stardew / Skyrim-style character creator for agents and personas.
//!
//! - `view`: the [`ForgeView`](view::ForgeView) entity (draft, navigation, undo, autosave,
//!   describe-in-words) and the [`ForgeEvent`](view::ForgeEvent)s its host carries out.
//! - `screens`: rendering of the resume prompt, class picker, step creator and celebration.
//! - `widgets`: reusable illustrated option widgets the step pages are built from.
//! - `logic`: pure, unit-tested decisions (rail marks, save bar, autosave, guide pose, focus).
//! - `text`: runtime i18n helpers (optional keys, `%{arg}` lists, trait labels).

pub mod logic;
mod screens;
pub mod text;
pub mod view;
pub mod widgets;

#[cfg(test)]
mod tests;
