//! Writing a query back as text (Q4): filter chips remove or add one part, and the query
//! round-trips through [`parse_query`](super::parse_query) unchanged in meaning.

use super::{HasFilter, IsFilter, PostQuery, QueryDate, QueryFilter};

impl HasFilter {
    pub const ALL: [Self; 4] = [Self::Image, Self::File, Self::Code, Self::Todo];

    /// The value as typed after `has:`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::File => "file",
            Self::Code => "code",
            Self::Todo => "todo",
        }
    }
}

impl IsFilter {
    pub const ALL: [Self; 5] = [Self::Open, Self::Done, Self::Scheduled, Self::Archived, Self::Locked];

    /// The value as typed after `is:`.
    pub fn key(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Done => "done",
            Self::Scheduled => "scheduled",
            Self::Archived => "archived",
            Self::Locked => "locked",
        }
    }
}

impl QueryDate {
    /// The bound as typed (`YYYY-MM-DD` for a day).
    pub fn text(&self) -> String {
        match self {
            Self::Day(day) => day.to_string(),
            Self::Raw(raw) => raw.clone(),
        }
    }
}

impl QueryFilter {
    /// The key before the colon.
    pub fn key(&self) -> &'static str {
        match self {
            Self::From(_) => "from",
            Self::Kind(_) => "kind",
            Self::Symbol(_) => "symbol",
            Self::Tint(_) => "tint",
            Self::Has(_) => "has",
            Self::Is(_) => "is",
            Self::Before(_) => "before",
            Self::After(_) => "after",
        }
    }

    /// The value after the colon, unquoted.
    pub fn value(&self) -> String {
        match self {
            Self::From(v) | Self::Kind(v) | Self::Symbol(v) | Self::Tint(v) => v.clone(),
            Self::Has(has) => has.key().to_string(),
            Self::Is(is) => is.key().to_string(),
            Self::Before(date) | Self::After(date) => date.text(),
        }
    }

    /// `key:value`, the value quoted when it holds a space.
    pub fn to_text(&self) -> String {
        let value = self.value();
        if value.contains(char::is_whitespace) {
            format!("{}:\"{value}\"", self.key())
        } else {
            format!("{}:{value}", self.key())
        }
    }
}

impl PostQuery {
    /// The query as text: words, phrases and filters, then the negated ones.
    pub fn to_text(&self) -> String {
        let quoted = |phrase: &String| format!("\"{phrase}\"");
        let kept = self.words.iter().cloned().chain(self.phrases.iter().map(quoted)).chain(self.filters.iter().map(QueryFilter::to_text));
        let dropped = self.excluded_words.iter().cloned().chain(self.excluded_phrases.iter().map(quoted)).chain(self.excluded_filters.iter().map(QueryFilter::to_text));
        let parts: Vec<String> = kept.chain(dropped.map(|part| format!("-{part}"))).collect();
        parts.join(" ")
    }

    /// The query without `filter` (among the negated filters when `negated`).
    pub fn without_filter(&self, filter: &QueryFilter, negated: bool) -> Self {
        let mut query = self.clone();
        let list = if negated { &mut query.excluded_filters } else { &mut query.filters };
        list.retain(|kept| kept != filter);
        query
    }

    /// The query with `filter` added, or without it when it is already there.
    pub fn toggled_filter(&self, filter: QueryFilter) -> Self {
        if self.filters.contains(&filter) {
            return self.without_filter(&filter, false);
        }
        let mut query = self.clone();
        query.filters.push(filter);
        query
    }
}
