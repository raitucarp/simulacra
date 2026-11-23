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
            recommended: Rc::from([]), posts: ProfilePosts { view, ..ProfilePosts::default() },
        };
        let _view = super::render_roster_view(params, callbacks());
    }
    for key in ["roster.posts.tab", "roster.posts.empty", "roster.posts.more", "roster.posts.feelings_only", "roster.posts.count_one", "roster.posts.count_other"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}
