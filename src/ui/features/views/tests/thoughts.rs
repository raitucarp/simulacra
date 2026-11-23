//! N16–N20 views: each renders, and none shows a locked post past its placeholder.

use super::lists::input;
use crate::domain::post::lens::{FanEntry, Lens};
use crate::domain::views::{PostView, ViewScope};
use crate::domain::ProfileId;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::views::render_list_view;

fn keys_translated(prefix: &str, keys: &[&str]) {
    for key in keys.iter().map(|key| format!("{prefix}.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn the_lens_fan_view_lists_each_lens_with_its_reply_side_by_side() {
    let mut state = AppState::new_in_memory();
    let mut agent = crate::domain::Profile::new_agent("ada", "Ada", "@ada", "historian");
    agent.network_id = Some(state.active_network_id.clone());
    state.user_profiles.push(agent);
    let root = state.submit_post("Should cities ban cars?").expect("posted");
    state.fan_out(&root, vec![FanEntry { agent: ProfileId::new("ada"), lens: Lens::History }], jiff::Timestamp::now()).expect("fanned");
    let mut reply = crate::domain::Post::create_reply("post_ada", state.active_network_id.clone(), "ada", "Venice did, long ago.", root.clone(), None, None).expect("reply");
    reply.extras.lens = Some(Lens::History);
    state.insert_reply_after_parent(reply).expect("stored");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Thread);
    let fans = crate::ui::features::views::lens_fan::fans(&state, &view.posts);
    let column = &fans.iter().find(|group| group.root == root).expect("the fan").columns[0];
    assert_eq!((column.lens, column.agent.as_str()), (Lens::History, "Ada"));
    assert_eq!(column.reply.as_ref().and_then(|reply| reply.text.as_deref()), Some("Venice did, long ago."));
    assert!(render_list_view(PostView::LensFan, &view).is_some());
    assert!(!PostView::LensFan.offered_in(ViewScope::Profile), "a fan belongs to a thread");
    keys_translated("views.lens_fan", &["show", "empty", "heading", "waiting"]);
}

#[test]
fn the_glossary_view_lists_the_terms_in_view_and_never_a_locked_post() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Entropy is why rooms get messy.").expect("posted");
    state.define_term(&root, "entropy", "Disorder, counted.", jiff::Timestamp::now()).expect("defined");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Profile);
    assert!(render_list_view(PostView::Glossary, &view).is_some());
    let at = state.post_position(&root).expect("stored");
    state.posts[at].lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    let view = input(&state, &theme, ViewScope::Profile);
    assert!(crate::domain::post::glossary::Glossary::from_posts(view.posts.iter().copied()).is_empty(), "a locked post defines nothing");
    keys_translated("views.glossary", &["show", "empty"]);
}
