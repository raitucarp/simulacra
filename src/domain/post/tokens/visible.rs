//! What a reader sees of a token tree: its text without markup, and its length.
//!
//! Mark delimiters, escapes, code fences and wikilink brackets are markup; a wikilink counts
//! as its target, a tag as `#tag`, code as its verbatim text.

use super::{tokenize_post_body, BodyToken};

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
        BodyToken::Bold(children) | BodyToken::Italic(children) | BodyToken::Underline(children) | BodyToken::Spoiler(children) => {
            children.iter().for_each(|child| push_visible(child, out));
        }
    }
}

/// Every wikilink target in `tokens`, including those inside marks, in order.
pub fn wikilink_targets(tokens: &[BodyToken]) -> Vec<String> {
    let mut targets = Vec::new();
    collect_targets(tokens, &mut targets);
    targets
}

fn collect_targets(tokens: &[BodyToken], out: &mut Vec<String>) {
    for token in tokens {
        match token {
            BodyToken::Wikilink { target, .. } => out.push(target.clone()),
            other => {
                if let Some((_, children)) = other.as_mark() {
                    collect_targets(children, out);
                }
            }
        }
    }
}
