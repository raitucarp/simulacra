//! [`KeyboardHint`]: a keyboard icon whose hover card (or Enter / Space) lists the forge's
//! shortcuts as key caps and what they do.

use super::explainer::ExplainerButton;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use std::rc::Rc;

const HINT_ID: &str = "forge-keyboard-hint";
const TITLE_KEY: &str = "forge.keys.title";
/// (key caps, what they do) of every shortcut, in reading order.
const SHORTCUTS: [(&str, &str); 5] = [
    ("forge.keys.tab_caps", "forge.keys.tab"),
    ("forge.keys.arrows_caps", "forge.keys.arrows"),
    ("forge.keys.activate_caps", "forge.keys.activate"),
    ("forge.keys.undo_caps", "forge.keys.undo"),
    ("forge.keys.escape_caps", "forge.keys.escape"),
];

/// The shortcut list as a keyboard-icon explainer.
pub struct KeyboardHint;

impl KeyboardHint {
    pub fn button(theme: &ThemeTokens) -> ExplainerButton {
        ExplainerButton::custom(HINT_ID, LucideIcon::Keyboard, Rc::new(card), theme)
    }
}

fn row(caps_key: &str, action_key: &str, theme: &ThemeTokens) -> Div {
    let caps = div()
        .flex_shrink_0()
        .min_w(theme.forge_key_cap_min_width)
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .bg(theme.bg_card)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_primary)
        .child(tr(caps_key));
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(caps)
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_secondary).line_height(relative(theme.line_height_body)).child(tr(action_key)))
}

fn card(theme: &ThemeTokens) -> Div {
    div()
        .w(theme.forge_explainer_width)
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::BOLD).child(tr(TITLE_KEY)))
        .children(SHORTCUTS.iter().map(|(caps, action)| row(caps, action, theme)))
}
