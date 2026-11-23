//! N10: a part of a series says which, under its author: the series' title, its number (of
//! the goal, when the series has one), a thin progress line, and buttons to the previous and
//! next part. The buttons name no neighbour, so a locked part never shows its text here.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::post::series::progress;
use crate::domain::{Post, PostId};
use crate::state::store::SeriesNav;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The strip's label: "<title> · part 12 of 30" (or "· part 12" open-ended).
pub(crate) fn series_label(nav: &SeriesNav) -> String {
    let entry = &nav.entry;
    match entry.goal {
        Some(goal) => t!("posts.series.part_of", title = entry.title.clone(), number = entry.number, goal = goal).to_string(),
        None => t!("posts.series.part", title = entry.title.clone(), number = entry.number).to_string(),
    }
}

fn step(id: String, icon: LucideIcon, label: String, target: Option<PostId>, handlers: &PostCardHandlers) -> Option<Button> {
    let (open, target) = (handlers.on_open_post.clone(), target?);
    Some(Button::new(SharedString::from(id)).ghost().xsmall().icon(Icon::new(icon)).label(label).on_click(move |_, window, cx| open(target.clone(), window, cx)))
}

/// The strip of a part of a series; `None` outside one.
pub(super) fn render_series_strip<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let nav = lookup.store().and_then(|state| state.series_nav(post))?;
    let label = div().flex_1().min_w_0().truncate().child(series_label(&nav));
    let previous = step(format!("series-previous-{}", post.id), LucideIcon::ChevronLeft, t!("posts.series.previous").to_string(), nav.previous.clone(), handlers);
    let next = step(format!("series-next-{}", post.id), LucideIcon::ChevronRight, t!("posts.series.next").to_string(), nav.next.clone(), handlers);
    let row = div().flex().items_center().gap(theme.inline_gap).child(Icon::new(LucideIcon::ListOrdered).xsmall().text_color(theme.text_muted)).child(label).children(previous).children(next);
    let share = progress(nav.position, Some(nav.entry.goal.unwrap_or(nav.posted as u32)));
    let line = div().w_full().h(theme.journal_progress_height).rounded(theme.radius_full).bg(theme.border_subtle).overflow_hidden().child(div().h_full().w(relative(share)).rounded(theme.radius_full).bg(theme.accent_brand));
    Some(div().w_full().flex().flex_col().gap(theme.spacing_3xs).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(row).child(line))
}
