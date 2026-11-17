//! The category rail: the search box, one keyboard-reachable button per category (with its
//! match count while searching and a dot on unsaved AI changes) and the privacy footer.
//! Up / Down move between categories while the category list has focus (not the search box).

use super::super::callbacks::TabAction;
use super::super::catalog::{hits_in, SearchHit};
use super::super::consts::{KEY_DOWN, KEY_UP, RAIL_ID, TAB_BUTTON_ID_PREFIX};
use super::super::props::{SettingsData, SettingsInputs};
use super::super::tabs::SettingsTab;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::Input;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// What the rail shows.
pub(in crate::ui::features::settings) struct RailView<'a> {
    pub active: SettingsTab,
    pub data: &'a SettingsData,
    pub inputs: &'a SettingsInputs,
    /// Search hits (empty when not searching).
    pub hits: &'a [SearchHit],
    pub on_select: TabAction,
}

/// The whole rail.
pub(in crate::ui::features::settings) fn render_rail(theme: &ThemeTokens, view: RailView) -> Div {
    let searching = !view.data.query.trim().is_empty();
    let buttons: Vec<_> = SettingsTab::ALL
        .into_iter()
        .map(|tab| {
            let count = searching.then(|| hits_in(view.hits, tab));
            let dirty = tab == SettingsTab::AiIntelligence && view.data.ai_dirty;
            tab_button(theme, tab, !searching && tab == view.active, count, dirty, view.on_select.clone())
        })
        .collect();
    div()
        .size_full()
        .bg(theme.bg_sidebar)
        .border_r(theme.border_width_hairline)
        .border_color(theme.border)
        .p(theme.panel_padding_compact)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(Input::new(&view.inputs.search).cleanable(true).prefix(Icon::new(LucideIcon::Search).size(theme.icon_size_sm)).text_size(theme.font_size_label))
        .child(category_list(theme, &view, buttons))
        .child(render_footer(theme))
}

/// The scrolling, keyboard-navigable list of category buttons.
fn category_list(theme: &ThemeTokens, view: &RailView, buttons: Vec<Stateful<Div>>) -> impl IntoElement {
    let (active, on_key) = (view.active, view.on_select.clone());
    div()
        .id(RAIL_ID)
        .track_focus(&view.inputs.rail_focus)
        .on_key_down(move |event: &KeyDownEvent, window, cx| {
            let offset = match event.keystroke.key.as_str() {
                KEY_UP => -1,
                KEY_DOWN => 1,
                _ => return,
            };
            cx.stop_propagation();
            on_key(active.step(offset), window, cx);
        })
        .flex_1()
        .min_h_0()
        .overflow_y_scrollbar()
        .child(div().flex().flex_col().gap(theme.spacing_2xs).children(buttons))
}

/// One category button.
fn tab_button(theme: &ThemeTokens, tab: SettingsTab, active: bool, count: Option<usize>, dirty: bool, on_select: TabAction) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let dim = count == Some(0);
    let pick: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |window, cx| on_select(tab, window, cx));
    let click = pick.clone();
    div()
        .id(SharedString::from(format!("{TAB_BUTTON_ID_PREFIX}{}", tab.id())))
        .w_full()
        .h(theme.settings_rail_item_height)
        .px(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.bg_transparent)
        .bg(if active { theme.bg_tab_active } else { theme.bg_transparent })
        .hover(move |style| style.bg(hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .when(dim, |button| button.opacity(theme.settings_disabled_opacity))
        .on_click(move |_, window, cx| click(window, cx))
        .keyboard_stop(theme, pick)
        .child(Icon::new(tab.icon()).size(theme.icon_size_lg).text_color(if active { theme.accent_brand } else { theme.text_muted }))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .text_size(theme.font_size_sm)
                .font_weight(if active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                .text_color(if active { theme.text_primary } else { theme.text_secondary })
                .child(tab.title()),
        )
        .when(dirty, |button| button.child(div().size(theme.dot_size_sm).rounded(theme.radius_full).bg(theme.warning)))
        .children(count.filter(|count| *count > 0).map(|count| count_badge(theme, count)))
}

/// Matches in a category while searching.
fn count_badge(theme: &ThemeTokens, count: usize) -> Div {
    div()
        .px(theme.spacing_xs)
        .rounded(theme.radius_full)
        .bg(theme.accent_dialectic_subtle)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.accent_dialectic)
        .child(count.to_string())
}

/// "Everything stays on this computer."
fn render_footer(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.hover_bg)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(LucideIcon::Lock).size(theme.icon_size_sm).text_color(theme.text_muted))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("settings.rail_footer").to_string()))
}
