//! The profile page's Posts tab (W7): paging, the feelings filter, and every view rendering.

use super::params::{LabMode, ProfileTab, RosterCallbacks, RosterParams};
use super::posts_state::{ProfilePosts, ProfilePostsEvent};
use super::posts_tab::{profile_posts, PROFILE_PAGE_SIZE};
use crate::domain::post::{Intensity, PostKind};
use crate::domain::views::calendar::YearMonth;
use crate::domain::views::PostView;
use crate::domain::ProfileId;
use crate::state::{AppState, ThreadAnchor};
use crate::theme::ThemeTokens;
use std::rc::Rc;

fn callbacks() -> RosterCallbacks {
    let profile = || -> super::params::ProfileCallback { Rc::new(|_, _, _| {}) };
    RosterCallbacks {
        select: profile(), switch_mode: Rc::new(|_, _, _| {}), create: Rc::new(|_, _| {}), edit: profile(), delete: profile(), activate_persona: profile(), trigger_agent: profile(),
        switch_profile_tab: Rc::new(|_, _, _| {}), open_post: Rc::new(|_, _, _| {}), posts: Rc::new(|_, _, _| {}),
    }
}

fn written(count: usize) -> (AppState, ProfileId) {
    let mut state = AppState::new_in_memory();
    for n in 0..count {
        state.submit_post(&format!("thought {n}")).expect("posted");
    }
    let felt = PostKind::feeling(&["🌧".to_string()], Intensity::new(2), "grey").expect("feeling");
    state.submit_feeling(felt, &ThreadAnchor::default()).expect("felt");
    let author = ProfileId::new(state.active_profile_id.as_str());
    (state, author)
}

#[test]
fn the_tab_pages_through_the_posts_and_filters_to_feelings() {
    let (state, author) = written(PROFILE_PAGE_SIZE + 4);
    let profile = state.user_profiles.iter().find(|p| p.id == author).expect("persona").clone();
    let first = ProfilePosts::default();
    let (posts, more) = profile_posts(&state, &profile, first);
    assert_eq!((posts.len(), more), (PROFILE_PAGE_SIZE, true));
    let (posts, more) = profile_posts(&state, &profile, first.after(ProfilePostsEvent::LoadMore));
    let total = state.post_count_by(&author);
    assert!(total >= PROFILE_PAGE_SIZE + 5 && total < PROFILE_PAGE_SIZE * 2, "the store may hold a seeded post: {total}");
    assert_eq!((posts.len(), more), (total, false));
    let feelings = first.after(ProfilePostsEvent::ToggleFeelings);
    let (posts, _) = profile_posts(&state, &profile, feelings);
    assert!(!posts.is_empty() && posts.iter().all(|post| post.kind.is_feeling()));
}

#[test]
fn a_new_view_or_filter_starts_on_the_first_page() {
    let shown = ProfilePosts::default().after(ProfilePostsEvent::LoadMore).after(ProfilePostsEvent::LoadMore);
    assert_eq!(shown.pages, 3);
    assert_eq!(shown.after(ProfilePostsEvent::Pick(PostView::Raw)).pages, 1);
    assert_eq!(shown.after(ProfilePostsEvent::ToggleFeelings).pages, 1);
    let month = YearMonth { year: 2026, month: 3 };
    assert_eq!(shown.after(ProfilePostsEvent::Month(month)).month, Some(month));
}

#[test]
fn the_posts_tab_renders_in_every_profile_view() {
    let (state, author) = written(3);
    let theme = ThemeTokens::default();
    for view in PostView::in_scope(crate::domain::views::ViewScope::Profile) {
        let params = RosterParams {
            state: &state, mode: LabMode::PersonaLab, selected_profile_id: Some(&author), theme: &theme, profile_tab: ProfileTab::Posts, beliefs: None,
            recommended: Rc::from([]), posts: ProfilePosts { view, ..ProfilePosts::default() }, persona_credences: Rc::from([]),
        };
        let _view = super::render_roster_view(params, callbacks());
    }
    for key in ["roster.posts.tab", "roster.posts.empty", "roster.posts.more", "roster.posts.feelings_only", "roster.posts.count_one", "roster.posts.count_other"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

#[test]
fn a_persona_s_beliefs_are_its_stated_credences_newest_first() {
    use super::persona_beliefs::persona_credences;
    use crate::domain::thinking::credence::Credence;
    let (mut state, author) = written(2);
    let ids: Vec<_> = state.posts.iter().filter(|post| post.author_id == author && !post.kind.is_feeling()).map(|post| post.id.clone()).take(2).collect();
    let at = |second: i64| jiff::Timestamp::from_second(second).expect("timestamp");
    let credences = [Credence::new(ids[0].clone(), 70, at(100)), Credence::new(ids[1].clone(), 20, at(200)), Credence::new(crate::domain::PostId::new("elsewhere"), 50, at(300))];
    let today = "2026-11-23".parse().expect("date");
    let listed = persona_credences(&state, &credences, &author, today);
    assert_eq!(listed.iter().map(|row| row.post_id.clone()).collect::<Vec<_>>(), [ids[1].clone(), ids[0].clone()]);
    assert!(listed[0].summary.contains("20"), "{}", listed[0].summary);
    let at_post = state.post_position(&ids[0]).expect("stored");
    state.posts[at_post].lock_with(&crate::domain::post::lock::LockPassword::new("seven tidy lanterns"), crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locks");
    let listed = persona_credences(&state, &credences, &author, today);
    assert_eq!(listed[1].excerpt, rust_i18n::t!("posts.lock.locked").to_string(), "a locked post shows only its placeholder");
}

#[test]
fn personas_and_agents_share_one_tab_row_and_a_persona_s_beliefs_tab_renders() {
    let (state, author) = written(1);
    let theme = ThemeTokens::default();
    for credences in [Vec::new(), vec![super::PersonaCredence { post_id: crate::domain::PostId::new("p"), summary: "70%".into(), excerpt: "a claim".into() }]] {
        let params = RosterParams {
            state: &state, mode: LabMode::PersonaLab, selected_profile_id: Some(&author), theme: &theme, profile_tab: ProfileTab::Beliefs, beliefs: None,
            recommended: Rc::from([]), posts: ProfilePosts::default(), persona_credences: Rc::from(credences),
        };
        let _view = super::render_roster_view(params, callbacks());
    }
    for key in ["title", "hint", "empty_title", "empty_hint"].map(|key| format!("roster.persona_beliefs.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}
