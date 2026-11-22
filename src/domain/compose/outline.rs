//! The split of free compose text as the composer previews it (T2): every post in thread
//! order (each before its replies), with how deep it nests, the raw line it came from and
//! its visible length, and the post it replies to. It follows the same rules as
//! [`super::split_compose`].

use crate::domain::post::visible_text_len;

/// One post of the split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeLine {
    /// The raw line (zero-based) of the text the post comes from.
    pub line: usize,
    /// How many posts it sits under (`0` for a post of the main thread).
    pub depth: usize,
    /// The post (index into the list) it replies to (T4): the line it is indented under, or,
    /// for a post of the main thread, the one before it in the main thread.
    pub parent: Option<usize>,
    pub text: String,
    /// Visible characters (markup excluded), as the per-post limit counts them.
    pub chars: usize,
}

impl ComposeLine {
    /// Whether the post is longer than `limit` visible characters.
    pub fn is_over(&self, limit: usize) -> bool {
        self.chars > limit
    }
}

/// Every post of freely composed `text`, in thread order.
pub fn compose_lines(text: &str) -> Vec<ComposeLine> {
    let mut open: Vec<(usize, usize)> = Vec::new();
    let (mut out, mut last_root) = (Vec::new(), None);
    for (line, raw) in text.split('\n').enumerate() {
        let Some((level, body)) = super::lines::item(raw) else { continue };
        while open.last().is_some_and(|(open_level, _)| *open_level >= level) {
            open.pop();
        }
        let parent = open.last().map(|(_, at)| *at).or(last_root.filter(|_| open.is_empty()));
        if open.is_empty() {
            last_root = Some(out.len());
        }
        out.push(ComposeLine { line, depth: open.len(), parent, text: body.to_string(), chars: visible_text_len(body) });
        open.push((level, out.len() - 1));
    }
    out
}
