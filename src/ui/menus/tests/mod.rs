//! Tests of the menu model, and the structure rules over every menu the app builds.

mod model;
mod specs;

use super::rules::{label_keys, violations};
use super::MenuSpec;
use crate::i18n::Language;

/// Every structure rule and every label translated in every locale, for `spec` named `name`.
pub(super) fn assert_tidy(name: &str, spec: &MenuSpec) {
    let problems = violations(spec);
    assert!(problems.is_empty(), "{name}: {problems:#?}");
    for key in label_keys(spec) {
        for language in Language::ALL {
            assert!(crate::_rust_i18n_try_translate(language.code(), &key).is_some(), "{name}: {key} missing in {}", language.code());
        }
    }
}
