//! Localized text of the import sheet: granularities, identities, notes, file facts and unit
//! positions, plus the illustration of each format family.

use crate::domain::preferences::PostIdentity;
use crate::import::{FormatFamily, Granularity, ParseNote, UnitPosition};
use crate::ui::illustration::{ILLUSTRATION_EXT, ILLUSTRATION_ROOT};
use rust_i18n::t;

/// Folder of the format-family cards.
const ART_DIR: &str = "import";
const BYTES_PER_KB: f64 = 1024.0;
const BYTES_PER_MB: f64 = 1024.0 * 1024.0;

/// `illustrations/import/<family>.svg`.
pub fn family_art(family: FormatFamily) -> String {
    format!("{ILLUSTRATION_ROOT}/{ART_DIR}/{}.{ILLUSTRATION_EXT}", family.id())
}

pub fn family_label(family: FormatFamily) -> String {
    let key = format!("import.family.{}", family.id());
    t!(key.as_str()).to_string()
}

pub fn granularity_label(granularity: Granularity) -> String {
    let key = format!("import.split.{}", granularity.id());
    t!(key.as_str()).to_string()
}

pub fn identity_label(identity: PostIdentity) -> String {
    match identity {
        PostIdentity::Yourself => t!("import.post_as.yourself"),
        PostIdentity::Anonymous => t!("import.post_as.anonymous"),
        PostIdentity::Pseudonym => t!("import.post_as.pseudonym"),
    }
    .to_string()
}

/// `12 KB`, `3.4 MB`.
pub fn file_size(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes >= BYTES_PER_MB {
        t!("import.size_mb", size = format!("{:.1}", bytes / BYTES_PER_MB)).to_string()
    } else {
        t!("import.size_kb", size = format!("{:.0}", (bytes / BYTES_PER_KB).ceil())).to_string()
    }
}

/// One parser note as a sentence.
pub fn note_text(note: &ParseNote) -> String {
    match note {
        ParseNote::Dimensions { width, height } => t!("import.note.dimensions", width = width, height = height),
        ParseNote::ArchiveEntries(count) => t!("import.note.archive_entries", count = count),
        ParseNote::ExtractedStrings(count) => t!("import.note.extracted_strings", count = count),
        ParseNote::PagesWithoutText(count) => t!("import.note.pages_without_text", count = count),
        ParseNote::Unsupported(feature) => t!("import.note.unsupported", feature = feature),
        ParseNote::Encrypted => t!("import.note.encrypted"),
        ParseNote::Unreadable(reason) => t!("import.note.unreadable", reason = reason),
    }
    .to_string()
}

/// `Ch 3 · ¶ 12 · 2` (one-based), with the chapter title when there is one.
pub fn position_label(position: UnitPosition, chapter_title: Option<&str>) -> String {
    let chapter = match chapter_title {
        Some(title) => title.to_string(),
        None => t!("import.chapter_number", number = position.chapter + 1).to_string(),
    };
    match (position.paragraph, position.sentence) {
        (Some(paragraph), Some(sentence)) => t!("import.position.sentence", chapter = chapter, paragraph = paragraph + 1, sentence = sentence + 1),
        (Some(paragraph), None) => t!("import.position.paragraph", chapter = chapter, paragraph = paragraph + 1),
        _ => chapter.into(),
    }
    .to_string()
}
