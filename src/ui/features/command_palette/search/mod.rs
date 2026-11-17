//! Pure search primitives of the palette: fuzzy scoring and match highlighting.

mod fuzzy;
mod highlight;

#[cfg(test)]
mod tests;

pub use fuzzy::{fuzzy_match, FuzzyMatch};
pub use highlight::highlight_ranges;
