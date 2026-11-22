//! The history sheet's comparison (V6): the word diff between any two versions, A (picked
//! by clicking a row) and B (its "B" button; the current version until another is picked).

use super::diff_view::{render_diff, render_stats};
use super::history::HistorySheet;
use super::sheet::{heading, hint};
use super::text::tr;
use crate::domain::data::history::{record_changes, RevisionSubject, word_diff, Version, VersionKind};
use crate::domain::post::Tint;
use crate::ui::features::composer::tint_label_key;
use crate::theme::ThemeTokens;
use crate::ui::features::composer::version_label;
use gpui_kit::*;
use rust_i18n::t;

/// How a side of the comparison is named: its number, else what it is.
pub(super) fn side_name(version: &Version) -> String {
    match (version.number, version.kind) {
        (Some(number), _) => version_label(number),
        (None, VersionKind::Current) => tr("data.history.current"),
        (None, VersionKind::Original) => tr("data.history.original"),
        (None, VersionKind::Earlier) => tr("data.history.earlier"),
    }
}

/// The diff from A to B, or a hint while they are the same version.
pub(super) fn comparison(sheet: &HistorySheet, theme: &ThemeTokens) -> Div {
    let last = sheet.versions.len() - 1;
    let (a, b) = (&sheet.versions[sheet.selected.min(last)], &sheet.versions[sheet.compare.min(last)]);
    let Some(ops) = sheet.diff() else {
        let note = if sheet.versions.len() > 1 { tr("data.history.same_version") } else { tr("data.history.no_versions") };
        return div().flex().flex_col().gap(theme.stack_gap).child(hint(note, theme)).child(render_diff(&word_diff(&a.content, &a.content), theme));
    };
    let title_change = (a.title != b.title && !a.title.is_empty() && !b.title.is_empty()).then(|| hint(t!("data.history.title_changed", before = a.title.as_str(), after = b.title.as_str()).to_string(), theme));
    let title = t!("data.history.changes_between", from = side_name(a), to = side_name(b)).to_string();
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(div().flex().flex_wrap().items_center().justify_between().gap(theme.control_gap).child(heading(title, theme)).child(render_stats(&ops, theme)))
        .children(title_change)
        .children(look_changes(sheet, (a, b), theme))
        .child(render_diff(&ops, theme))
}

fn tint_name(tint: Option<Tint>) -> String {
    t!(tint_label_key(tint)).to_string()
}

/// What changed beside the text of a post from A to B: its tint and its symbols (V10).
fn look_changes(sheet: &HistorySheet, (a, b): (&Version, &Version), theme: &ThemeTokens) -> Vec<Div> {
    if !matches!(sheet.subject, RevisionSubject::Post(_)) {
        return Vec::new();
    }
    let changes = record_changes(&a.record_json, &b.record_json);
    let tint = changes.tint.map(|(before, after)| hint(t!("data.history.tint_changed", before = tint_name(before), after = tint_name(after)).to_string(), theme));
    let symbols = changes.symbols.then(|| hint(tr("data.history.symbols_changed"), theme));
    let attachments = changes.attachments.then(|| hint(tr("data.history.attachments_changed"), theme)); // V11
    tint.into_iter().chain(symbols).chain(attachments).collect()
}
