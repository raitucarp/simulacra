//! The scheduled queue sheet (Y4): every waiting post, soonest first, with Edit, Publish now
//! and Cancel (confirmed first, since a cancelled post is deleted).

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, hint, sheet_frame, SheetParts};
use super::text::{format_time, tr};
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::*;
use jiff::Timestamp;
use std::rc::Rc;

const SHEET_ID: &str = "data-scheduled";
const CLOSE_ID: &str = "data-scheduled-close";
const EDIT_ID: &str = "data-scheduled-edit";
const PUBLISH_ID: &str = "data-scheduled-publish";
const CANCEL_ID: &str = "data-scheduled-cancel";
/// Lines of a waiting post's text shown before it is cut.
const EXCERPT_LINES: usize = 3;

/// One waiting post as the sheet lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledRow {
    pub id: PostId,
    pub text: String,
    pub publish_at: Option<Timestamp>,
}

fn row_button(id: &'static str, post: &PostId, icon: LucideIcon, label: String) -> Button {
    Button::new(SharedString::from(format!("{id}-{post}"))).ghost().xsmall().icon(Icon::new(icon)).label(label)
}

/// Asks before cancelling `post`.
fn confirm_cancel(post: PostId, on_event: DataAction, window: &mut Window, cx: &mut App) {
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (post, on_event) = (post.clone(), on_event.clone());
        alert
            .confirm()
            .title(tr("data.scheduled.cancel_title"))
            .description(tr("data.scheduled.cancel_desc"))
            .ok_text(tr("data.scheduled.cancel"))
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(tr("data.scheduled.keep"))
            .on_ok(move |_, window, cx| {
                on_event(DataEvent::CancelScheduled(post.clone()), window, cx);
                true
            })
    });
}

fn row(item: &ScheduledRow, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let when = item.publish_at.map(format_time).unwrap_or_default();
    let (edit, publish, cancel) = (on_event.clone(), on_event.clone(), on_event.clone());
    let (edit_id, publish_id, cancel_id) = (item.id.clone(), item.id.clone(), item.id.clone());
    let actions = div()
        .flex()
        .flex_wrap()
        .gap(theme.spacing_xs)
        .child(row_button(EDIT_ID, &item.id, LucideIcon::Pencil, tr("data.menu.edit")).on_click(move |_, w, cx| edit(DataEvent::EditScheduled(edit_id.clone()), w, cx)))
        .child(row_button(PUBLISH_ID, &item.id, LucideIcon::Send, tr("data.scheduled.publish_now")).on_click(move |_, w, cx| publish(DataEvent::PublishScheduledNow(publish_id.clone()), w, cx)))
        .child(row_button(CANCEL_ID, &item.id, LucideIcon::Trash, tr("data.scheduled.cancel")).on_click(move |_, w, cx| confirm_cancel(cancel_id.clone(), cancel.clone(), w, cx)));
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(when))
        .child(div().text_size(theme.font_size_xs).line_clamp(EXCERPT_LINES).text_ellipsis().text_color(theme.text_primary).child(item.text.clone()))
        .child(actions)
}

/// The overlay.
pub fn render_scheduled_sheet(rows: &[ScheduledRow], on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let body = match rows.is_empty() {
        true => div().child(hint(tr("data.scheduled.empty"), theme)),
        false => div().children(rows.iter().map(|item| row(item, &on_event, theme))),
    };
    let (close, closing) = (on_event.clone(), on_event);
    let buttons = footer(theme).child(Button::new(CLOSE_ID).ghost().small().label(tr("data.close")).on_click(move |_, w, cx| close(DataEvent::CloseScheduled, w, cx)));
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::CalendarClock,
        title: tr("data.scheduled.title"),
        subtitle: Some(crate::i18n::t_count("data.scheduled.count", rows.len())),
        body,
        footer: buttons,
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseScheduled, w, cx)),
    };
    sheet_frame(parts, theme)
}
