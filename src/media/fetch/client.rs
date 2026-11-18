//! The preview HTTP client: short timeouts, a few redirects that stay on `http(s)` (and off
//! private addresses unless allowed), no cookies (the client has no cookie store), no
//! `Referer`, and a generic user agent that says nothing about the user.

use crate::agent::tools::http::is_private_host;
use reqwest::redirect::{Attempt, Policy};
use std::time::Duration;

/// Whole-request and connect time limits.
pub const FETCH_TIMEOUT: Duration = Duration::from_secs(8);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(4);
/// Redirect hops followed at most.
pub const MAX_REDIRECTS: usize = 4;
/// The user agent of preview requests (some sites only serve their cards to a browser-like one).
const USER_AGENT: &str = "Mozilla/5.0 (compatible; SimulacraLinkPreview/1.0)";
const WEB_SCHEMES: [&str; 2] = ["http", "https"];

/// A client for one preview job.
pub fn preview_client(allow_private: bool) -> reqwest::Client {
    let redirects = Policy::custom(move |attempt: Attempt| {
        let url = attempt.url();
        if attempt.previous().len() > MAX_REDIRECTS {
            attempt.error("too many redirects")
        } else if !WEB_SCHEMES.contains(&url.scheme()) {
            attempt.error("redirect away from http(s)")
        } else if !allow_private && is_private_host(url) {
            attempt.error("redirect to a local or private address")
        } else {
            attempt.follow()
        }
    });
    reqwest::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .connect_timeout(CONNECT_TIMEOUT)
        .redirect(redirects)
        .referer(false)
        .user_agent(USER_AGENT)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}
