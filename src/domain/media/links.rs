//! The web links of a post: its attached link first, then every `http(s)` URL written in its
//! text, cleaned of trailing punctuation and de-duplicated in order.

use url::Url;

/// Schemes a link may use (the only ones ever fetched or opened).
const WEB_SCHEMES: [&str; 2] = ["https://", "http://"];
/// Characters that end a URL written in prose.
const URL_STOPPERS: [char; 6] = ['<', '>', '"', '`', '\u{a0}', '|'];
/// Punctuation trimmed off the end of a URL in prose ("see https://a.org/x."), Latin and CJK.
const TRAILING_PUNCTUATION: [char; 20] = [
    '.', ',', ';', ':', '!', '?', '\'', '"', '…', '»', '«', '”', '’', '。', '、', '，', '！', '？', '」', '』',
];
/// Closing brackets kept only when the URL also opens them (`/wiki/Rust_(language)`).
const BRACKET_PAIRS: [(char, char); 3] = [('(', ')'), ('[', ']'), ('{', '}')];
/// Links taken from one post at most.
pub const MAX_LINKS_PER_POST: usize = 8;
/// Conventional subdomain dropped from display domains.
const WWW_PREFIX: &str = "www.";

/// Every `http(s)` URL in `text`, in order of appearance, without duplicates.
pub fn detect_urls(text: &str) -> Vec<String> {
    let lower = text.to_ascii_lowercase();
    let mut found: Vec<String> = Vec::new();
    let mut at = 0;
    while let Some((start, scheme)) = next_scheme(&lower, at) {
        let tail = &text[start..];
        let end = tail.find(|c: char| c.is_whitespace() || URL_STOPPERS.contains(&c)).unwrap_or(tail.len());
        let candidate = trim_url(&tail[..end]);
        at = start + scheme.max(end);
        if let Some(url) = valid_web_url(candidate).filter(|url| !found.contains(url)) {
            found.push(url);
        }
        if found.len() == MAX_LINKS_PER_POST {
            break;
        }
    }
    found
}

/// The byte offset and length of the next scheme at or after `from` that starts a word.
fn next_scheme(lower: &str, from: usize) -> Option<(usize, usize)> {
    WEB_SCHEMES
        .iter()
        .filter_map(|scheme| {
            let mut search = from;
            while let Some(offset) = lower[search..].find(scheme) {
                let start = search + offset;
                let boundary = lower[..start].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
                if boundary {
                    return Some((start, scheme.len()));
                }
                search = start + scheme.len();
            }
            None
        })
        .min()
}

/// `raw` without trailing punctuation and unbalanced closing brackets.
fn trim_url(raw: &str) -> &str {
    let mut url = raw;
    loop {
        let before = url.len();
        url = url.trim_end_matches(TRAILING_PUNCTUATION);
        for (open, close) in BRACKET_PAIRS {
            if url.ends_with(close) && url.matches(close).count() > url.matches(open).count() {
                url = &url[..url.len() - close.len_utf8()];
            }
        }
        if url.len() == before {
            return url;
        }
    }
}

/// `raw` as a canonical `http(s)` URL with a host, if it is one.
fn valid_web_url(raw: &str) -> Option<String> {
    let url = Url::parse(raw.trim()).ok()?;
    let web = matches!(url.scheme(), "http" | "https");
    let host = url.host_str().is_some_and(|host| !host.is_empty());
    (web && host).then(|| url.to_string())
}

/// The links of a post: `attached` (the link field) first, then the URLs in `content`.
pub fn post_links(attached: Option<&str>, content: &str) -> Vec<String> {
    let mut links: Vec<String> = attached.and_then(valid_web_url).into_iter().collect();
    for url in detect_urls(content) {
        if !links.contains(&url) && links.len() < MAX_LINKS_PER_POST {
            links.push(url);
        }
    }
    links
}

/// The last path segment of `url` (`seminar-clip.mp4`), percent-decoding left as written.
pub fn link_file_name(url: &str) -> Option<String> {
    let parsed = Url::parse(url.trim()).ok()?;
    let name = parsed.path_segments()?.filter(|segment| !segment.is_empty()).next_back()?;
    Some(name.to_string())
}

/// The host of `url` as shown on cards (`www.` dropped, lowercase); the input when unparsable.
pub fn display_domain(url: &str) -> String {
    match Url::parse(url.trim()).ok().and_then(|url| url.host_str().map(str::to_ascii_lowercase)) {
        Some(host) => host.strip_prefix(WWW_PREFIX).map(str::to_string).unwrap_or(host),
        None => url.trim().to_string(),
    }
}
