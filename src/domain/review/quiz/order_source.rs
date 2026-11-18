//! Where an ordering puzzle comes from: a thread (a post's ancestors, then the earliest
//! reply at each step) or a passage of consecutive sentences.

use super::order::{MAX_PIECES, MIN_PIECES};
use crate::domain::{Post, PostId};
use std::collections::HashMap;

/// Characters of a post shown on one puzzle piece.
pub const PIECE_CHARS: usize = 220;
const ELLIPSIS: char = '…';

/// `text` cut to `limit` characters at a word boundary.
pub fn piece_text(text: &str, limit: usize) -> String {
    let text = text.trim();
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let cut: String = text.chars().take(limit).collect();
    let whole_word = text.chars().nth(limit).is_some_and(char::is_whitespace);
    let at_word = if whole_word { cut.as_str() } else { cut.rfind(char::is_whitespace).map_or(cut.as_str(), |end| &cut[..end]) };
    format!("{}{ELLIPSIS}", at_word.trim_end())
}

/// The thread through `start`: its ancestors, itself, then the earliest reply at each step,
/// at most [`MAX_PIECES`] posts, oldest first.
pub fn thread_line<'a>(posts: &'a [Post], start: &PostId) -> Vec<&'a Post> {
    let by_id: HashMap<&PostId, &Post> = posts.iter().map(|post| (&post.id, post)).collect();
    let mut line: Vec<&Post> = Vec::new();
    let mut cursor = by_id.get(start).copied();
    while let Some(post) = cursor.filter(|p| !line.iter().any(|seen| seen.id == p.id)) {
        line.insert(0, post);
        cursor = post.reply_to_id.as_ref().and_then(|id| by_id.get(id).copied());
    }
    while line.len() < MAX_PIECES {
        let Some(last) = line.last() else { break };
        let reply = posts
            .iter()
            .filter(|p| p.reply_to_id.as_ref() == Some(&last.id) && !line.iter().any(|seen| seen.id == p.id))
            .min_by_key(|p| p.created_at);
        match reply {
            Some(reply) => line.push(reply),
            None => break,
        }
    }
    let skip = line.len().saturating_sub(MAX_PIECES);
    line.split_off(skip)
}

/// A window of up to [`MAX_PIECES`] consecutive sentences starting near `offset`
/// (wrapping so that a full window fits); empty with fewer than [`MIN_PIECES`].
pub fn sentence_window(sentences: &[String], offset: usize) -> Vec<String> {
    if sentences.len() < MIN_PIECES {
        return Vec::new();
    }
    let size = sentences.len().min(MAX_PIECES);
    let start = offset % (sentences.len() - size + 1);
    sentences[start..start + size].to_vec()
}

/// Markers that open a quote or list line of plain or Markdown text.
const LINE_MARKERS: [&str; 4] = ["> ", "- ", "* ", "+ "];
const HEADING_MARK: char = '#';
const PARAGRAPH_BREAK: &str = "\n\n";

/// The prose paragraphs of `text`: headings dropped, quote and list markers removed, the
/// lines of a paragraph joined (so sentences never carry Markdown residue).
pub fn prose_paragraphs(text: &str) -> Vec<String> {
    text.replace("\r\n", "\n")
        .split(PARAGRAPH_BREAK)
        .map(|block| {
            block
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with(HEADING_MARK))
                .map(|line| LINE_MARKERS.iter().fold(line, |line, marker| line.strip_prefix(marker).unwrap_or(line)).trim())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|paragraph| !paragraph.is_empty())
        .collect()
}
