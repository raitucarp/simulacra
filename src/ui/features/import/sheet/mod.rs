//! The import sheet: a modal over the window showing the detected format, editable metadata,
//! the segmentation switch, a virtualized preview of the units with checkboxes, who posts
//! them, and "Create posts" / "Just add to vault". While the file is read it shows progress
//! and a cancel button.

mod author;
mod details;
mod footer;
mod header;
mod preview;
mod units;

use super::session::{ImportSession, ImportStage};
use crate::domain::preferences::PostIdentity;
use crate::import::Granularity;
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use std::rc::Rc;

const BACKDROP_ID: &str = "import-sheet-backdrop";
const DIALOG_ID: &str = "import-sheet-dialog";
const BODY_ID: &str = "import-sheet-body";

/// A callback without arguments.
pub type SheetAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// A callback with one argument.
pub type SheetHandler<A> = Rc<dyn Fn(A, &mut Window, &mut App) + 'static>;

/// The metadata and pseudonym text inputs.
#[derive(Clone)]
pub struct ImportInputs {
    pub title: Entity<InputState>,
    pub creators: Entity<InputState>,
    pub language: Entity<InputState>,
    pub date: Entity<InputState>,
    pub pseudonym: Entity<InputState>,
}

/// Everything the sheet can ask its owner to do.
#[derive(Clone)]
pub struct ImportSheetActions {
    pub on_close: SheetAction,
    pub on_granularity: SheetHandler<Granularity>,
    pub on_toggle: SheetHandler<usize>,
    pub on_select_all: SheetHandler<bool>,
    pub on_select_chapter: SheetHandler<usize>,
    pub on_identity: SheetHandler<PostIdentity>,
    /// `true`: create posts; `false`: only add the file to the vault.
    pub on_apply: SheetHandler<bool>,
}

/// The sheet for one session.
#[derive(IntoElement)]
pub struct ImportSheet {
    pub theme: ThemeTokens,
    pub session: ImportSession,
    pub inputs: ImportInputs,
    pub actions: ImportSheetActions,
    /// The pseudonym input holds a name.
    pub has_pseudonym: bool,
}

impl RenderOnce for ImportSheet {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let close = self.actions.on_close.clone();
        div()
            .id(BACKDROP_ID)
            .absolute()
            .inset_0()
            .occlude()
            .bg(self.theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| close(window, cx))
            .child(self.dialog())
    }
}

impl ImportSheet {
    fn dialog(&self) -> Stateful<Div> {
        let theme = &self.theme;
        let body = div().id(BODY_ID).flex_1().min_h_0().overflow_y_scroll().flex().flex_col();
        let body = match &self.session.stage {
            ImportStage::Ready => body
                .p(theme.panel_padding)
                .gap(theme.section_gap)
                .child(self.details())
                .child(self.author())
                .child(self.units()),
            ImportStage::Reading => body.child(self.progress()),
            ImportStage::Failed(reason) => body.child(self.failure(reason)),
        };
        div()
            .id(DIALOG_ID)
            .w(theme.import_sheet_width)
            .max_w_full()
            .max_h(theme.import_sheet_max_height)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_elevated)
            .elevation_shadow(theme)
            .flex()
            .flex_col()
            .overflow_hidden()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(self.header())
            .child(body)
            .child(self.footer())
    }
}
