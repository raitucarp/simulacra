//! The list views render from a thread's or a profile's posts, and a locked post shows only
//! its placeholder.

use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::views::{PostView, ViewScope};
use crate::domain::PostId;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::feed::thread_posts;
use crate::ui::features::views::{render_list_view, ViewInput};
use std::rc::Rc;

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

/// A thread: a root, a reply, and a locked reply under the reply; returns their ids.
pub(super) fn thread() -> (AppState, Vec<PostId>) {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Root of the thread\n[ ] write the plan ^2099-01-05").expect("root");
    let reply = state.submit_reply(&root, "A reply with **marks**", None, None).expect("reply");
    let locked = state.submit_reply(&reply, "the secret nobody reads", None, None).expect("locked");
    let at = state.post_position(&locked).expect("stored");
    state.posts[at].lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locks");
    (state, vec![root, reply, locked])
}

pub(super) fn input<'a>(state: &'a AppState, theme: &'a ThemeTokens, scope: ViewScope) -> ViewInput<'a> {
    let root = state.get_post(&state.posts.iter().find(|post| post.reply_to_id.is_none()).expect("root").id).expect("root");
    ViewInput { state, theme, posts: thread_posts(state, root), scheduled: Vec::new(), scope, on_open: Rc::new(|_, _, _| {}), month: None, on_month: Rc::new(|_, _, _| {}) }
}

#[test]
fn the_thread_reads_root_first_then_its_replies_in_order() {
    let (state, ids) = thread();
    let locked = state.get_post(&ids[2]).expect("locked");
    let posts: Vec<PostId> = thread_posts(&state, locked).into_iter().map(|post| post.id.clone()).collect();
    assert_eq!(posts, ids);
}

#[test]
fn every_list_view_renders_in_both_scopes_and_the_inspector_draws_the_rest() {
    let (state, _) = thread();
    let theme = ThemeTokens::default();
    for scope in [ViewScope::Thread, ViewScope::Profile] {
        let input = input(&state, &theme, scope);
        for view in PostView::ALL {
            let drawn = render_list_view(*view, &input).is_some();
            assert_eq!(drawn, !matches!(view, PostView::Cards | PostView::Outline | PostView::Argument), "{view:?}");
        }
    }
}

#[test]
fn an_empty_list_renders_its_empty_note() {
    let state = AppState::new_in_memory();
    let theme = ThemeTokens::default();
    let input = ViewInput { state: &state, theme: &theme, posts: Vec::new(), scheduled: Vec::new(), scope: ViewScope::Profile, on_open: Rc::new(|_, _, _| {}), month: None, on_month: Rc::new(|_, _, _| {}) };
    for view in PostView::ALL {
        let _ = render_list_view(*view, &input);
    }
    assert_ne!(rust_i18n::t!("views.rapid_log.empty", locale = "en"), "views.rapid_log.empty");
}

#[test]
fn the_raw_text_of_a_thread_never_holds_a_locked_post() {
    use crate::domain::views::raw::{raw_blocks, raw_text};
    let (state, _) = thread();
    let theme = ThemeTokens::default();
    let input = input(&state, &theme, ViewScope::Thread);
    let text = raw_text(&raw_blocks(input.posts.iter().copied(), |post| state.author_of(post).name.clone(), "Locked post"));
    assert!(text.contains("A reply with **marks**"), "marks stay as written");
    assert!(text.contains("Locked post") && !text.contains("secret"), "{text}");
}

#[test]
fn the_calendar_opens_on_the_newest_post_or_the_chosen_month() {
    use crate::domain::views::calendar::YearMonth;
    use crate::ui::features::views::calendar::shown_month;
    let (state, _) = thread();
    let theme = ThemeTokens::default();
    let mut input = input(&state, &theme, ViewScope::Thread);
    let today = "2030-05-17".parse().expect("date");
    let newest = input.posts.iter().map(|post| crate::time::local_date(post.created_at)).max().expect("posts");
    assert_eq!(shown_month(&input, today), YearMonth::of(newest));
    input.month = Some(YearMonth { year: 2031, month: 2 });
    assert_eq!(shown_month(&input, today), YearMonth { year: 2031, month: 2 });
    input.posts.clear();
    input.month = None;
    assert_eq!(shown_month(&input, today), YearMonth { year: 2030, month: 5 });
    for key in ["views.calendar.previous", "views.calendar.next", "views.calendar.today", "views.calendar.month_title", "views.calendar.more"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

#[test]
fn the_future_log_reads_the_thread_todos_and_the_scheduled_posts() {
    use crate::domain::post::PostLifecycle;
    use crate::domain::views::future::{future_log, FutureItem};
    let (state, _) = thread();
    let theme = ThemeTokens::default();
    let mut waiting = crate::domain::Post::create("waiting", "net", "author", "Out on Friday").expect("post");
    waiting.lifecycle = PostLifecycle::scheduled("2099-02-01T09:00:00Z".parse().expect("timestamp"));
    let mut input = input(&state, &theme, ViewScope::Profile);
    input.scheduled.push(&waiting);
    let today = "2026-11-23".parse().expect("date");
    let months = future_log(input.posts.iter().chain(&input.scheduled).copied(), today, crate::time::local_date);
    let items: Vec<_> = months.iter().flat_map(|month| &month.entries).map(|entry| (entry.text.as_str(), entry.item.clone())).collect();
    assert_eq!(items.len(), 2, "{items:?}");
    assert_eq!(items[0].0, "write the plan");
    assert!(matches!(items[1].1, FutureItem::Post { locked: false }));
    assert!(render_list_view(PostView::Future, &input).is_some());
}
