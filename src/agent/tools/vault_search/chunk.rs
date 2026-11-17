//! Word-window chunking and tokenization of vault documents.

/// Words per chunk.
pub const CHUNK_WORDS: usize = 120;
/// Words shared by consecutive chunks, so a passage cut at a boundary still matches.
pub const CHUNK_OVERLAP_WORDS: usize = 30;
/// Tokens shorter than this are ignored.
const MIN_TOKEN_CHARS: usize = 2;
/// Very common English words that carry no topical signal.
const STOPWORDS: [&str; 24] = [
    "the", "and", "for", "are", "but", "not", "you", "all", "any", "can", "was", "our", "has", "this",
    "that", "with", "from", "they", "have", "what", "which", "their", "there", "into",
];

/// One retrievable passage of a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    pub doc_id: String,
    pub title: String,
    /// Chunk position within the document.
    pub ordinal: usize,
    pub text: String,
}

/// Splits `content` into overlapping windows of [`CHUNK_WORDS`] words (at least one chunk
/// for a non-blank document).
pub fn chunk_document(doc_id: &str, title: &str, content: &str) -> Vec<Chunk> {
    let words: Vec<&str> = content.split_whitespace().collect();
    let stride = CHUNK_WORDS - CHUNK_OVERLAP_WORDS;
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < words.len() {
        let end = (start + CHUNK_WORDS).min(words.len());
        chunks.push(Chunk {
            doc_id: doc_id.to_string(),
            title: title.to_string(),
            ordinal: chunks.len(),
            text: words[start..end].join(" "),
        });
        if end == words.len() {
            break;
        }
        start += stride;
    }
    chunks
}

/// Lowercased alphanumeric tokens, minus stopwords and very short tokens.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|token| token.chars().count() >= MIN_TOKEN_CHARS)
        .map(str::to_lowercase)
        .filter(|token| !STOPWORDS.contains(&token.as_str()))
        .collect()
}
