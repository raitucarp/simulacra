//! A collapsible sidebar section: a header that folds it (remembered), at most
//! [`SECTION_PREVIEW_LIMIT`] rows, a "Show all" link and an empty hint.

use super::prefs::SidebarSection;
use super::SidebarCallbacks;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Rows shown before "Show all".
pub const SECTION_PREVIEW_LIMIT: usize = 5;
/// Suffix of the "Show all" link's element id.
const SHOW_ALL_ID_SUFFIX: &str = "-show-all";

/// How a section renders this frame.
pub(super) struct SectionView {
    pub section: SidebarSection,
    pub is_collapsed: bool,
    pub is_showing_all: bool,
    /// Rows before truncation.
    pub rows: Vec<Stateful<Div>>,
}

/// The rows a section shows: all of them, or the first few.
pub fn visible_count(total: usize, is_showing_all: bool) -> usize {
    if is_showing_all { total } else { total.min(SECTION_PREVIEW_LIMIT) }
}

pub(super) fn render_section(view: SectionView, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Div {
    let total = view.rows.len();
    let header = render_header(view.section, view.is_collapsed, total, callbacks, theme);
    let body = (!view.is_collapsed).then(|| {
        let shown = visible_count(total, view.is_showing_all);
        let link = (total > SECTION_PREVIEW_LIMIT).then(|| render_show_all(view.section, view.is_showing_all, total, callbacks, theme));
        let empty = (total == 0).then(|| div().px(theme.spacing_sm).py(theme.spacing_2xs).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!(view.section.empty_key()).to_string()));
        div().flex().flex_col().children(view.rows.into_iter().take(shown)).children(link).children(empty)
    });
    div().flex().flex_col().gap(theme.spacing_3xs).child(header).children(body)
}

fn render_header(section: SidebarSection, is_collapsed: bool, total: usize, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let (hover_text, on_click) = (theme.text_secondary, callbacks.on_toggle_section.clone());
    let on_key = on_click.clone();
    let chevron = if is_collapsed { LucideIcon::ChevronRight } else { LucideIcon::ChevronDown };
    div()
        .id(section.header_id())
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(hover_text))
        .keyboard_stop(theme, Rc::new(move |window, cx| on_key(section, window, cx)))
        .on_click(move |_, window, cx| on_click(section, window, cx))
        .child(Icon::new(chevron).size(theme.icon_size_xs))
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).child(t!(section.title_key()).to_string()))
        .when(total > 0, |row| row.child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).child(total.to_string())))
}

fn render_show_all(section: SidebarSection, is_showing_all: bool, total: usize, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let (accent, on_click) = (theme.accent_brand, callbacks.on_toggle_show_all.clone());
    let label = if is_showing_all { t!("sidebar.show_less").to_string() } else { t!("sidebar.show_all", count = total).to_string() };
    div()
        .id(SharedString::from(format!("{}{SHOW_ALL_ID_SUFFIX}", section.header_id())))
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .cursor_pointer()
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(accent))
        .on_click(move |_, window, cx| on_click(section, window, cx))
        .child(label)
}
