//! The format-neutral result of parsing: metadata, file facts and an ordered tree of
//! sections, each an ordered list of text blocks.

use super::detect::Format;

/// Bibliographic metadata, as far as the file carries it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub creators: Vec<String>,
    /// Language tag (`en`, `de-AT`, `ja`).
    pub language: Option<String>,
    /// Publication or creation date, as written in the file.
    pub date: Option<String>,
    pub identifiers: Vec<String>,
    pub publisher: Option<String>,
    pub description: Option<String>,
}

/// What every file yields, even an unrecognised one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileFacts {
    pub size_bytes: u64,
    /// Lower-case hex SHA-256 of the bytes.
    pub sha256: String,
    pub mime: &'static str,
}

/// The role of a block of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    /// A heading of level 1 (largest) to 6.
    Heading(u8),
    Paragraph,
    Quote,
    ListItem,
    /// Text whose line breaks matter (code, extracted strings).
    Preformatted,
}

/// One block of text; line breaks inside it (verse, addresses) are kept as `\n`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub kind: BlockKind,
    pub text: String,
}

impl Block {
    pub fn new(kind: BlockKind, text: impl Into<String>) -> Self {
        Self { kind, text: text.into() }
    }

    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::new(BlockKind::Paragraph, text)
    }

    pub fn is_heading(&self) -> bool {
        matches!(self.kind, BlockKind::Heading(_))
    }
}

/// A chapter (or page, part, sheet): an optional title and its blocks in reading order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Section {
    pub title: Option<String>,
    pub blocks: Vec<Block>,
}

impl Section {
    pub fn new(title: Option<String>, blocks: Vec<Block>) -> Self {
        Self { title, blocks }
    }

    /// The blocks that carry body text (everything but headings).
    pub fn body(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter().filter(|block| !block.is_heading())
    }
}

/// Something the parser wants the reader to know (shown as a note in the import sheet).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseNote {
    /// Image size in pixels.
    Dimensions { width: u32, height: u32 },
    /// Number of entries listed from an archive.
    ArchiveEntries(usize),
    /// Printable strings recovered from a binary file.
    ExtractedStrings(usize),
    /// PDF pages without a text layer (scanned pages).
    PagesWithoutText(usize),
    /// A compression or feature this reader does not decode (the file was read as binary).
    Unsupported(&'static str),
    /// The file is encrypted or DRM-protected (the file was read as binary).
    Encrypted,
    /// The file claimed a format but could not be read as it (the file was read as binary).
    Unreadable(String),
}

/// A parsed file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedDocument {
    pub format: Format,
    pub metadata: DocumentMetadata,
    pub facts: FileFacts,
    pub sections: Vec<Section>,
    pub notes: Vec<ParseNote>,
}

impl ParsedDocument {
    /// Every body block (no headings) across all sections, in reading order.
    pub fn body_blocks(&self) -> impl Iterator<Item = &Block> {
        self.sections.iter().flat_map(Section::body)
    }

    /// The whole text: headings and blocks, one blank line apart.
    pub fn full_text(&self) -> String {
        let blocks = self.sections.iter().flat_map(|section| section.blocks.iter());
        blocks.map(|block| block.text.as_str()).collect::<Vec<_>>().join("\n\n")
    }

    /// Number of body blocks.
    pub fn paragraph_count(&self) -> usize {
        self.body_blocks().count()
    }
}
