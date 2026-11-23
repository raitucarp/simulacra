//! The plan as bundle files: generated text, or a file copied from the media folder.

use super::markdown::{document_markdown, slug, thread_markdown, thread_title};
use super::plan::journal_in_scope;
use super::secrets::redacted_profile;
use super::tables::review_csv;
use super::{ExportPlan, ExportSource, BUNDLE_FORMAT, BUNDLE_FORMAT_VERSION};
use crate::domain::Post;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const MANIFEST_PATH: &str = "manifest.json";
const THREADS_DIR: &str = "threads";
const MEDIA_DIR: &str = "media";
const DOCUMENTS_DIR: &str = "documents";
const ORIGINALS_DIR: &str = "originals";
const IMPORTS_DIR: &str = crate::import::apply::IMPORTS_DIR_NAME;
const REVIEW_CSV: &str = "review/items.csv";
const REVIEW_JSON: &str = "review/items.json";
const AGENTS_DIR: &str = "agents";
const SETTINGS_PATH: &str = "settings.json";
const JOURNAL_PATH: &str = "journal/journal.json";
/// Characters of an id kept in file names (keeps names unique and short).
const ID_TAIL: usize = 8;
const DAY_FORMAT: &str = "%Y-%m-%d";
/// What the manifest says a plain bundle never holds.
const LEFT_OUT: [&str; 3] = ["credentials", "api_keys", "provider_settings"];

/// What one bundle file holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileBody {
    Text(String),
    /// Copied from this file (skipped when it is gone).
    Copy(PathBuf),
}

/// One file of the bundle, at a `/`-separated path inside the zip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleFile {
    pub path: String,
    pub body: FileBody,
}

fn text(path: String, body: String) -> BundleFile {
    BundleFile { path, body: FileBody::Text(body) }
}

fn tail(id: &str) -> String {
    let chars: Vec<char> = id.chars().filter(char::is_ascii_alphanumeric).collect();
    chars[chars.len().saturating_sub(ID_TAIL)..].iter().collect()
}

/// The images `post` shows: (path, description).
pub(super) fn post_images(post: &Post) -> Vec<(String, String)> {
    let attached = post.extras.images.iter().map(|image| (image.path.clone(), image.alt.clone()));
    let legacy = post.image_path.iter().filter(|path| !post.extras.images.iter().any(|i| &i.path == *path)).map(|path| (path.clone(), String::new()));
    attached.chain(legacy).collect()
}

fn media_source(path: &str, media_dir: &Path) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() { path.to_path_buf() } else { media_dir.join(path) }
}

/// Thread files and the images they show (each image once, named after its post).
fn thread_files(plan: &ExportPlan, source: &ExportSource) -> Vec<BundleFile> {
    let mut images: HashMap<String, String> = HashMap::new();
    let mut files = Vec::new();
    for thread in &plan.threads {
        for post in &thread.posts {
            for (path, _) in post_images(post).into_iter().chain(super::care::post_media(post)) {
                let name = Path::new(&path).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let inside = format!("{MEDIA_DIR}/{}-{name}", tail(post.id.as_str()));
                images.entry(path.clone()).or_insert_with(|| {
                    files.push(BundleFile { path: format!("{THREADS_DIR}/{inside}"), body: FileBody::Copy(media_source(&path, &source.media_dir)) });
                    inside
                });
            }
        }
        let first = &thread.posts[0];
        let name = format!("{THREADS_DIR}/{}-{}-{}.md", first.created_at.strftime(DAY_FORMAT), slug(&thread_title(&first.content)), tail(thread.root.as_str()));
        files.push(text(name, thread_markdown(thread, source, &|path| images.get(path).cloned().unwrap_or_default())));
    }
    files
}

fn document_files(plan: &ExportPlan, source: &ExportSource) -> Vec<BundleFile> {
    plan.documents.iter().flat_map(|doc| {
        let original = Some(doc.record.provenance.original.as_str()).filter(|name| plan.options.include_originals && !name.is_empty());
        let inside = original.map(|name| format!("{ORIGINALS_DIR}/{name}"));
        let markdown = text(format!("{DOCUMENTS_DIR}/{}-{}.md", slug(&doc.title), tail(&doc.id)), document_markdown(doc, &source.library, inside.as_deref()));
        let copy = original.zip(inside).map(|(name, inside)| BundleFile { path: format!("{DOCUMENTS_DIR}/{inside}"), body: FileBody::Copy(source.media_dir.join(IMPORTS_DIR).join(name)) });
        std::iter::once(markdown).chain(copy)
    }).collect()
}

fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

fn data_files(plan: &ExportPlan, source: &ExportSource) -> Vec<BundleFile> {
    let review = (!plan.review.is_empty()).then(|| [text(REVIEW_CSV.into(), review_csv(&plan.review)), text(REVIEW_JSON.into(), pretty(&json!(plan.review)))]);
    let agents = plan.agents.iter().map(|p| text(format!("{AGENTS_DIR}/{}-{}.json", slug(&p.handle), tail(p.id.as_str())), pretty(&redacted_profile(p))));
    let settings: serde_json::Map<String, Value> = source.settings.iter().map(|(k, v)| (k.clone(), serde_json::from_str(v).unwrap_or_else(|_| json!(v)))).collect();
    let settings = (plan.counts.settings > 0).then(|| text(SETTINGS_PATH.into(), pretty(&Value::Object(settings))));
    let journal = source.journal.as_ref().filter(|_| plan.options.include_journal).map(|book| text(JOURNAL_PATH.into(), pretty(&journal_in_scope(book, &plan.options.scope).0)));
    let reactions = (!plan.reactions.is_empty()).then(|| text(super::REACTIONS_PATH.into(), super::reactions_json(&plan.reactions, &source.authors)));
    review.into_iter().flatten().chain(agents).chain(settings).chain(journal).chain(reactions).collect()
}

fn manifest(plan: &ExportPlan, source: &ExportSource) -> BundleFile {
    let c = plan.counts;
    let scope = match &plan.options.scope {
        super::ExportScope::Everything => json!({ "kind": plan.options.scope.key() }),
        super::ExportScope::Collection(id) => json!({ "kind": plan.options.scope.key(), "collection": source.library.collection(id).map(|c| c.name.clone()) }),
        super::ExportScope::DateRange { from, to } => json!({ "kind": plan.options.scope.key(), "from": from.to_string(), "to": to.to_string() }),
        super::ExportScope::Persona(id) => json!({ "kind": plan.options.scope.key(), "persona": source.authors.get(id).map(|p| p.handle.clone()) }),
    };
    let counts = json!({ "threads": c.threads, "posts": c.posts, "documents": c.documents, "originals": c.originals, "images": c.images, "review_items": c.review_items, "agents": c.agents, "settings": c.settings, "journal_records": c.journal_records, "reactions": c.reactions });
    let value = json!({ "format": BUNDLE_FORMAT, "format_version": BUNDLE_FORMAT_VERSION, "app_version": source.app_version, "exported_at": crate::time::rfc3339(source.exported_at), "network": source.network_name, "scope": scope, "includes_originals": plan.options.include_originals, "includes_private_journal": plan.options.include_journal, "counts": counts, "left_out": LEFT_OUT });
    text(MANIFEST_PATH.into(), pretty(&value))
}

/// Every file of the bundle, the manifest first.
pub fn bundle_files(plan: &ExportPlan, source: &ExportSource) -> Vec<BundleFile> {
    std::iter::once(manifest(plan, source)).chain(thread_files(plan, source)).chain(document_files(plan, source)).chain(data_files(plan, source)).collect()
}
