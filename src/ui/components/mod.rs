//! Reusable, theme-token-driven UI components shared across features.
//!
//! Every component is a plain function (optionally taking a small spec struct) that returns
//! a `Div` / `Stateful<Div>`, so callers can keep chaining layout modifiers.

pub mod badge;
pub mod chip;
pub mod enum_chips;
pub mod hover_row;
pub mod labeled_field;
pub mod pill_tab;
pub mod save_footer;
pub mod section_card;
pub mod segmented;
pub mod status_dot;
pub mod stepper;
pub mod toggle_row;
pub mod trait_level;
pub mod trait_meter;
pub mod vertical_tab;

pub use badge::{meta_badge, mono_tag, BadgeColors};
pub use chip::{chip_row, choice_chip};
pub use enum_chips::EnumChips;
pub use hover_row::selectable_row;
pub use labeled_field::{field_label, labeled_group, labeled_input, labeled_textarea};
pub use pill_tab::{pill_tab, PillTab};
pub use save_footer::save_footer;
pub use section_card::{card_stack, fill, section_card, split_row};
pub use segmented::{segment_pill, segmented_track, Segment};
pub use status_dot::{status_dot, status_glyph};
pub use stepper::{mini_step_button, u8_stepper, StepperSpec};
pub use toggle_row::{toggle_row, ToggleSpec};
pub use trait_meter::{trait_meter, trait_semantic_label, TraitMeterSpec};
pub use vertical_tab::{vertical_tab, VerticalTab};
