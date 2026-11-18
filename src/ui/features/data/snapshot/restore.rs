//! The "Restore snapshot" sheet: choose a file and its passphrase, verify, then confirm.
//! Nothing is replaced before the confirmation, and the current vault is kept beside it.

use super::super::events::{DataAction, DataEvent};
use super::super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::super::text::tr;
use super::create::passphrase_field;
use super::{RestoreDialog, RestoreStep};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-restore";
const PICK_ID: &str = "data-restore-pick";
const VERIFY_ID: &str = "data-restore-verify";
const CONFIRM_ID: &str = "data-restore-confirm";
const CANCEL_ID: &str = "data-restore-cancel";

fn send(on_event: &DataAction, event: DataEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(event.clone(), window, cx)
}

fn choose_body(dialog: &RestoreDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let file = dialog.file.as_ref().and_then(|path| path.file_name()).map_or_else(|| tr("data.restore.no_file"), |name| name.to_string_lossy().to_string());
    let picker = div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(Button::new(PICK_ID).outline().small().icon(Icon::new(LucideIcon::Folder)).label(tr("data.restore.pick")).on_click(send(on_event, DataEvent::PickSnapshotFile)))
        .child(div().flex_1().min_w_0().truncate().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_secondary).child(file));
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(heading(tr("data.restore.file_title"), theme))
        .child(picker)
        .child(passphrase_field(tr("data.restore.passphrase"), &dialog.passphrase, theme))
        .child(hint(tr("data.restore.verify_hint"), theme))
}

fn verified_body(created_at: &str, counts: (usize, usize, usize), theme: &ThemeTokens) -> Div {
    let (posts, profiles, documents) = counts;
    let summary = t!("data.restore.summary", time = created_at, posts = posts, profiles = profiles, documents = documents).to_string();
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(notice(LucideIcon::ShieldCheck, summary, theme.accent_success, theme))
        .child(notice(LucideIcon::CircleAlert, tr("data.restore.replace_warning"), theme.warning, theme))
}

fn buttons(dialog: &RestoreDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let cancel = Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(on_event, DataEvent::CloseRestore));
    let action = match dialog.step {
        RestoreStep::Choose => {
            let label = if dialog.busy { tr("data.restore.verifying") } else { tr("data.restore.verify") };
            Button::new(VERIFY_ID).primary().small().icon(Icon::new(LucideIcon::ShieldCheck)).label(label).disabled(dialog.file.is_none() || dialog.busy).on_click(send(on_event, DataEvent::VerifySnapshot))
        }
        RestoreStep::Verified { .. } => Button::new(CONFIRM_ID).danger().small().icon(Icon::new(LucideIcon::ArchiveRestore)).label(tr("data.restore.confirm")).disabled(dialog.busy).on_click(send(on_event, DataEvent::ConfirmRestore)),
    };
    footer(theme).child(cancel).child(action)
}

/// The overlay.
pub fn render_restore_sheet(dialog: &RestoreDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let step = match &dialog.step {
        RestoreStep::Choose => choose_body(dialog, &on_event, theme),
        RestoreStep::Verified { created_at, posts, profiles, documents } => verified_body(created_at, (*posts, *profiles, *documents), theme),
    };
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let body = div().child(hint(tr("data.restore.explain"), theme)).child(step).children(problem);
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::ArchiveRestore,
        title: tr("data.restore.title"),
        subtitle: Some(tr("data.restore.subtitle")),
        body,
        footer: buttons(dialog, &on_event, theme),
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseRestore, w, cx)),
    };
    sheet_frame(parts, theme)
}
