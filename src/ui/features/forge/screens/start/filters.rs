//! The gallery's shelf tabs (every category, with its icon) and the search box.

use crate::domain::palette::{category_illustration, category_label_key, PaletteCategory, ALL_CATEGORIES_KEY};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ActionCallback, ForgeTextInput, KeyboardStop};
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const TAB_ID_PREFIX: &str = "forge-palette-tab";
const SEARCH_ID: &str = "forge-palette-search";
const SEARCH_PLACEHOLDER: &str = "forge.palette.ui.search_placeholder";

/// One shelf tab: the category icon over its name; the active tab is tinted.
fn tab(key: &str, active: bool, theme: &ThemeTokens, pick: ActionCallback) -> Stateful<Div> {
    let (hover_bg, click) = (theme.bg_elevated, pick.clone());
    let art = Illustration::asset(category_illustration(key), IllustrationKind::TraitCard).width(theme.forge_palette_tab_icon_width);
    div()
        .id(SharedString::from(format!("{TAB_ID_PREFIX}-{key}")))
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(if active { theme.accent_brand } else { theme.border_subtle })
        .bg(if active { theme.accent_brand_subtle } else { theme.bg_card })
        .cursor_pointer()
        .when(!active, |tab| tab.hover(move |style| style.bg(hover_bg)))
        .on_click(move |_, window, cx| click(window, cx))
        .keyboard_stop(theme, pick)
        .child(art)
        .child(
            div()
                .text_size(theme.font_size_sm)
                .font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                .text_color(if active { theme.accent_brand_text } else { theme.text_primary })
                .child(tr(&category_label_key(key))),
        )
}

impl ForgeView {
    /// The shelf tabs and the search box, wrapping onto several lines when narrow.
    pub(super) fn gallery_filters(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let all = std::iter::once((ALL_CATEGORIES_KEY, None));
        let shelves = all.chain(PaletteCategory::ALL.into_iter().map(|c| (c.key(), Some(c))));
        let tabs = shelves.map(|(key, category)| {
            let pick = handle.act(move |this, _, cx| this.set_shelf(category, cx));
            tab(key, self.start.category == category, theme, pick)
        });
        let search = ForgeTextInput::new(SEARCH_ID, self.start.search.clone(), theme, {
            let set = handle.with(|this, text: String, _, cx| this.set_search(text, cx));
            move |text, window, cx| set(text, window, cx)
        })
        .placeholder(tr(SEARCH_PLACEHOLDER));
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.spacing_sm)
            .children(tabs)
            .child(div().w(theme.forge_palette_search_width).child(search))
    }

    /// Shows the shelf of `category` (`None`: every palette).
    pub(crate) fn set_shelf(&mut self, category: Option<PaletteCategory>, cx: &mut Context<Self>) {
        if self.start.category != category {
            self.start.category = category;
            cx.notify();
        }
    }

    /// Filters the gallery by `text`.
    pub(crate) fn set_search(&mut self, text: String, cx: &mut Context<Self>) {
        if self.start.search != text {
            self.start.search = text;
            cx.notify();
        }
    }
}
