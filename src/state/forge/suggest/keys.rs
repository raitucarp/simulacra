//! Locale keys of the catalog: `suggest.ex.<path>.<n>` example values, `suggest.label.<path>.<n>`
//! chip names of long technical values, `suggest.tip.<path>` text tips, `suggest.why.<path>`
//! range and choice reasons and `suggest.combo.<id>` combination notes.

use super::model::{Example, Phrase};

const ROOT: &str = "suggest";
const EXAMPLE_GROUP: &str = "ex";
const LABEL_GROUP: &str = "label";
const TIP_GROUP: &str = "tip";
const WHY_GROUP: &str = "why";
const COMBO_GROUP: &str = "combo";

/// Translated example values every prose field offers.
pub const KEYED_EXAMPLES: usize = 5;

/// `suggest.ex.<path>.<n>` (`n` from 1).
pub fn example_key(path: &str, n: usize) -> String {
    format!("{ROOT}.{EXAMPLE_GROUP}.{path}.{n}")
}

/// `suggest.label.<path>.<n>` (`n` from 1).
pub fn label_key(path: &str, n: usize) -> String {
    format!("{ROOT}.{LABEL_GROUP}.{path}.{n}")
}

/// `suggest.tip.<path>`.
pub fn tip_key(path: &str) -> String {
    format!("{ROOT}.{TIP_GROUP}.{path}")
}

/// `suggest.why.<path>`.
pub fn why_key(path: &str) -> String {
    format!("{ROOT}.{WHY_GROUP}.{path}")
}

/// `suggest.combo.<id>`.
pub fn combo_key(id: &str) -> String {
    format!("{ROOT}.{COMBO_GROUP}.{id}")
}

/// The [`KEYED_EXAMPLES`] translated examples of `path`.
pub fn keyed_examples(path: &str) -> Vec<Example> {
    (1..=KEYED_EXAMPLES).map(|n| Example::same(Phrase::key(example_key(path, n)))).collect()
}

/// Literal examples (names, ids, URLs).
pub fn literal_examples<S: AsRef<str>>(values: &[S]) -> Vec<Example> {
    values.iter().map(|value| Example::same(Phrase::literal(value.as_ref()))).collect()
}

/// Long literal values under translated chip names (`suggest.label.<path>.<n>`).
pub fn named_examples(path: &str, values: &[&str]) -> Vec<Example> {
    values
        .iter()
        .enumerate()
        .map(|(at, value)| Example::named(Phrase::key(label_key(path, at + 1)), Phrase::literal(*value)))
        .collect()
}
