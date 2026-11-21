//! The tint picker of the composer (sb C3): a round swatch per container tint, plus one for
//! no tint, the chosen one ringed. Used for new threads and while editing a post.

use crate::domain::post::Tint;
use crate::theme::{tint_colors, ThemeTokens};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, Div, InteractiveElement, ParentElement, SharedString, StatefulInteractiveElement, Stateful, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Picks a tint (`None` clears it).
pub type TintCallback = Rc<dyn Fn(Option<Tint>, &mut Window, &mut App) + 'static>;

/// Element id prefix of the swatches (suffixed with the tint key, or `none`).
const SWATCH_ID: &str = "composer-tint";
/// Id suffix of the no-tint swatch.
const NONE_ID: &str = "none";

/// Locale key of a tint's name, or of "No tint".
pub fn tint_label_key(tint: Option<Tint>) -> &'static str {
    match tint {
        None => "composer.tint.none",
        Some(Tint::Amber) => "composer.tint.amber",
        Some(Tint::Rose) => "composer.tint.rose",
        Some(Tint::Sky) => "composer.tint.sky",
        Some(Tint::Mint) => "composer.tint.mint",
        Some(Tint::Violet) => "composer.tint.violet",
        Some(Tint::Slate) => "composer.tint.slate",
    }
}

/// Every choice in picker order: no tint first, then each tint.
pub(super) fn tint_choices() -> impl Iterator<Item = Option<Tint>> {
    std::iter::once(None).chain(Tint::ALL.iter().copied().map(Some))
}

/// The labelled row of swatches with `chosen` ringed.
pub(super) fn tint_picker(chosen: Option<Tint>, on_pick: TintCallback, theme: &ThemeTokens) -> Div {
    let label = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("composer.tint.label").to_string());
    let swatches = tint_choices().map(|tint| swatch(tint, tint == chosen, on_pick.clone(), theme));
    div().flex().items_center().gap(theme.control_gap).child(label).children(swatches)
}

fn swatch(tint: Option<Tint>, selected: bool, on_pick: TintCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let (fill, edge) = tint.map_or((theme.bg_app, theme.border), |tint| {
        let colors = tint_colors(tint, theme);
        (colors.background, colors.border)
    });
    let name = t!(tint_label_key(tint)).to_string();
    div()
        .id(SharedString::from(format!("{SWATCH_ID}-{}", tint.map_or(NONE_ID, Tint::key))))
        .size(theme.icon_size_md)
        .flex()
        .items_center()
        .justify_center()
        .rounded(theme.radius_full)
        .bg(fill)
        .cursor_pointer()
        .border(if selected { theme.border_width_threadline } else { theme.border_width_hairline })
        .border_color(if selected { theme.border_focus } else { edge })
        .when(tint.is_none(), |dot| dot.text_color(theme.text_muted).child(Icon::new(LucideIcon::X).size(theme.icon_size_xs)))
        .tooltip(move |window, cx| Tooltip::new(name.clone()).build(window, cx))
        .on_click(move |_, window, cx| on_pick(tint, window, cx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_choice_has_its_own_translated_name() {
        let keys: HashSet<&str> = tint_choices().map(tint_label_key).collect();
        assert_eq!(keys.len(), Tint::ALL.len() + 1);
        for key in keys.into_iter().chain(["composer.tint.label"]) {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
        assert_eq!(tint_choices().next(), Some(None), "no tint comes first");
    }

    #[test]
    fn the_picker_renders_with_any_choice() {
        let on_pick: TintCallback = Rc::new(|_, _, _| {});
        for chosen in tint_choices() {
            let _row = tint_picker(chosen, on_pick.clone(), &ThemeTokens::default());
        }
    }
}
