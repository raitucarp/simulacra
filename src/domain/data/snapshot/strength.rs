//! A passphrase strength hint: an estimate of guessing effort in bits, deliberately cautious.
//!
//! - **Characters:** length × log₂ of the alphabet the passphrase draws from (lower case,
//!   upper case, digits, symbols, other scripts); a character that repeats the previous one
//!   or continues a run (`abc`, `321`) counts a quarter.
//! - **Words:** a passphrase of several words is also estimated as about 11 bits per word
//!   (a vocabulary of a few thousand common words); the lower estimate wins.
//!
//! It is a hint, not a guarantee: it cannot know whether a phrase is a famous quotation.

/// Shortest passphrase a snapshot accepts.
pub const MIN_PASSPHRASE_CHARS: usize = 8;
/// Bits one word of a common vocabulary is worth.
const BITS_PER_WORD: f64 = 11.0;
/// Weight of a repeated or sequential character.
const RUN_WEIGHT: f64 = 0.25;
/// Upper bounds (bits) of the weak, fair and strong levels.
const WEAK_BELOW: f64 = 40.0;
const FAIR_BELOW: f64 = 60.0;
const STRONG_BELOW: f64 = 80.0;
/// Alphabet sizes: letters, digits, printable symbols, other scripts.
const LETTERS: f64 = 26.0;
const DIGITS: f64 = 10.0;
const SYMBOLS: f64 = 33.0;
const OTHER_SCRIPTS: f64 = 100.0;

/// How hard a passphrase looks to guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    Empty,
    Weak,
    Fair,
    Strong,
    VeryStrong,
}

impl Strength {
    /// Stable key (locale keys `data.snapshot.strength.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Weak => "weak",
            Self::Fair => "fair",
            Self::Strong => "strong",
            Self::VeryStrong => "very_strong",
        }
    }

    /// Filled segments of a four-segment meter.
    pub const fn level(self) -> usize {
        self as usize
    }
}

fn alphabet(passphrase: &str) -> f64 {
    let has = |test: fn(&char) -> bool| passphrase.chars().any(|c| test(&c));
    [
        (has(char::is_ascii_lowercase), LETTERS),
        (has(char::is_ascii_uppercase), LETTERS),
        (has(char::is_ascii_digit), DIGITS),
        (has(|c| c.is_ascii_punctuation() || *c == ' '), SYMBOLS),
        (has(|c| !c.is_ascii()), OTHER_SCRIPTS),
    ]
    .into_iter()
    .filter_map(|(present, size)| present.then_some(size))
    .sum()
}

fn character_bits(passphrase: &str) -> f64 {
    let per_char = alphabet(passphrase).max(1.0).log2();
    let chars: Vec<char> = passphrase.chars().collect();
    let weight = |i: usize| match i.checked_sub(1).map(|p| (chars[p] as i64, chars[i] as i64)) {
        Some((before, now)) if (now - before).abs() <= 1 => RUN_WEIGHT,
        _ => 1.0,
    };
    (0..chars.len()).map(weight).sum::<f64>() * per_char
}

/// The strength level of `passphrase`.
pub fn strength(passphrase: &str) -> Strength {
    if passphrase.is_empty() {
        return Strength::Empty;
    }
    let words = passphrase.split_whitespace().count();
    let by_words = if words > 1 { words as f64 * BITS_PER_WORD } else { f64::INFINITY };
    match character_bits(passphrase).min(by_words) {
        bits if bits < WEAK_BELOW => Strength::Weak,
        bits if bits < FAIR_BELOW => Strength::Fair,
        bits if bits < STRONG_BELOW => Strength::Strong,
        _ => Strength::VeryStrong,
    }
}

/// Whether `passphrase` is long enough to seal a snapshot.
pub fn meets_minimum(passphrase: &str) -> bool {
    passphrase.chars().count() >= MIN_PASSPHRASE_CHARS
}
