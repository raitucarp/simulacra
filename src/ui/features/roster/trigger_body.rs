//! A4: the body of an agent's trigger dialog: a new post or a reply to the network's newest
//! post; for a new post an optional topic and vault document; and what the run will likely
//! cost. The estimate says plainly that it is one (no prices are kept).

use crate::domain::document::DocumentId;
use crate::domain::post::{PostSeed, TriggerMode};
use crate::domain::spend::TriggerEstimate;
use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const NEW_POST_ID: &str = "trigger-new-post";
const REPLY_ID: &str = "trigger-reply";
const NO_DOCUMENT_ID: &str = "trigger-no-document";
const DOCUMENT_ID: &str = "trigger-document-";

/// What the dialog is set to (the topic lives in its input).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TriggerChoice {
    /// Reply to the newest post instead of writing a new one.
    pub reply: bool,
    pub document: Option<DocumentId>,
}

impl TriggerChoice {
    /// The run this choice and `topic` ask for.
    pub fn mode(&self, topic: &str) -> TriggerMode {
        match self.reply {
            true => TriggerMode::Reply,
            false => TriggerMode::NewPost { seed: PostSeed { topic: topic.trim().to_string(), document: self.document.clone() } },
        }
    }
}

/// Changes the choice (and redraws the dialog).
pub type ChangeChoice = Rc<dyn Fn(TriggerChoice, &mut Window, &mut App)>;

/// What the body shows.
pub struct TriggerBody<'a> {
    pub choice: &'a TriggerChoice,
    pub topic: &'a Entity<InputState>,
    /// The network's vault documents: id and title.
    pub documents: &'a [(DocumentId, String)],
    /// `None` when the run cannot start (a reply with no post to answer).
    pub estimate: Option<TriggerEstimate>,
    pub on_change: ChangeChoice,
}

/// The sentence under the choices: tokens and requests, or why nothing is sent.
pub fn estimate_text(estimate: Option<TriggerEstimate>) -> String {
    match estimate {
        None => t!("trigger.nothing_to_answer").to_string(),
        Some(estimate) if !estimate.uses_model() => t!("trigger.estimate.offline").to_string(),
        Some(estimate) => {
            let tokens = t!("trigger.estimate.tokens", sent = estimate.tokens_in, back = estimate.tokens_out).to_string();
            format!("{tokens} · {}", t_count("trigger.estimate.requests", estimate.requests as usize))
        }
    }
}

fn choose(id: SharedString, label: String, selected: bool, next: TriggerChoice, on_change: &ChangeChoice) -> Button {
    let on_change = on_change.clone();
    Button::new(id).ghost().small().selected(selected).label(label).on_click(move |_, window, cx| on_change(next.clone(), window, cx))
}

fn documents_row(body: &TriggerBody<'_>, theme: &ThemeTokens) -> Div {
    let current = body.choice;
    let none = choose(NO_DOCUMENT_ID.into(), t!("trigger.no_document").to_string(), current.document.is_none(), TriggerChoice { document: None, ..current.clone() }, &body.on_change);
    let chips = body.documents.iter().map(|(id, title)| {
        let next = TriggerChoice { document: Some(id.clone()), ..current.clone() };
        choose(SharedString::from(format!("{DOCUMENT_ID}{id}")), title.clone(), current.document.as_ref() == Some(id), next, &body.on_change)
    });
    div().flex().flex_wrap().gap(theme.spacing_2xs).child(none).children(chips)
}

/// The dialog's body.
pub fn render_trigger_body(body: TriggerBody<'_>, theme: &ThemeTokens) -> Div {
    let current = body.choice;
    let modes = div()
        .flex()
        .gap(theme.spacing_2xs)
        .child(choose(NEW_POST_ID.into(), t!("trigger.new_post").to_string(), !current.reply, TriggerChoice { reply: false, ..current.clone() }, &body.on_change))
        .child(choose(REPLY_ID.into(), t!("trigger.reply").to_string(), current.reply, TriggerChoice { reply: true, ..current.clone() }, &body.on_change));
    let label = |key: &str| div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(t!(key).to_string());
    let seed = div().flex().flex_col().gap(theme.spacing_xs).child(label("trigger.topic")).child(Input::new(body.topic).small()).child(label("trigger.document")).child(documents_row(&body, theme));
    let estimate = div()
        .flex()
        .items_start()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_secondary)
        .child(Icon::new(LucideIcon::Gauge).xsmall().text_color(theme.text_muted))
        .child(div().flex_1().min_w_0().flex().flex_col().child(estimate_text(body.estimate)).when(body.estimate.is_some_and(TriggerEstimate::uses_model), |text| text.child(div().text_color(theme.text_muted).child(t!("trigger.estimate.note").to_string()))));
    div().flex().flex_col().gap(theme.spacing_sm).child(modes).when(!current.reply, |this| this.child(seed)).child(estimate)
}
