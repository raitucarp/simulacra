//! Free recall: a typed answer compared with the expected one after normalisation (case,
//! accents, punctuation and spacing ignored), with an edit-distance tolerance for typos.
//! The verdict only suggests a grade; the learner grades themselves.

use super::super::grade::Grade;

/// Similarity from which an answer counts as close (a typo or two).
pub const CLOSE_SIMILARITY: f64 = 0.8;

/// How a typed answer compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecallVerdict {
    Exact,
    Close,
    Wrong,
}

impl RecallVerdict {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Close => "close",
            Self::Wrong => "wrong",
        }
    }

    /// The grade pre-selected for the learner.
    pub const fn suggested_grade(self) -> Grade {
        match self {
            Self::Exact => Grade::Good,
            Self::Close => Grade::Hard,
            Self::Wrong => Grade::Again,
        }
    }
}

/// Folds a Latin letter with a diacritic to its base letter (other characters unchanged).
fn fold_accent(c: char) -> char {
    const FOLDS: [(&str, char); 7] = [("àáâãäåā", 'a'), ("èéêëē", 'e'), ("ìíîïī", 'i'), ("òóôõöøō", 'o'), ("ùúûüū", 'u'), ("ç", 'c'), ("ñ", 'n')];
    FOLDS.iter().find(|(set, _)| set.contains(c)).map_or(c, |(_, base)| *base)
}

/// Lower case, accents folded, punctuation dropped, whitespace collapsed.
pub fn normalize_answer(text: &str) -> String {
    let cleaned: String = text
        .chars()
        .flat_map(char::to_lowercase)
        .map(fold_accent)
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Levenshtein distance in characters.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let first: Vec<usize> = (0..=b.len()).collect();
    let last = a.chars().enumerate().fold(first, |previous, (i, ca)| {
        let mut row = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            let substitute = previous[j] + usize::from(ca != *cb);
            row.push(substitute.min(previous[j + 1] + 1).min(row[j] + 1));
        }
        row
    });
    last[b.len()]
}

/// 1 for identical normalised answers, falling towards 0 with each edit.
pub fn similarity(expected: &str, typed: &str) -> f64 {
    let (expected, typed) = (normalize_answer(expected), normalize_answer(typed));
    let longest = expected.chars().count().max(typed.chars().count());
    if longest == 0 {
        return 0.0;
    }
    1.0 - edit_distance(&expected, &typed) as f64 / longest as f64
}

/// The verdict on `typed` against `expected`.
pub fn judge_recall(expected: &str, typed: &str) -> RecallVerdict {
    if normalize_answer(typed).is_empty() {
        return RecallVerdict::Wrong;
    }
    match similarity(expected, typed) {
        score if score >= 1.0 => RecallVerdict::Exact,
        score if score >= CLOSE_SIMILARITY => RecallVerdict::Close,
        _ => RecallVerdict::Wrong,
    }
}
