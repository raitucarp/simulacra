//! N13: a claim of the user's that is due for a check asks "Still true?" under its author,
//! with Confirm, Revise (edit it: a new version), Retract (N14) and Ask AI, and the AI's
//! answer once there is one. Never on a locked post, nor on anyone else's.

use super::controls::PostCardHandlers;
use crate::domain::post::correction::CorrectionKind;
use crate::domain::post::still_true::is_due;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::data::{CheckEvent, CorrectionEvent, DataAction, DataEvent};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;


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
    let written = crate::ui::format::date(crate::time::local_date(post.created_at));
    let question = div().flex_1().min_w_0().text_color(theme.text_primary).child(t!("posts.still_true.ask", date = written).to_string());
    let checking = data.ideas.rechecking.as_ref() == Some(&id);
    // The question has the whole width; the answers sit on their own row under it, so a narrow
    // column never squeezes the question to a word per line.
    let ask = div().flex().items_center().gap(theme.inline_gap).child(Icon::new(LucideIcon::RotateCcwClock).small().text_color(theme.accent_brand_text)).child(question);
    let row = div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.inline_gap)
        .child(button(post, "confirm", LucideIcon::CircleCheck, "posts.still_true.confirm", DataEvent::Check(CheckEvent::Confirm(id.clone())), on_event))
        .child(button(post, "revise", LucideIcon::Pencil, "posts.still_true.revise", DataEvent::EditPost(id.clone()), on_event))
        .child(button(post, "retract", LucideIcon::CircleX, "posts.still_true.retract", DataEvent::Correction(CorrectionEvent::Open(id.clone(), CorrectionKind::Retracted)), on_event))
        .when(!checking, |row| row.child(button(post, "ask", LucideIcon::Sparkles, "posts.still_true.ask_ai", DataEvent::Check(CheckEvent::Recheck(id.clone())), on_event)));
    let verdict = data.ideas.rechecks.iter().find(|(post, _)| post == &id).map(|(_, text)| div().text_color(theme.text_muted).child(text.clone()));
    let note = checking.then(|| div().text_color(theme.text_muted).child(t!("posts.still_true.checking").to_string()));
    let strip = div().w_full().flex().flex_col().gap(theme.spacing_3xs).px(theme.spacing_sm).py(theme.spacing_xs).rounded(theme.radius_md).bg(theme.bg_callout).text_size(theme.font_size_xs).child(ask).child(row).children(verdict).children(note);
    Some(strip)
}
