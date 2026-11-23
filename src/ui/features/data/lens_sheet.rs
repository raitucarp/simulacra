//! N16: the lens fan sheet. The user picks up to [`MAX_LENSES`] lenses; each goes to the agent
//! (of those that may read the post) whose knowledge fits it best, one lens per agent.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::domain::post::lens::{Lens, MAX_LENSES};
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-lens-fan";
const LENS_ID: &str = "data-lens-";

/// One interaction with a lens fan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LensEvent {
    /// Opens the sheet for the post.
    Open(PostId),
    /// Picks or drops a lens.
    Toggle(Lens),
    Send,
    Close,
}

/// The sheet's state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LensDialog {
    pub post_id: PostId,
    pub chosen: Vec<Lens>,
    /// How many agents may read the post (so may be asked).
    pub room: usize,
    pub problem: Option<String>,
}

impl LensDialog {
    /// The sheet for `post`, with the default lenses picked.
    pub fn new(post_id: PostId, room: usize) -> Self {
        Self { post_id, chosen: Lens::DEFAULTS.to_vec(), room, problem: None }
    }

    /// `lens` picked or dropped (never past [`MAX_LENSES`]).
    pub fn toggle(&mut self, lens: Lens) {
        match self.chosen.iter().position(|chosen| *chosen == lens) {
            Some(at) => _ = self.chosen.remove(at),
            None if self.chosen.len() < MAX_LENSES => self.chosen.push(lens),
            None => self.problem = Some(t!("posts.lens.too_many", count = MAX_LENSES).to_string()),
        }
    }
}

/// The translated name of `lens`.
pub fn lens_name(lens: Lens) -> String {
    tr(&format!("posts.lens.name_{}", lens.key()))
}

fn send(on_event: &DataAction, event: LensEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Thought(ThoughtEvent::Lens(event.clone())), window, cx)
}

fn lens_button(lens: Lens, chosen: bool, on_event: &DataAction) -> Button {
    let icon = if chosen { LucideIcon::Check } else { LucideIcon::Eye };
    Button::new(SharedString::from(format!("{LENS_ID}{}", lens.key()))).ghost().small().selected(chosen).icon(Icon::new(icon)).label(lens_name(lens)).on_click(send(on_event, LensEvent::Toggle(lens)))
}

/// The overlay.
pub fn render_lens_sheet(dialog: &LensDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let lenses = div().flex().flex_wrap().gap(theme.inline_gap).children(Lens::ALL.iter().map(|lens| lens_button(*lens, dialog.chosen.contains(lens), &on_event)));
    let picker = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.lens.pick"), theme)).child(lenses);
    let room = hint(t!("posts.lens.room", count = dialog.room).to_string(), theme);
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(t!("posts.lens.explain", count = MAX_LENSES).to_string(), theme)).child(picker).child(room).children(problem);
    let buttons = footer(theme)
        .child(Button::new("data-lens-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, LensEvent::Close)))
        .child(Button::new("data-lens-send").primary().small().icon(Icon::new(LucideIcon::Send)).label(tr("posts.lens.send")).on_click(send(&on_event, LensEvent::Send)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::Layers,
        title: tr("posts.lens.title"),
        subtitle: Some(tr("posts.lens.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Thought(ThoughtEvent::Lens(LensEvent::Close)), window, cx)),
    };
    sheet_frame(parts, theme)
}
