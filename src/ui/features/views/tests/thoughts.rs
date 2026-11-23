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

#[test]
fn the_open_questions_view_lists_open_ones_oldest_first_and_a_locked_one_as_its_placeholder() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Why do cats purr?").expect("posted");
    state.set_question(&root, true).expect("asked");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Profile);
    let rows = crate::domain::views::open_questions::open_rows(view.posts.iter().copied());
    assert_eq!(rows.iter().map(|row| row.text.as_deref()).collect::<Vec<_>>(), vec![Some("Why do cats purr?")]);
    assert!(render_list_view(PostView::OpenQuestions, &view).is_some());
    let mut forged = crate::domain::Post::create("forged", "n1", "u1", "secret question").expect("post");
    forged.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    forged.kind = crate::domain::post::PostKind::Question;
    let rows = crate::domain::views::open_questions::open_rows([&forged]);
    assert_eq!(rows[0].text, None, "a locked question is its placeholder");
    keys_translated("views.open_questions", &["show", "empty", "asked"]);
    keys_translated("posts.open_question", &["page_title", "page_explain", "chip_open", "chip_answered", "menu_answers"]);
}

#[test]
fn the_mind_changes_view_sets_the_old_view_beside_the_new_and_never_a_locked_post() {
    let mut state = AppState::new_in_memory();
    let before = state.submit_post("More roads mean less traffic.").expect("posted");
    let after = state.submit_post("Induced demand: more roads, more cars.").expect("posted");
    state.mark_changed_mind(&after, Some(&before), "Roads ease traffic.", jiff::Timestamp::now()).expect("marked");
    let theme = ThemeTokens::default();
    let view = input(&state, &theme, ViewScope::Profile);
    let all: Vec<&crate::domain::Post> = state.posts.iter().collect();
    let rows = crate::ui::features::views::mind_changes::mind_rows(&state, &all);
    assert_eq!(rows.len(), 1);
    assert_eq!((rows[0].was.as_str(), rows[0].revised.as_deref(), rows[0].now.as_str()), ("Roads ease traffic.", Some("More roads mean less traffic."), "Induced demand: more roads, more cars."));
    assert!(render_list_view(PostView::MindChanges, &view).is_some());
    let mut forged = crate::domain::Post::create("forged", "n1", "u1", "secret").expect("post");
    forged.lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    forged.extras.mind_change = Some(Box::new(crate::domain::post::mind::MindChange { revises: None, was: "Leaky old view.".into(), at: jiff::Timestamp::now() }));
    assert!(crate::ui::features::views::mind_changes::mind_rows(&state, &[&forged]).is_empty(), "a locked post is never listed");
    keys_translated("views.mind_changes", &["show", "empty", "before", "now"]);
}
