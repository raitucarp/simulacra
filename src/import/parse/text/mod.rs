//! Text formats: Markdown, plain text, RTF, CSV/TSV and JSON, plus the shared byte decoding
//! (UTF-8, UTF-16 with a byte-order mark, else Windows-1252).

mod delimited;
mod json;
mod markdown;
mod plain;
mod rtf;

#[cfg(test)]
mod tests;

pub use delimited::parse_delimited;
pub use json::parse_json;
pub use markdown::parse_markdown;
pub use plain::parse_plain;
pub use rtf::parse_rtf;

const UTF8_BOM: &[u8] = b"\xEF\xBB\xBF";
const UTF16_LE_BOM: &[u8] = b"\xFF\xFE";
const UTF16_BE_BOM: &[u8] = b"\xFE\xFF";

/// Decodes text bytes: a byte-order mark decides, then UTF-8, then Windows-1252.
pub fn decode_text(bytes: &[u8]) -> String {
    if let Some(rest) = bytes.strip_prefix(UTF8_BOM) {
        return String::from_utf8_lossy(rest).into_owned();
    }
    if bytes.starts_with(UTF16_LE_BOM) {
        return encoding_rs::UTF_16LE.decode(bytes).0.into_owned();
    }
    if bytes.starts_with(UTF16_BE_BOM) {
        return encoding_rs::UTF_16BE.decode(bytes).0.into_owned();
    }
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_string(),
        Err(_) => encoding_rs::WINDOWS_1252.decode(bytes).0.into_owned(),
    }
}
