//! Data and configuration formats: JSON, TOML and YAML.

use super::super::grammar::{Grammar, BOTH_QUOTES, C_BLOCK, C_LINE, HASH_LINE, PLAIN};

pub(super) const JSON: Grammar = Grammar {
    names: &["json", "jsonc", "json5"],
    line_comments: C_LINE,
    block_comment: C_BLOCK,
    quotes: &['"'],
    keywords: &["true", "false", "null"],
    ..PLAIN
};

pub(super) const TOML: Grammar = Grammar {
    names: &["toml"],
    line_comments: HASH_LINE,
    quotes: BOTH_QUOTES,
    keywords: &["true", "false"],
    ..PLAIN
};

pub(super) const YAML: Grammar = Grammar {
    names: &["yaml", "yml"],
    line_comments: HASH_LINE,
    quotes: BOTH_QUOTES,
    keywords: &["true", "false", "null", "yes", "no"],
    ..PLAIN
};
