//! N16: a reply written through a lens names it under its author, and a post fanned out
//! through lenses lists them, each ticked once its agent has replied. Neither shows on a
//! locked post.

use super::lookup::PostLookup;
use crate::domain::post::lens::FanLine;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::data::lens_name;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The lens chip's text: "Through the lens of ethics" (`None`: no lens, or locked).
pub(crate) fn lens_chip_text(post: &Post) -> Option<String> {
    let lens = post.extras.lens.filter(|_| !post.is_locked())?;
    Some(t!("posts.lens.chip", lens = lens_name(lens)).to_string())
}

fn chip(icon: LucideIcon, text: String, theme: &ThemeTokens) -> Div {
    div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(Icon::new(icon).xsmall().text_color(theme.text_muted)).child(div().min_w_0().truncate().child(text))
}

/// The chip of a reply through a lens; `None` for every other post.
pub(super) fn render_lens_chip(post: &Post, theme: &ThemeTokens) -> Option<Div> {
    Some(chip(LucideIcon::Eye, lens_chip_text(post)?, theme))
}

fn fan_item(line: &FanLine, theme: &ThemeTokens) -> Div {
    let (icon, colour) = if line.reply.is_some() { (LucideIcon::Check, theme.accent_success) } else { (LucideIcon::Clock, theme.text_muted) };
    div().flex().items_center().gap(theme.spacing_3xs).child(Icon::new(icon).xsmall().text_color(colour)).child(lens_name(line.lens))
}

/// The fan strip of a post fanned out through lenses; `None` for every other post.
pub(super) fn render_fan_strip<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens) -> Option<Div> {
    let lines = lookup.store()?.fan_lines(&post.id);
    if lines.is_empty() {
        return None;
    }
    let items = lines.iter().map(|line| fan_item(line, theme));
    Some(chip(LucideIcon::Layers, crate::ui::features::data::tr("posts.lens.strip"), theme).flex_wrap().gap_x(theme.inline_gap).children(items))
}
