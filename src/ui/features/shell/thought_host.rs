//! N16–N20 in the shell's data host: the open sheets of the lens fan (N16).

use crate::ui::features::data::LensDialog;

/// The open N16–N20 sheets and what is under way.
#[derive(Default)]
pub struct ThoughtSheets {
    /// N16: picking the lenses of a fan.
    pub lens: Option<LensDialog>,
}

impl ThoughtSheets {
    /// Whether one of them is open.
    pub fn any_open(&self) -> bool {
        self.lens.is_some()
    }
}
