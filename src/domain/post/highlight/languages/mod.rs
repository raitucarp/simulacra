//! The languages the highlighter knows, looked up by fence tag.

mod compiled;
mod data;
mod scripts;

use super::grammar::Grammar;

/// Every known language.
const GRAMMARS: &[&Grammar] = &[
    &compiled::RUST,
    &compiled::GO,
    &compiled::C,
    &compiled::CPP,
    &compiled::JAVA,
    &compiled::CSHARP,
    &compiled::KOTLIN,
    &scripts::PYTHON,
    &scripts::JAVASCRIPT,
    &scripts::TYPESCRIPT,
    &scripts::SHELL,
    &scripts::SQL,
    &data::JSON,
    &data::TOML,
    &data::YAML,
];

/// The grammar a fence tag names (tags are lower-cased by the body parser).
pub(super) fn grammar_for(tag: &str) -> Option<&'static Grammar> {
    let tag = tag.trim().to_lowercase();
    GRAMMARS.iter().copied().find(|grammar| grammar.names.contains(&tag.as_str()))
}
