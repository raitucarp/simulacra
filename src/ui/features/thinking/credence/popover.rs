//! The popover: the stated chance with presets and nudges, the "resolves by" presets, and
//! (while open) the resolution note with true / false / ambiguous, or (once resolved) the
//! outcome with a reopen button; remove at the bottom.

use super::super::labels::{keyed, outcome_colour, outcome_label, tr};
use super::{CredenceChip, CredenceControls, CredenceEvent, NUDGE_POINTS, PERCENT_PRESETS};
use crate::domain::thinking::credence::{DuePreset, Outcome};
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::components::choice_chip;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const PERCENT_ID: &str = "credence-percent";
const DUE_ID: &str = "credence-due";
const OUTCOME_ID: &str = "credence-outcome";
const BUTTON_IDS: [&str; 4] = ["credence-less", "credence-more", "credence-reopen", "credence-remove"];

/// "70% · resolves by 2026-12-01" (the chip's tooltip).
pub fn summary(chip: &CredenceChip) -> String {
    let base = match chip.outcome {
        Some(outcome) => t!("thinking.credence.resolved_summary", percent = chip.percent, outcome = outcome_label(outcome)).to_string(),
        None => t!("thinking.credence.summary", percent = chip.percent).to_string(),
    };
    match chip.due.filter(|_| chip.outcome.is_none()) {
        Some(due) => format!("{base} · {}", t!("thinking.credence.due_on", date = crate::ui::format::date(due))),
        None => base,
    }
}

fn send(controls: &CredenceControls, event: CredenceEvent) -> impl Fn(&mut Window, &mut App) + 'static {
    let on_event = controls.on_event.clone();
    move |window, cx| on_event(event.clone(), window, cx)
}

fn section(title: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(title)
}

/// The popover's body for `post`.
pub fn popover_content(post: &PostId, controls: &CredenceControls, theme: &ThemeTokens) -> Div {
    let chip = controls.chip.clone();
    let headline = chip.as_ref().map_or_else(|| tr("thinking.credence.new_hint"), summary);
    let body = div()
        .w(theme.credence_popover_width)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("thinking.credence.title")))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(headline))
        .child(percent_row(post, controls, chip.as_ref().map(|c| c.percent), theme));
    let Some(chip) = chip else { return body };
    let remove = Button::new(BUTTON_IDS[3]).ghost().xsmall().label(tr("thinking.credence.remove")).on_click({
        let act = send(controls, CredenceEvent::Remove(post.clone()));
        move |_, window, cx| act(window, cx)
    });
    let state = if chip.outcome.is_some() { resolved_row(post, controls, &chip, theme) } else { open_rows(post, controls, theme) };
    body.child(state).child(div().flex().justify_end().child(remove))
}

fn percent_row(post: &PostId, controls: &CredenceControls, current: Option<u8>, theme: &ThemeTokens) -> Div {
    let chips = PERCENT_PRESETS.iter().map(|percent| {
        let act = send(controls, CredenceEvent::SetPercent(post.clone(), *percent));
        choice_chip(ElementId::NamedInteger(PERCENT_ID.into(), u64::from(*percent)), &format!("{percent}%"), current == Some(*percent), theme, act).text_size(theme.font_size_2xs)
    });
    let nudge = |slot: usize, points: i8, label: &str| {
        let act = send(controls, CredenceEvent::Nudge(post.clone(), points));
        Button::new(BUTTON_IDS[slot]).ghost().xsmall().label(label.to_string()).on_click(move |_, window, cx| act(window, cx))
    };
    let nudges = current.map(|_| div().flex().gap(theme.spacing_xs).child(nudge(0, -NUDGE_POINTS, &format!("−{NUDGE_POINTS}"))).child(nudge(1, NUDGE_POINTS, &format!("+{NUDGE_POINTS}"))));
    div().flex().flex_col().gap(theme.spacing_xs).child(section(tr("thinking.credence.chance"), theme)).child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips)).children(nudges)
}

/// The "resolves by" presets and the resolution buttons of an open prediction.
fn open_rows(post: &PostId, controls: &CredenceControls, theme: &ThemeTokens) -> Div {
    let presets = DuePreset::ALL.iter().map(|preset| Some(*preset)).chain(std::iter::once(None));
    let chips = presets.enumerate().map(|(i, preset)| {
        let label = preset.map_or_else(|| tr("thinking.credence.no_date"), |p| keyed("thinking.credence.preset", p.key()));
        let act = send(controls, CredenceEvent::SetDue(post.clone(), preset));
        choice_chip(ElementId::NamedInteger(DUE_ID.into(), i as u64), &label, false, theme, act).text_size(theme.font_size_2xs)
    });
    let outcomes = Outcome::ALL.iter().enumerate().map(|(i, outcome)| {
        let act = send(controls, CredenceEvent::Resolve(post.clone(), *outcome));
        Button::new(ElementId::NamedInteger(OUTCOME_ID.into(), i as u64)).outline().small().label(outcome_label(*outcome)).on_click(move |_, window, cx| act(window, cx))
    });
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(section(tr("thinking.credence.resolves_by"), theme))
        .child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips))
        .child(section(tr("thinking.credence.resolve"), theme))
        .child(Input::new(&controls.note).small())
        .child(div().flex().flex_wrap().gap(theme.spacing_xs).children(outcomes))
}

fn resolved_row(post: &PostId, controls: &CredenceControls, chip: &CredenceChip, theme: &ThemeTokens) -> Div {
    let outcome = chip.outcome.unwrap_or(Outcome::Ambiguous);
    let act = send(controls, CredenceEvent::Reopen(post.clone()));
    let reopen = Button::new(BUTTON_IDS[2]).ghost().xsmall().label(tr("thinking.credence.reopen")).on_click(move |_, window, cx| act(window, cx));
    let badge = div().px(theme.spacing_sm).py(theme.spacing_3xs).rounded(theme.radius_full).bg(outcome_colour(Some(outcome), theme)).text_size(theme.font_size_2xs).text_color(theme.on_accent).child(outcome_label(outcome));
    let note = (!chip.note.is_empty()).then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(chip.note.clone()));
    div().flex().flex_col().gap(theme.spacing_xs).child(div().flex().items_center().gap(theme.control_gap).child(badge).child(reopen)).children(note)
}
