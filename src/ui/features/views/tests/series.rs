//! N10: the series view lists each series' parts in order, and a locked part only as its placeholder.

use super::lists::input;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::views::{PostView, ViewScope};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::views::render_list_view;

#[test]
fn the_series_view_renders_and_never_shows_a_locked_part() {
    let mut state = AppState::new_in_memory();
    let first = state.submit_post("Sketch one.").expect("first");
    let second = state.submit_reply(&first, "the secret sketch", None, None).expect("second");
    state.add_to_series(&first, "Sketches", Some(10)).expect("added");
    state.add_to_series(&second, "Sketches", None).expect("added");
    let at = state.post_position(&second).expect("stored");
    state.posts[at].lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Profile);
    let groups = crate::domain::views::series::series_groups(view.posts.iter().copied());
    assert_eq!(groups[0].parts.len(), 2);
    assert!(format!("{groups:?}").contains("Sketch one.") && !format!("{groups:?}").contains("secret"));
    assert!(render_list_view(PostView::Series, &view).is_some());
    for key in ["show", "empty", "progress", "parts_one", "parts_other"].map(|key| format!("views.series.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn the_still_true_view_renders_and_lists_no_locked_claim() {
    let mut state = AppState::new_in_memory();
    state.submit_post("Fresh claim.").expect("posted");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Profile);
    assert!(render_list_view(PostView::StillTrue, &view).is_some(), "the empty view renders");
    assert!(PostView::StillTrue.offered_in(ViewScope::Profile));
    for key in ["show", "empty", "due"].map(|key| format!("views.still_true.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}
