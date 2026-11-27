//! The translation backend that hands out Arabic in visual order (T21): asked for an Arabic
//! string, it reads the translation as written (through the generated tables, with itself
//! switched off for that one lookup), converts it with [`super::visual_order`] once and
//! keeps the result. Every other language passes straight through to the generated tables.

use super::visual::visual_order;
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::RwLock;

const ARABIC: &str = "ar";
/// Search words of the palette are matched against what people type, never drawn: they stay
/// in logical order.
const MATCH_ONLY_SUFFIX: &str = "_keywords";

thread_local! {
    /// Set while this backend reads the logical translation, so that read reaches the tables.
    static READING_LOGICAL: Cell<bool> = const { Cell::new(false) };
}

/// Extends the generated translations (see `i18n!` in `lib.rs`).
#[derive(Default)]
pub struct VisualArabic {
    converted: RwLock<HashMap<String, String>>,
}

impl VisualArabic {
    fn convert(&self, key: &str) -> Option<String> {
        READING_LOGICAL.with(|reading| reading.set(true));
        let logical = crate::_rust_i18n_try_translate(ARABIC, key).map(Cow::into_owned);
        READING_LOGICAL.with(|reading| reading.set(false));
        let visual = visual_order(&logical?).into_owned();
        if let Ok(mut converted) = self.converted.write() {
            converted.insert(key.to_string(), visual.clone());
        }
        Some(visual)
    }
}

impl rust_i18n::Backend for VisualArabic {
    fn available_locales(&self) -> Vec<Cow<'_, str>> {
        Vec::new()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<Cow<'_, str>> {
        if locale != ARABIC || key.ends_with(MATCH_ONLY_SUFFIX) || READING_LOGICAL.with(Cell::get) {
            return None;
        }
        let known = self.converted.read().ok().and_then(|converted| converted.get(key).cloned());
        known.or_else(|| self.convert(key)).map(Cow::Owned)
    }

    fn messages_for_locale(&self, _locale: &str) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        None
    }
}
