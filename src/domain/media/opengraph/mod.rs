//! A tolerant reader of what a page says about itself: OpenGraph (`og:*`), Twitter cards
//! (`twitter:*`), `<title>`, the meta description and the site icon. Built on the shared
//! markup tokenizer, so malformed HTML never fails; it just yields less.

mod charset;
mod duration;
mod meta;

pub use charset::decode_html;
pub use duration::parse_iso_duration;

use super::preview::LinkPreview;
use crate::import::markup::first_element_text;
use crate::import::parse::normalize_whitespace;
use meta::{icon_href, MetaTags};
use url::Url;

/// Characters kept of a title and of a description.
const MAX_TITLE_CHARS: usize = 200;
const MAX_DESCRIPTION_CHARS: usize = 400;
/// Where the document head ends (everything a preview needs is above it).
const HEAD_END: &str = "</head";
/// Meta keys in order of preference.
const TITLE_KEYS: [&str; 2] = ["og:title", "twitter:title"];
const DESCRIPTION_KEYS: [&str; 3] = ["og:description", "twitter:description", "description"];
const SITE_KEYS: [&str; 2] = ["og:site_name", "application-name"];
const IMAGE_KEYS: [&str; 5] = ["og:image:secure_url", "og:image", "og:image:url", "twitter:image", "twitter:image:src"];
const VIDEO_KEYS: [&str; 4] = ["og:video", "og:video:url", "og:video:secure_url", "twitter:player"];
const SECONDS_KEYS: [&str; 2] = ["og:video:duration", "video:duration"];
const ISO_DURATION_KEY: &str = "duration";
const TYPE_KEY: &str = "og:type";
const VIDEO_TYPE_PREFIX: &str = "video";
const TITLE_ELEMENT: &str = "title";
/// The conventional icon location when the page names none.
const DEFAULT_ICON_PATH: &str = "/favicon.ico";

/// The preview of the page at `page_url` whose HTML is `html`.
pub fn parse_link_preview(html: &str, page_url: &str) -> LinkPreview {
    let head = head_of(html);
    let tags = MetaTags::read(head);
    let base = Url::parse(page_url).ok();
    let title = tags.first(&TITLE_KEYS).or_else(|| first_element_text(head, TITLE_ELEMENT)).unwrap_or_default();
    let declared_video = tags.first(&[TYPE_KEY]).is_some_and(|kind| kind.to_ascii_lowercase().starts_with(VIDEO_TYPE_PREFIX));
    let preview = LinkPreview {
        title: clip(&title, MAX_TITLE_CHARS),
        description: clip(&tags.first(&DESCRIPTION_KEYS).unwrap_or_default(), MAX_DESCRIPTION_CHARS),
        site_name: clip(&tags.first(&SITE_KEYS).unwrap_or_default(), MAX_TITLE_CHARS),
        image_url: tags.all(&IMAGE_KEYS).find_map(|raw| absolute_web_url(base.as_ref(), &raw)),
        icon_url: icon_href(head).and_then(|raw| absolute_web_url(base.as_ref(), &raw)).or_else(|| absolute_web_url(base.as_ref(), DEFAULT_ICON_PATH)),
        is_video: declared_video || tags.first(&VIDEO_KEYS).is_some(),
        duration_secs: tags
            .first(&SECONDS_KEYS)
            .and_then(|raw| raw.trim().parse::<u32>().ok())
            .or_else(|| tags.first(&[ISO_DURATION_KEY]).and_then(|raw| parse_iso_duration(&raw))),
    };
    preview
}

/// The document head (the whole text when it has no closing head tag).
fn head_of(html: &str) -> &str {
    let lower = html.to_ascii_lowercase();
    lower.find(HEAD_END).map_or(html, |end| &html[..end])
}

/// `raw` resolved against `base`, kept only when it is an `http(s)` URL.
fn absolute_web_url(base: Option<&Url>, raw: &str) -> Option<String> {
    let raw = raw.trim();
    let url = match base {
        Some(base) => base.join(raw).ok()?,
        None => Url::parse(raw).ok()?,
    };
    matches!(url.scheme(), "http" | "https").then(|| url.to_string())
}

/// Whitespace-normalised `text` cut to `max_chars`, with an ellipsis when cut.
fn clip(text: &str, max_chars: usize) -> String {
    let clean = normalize_whitespace(text);
    if clean.chars().count() <= max_chars {
        return clean;
    }
    let cut: String = clean.chars().take(max_chars).collect();
    format!("{}…", cut.trim_end())
}
