//! Pure format detection: magic bytes first, then the extension or MIME type, then the shape
//! of the content. Anything else is [`Format::Unknown`], which still imports as a binary.

mod format;
mod magic;
mod name;

#[cfg(test)]
mod tests;

pub use format::{Format, FormatFamily};
pub use name::{extension_of, format_of_mime};

/// Formats that live inside a zip container.
const ZIP_BASED: [Format; 3] = [Format::Epub, Format::Docx, Format::Odt];

/// The format of a file called `name` with the given bytes and optional MIME type.
pub fn detect_format(name: &str, mime_hint: Option<&str>, bytes: &[u8]) -> Format {
    let hinted = name::extension_of(name)
        .and_then(|ext| name::format_of_extension(&ext))
        .or_else(|| mime_hint.and_then(name::format_of_mime));
    match magic::sniff_magic(bytes) {
        Some(Format::Zip) => hinted.filter(|format| ZIP_BASED.contains(format)).unwrap_or(Format::Zip),
        Some(found) => found,
        None if magic::is_mpeg_audio(bytes) => Format::Mp3,
        None => by_content(hinted, bytes),
    }
}

/// Formats without a signature: trust a text hint when the bytes are text, sniff otherwise.
fn by_content(hinted: Option<Format>, bytes: &[u8]) -> Format {
    let is_text = name::looks_like_text(bytes);
    match hinted {
        Some(format) if is_text && is_text_format(format) => format,
        _ if is_text && !bytes.is_empty() => name::sniff_text(bytes),
        _ => Format::Unknown,
    }
}

fn is_text_format(format: Format) -> bool {
    matches!(
        format,
        Format::Html | Format::Markdown | Format::PlainText | Format::Csv | Format::Tsv | Format::Json | Format::Rtf
    )
}
