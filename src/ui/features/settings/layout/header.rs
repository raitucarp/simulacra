//! The content header: the category (or search) title and subtitle, the category's header
//! illustration and the close button.

use super::super::callbacks::PlainAction;
use super::super::consts::CLOSE_BUTTON_ID;
use super::super::tabs::SettingsTab;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// What the header names.
pub(in crate::ui::features::settings) enum HeaderSubject {
    Tab(SettingsTab),
    /// Search results: the number of matching settings.
    Search(usize),
}

/// The header bar.
pub(in crate::ui::features::settings) fn render_header(theme: &ThemeTokens, subject: HeaderSubject, on_close: PlainAction) -> Div {
    let (title, subtitle, art) = match subject {
        HeaderSubject::Tab(tab) => (tab.title(), tab.description(), Some(tab.illustration())),
        HeaderSubject::Search(count) => (t!("prefs.ui.search_title").to_string(), t!("prefs.ui.search_count", count = count).to_string(), None),
    };
    div()
        .flex_none()
        .min_h(theme.settings_header_height)
        .px(theme.panel_padding)
        .py(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border)
        .flex()
        .items_center()
        .gap(theme.section_gap)
        .children(art.map(|path| Illustration::asset(path, IllustrationKind::TraitCard).width(theme.settings_header_art_width)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .gap(theme.spacing_2xs)
                .child(div().text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(title))
                .child(div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(subtitle)),
        )
        .child(
            Button::new(CLOSE_BUTTON_ID)
                .ghost()
                .icon(Icon::new(LucideIcon::X).size(theme.icon_size_lg))
                .tooltip(t!("prefs.ui.close_hint").to_string())
                .on_click(move |_, window, cx| on_close(window, cx)),
        )
}
