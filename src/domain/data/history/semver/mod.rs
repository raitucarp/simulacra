//! Version numbers (sb V1, V2): a `major.minor.patch` value written as text (`2.1.0`), stored in
//! `revisions.version`, and the rule picking which part an edit bumps ([`Bump::between`]).

mod bump;
mod wire; // V3: stored as text

pub use bump::{Bump, MAJOR_REMOVED_PERCENT, PATCH_MAX_WORDS};

use std::fmt;
use std::str::FromStr;

/// Separator between the three numbers.
const SEPARATOR: char = '.';

/// A semantic version number. Ordered by major, then minor, then patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

/// Why a text is not a version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseSemVerError;

impl fmt::Display for ParseSemVerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected a version number major.minor.patch")
    }
}

impl std::error::Error for ParseSemVerError {}

impl SemVer {
    /// The first version of anything: `1.0.0`.
    pub const ORIGINAL: Self = Self::new(1, 0, 0);

    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    /// The next version after `bump`: a major bump resets minor and patch, a minor bump
    /// resets patch. Saturates instead of overflowing.
    #[must_use]
    pub const fn bumped(self, bump: Bump) -> Self {
        match bump {
            Bump::Major => Self::new(self.major.saturating_add(1), 0, 0),
            Bump::Minor => Self::new(self.major, self.minor.saturating_add(1), 0),
            Bump::Patch => Self::new(self.major, self.minor, self.patch.saturating_add(1)),
        }
    }

    /// The version after `edits` edits of the original (`0` is the original itself), for
    /// chains backfilled before bumps were read from the diff: each edit was a minor version.
    pub const fn after_edits(edits: u32) -> Self {
        Self::new(Self::ORIGINAL.major, edits, 0)
    }

    /// The version one minor edit after this one.
    #[must_use]
    pub const fn next_edit(self) -> Self {
        self.bumped(Bump::Minor)
    }

    /// The version written as `major.minor.patch`, if `raw` is one.
    pub fn parse(raw: &str) -> Option<Self> {
        raw.parse().ok()
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::ORIGINAL
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{SEPARATOR}{}{SEPARATOR}{}", self.major, self.minor, self.patch)
    }
}

/// One number: ASCII digits only, without a sign or a leading zero (except `0` itself).
fn number(part: &str) -> Result<u32, ParseSemVerError> {
    let digits = !part.is_empty() && part.bytes().all(|b| b.is_ascii_digit());
    if !digits || (part.len() > 1 && part.starts_with('0')) {
        return Err(ParseSemVerError);
    }
    part.parse().map_err(|_| ParseSemVerError)
}

impl FromStr for SemVer {
    type Err = ParseSemVerError;

    /// Parses exactly `major.minor.patch` (surrounding whitespace allowed).
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut parts = text.trim().split(SEPARATOR);
        let mut next = || parts.next().ok_or(ParseSemVerError).and_then(number);
        let version = Self::new(next()?, next()?, next()?);
        match parts.next() {
            None => Ok(version),
            Some(_) => Err(ParseSemVerError),
        }
    }
}

#[cfg(test)]
mod tests;
