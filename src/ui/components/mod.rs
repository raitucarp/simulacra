//! Reusable, theme-token-driven UI components shared across features.
//!
//! Every component is a plain function (optionally taking a small spec struct) that returns
//! a `Div` / `Stateful<Div>`, so callers can keep chaining layout modifiers. `image_picker`
//! wraps the native image file dialog used by several features; `document_picker` does the same
//! for plain-text vault documents.

pub mod art_state;
pub mod badge;
pub mod chip;
pub mod document_picker;
pub mod hover_row;
pub mod image_picker;
pub mod measured_width;
pub mod persisted_split;
pub mod segmented;
pub mod status_dot;
pub mod stepper;

pub use art_state::{art_state, ArtState};
pub use badge::{meta_badge, mono_tag, BadgeColors};
pub use chip::{chip_row, choice_chip};
pub use document_picker::{
    pick_document_path, read_document_file, DocumentImportError, MAX_DOCUMENT_KB,
};
pub use hover_row::selectable_row;
pub use image_picker::{pick_image_path, pick_image_paths};
pub use measured_width::measured_width;
pub use persisted_split::{persisted_h_resizable, persisted_v_resizable, PersistedSplit, SplitPane};
pub use segmented::{segment_pill, segmented_track, Segment};
pub use status_dot::{status_dot, status_glyph};
pub use stepper::mini_step_button;
