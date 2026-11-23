//! N13: a claim of the user's that is due for a check asks "Still true?" under its author,
//! with Confirm, Revise (edit it: a new version) and Retract (N14). Never on a locked post, nor on anyone else's.

use super::controls::PostCardHandlers;
use crate::domain::post::correction::CorrectionKind;
use crate::domain::post::still_true::is_due;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::data::{CheckEvent, CorrectionEvent, DataAction, DataEvent};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// How the strip shows when the post was written.
const DATE_FORMAT: &str = "%Y-%m-%d";

/// Whether the strip asks about `post` now (`own`: the user wrote it).
pub(crate) fn asks_still_true(post: &Post, own: bool, now: jiff::Timestamp) -> bool {
    own && !post.is_locked() && is_due(post, now)
}

fn button(post: &Post, part: &str, icon: LucideIcon, label: &str, event: DataEvent, on_event: &DataAction) -> Button {
    let on_event = on_event.clone();
    Button::new(SharedString::from(format!("still-true-{part}-{}", post.id))).ghost().xsmall().icon(Icon::new(icon)).label(t!(label).to_string()).on_click(move |_, window, cx| on_event(event.clone(), window, cx))
}

/// The strip; `None` when the post is not due (or not the user's, or locked).
pub(super) fn render_still_true_strip(post: &Post, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let data = handlers.data.as_ref().filter(|data| asks_still_true(post, data.own, jiff::Timestamp::now()))?;
    let (id, on_event) = (post.id.clone(), &data.on_event);
    let written = crate::time::local(post.created_at).strftime(DATE_FORMAT).to_string();
    let question = div().flex_1().min_w_0().text_color(theme.text_primary).child(t!("posts.still_true.ask", date = written).to_string());
    let row = div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(LucideIcon::RotateCcwClock).small().text_color(theme.accent_brand))
        .child(question)
        .child(button(post, "confirm", LucideIcon::CircleCheck, "posts.still_true.confirm", DataEvent::Check(CheckEvent::Confirm(id.clone())), on_event))
        .child(button(post, "revise", LucideIcon::Pencil, "posts.still_true.revise", DataEvent::EditPost(id.clone()), on_event))
        .child(button(post, "retract", LucideIcon::CircleX, "posts.still_true.retract", DataEvent::Correction(CorrectionEvent::Open(id.clone(), CorrectionKind::Retracted)), on_event));
    let strip = div().w_full().flex().flex_col().gap(theme.spacing_3xs).px(theme.spacing_sm).py(theme.spacing_xs).rounded(theme.radius_md).bg(theme.bg_callout).text_size(theme.font_size_xs).child(row);
    Some(strip)
}
