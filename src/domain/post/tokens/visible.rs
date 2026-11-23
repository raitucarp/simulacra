//! What a reader sees of a token tree: its text without markup, and its length.
//!
//! Mark delimiters, escapes, code fences and wikilink brackets are markup; a wikilink counts
//! as its target, a tag as `#tag`, code as its verbatim text. An embed (N1) shows another
//! post's text, so only its trailing punctuation counts.

use super::{tokenize_post_body, BodyToken};
use crate::domain::data::history::SemVer;

/// Visible characters of `tokens` (the post length the character limit applies to).
pub fn visible_len(tokens: &[BodyToken]) -> usize {
    tokens.iter().map(token_len).sum()
}

/// Visible length of the trimmed `content`, after parsing its markup.
pub fn visible_text_len(content: &str) -> usize {
    visible_len(&tokenize_post_body(content.trim()))
}

fn token_len(token: &BodyToken) -> usize {
    let count = |text: &str| text.chars().count();
    match token {
        BodyToken::Text(text) | BodyToken::Code(text) | BodyToken::CodeBlock { code: text, .. } => count(text),
        BodyToken::Wikilink { target: text, trailing }
        | BodyToken::Hashtag { tag: text, trailing }
        | BodyToken::Mention { handle: text, trailing }
        | BodyToken::Url { url: text, trailing } => count(text) + count(trailing),
        BodyToken::Embed { trailing, .. } => count(trailing),
        BodyToken::Bold(children) | BodyToken::Italic(children) | BodyToken::Underline(children) | BodyToken::Spoiler(children) => {
            visible_len(children)
        }
    }
}

/// The visible text of `tokens`, in reading order; its length is [`visible_len`].
pub fn visible_text(tokens: &[BodyToken]) -> String {
    let mut out = String::new();
    for token in tokens {
        push_visible(token, &mut out);
    }
    out
}

fn push_visible(token: &BodyToken, out: &mut String) {
    match token {
        BodyToken::Text(text) | BodyToken::Code(text) | BodyToken::CodeBlock { code: text, .. } => out.push_str(text),
        BodyToken::Wikilink { target: text, trailing }
        | BodyToken::Hashtag { tag: text, trailing }
        | BodyToken::Mention { handle: text, trailing }
        | BodyToken::Url { url: text, trailing } => {
            out.push_str(text);
            out.push_str(trailing);
        }
        BodyToken::Embed { trailing, .. } => out.push_str(trailing),
        BodyToken::Bold(children) | BodyToken::Italic(children) | BodyToken::Underline(children) | BodyToken::Spoiler(children) => {
            children.iter().for_each(|child| push_visible(child, out));
        }
    }
}

/// Every wikilink target in `tokens`, including those inside marks, in order.
pub fn wikilink_targets(tokens: &[BodyToken]) -> Vec<String> {
    let mut targets = Vec::new();
    walk(tokens, &mut |token| {
        if let BodyToken::Wikilink { target, .. } = token {
            targets.push(target.clone());
        }
    });
    targets
}

/// Every embed in `tokens` (N1), including those inside marks, in order: the target and the
/// version written with it.
pub fn embed_targets(tokens: &[BodyToken]) -> Vec<(String, Option<SemVer>)> {
    let mut targets = Vec::new();
    walk(tokens, &mut |token| {
        if let BodyToken::Embed { target, version, .. } = token {
            targets.push((target.clone(), *version));
        }
    });
    targets
}

/// Visits every token of the tree, marks before their children.
fn walk(tokens: &[BodyToken], visit: &mut impl FnMut(&BodyToken)) {
    for token in tokens {
        visit(token);
        if let Some((_, children)) = token.as_mark() {
            walk(children, visit);
        }
    }
}
