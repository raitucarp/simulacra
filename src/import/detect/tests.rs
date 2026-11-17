use super::*;

fn palm_db(kind: &[u8; 8]) -> Vec<u8> {
    let mut bytes = vec![0u8; 78];
    bytes[60..68].copy_from_slice(kind);
    bytes
}

fn zip_with_mimetype(mime: &str) -> Vec<u8> {
    let mut bytes = b"PK\x03\x04".to_vec();
    bytes.resize(30, 0);
    bytes.extend_from_slice(b"mimetype");
    bytes.extend_from_slice(mime.as_bytes());
    bytes
}

#[test]
fn magic_bytes_win_over_the_extension() {
    assert_eq!(detect_format("notes.txt", None, b"%PDF-1.7\n..."), Format::Pdf);
    assert_eq!(detect_format("a.bin", None, b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR"), Format::Png);
    assert_eq!(detect_format("x", None, b"{\\rtf1\\ansi hello}"), Format::Rtf);
    assert_eq!(detect_format("song", None, b"ID3\x04\0\0\0\0\0\0"), Format::Mp3);
    assert_eq!(detect_format("clip", None, b"\0\0\0\x18ftypmp42"), Format::Mp4);
    assert_eq!(detect_format("pic", None, b"RIFF\0\0\0\0WEBPVP8 "), Format::Webp);
}

#[test]
fn palm_databases_are_mobi() {
    assert_eq!(detect_format("book.azw3", None, &palm_db(b"BOOKMOBI")), Format::Mobi);
    assert_eq!(detect_format("old.pdb", None, &palm_db(b"TEXtREAd")), Format::Mobi);
}

#[test]
fn zip_containers_are_told_apart() {
    assert_eq!(detect_format("b", None, &zip_with_mimetype("application/epub+zip")), Format::Epub);
    assert_eq!(detect_format("d", None, &zip_with_mimetype("application/vnd.oasis.opendocument.text")), Format::Odt);
    let mut docx = b"PK\x03\x04".to_vec();
    docx.extend_from_slice(b"....word/document.xml....");
    assert_eq!(detect_format("report", None, &docx), Format::Docx);
    assert_eq!(detect_format("plain.zip", None, b"PK\x03\x04 other"), Format::Zip);
    assert_eq!(detect_format("odd.epub", None, b"PK\x03\x04 other"), Format::Epub);
}

#[test]
fn text_formats_follow_the_extension_or_the_content() {
    assert_eq!(detect_format("a.md", None, b"# Title\n\nBody"), Format::Markdown);
    assert_eq!(detect_format("a.csv", None, b"a,b\n1,2"), Format::Csv);
    assert_eq!(detect_format("a.tsv", None, b"a\tb\n1\t2"), Format::Tsv);
    assert_eq!(detect_format("page", None, b"<!DOCTYPE html><html><body>x</body></html>"), Format::Html);
    assert_eq!(detect_format("data", None, br#"{"title": "x"}"#), Format::Json);
    assert_eq!(detect_format("note", None, "Just a thought — ünïcode".as_bytes()), Format::PlainText);
    assert_eq!(detect_format("clip", Some("text/markdown; charset=utf-8"), b"Body"), Format::Markdown);
}

#[test]
fn binary_without_a_signature_is_unknown() {
    let noise: Vec<u8> = (0..512u32).map(|i| (i.wrapping_mul(2654435761) >> 13) as u8).collect();
    assert_eq!(detect_format("random.bin", None, &noise), Format::Unknown);
    assert_eq!(detect_format("empty", None, b""), Format::Unknown);
    assert_eq!(Format::Unknown.family(), FormatFamily::Unknown);
}

#[test]
fn every_format_has_a_mime_and_a_family() {
    for format in [Format::Epub, Format::Mobi, Format::Docx, Format::Flac, Format::Rar] {
        assert!(format.mime().contains('/'));
        assert!(FormatFamily::ALL.contains(&format.family()));
    }
    assert_eq!(extension_of("C:\\books\\Moby.Dick.EPUB").as_deref(), Some("epub"));
    assert_eq!(extension_of("README"), None);
}
