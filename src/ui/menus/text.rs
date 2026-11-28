//! A menu label: the text shown and, when it is a translation, the locale key it came from
//! (so the structure tests can check that every key exists in every locale).

use gpui_kit::SharedString;
use rust_i18n::t;

/// The label of a menu item, submenu or section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuText {
    /// The locale key, `None` for names the user chose (collections, series, agents).
    pub key: Option<SharedString>,
    pub text: SharedString,
}

impl MenuText {
    /// The translation of `key`.
    pub fn key(key: impl Into<SharedString>) -> Self {
        let key = key.into();
        let text = SharedString::from(t!(key.as_ref()).to_string());
        Self { key: Some(key), text }
    }

    /// A translation of `key` already formatted with its arguments.
    pub fn keyed(key: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        Self { key: Some(key.into()), text: text.into() }
    }

    /// Text that is not translated (a name the user gave).
    pub fn plain(text: impl Into<SharedString>) -> Self {
        Self { key: None, text: text.into() }
    }
}
