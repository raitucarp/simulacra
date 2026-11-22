//! L13: what an agent learns about the files attached to a post: their names and facts, never
//! their contents. Built from the post's own references alone, so no file is opened.

use super::{file_family, FileRef};

/// Bytes per kilobyte and megabyte in the facts line.
const KB: f64 = 1024.0;
const MB: f64 = 1024.0 * 1024.0;
/// How a file's date is written.
const DATE_FORMAT: &str = "%Y-%m-%d";
/// The sentence that opens the list (agents read English prompts).
const LEAD: &str = "Attached files (names and facts only; their contents are not shared with you):";

/// The note an agent reads about `files`; empty when there are none.
pub fn files_note(files: &[FileRef]) -> String {
    if files.is_empty() {
        return String::new();
    }
    let lines: Vec<String> = files.iter().map(file_line).collect();
    format!("{LEAD}\n{}", lines.join("\n"))
}

/// `content` followed by the note about `files`, as an agent reads a post.
pub fn with_files_note(content: &str, files: &[FileRef]) -> String {
    match files_note(files) {
        note if note.is_empty() => content.to_string(),
        note => format!("{content}\n\n{note}"),
    }
}

/// `- report.pdf (pdf, application/pdf, 1.2 MB, modified 2026-11-02)`.
fn file_line(file: &FileRef) -> String {
    let mut facts = vec![file_family(&file.mime).id().to_string()];
    facts.extend((!file.mime.is_empty()).then(|| file.mime.clone()));
    facts.push(size_label(file.size));
    facts.extend(file.modified.map(|at| format!("modified {}", at.strftime(DATE_FORMAT))));
    format!("- {} ({})", file.name(), facts.join(", "))
}

fn size_label(bytes: u64) -> String {
    let bytes = bytes as f64;
    if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else {
        format!("{:.0} KB", (bytes / KB).ceil())
    }
}
