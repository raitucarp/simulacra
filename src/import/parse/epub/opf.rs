//! The OPF package document: Dublin Core metadata, the manifest and the spine.

use crate::import::document::DocumentMetadata;
use crate::import::markup::{element_texts, first_element_text, tags_named};
use crate::import::parse::container::resolve_path;

/// One manifest item, its path already resolved inside the container.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestItem {
    pub id: String,
    pub path: String,
    pub media_type: String,
    pub properties: String,
}

/// What the importer needs from the package document.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Package {
    pub metadata: DocumentMetadata,
    pub manifest: Vec<ManifestItem>,
    /// Container paths of the spine items, in reading order.
    pub spine: Vec<String>,
    /// Manifest id of the NCX table of contents (EPUB 2).
    pub ncx_id: Option<String>,
}

impl Package {
    /// The EPUB 3 navigation document.
    pub fn nav_item(&self) -> Option<&ManifestItem> {
        self.manifest.iter().find(|item| item.properties.split_whitespace().any(|p| p == "nav"))
    }

    /// The EPUB 2 NCX.
    pub fn ncx_item(&self) -> Option<&ManifestItem> {
        let by_id = self.ncx_id.as_ref().and_then(|id| self.manifest.iter().find(|item| &item.id == id));
        by_id.or_else(|| self.manifest.iter().find(|item| item.media_type == "application/x-dtbncx+xml"))
    }
}

/// The package document path named by `META-INF/container.xml`.
pub fn rootfile_path(container_xml: &str) -> Option<String> {
    tags_named(container_xml, "rootfile").find_map(|tag| tag.attr("full-path"))
}

/// Reads the package document found at `opf_path`.
pub fn parse_package(opf: &str, opf_path: &str) -> Package {
    let manifest: Vec<ManifestItem> = tags_named(opf, "item")
        .filter_map(|tag| {
            let href = tag.attr("href")?;
            Some(ManifestItem {
                id: tag.attr("id").unwrap_or_default(),
                path: resolve_path(opf_path, &href),
                media_type: tag.attr("media-type").unwrap_or_default(),
                properties: tag.attr("properties").unwrap_or_default(),
            })
        })
        .collect();
    let spine = tags_named(opf, "itemref")
        .filter_map(|tag| tag.attr("idref"))
        .filter_map(|idref| manifest.iter().find(|item| item.id == idref).map(|item| item.path.clone()))
        .collect();
    let ncx_id = tags_named(opf, "spine").find_map(|tag| tag.attr("toc"));
    Package { metadata: metadata(opf), manifest, spine, ncx_id }
}

fn metadata(opf: &str) -> DocumentMetadata {
    DocumentMetadata {
        title: first_element_text(opf, "title"),
        creators: element_texts(opf, "creator"),
        language: first_element_text(opf, "language"),
        date: first_element_text(opf, "date"),
        identifiers: element_texts(opf, "identifier"),
        publisher: first_element_text(opf, "publisher"),
        description: first_element_text(opf, "description"),
    }
}
