//! What the background scan of an attached file found: whether it is still where the post
//! says, and the preview its card shows.

/// The outcome of scanning a file a post links to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileScan {
    /// Nothing readable at the path.
    Missing,
    /// The file is there.
    Present(FileFacts),
}

/// What is known about a file that is there.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FileFacts {
    pub preview: FilePreview,
}

/// The preview a file card shows beside the file's facts (images are previewed through the
/// image scan instead).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FilePreview {
    /// The plain file card.
    #[default]
    None,
    /// A video: the OS's poster frame when it has one (L6), and how long it plays (L7).
    Video { poster: Option<String>, seconds: Option<u32> },
    /// An audio file and how long it plays (L8).
    Audio { seconds: Option<u32> },
    /// The first lines of a text or code file and the language that colours them (L9);
    /// `more` when the file goes on.
    Text { lines: String, lang: Option<String>, more: bool },
    /// A PDF or an EPUB as the import parsers read it (L10): its title and authors, and its
    /// pages (PDF) or chapters (EPUB).
    Document { title: Option<String>, creators: Vec<String>, pages: Option<usize>, chapters: Option<usize> },
}

impl FileScan {
    /// The preview of a file that is there.
    pub fn preview(&self) -> Option<&FilePreview> {
        match self {
            Self::Present(facts) => Some(&facts.preview),
            Self::Missing => None,
        }
    }
}
