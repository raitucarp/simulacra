//! The history sheet: every version of a post or a vault document, newest first, and the
//! word diff from the chosen version to the current one, with "Restore this version".

use super::diff_view::{render_diff, render_stats};
use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, sheet_frame, SheetParts};
use super::text::{format_time, tr};
use crate::domain::data::history::{word_diff, RevisionSubject, Version, VersionKind};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-history";
const ROW_ID_PREFIX: &str = "data-history-version-";
const RESTORE_ID: &str = "data-history-restore";
const CLOSE_ID: &str = "data-history-close";
const EDIT_FROM_ID: &str = "data-history-edit-from";

/// What the sheet shows.
#[derive(Debug, Clone, PartialEq)]
pub struct HistorySheet {
    pub subject: RevisionSubject,
    /// The post's opening words or the document's title.
    pub title: String,
    pub versions: Vec<Version>,
    pub selected: usize,
}

impl HistorySheet {
    /// The sheet on its most recent earlier version (the current one when there is none).
    pub fn new(subject: RevisionSubject, title: String, versions: Vec<Version>) -> Self {
        let selected = usize::from(versions.len() > 1);
        Self { subject, title, versions, selected }
    }
}

fn kind_label(version: &Version) -> String {
    match version.kind {
        VersionKind::Current => tr("data.history.current"),
        VersionKind::Earlier => tr("data.history.earlier"),
        VersionKind::Original => tr("data.history.original"),
    }
}

fn version_row(index: usize, version: &Version, selected: bool, on_event: &DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let on_event = on_event.clone();
    let when = version.replaced_at.map_or_else(|| tr("data.history.now"), |at| t!("data.history.until", time = format_time(at)).to_string());
    let hover = theme.hover_bg;
    div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{index}")))
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .when(selected, |row| row.bg(theme.accent_brand_subtle).border(theme.border_width_hairline).border_color(theme.accent_brand_border))
        .when(!selected, |row| row.hover(move |s| s.bg(hover)))
        .on_click(move |_, window, cx| on_event(DataEvent::SelectVersion(index), window, cx))
        .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(kind_label(version)))
        .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(when))
}

fn comparison(sheet: &HistorySheet, theme: &ThemeTokens) -> Div {
    let current = &sheet.versions[0];
    let chosen = &sheet.versions[sheet.selected.min(sheet.versions.len() - 1)];
    if chosen.kind == VersionKind::Current {
        let note = if sheet.versions.len() > 1 { tr("data.history.pick_hint") } else { tr("data.history.no_versions") };
        return div().flex().flex_col().gap(theme.stack_gap).child(hint(note, theme)).child(render_diff(&word_diff(&current.content, &current.content), theme));
    }
    let ops = word_diff(&chosen.content, &current.content);
    let title_change = (chosen.title != current.title && !chosen.title.is_empty()).then(|| hint(t!("data.history.title_changed", before = chosen.title.as_str(), after = current.title.as_str()).to_string(), theme));
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(div().flex().flex_wrap().items_center().justify_between().gap(theme.control_gap).child(heading(tr("data.history.changes_since"), theme)).child(render_stats(&ops, theme)))
        .children(title_change)
        .child(render_diff(&ops, theme))
}

/// The overlay.
pub fn render_history_sheet(sheet: &HistorySheet, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let rows = sheet.versions.iter().enumerate().map(|(i, v)| version_row(i, v, i == sheet.selected, &on_event, theme));
    let list = div().w(theme.history_list_width).flex_none().flex().flex_col().gap(theme.spacing_2xs).child(heading(tr("data.history.versions"), theme)).children(rows);
    let body = div().child(div().flex().flex_wrap().items_start().gap(theme.section_gap).child(list).child(div().flex_1().min_w(theme.history_list_width).child(comparison(sheet, theme))));
    let (restore, close, closing, branch) = (on_event.clone(), on_event.clone(), on_event.clone(), on_event);
    let selected = sheet.selected;
    let is_post = matches!(sheet.subject, RevisionSubject::Post(_));
    let buttons = footer(theme)
        .child(Button::new(CLOSE_ID).ghost().small().label(tr("data.close")).on_click(move |_, w, cx| close(DataEvent::CloseHistory, w, cx)))
        .when(is_post, |row| row.child(Button::new(EDIT_FROM_ID).ghost().small().icon(Icon::new(LucideIcon::GitBranch)).label(tr("data.history.edit_from")).disabled(selected == 0).on_click(move |_, w, cx| branch(DataEvent::EditFromVersion(selected), w, cx)))) // V4
        .child(Button::new(RESTORE_ID).primary().small().icon(Icon::new(LucideIcon::RotateCcw)).label(tr("data.history.restore")).disabled(selected == 0).on_click(move |_, w, cx| restore(DataEvent::RestoreVersion(selected), w, cx)));
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::RotateCcwClock,
        title: tr("data.history.title"),
        subtitle: Some(sheet.title.clone()),
        body,
        footer: buttons,
        on_close: Rc::new(move |w, cx| closing(DataEvent::CloseHistory, w, cx)),
    };
    sheet_frame(parts, theme)
}
