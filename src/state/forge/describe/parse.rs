//! Tolerant parsing of the LLM's character-sheet reply.

use super::{describable_traits, DraftPatch, PatchWarning, DESCRIBABLE_COUNTS, DESCRIBABLE_TEXT, DESCRIBE_GROUP};
use crate::state::forge::step::FORGE_I18N_ROOT;
use crate::state::forge::edit::DraftEdit;
use crate::state::forge::fields::{ScoreField, MAX_AGE, MAX_SCORE};
use serde_json::{Map, Value};
use thiserror::Error;

const OBJECT_OPEN: char = '{';
const OBJECT_CLOSE: char = '}';

/// Why a reply could not be read at all (individual bad keys are warnings instead).
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("the reply contains no JSON object")]
    NoJsonObject,
    #[error("the reply's JSON is malformed: {0}")]
    Malformed(String),
}

impl ParseError {
    pub const fn key(&self) -> &'static str {
        match self {
            Self::NoJsonObject => "error_no_json",
            Self::Malformed(_) => "error_malformed",
        }
    }

    /// i18n key of the message (`%{detail}` carries the parser error).
    pub fn message_key(&self) -> String {
        format!("{FORGE_I18N_ROOT}.{DESCRIBE_GROUP}.{}", self.key())
    }
}

/// The outermost `{…}` span, which skips code fences and chatter around the object.
fn object_span(raw: &str) -> Option<&str> {
    let start = raw.find(OBJECT_OPEN)?;
    let end = raw.rfind(OBJECT_CLOSE)?;
    (end > start).then(|| &raw[start..=end])
}

/// A whole number from a JSON number or numeric string (fractions are rounded).
fn number(value: &Value) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// Parses an LLM reply into edits; unknown keys and invalid values become warnings.
pub fn parse_described(raw: &str) -> Result<DraftPatch, ParseError> {
    let span = object_span(raw).ok_or(ParseError::NoJsonObject)?;
    let map: Map<String, Value> =
        serde_json::from_str(span).map_err(|e| ParseError::Malformed(e.to_string()))?;
    let mut patch = DraftPatch::default();
    for (key, value) in &map {
        if !value.is_null() {
            parse_entry(key, value, &mut patch);
        }
    }
    Ok(patch)
}

fn parse_entry(key: &str, value: &Value, patch: &mut DraftPatch) {
    let outcome = if let Some(field) = DESCRIBABLE_TEXT.iter().find(|f| f.path() == key) {
        let Some(text) = value.as_str().map(str::trim) else { return wrong_type(key, patch) };
        if text.is_empty() {
            return;
        }
        Ok(DraftEdit::Text(*field, text.to_string()))
    } else if let Some(field) = DESCRIBABLE_COUNTS.iter().find(|f| f.path() == key) {
        clamped(key, value, u64::from(MAX_AGE), patch).map(|n| DraftEdit::Count(*field, n as u16))
    } else if let Some(field) = ScoreField::ALL.iter().find(|f| f.path() == key) {
        clamped(key, value, u64::from(MAX_SCORE), patch).map(|n| DraftEdit::Score(*field, n as u8))
    } else if let Some(field) = describable_traits().into_iter().find(|f| f.path() == key) {
        let Some(text) = value.as_str() else { return wrong_type(key, patch) };
        match field.parse(text) {
            Some(parsed) => Ok(DraftEdit::Trait(parsed)),
            None => return patch.warnings.push(PatchWarning::InvalidOption { field, value: text.to_string() }),
        }
    } else {
        return patch.warnings.push(PatchWarning::UnknownField(key.to_string()));
    };
    match outcome {
        Ok(edit) => patch.edits.push(edit),
        Err(()) => wrong_type(key, patch),
    }
}

fn wrong_type(key: &str, patch: &mut DraftPatch) {
    patch.warnings.push(PatchWarning::WrongType(key.to_string()));
}

/// The rounded number in `0..=max`; out-of-range values are clamped with a warning.
fn clamped(key: &str, value: &Value, max: u64, patch: &mut DraftPatch) -> Result<u64, ()> {
    let raw = number(value).filter(|n| n.is_finite()).ok_or(())?.round();
    let bounded = raw.clamp(0.0, max as f64);
    if bounded != raw {
        let out_of_range = PatchWarning::OutOfRange { field: key.to_string(), value: raw.to_string() };
        patch.warnings.push(out_of_range);
    }
    Ok(bounded as u64)
}
