//! Every format the importer recognises, its stable id, MIME type and family.

/// A recognised file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Epub,
    /// MOBI, AZW and AZW3 (PalmDB with a MOBI or KF8 header).
    Mobi,
    Pdf,
    Docx,
    Odt,
    Html,
    Markdown,
    PlainText,
    Rtf,
    Csv,
    Tsv,
    Json,
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Mp3,
    Wav,
    Ogg,
    Flac,
    Mp4,
    Matroska,
    Avi,
    Zip,
    Tar,
    Gzip,
    SevenZip,
    Rar,
    Unknown,
}

/// Format families, one illustration card each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatFamily {
    Book,
    Pdf,
    Office,
    Web,
    Text,
    Data,
    Image,
    Audio,
    Video,
    Archive,
    Unknown,
}

impl FormatFamily {
    /// Every family, in display order.
    pub const ALL: [Self; 11] = [
        Self::Book, Self::Pdf, Self::Office, Self::Web, Self::Text, Self::Data, Self::Image, Self::Audio,
        Self::Video, Self::Archive, Self::Unknown,
    ];

    /// Stable id (illustration file and locale key).
    pub const fn id(self) -> &'static str {
        match self {
            Self::Book => "book",
            Self::Pdf => "pdf",
            Self::Office => "office",
            Self::Web => "web",
            Self::Text => "text",
            Self::Data => "data",
            Self::Image => "image",
            Self::Audio => "audio",
            Self::Video => "video",
            Self::Archive => "archive",
            Self::Unknown => "unknown",
        }
    }

    /// Whether files of this family carry readable text (and so units to post).
    pub const fn has_text(self) -> bool {
        !matches!(self, Self::Image | Self::Audio | Self::Video)
    }
}

impl Format {
    /// Short display name (a format name, not translated).
    pub const fn label(self) -> &'static str {
        match self {
            Self::Epub => "EPUB", Self::Mobi => "MOBI / AZW3", Self::Pdf => "PDF", Self::Docx => "DOCX",
            Self::Odt => "ODT", Self::Html => "HTML", Self::Markdown => "Markdown", Self::PlainText => "TXT",
            Self::Rtf => "RTF", Self::Csv => "CSV", Self::Tsv => "TSV", Self::Json => "JSON", Self::Png => "PNG",
            Self::Jpeg => "JPEG", Self::Gif => "GIF", Self::Webp => "WebP", Self::Bmp => "BMP", Self::Mp3 => "MP3",
            Self::Wav => "WAV", Self::Ogg => "OGG", Self::Flac => "FLAC", Self::Mp4 => "MP4",
            Self::Matroska => "MKV / WebM", Self::Avi => "AVI", Self::Zip => "ZIP", Self::Tar => "TAR",
            Self::Gzip => "GZIP", Self::SevenZip => "7Z", Self::Rar => "RAR", Self::Unknown => "BIN",
        }
    }

    /// MIME type.
    pub const fn mime(self) -> &'static str {
        match self {
            Self::Epub => "application/epub+zip", Self::Mobi => "application/x-mobipocket-ebook",
            Self::Pdf => "application/pdf",
            Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Self::Odt => "application/vnd.oasis.opendocument.text", Self::Html => "text/html",
            Self::Markdown => "text/markdown", Self::PlainText => "text/plain", Self::Rtf => "application/rtf",
            Self::Csv => "text/csv", Self::Tsv => "text/tab-separated-values", Self::Json => "application/json",
            Self::Png => "image/png", Self::Jpeg => "image/jpeg", Self::Gif => "image/gif", Self::Webp => "image/webp",
            Self::Bmp => "image/bmp", Self::Mp3 => "audio/mpeg", Self::Wav => "audio/wav", Self::Ogg => "audio/ogg",
            Self::Flac => "audio/flac", Self::Mp4 => "video/mp4", Self::Matroska => "video/x-matroska",
            Self::Avi => "video/x-msvideo", Self::Zip => "application/zip", Self::Tar => "application/x-tar",
            Self::Gzip => "application/gzip", Self::SevenZip => "application/x-7z-compressed",
            Self::Rar => "application/vnd.rar", Self::Unknown => "application/octet-stream",
        }
    }

    /// The family the format belongs to.
    pub const fn family(self) -> FormatFamily {
        match self {
            Self::Epub | Self::Mobi => FormatFamily::Book,
            Self::Pdf => FormatFamily::Pdf,
            Self::Docx | Self::Odt | Self::Rtf => FormatFamily::Office,
            Self::Html => FormatFamily::Web,
            Self::Markdown | Self::PlainText => FormatFamily::Text,
            Self::Csv | Self::Tsv | Self::Json => FormatFamily::Data,
            Self::Png | Self::Jpeg | Self::Gif | Self::Webp | Self::Bmp => FormatFamily::Image,
            Self::Mp3 | Self::Wav | Self::Ogg | Self::Flac => FormatFamily::Audio,
            Self::Mp4 | Self::Matroska | Self::Avi => FormatFamily::Video,
            Self::Zip | Self::Tar | Self::Gzip | Self::SevenZip | Self::Rar => FormatFamily::Archive,
            Self::Unknown => FormatFamily::Unknown,
        }
    }
}
