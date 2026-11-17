//! What the vault view shows, taken from the store: the active network's documents, the
//! vocabulary, link counts and backlink previews. The shell rebuilds it when the store's
//! fingerprint changes and hands it to the panel.

use crate::domain::document::links::{backlinks, link_counts, ScoredDocument};
use crate::domain::document::record::fnv1a;
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::Library;
use crate::domain::preferences::{Choice, ReadingWidth};
use crate::domain::{GroundingDocument, NetworkId, PostId};
use crate::state::AppState;
use crate::storage::Vault;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::rc::Rc;

/// Words of a backlink excerpt.
const EXCERPT_WORDS: usize = 28;
const ELLIPSIS: &str = "…";

/// A post that links to a document, as listed on its entry page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BacklinkPreview {
    pub post_id: PostId,
    pub author: String,
    pub excerpt: String,
    pub created_at: DateTime<Utc>,
}

/// Everything the vault renders, shared cheaply between frames.
#[derive(Clone)]
pub struct VaultData {
    pub network_id: NetworkId,
    pub documents: Rc<Vec<GroundingDocument>>,
    pub library: Rc<Library>,
    pub tree: Rc<SubjectTree>,
    pub link_counts: Rc<HashMap<String, usize>>,
    pub backlinks: Rc<HashMap<String, Vec<BacklinkPreview>>>,
    /// Semantic neighbours per document id: the hook an embedding index fills.
    pub semantic: Rc<HashMap<String, Vec<ScoredDocument>>>,
    /// The vault worker for full-text search (none without persistence).
    pub vault: Option<Vault>,
    /// Display name of the active persona (recorded as "added by").
    pub persona: String,
    /// Appearance preferences: instant changes instead of motion, and the reader's measure.
    pub reduced_motion: bool,
    pub reading_width: ReadingWidth,
}

fn excerpt(text: &str) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.len() <= EXCERPT_WORDS {
        words.join(" ")
    } else {
        format!("{}{ELLIPSIS}", words[..EXCERPT_WORDS].join(" "))
    }
}

impl VaultData {
    /// The vault's view of `state` (active network only).
    pub fn from_state(state: &AppState) -> Self {
        let documents: Vec<GroundingDocument> = state.current_documents().cloned().collect();
        let previews = documents
            .iter()
            .map(|doc| {
                let list = backlinks(doc, &state.posts)
                    .into_iter()
                    .map(|post| BacklinkPreview {
                        post_id: post.id.clone(),
                        author: state.author_of(post).name.clone(),
                        excerpt: excerpt(&post.content),
                        created_at: post.created_at,
                    })
                    .collect();
                (doc.id.clone(), list)
            })
            .collect();
        Self {
            network_id: state.active_network_id.clone(),
            link_counts: Rc::new(link_counts(&documents, &state.posts)),
            documents: Rc::new(documents),
            tree: Rc::new(state.library.tree()),
            library: Rc::new(state.library.clone()),
            backlinks: Rc::new(previews),
            semantic: Rc::default(),
            vault: state.db.clone(),
            persona: state.current_profile().map(|p| p.name.clone()).unwrap_or_default(),
            reduced_motion: state.preferences().appearance.reduced_motion,
            reading_width: state.preferences().appearance.reading_width,
        }
    }

    /// A cheap fingerprint of what [`Self::from_state`] reads, to skip rebuilding.
    pub fn fingerprint(state: &AppState) -> u64 {
        let docs: String = state
            .current_documents()
            .map(|d| format!("{}|{}|{:?}|{}|{}", d.id, d.title, d.record.updated_at, d.record.reading.progress, d.record.reading.status.key()))
            .collect();
        let library = format!("{}|{}", state.library.subjects.len(), state.library.collections.len());
        let appearance = &state.preferences().appearance;
        let display = format!("{}|{}|{}", appearance.reduced_motion, appearance.reading_width.key(), state.active_profile_id);
        let text = format!("{}#{docs}#{library}#{}#{display}", state.active_network_id, state.posts.len());
        fnv1a(text.as_bytes())
    }

    pub fn document(&self, id: &str) -> Option<&GroundingDocument> {
        self.documents.iter().find(|d| d.id == id)
    }
}
