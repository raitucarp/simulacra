//! RTF: text outside destination groups (font and color tables, pictures, headers), with
//! `\par` paragraph breaks, `\'hh` and `\uN` characters, and the `\info` title and author.

mod control;

use crate::import::document::Block;
use crate::import::parse::sections::{normalize_lines, normalize_whitespace, sections_from_blocks};
use crate::import::parse::Content;
use control::{read_control, Control};

/// Destinations whose text is never prose.
const SKIPPED: [&str; 19] = [
    "fonttbl", "colortbl", "stylesheet", "pict", "header", "footer", "headerl", "headerr", "footerl",
    "footerr", "object", "listtable", "listoverridetable", "rsidtbl", "generator", "xmlnstbl", "themedata",
    "info", "datastore",
];
/// Named characters.
const SYMBOLS: [(&str, char); 7] = [
    ("emdash", '—'), ("endash", '–'), ("lquote", '‘'), ("rquote", '’'), ("ldblquote", '“'),
    ("rdblquote", '”'), ("bullet", '•'),
];
const UNICODE_WRAP: i32 = 65536;

/// Where a group's text goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sink {
    Body,
    Skip,
    Title,
    Author,
}

#[derive(Default)]
struct Reader {
    groups: Vec<Sink>,
    paragraph: String,
    paragraphs: Vec<String>,
    title: String,
    author: String,
    unicode_skip: usize,
    pending_skip: usize,
}

impl Reader {
    fn sink(&self) -> Sink {
        self.groups.last().copied().unwrap_or(Sink::Body)
    }

    fn set_sink(&mut self, sink: Sink) {
        if let Some(current) = self.groups.last_mut() {
            *current = sink;
        }
    }

    fn push(&mut self, c: char) {
        if self.pending_skip > 0 {
            self.pending_skip -= 1;
            return;
        }
        match self.sink() {
            Sink::Body => self.paragraph.push(c),
            Sink::Title => self.title.push(c),
            Sink::Author => self.author.push(c),
            Sink::Skip => {}
        }
    }

    fn end_paragraph(&mut self) {
        let text = normalize_lines(&std::mem::take(&mut self.paragraph));
        self.paragraphs.extend((!text.is_empty()).then_some(text));
    }

    fn word(&mut self, word: &str, parameter: Option<i32>) {
        let in_info = self.sink() == Sink::Skip;
        match word {
            "par" | "sect" | "page" if self.sink() == Sink::Body => self.end_paragraph(),
            "line" => self.push('\n'),
            "tab" => self.push(' '),
            "uc" => self.unicode_skip = parameter.unwrap_or(1).max(0) as usize,
            "u" => {
                let code = parameter.unwrap_or(0);
                let code = if code < 0 { code + UNICODE_WRAP } else { code };
                char::from_u32(code as u32).into_iter().for_each(|c| self.push(c));
                self.pending_skip = self.unicode_skip;
            }
            "title" if in_info => self.set_sink(Sink::Title),
            "author" if in_info => self.set_sink(Sink::Author),
            skipped if SKIPPED.contains(&skipped) => self.set_sink(Sink::Skip),
            symbol => {
                if let Some((_, c)) = SYMBOLS.iter().find(|(name, _)| *name == symbol) {
                    self.push(*c);
                }
            }
        }
    }
}

/// Parses RTF source text.
pub fn parse_rtf(source: &str) -> Content {
    let mut reader = Reader { unicode_skip: 1, ..Reader::default() };
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => reader.groups.push(reader.sink()),
            '}' => drop(reader.groups.pop()),
            '\\' => match read_control(&mut chars) {
                Control::Word(word, parameter) => reader.word(&word, parameter),
                Control::Literal(c) => reader.push(c),
                Control::Ignorable => reader.set_sink(Sink::Skip),
                Control::Break => reader.end_paragraph(),
                Control::Other => {}
            },
            '\r' | '\n' => {}
            other => reader.push(other),
        }
    }
    reader.end_paragraph();
    let mut content = Content {
        sections: sections_from_blocks(reader.paragraphs.into_iter().map(Block::paragraph).collect()),
        ..Content::default()
    };
    content.metadata.title = Some(normalize_whitespace(&reader.title)).filter(|t| !t.is_empty());
    content.metadata.creators = Some(normalize_whitespace(&reader.author)).filter(|a| !a.is_empty()).into_iter().collect();
    content
}
