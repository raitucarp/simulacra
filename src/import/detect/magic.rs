//! Format detection from leading magic bytes (and, for zip containers, the names inside).

use super::Format;

/// Offset of the PalmDB type and creator (`BOOKMOBI`, `TEXtREAd`).
const PALM_TYPE_OFFSET: usize = 60;
const PALM_TYPES: [&[u8]; 2] = [b"BOOKMOBI", b"TEXtREAd"];
/// Offset of the `ustar` signature in a tar header.
const TAR_MAGIC_OFFSET: usize = 257;
/// Offset of the container `mimetype` text in the first local zip header (EPUB, ODF).
const ZIP_FIRST_DATA_OFFSET: usize = 30;
const EPUB_MIME: &[u8] = b"application/epub+zip";
const ODT_MIME: &[u8] = b"application/vnd.oasis.opendocument.text";
const DOCX_MARKER: &[u8] = b"word/document.xml";

/// `(signature, offset, format)` checked in order.
const SIGNATURES: [(&[u8], usize, Format); 20] = [
    (b"%PDF-", 0, Format::Pdf),
    (b"{\\rtf", 0, Format::Rtf),
    (b"\x89PNG\r\n\x1a\n", 0, Format::Png),
    (b"\xFF\xD8\xFF", 0, Format::Jpeg),
    (b"GIF87a", 0, Format::Gif),
    (b"GIF89a", 0, Format::Gif),
    (b"WEBP", 8, Format::Webp),
    (b"WAVE", 8, Format::Wav),
    (b"AVI ", 8, Format::Avi),
    (b"BM", 0, Format::Bmp),
    (b"ID3", 0, Format::Mp3),
    (b"OggS", 0, Format::Ogg),
    (b"fLaC", 0, Format::Flac),
    (b"ftyp", 4, Format::Mp4),
    (b"\x1A\x45\xDF\xA3", 0, Format::Matroska),
    (b"\x1F\x8B", 0, Format::Gzip),
    (b"7z\xBC\xAF\x27\x1C", 0, Format::SevenZip),
    (b"Rar!\x1A\x07", 0, Format::Rar),
    (b"ustar", TAR_MAGIC_OFFSET, Format::Tar),
    (b"PK\x03\x04", 0, Format::Zip),
];

fn has_at(bytes: &[u8], offset: usize, signature: &[u8]) -> bool {
    bytes.get(offset..offset + signature.len()) == Some(signature)
}

fn contains(bytes: &[u8], needle: &[u8]) -> bool {
    bytes.windows(needle.len()).any(|window| window == needle)
}

/// The format named by the bytes' signature, if any. Zip containers are told apart by their
/// `mimetype` entry (EPUB, ODT) or their main part (DOCX).
pub fn sniff_magic(bytes: &[u8]) -> Option<Format> {
    if PALM_TYPES.iter().any(|kind| has_at(bytes, PALM_TYPE_OFFSET, kind)) {
        return Some(Format::Mobi);
    }
    let found = SIGNATURES.iter().find(|(signature, offset, _)| has_at(bytes, *offset, signature));
    found.map(|(_, _, format)| match format {
        Format::Zip => zip_flavour(bytes),
        Format::Bmp if bytes.len() < 26 => Format::Unknown,
        other => *other,
    })
}

fn zip_flavour(bytes: &[u8]) -> Format {
    if has_at(bytes, ZIP_FIRST_DATA_OFFSET + "mimetype".len(), EPUB_MIME) {
        Format::Epub
    } else if has_at(bytes, ZIP_FIRST_DATA_OFFSET + "mimetype".len(), ODT_MIME) {
        Format::Odt
    } else if contains(bytes, DOCX_MARKER) {
        Format::Docx
    } else if contains(bytes, b"META-INF/container.xml") && contains(bytes, EPUB_MIME) {
        Format::Epub
    } else {
        Format::Zip
    }
}

/// Whether an MP3 frame sync starts the bytes (files without an ID3 tag).
pub fn is_mpeg_audio(bytes: &[u8]) -> bool {
    matches!(bytes, [0xFF, second, ..] if second & 0xE0 == 0xE0 && second & 0x06 != 0)
}
