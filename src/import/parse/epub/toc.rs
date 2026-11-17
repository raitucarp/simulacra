//! Chapter titles from the table of contents: the EPUB 3 `nav` document, else the EPUB 2 NCX.
//! The first entry pointing into a file names that file.

use crate::import::markup::{decode_entities, tokenize, Token};
use crate::import::parse::container::resolve_path;
use crate::import::parse::normalize_whitespace;
use std::collections::HashMap;

/// Container path → chapter title.
pub type ChapterTitles = HashMap<String, String>;

fn insert_first(titles: &mut ChapterTitles, path: String, title: String) {
    if !title.is_empty() {
        titles.entry(path).or_insert(title);
    }
}

/// Titles from a navigation document at `nav_path`: the links inside its `toc` nav (or its
/// first nav when none is typed).
pub fn nav_titles(nav: &str, nav_path: &str) -> ChapterTitles {
    let has_typed_toc = nav.contains("\"toc\"") || nav.contains("'toc'");
    let mut titles = ChapterTitles::new();
    let (mut in_toc, mut link): (bool, Option<(String, String)>) = (false, None);
    for token in tokenize(nav) {
        match token {
            Token::Open(tag) if tag.is("nav") => {
                let is_toc = tag.attr("epub:type").is_some_and(|kind| kind.split_whitespace().any(|k| k == "toc"));
                in_toc = is_toc || (!has_typed_toc && titles.is_empty());
            }
            Token::Close(name) if name.trim().eq_ignore_ascii_case("nav") => in_toc = false,
            Token::Open(tag) if in_toc && tag.is("a") => {
                link = tag.attr("href").map(|href| (resolve_path(nav_path, &href), String::new()));
            }
            Token::Text(text) => {
                if let Some((_, title)) = link.as_mut() {
                    title.push_str(&decode_entities(text));
                }
            }
            token if token.closes("a") => {
                if let Some((path, title)) = link.take() {
                    insert_first(&mut titles, path, normalize_whitespace(&title));
                }
            }
            _ => {}
        }
    }
    titles
}

/// Titles from an NCX at `ncx_path`: each `navPoint`'s label text and content source.
pub fn ncx_titles(ncx: &str, ncx_path: &str) -> ChapterTitles {
    let mut titles = ChapterTitles::new();
    let (mut label, mut in_text) = (String::new(), false);
    for token in tokenize(ncx) {
        match token {
            Token::Open(tag) if tag.is("navPoint") => label.clear(),
            Token::Open(tag) if tag.is("text") => in_text = !tag.self_closing,
            Token::Text(text) if in_text => label.push_str(&decode_entities(text)),
            Token::Open(tag) if tag.is("content") => {
                if let Some(src) = tag.attr("src") {
                    insert_first(&mut titles, resolve_path(ncx_path, &src), normalize_whitespace(&label));
                }
            }
            token if token.closes("text") => in_text = false,
            _ => {}
        }
    }
    titles
}
