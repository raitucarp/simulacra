//! Opening files for import: the picker, drops, pasted paths and command-line files are
//! queued, each read and parsed on the background executor while the sheet shows progress.

use super::super::import_state::ImportUi;
use super::super::view::AppView;
use crate::import::apply::max_bytes;
use crate::import::plan::effective_post_limit;
use crate::import::{load_file, LoadError, LoadedFile};
use crate::ui::features::import::{file_size, ImportSession};
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;
use std::time::Instant;

/// Set to print how long each import stage took (`simulacra: import ...`) to stderr.
pub(super) const TIMING_ENV: &str = "SIMULACRA_IMPORT_TIMING";
/// Separator between the creators typed into one input.
pub(super) const CREATOR_SEPARATOR: &str = ", ";

/// Prints a stage timing when [`TIMING_ENV`] is set.
pub(super) fn report_timing(stage: &str, started: Instant) {
    if std::env::var_os(TIMING_ENV).is_some() {
        eprintln!("simulacra: import {stage} took {} ms", started.elapsed().as_millis());
    }
}

impl AppView {
    /// Opens the native picker for any file type and imports the picked files.
    pub(crate) fn pick_import_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let title = t!("import.picker_title").to_string();
        cx.spawn_in(window, async move |entity, cx| {
            let Some(files) = rfd::AsyncFileDialog::new().set_title(&title).pick_files().await else { return };
            let paths: Vec<PathBuf> = files.iter().map(|file| file.path().to_path_buf()).collect();
            let _ = entity.update_in(cx, |this, window, cx| this.import_paths(paths, window, cx));
        })
        .detach();
    }

    /// Queues `paths` and opens the first one unless a sheet is already open.
    pub(crate) fn import_paths(&mut self, paths: Vec<PathBuf>, window: &mut Window, cx: &mut Context<Self>) {
        self.import.dragging = false;
        self.import.queue.extend(paths);
        if self.import.session.is_none() {
            self.open_next_import(window, cx);
        }
        cx.notify();
    }

    /// Opens the sheet for the next queued file and reads it in the background.
    fn open_next_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.import.queue.pop_front() else { return };
        let prefs = self.state.preferences();
        let name = path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
        let limit = effective_post_limit(prefs.writing.post_length_limit);
        self.import.session = Some(ImportSession::reading(name, prefs.import.segmentation.into(), prefs.import.identity, limit));
        let max = max_bytes(prefs.import.max_file_mb);
        let task = cx.spawn_in(window, async move |entity, cx| {
            let started = Instant::now();
            let loaded = cx.background_executor().spawn(async move { load_file(&path, max) }).await;
            report_timing("read and parse", started);
            let _ = entity.update_in(cx, |this, window, cx| this.finish_loading(loaded, window, cx));
        });
        self.import.task = Some(task);
    }

    fn finish_loading(&mut self, loaded: Result<LoadedFile, LoadError>, window: &mut Window, cx: &mut Context<Self>) {
        self.import.task = None;
        if let Ok(file) = &loaded {
            self.fill_import_inputs(file, window, cx);
        }
        let started = Instant::now();
        if let Some(session) = self.import.session.as_mut() {
            match loaded {
                Ok(file) => session.load(file),
                Err(LoadError::TooLarge { size, max }) => session.fail(t!("import.too_large", size = file_size(size), max = file_size(max)).to_string()),
                Err(LoadError::Unreadable(reason)) => session.fail(reason),
            }
        }
        report_timing("segment", started);
        cx.notify();
    }

    /// Fills the metadata inputs from the parsed file.
    fn fill_import_inputs(&self, file: &LoadedFile, window: &mut Window, cx: &mut Context<Self>) {
        let meta = &file.doc.metadata;
        let inputs = &self.import.inputs;
        let values = [
            (&inputs.title, meta.title.clone().unwrap_or_default()),
            (&inputs.creators, meta.creators.join(CREATOR_SEPARATOR)),
            (&inputs.language, meta.language.clone().unwrap_or_default()),
            (&inputs.date, meta.date.clone().unwrap_or_default()),
        ];
        for (input, value) in values {
            input.update(cx, |state, cx| state.set_value(value, window, cx));
        }
    }

    /// Closes the sheet (a read in progress is cancelled) and opens the next queued file.
    pub(crate) fn close_import(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.import.task = None;
        self.import.session = None;
        self.open_next_import(window, cx);
        cx.notify();
    }

    /// Files named on the command line (`simulacra book.epub`) open in the import sheet.
    pub(crate) fn import_command_line_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths: Vec<PathBuf> = std::env::args_os().skip(1).map(PathBuf::from).filter(|path| path.is_file()).collect();
        if !paths.is_empty() {
            self.import_paths(paths, window, cx);
        }
    }
}

impl ImportUi {
    /// Whether the pseudonym input holds a name.
    pub(crate) fn has_pseudonym(&self, cx: &App) -> bool {
        !Self::text(&self.inputs.pseudonym, cx).is_empty()
    }
}
