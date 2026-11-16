//! Bottom-of-rail "Quick Archetype Presets" buttons.

use super::archetypes::{apply_archetype_preset, ArchetypePreset, ARCHETYPE_PRESETS};
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::features::agent_editor::callbacks::UpdateProfileCallback;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const PRESET_ID_PREFIX: &str = "archetype-preset-btn";

pub(super) fn render_presets(
    profile: &Profile,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
) -> Div {
    div()
        .w_full()
        .pt(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(
            div()
                .px(theme.spacing_2xs)
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .child(t!("lab.presets_title").to_string()),
        )
        .children(ARCHETYPE_PRESETS.iter().enumerate().map(|(idx, preset)| {
            preset_button(idx, preset, profile, theme, on_update.clone())
        }))
}

fn preset_button(
    idx: usize,
    preset: &ArchetypePreset,
    profile: &Profile,
    theme: &ThemeTokens,
    on_update: Rc<UpdateProfileCallback>,
) -> Stateful<Div> {
    let base = profile.clone();
    let hover_bg = theme.hover_bg;
    let hover_border = theme.accent_brand_border;
    div()
        .id(format!("{}-{}", PRESET_ID_PREFIX, idx))
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |h| h.bg(hover_bg).border_color(hover_border))
        .on_click(move |_, w, cx| {
            let mut updated = base.clone();
            apply_archetype_preset(&mut updated, idx);
            on_update(updated, w, cx);
        })
        .flex()
        .items_center()
        .justify_between()
        .child(preset_name(preset, theme))
        .child(preset_code(preset, theme))
}

fn preset_name(preset: &ArchetypePreset, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_secondary)
        .truncate()
        .child(preset.name)
}

fn preset_code(preset: &ArchetypePreset, theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.accent_agent)
        .child(preset.code)
}
