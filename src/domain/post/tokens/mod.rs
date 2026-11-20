//! Post body token model: every inline token (text, links, tags, marks, code) and the one
//! block token (fenced code), plus the parser that produces them.
//!
//! The store tokenizes each body once (on insert / load) and renderers reuse the tokens.
//! Marks nest (`**bold *and italic***`), so mark tokens hold their children; code never does.

mod atoms;
mod builder;
mod fence;
mod inline;
mod visible;

pub use atoms::is_trailing_punctuation;
pub use visible::{visible_len, visible_text, visible_text_len, wikilink_targets};

use super::parse::{HASHTAG_SIGIL, WIKILINK_OPEN};

/// Characters that may start markup: line breaks, mark delimiters and escapes.
const MARKUP_CHARS: [char; 6] = ['\n', '*', '_', '`', '|', '\\'];

/// One token of a post body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyToken {
    /// Plain text run (may contain line breaks).
    Text(String),
    /// Obsidian `[[target]]` link followed by optional trailing punctuation.
    Wikilink { target: String, trailing: String },
    /// `#Tag` followed by optional trailing punctuation.
    Hashtag { tag: String, trailing: String },
    /// `@handle` mention followed by optional trailing punctuation (not produced yet).
    Mention { handle: String, trailing: String },
    /// Web address followed by optional trailing punctuation (not produced yet).
    Url { url: String, trailing: String },
    /// `**bold**` run.
    Bold(Vec<BodyToken>),
    /// `*italic*` run.
    Italic(Vec<BodyToken>),
    /// `__underline__` run.
    Underline(Vec<BodyToken>),
    /// `||spoiler||` run.
    Spoiler(Vec<BodyToken>),
    /// `` `inline code` ``, shown verbatim.
    Code(String),
    /// Fenced code block; `lang` is the lower-cased tag, `None` (plain) when missing or malformed.
    CodeBlock { lang: Option<String>, code: String },
}

/// An inline mark that wraps other tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    Bold,
    Italic,
    Underline,
    Spoiler,
}

impl Mark {
    /// The delimiter written on both sides of the marked text.
    pub fn delimiter(self) -> &'static str {
        match self {
            Self::Bold => "**",
            Self::Italic => "*",
            Self::Underline => "__",
            Self::Spoiler => "||",
        }
    }

    /// The token holding `children` under this mark.
    pub fn wrap(self, children: Vec<BodyToken>) -> BodyToken {
        match self {
            Self::Bold => BodyToken::Bold(children),
            Self::Italic => BodyToken::Italic(children),
            Self::Underline => BodyToken::Underline(children),
            Self::Spoiler => BodyToken::Spoiler(children),
        }
    }
}

impl BodyToken {
    /// The mark and children of a mark token; `None` for every other token.
    pub fn as_mark(&self) -> Option<(Mark, &[BodyToken])> {
        match self {
            Self::Bold(children) => Some((Mark::Bold, children)),
            Self::Italic(children) => Some((Mark::Italic, children)),
            Self::Underline(children) => Some((Mark::Underline, children)),
            Self::Spoiler(children) => Some((Mark::Spoiler, children)),
            _ => None,
        }
    }
}

/// True when the content needs tokenized rendering (links, hashtags, marks or line breaks).
pub fn has_rich_markup(content: &str) -> bool {
    content.contains(WIKILINK_OPEN) || content.contains(HASHTAG_SIGIL) || content.contains(MARKUP_CHARS)
}

/// Splits post content into ordered tokens: fenced code blocks first, then inline marks,
/// wikilinks and hashtags in the prose between them.
pub fn tokenize_post_body(content: &str) -> Vec<BodyToken> {
    let mut tokens = Vec::new();
    for segment in fence::segments(content) {
        match segment {
            fence::Segment::Prose(text) => tokens.extend(inline::parse_inline(text)),
            fence::Segment::Code { lang, code } => tokens.push(BodyToken::CodeBlock { lang, code }),
        }
    }
    tokens
}

#[cfg(test)]
mod tests;
