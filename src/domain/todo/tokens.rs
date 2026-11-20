//! Recognising one whitespace-separated word as a to-do token.

/// Characters a due phrase or repeat rule may contain besides letters and digits.
const RAW_EXTRA: [char; 5] = ['-', '+', '/', ',', ':'];
/// Characters a context or delegate name may contain besides letters and digits.
const NAME_EXTRA: [char; 2] = ['-', '_'];
/// The most `!` a priority token has.
const MAX_PRIORITY: usize = 3;
const MINUTES_PER_HOUR: u32 = 60;

/// The value a token word carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenValue<'a> {
    Due(&'a str),
    Repeat(&'a str),
    Priority(u8),
    Context(&'a str),
    Estimate(u32),
    Delegate(&'a str),
}

impl TokenValue<'_> {
    pub fn kind(&self) -> super::TodoTokenKind {
        use super::TodoTokenKind as K;
        match self {
            Self::Due(_) => K::Due,
            Self::Repeat(_) => K::Repeat,
            Self::Priority(_) => K::Priority,
            Self::Context(_) => K::Context,
            Self::Estimate(_) => K::Estimate,
            Self::Delegate(_) => K::Delegate,
        }
    }

    /// Reads a whole `word` as a token; `None` when it is plain text.
    pub fn parse(word: &str) -> Option<TokenValue<'_>> {
        if let Some(rule) = word.strip_prefix("^^") {
            return raw(rule).map(TokenValue::Repeat);
        }
        if let Some(phrase) = word.strip_prefix('^') {
            return raw(phrase).map(TokenValue::Due);
        }
        if let Some(level) = priority(word) {
            return Some(TokenValue::Priority(level));
        }
        let mut chars = word.chars();
        let sigil = chars.next()?;
        let rest = chars.as_str();
        match sigil {
            '&' => name(rest).map(TokenValue::Context),
            '@' => name(rest).map(TokenValue::Delegate),
            '~' => parse_estimate(rest).map(TokenValue::Estimate),
            _ => None,
        }
    }
}

/// Minutes in an estimate such as `25m`, `2h` or `1h30m` (without the `~`). Zero, unknown
/// units, decimals and overflow are rejected.
pub fn parse_estimate(text: &str) -> Option<u32> {
    let (hours, rest) = number_with_unit(text, 'h');
    let (minutes, rest) = number_with_unit(rest, 'm');
    if !rest.is_empty() || (hours.is_none() && minutes.is_none()) {
        return None;
    }
    let total = hours.unwrap_or(0).checked_mul(MINUTES_PER_HOUR)?.checked_add(minutes.unwrap_or(0))?;
    (total > 0).then_some(total)
}

/// A leading run of digits followed by `unit`, and the text after it.
fn number_with_unit(text: &str, unit: char) -> (Option<u32>, &str) {
    let digits = text.len() - text.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    let Some(rest) = text[digits..].strip_prefix(unit) else { return (None, text) };
    match text[..digits].parse() {
        Ok(value) if digits > 0 => (Some(value), rest),
        _ => (None, text),
    }
}

fn raw(text: &str) -> Option<&str> {
    let valid = text.chars().all(|c| c.is_alphanumeric() || RAW_EXTRA.contains(&c));
    (valid && !text.is_empty()).then_some(text)
}

fn name(text: &str) -> Option<&str> {
    let starts_well = text.chars().next().is_some_and(char::is_alphanumeric);
    let valid = text.chars().all(|c| c.is_alphanumeric() || NAME_EXTRA.contains(&c));
    (starts_well && valid).then_some(text)
}

fn priority(word: &str) -> Option<u8> {
    let bangs = !word.is_empty() && word.len() <= MAX_PRIORITY && word.chars().all(|c| c == '!');
    bangs.then(|| u8::try_from(word.len()).unwrap_or(u8::MAX))
}
