//! The "Encrypted snapshot" sheet.

use super::super::events::{DataAction, DataEvent};
use super::super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::super::text::tr;
use super::meter::strength_meter;
use super::SnapshotDialog;
use crate::domain::data::snapshot::MIN_PASSPHRASE_CHARS;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-snapshot";
const CREATE_ID: &str = "data-snapshot-create";
const CANCEL_ID: &str = "data-snapshot-cancel";

/// A labelled masked passphrase field.
pub(super) fn passphrase_field(label: String, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
        .child(Input::new(input).mask_toggle())
}

fn status_line(dialog: &SnapshotDialog, theme: &ThemeTokens) -> Option<Div> {
    let text = if !dialog.long_enough {
        t!("data.snapshot.too_short", count = MIN_PASSPHRASE_CHARS).to_string()
    } else if !dialog.matches {
        tr("data.snapshot.mismatch")
    } else {
        return None;
    };
    Some(hint(text, theme))
}

/// The overlay.
pub fn render_snapshot_sheet(dialog: &SnapshotDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let fields = div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(heading(tr("data.snapshot.passphrase_title"), theme))
        .child(passphrase_field(tr("data.snapshot.passphrase"), &dialog.passphrase, theme))
        .child(strength_meter(dialog.strength, theme))
        .child(hint(tr(&format!("data.snapshot.tip.{}", dialog.strength.key())), theme))
        .child(passphrase_field(tr("data.snapshot.repeat"), &dialog.repeat, theme))
        .children(status_line(dialog, theme));
    let body = div()
        .child(hint(tr("data.snapshot.explain"), theme))
        .child(fields)
        .child(notice(LucideIcon::KeyRound, tr("data.snapshot.unrecoverable"), theme.warning, theme))
        .children(problem);
    let (create, cancel, closing) = (on_event.clone(), on_event.clone(), on_event);
    let label = if dialog.busy { tr("data.snapshot.sealing") } else { tr("data.snapshot.create") };
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(move |_, w, cx| cancel(DataEvent::CloseSnapshot, w, cx)))
        .child(Button::new(CREATE_ID).primary().small().icon(Icon::new(LucideIcon::LockKeyhole)).label(label).disabled(!dialog.ready()).on_click(move |_, w, cx| create(DataEvent::CreateSnapshot, w, cx)));
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::LockKeyhole,
        title: tr("data.snapshot.title"),
        subtitle: Some(tr("data.snapshot.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseSnapshot, w, cx)),
    };
    sheet_frame(parts, theme)
}
