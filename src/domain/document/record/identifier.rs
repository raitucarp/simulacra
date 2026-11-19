//! Standard identifiers (ISBN, DOI, URL) and their validation.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// The identifier scheme of a document.
    IdentifierScheme {
        Isbn => "isbn",
        Doi => "doi",
        Url => "url",
    }
}

/// Why an identifier is not well-formed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifierProblem {
    /// Not 10 or 13 digits, or the check digit does not match.
    BadIsbn,
    /// Not of the form `10.<registrant>/<suffix>`.
    BadDoi,
    /// Not an `http://` or `https://` address with a host.
    BadUrl,
}

const ISBN10_LEN: usize = 10;
const ISBN13_LEN: usize = 13;
const ISBN10_CHECK_X: char = 'X';
const ISBN10_X_VALUE: u32 = 10;
const ISBN10_MODULUS: u32 = 11;
const ISBN13_MODULUS: u32 = 10;
const ISBN13_ODD_WEIGHT: u32 = 3;
const DOI_DIRECTORY: &str = "10.";
const DOI_SEPARATOR: char = '/';
const URL_SCHEMES: [&str; 2] = ["https://", "http://"];

/// A standard identifier; an empty value means none.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Identifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<IdentifierScheme>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
}

impl Identifier {
    pub fn new(scheme: IdentifierScheme, value: impl Into<String>) -> Self {
        Self { scheme: Some(scheme), value: value.into().trim().to_string() }
    }

    /// Whether no identifier is recorded.
    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }

    /// The identifier's problem, if any (an empty identifier is fine).
    pub fn problem(&self) -> Option<IdentifierProblem> {
        match (self.scheme, self.is_empty()) {
            (_, true) | (None, false) => None,
            (Some(IdentifierScheme::Isbn), false) => (!is_valid_isbn(&self.value)).then_some(IdentifierProblem::BadIsbn),
            (Some(IdentifierScheme::Doi), false) => (!is_valid_doi(&self.value)).then_some(IdentifierProblem::BadDoi),
            (Some(IdentifierScheme::Url), false) => (!is_valid_url(&self.value)).then_some(IdentifierProblem::BadUrl),
        }
    }
}

/// The digits and a trailing `X` of an ISBN, without hyphens or spaces.
fn isbn_characters(raw: &str) -> Vec<char> {
    raw.chars().filter(|c| c.is_ascii_digit() || c.eq_ignore_ascii_case(&ISBN10_CHECK_X)).collect()
}

/// ISBN-10 (weights 10..1, mod 11, `X` = 10 last) or ISBN-13 (weights 1,3, mod 10).
pub fn is_valid_isbn(raw: &str) -> bool {
    let chars = isbn_characters(raw);
    match chars.len() {
        ISBN10_LEN => {
            let values: Option<Vec<u32>> = chars.iter().enumerate().map(|(at, c)| isbn10_value(*c, at)).collect();
            values.is_some_and(|values| {
                let weighted: u32 = values.iter().zip((1..=ISBN10_LEN as u32).rev()).map(|(v, w)| v * w).sum();
                weighted % ISBN10_MODULUS == 0
            })
        }
        ISBN13_LEN => {
            let digits: Option<Vec<u32>> = chars.iter().map(|c| c.to_digit(10)).collect();
            digits.is_some_and(|digits| {
                let weight = |at: usize| if at % 2 == 0 { 1 } else { ISBN13_ODD_WEIGHT };
                let sum: u32 = digits.iter().enumerate().map(|(at, d)| d * weight(at)).sum();
                sum % ISBN13_MODULUS == 0
            })
        }
        _ => false,
    }
}

fn isbn10_value(c: char, at: usize) -> Option<u32> {
    match c.to_digit(10) {
        Some(digit) => Some(digit),
        None if at == ISBN10_LEN - 1 => Some(ISBN10_X_VALUE),
        None => None,
    }
}

/// `10.<registrant>/<suffix>`, with an optional `doi:` or resolver prefix stripped.
pub fn is_valid_doi(raw: &str) -> bool {
    let doi = raw.trim().trim_start_matches("doi:").trim_start_matches("https://doi.org/");
    doi.strip_prefix(DOI_DIRECTORY)
        .and_then(|rest| rest.split_once(DOI_SEPARATOR))
        .is_some_and(|(registrant, suffix)| {
            !registrant.is_empty() && registrant.chars().all(|c| c.is_ascii_digit() || c == '.') && !suffix.is_empty()
        })
}

/// An `http(s)://host...` address without whitespace.
pub fn is_valid_url(raw: &str) -> bool {
    let raw = raw.trim();
    URL_SCHEMES
        .iter()
        .find_map(|scheme| raw.strip_prefix(scheme))
        .is_some_and(|rest| {
            let host = rest.split(['/', '?', '#']).next().unwrap_or_default();
            host.contains('.') && !rest.chars().any(char::is_whitespace)
        })
}
