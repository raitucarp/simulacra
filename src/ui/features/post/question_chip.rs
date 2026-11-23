//! N9: a post asked of the room says so under its author: who was asked, or that an answer
//! was accepted. A locked question shows nothing beyond its placeholder.

use super::lookup::PostLookup;
use crate::domain::{Post, ProfileId};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The chip's text for `post`, naming the agents asked with `name_of` (`None`: not asked, or locked).
pub(crate) fn question_text(post: &Post, name_of: impl Fn(&ProfileId) -> String) -> Option<String> {
    let question = post.extras.question.as_deref().filter(|_| !post.is_locked())?;
    let text = match question.accepted {
        Some(_) => t!("posts.question.chip_answered").to_string(),
        None => t!("posts.question.chip", names = question.asked.iter().map(|asked| name_of(&asked.agent)).collect::<Vec<_>>().join(", ")).to_string(),
    };
    Some(text)
}

/// The chip; `None` for a post never asked.
pub(super) fn render_question_chip<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens) -> Option<Div> {
    let name_of = |id: &ProfileId| lookup.store().and_then(|state| state.user_profiles.iter().find(|p| &p.id == id)).map_or_else(|| id.to_string(), |p| p.name.clone());
    let text = question_text(post, name_of)?;
    let colour = if post.extras.question.as_ref().is_some_and(|q| q.accepted.is_some()) { theme.accent_success } else { theme.text_muted };
    let chip = div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_2xs)
        .text_color(colour)
        .child(Icon::new(LucideIcon::HelpCircle).xsmall().text_color(colour))
        .child(div().min_w_0().truncate().child(text));
    Some(chip)
}
