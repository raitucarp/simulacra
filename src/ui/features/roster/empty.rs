//! The illustrated empty state when the current roster has nobody in it.

use super::pane::create_label;
use super::params::LabMode;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const CREATE_ID: &str = "roster-empty-create";

pub(super) fn render_empty_roster(mode: LabMode, theme: &ThemeTokens, on_create: Rc<dyn Fn(&mut Window, &mut App)>) -> Div {
    let title = match mode {
        LabMode::NetworkAgents => t!("lab.empty_agents"),
        LabMode::PersonaLab => t!("lab.empty_personas"),
    };
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(theme.spacing_md)
        .child(Illustration::empty(EmptyKey::NoAgents).width(theme.forge_empty_art_width))
        .child(div().text_size(theme.font_size_lg).font_weight(FontWeight::SEMIBOLD).child(title.to_string()))
        .child(div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(t!("lab.empty_hint").to_string()))
        .child(
            Button::new(CREATE_ID)
                .primary()
                .icon(LucideIcon::Plus)
                .label(create_label(mode))
                .on_click(move |_, w, cx| on_create(w, cx)),
        )
}
