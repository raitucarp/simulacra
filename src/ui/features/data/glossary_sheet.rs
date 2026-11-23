//! N17: defining a term from a post's menu: the sheet asks for the term and what it means,
//! and can ask the AI for a draft of the definition; taking a definition back needs no sheet.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::base::Disableable;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-glossary";

/// One interaction with the glossary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GlossaryEvent {
    /// Opens the sheet to define a term in the post.
    Open(PostId),
    /// The post no longer defines its term.
    Undefine(PostId),
    /// Asks the AI for a draft of the definition.
    Draft,
    Save,
    Close,
}

/// The sheet's state.
#[derive(Clone)]
pub struct GlossaryDialog {
    pub post_id: PostId,
    pub term: Entity<InputState>,
    pub text: Entity<InputState>,
    /// The AI is drafting the definition.
    pub drafting: bool,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: GlossaryEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Thought(ThoughtEvent::Glossary(event.clone())), window, cx)
}

fn field(label: &str, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr(label), theme)).child(Input::new(input))
}

/// The overlay.
pub fn render_glossary_sheet(dialog: &GlossaryDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let draft_label = if dialog.drafting { tr("posts.glossary.drafting") } else { tr("posts.glossary.draft") };
    let draft = Button::new("data-glossary-draft").ghost().xsmall().disabled(dialog.drafting).icon(Icon::new(LucideIcon::Sparkles)).label(draft_label).on_click(send(&on_event, GlossaryEvent::Draft));
    let definition = field("posts.glossary.definition_label", &dialog.text, theme).child(div().flex().justify_end().child(draft));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.glossary.explain"), theme)).child(field("posts.glossary.term_label", &dialog.term, theme)).child(definition).children(problem);
    let buttons = footer(theme)
        .child(Button::new("data-glossary-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, GlossaryEvent::Close)))
        .child(Button::new("data-glossary-save").primary().small().icon(Icon::new(LucideIcon::BookOpen)).label(tr("posts.glossary.save")).on_click(send(&on_event, GlossaryEvent::Save)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::BookOpen,
        title: tr("posts.glossary.title"),
        subtitle: Some(tr("posts.glossary.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Thought(ThoughtEvent::Glossary(GlossaryEvent::Close)), window, cx)),
    };
    sheet_frame(parts, theme)
}
