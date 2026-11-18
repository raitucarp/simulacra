//! The four grade buttons: each names the grade, shows the interval it gives and its key.

use super::super::text::{grade_hint, grade_label, interval_label};
use crate::domain::review::Grade;
use crate::theme::ThemeTokens;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const GRADE_ID_PREFIX: &str = "review-grade-";

/// A grade chosen with a button.
pub type GradeAction = Rc<dyn Fn(Grade, &mut Window, &mut App) + 'static>;

/// The accent of a grade.
pub fn grade_color(grade: Grade, theme: &ThemeTokens) -> Rgba {
    match grade {
        Grade::Again => theme.accent_danger,
        Grade::Hard => theme.warning,
        Grade::Good => theme.accent_success,
        Grade::Easy => theme.info,
    }
}

fn grade_button(grade: Grade, interval: Option<u32>, suggested: bool, on_grade: GradeAction, theme: &ThemeTokens) -> Stateful<Div> {
    let color = grade_color(grade, theme);
    let hint = grade_hint(grade);
    div()
        .id(ElementId::Name(format!("{GRADE_ID_PREFIX}{}", grade.key()).into()))
        .flex_1()
        .min_w_0()
        .h(theme.review_grade_button_height)
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(theme.spacing_3xs)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(if suggested { color } else { theme.border })
        .bg(if suggested { theme.bg_elevated } else { theme.bg_card })
        .cursor_pointer()
        .hover(|button| button.bg(theme.hover_bg))
        .tooltip(move |window, cx| Tooltip::new(hint.clone()).build(window, cx))
        .on_click(move |_, window, cx| on_grade(grade, window, cx))
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .child(render_key_cap((grade.index() + 1).to_string(), theme))
                .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(color).child(grade_label(grade))),
        )
        .when_some(interval, |button, days| {
            button.child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(interval_label(days)))
        })
}

/// The row of grade buttons; `suggested` is outlined (the typed-answer verdict).
pub fn render_grade_buttons(intervals: Option<[u32; 4]>, suggested: Option<Grade>, on_grade: GradeAction, theme: &ThemeTokens) -> Div {
    let buttons = Grade::ALL.map(|grade| {
        let interval = intervals.map(|all| all[grade.index()]);
        grade_button(grade, interval, suggested == Some(grade), on_grade.clone(), theme)
    });
    div().w_full().flex().gap(theme.control_gap).children(buttons)
}
