//! The import sheet's model (no UI types): the file being read, its parsed document, the
//! units at the chosen granularity, which of them are selected and who posts them. Every
//! change keeps the dry-run counts current.

use crate::domain::preferences::PostIdentity;
use crate::import::plan::{dry_run, plan_posts, DryRun, SourceInfo};
use crate::import::{segment_document, Granularity, LoadedFile, Unit};
use std::sync::Arc;

/// Where the sheet is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportStage {
    /// Reading and parsing in the background.
    Reading,
    Ready,
    /// The file could not be read at all (too large, missing).
    Failed(String),
}

/// The open import sheet.
#[derive(Debug, Clone)]
pub struct ImportSession {
    pub file_name: String,
    pub stage: ImportStage,
    pub loaded: Option<LoadedFile>,
    pub granularity: Granularity,
    pub units: Arc<Vec<Unit>>,
    pub selected: Vec<bool>,
    pub identity: PostIdentity,
    pub post_limit: usize,
    pub counts: DryRun,
}

impl ImportSession {
    /// A sheet waiting for `file_name` to be read.
    pub fn reading(file_name: impl Into<String>, granularity: Granularity, identity: PostIdentity, post_limit: usize) -> Self {
        Self {
            file_name: file_name.into(),
            stage: ImportStage::Reading,
            loaded: None,
            granularity,
            units: Arc::default(),
            selected: Vec::new(),
            identity,
            post_limit,
            counts: DryRun::default(),
        }
    }

    /// The file was read: segment it and select every unit.
    pub fn load(&mut self, loaded: LoadedFile) {
        self.loaded = Some(loaded);
        self.stage = ImportStage::Ready;
        self.set_granularity(self.granularity);
    }

    pub fn fail(&mut self, reason: String) {
        self.stage = ImportStage::Failed(reason);
    }

    /// Re-segments at `granularity`, selecting every unit.
    pub fn set_granularity(&mut self, granularity: Granularity) {
        self.granularity = granularity;
        self.units = Arc::new(self.loaded.as_ref().map(|file| segment_document(&file.doc, granularity)).unwrap_or_default());
        self.selected = vec![true; self.units.len()];
        self.refresh();
    }

    pub fn toggle(&mut self, index: usize) {
        if let Some(flag) = self.selected.get_mut(index) {
            *flag = !*flag;
            self.refresh();
        }
    }

    pub fn select_all(&mut self, on: bool) {
        self.selected.iter_mut().for_each(|flag| *flag = on);
        self.refresh();
    }

    /// Selects exactly the units of `chapter`.
    pub fn select_chapter(&mut self, chapter: usize) {
        self.selected = self.units.iter().map(|unit| unit.position.chapter == chapter).collect();
        self.refresh();
    }

    /// The selected units, in reading order.
    pub fn selected_units(&self) -> Vec<Unit> {
        self.units.iter().zip(&self.selected).filter(|(_, on)| **on).map(|(unit, _)| unit.clone()).collect()
    }

    pub fn selected_count(&self) -> usize {
        self.selected.iter().filter(|on| **on).count()
    }

    /// Title of each chapter (section) of the document.
    pub fn chapter_titles(&self) -> Vec<Option<String>> {
        self.loaded.as_ref().map(|file| file.doc.sections.iter().map(|s| s.title.clone()).collect()).unwrap_or_default()
    }

    /// Indexes of the chapters that hold at least one unit.
    pub fn chapters_with_units(&self) -> Vec<usize> {
        let mut chapters: Vec<usize> = self.units.iter().map(|unit| unit.position.chapter).collect();
        chapters.dedup();
        chapters
    }

    fn refresh(&mut self) {
        let source = SourceInfo { document_id: String::new(), title: String::new(), chapter_titles: Vec::new() };
        let selected = self.units.iter().zip(&self.selected).filter(|(_, on)| **on).map(|(unit, _)| unit);
        self.counts = dry_run(&plan_posts(selected, &source, self.post_limit));
    }
}
