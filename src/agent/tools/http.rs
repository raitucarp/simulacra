//! Shared HTTP plumbing of web-facing tools: URL policy, capped reads, redirects and
//! URL-encoding.

use super::context::FetchPolicy;
use reqwest::redirect::{Attempt, Policy};
use reqwest::Url;
use std::net::IpAddr;

/// Redirect hops followed at most.
const MAX_REDIRECTS: usize = 5;
/// Allowed URL schemes.
const ALLOWED_SCHEMES: [&str; 2] = ["http", "https"];
/// Host names that always resolve locally.
const LOCAL_HOSTS: [&str; 1] = ["localhost"];
/// Host suffixes that resolve locally (mDNS, `*.localhost`, internal zones).
const LOCAL_SUFFIXES: [&str; 3] = [".localhost", ".local", ".internal"];

/// Parses `raw` and enforces the scheme and (unless allowed) the public-host policy.
pub fn checked_url(raw: &str, policy: &FetchPolicy) -> Result<Url, String> {
    let url = Url::parse(raw.trim()).map_err(|e| format!("Invalid URL \"{raw}\": {e}"))?;
    if !ALLOWED_SCHEMES.contains(&url.scheme()) {
        return Err(format!("Only http and https URLs are allowed, not \"{}\".", url.scheme()));
    }
    if !policy.allow_private_hosts && is_private_host(&url) {
        return Err(format!("Refusing to fetch a local or private address: {}", url.host_str().unwrap_or_default()));
    }
    Ok(url)
}

/// Whether `url` targets a loopback, private, link-local or otherwise local host.
pub fn is_private_host(url: &Url) -> bool {
    let Some(host) = url.host_str() else { return true };
    let host = host.trim_start_matches('[').trim_end_matches(']').to_lowercase();
    if LOCAL_HOSTS.contains(&host.as_str()) || LOCAL_SUFFIXES.iter().any(|s| host.ends_with(s)) {
        return true;
    }
    match host.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => ip.is_loopback() || ip.is_private() || ip.is_link_local() || ip.is_unspecified() || ip.is_broadcast(),
        Ok(IpAddr::V6(ip)) => {
            let first = ip.segments()[0];
            ip.is_loopback() || ip.is_unspecified() || (first & 0xfe00) == 0xfc00 || (first & 0xffc0) == 0xfe80
        }
        Err(_) => false,
    }
}

/// A client with `policy`'s timeout whose redirects are re-checked against the host policy.
pub fn client(policy: &FetchPolicy) -> reqwest::Client {
    let allow_private = policy.allow_private_hosts;
    let redirects = Policy::custom(move |attempt: Attempt| {
        if attempt.previous().len() >= MAX_REDIRECTS {
            attempt.error("too many redirects")
        } else if !allow_private && is_private_host(attempt.url()) {
            attempt.error("redirect to a local or private address")
        } else {
            attempt.follow()
        }
    });
    reqwest::Client::builder()
        .timeout(policy.timeout)
        .redirect(redirects)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

/// Reads at most `max_bytes` of the body; `true` when it was cut.
pub async fn read_capped(mut resp: reqwest::Response, max_bytes: usize) -> Result<(Vec<u8>, bool), String> {
    let mut body = Vec::new();
    while let Some(chunk) = resp.chunk().await.map_err(|e| format!("Reading the response failed: {e}"))? {
        let room = max_bytes.saturating_sub(body.len());
        if chunk.len() > room {
            body.extend_from_slice(&chunk[..room]);
            return Ok((body, true));
        }
        body.extend_from_slice(&chunk);
    }
    Ok((body, false))
}

/// Percent-encodes everything but RFC 3986 unreserved characters.
pub fn percent_encode(raw: &str) -> String {
    raw.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            other => format!("%{other:02X}"),
        })
        .collect()
}

/// `text` cut to `max_chars` characters, marking the cut.
pub fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut cut: String = text.chars().take(max_chars).collect();
    cut.push_str(" …[truncated]");
    cut
}
