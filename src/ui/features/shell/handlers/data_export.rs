//! The export dialog in the shell (sb F2): Settings › Data › Export and the palette open it;
//! every choice recomputes the preview; "Export…" asks where, then writes the zip off the UI
//! thread.

use super::super::view::AppView;
use crate::domain::data::export::{bundle_files, plan_export, ExportOptions, ExportScope, ExportSource, SETTINGS_ALLOW_LIST};
use crate::domain::preferences::ToastKind;
use crate::storage::data::write_bundle;
use crate::ui::features::data::{tr, DataEvent, ExportDialog, ScopeKind};
use chrono::{Duration, Local, NaiveDate, Utc};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

const DATE_FORMAT: &str = "%Y-%m-%d";
/// The date range a new dialog proposes, in days back from today.
const DEFAULT_RANGE_DAYS: i64 = 30;
const FILE_NAME_FORMAT: &str = "simulacra-export-%Y%m%d-%H%M%S.zip";
const ZIP_EXTENSIONS: [&str; 1] = ["zip"];

impl AppView {
    pub(in crate::ui::features::shell) fn on_export_event(&mut self, event: DataEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event == DataEvent::OpenExport {
            return self.open_export(window, cx);
        }
        let Some(dialog) = self.data.export.as_mut() else { return };
        match event {
            DataEvent::SetScope(kind) => dialog.scope = kind,
            DataEvent::PickCollection(id) => dialog.collection = Some(id),
            DataEvent::PickPersona(id) => dialog.persona = Some(id),
            DataEvent::ToggleOriginals => dialog.include_originals = !dialog.include_originals,
            DataEvent::ToggleJournal => dialog.include_journal = !dialog.include_journal,
            DataEvent::LastDays(days) => return self.fill_date_range(days, window, cx),
            DataEvent::RunExport => return self.run_export(window, cx),
            _ => self.data.export = None,
        }
        self.refresh_export_preview(cx);
    }

    fn open_export(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chrome.is_settings_open = false;
        let personas = self.state.user_profiles.iter().filter(|p| p.is_human_persona()).map(|p| (p.id.clone(), p.name.clone())).collect();
        let (from, to) = (self.data.inputs.from.clone(), self.data.inputs.to.clone());
        self.data.export = Some(ExportDialog::new(self.network_collections(), personas, from, to));
        self.fill_date_range(DEFAULT_RANGE_DAYS, window, cx);
    }

    fn fill_date_range(&mut self, days: i64, window: &mut Window, cx: &mut Context<Self>) {
        let today = Local::now().date_naive();
        let from = (today - Duration::days(days)).format(DATE_FORMAT).to_string();
        self.data.inputs.from.update(cx, |input, cx| input.set_value(from, window, cx));
        self.data.inputs.to.update(cx, |input, cx| input.set_value(today.format(DATE_FORMAT).to_string(), window, cx));
        self.refresh_export_preview(cx);
    }

    /// The dialog's choice as export options, or why it is incomplete.
    fn export_options(&self, dialog: &ExportDialog, cx: &App) -> Result<ExportOptions, String> {
        let date = |input: &Entity<gpui_kit::component::input::InputState>| NaiveDate::parse_from_str(input.read(cx).value().trim(), DATE_FORMAT).map_err(|_| tr("data.export.bad_date"));
        let scope = match dialog.scope {
            ScopeKind::Everything => ExportScope::Everything,
            ScopeKind::Collection => ExportScope::Collection(dialog.collection.clone().ok_or_else(|| tr("data.export.pick_collection"))?),
            ScopeKind::Persona => ExportScope::Persona(dialog.persona.clone().ok_or_else(|| tr("data.export.pick_persona"))?),
            ScopeKind::DateRange => {
                let (from, to) = (date(&dialog.from)?, date(&dialog.to)?);
                if from > to {
                    return Err(tr("data.export.dates_reversed"));
                }
                ExportScope::DateRange { from, to }
            }
        };
        Ok(ExportOptions { scope, include_originals: dialog.include_originals, include_journal: dialog.include_journal })
    }

    /// What an export may draw from, copied off the store.
    fn export_source(&self, include_journal: bool) -> ExportSource {
        let network = &self.state.active_network_id;
        let settings = self.state.db.as_ref().and_then(|db| db.setting_rows(SETTINGS_ALLOW_LIST).ok()).unwrap_or_default();
        let authors = self.state.user_profiles.iter().chain(self.state.archived_authors.values()).map(|p| (p.id.clone(), p.clone())).collect();
        ExportSource {
            network_name: self.state.current_network().map(|n| n.name.clone()).unwrap_or_default(),
            posts: self.state.posts().iter().filter(|p| &p.network_id == network).cloned().collect(),
            authors,
            profiles: self.state.user_profiles.iter().filter(|p| p.network_id.is_none() || p.network_id.as_ref() == Some(network)).cloned().collect(),
            documents: self.state.documents.iter().filter(|d| &d.network_id == network).cloned().collect(),
            library: self.state.library.clone(),
            review: self.review.deck.items.iter().filter(|i| &i.network_id == network).cloned().collect(),
            entries: self.data.shelf.entries.clone(),
            settings,
            journal: include_journal.then(|| self.journal.book.clone()),
            media_dir: self.state.media_dir(),
            exported_at: Utc::now(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Recomputes the dialog's preview.
    pub(in crate::ui::features::shell) fn refresh_export_preview(&mut self, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.export.clone() else { return };
        let (counts, problem) = match self.export_options(&dialog, cx) {
            Ok(options) => (Some(plan_export(&self.export_source(options.include_journal), &options).counts), None),
            Err(problem) => (None, Some(problem)),
        };
        if let Some(open) = self.data.export.as_mut() {
            (open.counts, open.problem) = (counts, problem);
        }
    }

    /// Asks where to save, then writes the bundle in the background.
    fn run_export(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(options) = self.data.export.as_ref().and_then(|dialog| self.export_options(dialog, cx).ok()) else { return };
        let source = self.export_source(options.include_journal);
        let name = Local::now().format(FILE_NAME_FORMAT).to_string();
        let filter = tr("data.export.file_filter");
        cx.spawn_in(window, async move |view, cx| {
            let Some(file) = rfd::AsyncFileDialog::new().set_file_name(name).add_filter(filter, &ZIP_EXTENSIONS).save_file().await else { return };
            let path = file.path().to_path_buf();
            let _ = view.update(cx, |this, cx| this.set_export_busy(true, cx));
            let target = path.clone();
            let written = cx.background_executor().spawn(async move { write_bundle(&target, &bundle_files(&plan_export(&source, &options), &source)) }).await;
            let _ = view.update_in(cx, |this, window, cx| this.export_finished(written.map(|report| (path, report.files)), window, cx));
        })
        .detach();
    }

    fn set_export_busy(&mut self, busy: bool, cx: &mut Context<Self>) {
        if let Some(dialog) = self.data.export.as_mut() {
            dialog.busy = busy;
        }
        cx.notify();
    }

    fn export_finished(&mut self, written: crate::error::Result<(std::path::PathBuf, usize)>, window: &mut Window, cx: &mut Context<Self>) {
        self.set_export_busy(false, cx);
        match written {
            Ok((path, files)) => {
                self.data.export = None;
                let note = Notification::success(t!("data.export.done", count = files, path = path.display().to_string()).to_string());
                self.toast(ToastKind::Confirmation, note, window, cx);
            }
            Err(err) => window.push_notification(Notification::error(t!("data.export.failed", reason = err.to_string()).to_string()), cx),
        }
    }
}
