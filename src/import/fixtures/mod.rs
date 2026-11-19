//! Tiny synthetic files built in memory for the unit tests: zip containers (EPUB, DOCX, ODT),
//! PalmDB/MOBI books with hand-written PalmDOC records, and a hand-made PDF.

mod mobi;
mod office;
mod pdf;

pub use mobi::{mobi_book, palmdoc_compress, MobiOptions};
pub use office::{docx, odt};
pub use pdf::tiny_pdf;

use std::io::{Cursor, Write};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

/// A zip archive of `(path, contents)`; the first entry is stored, the rest deflated.
pub fn zip_of(entries: &[(&str, &str)]) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    for (index, (path, contents)) in entries.iter().enumerate() {
        let method = if index == 0 { CompressionMethod::Stored } else { CompressionMethod::Deflated };
        writer.start_file(*path, SimpleFileOptions::default().compression_method(method)).expect("zip entry");
        writer.write_all(contents.as_bytes()).expect("zip write");
    }
    writer.finish().expect("zip finish").into_inner()
}

const CONTAINER: &str = r#"<?xml version="1.0"?><container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0"><rootfiles><rootfile full-path="epub/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#;

const NAV: &str = r#"<html xmlns:epub="http://www.idpf.org/2007/ops"><body><nav epub:type="landmarks"><a href="text/one.xhtml">Start</a></nav><nav epub:type="toc"><ol><li><a href="text/one.xhtml">I. The Beginning</a></li><li><a href="text/two%20b.xhtml#x">II. The End</a></li></ol></nav></body></html>"#;

fn chapter(heading: &str, paragraphs: &[&str]) -> String {
    let body: String = paragraphs.iter().map(|p| format!("<p>{p}</p>")).collect();
    format!(r#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en"><head><title>x</title></head><body><section><h2>{heading}</h2>{body}</section></body></html>"#)
}

/// A two-chapter EPUB 3 with Dublin Core metadata and a nav document.
pub fn epub() -> Vec<u8> {
    let opf = r#"<?xml version="1.0"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0"><metadata xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title id="t">A Tiny Book</dc:title><dc:creator>Ada Lovelace</dc:creator><dc:creator>Charles Babbage</dc:creator><dc:language>en-GB</dc:language><dc:date>1843-01-01</dc:date><dc:identifier>urn:uuid:1234</dc:identifier></metadata><manifest><item id="nav" href="toc.xhtml" media-type="application/xhtml+xml" properties="nav"/><item id="c2" href="text/two%20b.xhtml" media-type="application/xhtml+xml"/><item id="c1" href="text/one.xhtml" media-type="application/xhtml+xml"/></manifest><spine><itemref idref="c1"/><itemref idref="c2"/></spine></package>"#;
    let one = chapter("1", &["It was a dark night. Mr. Holmes sat still.", "The second paragraph."]);
    let two = chapter("2", &["Finally, the end."]);
    zip_of(&[
        ("mimetype", "application/epub+zip"),
        ("META-INF/container.xml", CONTAINER),
        ("epub/content.opf", opf),
        ("epub/toc.xhtml", NAV),
        ("epub/text/one.xhtml", &one),
        ("epub/text/two b.xhtml", &two),
    ])
}
