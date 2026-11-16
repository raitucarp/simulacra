//! [`ToggleCard`]: an on/off setting as a card with a title, an optional explanation line
//! and a kit switch. Clicking the text flips it too.

use super::ValueCallback;
use crate::state::forge::ToggleField;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{field_label, tr_opt};
use gpui_kit::component::switch::Switch;
use gpui_kit::*;
use std::rc::Rc;

const CARD_ID_PREFIX: &str = "forge-toggle";
/// `forge.ui.toggle.{path}`: optional one-line explanation under the title.
const HINT_ROOT: &str = "forge.ui.toggle";

/// A switch card bound to one toggle field (or any labeled flag via [`ToggleCard::custom`]).
#[derive(IntoElement)]
pub struct ToggleCard {
    id: SharedString,
    title: String,
    hint: Option<String>,
    checked: bool,
    theme: ThemeTokens,
    on_toggle: ValueCallback<bool>,
}

impl ToggleCard {
    /// A card for `field` showing `checked`; `on_toggle` gets the new value.
    pub fn new(field: ToggleField, checked: bool, theme: &ThemeTokens, on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        let id = SharedString::from(format!("{CARD_ID_PREFIX}-{}", field.path()));
        let hint = tr_opt(&format!("{HINT_ROOT}.{}", field.path()));
        Self::custom(id, field_label(field.path()), checked, theme, on_toggle).hint(hint)
    }

    /// A card for any flag with a ready-made title.
    pub fn custom(
        id: impl Into<SharedString>,
        title: impl Into<String>,
        checked: bool,
        theme: &ThemeTokens,
        on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self { id: id.into(), title: title.into(), hint: None, checked, theme: theme.clone(), on_toggle: Rc::new(on_toggle) }
    }

    /// Explanation shown under the title.
    pub fn hint(mut self, hint: Option<String>) -> Self {
        self.hint = hint;
        self
    }
}

impl RenderOnce for ToggleCard {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        let checked = self.checked;
        let (card_toggle, switch_toggle) = (self.on_toggle.clone(), self.on_toggle);
        let text = div()
            .id(SharedString::from(format!("{}-text", self.id)))
            .cursor_pointer()
            .on_click(move |_, window, cx| card_toggle(!checked, window, cx))
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_2xs)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).child(self.title))
            .children(self.hint.map(|hint| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(hint)));
        div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .gap(theme.spacing_md)
            .p(theme.spacing_md)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(if checked { theme.accent_brand_border } else { theme.border_subtle })
            .bg(if checked { theme.accent_brand_subtle } else { theme.bg_card })
            .child(text)
            .child(
                Switch::new(SharedString::from(format!("{}-switch", self.id)))
                    .checked(checked)
                    .color(theme.accent_brand)
                    .on_click(move |value, window, cx| switch_toggle(*value, window, cx)),
            )
    }
}
