//! N8: a post with a scope says so under its author: "Only me", or the agents it is shared
//! with. A post for everyone shows nothing.

use super::lookup::PostLookup;
use crate::domain::post::audience::Audience;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The badge's text for `post`, naming chosen agents with `name_of` (`None`: everyone).
pub(crate) fn audience_text(post: &Post, name_of: impl Fn(&crate::domain::ProfileId) -> String) -> Option<String> {
    let text = match post.extras.audience.as_deref()? {
        Audience::OnlyMe => t!("posts.audience.only_me").to_string(),
        Audience::Agents { agents } if agents.is_empty() => t!("posts.audience.only_me").to_string(),
        Audience::Agents { agents } => t!("posts.audience.shared_with", names = agents.iter().map(name_of).collect::<Vec<_>>().join(", ")).to_string(),
    };
    Some(text)
}

/// The badge; `None` for a post everyone may read.
pub(super) fn render_audience_badge<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens) -> Option<Div> {
    let name_of = |id: &crate::domain::ProfileId| lookup.store().and_then(|state| state.user_profiles.iter().find(|p| &p.id == id)).map_or_else(|| id.to_string(), |p| p.name.clone());
    let text = audience_text(post, name_of)?;
    let icon = if matches!(post.extras.audience.as_deref(), Some(Audience::Agents { agents }) if !agents.is_empty()) { LucideIcon::Users } else { LucideIcon::EyeOff };
    let badge = div()
        .max_w_full()
        .min_w_0()
        .overflow_hidden()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .child(Icon::new(icon).xsmall().text_color(theme.text_muted))
        .child(div().min_w_0().truncate().child(text));
    Some(badge)
}
