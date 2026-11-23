//! The Posts tab of a profile (W7): everything the persona or agent wrote, newest first, a
//! page at a time, in the view picked from the registry's profile views. "Feelings only"
//! narrows it to the feelings it posted (M4). Posts waiting to be published appear only in
//! the future log; every other view shows what was published.

use super::params::{RosterCallbacks, RosterParams};
use super::post_cards::post_cards;
use super::posts_state::{ProfilePosts, ProfilePostsEvent};
use crate::domain::views::{PostView, ViewScope};
use crate::domain::{Post, Profile};
use crate::i18n::t_count;
use crate::state::store::Page;
use crate::state::AppState;
use crate::ui::features::views::common::empty_note;
use crate::ui::features::views::{render_list_view, render_view_switcher, PickMonth, PickView, ViewInput};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Posts per page of the timeline.
pub const PROFILE_PAGE_SIZE: usize = 30;
const SWITCHER_ID: &str = "profile-view";
const FEELINGS_ID: &str = "profile-feelings-only";
const MORE_ID: &str = "profile-posts-more";

/// Sends a Posts tab event to the shell.
pub type PostsAction = Rc<dyn Fn(ProfilePostsEvent, &mut Window, &mut App)>;

/// The posts the tab lists up to its shown page, and whether more wait after them.
pub fn profile_posts<'a>(state: &'a AppState, profile: &Profile, shown: ProfilePosts) -> (Vec<&'a Post>, bool) {
    let page = Page::first(PROFILE_PAGE_SIZE.saturating_mul(shown.pages.max(1)));
    let written = if shown.feelings_only { state.feelings_by(&profile.id, page) } else { state.posts_by(&profile.id, page) };
    let more = written.len() == page.size;
    (written, more)
}

pub(super) fn posts_column(profile: &Profile, params: &RosterParams<'_>, callbacks: &RosterCallbacks) -> Div {
    let (theme, shown, send) = (params.theme, params.posts, callbacks.posts.clone());
    let (written, more) = profile_posts(params.state, profile, shown);
    let (scheduled, published): (Vec<&Post>, Vec<&Post>) = written.into_iter().partition(|post| post.lifecycle.is_scheduled());
    let body = match shown.view {
        PostView::Cards if published.is_empty() => empty_note("roster.posts.empty", theme),
        PostView::Cards => post_cards(&published, profile, params.state, &callbacks.open_post, theme).into_any_element(),
        view => {
            let month = send.clone();
            let on_month: PickMonth = Rc::new(move |target, window, cx| month(ProfilePostsEvent::Month(target), window, cx));
            let input = ViewInput { state: params.state, theme, posts: published, scheduled, scope: ViewScope::Profile, on_open: callbacks.open_post.clone(), month: shown.month, on_month };
            render_list_view(view, &input).unwrap_or_else(|| empty_note("roster.posts.empty", theme))
        }
    };
    let more = more.then(|| {
        let send = send.clone();
        Button::new(MORE_ID).ghost().small().label(t!("roster.posts.more").to_string()).on_click(move |_, window, cx| send(ProfilePostsEvent::LoadMore, window, cx))
    });
    div().w_full().max_w(theme.reader_measure_width).flex().flex_col().gap(theme.spacing_sm).child(toolbar(profile, shown, &send, params)).child(body).children(more)
}

fn toolbar(profile: &Profile, shown: ProfilePosts, send: &PostsAction, params: &RosterParams<'_>) -> Div {
    let theme = params.theme;
    let pick: PickView = {
        let send = send.clone();
        Rc::new(move |view, window, cx| send(ProfilePostsEvent::Pick(view), window, cx))
    };
    let toggle = send.clone();
    let feelings = Button::new(FEELINGS_ID)
        .ghost()
        .xsmall()
        .selected(shown.feelings_only)
        .icon(Icon::new(LucideIcon::HeartPulse))
        .label(t!("roster.posts.feelings_only").to_string())
        .on_click(move |_, window, cx| toggle(ProfilePostsEvent::ToggleFeelings, window, cx));
    let count = div().flex_1().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t_count("roster.posts.count", params.state.post_count_by(&profile.id)));
    div().w_full().flex().items_center().gap(theme.control_gap).child(count).child(feelings).child(render_view_switcher(ViewScope::Profile, shown.view, SWITCHER_ID, &pick, theme))
}
