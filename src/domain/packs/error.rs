//! Why a pack is refused.

/// The manifest rule a value breaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackRule {
    /// Ids are plain names (`[a-z0-9_-]`) and `classic` is reserved.
    Id,
    /// A pack has a name and a version.
    Name,
    Version,
    /// Keys are relative illustration paths (`empty/no_posts`).
    Key,
    /// Files are relative `.webp`/`.png` paths inside the pack.
    File,
    /// Checksums are lowercase hex SHA-256 digests.
    Checksum,
    /// Images are between one pixel and the size cap on each side.
    Size,
}

/// Why a pack (or its archive) was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackError {
    /// `pack.json` is not JSON of the expected shape.
    Manifest(String),
    /// A value breaks a manifest rule.
    Invalid(PackRule, String),
    /// The archive lacks `pack.json` or a file the manifest lists.
    Missing(String),
    /// A file does not match the SHA-256 its manifest pins.
    Checksum(String),
    /// Not a readable `.simpack` archive.
    Archive(String),
    /// More files or bytes than a pack may hold.
    TooLarge,
    /// The id belongs to a pack shipped with the app.
    BuiltIn(String),
    /// Reading or writing the pack store failed.
    Io(String),
}

impl PackError {
    /// The value the error names (a file, key or id), or the underlying message.
    pub fn detail(&self) -> &str {
        match self {
            Self::Manifest(detail) | Self::Invalid(_, detail) | Self::Missing(detail) | Self::Checksum(detail) => detail,
            Self::Archive(detail) | Self::BuiltIn(detail) | Self::Io(detail) => detail,
            Self::TooLarge => "",
        }
    }
}
