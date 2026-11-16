//! Handle normalisation: one stored form (`@Body`, case kept) and one comparison key
//! (lower-cased body), so `@Theo`, `theo` and `@@THEO` all name the same profile.

use super::HANDLE_PREFIX;

/// The handle body: trimmed, without any leading `@` sigils.
fn handle_body(raw: &str) -> &str {
    raw.trim().trim_start_matches(HANDLE_PREFIX).trim_start()
}

/// Stored form of a typed handle: exactly one leading `@`, case kept; `""` when blank.
pub fn canonical_handle(raw: &str) -> String {
    let body = handle_body(raw);
    if body.is_empty() {
        String::new()
    } else {
        format!("{HANDLE_PREFIX}{body}")
    }
}

/// Case-insensitive comparison key of a handle (lower-cased body without `@`).
pub fn handle_key(raw: &str) -> String {
    handle_body(raw).to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{canonical_handle, handle_key};

    #[test]
    fn repeated_sigils_collapse_and_case_is_ignored_for_comparison() {
        assert_eq!(canonical_handle("  @@Theo "), "@Theo");
        assert_eq!(canonical_handle("theo"), "@theo");
        assert_eq!(canonical_handle(" @@ "), "");
        assert_eq!(handle_key("@@THEO"), handle_key("theo"));
    }
}
