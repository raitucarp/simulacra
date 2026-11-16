//! Hero header quick actions: save, synthesize post / use persona, delete.

use super::callbacks::SharedCallbacks;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::components::{meta_badge, BadgeColors};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const SAVE_BUTTON_ID: &str = "save-agent-text-fields-btn";
const TRIGGER_BUTTON_ID: &str = "trigger-this-agent-post-btn";
const ACTIVATE_BUTTON_ID: &str = "activate-human-persona-btn";
const DELETE_BUTTON_ID: &str = "delete-lab-profile-btn";

/// Render-time flags for the header actions.
#[derive(Debug, Clone, Copy)]
pub(crate) struct HeaderFlags {
    /// The profile is the human persona currently used as the active user.
    pub active_user_persona: bool,
    /// More than one roster entry exists, so deleting is allowed.
    pub can_delete: bool,
}

pub(crate) fn render_header_actions(
    profile: &Profile,
    flags: HeaderFlags,
    theme: &ThemeTokens,
    callbacks: &SharedCallbacks,
) -> Div {
    let on_save = callbacks.save_text.clone();
    div()
        .flex_shrink_0()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            Button::new(SAVE_BUTTON_ID)
                .ghost()
                .compact()
                .icon(Icon::new(LucideIcon::Check))
                .label(t!("lab.save_changes").to_string())
                .on_click(move |_, w, cx| on_save(w, cx)),
        )
        .child(primary_action(profile, flags, theme, callbacks))
        .when(flags.can_delete, |s| s.child(delete_button(profile, callbacks)))
}

/// Agents get "Synthesize post"; personas get "Use persona" or an "active" badge.
fn primary_action(
    profile: &Profile,
    flags: HeaderFlags,
    theme: &ThemeTokens,
    callbacks: &SharedCallbacks,
) -> AnyElement {
    let prof_id = profile.id.clone();
    if profile.is_agent() {
        let on_trigger = callbacks.trigger_agent.clone();
        return Button::new(TRIGGER_BUTTON_ID)
            .primary()
            .compact()
            .icon(Icon::new(LucideIcon::Sparkles))
            .label(t!("lab.synthesize_post").to_string())
            .on_click(move |_, w, cx| on_trigger(prof_id.clone(), w, cx))
            .into_any_element();
    }
    if flags.active_user_persona {
        let colors = BadgeColors::new(
            theme.accent_success,
            theme.accent_success_subtle,
            theme.accent_success_border,
        );
        return meta_badge(&t!("lab.active_persona_badge"), colors, theme).into_any_element();
    }
    let on_activate = callbacks.activate_persona.clone();
    Button::new(ACTIVATE_BUTTON_ID)
        .primary()
        .compact()
        .icon(Icon::new(LucideIcon::UserCheck))
        .label(t!("lab.use_persona").to_string())
        .on_click(move |_, w, cx| on_activate(prof_id.clone(), w, cx))
        .into_any_element()
}

fn delete_button(profile: &Profile, callbacks: &SharedCallbacks) -> Button {
    let prof_id = profile.id.clone();
    let on_delete = callbacks.delete.clone();
    Button::new(DELETE_BUTTON_ID)
        .ghost()
        .compact()
        .icon(Icon::new(LucideIcon::Trash))
        .tooltip(t!("lab.delete_tooltip").to_string())
        .on_click(move |_, w, cx| on_delete(prof_id.clone(), w, cx))
}
