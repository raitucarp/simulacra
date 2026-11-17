//! The resource type of a document (the Dublin Core "type" element).

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// What kind of resource a document is.
    DocumentKind {
        Book => "book",
        Article => "article",
        Note => "note",
        WebPage => "web_page",
        Transcript => "transcript",
        Dataset => "dataset",
        Media => "media",
        Other => "other",
    }
}

/// MIME types used as the default format of each kind.
const TEXT_PLAIN: &str = "text/plain";
const TEXT_MARKDOWN: &str = "text/markdown";
const TEXT_HTML: &str = "text/html";
const TEXT_CSV: &str = "text/csv";
/// Folder of the kind illustrations (`assets/illustrations/doc_kind/<key>.svg`).
const KIND_ART_DIR: &str = "illustrations/doc_kind";

impl Default for DocumentKind {
    fn default() -> Self {
        Self::Note
    }
}

impl DocumentKind {
    /// The format assumed when none was recorded.
    pub const fn default_format(self) -> &'static str {
        match self {
            Self::Note => TEXT_MARKDOWN,
            Self::WebPage => TEXT_HTML,
            Self::Dataset => TEXT_CSV,
            Self::Book | Self::Article | Self::Transcript | Self::Media | Self::Other => TEXT_PLAIN,
        }
    }

    /// Asset path of this kind's illustration.
    pub fn illustration(self) -> String {
        format!("{KIND_ART_DIR}/{}.svg", self.key())
    }

    /// Whether the kind is usually cited with its title in italics (a whole work) rather
    /// than in quotation marks (a part of something larger).
    pub const fn is_whole_work(self) -> bool {
        matches!(self, Self::Book | Self::Dataset | Self::Media | Self::Other)
    }
}
