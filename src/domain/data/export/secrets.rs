//! Secrets never leave in a plain bundle.
//!
//! - **Settings:** only the rows on [`SETTINGS_ALLOW_LIST`] are exported. It is an allow-list:
//!   a setting added later stays out until someone decides it is safe. Credentials, the LLM
//!   provider rows (keys, endpoints, models) and usage counters are not on it.
//! - **Agent configurations:** every field whose name suggests a secret (key, token, secret,
//!   passphrase, credential, authorization, `*_key`) holding text is replaced by [`REDACTED`], custom
//!   request headers lose their values, and query strings are cut from URLs.

use crate::domain::Profile;
use crate::storage::SettingKey;
use serde_json::Value;

/// Settings an export may carry: appearance and the typed preferences, nothing else.
pub const SETTINGS_ALLOW_LIST: &[SettingKey] = &[
    SettingKey::ThemePresetId, SettingKey::ThemeOverrides, SettingKey::ThemeCustomPalettes, SettingKey::ThemeDensity,
    SettingKey::ThemeTypeScale, SettingKey::ThemeRadiusScale, SettingKey::ThemeFontFamily, SettingKey::ThemeAppearance,
    SettingKey::ThemeDarkPalette, SettingKey::ThemeLightPalette, SettingKey::PrefsGeneral, SettingKey::PrefsAppearance,
    SettingKey::PrefsWriting, SettingKey::PrefsAi, SettingKey::PrefsEmbeddings, SettingKey::PrefsImages, SettingKey::PrefsAgents,
    SettingKey::PrefsLearning, SettingKey::PrefsImport, SettingKey::PrefsPrivacy, SettingKey::PrefsNotifications,
    SettingKey::PrefsAdvanced, SettingKey::InspectorMode,
];

/// What a redacted value reads.
pub const REDACTED: &str = "[redacted]";
/// Field-name fragments that mark a secret (compared in lower case).
const SECRET_MARKERS: [&str; 8] = ["api_key", "apikey", "token", "secret", "password", "passphrase", "credential", "authorization"];
/// A field named exactly this, or ending in `_key`, is a secret too.
const KEY_NAME: &str = "key";
const KEY_SUFFIX: &str = "_key";
const HEADERS_FIELD: &str = "headers";
const HEADER_VALUE_FIELD: &str = "value";
const URL_FIELD: &str = "url";
const QUERY_START: char = '?';

fn is_secret_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower == KEY_NAME || lower.ends_with(KEY_SUFFIX) || SECRET_MARKERS.iter().any(|marker| lower.contains(marker))
}

/// Every string inside `value` replaced by [`REDACTED`] (numbers and flags stay).
fn blank_strings(value: Value) -> Value {
    match value {
        Value::String(_) => Value::String(REDACTED.to_string()),
        Value::Array(items) => Value::Array(items.into_iter().map(blank_strings).collect()),
        Value::Object(fields) => Value::Object(fields.into_iter().map(|(k, v)| (k, blank_strings(v))).collect()),
        other => other,
    }
}

fn redact_headers(value: Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.into_iter().map(redact_headers).collect()),
        Value::Object(fields) => Value::Object(fields.into_iter().map(|(k, v)| if k == HEADER_VALUE_FIELD { (k, blank_strings(v)) } else { (k, redact_headers(v)) }).collect()),
        other => other,
    }
}

fn redact_field(name: &str, value: Value) -> Value {
    match value {
        Value::String(url) if name == URL_FIELD => Value::String(url.split(QUERY_START).next().unwrap_or_default().to_string()),
        value if name == HEADERS_FIELD => redact_headers(value),
        value if is_secret_name(name) => blank_strings(value),
        value => redact(value),
    }
}

/// `value` with every secret-looking field redacted, recursively.
pub fn redact(value: Value) -> Value {
    match value {
        Value::Object(fields) => Value::Object(fields.into_iter().map(|(k, v)| {
            let cleaned = redact_field(&k, v);
            (k, cleaned)
        }).collect()),
        Value::Array(items) => Value::Array(items.into_iter().map(redact).collect()),
        other => other,
    }
}

/// `profile` as JSON with its secrets redacted.
pub fn redacted_profile(profile: &Profile) -> Value {
    serde_json::to_value(profile).map(redact).unwrap_or(Value::Null)
}
