use super::*;
use crate::import::fixtures;

#[test]
fn reads_metadata_spine_order_and_nav_titles() {
    let content = parse(&fixtures::epub()).expect("epub parses");
    let meta = &content.metadata;
    assert_eq!(meta.title.as_deref(), Some("A Tiny Book"));
    assert_eq!(meta.creators, ["Ada Lovelace", "Charles Babbage"]);
    assert_eq!(meta.language.as_deref(), Some("en-GB"));
    assert_eq!(meta.date.as_deref(), Some("1843-01-01"));
    assert_eq!(meta.identifiers, ["urn:uuid:1234"]);
    let titles: Vec<Option<&str>> = content.sections.iter().map(|s| s.title.as_deref()).collect();
    assert_eq!(titles, [Some("I. The Beginning"), Some("II. The End")]);
    let first: Vec<&str> = content.sections[0].body().map(|b| b.text.as_str()).collect();
    assert_eq!(first, ["It was a dark night. Mr. Holmes sat still.", "The second paragraph."]);
}

#[test]
fn ncx_titles_name_chapters_without_a_nav() {
    let ncx = r#"<ncx><navMap><navPoint><navLabel><text>Part One</text></navLabel><content src="text/p1.xhtml"/>
        <navPoint><navLabel><text>Chapter 1</text></navLabel><content src="text/c1.xhtml#a"/></navPoint></navPoint></navMap></ncx>"#;
    let titles = toc::ncx_titles(ncx, "OEBPS/toc.ncx");
    assert_eq!(titles.get("OEBPS/text/p1.xhtml").map(String::as_str), Some("Part One"));
    assert_eq!(titles.get("OEBPS/text/c1.xhtml").map(String::as_str), Some("Chapter 1"));
}

#[test]
fn a_zip_that_is_not_an_epub_is_an_error() {
    let not_epub = fixtures::zip_of(&[("readme.txt", "hello")]);
    assert!(matches!(parse(&not_epub), Err(ParseError::Unreadable(_))));
    assert!(parse(b"garbage").is_err());
}

#[test]
fn rights_file_means_drm() {
    let locked = fixtures::zip_of(&[("mimetype", "application/epub+zip"), ("META-INF/rights.xml", "<rights/>")]);
    assert_eq!(parse(&locked), Err(ParseError::Encrypted));
}

#[test]
fn a_collection_in_one_file_splits_at_its_poem_titles() {
    let html = html_blocks("<body><p>Preface line.</p><h2>Mending Wall</h2><p>Something there is</p><h2>The Mountain</h2><p>The mountain held</p></body>");
    let sections = spine_sections(html, Some("North of Boston".into()));
    let titles: Vec<Option<&str>> = sections.iter().map(|s| s.title.as_deref()).collect();
    assert_eq!(titles, [Some("North of Boston"), Some("Mending Wall"), Some("The Mountain")]);
    let single = spine_sections(html_blocks("<h2>I</h2><p>Call me Ishmael.</p>"), Some("Loomings".into()));
    assert_eq!(single.len(), 1);
    assert_eq!(single[0].title.as_deref(), Some("Loomings"));
}
