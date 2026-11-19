//! Import and capture overlays (sheet, quick capture, drop hint) and the window-wide drop,
//! paste and shortcut listeners on the root element.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::import::keys::PasteAnywhere;
use crate::ui::features::import::{render_drop_overlay, CaptureBox, ImportSheet, ImportSheetActions};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// The import sheet, while a file is open.
    pub(super) fn import_sheet(&self, bind: &Binder, cx: &App) -> Option<ImportSheet> {
        let session = self.import.session.clone()?;
        let actions = ImportSheetActions {
            on_close: Rc::new(bind.act(Self::close_import)),
            on_granularity: Rc::new(bind.with(Self::select_import_granularity)),
            on_toggle: Rc::new(bind.with(Self::toggle_import_unit)),
            on_select_all: Rc::new(bind.with(Self::select_all_import_units)),
            on_select_chapter: Rc::new(bind.with(Self::select_import_chapter)),
            on_identity: Rc::new(bind.with(Self::select_import_identity)),
            on_apply: Rc::new(bind.with(Self::apply_import_sheet)),
        };
        let has_pseudonym = self.import.has_pseudonym(cx);
        Some(ImportSheet { theme: self.theme.clone(), session, inputs: self.import.inputs.clone(), actions, has_pseudonym })
    }

    /// The quick-capture box, while open.
    pub(super) fn capture_box(&self, bind: &Binder) -> Option<CaptureBox> {
        self.import.capture_open.then(|| CaptureBox {
            theme: self.theme.clone(),
            input: self.import.capture_input.clone(),
            inbox_count: self.state.inbox_count(),
            on_save: Rc::new(bind.act(Self::save_capture)),
            on_close: Rc::new(bind.act(Self::close_quick_capture)),
            on_open_inbox: Rc::new(bind.act(Self::open_inbox)),
        })
    }

    /// The drop hint while files are dragged over the window (the open composer takes images
    /// itself, so the import hint stays hidden then).
    pub(super) fn drop_hint(&self, cx: &App) -> Option<Div> {
        (self.import.dragging && !self.composer_modal.read(cx).is_open()).then(|| render_drop_overlay(&self.theme))
    }

    /// Drop, drag, paste and shortcut listeners for the root element.
    pub(super) fn with_import_listeners(&self, root: Div, cx: &mut Context<Self>) -> Div {
        root.on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| this.drop_files(paths.paths().to_vec(), window, cx)))
            .on_drag_move(cx.listener(|this, _: &DragMoveEvent<ExternalPaths>, _, cx| {
                if !this.import.dragging {
                    this.import.dragging = true;
                    cx.notify();
                }
            }))
            .on_file_drop_exit(cx.listener(|this, _: &FileDropEvent, _, cx| {
                this.import.dragging = false;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &PasteAnywhere, window, cx| this.paste_anywhere(window, cx)))
    }
}
