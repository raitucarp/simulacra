//! The right pane: "Surprise me" and "Write it for me", the editable prompt (rebuilt from the
//! picks, free to edit), the negative prompt when the engine takes one, the AI's ideas, then
//! generation and its results.

use super::labels::{failure_art, failure_text, tr};
use super::model::{CandidatesStatus, IdeasEvent};
use super::results::render_generation;
use super::sheet::{IdeasAction, IdeasInputs};
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, NoticeKey};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Textarea;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const SCROLL_ID: &str = "image-ideas-pane";
const SURPRISE_ID: &str = "image-ideas-surprise";
const WRITE_ID: &str = "image-ideas-write";
const STOP_ID: &str = "image-ideas-stop-writing";
const USE_ID: &str = "image-ideas-use";

/// The scrollable pane right of the picks.
pub(super) fn render_prompt_pane(inputs: &IdeasInputs<'_>) -> Stateful<Div> {
    let (sheet, theme, on_event) = (inputs.sheet, inputs.theme, &inputs.on_event);
    let writing = sheet.candidates == CandidatesStatus::Running;
    let surprise = event_button(Button::new(SURPRISE_ID).outline().icon(Icon::new(LucideIcon::Dices)).label(tr("surprise")), IdeasEvent::Surprise, on_event);
    let write = event_button(Button::new(WRITE_ID).outline().icon(Icon::new(LucideIcon::WandSparkles)).label(tr("write")).loading(writing), IdeasEvent::WriteForMe, on_event);
    let negative = sheet.negative.clone().map(|text| muted(format!("{} {text}", tr("negative_label")), theme));
    let editor = div()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .bg(theme.input_bg)
        .text_size(theme.font_size_sm)
        .child(Textarea::new(inputs.prompt).bordered(false));
    div()
        .id(SCROLL_ID)
        .flex_1()
        .min_w_0()
        .h_full()
        .overflow_y_scroll()
        .p(theme.panel_padding)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(div().flex().flex_wrap().gap(theme.control_gap).child(surprise).child(write))
        .child(label(tr("prompt_label"), theme))
        .child(editor)
        .child(muted(tr("prompt_hint"), theme))
        .children(negative)
        .children(render_candidates(&sheet.candidates, on_event, theme))
        .child(render_generation(sheet, on_event, theme))
}

fn event_button(button: Button, event: IdeasEvent, on_event: &IdeasAction) -> Button {
    let on_event = on_event.clone();
    button.small().on_click(move |_, window, cx| on_event(event.clone(), window, cx))
}

pub(super) fn label(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(text)
}

pub(super) fn muted(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).text_color(theme.text_muted).line_height(relative(theme.line_height_body)).child(text)
}

/// A notice with art, centred in the pane.
pub(super) fn notice(art: NoticeKey, text: String, theme: &ThemeTokens) -> Div {
    div().py(theme.spacing_sm).flex().flex_col().items_center().gap(theme.stack_gap).child(art_state(ArtState::hint(Illustration::notice(art).width(theme.art_notice_width), text), theme))
}

/// The AI's ideas, what is happening, or why there are none.
fn render_candidates(status: &CandidatesStatus, on_event: &IdeasAction, theme: &ThemeTokens) -> Option<Div> {
    match status {
        CandidatesStatus::Idle => None,
        CandidatesStatus::Running => {
            let stop = event_button(Button::new(STOP_ID).ghost().icon(Icon::new(LucideIcon::X)).label(tr("cancel")), IdeasEvent::Cancel, on_event);
            Some(notice(NoticeKey::Working, tr("writing"), theme).child(stop))
        }
        CandidatesStatus::Failed(failure) => Some(notice(failure_art(failure), failure_text(failure), theme)),
        CandidatesStatus::Ready(ideas) => {
            let cards = ideas.iter().enumerate().map(|(index, idea)| candidate_card(index, idea, on_event, theme));
            Some(div().flex().flex_col().gap(theme.spacing_xs).child(label(tr("candidates_title"), theme)).children(cards))
        }
    }
}

fn candidate_card(index: usize, idea: &str, on_event: &IdeasAction, theme: &ThemeTokens) -> Div {
    let pick = event_button(Button::new((USE_ID, index)).ghost().icon(Icon::new(LucideIcon::Check)).label(tr("use_candidate")), IdeasEvent::UseCandidate(index), on_event);
    div()
        .p(theme.spacing_sm)
        .flex()
        .items_start()
        .gap(theme.control_gap)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_card)
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).line_height(relative(theme.line_height_body)).child(idea.to_string()))
        .child(pick)
}
