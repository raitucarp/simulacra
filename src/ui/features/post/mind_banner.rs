//! N19: a post that changed the user's mind wears a banner under its author: what they used to
//! think and the earlier post it revises (a click opens it); a revised post says it was
//! revised later (a click opens the newest post that revised it). Neither shows on a locked
//! post, nor names a locked one.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::views::rapid_log::one_line;
use crate::domain::{Post, PostId};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// The banner's lines: "Before: …" and "Revises: …" (`None`: unmarked, or locked).
pub(crate) fn mind_lines<L: PostLookup>(post: &Post, lookup: L) -> Option<Vec<String>> {
    let change = post.extras.mind_change.as_deref().filter(|_| !post.is_locked())?;
    let was = (!change.was.is_empty()).then(|| t!("posts.mind.was", was = change.was.clone()).to_string());
    let revised = change.revises.as_ref().and_then(|id| lookup.find_post(id.as_str())).filter(|old| !old.is_locked()).map(|old| t!("posts.mind.revises", post = one_line(old)).to_string());
    Some(was.into_iter().chain(revised).collect())
}

fn opener(target: Option<PostId>, part: &str, post: &Post, handlers: &PostCardHandlers, row: Div) -> Div {
    let Some(target) = target else { return row };
    let on_open = handlers.on_open_post.clone();
    div().min_w_0().child(row.id(SharedString::from(format!("mind-{part}-{}", post.id))).cursor_pointer().on_click(move |_, window, cx| on_open(target.clone(), window, cx)))
}

/// The banner of a post that changed the user's mind; `None` for every other post.
pub(super) fn render_mind_banner<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let lines = mind_lines(post, lookup)?;
    let target = post.extras.mind_change.as_ref().and_then(|change| change.revises.clone()).filter(|id| lookup.find_post(id.as_str()).is_some_and(|old| !old.is_locked()));
    // The revised post is quoted on one line: the banner points at it, a click opens it whole.
    let revises_at = target.as_ref().map(|_| lines.len() - 1);
    let head = div().flex().items_center().gap(theme.inline_gap).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_dialectic).child(Icon::new(LucideIcon::RefreshCw).xsmall().text_color(theme.accent_dialectic)).child(t!("posts.mind.banner").to_string());
    let body = lines.into_iter().enumerate().map(|(at, line)| div().min_w_0().text_color(theme.text_primary).when(revises_at == Some(at), |row| row.truncate()).child(line));
    let banner = div().w_full().flex().flex_col().gap(theme.spacing_3xs).px(theme.spacing_sm).py(theme.spacing_xs).rounded(theme.radius_md).bg(theme.bg_callout).border_l(theme.border_width_threadline).border_color(theme.accent_dialectic).text_size(theme.font_size_xs).child(head).children(body);
    Some(opener(target, "banner", post, handlers, banner))
}

/// The chip of a post a later post revised; `None` for every other post (and a locked one).
pub(super) fn render_revised_chip<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let newest = lookup.store()?.revised_by(&post.id).first().map(|later| later.id.clone())?;
    let chip = div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.accent_dialectic).child(Icon::new(LucideIcon::RefreshCw).xsmall().text_color(theme.accent_dialectic)).child(t!("posts.mind.revised").to_string());
    Some(opener(Some(newest), "revised", post, handlers, chip))
}
