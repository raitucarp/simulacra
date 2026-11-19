//! Capped GET requests: a page (HTML parsed into a preview, or an image link) and raw bytes.

use crate::agent::tools::http::{checked_url, read_capped};
use crate::agent::tools::FetchPolicy;
use crate::domain::media::{decode_html, parse_link_preview, LinkPreview};
use reqwest::header::{ACCEPT, CONTENT_TYPE};

/// Bytes of a page read at most (the head, where previews live, comes first).
pub const MAX_PAGE_BYTES: usize = 512 * 1024;
/// Bytes of a preview image and of a site icon downloaded at most.
pub const MAX_IMAGE_BYTES: usize = 6 * 1024 * 1024;
pub const MAX_ICON_BYTES: usize = 256 * 1024;
/// What preview requests accept.
const PAGE_ACCEPT: &str = "text/html,application/xhtml+xml;q=0.9,image/*;q=0.8";
const IMAGE_ACCEPT: &str = "image/*";
/// Content-type fragments of pages and images.
const HTML_MARKERS: [&str; 2] = ["html", "xml"];
const IMAGE_MARKER: &str = "image/";

/// What a link led to.
pub(super) enum Page {
    Html(LinkPreview),
    /// The link is an image itself.
    Image,
}

/// Fetches `url` and reads it as a page.
pub(super) async fn fetch_page(client: &reqwest::Client, url: &str, allow_private: bool) -> Result<Page, String> {
    let (bytes, content_type) = get_capped(client, url, allow_private, MAX_PAGE_BYTES, PAGE_ACCEPT).await?;
    let kind = content_type.as_deref().unwrap_or_default().to_ascii_lowercase();
    if kind.starts_with(IMAGE_MARKER) {
        return Ok(Page::Image);
    }
    if !kind.is_empty() && !HTML_MARKERS.iter().any(|marker| kind.contains(marker)) {
        return Err(format!("not a page: {kind}"));
    }
    let html = decode_html(&bytes, content_type.as_deref());
    Ok(Page::Html(parse_link_preview(&html, url)))
}

/// Fetches up to `max_bytes` of the image at `url`.
pub(super) async fn fetch_capped(client: &reqwest::Client, url: &str, allow_private: bool, max_bytes: usize) -> Result<(Vec<u8>, Option<String>), String> {
    get_capped(client, url, allow_private, max_bytes, IMAGE_ACCEPT).await
}

/// A successful GET of an `http(s)` URL, its body cut at `max_bytes`, and its content type.
async fn get_capped(client: &reqwest::Client, url: &str, allow_private: bool, max_bytes: usize, accept: &str) -> Result<(Vec<u8>, Option<String>), String> {
    let policy = FetchPolicy { allow_private_hosts: allow_private, ..FetchPolicy::default() };
    let url = checked_url(url, &policy)?;
    let response = client.get(url).header(ACCEPT, accept).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("status {}", response.status()));
    }
    let content_type = response.headers().get(CONTENT_TYPE).and_then(|v| v.to_str().ok()).map(str::to_string);
    let (bytes, _cut) = read_capped(response, max_bytes).await?;
    Ok((bytes, content_type))
}
