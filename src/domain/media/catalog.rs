//! The in-memory media cache: link previews by URL, image facts by file path, and what is
//! being fetched or scanned right now. Every change returns the next catalog and bumps its
//! revision, so the shell can tell cheaply whether cards need re-measuring.

use super::image::ImageFacts;
use super::preview::PreviewEntry;
use std::collections::{HashMap, HashSet};

/// The outcome of scanning an image file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageScan {
    Known(ImageFacts),
    /// Missing or not a decodable image (cards show the missing-image art).
    Unreadable,
}

/// What a card can show for a URL right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewLookup<'a> {
    /// A preview, possibly stale (it shows while a refresh runs).
    Ready(&'a PreviewEntry),
    /// Being fetched for the first time.
    Loading,
    /// Never fetched, failed, or empty: the card falls back to a plain link.
    Unavailable,
}

/// Link previews and image facts, with the work in flight.
#[derive(Debug, Clone, Default)]
pub struct MediaCatalog {
    previews: HashMap<String, PreviewEntry>,
    images: HashMap<String, ImageScan>,
    fetching: HashSet<String>,
    scanning: HashSet<String>,
    revision: u64,
}

impl MediaCatalog {
    /// A catalog of stored previews and image scans.
    pub fn from_rows(previews: Vec<PreviewEntry>, images: Vec<(String, ImageScan)>) -> Self {
        let previews = previews.into_iter().map(|entry| (entry.url.clone(), entry)).collect();
        Self { previews, images: images.into_iter().collect(), ..Self::default() }
    }

    /// Bumped on every stored result.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn preview(&self, url: &str) -> PreviewLookup<'_> {
        match self.previews.get(url) {
            Some(entry) if entry.is_ready() => PreviewLookup::Ready(entry),
            _ if self.fetching.contains(url) => PreviewLookup::Loading,
            _ => PreviewLookup::Unavailable,
        }
    }

    /// The scan of the image at `path`, once done.
    pub fn image(&self, path: &str) -> Option<&ImageScan> {
        self.images.get(path)
    }

    /// Up to `limit` of `urls` that are not cached or have gone stale at `now`, and are not
    /// being fetched already, without duplicates.
    pub fn previews_due<'a>(&self, urls: impl IntoIterator<Item = &'a str>, now: i64, limit: usize) -> Vec<String> {
        let due = urls.into_iter().filter(|url| !self.fetching.contains(*url) && self.previews.get(*url).is_none_or(|entry| !entry.is_fresh(now)));
        unique(due, limit)
    }

    /// Up to `limit` of `paths` never scanned and not being scanned, without duplicates.
    pub fn images_due<'a>(&self, paths: impl IntoIterator<Item = &'a str>, limit: usize) -> Vec<String> {
        let due = paths.into_iter().filter(|path| !self.scanning.contains(*path) && !self.images.contains_key(*path));
        unique(due, limit)
    }

    /// How many fetches and scans are running.
    pub fn in_flight(&self) -> (usize, usize) {
        (self.fetching.len(), self.scanning.len())
    }

    /// The catalog with `urls` marked as being fetched and `paths` as being scanned.
    #[must_use]
    pub fn started(mut self, urls: &[String], paths: &[String]) -> Self {
        self.fetching.extend(urls.iter().cloned());
        self.scanning.extend(paths.iter().cloned());
        self
    }

    /// The catalog with a finished fetch stored.
    #[must_use]
    pub fn with_preview(mut self, entry: PreviewEntry) -> Self {
        self.fetching.remove(&entry.url);
        self.previews.insert(entry.url.clone(), entry);
        self.revision += 1;
        self
    }

    /// The catalog with a finished scan of `path` stored.
    #[must_use]
    pub fn with_image(mut self, path: String, scan: ImageScan) -> Self {
        self.scanning.remove(&path);
        self.images.insert(path, scan);
        self.revision += 1;
        self
    }

    /// The catalog with every cached result forgotten (work in flight still lands).
    #[must_use]
    pub fn cleared(self) -> Self {
        Self { fetching: self.fetching, scanning: self.scanning, revision: self.revision + 1, ..Self::default() }
    }
}

/// The first `limit` distinct items of `items`, owned.
fn unique<'a>(items: impl Iterator<Item = &'a str>, limit: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for item in items {
        if out.len() == limit {
            break;
        }
        if !out.iter().any(|seen| seen == item) {
            out.push(item.to_string());
        }
    }
    out
}
