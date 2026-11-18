//! The `<meta>` tags of a document head as `(key, content)` pairs, and the site icon link.

use crate::import::markup::tags_named;

/// Attributes naming a meta tag, in order (`property` for OpenGraph, `name` for the rest,
/// `itemprop` for schema.org durations).
const KEY_ATTRIBUTES: [&str; 3] = ["property", "name", "itemprop"];
const CONTENT_ATTRIBUTE: &str = "content";
const META_ELEMENT: &str = "meta";
const LINK_ELEMENT: &str = "link";
const REL_ATTRIBUTE: &str = "rel";
const HREF_ATTRIBUTE: &str = "href";
/// `rel` tokens of a site icon, best first.
const ICON_RELS: [&str; 3] = ["icon", "shortcut", "apple-touch-icon"];

/// Every keyed meta tag of a head, in document order.
pub(super) struct MetaTags(Vec<(String, String)>);

impl MetaTags {
    pub(super) fn read(head: &str) -> Self {
        let pairs = tags_named(head, META_ELEMENT)
            .filter_map(|tag| {
                let key = KEY_ATTRIBUTES.iter().find_map(|name| tag.attr(name))?;
                let content = tag.attr(CONTENT_ATTRIBUTE)?;
                let content = content.trim();
                (!content.is_empty()).then(|| (key.trim().to_ascii_lowercase(), content.to_string()))
            })
            .collect();
        Self(pairs)
    }

    /// Every value of any of `keys`, by key preference, then document order.
    pub(super) fn all<'a>(&'a self, keys: &'a [&'a str]) -> impl Iterator<Item = String> + 'a {
        keys.iter().flat_map(move |key| self.0.iter().filter(move |(k, _)| k == key).map(|(_, v)| v.clone()))
    }

    /// The first value of the most preferred of `keys` present.
    pub(super) fn first(&self, keys: &[&str]) -> Option<String> {
        self.all(keys).next()
    }
}

/// The `href` of the best site-icon `<link>` in `head`.
pub(super) fn icon_href(head: &str) -> Option<String> {
    let links: Vec<(String, String)> = tags_named(head, LINK_ELEMENT)
        .filter_map(|tag| Some((tag.attr(REL_ATTRIBUTE)?.to_ascii_lowercase(), tag.attr(HREF_ATTRIBUTE)?)))
        .filter(|(_, href)| !href.trim().is_empty())
        .collect();
    ICON_RELS.iter().find_map(|wanted| {
        links.iter().find(|(rel, _)| rel.split_whitespace().any(|token| token == *wanted)).map(|(_, href)| href.clone())
    })
}
