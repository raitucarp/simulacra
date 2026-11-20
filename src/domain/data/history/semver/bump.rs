//! Which part of a version number an edit bumps, read from the word diff of the two texts:
//! a typo-sized fix is a patch, an addition that keeps most of the old text is a minor
//! version, and a rewrite (most of the old words gone) is a major version.

use super::super::diff::{diff_stats, word_count, word_diff, DiffStats};

/// Most words a patch may add, and most it may remove: a fixed typo, a swapped word, a
/// missing "not". Anything larger is at least a minor version.
pub const PATCH_MAX_WORDS: usize = 3;

/// Share of the old words (in percent) an edit must remove to be a rewrite. Removing half
/// of the text or more replaces what it said, even if as many words come back.
pub const MAJOR_REMOVED_PERCENT: usize = 50;

/// The part of a version number an edit bumps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bump {
    /// A typo-sized fix: a few words changed, nothing of substance added.
    Patch,
    /// An addition or extension that keeps most of the old text.
    Minor,
    /// A rewrite: most of the old text removed or replaced.
    Major,
}

impl Bump {
    /// The bump of the edit turning `old` into `new`. Letter case is ignored, so whitespace,
    /// punctuation and capitalisation alone (and identical texts) are a patch; writing into
    /// an empty text is an addition.
    pub fn between(old: &str, new: &str) -> Self {
        let (old, new) = (old.to_lowercase(), new.to_lowercase());
        Self::from_stats(diff_stats(&word_diff(&old, &new)), word_count(&old))
    }

    /// The bump of an edit that adds and removes `stats` words of a text that had
    /// `old_words` words.
    ///
    /// - **Major:** every old word is gone, or more than [`PATCH_MAX_WORDS`] words and at
    ///   least [`MAJOR_REMOVED_PERCENT`] of the old ones are removed.
    /// - **Patch:** otherwise, when at most [`PATCH_MAX_WORDS`] are added and removed (and
    ///   the old text had words: the first words written into an empty text are an addition).
    /// - **Minor:** everything else (additions, extensions, partial rewording).
    pub fn from_stats(stats: DiffStats, old_words: usize) -> Self {
        let nothing_kept = old_words > 0 && stats.removed >= old_words;
        let first_words = old_words == 0 && stats.added > 0;
        let small = !first_words && stats.added <= PATCH_MAX_WORDS && stats.removed <= PATCH_MAX_WORDS;
        let mostly_removed = old_words > 0 && stats.removed * 100 >= MAJOR_REMOVED_PERCENT * old_words;
        if nothing_kept || (!small && mostly_removed) {
            Self::Major
        } else if small {
            Self::Patch
        } else {
            Self::Minor
        }
    }
}
