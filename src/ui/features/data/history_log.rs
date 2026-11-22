//! The edit log under a post's version tree (V7): each version's number, when it was saved,
//! the bump of the edit that made it and its note.

use super::sheet::heading;
use super::text::{format_time, tr};
use crate::domain::data::history::LogEntry;
use crate::theme::ThemeTokens;
use crate::ui::features::composer::{bump_label_key, version_label};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// What the entry's edit did: its bump, or "Original".
pub(super) fn bump_text(entry: &LogEntry) -> String {
    entry.bump.map_or_else(|| tr("data.history.original"), |bump| t!(bump_label_key(bump)).to_string())
}

fn cell(text: String, width: Option<Pixels>, colour: Rgba, theme: &ThemeTokens) -> Div {
    div().flex_none().when_some(width, |cell, width| cell.w(width)).text_size(theme.font_size_xs).text_color(colour).child(text)
}

fn entry_row(entry: &LogEntry, theme: &ThemeTokens) -> Div {
    let time = entry.edited_at.map_or_else(|| tr("data.history.log_unknown_time"), format_time);
    let note = if entry.note.is_empty() { tr("data.history.log_no_note") } else { entry.note.clone() };
    let note_colour = if entry.note.is_empty() { theme.text_muted } else { theme.text_primary };
    let number_colour = if entry.current { theme.accent_brand } else { theme.text_secondary };
    div()
        .flex()
        .items_start()
        .gap(theme.control_gap)
        .py(theme.spacing_2xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(cell(version_label(entry.version), Some(theme.data_count_width), number_colour, theme).font_family(theme.font_family_mono.clone()))
        .child(cell(time, Some(theme.data_label_width), theme.text_muted, theme).font_family(theme.font_family_mono.clone()))
        .child(cell(bump_text(entry), Some(theme.data_count_width), theme.text_secondary, theme))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(note_colour).child(note))
}

/// The log section, or `None` without entries.
pub(super) fn render_edit_log(log: &[LogEntry], theme: &ThemeTokens) -> Option<Div> {
    (!log.is_empty()).then(|| div().flex().flex_col().gap(theme.spacing_2xs).child(heading(tr("data.history.log"), theme)).children(log.iter().map(|entry| entry_row(entry, theme))))
}
