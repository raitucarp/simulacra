//! UI architecture organized hierarchically into Atoms, Molecules, Organisms, and Views.

pub mod assets;
pub mod atoms;
pub mod components;
pub mod features;
pub mod icons;
pub mod illustration;
pub mod keymap;
pub mod molecules;
pub mod organisms;
pub mod render_stats;
pub mod responsive;
pub mod views;

pub use icons::{Lucide, LucideIcon};
pub use views::AppView;
