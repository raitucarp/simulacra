//! URL host extraction for link preview cards.

/// URL schemes stripped before host extraction (checked in order).
const URL_SCHEMES: [&str; 2] = ["https://", "http://"];
/// Characters terminating the host part of a URL.
const HOST_TERMINATORS: [char; 3] = ['/', '?', '#'];
/// Conventional subdomain stripped from hosts.
const WWW_PREFIX: &str = "www.";

/// Extracts a clean host domain slug from an arbitrary URL.
pub fn extract_host_domain(url: &str) -> String {
    let lower = url.trim().to_ascii_lowercase();
    let without_scheme = URL_SCHEMES
        .iter()
        .find_map(|scheme| lower.strip_prefix(scheme))
        .unwrap_or(&lower);

    let host_part = without_scheme
        .split(HOST_TERMINATORS)
        .next()
        .unwrap_or(without_scheme);

    host_part.strip_prefix(WWW_PREFIX).unwrap_or(host_part).to_string()
}
