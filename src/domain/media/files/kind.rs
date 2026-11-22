//! L3: the type of a file, from its leading bytes first and then its extension. The import's
//! detector does the work; this tells apart the media containers it groups together (an
//! audio-only MP4, a QuickTime movie, WebM) and knows a few media extensions it does not.

use crate::import::detect::{extension_of, format_of_mime};
use crate::import::{detect_format, Format, FormatFamily};

/// Leading bytes read to tell a file's type.
pub const SNIFF_BYTES: usize = 8 * 1024;
/// The MIME type of a file whose type is unknown.
pub const UNKNOWN_MIME: &str = "application/octet-stream";
/// Offset of the major brand of an ISO media file (`ftyp` box).
const BRAND_AT: usize = 8;
const AUDIO_BRANDS: [&[u8]; 3] = [b"M4A ", b"M4B ", b"M4P "];
const QUICKTIME_BRAND: &[u8] = b"qt  ";
const MP4_AUDIO: &str = "audio/mp4";
const QUICKTIME: &str = "video/quicktime";
const WEBM: &str = "video/webm";
/// Extensions the import does not know, with their MIME types.
const MEDIA_EXTENSIONS: [(&str, &str); 9] = [
    ("mov", QUICKTIME), ("m4a", MP4_AUDIO), ("webm", WEBM), ("aac", "audio/aac"), ("opus", "audio/ogg"),
    ("oga", "audio/ogg"), ("ogv", "video/ogg"), ("svg", "image/svg+xml"), ("mpg", "video/mpeg"),
];
/// MIME prefixes naming a family when the exact type is not one the import knows.
const PREFIX_FAMILIES: [(&str, FormatFamily); 4] =
    [("image/", FormatFamily::Image), ("video/", FormatFamily::Video), ("audio/", FormatFamily::Audio), ("text/", FormatFamily::Text)];

/// The MIME type of a file called `name` whose first bytes are `head`.
pub fn detect_mime(name: &str, head: &[u8]) -> String {
    let extension = extension_of(name);
    let by_extension = || extension.as_deref().and_then(|ext| MEDIA_EXTENSIONS.iter().find(|(known, _)| *known == ext)).map(|(_, mime)| *mime);
    let mime = match detect_format(name, None, head) {
        Format::Mp4 => iso_media_mime(head).unwrap_or(Format::Mp4.mime()),
        Format::Matroska if extension.as_deref() == Some("webm") => WEBM,
        Format::Ogg => by_extension().unwrap_or(Format::Ogg.mime()),
        Format::Unknown => by_extension().unwrap_or(UNKNOWN_MIME),
        format => format.mime(),
    };
    mime.to_string()
}

/// The type an ISO media file's major brand names, when it is not a plain MP4 video.
fn iso_media_mime(head: &[u8]) -> Option<&'static str> {
    let brand = head.get(BRAND_AT..BRAND_AT + 4)?;
    if AUDIO_BRANDS.contains(&brand) {
        Some(MP4_AUDIO)
    } else {
        (brand == QUICKTIME_BRAND).then_some(QUICKTIME)
    }
}

/// The family of a MIME type (the card art, and which preview a file gets).
pub fn file_family(mime: &str) -> FormatFamily {
    let essence = mime.split(';').next().unwrap_or_default().trim().to_ascii_lowercase();
    format_of_mime(&essence)
        .map(Format::family)
        .or_else(|| PREFIX_FAMILIES.iter().find(|(prefix, _)| essence.starts_with(prefix)).map(|(_, family)| *family))
        .unwrap_or(FormatFamily::Unknown)
}
