//! Format hints from the file name extension, a MIME type and the shape of text content.

use super::Format;

/// Extensions (lower case, no dot) and the format they name.
const EXTENSIONS: [(&str, Format); 38] = [
    ("epub", Format::Epub), ("mobi", Format::Mobi), ("azw", Format::Mobi), ("azw3", Format::Mobi),
    ("prc", Format::Mobi), ("pdf", Format::Pdf), ("docx", Format::Docx), ("odt", Format::Odt),
    ("html", Format::Html), ("htm", Format::Html), ("xhtml", Format::Html), ("md", Format::Markdown),
    ("markdown", Format::Markdown), ("txt", Format::PlainText), ("text", Format::PlainText),
    ("log", Format::PlainText), ("rtf", Format::Rtf), ("csv", Format::Csv), ("tsv", Format::Tsv),
    ("tab", Format::Tsv), ("json", Format::Json), ("png", Format::Png), ("jpg", Format::Jpeg),
    ("jpeg", Format::Jpeg), ("gif", Format::Gif), ("webp", Format::Webp), ("bmp", Format::Bmp),
    ("mp3", Format::Mp3), ("wav", Format::Wav), ("ogg", Format::Ogg), ("flac", Format::Flac),
    ("mp4", Format::Mp4), ("m4v", Format::Mp4), ("mkv", Format::Matroska), ("webm", Format::Matroska),
    ("zip", Format::Zip), ("tar", Format::Tar), ("gz", Format::Gzip),
];

/// Bytes sampled when deciding whether content is text.
const TEXT_SAMPLE_BYTES: usize = 4096;
/// Share of control characters above which a sample is binary.
const MAX_CONTROL_SHARE: f32 = 0.02;

/// The lower-case extension of `name` (without the dot).
pub fn extension_of(name: &str) -> Option<String> {
    let file = name.rsplit(['/', '\\']).next().unwrap_or(name);
    file.rsplit_once('.').map(|(_, ext)| ext.to_ascii_lowercase()).filter(|ext| !ext.is_empty())
}

/// The format an extension names.
pub fn format_of_extension(extension: &str) -> Option<Format> {
    EXTENSIONS.iter().find(|(ext, _)| *ext == extension).map(|(_, format)| *format)
}

/// The format a MIME type names (parameters such as `; charset=` are ignored).
pub fn format_of_mime(mime: &str) -> Option<Format> {
    let essence = mime.split(';').next().unwrap_or_default().trim().to_ascii_lowercase();
    let known = EXTENSIONS.iter().map(|(_, format)| *format).find(|format| format.mime() == essence);
    known.or(match essence.as_str() {
        "application/xhtml+xml" => Some(Format::Html),
        "text/x-markdown" => Some(Format::Markdown),
        "text/rtf" => Some(Format::Rtf),
        _ => None,
    })
}

/// Whether the leading bytes look like text (valid UTF-8, almost no control characters).
pub fn looks_like_text(bytes: &[u8]) -> bool {
    let sample = &bytes[..bytes.len().min(TEXT_SAMPLE_BYTES)];
    let text = match std::str::from_utf8(sample) {
        Ok(text) => text,
        Err(err) if err.valid_up_to() + 4 >= sample.len() => sample_prefix(sample, err.valid_up_to()),
        Err(_) => return false,
    };
    let controls = text.chars().filter(|c| c.is_control() && !c.is_whitespace()).count();
    let total = text.chars().count().max(1);
    (controls as f32 / total as f32) <= MAX_CONTROL_SHARE
}

/// The valid UTF-8 prefix of a sample cut in the middle of a character.
fn sample_prefix(sample: &[u8], valid: usize) -> &str {
    std::str::from_utf8(&sample[..valid]).unwrap_or_default()
}

/// The text format suggested by the content itself.
pub fn sniff_text(bytes: &[u8]) -> Format {
    let head = String::from_utf8_lossy(&bytes[..bytes.len().min(TEXT_SAMPLE_BYTES)]).to_ascii_lowercase();
    let trimmed = head.trim_start_matches('\u{feff}').trim_start();
    if trimmed.starts_with("<!doctype html") || trimmed.starts_with("<html") || head.contains("<body") {
        Format::Html
    } else if (trimmed.starts_with('{') || trimmed.starts_with('['))
        && serde_json::from_slice::<serde_json::Value>(bytes).is_ok()
    {
        Format::Json
    } else if trimmed.starts_with("# ") || head.contains("\n## ") {
        Format::Markdown
    } else {
        Format::PlainText
    }
}
