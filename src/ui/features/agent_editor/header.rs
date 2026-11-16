//! Compact hero header: clickable avatar, name, handle, vocation and meta badges.

use super::callbacks::{PickAvatarCallback, SharedCallbacks};
use super::header_actions::{render_header_actions, HeaderFlags};
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{meta_badge, BadgeColors};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const AVATAR_TRIGGER_ID: &str = "hero-avatar-upload-trigger";

pub(crate) fn render_hero_header(
    profile: &Profile,
    flags: HeaderFlags,
    theme: &ThemeTokens,
    callbacks: &SharedCallbacks,
) -> Div {
    div()
        .w_full()
        .px(theme.spacing_lg)
        .py(theme.spacing_sm)
        .bg(theme.bg_elevated)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_md)
        .child(identity_block(profile, theme, callbacks.pick_avatar.clone()))
        .child(render_header_actions(profile, flags, theme, callbacks))
}

fn identity_block(profile: &Profile, theme: &ThemeTokens, on_pick: Rc<PickAvatarCallback>) -> Div {
    let prof_id = profile.id.clone();
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_md)
        .min_w_0()
        .flex_1()
        .child(
            div()
                .id(AVATAR_TRIGGER_ID)
                .cursor_pointer()
                .on_click(move |_, w, cx| on_pick(prof_id.clone(), w, cx))
                .child(render_profile_avatar(profile, theme.hero_avatar_size, theme)),
        )
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_2xs)
                .child(name_row(profile, theme))
                .child(badge_row(profile, theme)),
        )
}

fn name_row(profile: &Profile, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .flex_wrap()
        .child(
            div()
                .font_family(theme.font_family_display.clone())
                .text_size(theme.font_size_md)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(profile.name.clone()),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .text_color(theme.text_muted)
                .child(profile.display_handle()),
        )
        .when(!profile.config.occupation.is_empty(), |s| {
            s.child(
                div()
                    .text_size(theme.font_size_xs)
                    .text_color(theme.text_secondary)
                    .child(format!("· {}", profile.config.occupation)),
            )
        })
}

fn badge_row(profile: &Profile, theme: &ThemeTokens) -> Div {
    let cfg = &profile.config;
    let mbti = format!("{} · {}", cfg.mbti_full_code(), cfg.mbti.archetype_name());
    let ennea = format!("{} ({})", cfg.enneagram.code(), cfg.instinctual_variant.code());
    let brand = BadgeColors::new(theme.accent_brand, theme.accent_brand_subtle, theme.accent_brand_border);
    let dialectic = BadgeColors::new(
        theme.accent_dialectic,
        theme.accent_dialectic_subtle,
        theme.accent_dialectic_border,
    );
    let language = BadgeColors::new(theme.accent_user, theme.hover_bg, theme.border_subtle);
    div()
        .flex()
        .items_center()
        .flex_wrap()
        .gap(theme.spacing_xs)
        .child(meta_badge(&mbti, brand, theme))
        .child(meta_badge(&ennea, dialectic, theme))
        .child(meta_badge(&cfg.primary_language, language, theme))
}
