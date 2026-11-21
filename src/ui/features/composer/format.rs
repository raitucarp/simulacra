//! Formatting toolbar of the composers: bold, italic, underline, code and spoiler buttons
//! that wrap the selection of a draft (or unwrap it when it is already wrapped).

use crate::domain::post::format::{toggle_mark, FormatMark};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::TextareaState;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, App, Div, Entity, ParentElement, SharedString, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Applies a mark to the draft the toolbar belongs to.
pub type FormatCallback = Rc<dyn Fn(FormatMark, &mut Window, &mut App) + 'static>;

/// Element id prefix of the toolbar buttons (suffixed with the mark id).
const FORMAT_BUTTON_ID: &str = "composer-format";

/// The toolbar icon of a mark.
pub(super) fn format_icon(mark: FormatMark) -> LucideIcon {
    match mark {
        FormatMark::Bold => LucideIcon::Bold,
        FormatMark::Italic => LucideIcon::Italic,
        FormatMark::Underline => LucideIcon::Underline,
        FormatMark::Code => LucideIcon::Code,
        FormatMark::Spoiler => LucideIcon::EyeOff,
    }
}

/// Locale key of a mark's button label.
pub(super) const fn format_label_key(mark: FormatMark) -> &'static str {
    match mark {
        FormatMark::Bold => "composer.format_bold",
        FormatMark::Italic => "composer.format_italic",
        FormatMark::Underline => "composer.format_underline",
        FormatMark::Code => "composer.format_code",
        FormatMark::Spoiler => "composer.format_spoiler",
    }
}

/// Toggles `mark` on the selection of `input`, keeps the result selected and focuses it.
pub(super) fn apply_format(input: &Entity<TextareaState>, mark: FormatMark, window: &mut Window, cx: &mut App) {
    input.update(cx, |state, cx| {
        let edit = toggle_mark(&state.value(), state.selected_range(), mark);
        state.set_selected_range(edit.range.clone(), cx);
        state.replace(edit.text, window, cx);
        state.set_selected_range(edit.selection, cx);
        state.focus(window, cx);
    });
}

/// The row of format buttons; `scope` keeps their ids apart between composers.
pub(super) fn format_buttons(scope: &'static str, on_format: FormatCallback, theme: &ThemeTokens) -> Div {
    let buttons = FormatMark::ALL.into_iter().map(|mark| {
        let on_format = on_format.clone();
        Button::new(SharedString::from(format!("{FORMAT_BUTTON_ID}-{scope}-{}", mark.id())))
            .ghost()
            .small()
            .icon(Icon::new(format_icon(mark)))
            .tooltip(t!(format_label_key(mark)).to_string())
            .on_click(move |_, window, cx| on_format(mark, window, cx))
    });
    div().flex().items_center().gap(theme.control_gap).children(buttons)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_mark_has_its_own_icon_and_a_translated_label() {
        let icons: HashSet<LucideIcon> = FormatMark::ALL.into_iter().map(format_icon).collect();
        assert_eq!(icons.len(), FormatMark::ALL.len());
        for mark in FormatMark::ALL {
            let key = format_label_key(mark);
            assert_ne!(rust_i18n::t!(key, locale = "en"), key, "{mark:?}");
            assert!(key.ends_with(mark.id()));
        }
    }

    #[test]
    fn the_toolbar_renders_one_button_per_mark() {
        let callback: FormatCallback = Rc::new(|_, _, _| {});
        let _row = format_buttons("test", callback, &ThemeTokens::default());
    }
}
