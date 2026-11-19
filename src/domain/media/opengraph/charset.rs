//! Text of a fetched page: the charset from the `Content-Type` header, else from a
//! `<meta charset>` near the top, else UTF-8 (invalid bytes replaced).

use encoding_rs::Encoding;

const CHARSET_PARAM: &str = "charset=";
/// Bytes searched for a `<meta charset>` declaration.
const SNIFF_BYTES: usize = 2048;
/// Characters ending a charset label.
const LABEL_END: [char; 5] = ['"', '\'', ';', ' ', '>'];

/// `bytes` decoded with the charset `content_type` or the page declares.
pub fn decode_html(bytes: &[u8], content_type: Option<&str>) -> String {
    let declared = content_type.and_then(charset_label).or_else(|| {
        let head = String::from_utf8_lossy(&bytes[..bytes.len().min(SNIFF_BYTES)]).to_ascii_lowercase();
        charset_label(&head)
    });
    let encoding = declared.and_then(|label| Encoding::for_label(label.as_bytes())).unwrap_or(encoding_rs::UTF_8);
    encoding.decode(bytes).0.into_owned()
}

/// The label after the first `charset=` in `text`.
fn charset_label(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let start = lower.find(CHARSET_PARAM)? + CHARSET_PARAM.len();
    let label: String = lower[start..].trim_start_matches(['"', '\'']).chars().take_while(|c| !LABEL_END.contains(c)).collect();
    (!label.is_empty()).then_some(label)
}
