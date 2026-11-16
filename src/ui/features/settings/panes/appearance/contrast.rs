//! Live WCAG contrast checks of the edited palette and their badges.

use super::consts::{GRADE_AA, GRADE_AAA, RATIO_SUFFIX};
use crate::theme::color::{contrast_ratio, AAA_TEXT_CONTRAST, AA_TEXT_CONTRAST, AA_UI_CONTRAST};
use crate::theme::{ThemePalette, ThemeTokens};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, ParentElement, Rgba, Styled};
use rust_i18n::t;

/// WCAG level a contrast ratio reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastGrade {
    Aaa,
    Aa,
    Fail,
}

impl ContrastGrade {
    /// Grades `ratio` against an AA minimum and an AAA minimum.
    pub fn of(ratio: f32, aa: f32, aaa: f32) -> Self {
        if ratio >= aaa {
            ContrastGrade::Aaa
        } else if ratio >= aa {
            ContrastGrade::Aa
        } else {
            ContrastGrade::Fail
        }
    }
}

/// One foreground/background pair with its measured ratio and grade.
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastCheck {
    /// i18n keys of the foreground and background roles.
    pub fg_role: &'static str,
    pub bg_role: &'static str,
    pub ratio: f32,
    pub grade: ContrastGrade,
}

fn text_check(fg_role: &'static str, fg: Rgba, bg_role: &'static str, bg: Rgba) -> ContrastCheck {
    let ratio = contrast_ratio(fg, bg);
    ContrastCheck { fg_role, bg_role, ratio, grade: ContrastGrade::of(ratio, AA_TEXT_CONTRAST, AAA_TEXT_CONTRAST) }
}

/// Text and muted text on every surface of the raw palette (what the user picked), plus the
/// derived accent label on the accent (graded as large text / UI: AA 3.0, AAA 4.5).
pub fn contrast_checks(palette: &ThemePalette, tokens: &ThemeTokens) -> Vec<ContrastCheck> {
    let c = &palette.colors;
    let surfaces = [("appearance.role_bg", c.bg), ("appearance.role_surface", c.surface), ("appearance.role_elevated", c.elevated)];
    let texts = [("appearance.role_fg", c.fg), ("appearance.role_fg_muted", c.fg_muted)];
    let mut checks: Vec<ContrastCheck> = texts
        .iter()
        .flat_map(|&(fg_role, fg)| surfaces.iter().map(move |&(bg_role, bg)| text_check(fg_role, fg, bg_role, bg)))
        .collect();
    let ratio = contrast_ratio(tokens.on_accent, c.accent);
    checks.push(ContrastCheck {
        fg_role: "appearance.role_on_accent",
        bg_role: "appearance.role_accent",
        ratio,
        grade: ContrastGrade::of(ratio, AA_UI_CONTRAST, AA_TEXT_CONTRAST),
    });
    checks
}

/// Badge color and label of a grade (chrome drawn with the app's own tokens).
fn grade_style(grade: ContrastGrade, theme: &ThemeTokens) -> (Rgba, Rgba, String) {
    match grade {
        ContrastGrade::Aaa => (theme.accent_success_subtle, theme.accent_success, GRADE_AAA.to_string()),
        ContrastGrade::Aa => (theme.accent_brand_subtle, theme.accent_brand, GRADE_AA.to_string()),
        ContrastGrade::Fail => (theme.accent_dialectic_subtle, theme.accent_danger, t!("appearance.contrast_fail").to_string()),
    }
}

/// One row: "Text on background", the ratio and the grade badge.
pub fn render_check(check: &ContrastCheck, theme: &ThemeTokens) -> Div {
    let (fill, ink, label) = grade_style(check.grade, theme);
    let pair = t!("appearance.contrast_pair", fg = t!(check.fg_role), bg = t!(check.bg_role)).to_string();
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .text_size(theme.font_size_xs)
        .child(div().flex_1().truncate().text_color(theme.text_secondary).child(pair))
        .child(div().font_family(theme.font_family_mono.clone()).text_color(theme.text_muted).child(format!("{:.2}{RATIO_SUFFIX}", check.ratio)))
        .child(
            div()
                .px(theme.spacing_sm)
                .rounded(theme.radius_full)
                .bg(fill)
                .text_color(ink)
                .font_weight(FontWeight::SEMIBOLD)
                .when(check.grade == ContrastGrade::Fail, |badge| badge.border(theme.border_width_hairline).border_color(ink))
                .child(label),
        )
}
