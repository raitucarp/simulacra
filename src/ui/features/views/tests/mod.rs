mod lists;

use super::{render_view_switcher, switcher_button_id, view_label_key, PickView, PostView, ViewScope};
use crate::theme::ThemeTokens;
use std::collections::HashSet;
use std::rc::Rc;

#[test]
fn every_view_has_a_translated_name_and_its_own_icon() {
    let mut icons = HashSet::new();
    for view in PostView::ALL {
        let key = view_label_key(*view);
        for locale in ["en", "ja", "ar"] {
            assert_ne!(rust_i18n::t!(key, locale = locale), key, "{view:?} in {locale}");
        }
        assert!(icons.insert(format!("{:?}", super::view_icon(*view))), "{view:?} shares an icon");
    }
}

#[test]
fn switcher_ids_are_distinct_per_view() {
    let ids: HashSet<_> = PostView::ALL.iter().map(|view| switcher_button_id("inspector-mode", *view)).collect();
    assert_eq!(ids.len(), PostView::ALL.len());
    assert_eq!(switcher_button_id("inspector-mode", PostView::Cards).as_ref(), "inspector-mode-cards");
}

#[test]
fn the_switcher_renders_for_every_scope_and_view() {
    let theme = ThemeTokens::default();
    let on_pick: PickView = Rc::new(|_, _, _| {});
    for scope in [ViewScope::Thread, ViewScope::Profile] {
        for view in PostView::ALL {
            let _switcher = render_view_switcher(scope, *view, "test", &on_pick, &theme);
        }
    }
}
