//! N6: the what-if view puts each fork's lines side by side, and a locked post in a line, or a
//! locked first post of a branch, shows only its placeholder.

use super::lists::input;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
use crate::domain::views::{PostView, ViewScope};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::views::branches::{forks, LineColumn};
use crate::ui::features::views::render_list_view;

#[test]
fn each_fork_shows_its_main_line_and_branches_and_never_a_locked_text() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Pick a database.").expect("root");
    let plain = state.submit_reply(&root, "Postgres, as usual.", None, None).expect("plain");
    let fork = state.fork_branch(&root, "What if SQLite?", "One file, no server.").expect("fork");
    let secret = state.fork_branch(&root, "What if the secret plan?", "the secret nobody reads").expect("secret");
    let at = state.post_position(&secret).expect("stored");
    state.posts[at].lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    let theme = ThemeTokens::default();
    let input = input(&state, &theme, ViewScope::Thread);
    let found = forks(&state, &input.posts);
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].at.id, root);
    let labels: Vec<Option<String>> = found[0].columns.iter().map(|column: &LineColumn| column.label.clone()).collect();
    assert_eq!(labels, vec![None, Some("What if SQLite?".into()), Some(LOCKED_PLACEHOLDER.into())]);
    assert_eq!(found[0].columns[0].rows[0].id, plain);
    assert_eq!((found[0].columns[1].rows[0].id.clone(), found[0].columns[1].rows[0].text.clone()), (fork, Some("One file, no server.".into())));
    assert_eq!(found[0].columns[2].rows[0].text, None, "a locked post shows only its placeholder");
    assert!(format!("{found:?}").contains("SQLite") && !format!("{found:?}").contains("secret"));
    assert!(render_list_view(PostView::Branches, &input).is_some());
    for key in ["show", "empty", "forked_at"].map(|key| format!("views.branches.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn a_thread_without_forks_has_none() {
    let (state, _) = super::lists::thread();
    let theme = ThemeTokens::default();
    assert!(forks(&state, &input(&state, &theme, ViewScope::Thread).posts).is_empty());
}
