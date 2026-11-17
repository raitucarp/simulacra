use super::*;
use crate::import::detect::Format;
use crate::import::fixtures::zip_of;

#[test]
fn checksums_are_hex_sha256() {
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn unknown_binaries_yield_their_printable_strings() {
    let mut bytes: Vec<u8> = (0..64u8).map(|i| i.wrapping_mul(37) | 0x80).collect();
    bytes.extend_from_slice(b"\x00Copyright 1999 Example Labs\x01\x02short\x03/usr/lib/libthing.so.1\x00");
    let content = parse_unknown(&bytes);
    let text: Vec<&str> = content.sections[0].blocks.iter().map(|b| b.text.as_str()).collect();
    assert_eq!(text, ["Copyright 1999 Example Labs\n/usr/lib/libthing.so.1"]);
    assert_eq!(content.notes, [ParseNote::ExtractedStrings(2)]);
    assert!(parse_unknown(&[0u8; 16]).sections.is_empty());
}

#[test]
fn image_headers_give_pixel_sizes() {
    let mut png = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
    png.extend_from_slice(&640u32.to_be_bytes());
    png.extend_from_slice(&480u32.to_be_bytes());
    assert_eq!(image_dimensions(Format::Png, &png), Some((640, 480)));
    assert_eq!(image_dimensions(Format::Gif, b"GIF89a\x20\x00\x10\x00"), Some((32, 16)));
    let jpeg = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x04, 0x00, 0x00, 0xFF, 0xC0, 0x00, 0x11, 0x08, 0x01, 0x00, 0x02, 0x00];
    assert_eq!(image_dimensions(Format::Jpeg, &jpeg), Some((512, 256)));
    let note = parse_image(Format::Png, &png).notes;
    assert_eq!(note, [ParseNote::Dimensions { width: 640, height: 480 }]);
    assert_eq!(image_dimensions(Format::Png, b"short"), None);
}

#[test]
fn archives_list_their_entries() {
    let zip = zip_of(&[("notes/a.txt", "a"), ("notes/b.md", "b")]);
    let content = parse_archive(Format::Zip, &zip).expect("zip lists");
    let names: Vec<&str> = content.sections[0].blocks.iter().map(|b| b.text.as_str()).collect();
    assert_eq!(names, ["notes/a.txt", "notes/b.md"]);
    assert_eq!(content.notes, [ParseNote::ArchiveEntries(2)]);
}

#[test]
fn tar_headers_and_gzip_names_are_read() {
    let mut tar = vec![0u8; 512 * 3];
    tar[..9].copy_from_slice(b"hello.txt");
    tar[124..135].copy_from_slice(b"00000000005");
    tar[156] = b'0';
    tar[257..262].copy_from_slice(b"ustar");
    assert_eq!(archive::tar_entries(&tar), [("hello.txt".to_string(), 5)]);
    let gzip = b"\x1F\x8B\x08\x08\0\0\0\0\0\x03draft.md\0rest";
    assert_eq!(archive::gzip_name(gzip).as_deref(), Some("draft.md"));
}
