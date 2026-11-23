//! N19: "Changed my mind…" from a post's menu: the sheet asks what the user used to think and
//! which earlier post of theirs this one revises (found by its words); either is enough.
//! Taking the mark back needs no sheet.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-mind";
const PICK_ID: &str = "data-mind-pick-";

/// One interaction with a change of mind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MindEvent {
    /// Opens the sheet for the post.
    Open(PostId),
    /// Takes the mark off the post.
    Unmark(PostId),
    /// Picks (or drops) the earlier post it revises.
    Pick(PostId),
    Save,
    Close,
}

/// The sheet's state.
#[derive(Clone)]
pub struct MindDialog {
    pub post_id: PostId,
    pub was: Entity<InputState>,
    pub find: Entity<InputState>,
    /// The earlier posts the search finds: (id, text on one line).
    pub candidates: Vec<(PostId, String)>,
    pub chosen: Option<PostId>,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: MindEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Thought(ThoughtEvent::Mind(event.clone())), window, cx)
}

fn candidate(id: &PostId, text: &str, chosen: bool, on_event: &DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let icon = if chosen { LucideIcon::CircleCheck } else { LucideIcon::Quote };
    let colour = if chosen { theme.accent_success } else { theme.text_muted };
    let (hover, bg) = (theme.hover_bg, if chosen { theme.bg_callout } else { theme.bg_elevated });
    div()
        .id(SharedString::from(format!("{PICK_ID}{id}")))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .p(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .bg(bg)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(send(on_event, MindEvent::Pick(id.clone())))
        .child(Icon::new(icon).xsmall().text_color(colour))
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(text.to_string()))
}

/// The overlay.
pub fn render_mind_sheet(dialog: &MindDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let was = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.mind.was_label"), theme)).child(Input::new(&dialog.was));
    let rows = dialog.candidates.iter().map(|(id, text)| candidate(id, text, dialog.chosen.as_ref() == Some(id), &on_event, theme));
    let empty = dialog.candidates.is_empty().then(|| hint(tr("posts.mind.no_candidates"), theme));
    let revises = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.mind.revises_label"), theme)).child(Input::new(&dialog.find)).children(rows).children(empty);
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.mind.explain"), theme)).child(was).child(revises).children(problem);
    let buttons = footer(theme)
        .child(Button::new("data-mind-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, MindEvent::Close)))
        .child(Button::new("data-mind-save").primary().small().icon(Icon::new(LucideIcon::RefreshCw)).label(tr("posts.mind.save")).on_click(send(&on_event, MindEvent::Save)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::RefreshCw,
        title: tr("posts.mind.title"),
        subtitle: Some(tr("posts.mind.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Thought(ThoughtEvent::Mind(MindEvent::Close)), window, cx)),
    };
    sheet_frame(parts, theme)
}
