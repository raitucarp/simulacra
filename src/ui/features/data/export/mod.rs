//! The export dialog: pick the scope (everything, a collection, a date range, one persona),
//! choose whether originals and the private journal go along, and see exactly what will be
//! written before choosing where.

mod sections;

use super::events::{DataAction, DataEvent, ScopeKind};
use super::sheet::{footer, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::data::export::Counts;
use crate::domain::document::CollectionId;
use crate::domain::ProfileId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::InputState;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-export";
const RUN_ID: &str = "data-export-run";
const CANCEL_ID: &str = "data-export-cancel";

/// The dialog's state.
#[derive(Clone)]
pub struct ExportDialog {
    pub scope: ScopeKind,
    pub collection: Option<CollectionId>,
    pub persona: Option<ProfileId>,
    pub include_originals: bool,
    pub include_journal: bool,
    /// The active network's collections and the user's personas, as offered.
    pub collections: Vec<(CollectionId, String)>,
    pub personas: Vec<(ProfileId, String)>,
    pub from: Entity<InputState>,
    pub to: Entity<InputState>,
    /// What the current choice would write (`None` while the choice is incomplete).
    pub counts: Option<Counts>,
    /// Why the choice is incomplete.
    pub problem: Option<String>,
    pub busy: bool,
}

impl ExportDialog {
    /// Everything, nothing optional ticked.
    pub fn new(collections: Vec<(CollectionId, String)>, personas: Vec<(ProfileId, String)>, from: Entity<InputState>, to: Entity<InputState>) -> Self {
        Self { scope: ScopeKind::Everything, collection: None, persona: None, include_originals: false, include_journal: false, collections, personas, from, to, counts: None, problem: None, busy: false }
    }
}

/// The overlay.
pub fn render_export_sheet(dialog: &ExportDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let body = div()
        .child(sections::scope_section(dialog, &on_event, theme))
        .child(sections::include_section(dialog, &on_event, theme))
        .child(sections::preview_section(dialog, theme));
    let (run, cancel, closing) = (on_event.clone(), on_event.clone(), on_event);
    let label = if dialog.busy { tr("data.export.writing") } else { tr("data.export.run") };
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(move |_, w, cx| cancel(DataEvent::CloseExport, w, cx)))
        .child(Button::new(RUN_ID).primary().small().icon(Icon::new(LucideIcon::Download)).label(label).disabled(dialog.counts.is_none() || dialog.busy).on_click(move |_, w, cx| run(DataEvent::RunExport, w, cx)));
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::Package,
        title: tr("data.export.title"),
        subtitle: Some(tr("data.export.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseExport, w, cx)),
    };
    sheet_frame(parts, theme)
}
