//! An opening tag: its name, raw attribute text and attribute lookup.

use super::entities::decode_entities;

/// An opening tag, borrowed from the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag<'a> {
    /// Raw name, possibly prefixed (`w:p`, `dc:title`).
    pub name: &'a str,
    attributes: &'a str,
    pub self_closing: bool,
}

/// The part of a (possibly prefixed) name after the colon.
pub fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

impl<'a> Tag<'a> {
    /// Parses the text between `<` and `>`.
    pub fn parse(inner: &'a str) -> Self {
        let trimmed = inner.trim_end();
        let self_closing = trimmed.ends_with('/');
        let body = trimmed.trim_end_matches('/');
        let name_end = body.find(|c: char| c.is_whitespace()).unwrap_or(body.len());
        Self { name: &body[..name_end], attributes: &body[name_end..], self_closing }
    }

    /// The local name (without prefix).
    pub fn local(&self) -> &'a str {
        local_name(self.name)
    }

    /// Whether the local name is `name` (case-insensitive).
    pub fn is(&self, name: &str) -> bool {
        self.local().eq_ignore_ascii_case(name)
    }

    /// The decoded value of attribute `wanted`. A prefixed name (`epub:type`) must match
    /// exactly; a bare name (`href`) matches any prefix.
    pub fn attr(&self, wanted: &str) -> Option<String> {
        self.attribute_pairs().find_map(|(key, value)| {
            let matches = if wanted.contains(':') {
                key.eq_ignore_ascii_case(wanted)
            } else {
                local_name(key).eq_ignore_ascii_case(wanted)
            };
            matches.then(|| decode_entities(value).into_owned())
        })
    }

    /// `(name, raw value)` of every attribute; a bare attribute has an empty value.
    fn attribute_pairs(&self) -> impl Iterator<Item = (&'a str, &'a str)> {
        let mut rest = self.attributes;
        std::iter::from_fn(move || {
            rest = rest.trim_start();
            if rest.is_empty() {
                return None;
            }
            let key_end = rest.find(|c: char| c == '=' || c.is_whitespace()).unwrap_or(rest.len());
            let key = &rest[..key_end];
            rest = rest[key_end..].trim_start();
            let Some(after_eq) = rest.strip_prefix('=') else { return Some((key, "")) };
            let (value, remaining) = split_value(after_eq.trim_start());
            rest = remaining;
            Some((key, value))
        })
    }
}

/// Splits a quoted or bare attribute value from the text after it.
fn split_value(text: &str) -> (&str, &str) {
    match text.chars().next() {
        Some(quote @ ('"' | '\'')) => {
            let body = &text[1..];
            let end = body.find(quote).unwrap_or(body.len());
            (&body[..end], body.get(end + 1..).unwrap_or_default())
        }
        _ => {
            let end = text.find(char::is_whitespace).unwrap_or(text.len());
            (&text[..end], &text[end..])
        }
    }
}
