//! Texts the catalog hands out: a [`Phrase`] (a key to translate or a literal), one example
//! chip, and how an example is appended to text already typed.

/// A user-visible text: an i18n key to translate, or a literal (names, ids, URLs, JSON).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phrase {
    Key(String),
    Literal(String),
}

impl Phrase {
    pub fn key(key: impl Into<String>) -> Self {
        Self::Key(key.into())
    }

    pub fn literal(text: impl Into<String>) -> Self {
        Self::Literal(text.into())
    }

    /// The text, with a key translated by `translate`.
    pub fn resolve(&self, translate: impl Fn(&str) -> String) -> String {
        match self {
            Self::Key(key) => translate(key),
            Self::Literal(text) => text.clone(),
        }
    }
}

/// One example chip: what it shows and what it inserts (the same for short values).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Example {
    pub label: Phrase,
    pub value: Phrase,
}

impl Example {
    /// A chip showing and inserting `phrase`.
    pub fn same(phrase: Phrase) -> Self {
        Self { label: phrase.clone(), value: phrase }
    }

    /// A chip named `label` inserting a longer `value` (templates, schemas).
    pub fn named(label: Phrase, value: Phrase) -> Self {
        Self { label, value }
    }
}

/// How an example is appended to text already in the field (shift-click).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joiner {
    /// Lists: `a, b`.
    Comma,
    /// Prose and hashtags: `a b`.
    Space,
    /// Prompts and templates: one per line.
    Line,
}

impl Joiner {
    const fn separator(self) -> &'static str {
        match self {
            Self::Comma => ", ",
            Self::Space => " ",
            Self::Line => "\n",
        }
    }

    /// `addition` after `current` (just `addition` when the field is blank).
    pub fn join(self, current: &str, addition: &str) -> String {
        let head = current.trim_end();
        if head.is_empty() {
            addition.to_string()
        } else {
            format!("{head}{}{addition}", self.separator())
        }
    }
}
