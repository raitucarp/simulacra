//! The lexical shape of one language: comment and string delimiters, keywords and types.

/// Words, comments and quotes of a language, enough to colour it lexically.
pub(super) struct Grammar {
    /// Fence tags naming the language (lower case).
    pub names: &'static [&'static str],
    /// Markers that start a comment running to the end of the line.
    pub line_comments: &'static [&'static str],
    /// Opening and closing marker of a block comment.
    pub block_comment: Option<(&'static str, &'static str)>,
    /// Characters that open and close a string literal.
    pub quotes: &'static [char],
    /// `'` opens a one-character literal only (`'a'`, `'\n'`), so lifetimes stay plain.
    pub char_literals: bool,
    /// Quote characters whose strings may run over line breaks (template strings).
    pub multiline_quotes: &'static [char],
    pub keywords: &'static [&'static str],
    /// Built-in type names.
    pub types: &'static [&'static str],
    /// Identifiers starting with an upper-case letter are type names.
    pub capitalized_types: bool,
    /// Keywords match whatever their case (SQL).
    pub case_insensitive: bool,
}

/// A grammar with nothing set, the base the language tables extend.
pub(super) const PLAIN: Grammar = Grammar {
    names: &[],
    line_comments: &[],
    block_comment: None,
    quotes: &[],
    char_literals: false,
    multiline_quotes: &[],
    keywords: &[],
    types: &[],
    capitalized_types: false,
    case_insensitive: false,
};

/// `//` line comments and `/* */` block comments.
pub(super) const C_LINE: &[&str] = &["//"];
pub(super) const C_BLOCK: Option<(&str, &str)> = Some(("/*", "*/"));
/// `#` line comments (scripts and config files).
pub(super) const HASH_LINE: &[&str] = &["#"];
/// Double and single quotes.
pub(super) const BOTH_QUOTES: &[char] = &['"', '\''];

impl Grammar {
    /// Whether `word` is a keyword of the language.
    pub(super) fn is_keyword(&self, word: &str) -> bool {
        if self.case_insensitive {
            self.keywords.iter().any(|k| k.eq_ignore_ascii_case(word))
        } else {
            self.keywords.contains(&word)
        }
    }

    /// Whether `word` names a type (built-in, or capitalized where that marks types).
    pub(super) fn is_type(&self, word: &str) -> bool {
        let capitalized = self.capitalized_types && word.starts_with(|c: char| c.is_uppercase());
        capitalized || self.types.iter().any(|t| if self.case_insensitive { t.eq_ignore_ascii_case(word) } else { *t == word })
    }
}
