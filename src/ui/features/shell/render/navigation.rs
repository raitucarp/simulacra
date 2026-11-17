//! The keyboard shortcut sheet overlay (the command palette is its own entity).

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::keymap::ShortcutSheet;
use std::rc::Rc;

impl AppView {
    /// The shortcut sheet, while open.
    pub(super) fn shortcut_sheet(&self, bind: &Binder) -> Option<ShortcutSheet> {
        self.palette.is_sheet_open.then(|| {
            ShortcutSheet::new(&self.theme, self.palette.sheet_focus.clone(), Rc::new(bind.act(Self::toggle_shortcut_sheet)))
        })
    }
}
