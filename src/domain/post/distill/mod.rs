//! N15: distilling a thread into a vault document. The thread, in reading order, becomes
//! Markdown: a title, a line saying where it came from, an optional summary, then each post
//! under its author and date with its text and a link back to it (`simulacra://…/post/<id>`).
//! A locked post keeps its place with only the placeholder; a retracted or corrected one
//! says so. The thread's root remembers the document ([`Distilled`], `extras.distilled`).

use super::correction::Correction;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// The scheme and segment of a post's permalink (as "Copy link" writes it).
pub const PERMALINK_SCHEME: &str = "simulacra://";
pub const PERMALINK_POST_SEGMENT: &str = "/post/";
/// The longest document title, in characters.
pub const MAX_TITLE_CHARS: usize = 80;

/// The vault document a thread was distilled into (`extras.distilled` on its root, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Distilled {
    pub doc_id: String,
    pub title: String,
}

/// One post of the thread as it goes into the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistillPost {
    pub link: String,
    pub author: String,
    /// Its date as the document shows it.
    pub date: String,
    /// Its text (the placeholder when locked).
    pub text: String,
    pub correction: Option<Correction>,
}

/// The words the document is written with (translated by the caller).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DistillWords {
    /// "Distilled from a thread of 4 posts."
    pub origin: String,
    pub summary_heading: String,
    pub open_post: String,
    pub retracted: String,
    pub corrected: String,
    /// The title when the root has no words of its own to give one.
    pub untitled: String,
    /// What a locked post shows, and a post kept from some readers (N8: the vault is read by every agent).
    pub locked: String,
    pub restricted: String,
}

/// The permalink of post `id` in `network`.
pub fn post_link(network: &str, id: &str) -> String {
    format!("{PERMALINK_SCHEME}{network}{PERMALINK_POST_SEGMENT}{id}")
}

/// A document title from the root's text: its first line, cut at a word.
pub fn title_from(text: &str) -> String {
    let line = text.lines().map(str::trim).find(|line| !line.is_empty()).unwrap_or_default();
    if line.chars().count() <= MAX_TITLE_CHARS {
        return line.to_string();
    }
    let cut: String = line.chars().take(MAX_TITLE_CHARS).collect();
    format!("{}…", cut.rsplit_once(' ').map_or(cut.as_str(), |(head, _)| head).trim_end())
}

fn write_post(out: &mut String, post: &DistillPost, words: &DistillWords) {
    let _ = writeln!(out, "## {} · {}\n", post.author, post.date);
    if let Some(correction) = &post.correction {
        let label = match correction.kind {
            super::correction::CorrectionKind::Retracted => &words.retracted,
            super::correction::CorrectionKind::Corrected => &words.corrected,
        };
        let _ = writeln!(out, "> **{label}:** {}\n", correction.reason);
    }
    let _ = writeln!(out, "{}\n", post.text.trim());
    let _ = writeln!(out, "[{}]({})\n", words.open_post, post.link);
}

/// The thread as a Markdown document.
pub fn thread_markdown(title: &str, posts: &[DistillPost], summary: Option<&str>, words: &DistillWords) -> String {
    let mut out = format!("# {}\n\n*{}*\n\n", title.trim(), words.origin);
    if let Some(summary) = summary.map(str::trim).filter(|summary| !summary.is_empty()) {
        let _ = writeln!(out, "## {}\n\n{summary}\n", words.summary_heading);
    }
    for post in posts {
        write_post(&mut out, post, words);
    }
    out.trim_end().to_string() + "\n"
}

#[cfg(test)]
mod tests;
