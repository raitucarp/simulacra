//! Import and capture UI: the import sheet (format, metadata, segmentation, unit preview,
//! who posts), the quick-capture box, the drop overlay and the command ids other surfaces
//! (the sidebar's "New" menu, the command palette) use to reach them.

mod capture_box;
mod drop_overlay;
pub mod keys;
mod labels;
mod session;
mod sheet;

#[cfg(test)]
mod tests;

pub use capture_box::{CaptureAction, CaptureBox};
pub use drop_overlay::render_drop_overlay;
pub use labels::{family_art, family_label, file_size};
pub use session::{ImportSession, ImportStage};
pub use sheet::{ImportInputs, ImportSheet, ImportSheetActions, SheetAction, SheetHandler};
