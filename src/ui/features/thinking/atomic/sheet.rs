//! The "Check atomicity" sheet: the post's score, its claims, the findings that explain the
//! score, and the proposed split (one post per claim) with "Split into atomic posts", which
//! opens the composer on those posts, quoting the original, for the user to edit and post.

use super::super::labels::{finding_text, tr};
use super::score_colour;
use crate::domain::thinking::atomic::AtomicReport;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "atomic-backdrop";
const CARD_ID: &str = "atomic-card";
const SCROLL_ID: &str = "atomic-scroll";
const BUTTON_IDS: [&str; 2] = ["atomic-split", "atomic-close"];

/// What the sheet shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicSheet {
    pub post_id: PostId,
    /// The post's opening words.
    pub source: String,
    pub report: AtomicReport,
    /// One post per claim.
    pub proposal: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicSheetEvent {
    Split,
    Close,
}

/// Sends a sheet event to the shell.
pub type SheetAction = Rc<dyn Fn(AtomicSheetEvent, &mut Window, &mut App) + 'static>;

fn heading(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(text)
}

fn numbered(items: &[String], theme: &ThemeTokens) -> Div {
    let rows = items.iter().enumerate().map(|(i, item)| {
        div().flex().items_start().gap(theme.inline_gap).text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).child(div().flex_none().text_color(theme.text_muted).child(format!("{}.", i + 1))).child(div().flex_1().min_w_0().text_color(theme.text_primary).child(item.clone()))
    });
    div().flex().flex_col().gap(theme.spacing_xs).children(rows)
}

fn score_block(sheet: &AtomicSheet, theme: &ThemeTokens) -> Div {
    let colour = score_colour(sheet.report.score, theme);
    let verdict = if sheet.report.is_atomic() { tr("thinking.atomic.is_atomic") } else { t!("thinking.atomic.claims", count = sheet.report.claims.len()).to_string() };
    div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(div().flex_none().size(theme.atomic_score_size).rounded(theme.radius_full).border(theme.border_width_threadline).border_color(colour).flex().items_center().justify_center().text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(colour).child(sheet.report.score.to_string()))
        .child(div().flex().flex_col().gap(theme.spacing_3xs).child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(verdict)).child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr("thinking.atomic.score_hint"))))
}

fn body(sheet: &AtomicSheet, theme: &ThemeTokens) -> Stateful<Div> {
    let findings: Vec<String> = sheet.report.findings.iter().map(finding_text).collect();
    let split = sheet.proposal.len() > 1;
    let posts = sheet.proposal.iter().map(|post| div().p(theme.spacing_sm).rounded(theme.radius_md).bg(theme.bg_app).border(theme.border_width_hairline).border_color(theme.border_subtle).text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(post.clone()));
    div()
        .id(SCROLL_ID)
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .flex()
        .flex_col()
        .gap(theme.section_gap)
        .p(theme.card_padding)
        .child(score_block(sheet, theme))
        .child(div().flex().flex_col().gap(theme.stack_gap).child(heading(tr("thinking.atomic.claims_title"), theme)).child(numbered(&sheet.report.claims, theme)))
        .when(!findings.is_empty(), |b| b.child(div().flex().flex_col().gap(theme.stack_gap).child(heading(tr("thinking.atomic.findings_title"), theme)).child(numbered(&findings, theme))))
        .when(split, |b| b.child(div().flex().flex_col().gap(theme.stack_gap).child(heading(t!("thinking.atomic.proposal_title", count = sheet.proposal.len()).to_string(), theme)).children(posts)))
}

/// The overlay.
pub fn render_atomic_sheet(sheet: &AtomicSheet, on_event: SheetAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (dismiss, close, split) = (on_event.clone(), on_event.clone(), on_event);
    let header = div().flex().items_center().gap(theme.inline_gap).px(theme.card_padding).py(theme.spacing_sm).border_b(theme.border_width_hairline).border_color(theme.border_subtle)
        .child(Icon::new(LucideIcon::Atom).size(theme.icon_size_sm).text_color(theme.accent_brand))
        .child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("thinking.atomic.title"))).child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(sheet.source.clone())));
    let footer = div().flex().justify_end().gap(theme.control_gap).px(theme.card_padding).py(theme.spacing_sm).border_t(theme.border_width_hairline).border_color(theme.border_subtle)
        .child(Button::new(BUTTON_IDS[1]).ghost().small().label(tr("thinking.close")).on_click(move |_, w, cx| close(AtomicSheetEvent::Close, w, cx)))
        .when(sheet.proposal.len() > 1, |f| f.child(Button::new(BUTTON_IDS[0]).primary().small().icon(Icon::new(LucideIcon::Split)).label(tr("thinking.atomic.split")).on_click(move |_, w, cx| split(AtomicSheetEvent::Split, w, cx))));
    let card = div().id(CARD_ID).w(theme.atomic_sheet_width).max_w_full().max_h(theme.atomic_sheet_max_height).flex().flex_col().overflow_hidden().rounded(theme.radius_lg).bg(theme.bg_elevated).border(theme.border_width_hairline).border_color(theme.border).elevation_shadow(theme)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .child(header).child(body(sheet, theme)).child(footer);
    div().id(BACKDROP_ID).absolute().inset_0().occlude().bg(theme.overlay).flex().flex_col().items_center().pt(theme.palette_top_offset)
        .on_mouse_down(MouseButton::Left, move |_, w, cx| dismiss(AtomicSheetEvent::Close, w, cx))
        .child(card)
}
