//! A2: a persona's Beliefs tab. A persona does not form beliefs of its own (agents do, as
//! they read and argue); its equivalent is the credences the user stated on its posts
//! (sb E1), newest first, each opening its post. Without any, an empty state says why the
//! tab is empty and how to fill it.

use super::recommended::OpenPostCallback;
use crate::domain::thinking::credence::Credence;
use crate::domain::views::rapid_log::one_line;
use crate::domain::{Post, PostId, Profile, ProfileId};
use crate::state::AppState;
use crate::ui::features::thinking::credence::{credence_summary, CredenceChip};
use jiff::civil::Date;
use crate::theme::ThemeTokens;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID: &str = "persona-credence-";

/// One stated credence on a persona's post, as the tab lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaCredence {
    pub post_id: PostId,
    /// "70% that this is true · resolves by …" (or how it resolved).
    pub summary: String,
    /// The post's opening line (the placeholder of a locked post).
    pub excerpt: String,
}

pub(super) fn persona_beliefs_column(profile: &Profile, credences: &[PersonaCredence], on_open: &OpenPostCallback, theme: &ThemeTokens) -> Div {
    let column = div().w_full().max_w(theme.reader_measure_width).flex().flex_col().gap(theme.spacing_xs);
    if credences.is_empty() {
        return column.items_center().gap(theme.spacing_sm).child(Illustration::empty(EmptyKey::InspectorIdle).width(theme.art_empty_width)).child(heading(t!("roster.persona_beliefs.empty_title", name = profile.name).to_string(), theme)).child(hint(t!("roster.persona_beliefs.empty_hint").to_string(), theme));
    }
    column.child(heading(t!("roster.persona_beliefs.title", name = profile.name).to_string(), theme)).child(hint(t!("roster.persona_beliefs.hint").to_string(), theme)).children(credences.iter().map(|credence| row(credence, on_open, theme)))
}

fn heading(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(text)
}

fn hint(text: String, theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)
}

fn row(credence: &PersonaCredence, on_open: &OpenPostCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let (on_open, id, hover) = (on_open.clone(), credence.post_id.clone(), theme.hover_bg);
    div()
        .id(SharedString::from(format!("{ROW_ID}{}", credence.post_id)))
        .p(theme.spacing_sm)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(move |_, window, cx| on_open(id.clone(), window, cx))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::MEDIUM).text_color(theme.accent_user).child(credence.summary.clone()))
        .child(div().truncate().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(credence.excerpt.clone()))
}

/// The credences stated on `persona`'s posts, newest first, as the tab lists them.
pub fn persona_credences<'a>(state: &AppState, credences: impl IntoIterator<Item = &'a Credence>, persona: &ProfileId, today: Date) -> Vec<PersonaCredence> {
    let mut mine: Vec<(&Credence, &Post)> = credences.into_iter().filter_map(|credence| state.get_post(&credence.post_id).filter(|post| &post.author_id == persona).map(|post| (credence, post))).collect();
    mine.sort_by_key(|(credence, _)| std::cmp::Reverse(credence.created_at));
    mine.into_iter()
        .map(|(credence, post)| {
            let excerpt = if post.is_locked() { t!("posts.lock.locked").to_string() } else { one_line(post) };
            PersonaCredence { post_id: post.id.clone(), summary: credence_summary(&CredenceChip::of(credence, today)), excerpt }
        })
        .collect()
}
