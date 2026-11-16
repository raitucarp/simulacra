//! Right-aligned footer holding a compact primary "save" button.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::*;

/// Footer row with a check-icon primary button labeled `label`.
pub fn save_footer(
    id: impl Into<ElementId>,
    label: String,
    theme: &ThemeTokens,
    on_save: impl Fn(&mut Window, &mut App) + 'static,
) -> Div {
    div().flex().justify_end().pt(theme.spacing_xs).child(
        Button::new(id)
            .primary()
            .compact()
            .icon(Icon::new(LucideIcon::Check))
            .label(label)
            .on_click(move |_, w, cx| on_save(w, cx)),
    )
}
