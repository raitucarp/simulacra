//! Markdown of threads and vault records. Front matter values are written as JSON scalars
//! and lists, which every YAML reader accepts; post text is kept verbatim, so `[[wikilinks]]`,
//! `#tags` and the post marks (bold, italic, underline, code, spoilers, fenced blocks) survive.

use super::{ExportSource, ThreadExport};
use crate::domain::document::Library;
use crate::domain::{GroundingDocument, Post};
use serde_json::{json, Value};

const FENCE: &str = "---";
/// Characters of a thread's first post naming the thread.
const TITLE_CHARS: usize = 72;
/// Characters of a file-name slug.
const SLUG_CHARS: usize = 48;
const SLUG_FALLBACK: &str = "untitled";
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M UTC";

fn front_matter(fields: Vec<(&str, Value)>) -> String {
    let lines: String = fields.into_iter().filter(|(_, v)| !v.is_null() && v.as_str() != Some("") && v.as_array().is_none_or(|a| !a.is_empty())).map(|(key, value)| format!("{key}: {value}\n")).collect();
    format!("{FENCE}\n{lines}{FENCE}\n\n")
}

/// The first line of `text`, cut to a title.
pub fn thread_title(text: &str) -> String {
    let line = text.lines().map(str::trim).find(|l| !l.is_empty()).unwrap_or_default();
    match line.char_indices().nth(TITLE_CHARS) {
        Some((at, _)) => format!("{}…", line[..at].trim_end()),
        None => line.to_string(),
    }
}

/// A lower-case ASCII file-name slug of `text`.
pub fn slug(text: &str) -> String {
    let words: Vec<String> = text.split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_ascii_lowercase).collect();
    let joined = words.join("-");
    let cut: String = joined.chars().take(SLUG_CHARS).collect();
    let cut = cut.trim_end_matches('-').to_string();
    if cut.is_empty() { SLUG_FALLBACK.to_string() } else { cut }
}

fn author_line(post: &Post, source: &ExportSource) -> String {
    let (name, handle) = source.authors.get(&post.author_id).map_or_else(|| (post.author_id.to_string(), String::new()), |p| (p.name.clone(), p.handle.clone()));
    let who = if handle.is_empty() { name } else { format!("{name} ({handle})") };
    format!("### {who} · {}\n\n", post.created_at.strftime(TIME_FORMAT))
}

/// N7, N10: a post's place in a series and its pinned quote, as Markdown; nothing for a locked
/// post, which travels as its seal only.
fn idea_block(post: &Post) -> String {
    if post.is_locked() {
        return String::new();
    }
    let series = post.extras.series.as_ref().map(|entry| format!("*{} #{}*\n\n", entry.title, entry.number)).unwrap_or_default();
    let pinned = post.extras.pinned.as_ref().map(|quote| format!("> {}\n>\n> — {}\n\n", quote.passage.replace('\n', "\n> "), quote.title)).unwrap_or_default();
    format!("{series}{pinned}")
}

fn post_block(post: &Post, source: &ExportSource, image_path: &dyn Fn(&str) -> String) -> String {
    let images: String = super::bundle::post_images(post).iter().map(|(path, alt)| format!("![{alt}]({})\n\n", image_path(path))).collect();
    let link = post.link_url.as_ref().map(|url| format!("<{url}>\n\n")).unwrap_or_default();
    let quote = post.quote_id.as_ref().map(|id| format!("> Quotes [[{id}]]\n\n")).unwrap_or_default();
    let body = post.extras.lock.as_ref().map_or_else(|| post.content.trim().to_string(), crate::domain::post::lock::lock_block); // X6: a locked post travels as its seal only
    format!("{}{body}\n\n{}{}{}{images}{link}{quote}", author_line(post, source), idea_block(post), super::care::care_block(post, image_path), super::thoughts::thought_block(post)) // N16–N20
}

/// `items` without repeats, in first-seen order.
fn unique(items: impl Iterator<Item = String>) -> Vec<String> {
    items.fold(Vec::new(), |mut all, item| {
        if !all.contains(&item) {
            all.push(item);
        }
        all
    })
}

/// One thread as a Markdown file; `image_path` names where each image is in the bundle.
pub fn thread_markdown(thread: &ThreadExport, source: &ExportSource, image_path: &dyn Fn(&str) -> String) -> String {
    let first = &thread.posts[0];
    let last = thread.posts.last().unwrap_or(first);
    let authors = unique(thread.posts.iter().filter_map(|p| source.authors.get(&p.author_id).map(|a| a.handle.clone())));
    let tags = unique(thread.posts.iter().flat_map(|p| Post::parse_hashtags(&p.content)));
    let title = thread_title(&first.content);
    let head = front_matter(vec![
        ("title", json!(title)), ("thread", json!(thread.root.as_str())), ("network", json!(source.network_name)),
        ("created", json!(crate::time::rfc3339(first.created_at))), ("updated", json!(crate::time::rfc3339(last.created_at))),
        ("posts", json!(thread.posts.len())), ("authors", json!(authors)), ("tags", json!(tags)),
    ]);
    let body: Vec<String> = thread.posts.iter().map(|p| post_block(p, source, image_path)).collect();
    format!("{head}# {title}\n\n{}", body.join(&format!("{FENCE}\n\n")))
}

/// A vault document as Markdown: its record as front matter, its abstract, then its text.
pub fn document_markdown(doc: &GroundingDocument, library: &Library, original: Option<&str>) -> String {
    let record = &doc.record;
    let creators: Vec<String> = record.creators.iter().map(|c| c.name.clone()).collect();
    let subjects: Vec<String> = record.subjects.iter().filter_map(|id| library.subject(id).map(|s| s.label.clone())).collect();
    let collections: Vec<String> = record.collections.iter().filter_map(|id| library.collection(id).map(|c| c.name.clone())).collect();
    let head = front_matter(vec![
        ("title", json!(doc.title)), ("subtitle", json!(record.subtitle)), ("kind", json!(record.kind)),
        ("creators", json!(creators)), ("date", json!(record.date)), ("publisher", json!(record.publisher)),
        ("language", json!(record.language)), ("identifier", json!(record.identifier.value)), ("subjects", json!(subjects)),
        ("collections", json!(collections)), ("added", json!(crate::time::rfc3339(doc.created_at))), ("source", json!(record.provenance.source)),
        ("rights", json!(record.rights)), ("original", json!(original.unwrap_or_default())),
    ]);
    let summary = if record.description.trim().is_empty() { String::new() } else { format!("> {}\n\n", record.description.trim().replace('\n', "\n> ")) };
    format!("{head}# {}\n\n{summary}{}\n", doc.title, doc.content.trim())
}
