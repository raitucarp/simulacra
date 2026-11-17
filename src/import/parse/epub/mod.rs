//! EPUB 2 and 3: the container points at the OPF package; its spine gives the reading order,
//! its nav (or NCX) the chapter titles, and every XHTML chapter becomes one section.

mod opf;
mod toc;

#[cfg(test)]
mod tests;

use super::container::ZipBundle;
use super::html::{html_blocks, HtmlContent};
use super::sections::sections_from_blocks;
use super::{Content, ParseError, ParseResult};
use crate::import::document::Section;
use opf::Package;
use toc::ChapterTitles;

const CONTAINER_PATH: &str = "META-INF/container.xml";
/// Present only in DRM-protected books.
const RIGHTS_PATH: &str = "META-INF/rights.xml";

/// Parses an EPUB container.
pub fn parse(bytes: &[u8]) -> ParseResult {
    let mut zip = ZipBundle::open(bytes)?;
    if zip.read(RIGHTS_PATH).is_some() {
        return Err(ParseError::Encrypted);
    }
    let container = zip.read_text(CONTAINER_PATH).ok_or_else(|| missing(CONTAINER_PATH))?;
    let opf_path = opf::rootfile_path(&container).ok_or_else(|| missing("rootfile"))?;
    let opf_src = zip.read_text(&opf_path).ok_or_else(|| missing(&opf_path))?;
    let package = opf::parse_package(&opf_src, &opf_path);
    let titles = chapter_titles(&mut zip, &package);
    let mut language = None;
    let sections: Vec<Section> = package
        .spine
        .iter()
        .flat_map(|path| {
            let html = html_blocks(&zip.read_text(path).unwrap_or_default());
            language = language.take().or(html.language.clone());
            spine_sections(html, titles.get(path).cloned())
        })
        .collect();
    let mut metadata = package.metadata;
    metadata.language = metadata.language.or(language);
    Ok(Content { metadata, sections, notes: Vec::new() })
}

/// The sections of one spine document: one per top-level heading when it holds several
/// (a poetry collection in one file), else the whole document under its contents title.
fn spine_sections(html: HtmlContent, toc_title: Option<String>) -> Vec<Section> {
    let first_heading = html.blocks.iter().find(|block| block.is_heading()).map(|block| block.text.clone());
    let fallback = toc_title.or(first_heading).or(html.title);
    let mut parts = sections_from_blocks(html.blocks);
    if parts.len() == 1 {
        parts[0].title = fallback;
    } else if let Some(untitled) = parts.first_mut().filter(|part| part.title.is_none()) {
        untitled.title = fallback;
    }
    parts
}

fn missing(what: &str) -> ParseError {
    ParseError::Unreadable(format!("missing {what}"))
}

/// Titles from the nav document, else from the NCX.
fn chapter_titles(zip: &mut ZipBundle, package: &Package) -> ChapterTitles {
    let from_nav = package
        .nav_item()
        .and_then(|item| zip.read_text(&item.path).map(|nav| toc::nav_titles(&nav, &item.path)))
        .filter(|titles| !titles.is_empty());
    from_nav
        .or_else(|| package.ncx_item().and_then(|item| zip.read_text(&item.path).map(|ncx| toc::ncx_titles(&ncx, &item.path))))
        .unwrap_or_default()
}
