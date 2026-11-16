//! Plural category rules and the count-label keys every locale must define.

use super::{plural_key_in, PluralCategory, Language};

#[test]
fn english_and_german_are_singular_only_for_one() {
    for locale in ["en", "de", "es", "id", "ar"] {
        assert_eq!(PluralCategory::of(locale, 1), PluralCategory::One);
        for n in [0, 2, 5, 11, 21] {
            assert_eq!(PluralCategory::of(locale, n), PluralCategory::Other, "{locale} {n}");
        }
    }
}

#[test]
fn french_and_portuguese_treat_zero_as_singular() {
    for locale in ["fr", "pt"] {
        assert_eq!(PluralCategory::of(locale, 0), PluralCategory::One);
        assert_eq!(PluralCategory::of(locale, 1), PluralCategory::One);
        assert_eq!(PluralCategory::of(locale, 2), PluralCategory::Other);
    }
}

#[test]
fn russian_follows_the_last_digit() {
    for n in [1, 21, 101, 1001] {
        assert_eq!(PluralCategory::of("ru", n), PluralCategory::One, "{n}");
    }
    for n in [0, 2, 5, 11, 111, 12] {
        assert_eq!(PluralCategory::of("ru", n), PluralCategory::Other, "{n}");
    }
}

#[test]
fn cjk_has_a_single_form() {
    assert_eq!(PluralCategory::of("ja", 1), PluralCategory::Other);
    assert_eq!(PluralCategory::of("zh-CN", 1), PluralCategory::Other);
}

#[test]
fn keys_carry_the_category_suffix() {
    assert_eq!(plural_key_in("en", "feed.replies_count", 1), "feed.replies_count_one");
    assert_eq!(plural_key_in("en", "feed.replies_count", 2), "feed.replies_count_other");
}

/// Every count label base passed to `t_count` / `plural_key`.
const COUNT_LABEL_BASES: &[&str] = &[
    "vault.chunks_count", "vault.corpus_header", "graph.nodes_count", "graph.links_count",
    "graph.mini_nodes", "graph.mini_edges", "feed.replies_count", "feed.show_more",
    "feed.show_more_replies", "sidebar.posts_count", "sidebar.doc_count", "sidebar.member_count",
    "sidebar.member_agents", "cadence.badge", "cadence.day_posts", "post.refs_open_thread",
];

#[test]
fn every_count_label_has_both_variants_in_every_locale() {
    for language in Language::ALL {
        for base in COUNT_LABEL_BASES {
            for count in [1, 2] {
                let key = plural_key_in(language.code(), base, count);
                assert!(crate::_rust_i18n_try_translate(language.code(), &key).is_some(), "{} {key}", language.code());
            }
        }
    }
}

#[test]
fn singular_and_plural_english_labels_differ() {
    let one = crate::_rust_i18n_try_translate("en", &plural_key_in("en", "feed.replies_count", 1));
    let other = crate::_rust_i18n_try_translate("en", &plural_key_in("en", "feed.replies_count", 2));
    assert_eq!(one.as_deref(), Some("%{count} reply"));
    assert_eq!(other.as_deref(), Some("%{count} replies"));
}
