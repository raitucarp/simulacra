//! What a vector is for, and how a model is told.
//!
//! `gemini-embedding-001` takes a `taskType` field (and a `title` for documents). Newer
//! models without that field (`gemini-embedding-2`, EmbeddingGemma) read the task from a
//! text prefix instead: `task: search result | query: …` for queries and
//! `title: … | text: …` for documents. Plain OpenAI-style models take the text as is.

/// The embedding task (Gemini `TaskType` wire names).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskType {
    /// A passage stored for search (posts, document chunks).
    RetrievalDocument,
    /// A search query.
    RetrievalQuery,
    /// Symmetric similarity between two texts.
    SemanticSimilarity,
    /// Grouping texts (the embedding map).
    Clustering,
}

impl TaskType {
    /// The `taskType` value.
    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::RetrievalDocument => "RETRIEVAL_DOCUMENT",
            Self::RetrievalQuery => "RETRIEVAL_QUERY",
            Self::SemanticSimilarity => "SEMANTIC_SIMILARITY",
            Self::Clustering => "CLUSTERING",
        }
    }

    /// The task name in a text prefix (`task: <name> | query: …`).
    const fn prefix_name(self) -> &'static str {
        match self {
            Self::RetrievalDocument | Self::RetrievalQuery => "search result",
            Self::SemanticSimilarity => "sentence similarity",
            Self::Clustering => "clustering",
        }
    }
}

/// How a model receives the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskEncoding {
    /// `taskType` (and `title`) request fields.
    Field,
    /// A task prefix in the text.
    Prefix,
    /// The text alone.
    Plain,
}

/// Models that take the `taskType` field.
const TASK_FIELD_MODELS: [&str; 3] = ["gemini-embedding-001", "text-embedding-004", "embedding-001"];
/// Model id fragments of models that read a task prefix.
const PREFIX_MODEL_MARKERS: [&str; 2] = ["gemini-embedding-2", "embeddinggemma"];
/// Title placeholder of an untitled document in a prefix.
const NO_TITLE: &str = "none";

/// How the Gemini API passes the task to `model`.
pub fn gemini_encoding(model: &str) -> TaskEncoding {
    let bare = model.trim().trim_start_matches("models/");
    if TASK_FIELD_MODELS.contains(&bare) {
        TaskEncoding::Field
    } else {
        TaskEncoding::Prefix
    }
}

/// How a local or OpenAI-compatible server passes the task to `model`.
pub fn plain_encoding(model: &str) -> TaskEncoding {
    let lower = model.to_lowercase();
    if PREFIX_MODEL_MARKERS.iter().any(|marker| lower.contains(marker)) {
        TaskEncoding::Prefix
    } else {
        TaskEncoding::Plain
    }
}

/// `text` (with `title` for documents) as a prefixed input.
pub fn prefixed(task: TaskType, title: &str, text: &str) -> String {
    match task {
        TaskType::RetrievalDocument => {
            let title = if title.trim().is_empty() { NO_TITLE } else { title.trim() };
            format!("title: {title} | text: {text}")
        }
        other => format!("task: {} | query: {text}", other.prefix_name()),
    }
}
