//! The "Schedule…" picker of the composer (Y3): a footer button opens a row with the moment
//! field (local time, `2026-11-22 09:00`), one-click presets, and a line saying when the
//! thread goes out or why the typed moment cannot be used.

use crate::domain::post::schedule::{format_moment, SchedulePreset, ScheduleProblem};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, App, Div, Entity, ParentElement, SharedString, Styled, Window};
use jiff::tz::TimeZone;
use jiff::Timestamp;
use rust_i18n::t;
use std::rc::Rc;

/// Picks a preset moment.
pub type PresetCallback = Rc<dyn Fn(SchedulePreset, &mut Window, &mut App) + 'static>;
/// A no-argument picker action (open, close).
pub type ScheduleAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

const BUTTON_ID: &str = "composer-schedule";
const CLEAR_ID: &str = "composer-schedule-clear";
const PRESET_ID: &str = "composer-schedule-preset";

/// The open picker.
#[derive(Clone)]
pub struct ScheduleView {
    pub input: Entity<InputState>,
    /// The typed moment, or why it cannot be used; `None` while the field is empty.
    pub choice: Option<Result<Timestamp, ScheduleProblem>>,
    pub on_preset: PresetCallback,
    pub on_clear: ScheduleAction,
}

/// Locale key of a preset's name.
pub fn preset_label_key(preset: SchedulePreset) -> &'static str {
    match preset {
        SchedulePreset::InAnHour => "composer.schedule.preset.in_an_hour",
        SchedulePreset::TomorrowMorning => "composer.schedule.preset.tomorrow_morning",
        SchedulePreset::InAWeek => "composer.schedule.preset.in_a_week",
    }
}

/// An example moment as it is typed (tomorrow morning), for the field and its hint.
pub fn example_moment() -> String {
    let now = crate::time::local_now();
    let example = SchedulePreset::TomorrowMorning.moment(&now).unwrap_or(now);
    format_moment(example.timestamp(), example.time_zone())
}

/// The line under the field: when the thread goes out, or what is wrong (`true`).
pub fn status_line(choice: Option<Result<Timestamp, ScheduleProblem>>) -> Option<(String, bool)> {
    Some(match choice? {
        Ok(at) => (t!("composer.schedule.goes_out", when = format_moment(at, &TimeZone::system())).to_string(), false),
        Err(ScheduleProblem::Unreadable) => (t!("composer.schedule.unreadable", example = example_moment()).to_string(), true),
        Err(ScheduleProblem::NotInFuture) => (t!("composer.schedule.not_in_future").to_string(), true),
    })
}

/// The footer's "Schedule…" toggle, highlighted while the picker is open.
pub(super) fn schedule_button(open: bool, on_toggle: ScheduleAction) -> Button {
    Button::new(BUTTON_ID)
        .ghost()
        .small()
        .selected(open)
        .icon(Icon::new(LucideIcon::CalendarClock))
        .label(t!("composer.schedule.button").to_string())
        .on_click(move |_, window, cx| on_toggle(window, cx))
}

fn preset_button(preset: SchedulePreset, on_preset: PresetCallback) -> Button {
    Button::new(SharedString::from(format!("{PRESET_ID}-{}", preset.key())))
        .ghost()
        .xsmall()
        .label(t!(preset_label_key(preset)).to_string())
        .on_click(move |_, window, cx| on_preset(preset, window, cx))
}

/// The picker row: field, presets and status.
pub(super) fn schedule_row(view: &ScheduleView, theme: &ThemeTokens) -> Div {
    let on_clear = view.on_clear.clone();
    let clear = Button::new(CLEAR_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(t!("composer.schedule.clear").to_string()).on_click(move |_, window, cx| on_clear(window, cx));
    let label = div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("composer.schedule.label").to_string());
    let field = div().flex().items_center().gap(theme.control_gap).child(label).child(div().flex_1().child(Input::new(&view.input).small())).child(clear);
    let presets = div().flex().flex_wrap().items_center().gap(theme.spacing_xs).children(SchedulePreset::ALL.map(|preset| preset_button(preset, view.on_preset.clone())));
    let status = status_line(view.choice).map(|(text, problem)| {
        let colour = if problem { theme.accent_danger } else { theme.text_secondary };
        div().text_size(theme.font_size_xs).text_color(colour).child(text)
    });
    div().w_full().flex().flex_col().gap(theme.spacing_xs).px(theme.panel_padding).py(theme.spacing_xs).border_t(theme.border_width_hairline).border_color(theme.border_subtle).child(field).child(presets).children(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_label_is_translated_and_the_status_reads_each_case() {
        let keys = SchedulePreset::ALL.map(preset_label_key);
        for key in keys.into_iter().chain(["composer.schedule.button", "composer.schedule.label", "composer.schedule.clear"]) {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
        assert_eq!(status_line(None), None);
        assert!(status_line(Some(Ok(Timestamp::now()))).is_some_and(|(_, problem)| !problem));
        for problem in [ScheduleProblem::Unreadable, ScheduleProblem::NotInFuture] {
            assert!(status_line(Some(Err(problem))).is_some_and(|(_, is_problem)| is_problem));
        }
        assert!(crate::domain::post::schedule::parse_moment(&example_moment(), &crate::time::local_now()).is_ok());
    }
}
