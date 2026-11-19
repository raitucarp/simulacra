//! The export dialog's sections: scope, what to include, and the preview.

use super::super::events::{DataAction, DataEvent, ScopeKind};
use super::super::sheet::{heading, hint, notice, tick_row};
use super::super::text::tr;
use super::ExportDialog;
use crate::domain::data::export::Counts;
use crate::theme::ThemeTokens;
use crate::ui::components::{chip_row, choice_chip, segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use std::rc::Rc;

const SCOPE_ID_PREFIX: &str = "data-export-scope-";
const PICK_ID_PREFIX: &str = "data-export-pick-";
const DAYS_ID_PREFIX: &str = "data-export-days-";
const ORIGINALS_ID: &str = "data-export-originals";
const JOURNAL_ID: &str = "data-export-journal";
/// Quick date ranges, in days back from today.
const DAY_PRESETS: [i64; 4] = [7, 30, 90, 365];

fn send(on_event: &DataAction, event: DataEvent) -> impl Fn(&mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |window, cx| on_event(event.clone(), window, cx)
}

const fn scope_icon(kind: ScopeKind) -> LucideIcon {
    match kind {
        ScopeKind::Everything => LucideIcon::Boxes,
        ScopeKind::Collection => LucideIcon::Folder,
        ScopeKind::DateRange => LucideIcon::Calendar,
        ScopeKind::Persona => LucideIcon::User,
    }
}

fn scope_detail(dialog: &ExportDialog, on_event: &DataAction, theme: &ThemeTokens) -> Option<Div> {
    let chips = |items: Vec<(String, String, bool, DataEvent)>| {
        let row = chip_row(theme).children(items.into_iter().map(|(id, label, on, event)| choice_chip(SharedString::from(format!("{PICK_ID_PREFIX}{id}")), &label, on, theme, send(on_event, event))));
        div().id(PICK_ID_PREFIX).max_h(theme.export_pick_max_height).overflow_y_scroll().child(row)
    };
    match dialog.scope {
        ScopeKind::Everything => None,
        ScopeKind::Collection if dialog.collections.is_empty() => Some(hint(tr("data.export.no_collections"), theme)),
        ScopeKind::Collection => Some(div().child(chips(dialog.collections.iter().map(|(id, name)| (id.to_string(), name.clone(), dialog.collection.as_ref() == Some(id), DataEvent::PickCollection(id.clone()))).collect()))),
        ScopeKind::Persona => Some(div().child(chips(dialog.personas.iter().map(|(id, name)| (id.to_string(), name.clone(), dialog.persona.as_ref() == Some(id), DataEvent::PickPersona(id.clone()))).collect()))),
        ScopeKind::DateRange => Some(date_range(dialog, on_event, theme)),
    }
}

fn date_range(dialog: &ExportDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let field = |label: String, input: &Entity<InputState>| div().flex().items_center().gap(theme.inline_gap).child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(label)).child(div().w(theme.data_date_field_width).child(Input::new(input).small()));
    let presets = DAY_PRESETS.map(|days| choice_chip(SharedString::from(format!("{DAYS_ID_PREFIX}{days}")), &rust_i18n::t!("data.export.last_days", count = days).to_string(), false, theme, send(on_event, DataEvent::LastDays(days))));
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(div().flex().flex_wrap().gap(theme.control_gap).child(field(tr("data.export.from"), &dialog.from)).child(field(tr("data.export.to"), &dialog.to)))
        .child(chip_row(theme).children(presets))
}

pub(super) fn scope_section(dialog: &ExportDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let pills = ScopeKind::ALL.into_iter().map(|kind| {
        let segment = Segment { id: SharedString::from(format!("{SCOPE_ID_PREFIX}{}", kind.key())).into(), icon: scope_icon(kind), label: tr(&format!("data.export.scope.{}", kind.key())), active: dialog.scope == kind, active_icon_color: theme.accent_brand };
        segment_pill(segment, theme, send(on_event, DataEvent::SetScope(kind)))
    });
    div().flex().flex_col().gap(theme.stack_gap).child(heading(tr("data.export.scope_title"), theme)).child(segmented_track(theme).children(pills)).children(scope_detail(dialog, on_event, theme))
}

pub(super) fn include_section(dialog: &ExportDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let originals = tick_row(ORIGINALS_ID, tr("data.export.include_originals"), dialog.include_originals, Rc::new(send(on_event, DataEvent::ToggleOriginals)), theme);
    let journal = tick_row(JOURNAL_ID, tr("data.export.include_journal"), dialog.include_journal, Rc::new(send(on_event, DataEvent::ToggleJournal)), theme);
    let warning = dialog.include_journal.then(|| notice(LucideIcon::LockKeyhole, tr("data.export.journal_warning"), theme.warning, theme));
    div().flex().flex_col().gap(theme.stack_gap).child(heading(tr("data.export.include_title"), theme)).child(originals).child(journal).children(warning)
}

fn count_rows(counts: &Counts) -> [(&'static str, usize); 9] {
    [
        ("threads", counts.threads), ("posts", counts.posts), ("documents", counts.documents), ("originals", counts.originals), ("images", counts.images),
        ("review_items", counts.review_items), ("agents", counts.agents), ("settings", counts.settings), ("journal_records", counts.journal_records),
    ]
}

pub(super) fn preview_section(dialog: &ExportDialog, theme: &ThemeTokens) -> Div {
    let head = heading(tr("data.export.preview_title"), theme);
    let Some(counts) = dialog.counts else {
        return div().flex().flex_col().gap(theme.stack_gap).child(head).child(hint(dialog.problem.clone().unwrap_or_default(), theme));
    };
    let rows = count_rows(&counts).into_iter().map(|(key, count)| {
        let colour = if count == 0 { theme.text_muted } else { theme.text_primary };
        div().flex().items_center().gap(theme.inline_gap).py(theme.spacing_3xs).border_b(theme.border_width_hairline).border_color(theme.border_subtle)
            .child(div().w(theme.data_count_width).flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(colour).child(count.to_string()))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(colour).child(tr(&format!("data.export.count.{key}"))))
    });
    let never = notice(LucideIcon::ShieldCheck, tr("data.export.never_included"), theme.accent_success, theme);
    div().flex().flex_col().gap(theme.stack_gap).child(head).child(div().flex().flex_col().children(rows)).child(never)
}
