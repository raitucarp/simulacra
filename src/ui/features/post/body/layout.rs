//! Lays a token tree out as the inline items the body renders: whole words (with the byte
//! ranges each mark covers), line breaks, links, tags, code spans and code blocks.
//!
//! Words are whitespace-separated, so a word can mix marks (`un**believ**able`) and still
//! wrap as one unit.

use super::super::consts::{CARRIAGE_RETURN, LINE_BREAK};
use crate::domain::post::{BodyToken, Mark};
use std::ops::Range;

/// The marks in force on a run of text.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct MarkSet {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    /// Parsed but not hidden yet: the spoiler reveal is its own task, so the text shows plainly.
    pub spoiler: bool,
}

impl MarkSet {
    fn with(mut self, mark: Mark) -> Self {
        match mark {
            Mark::Bold => self.bold = true,
            Mark::Italic => self.italic = true,
            Mark::Underline => self.underline = true,
            Mark::Spoiler => self.spoiler = true,
        }
        self
    }
}

/// One inline item of the body flow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Item<'a> {
    /// A word and the marked byte ranges inside it.
    Word { text: String, marks: Vec<(Range<usize>, MarkSet)> },
    /// End of a line.
    Break,
    Code(&'a str),
    Wikilink { target: &'a str, trailing: &'a str },
    Hashtag { tag: &'a str, trailing: &'a str },
    /// A fenced block; it fills the width, so it sits on its own line.
    Block { lang: Option<&'a str>, code: &'a str },
}

/// Whether the line after the break at `index` is blank (drawn as a paragraph gap).
pub(crate) fn blank_line_after(items: &[Item<'_>], index: usize) -> bool {
    matches!(items.get(index + 1), None | Some(Item::Break | Item::Block { .. }))
}

/// The inline items of `tokens`, in order.
pub(crate) fn layout(tokens: &[BodyToken]) -> Vec<Item<'_>> {
    let mut builder = Builder::default();
    builder.tokens(tokens, MarkSet::default());
    builder.flush();
    builder.items
}

#[derive(Default)]
struct Builder<'a> {
    items: Vec<Item<'a>>,
    word: String,
    marks: Vec<(Range<usize>, MarkSet)>,
}

impl<'a> Builder<'a> {
    fn tokens(&mut self, tokens: &'a [BodyToken], marks: MarkSet) {
        for token in tokens {
            self.token(token, marks);
        }
    }

    fn token(&mut self, token: &'a BodyToken, marks: MarkSet) {
        match token {
            BodyToken::Text(text) => self.text(text, marks),
            BodyToken::Mention { handle: text, trailing } | BodyToken::Url { url: text, trailing } => {
                self.text(text, marks);
                self.text(trailing, marks);
            }
            BodyToken::Code(code) => self.item(Item::Code(code)),
            BodyToken::Wikilink { target, trailing } => self.item(Item::Wikilink { target, trailing }),
            BodyToken::Hashtag { tag, trailing } => self.item(Item::Hashtag { tag, trailing }),
            BodyToken::CodeBlock { lang, code } => self.item(Item::Block { lang: lang.as_deref(), code }),
            BodyToken::Bold(_) | BodyToken::Italic(_) | BodyToken::Underline(_) | BodyToken::Spoiler(_) => {
                if let Some((mark, children)) = token.as_mark() {
                    self.tokens(children, marks.with(mark));
                }
            }
        }
    }

    /// Splits text into words at whitespace and into lines at line breaks.
    fn text(&mut self, text: &str, marks: MarkSet) {
        for (line_index, line) in text.split(LINE_BREAK).enumerate() {
            if line_index > 0 {
                self.item(Item::Break);
            }
            for (word_index, chunk) in line.trim_end_matches(CARRIAGE_RETURN).split(char::is_whitespace).enumerate() {
                if word_index > 0 {
                    self.flush();
                }
                self.append(chunk, marks);
            }
        }
    }

    fn append(&mut self, chunk: &str, marks: MarkSet) {
        if chunk.is_empty() {
            return;
        }
        let start = self.word.len();
        self.word.push_str(chunk);
        if marks != MarkSet::default() {
            self.marks.push((start..self.word.len(), marks));
        }
    }

    /// Ends the current word, if any.
    fn flush(&mut self) {
        if !self.word.is_empty() {
            let text = std::mem::take(&mut self.word);
            self.items.push(Item::Word { text, marks: std::mem::take(&mut self.marks) });
        }
    }

    fn item(&mut self, item: Item<'a>) {
        self.flush();
        self.items.push(item);
    }
}
