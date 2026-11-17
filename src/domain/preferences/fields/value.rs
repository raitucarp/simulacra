//! The generic value and control kind of an editable preference, and the pure conversions
//! between them and the typed category fields.

use super::super::{Bounds, Choice};

/// One preference's value, as the settings screen reads and writes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingValue {
    Toggle(bool),
    /// A [`Choice`] wire key.
    Choice(&'static str),
    Number(i64),
    Text(String),
}

/// What a number counts, for its readout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberUnit {
    Characters,
    Seconds,
    Attempts,
    Requests,
    Tokens,
    TokensPerDay,
    Posts,
    Generations,
    Replies,
    Cards,
    Percent,
    Megabytes,
    /// An hour of the day, shown as `22:00`.
    Hour,
}

impl NumberUnit {
    /// Stable key (i18n suffix of the readout).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Characters => "characters",
            Self::Seconds => "seconds",
            Self::Attempts => "attempts",
            Self::Requests => "requests",
            Self::Tokens => "tokens",
            Self::TokensPerDay => "tokens_per_day",
            Self::Posts => "posts",
            Self::Generations => "generations",
            Self::Replies => "replies",
            Self::Cards => "cards",
            Self::Percent => "percent",
            Self::Megabytes => "megabytes",
            Self::Hour => "hour",
        }
    }
}

/// A numeric preference's range, unit and whether `0` reads as "no limit".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberSpec {
    pub bounds: Bounds,
    pub unit: NumberUnit,
    pub zero_is_unlimited: bool,
}

/// The control a preference is edited with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    Toggle,
    /// One of these wire keys.
    Choice(&'static [&'static str]),
    Number(NumberSpec),
    Text,
    /// A profile id (blank = the last used persona).
    Persona,
    /// Shown, never editable.
    Locked,
}

/// A number preference counted in `unit`.
pub(super) const fn number(bounds: Bounds, unit: NumberUnit) -> FieldKind {
    FieldKind::Number(NumberSpec { bounds, unit, zero_is_unlimited: false })
}

/// A number preference where `0` means "no limit".
pub(super) const fn limit(bounds: Bounds, unit: NumberUnit) -> FieldKind {
    FieldKind::Number(NumberSpec { bounds, unit, zero_is_unlimited: true })
}

/// The choice control of `C`.
pub(super) fn choices<C: Choice>() -> FieldKind {
    FieldKind::Choice(C::KEYS)
}

pub(super) fn choice_value<C: Choice>(option: C) -> SettingValue {
    SettingValue::Choice(option.key())
}

pub(super) fn number_value(value: u32) -> SettingValue {
    SettingValue::Number(i64::from(value))
}

/// `value` as a toggle, else `current`.
pub(super) fn toggle(value: &SettingValue, current: bool) -> bool {
    match value {
        SettingValue::Toggle(on) => *on,
        _ => current,
    }
}

/// `value` as an option of `C`, else `current`.
pub(super) fn choice<C: Choice>(value: &SettingValue, current: C) -> C {
    match value {
        SettingValue::Choice(key) => C::from_key(key).unwrap_or(current),
        _ => current,
    }
}

/// `value` as a non-negative number (clamping happens in `validated`), else `current`.
pub(super) fn count(value: &SettingValue, current: u32) -> u32 {
    match value {
        SettingValue::Number(number) => u32::try_from((*number).max(0)).unwrap_or(u32::MAX),
        _ => current,
    }
}

/// `value` as text, else `current`.
pub(super) fn text(value: &SettingValue, current: String) -> String {
    match value {
        SettingValue::Text(text) => text.clone(),
        _ => current,
    }
}
