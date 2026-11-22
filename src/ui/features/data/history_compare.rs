//! The history sheet's comparison: the word diff from the chosen version to the current one.

use super::diff_view::{render_diff, render_stats};
use super::history::HistorySheet;
use super::sheet::{heading, hint};
use super::text::tr;
use crate::domain::data::history::word_diff;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

/// The diff, or a hint while the current version is chosen.
pub(super) fn comparison(sheet: &HistorySheet, theme: &ThemeTokens) -> Div {
    let current = sheet.current();
    let chosen = &sheet.versions[sheet.selected.min(sheet.versions.len() - 1)];
    if sheet.chose_current() {
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
