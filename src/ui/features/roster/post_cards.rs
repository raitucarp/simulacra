//! The profile timeline's cards (W7): each post the profile wrote as a compact card (its
//! symbol or the author's portrait, when it was written, its text with marks and to-dos),
//! a click opening it in the timeline. A locked post shows only its placeholder.

use super::recommended::OpenPostCallback;
use crate::domain::{Post, Profile};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::features::post::{feeling_line, render_post_content, BodyLinks};
use crate::ui::features::symbols::render_symbol_face;
use gpui_kit::*;

const CARD_ID: &str = "profile-post-card-";

/// The cards of `posts`, newest first.
pub(super) fn post_cards(posts: &[&Post], profile: &Profile, state: &AppState, on_open: &OpenPostCallback, theme: &ThemeTokens) -> Div {
    div().w_full().flex().flex_col().gap(theme.spacing_xs).children(posts.iter().map(|post| card(post, profile, state, on_open, theme)))
}

fn card(post: &Post, profile: &Profile, state: &AppState, on_open: &OpenPostCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let face = match post.extras.symbols.face() {
        Some(face) => render_symbol_face(post.id.as_str(), &face, theme.contact_avatar_size, theme).into_any_element(),
        None => render_profile_avatar(profile, theme.contact_avatar_size, theme).into_any_element(),
    };
    let when = crate::ui::format::post_stamp(post.created_at);
    let head = div().flex().items_center().gap(theme.spacing_xs).child(face).child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(when));
    let (on_open, id, hover) = (on_open.clone(), post.id.clone(), theme.hover_bg);
    div()
        .id(SharedString::from(format!("{CARD_ID}{}", post.id)))
        .w_full()
        .p(theme.spacing_sm)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |card| card.bg(hover))
        .on_click(move |_, window, cx| on_open(id.clone(), window, cx))
        .child(head)
        .child(body(post, state, theme))
}

/// A feeling's emojis and note, else the text (the placeholder of a locked post).
fn body(post: &Post, state: &AppState, theme: &ThemeTokens) -> AnyElement {
    match feeling_line(&post.kind, theme).filter(|_| !post.is_locked()) {
        Some(line) => line.into_any_element(),
        None => render_post_content(post, state, theme, &BodyLinks::default(), None),
    }
}
