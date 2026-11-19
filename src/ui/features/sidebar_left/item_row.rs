//! One row of the Pinned, Recent, Sources and Members sections: an icon or avatar, a label,
//! an optional detail and a pin toggle that appears on hover (always shown once pinned).

use super::prefs::NavRef;
use super::tooltip::with_tooltip;
use super::SidebarCallbacks;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Hover group shared by a row and its pin toggle.
const ROW_GROUP: &str = "sidebar-item-row";
/// Suffix of the pin toggle's element id.
const PIN_ID_SUFFIX: &str = "-pin";

/// What a row starts with.
pub(super) enum Leading {
    Icon(LucideIcon),
    Avatar(Box<Profile>),
}

/// The content of one row.
pub(super) struct ItemRow {
    pub id: SharedString,
    pub reference: NavRef,
    pub label: String,
    pub detail: Option<String>,
    pub leading: Leading,
    pub is_pinned: bool,
}

fn render_leading(leading: &Leading, theme: &ThemeTokens) -> Div {
    match leading {
        Leading::Icon(icon) => div().w(theme.nav_icon_slot_width).flex().flex_none().justify_center().text_color(theme.text_muted).child(Icon::new(*icon).size(theme.icon_size_xs)),
        Leading::Avatar(profile) => div().flex_none().child(render_profile_avatar(profile, theme.avatar_size_sm, theme)),
    }
}

pub(super) fn render_item_row(row: ItemRow, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let (on_open, on_key) = (callbacks.on_open.clone(), callbacks.on_open.clone());
    let (open_ref, key_ref) = (row.reference.clone(), row.reference.clone());
    let detail = row.detail.clone().map(|text| {
        div().flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text)
    });
    div()
        .id(row.id.clone())
        .group(ROW_GROUP)
        .w_full()
        .h(theme.nav_row_height)
        .px(theme.spacing_sm)
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .hover(move |style| style.bg(hover_bg))
        .keyboard_stop(theme, Rc::new(move |window, cx| on_key(key_ref.clone(), window, cx)))
        .on_click(move |_, window, cx| on_open(open_ref.clone(), window, cx))
        .child(render_leading(&row.leading, theme))
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_label).text_color(theme.text_secondary).child(row.label.clone()))
        .children(detail)
        .child(render_pin_toggle(&row, callbacks, theme))
}

fn render_pin_toggle(row: &ItemRow, callbacks: &SidebarCallbacks, theme: &ThemeTokens) -> Stateful<Div> {
    let (on_toggle, reference) = (callbacks.on_toggle_pin.clone(), row.reference.clone());
    let (icon, label_key) = if row.is_pinned { (LucideIcon::PinOff, "sidebar.unpin") } else { (LucideIcon::Pin, "sidebar.pin") };
    let accent = theme.accent_brand;
    let toggle = div()
        .id(SharedString::from(format!("{}{PIN_ID_SUFFIX}", row.id)))
        .flex_none()
        .p(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(accent))
        .when(!row.is_pinned, |toggle| toggle.invisible().group_hover(ROW_GROUP, |style| style.visible()))
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_toggle(reference.clone(), window, cx);
        })
        .child(Icon::new(icon).size(theme.icon_size_xs));
    with_tooltip(toggle, t!(label_key).to_string(), None)
}
